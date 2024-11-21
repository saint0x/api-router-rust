use hyper::{Body, Request, Response, Client};
use hyper_tls::HttpsConnector;
use std::sync::Arc;
use crate::proxy::types::ProxyResult;

/// The @zap decorator - pure routing without pooling or batching
#[derive(Clone)]
pub struct ZapDecorator {
    client: Arc<Client<HttpsConnector<hyper::client::HttpConnector>>>,
}

impl ZapDecorator {
    /// Forward request through our router
    pub async fn apply(&self, req: Request<Body>) -> ProxyResult<Response<Body>> {
        let uri = req.uri();
        let path = uri.path();
        
        // Extract URL from @zap prefix
        // Remove "@zap/" prefix and get the actual URL
        if !path.starts_with("/@zap/") {
            return Err(crate::proxy::types::ProxyError::InvalidUri(
                "URL must start with @zap/".to_string()
            ));
        }

        // Get the actual URL (everything after @zap/)
        let actual_url = &path[6..];

        // Build destination URL with query params
        let destination = if let Some(query) = uri.query() {
            format!("https://{}{}", actual_url, query)
        } else {
            format!("https://{}", actual_url)
        };

        // Forward request
        let new_req = Request::builder()
            .method(req.method().clone())
            .uri(destination)
            .body(req.into_body())
            .map_err(|e| crate::proxy::types::ProxyError::ForwardError(e.to_string()))?;

        self.client.request(new_req).await
            .map_err(|e| crate::proxy::types::ProxyError::ForwardError(e.to_string()))
    }

    /// Create a basic decorator without any pooling
    pub fn basic() -> Self {
        let https = HttpsConnector::new();
        let client = Client::builder()
            .pool_idle_timeout(std::time::Duration::from_secs(0)) // Disable connection pooling
            .pool_max_idle_per_host(0) // No idle connections
            .build::<_, hyper::Body>(https);

        ZapDecorator {
            client: Arc::new(client),
        }
    }
}
