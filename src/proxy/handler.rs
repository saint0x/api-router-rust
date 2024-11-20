use std::sync::Arc;
use hyper::{Body, Request, Response, StatusCode};
use tokio::time::timeout;
use tracing::{error, info, warn};

use crate::proxy::{
    parser::UrlParser,
    pool::ConnectionPool,
    types::{ProxyConfig, ProxyError, ProxyResult},
};

/// Handles proxying requests to their destinations
pub struct ProxyHandler {
    /// Connection pool for reuse
    pool: Arc<ConnectionPool>,
    /// Configuration
    config: ProxyConfig,
}

impl ProxyHandler {
    /// Create a new proxy handler
    pub fn new(config: ProxyConfig) -> Self {
        Self {
            pool: Arc::new(ConnectionPool::new(config.clone())),
            config,
        }
    }

    /// Handle an incoming request
    pub async fn handle_request(&self, req: Request<Body>) -> Response<Body> {
        match self.proxy_request(req).await {
            Ok(response) => response,
            Err(e) => self.handle_error(e),
        }
    }

    /// Proxy the request to its destination
    async fn proxy_request(&self, req: Request<Body>) -> ProxyResult<Response<Body>> {
        // Parse the request URL
        let proxy_req = UrlParser::parse(req)?;
        
        // Validate destination
        UrlParser::validate_destination(&proxy_req.destination)?;

        // Get the full destination URL
        let destination_url = UrlParser::build_destination_url(&proxy_req)?;
        
        // Get client from pool
        let client = self.pool
            .get_client(proxy_req.destination.host().unwrap())
            .await?;

        // Build forwarded request
        let mut forward_req = Request::builder()
            .method(proxy_req.request.method())
            .uri(destination_url);

        // Copy headers if configured
        if self.config.preserve_headers {
            for (key, value) in proxy_req.request.headers() {
                forward_req = forward_req.header(key, value);
            }
        }

        // Add proxy headers
        forward_req = forward_req
            .header("X-Forwarded-By", "zap.rs")
            .header("X-Forwarded-Proto", proxy_req.destination.scheme_str().unwrap_or("http"));

        // Forward the request with timeout
        let forward_req = forward_req
            .body(proxy_req.request.into_body())
            .map_err(|e| ProxyError::ForwardError(e.to_string()))?;

        let response = timeout(
            self.config.timeout,
            client.request(forward_req)
        ).await
        .map_err(|_| ProxyError::Timeout(self.config.timeout))?
        .map_err(|e| ProxyError::ForwardError(e.to_string()))?;

        Ok(response)
    }

    /// Handle proxy errors
    fn handle_error(&self, error: ProxyError) -> Response<Body> {
        let (status, message) = match &error {
            ProxyError::InvalidDestination(_) => (
                StatusCode::BAD_REQUEST,
                "Invalid destination URL"
            ),
            ProxyError::Timeout(_) => (
                StatusCode::GATEWAY_TIMEOUT,
                "Request timed out"
            ),
            ProxyError::ForwardError(_) => (
                StatusCode::BAD_GATEWAY,
                "Error forwarding request"
            ),
            ProxyError::ConnectionError(_) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "Connection error"
            ),
        };

        error!(?error, "Proxy error occurred");

        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(Body::from(format!(
                r#"{{"error":"{}", "message":"{}"}}"#,
                status.as_str(),
                message
            )))
            .unwrap_or_else(|_| Response::new(Body::empty()))
    }

    /// Get statistics about the connection pool
    pub async fn get_stats(&self) -> ProxyResult<String> {
        let count = self.pool.client_count().await;
        Ok(format!("Active connections: {}", count))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_proxy_handler() {
        let config = ProxyConfig {
            timeout: Duration::from_secs(30),
            pool_size: 10,
            max_retries: 3,
            preserve_headers: true,
        };

        let handler = ProxyHandler::new(config);

        // Test invalid URL
        let req = Request::builder()
            .uri("/invalid-url")
            .body(Body::empty())
            .unwrap();

        let response = handler.handle_request(req).await;
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }
}
