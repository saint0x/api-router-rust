//! Proxy module for handling request forwarding
//! 
//! This module provides functionality to proxy HTTP requests through the Zap router.
//! It maintains connection pools, handles request parsing, and manages error recovery.

mod types;
mod parser;
mod pool;
mod handler;

pub use types::{ProxyConfig, ProxyError, ProxyResult};
pub use handler::ProxyHandler;

use hyper::{Body, Request, Response};
use std::sync::Arc;

/// Main proxy service that handles incoming requests
#[derive(Clone)]
pub struct Proxy {
    handler: Arc<ProxyHandler>,
}

impl Proxy {
    /// Create a new proxy instance with default configuration
    pub fn new() -> Self {
        Self::with_config(ProxyConfig::default())
    }

    /// Create a new proxy instance with custom configuration
    pub fn with_config(config: ProxyConfig) -> Self {
        Self {
            handler: Arc::new(ProxyHandler::new(config)),
        }
    }

    /// Handle an incoming request
    pub async fn handle_request(&self, req: Request<Body>) -> Response<Body> {
        self.handler.handle_request(req).await
    }

    /// Get proxy statistics
    pub async fn get_stats(&self) -> ProxyResult<String> {
        self.handler.get_stats().await
    }
}

impl Default for Proxy {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::StatusCode;

    #[tokio::test]
    async fn test_proxy_creation() {
        let proxy = Proxy::new();
        
        // Test invalid request
        let req = Request::builder()
            .uri("/invalid")
            .body(Body::empty())
            .unwrap();
        
        let response = proxy.handle_request(req).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_proxy_stats() {
        let proxy = Proxy::new();
        let stats = proxy.get_stats().await;
        assert!(stats.is_ok());
    }
}
