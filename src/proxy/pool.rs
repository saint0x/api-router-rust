use std::sync::Arc;
use std::time::Duration;
use hyper::client::HttpConnector;
use hyper::{Body, Client, Request, Response, Uri};
use hyper_tls::HttpsConnector;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use tracing::{error, debug};

use super::types::{ProxyConfig, ProxyResult, ProxyError};

const MAX_RETRIES: u32 = 3;
const RETRY_DELAY_MS: u64 = 100;

pub struct ProxyPool {
    client: Client<HttpsConnector<HttpConnector>>,
    config: ProxyConfig,
    semaphore: Arc<Semaphore>,
}

impl ProxyPool {
    pub fn new(config: ProxyConfig) -> Self {
        // Configure HTTPS connector with reasonable defaults
        let mut https = HttpsConnector::new();
        https.https_only(false); // Allow both HTTP and HTTPS

        // Configure client with connection pooling
        let client = Client::builder()
            .pool_idle_timeout(Duration::from_secs(30))
            .pool_max_idle_per_host(config.pool_size as usize)
            .retry_canceled_requests(true)
            .set_host(true)
            .build::<_, Body>(https);

        // Create semaphore for connection limiting
        let semaphore = Arc::new(Semaphore::new(config.pool_size as usize));

        ProxyPool { 
            client,
            config,
            semaphore,
        }
    }

    pub async fn forward_request(
        &self,
        uri: Uri,
        req: Request<Body>,
    ) -> ProxyResult<Response<Body>> {
        // Acquire semaphore permit for connection limiting
        let _permit = self.semaphore.acquire().await
            .map_err(|e| ProxyError::ForwardError(format!("Failed to acquire connection: {}", e)))?;

        let mut retries = 0;
        loop {
            match self.try_forward_request(uri.clone(), req.try_clone()?).await {
                Ok(response) => return Ok(response),
                Err(e) => {
                    if retries >= MAX_RETRIES {
                        return Err(e);
                    }
                    retries += 1;
                    debug!("Retry {}/{} after error: {}", retries, MAX_RETRIES, e);
                    tokio::time::sleep(Duration::from_millis(RETRY_DELAY_MS * retries as u64)).await;
                }
            }
        }
    }

    async fn try_forward_request(
        &self,
        uri: Uri,
        mut req: Request<Body>,
    ) -> ProxyResult<Response<Body>> {
        // Update request URI to destination
        *req.uri_mut() = uri;

        // Forward the request with timeout
        match timeout(
            Duration::from_secs(self.config.pool_timeout_secs),
            self.client.request(req)
        ).await {
            Ok(result) => {
                match result {
                    Ok(response) => Ok(response),
                    Err(e) => {
                        error!("Failed to forward request: {}", e);
                        Err(ProxyError::ForwardError(e.to_string()))
                    }
                }
            },
            Err(_) => {
                error!("Request timed out after {} seconds", self.config.pool_timeout_secs);
                Err(ProxyError::ForwardError("Request timed out".to_string()))
            }
        }
    }

    pub fn get_metrics(&self) -> ProxyPoolMetrics {
        ProxyPoolMetrics {
            available_connections: self.semaphore.available_permits(),
            max_connections: self.config.pool_size as usize,
        }
    }
}

#[derive(Debug)]
pub struct ProxyPoolMetrics {
    pub available_connections: usize,
    pub max_connections: usize,
}

// Extension trait for Request cloning
trait RequestExt {
    fn try_clone(&self) -> ProxyResult<Request<Body>>;
}

impl RequestExt for Request<Body> {
    fn try_clone(&self) -> ProxyResult<Request<Body>> {
        let mut builder = Request::builder()
            .method(self.method().clone())
            .uri(self.uri().clone());
        
        // Clone headers
        for (name, value) in self.headers() {
            builder = builder.header(name, value);
        }

        // Create empty body for now (actual body handling would need streaming support)
        Ok(builder
            .body(Body::empty())
            .map_err(|e| ProxyError::RequestBuildError(e.to_string()))?)
    }
}
