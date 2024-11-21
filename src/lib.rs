//! Rust API Router
//! 
//! A high-performance API router and proxy server with support for:
//! - Dynamic route handling with path parameters
//! - Request/response transformation
//! - Middleware chains
//! - Connection pooling
//! - Caching and rate limiting
//! - @zap decorator syntax for routing configuration

#![warn(missing_docs)]

pub mod proxy;
pub mod router;

#[cfg(feature = "batched")]
pub mod batched;

// Re-export commonly used types
pub use proxy::{
    ProxyBuilder,
    RequestHandler,
    ProxyConfig,
    RouteConfig,
    Middleware,
    Transform,
    ProxyError,
    ProxyResult,
};

// Convenience function to create a proxy with default settings
pub fn create_proxy() -> RequestHandler {
    proxy::create_basic_proxy()
}

// Convenience function to create a proxy with production settings
pub fn create_production_proxy() -> RequestHandler {
    proxy::create_production_proxy()
}

/// Configuration for the proxy server
pub mod config {
    pub use crate::proxy::{
        ProxyConfig,
        RouteConfig,
        TransformConfig,
        CacheConfig,
        RateLimitConfig,
    };
}

/// Middleware components
pub mod middleware {
    pub use crate::proxy::{
        LoggingMiddleware,
        HeaderTransform,
        RateLimiter,
        CacheMiddleware,
    };
}

/// Error types
pub mod error {
    pub use crate::proxy::{ProxyError, ProxyResult};
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::{Body, Request, Response, Method};

    #[tokio::test]
    async fn test_basic_proxy() {
        let handler = create_proxy();
        
        let req = Request::builder()
            .method(Method::GET)
            .uri("http://localhost:3000/api/test")
            .body(Body::empty())
            .unwrap();

        let res = handler.handle_request(req).await;
        assert!(res.status().is_success() || res.status().is_client_error());
    }

    #[tokio::test]
    async fn test_middleware_chain() {
        use middleware::LoggingMiddleware;

        let handler = ProxyBuilder::new()
            .with_middleware(LoggingMiddleware::new("test"))
            .build();

        let req = Request::builder()
            .method(Method::GET)
            .uri("http://localhost:3000/api/test")
            .body(Body::empty())
            .unwrap();

        let res = handler.handle_request(req).await;
        assert!(res.status().is_success() || res.status().is_client_error());
    }

    #[tokio::test]
    async fn test_transform() {
        use std::collections::HashMap;
        use middleware::HeaderTransform;

        let mut headers = HashMap::new();
        headers.insert("X-Test".to_string(), "test-value".to_string());

        let handler = ProxyBuilder::new()
            .with_transform(HeaderTransform::new(headers.clone(), HashMap::new()))
            .build();

        let req = Request::builder()
            .method(Method::GET)
            .uri("http://localhost:3000/api/test")
            .body(Body::empty())
            .unwrap();

        let res = handler.handle_request(req).await;
        assert!(res.status().is_success() || res.status().is_client_error());
    }
}
