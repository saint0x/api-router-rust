use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use hyper::{Body, Request, Response, HeaderMap};
use http::header::HeaderName;
use tracing::{info, warn};

use super::types::{Middleware, Transform, ProxyRequest, ProxyResult, ProxyError};

// Logging middleware
pub struct LoggingMiddleware {
    prefix: String,
}

impl LoggingMiddleware {
    pub fn new(prefix: &str) -> Self {
        LoggingMiddleware {
            prefix: prefix.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl Middleware for LoggingMiddleware {
    async fn handle(&self, req: &mut ProxyRequest) -> ProxyResult<()> {
        info!(
            "{} {} {} {}",
            self.prefix,
            req.request.method(),
            req.request.uri(),
            if let Some(params) = &req.path_params {
                format!("params: {:?}", params.params)
            } else {
                "no params".to_string()
            }
        );
        Ok(())
    }
}

// Header transform
pub struct HeaderTransform {
    request_headers: HashMap<HeaderName, String>,
    response_headers: HashMap<HeaderName, String>,
}

impl HeaderTransform {
    pub fn new(
        request_headers: HashMap<String, String>,
        response_headers: HashMap<String, String>,
    ) -> Self {
        // Convert string header names to HeaderName
        let req_headers = request_headers.into_iter()
            .filter_map(|(k, v)| {
                HeaderName::from_bytes(k.as_bytes()).ok().map(|name| (name, v))
            })
            .collect();

        let res_headers = response_headers.into_iter()
            .filter_map(|(k, v)| {
                HeaderName::from_bytes(k.as_bytes()).ok().map(|name| (name, v))
            })
            .collect();

        HeaderTransform {
            request_headers: req_headers,
            response_headers: res_headers,
        }
    }

    fn apply_headers(headers: &mut HeaderMap, to_apply: &HashMap<HeaderName, String>) {
        for (key, value) in to_apply {
            if let Ok(header_value) = value.parse() {
                headers.insert(key, header_value);
            }
        }
    }
}

#[async_trait::async_trait]
impl Transform for HeaderTransform {
    async fn transform_request(&self, req: &mut Request<Body>) -> ProxyResult<()> {
        Self::apply_headers(req.headers_mut(), &self.request_headers);
        Ok(())
    }

    async fn transform_response(&self, res: &mut Response<Body>) -> ProxyResult<()> {
        Self::apply_headers(res.headers_mut(), &self.response_headers);
        Ok(())
    }
}

// Rate limiter
pub struct RateLimiter {
    requests: Arc<RwLock<HashMap<String, Vec<Instant>>>>,
    window: Duration,
    max_requests: usize,
}

impl RateLimiter {
    pub fn new(window_secs: u64, max_requests: usize) -> Self {
        RateLimiter {
            requests: Arc::new(RwLock::new(HashMap::new())),
            window: Duration::from_secs(window_secs),
            max_requests,
        }
    }

    async fn is_rate_limited(&self, key: &str) -> bool {
        let now = Instant::now();
        let mut requests = self.requests.write().await;
        
        // Get or create timestamps for this key
        let timestamps = requests.entry(key.to_string()).or_insert_with(Vec::new);
        
        // Remove old timestamps
        timestamps.retain(|&ts| now.duration_since(ts) <= self.window);
        
        // Check if we're over the limit
        if timestamps.len() >= self.max_requests {
            warn!("Rate limit exceeded for {}", key);
            true
        } else {
            timestamps.push(now);
            false
        }
    }
}

#[async_trait::async_trait]
impl Middleware for RateLimiter {
    async fn handle(&self, req: &mut ProxyRequest) -> ProxyResult<()> {
        // Use IP address or some other identifier as the rate limit key
        let key = req.request
            .headers()
            .get("x-forwarded-for")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown")
            .to_string();

        if self.is_rate_limited(&key).await {
            return Err(ProxyError::MiddlewareError(
                "Rate limit exceeded".to_string()
            ));
        }

        Ok(())
    }
}

// Cache middleware
pub struct CacheMiddleware {
    cache: Arc<RwLock<HashMap<String, (Vec<u8>, Instant)>>>,
    ttl: Duration,
}

impl CacheMiddleware {
    pub fn new(ttl_secs: u64) -> Self {
        CacheMiddleware {
            cache: Arc::new(RwLock::new(HashMap::new())),
            ttl: Duration::from_secs(ttl_secs),
        }
    }

    fn generate_cache_key(req: &Request<Body>) -> String {
        format!("{}:{}", req.method(), req.uri())
    }
}

#[async_trait::async_trait]
impl Middleware for CacheMiddleware {
    async fn handle(&self, req: &mut ProxyRequest) -> ProxyResult<()> {
        // Only cache GET requests
        if req.request.method() != hyper::Method::GET {
            return Ok(());
        }

        let cache_key = Self::generate_cache_key(&req.request);
        let now = Instant::now();

        // Check cache
        if let Some((cached_response, timestamp)) = self.cache.read().await.get(&cache_key) {
            if now.duration_since(*timestamp) <= self.ttl {
                info!("Cache hit for {}", cache_key);
                // TODO: Set cached response in request context for later use
            }
        }

        Ok(())
    }
}
