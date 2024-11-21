use std::sync::Arc;
use hyper::{Body, Request, Response, StatusCode, Uri};
use tracing::{error, info};

use super::{
    parser::RequestParser,
    pool::ProxyPool,
    types::{ProxyConfig, ProxyError, Middleware, Transform},
};

pub struct RequestHandler {
    pool: Arc<ProxyPool>,
    config: ProxyConfig,
    middleware: Vec<Arc<dyn Middleware>>,
    transforms: Vec<Arc<dyn Transform>>,
}

impl RequestHandler {
    pub fn new() -> Self {
        let config = ProxyConfig::default();
        let pool = Arc::new(ProxyPool::new(config.clone()));
        
        RequestHandler { 
            pool,
            config,
            middleware: Vec::new(),
            transforms: Vec::new(),
        }
    }

    pub fn with_config(config: ProxyConfig) -> Self {
        let pool = Arc::new(ProxyPool::new(config.clone()));
        
        RequestHandler {
            pool,
            config,
            middleware: Vec::new(),
            transforms: Vec::new(),
        }
    }

    pub fn add_middleware(&mut self, middleware: Arc<dyn Middleware>) {
        self.middleware.push(middleware);
    }

    pub fn add_transform(&mut self, transform: Arc<dyn Transform>) {
        self.transforms.push(transform);
    }

    pub async fn handle_request(&self, req: Request<Body>) -> Response<Body> {
        match self.process_request(req).await {
            Ok(response) => response,
            Err(e) => self.handle_error(e),
        }
    }

    async fn process_request(&self, req: Request<Body>) -> Result<Response<Body>, ProxyError> {
        // Parse the request and extract route configuration
        let mut proxy_req = RequestParser::parse(req)?;

        // Apply global middleware first
        for middleware in &self.middleware {
            middleware.handle(&mut proxy_req).await?;
        }

        // Clone necessary data from route config
        let (transforms, cache_config, rate_limit) = if let Some(route_config) = &proxy_req.route_config {
            (
                route_config.transforms.clone(),
                route_config.cache.clone(),
                route_config.rate_limit.clone(),
            )
        } else {
            Default::default()
        };

        // Apply route-specific middleware
        if let Some(route_config) = &proxy_req.route_config {
            for middleware_name in &route_config.middleware {
                info!("Applying middleware: {}", middleware_name);
            }
        }

        // Apply rate limiting if configured
        if let Some(rate_limit) = rate_limit {
            info!("Applying rate limit: {} req/s", rate_limit.requests_per_second);
        }

        // Check cache if configured
        if let Some(cache_config) = cache_config {
            info!("Checking cache with TTL: {}s", cache_config.ttl_seconds);
        }

        // Apply request transforms
        if let Some(request_transforms) = transforms.request {
            info!("Applying request transforms");
            // Apply headers
            if let Some(headers) = request_transforms.headers {
                for (key, value) in headers {
                    if let Ok(header_value) = value.parse() {
                        proxy_req.request.headers_mut().insert(
                            key.as_str(),
                            header_value
                        );
                    }
                }
            }

            // Apply query parameters
            if let Some(query) = request_transforms.query {
                let mut parts = proxy_req.request.uri().clone().into_parts();
                let mut query_string = String::new();
                for (key, value) in query {
                    if !query_string.is_empty() {
                        query_string.push('&');
                    }
                    query_string.push_str(&format!("{}={}", key, value));
                }
                if !query_string.is_empty() {
                    if let Some(path_and_query) = parts.path_and_query {
                        let new_path_and_query = format!("{}?{}", path_and_query.path(), query_string);
                        parts.path_and_query = Some(new_path_and_query.parse().unwrap());
                    }
                }
                *proxy_req.request.uri_mut() = Uri::from_parts(parts).unwrap();
            }
        }

        // Apply global transforms
        for transform in &self.transforms {
            transform.transform_request(&mut proxy_req.request).await?;
        }

        // Forward the request
        let mut response = self.pool.forward_request(
            proxy_req.destination,
            proxy_req.request
        ).await?;

        // Apply response transforms
        if let Some(response_transforms) = transforms.response {
            info!("Applying response transforms");
            // Apply headers
            if let Some(headers) = response_transforms.headers {
                for (key, value) in headers {
                    if let Ok(header_value) = value.parse() {
                        response.headers_mut().insert(
                            key.as_str(),
                            header_value
                        );
                    }
                }
            }
        }

        // Apply global transforms
        for transform in &self.transforms {
            transform.transform_response(&mut response).await?;
        }

        Ok(response)
    }

    fn handle_error(&self, error: ProxyError) -> Response<Body> {
        error!("Request handling error: {}", error);

        let (status, message) = match error {
            ProxyError::InvalidPath(_) => (
                StatusCode::BAD_REQUEST,
                "Invalid request path",
            ),
            ProxyError::InvalidUri(_) => (
                StatusCode::BAD_REQUEST,
                "Invalid request URI",
            ),
            ProxyError::RequestBuildError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to process request",
            ),
            ProxyError::ForwardError(_) => (
                StatusCode::BAD_GATEWAY,
                "Failed to forward request",
            ),
            ProxyError::InvalidDecorator(_) => (
                StatusCode::BAD_REQUEST,
                "Invalid @zap decorator",
            ),
            ProxyError::MiddlewareError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Middleware error",
            ),
            ProxyError::TransformError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Transform error",
            ),
        };

        let error_response = serde_json::json!({
            "error": message,
            "code": status.as_u16(),
            "timestamp": chrono::Utc::now().to_rfc3339()
        });

        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(Body::from(serde_json::to_string(&error_response).unwrap()))
            .unwrap()
    }
}
