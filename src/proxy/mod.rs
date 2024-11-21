use std::collections::HashMap;
use hyper::{Body, Request, Response};

pub use self::types::{
    ProxyError, ProxyResult, ProxyConfig, RouteConfig,
    Middleware, Transform, PathParams, TransformConfig,
    CacheConfig, RateLimitConfig,
};
pub use self::handler::RequestHandler;
pub use self::parser::RequestParser;
pub use self::pool::{ProxyPool, ProxyPoolMetrics};
pub use self::middleware::{
    LoggingMiddleware,
    HeaderTransform,
    RateLimiter,
    CacheMiddleware,
};

mod types;
mod handler;
mod parser;
mod pool;
mod middleware;

pub struct ProxyBuilder {
    config: ProxyConfig,
    middleware: Vec<Arc<dyn Middleware>>,
    transforms: Vec<Arc<dyn Transform>>,
    request_headers: HashMap<String, String>,
    response_headers: HashMap<String, String>,
}

use std::sync::Arc;

impl Default for ProxyBuilder {
    fn default() -> Self {
        ProxyBuilder {
            config: ProxyConfig::default(),
            middleware: Vec::new(),
            transforms: Vec::new(),
            request_headers: HashMap::new(),
            response_headers: HashMap::new(),
        }
    }
}

impl ProxyBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_pool_size(mut self, size: u32) -> Self {
        self.config.pool_size = size;
        self
    }

    pub fn with_timeout(mut self, timeout_secs: u64) -> Self {
        self.config.pool_timeout_secs = timeout_secs;
        self
    }

    pub fn with_middleware<M: Middleware + 'static>(mut self, middleware: M) -> Self {
        self.middleware.push(Arc::new(middleware));
        self
    }

    pub fn with_transform<T: Transform + 'static>(mut self, transform: T) -> Self {
        self.transforms.push(Arc::new(transform));
        self
    }

    pub fn with_request_header(mut self, key: &str, value: &str) -> Self {
        self.request_headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_response_header(mut self, key: &str, value: &str) -> Self {
        self.response_headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn build(self) -> RequestHandler {
        let mut handler = RequestHandler::with_config(self.config);

        // Add header transform if headers were specified
        if !self.request_headers.is_empty() || !self.response_headers.is_empty() {
            handler.add_transform(Arc::new(HeaderTransform::new(
                self.request_headers,
                self.response_headers,
            )));
        }

        // Add custom transforms
        for transform in self.transforms {
            handler.add_transform(transform);
        }

        // Add middleware
        for middleware in self.middleware {
            handler.add_middleware(middleware);
        }

        handler
    }
}

// Convenience function to create a basic proxy with logging
pub fn create_basic_proxy() -> RequestHandler {
    ProxyBuilder::new()
        .with_middleware(LoggingMiddleware::new("proxy"))
        .build()
}

// Convenience function to create a proxy with common production settings
pub fn create_production_proxy() -> RequestHandler {
    ProxyBuilder::new()
        .with_pool_size(100)
        .with_timeout(30)
        .with_middleware(LoggingMiddleware::new("production"))
        .with_middleware(RateLimiter::new(60, 1000)) // 1000 req/min
        .with_middleware(CacheMiddleware::new(300)) // 5 min cache
        .with_response_header("X-Proxy-Version", env!("CARGO_PKG_VERSION"))
        .build()
}

// Helper function to handle proxy requests
pub async fn handle_proxy_request(
    handler: &RequestHandler,
    req: Request<Body>,
) -> Result<Response<Body>, ProxyError> {
    Ok(handler.handle_request(req).await)
}
