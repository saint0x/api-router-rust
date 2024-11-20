use std::sync::Arc;
use std::collections::HashMap;
use hyper::{Body, Request, Response};
use serde_json::Value;
use tokio::sync::Mutex;

use crate::proxy::Proxy;

type BoxFut = std::pin::Pin<Box<dyn std::future::Future<Output = Response<Body>> + Send>>;
type HandlerFn = Arc<dyn Fn(Request<Body>) -> BoxFut + Send + Sync>;
type Routes = Arc<Mutex<HashMap<(String, String), HandlerFn>>>;

/// Core router implementation
#[derive(Clone)]
pub struct Router {
    /// Core routing table
    routes: Routes,
    /// Optional proxy handler
    proxy: Option<Proxy>,
}

impl Router {
    /// Create a new router instance
    pub fn new() -> Self {
        Router {
            routes: Arc::new(Mutex::new(HashMap::new())),
            proxy: None,
        }
    }

    /// Enable proxy functionality
    pub fn with_proxy(mut self) -> Self {
        self.proxy = Some(Proxy::new());
        self
    }

    /// Add a route handler
    pub async fn add_route<F, Fut>(&self, method: &str, path: &str, handler: F)
    where
        F: Fn(Request<Body>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = Response<Body>> + Send + 'static,
    {
        let boxed_handler: HandlerFn = Arc::new(move |req| Box::pin(handler(req)));
        let mut routes = self.routes.lock().await;
        routes.insert((method.to_string(), path.to_string()), boxed_handler);
    }

    /// Handle an incoming request
    pub async fn handle_request(&self, req: Request<Body>) -> Response<Body> {
        let method = req.method().to_string();
        let path = req.uri().path().to_string();

        // Check if this is a proxy request (starts with proxy prefix)
        if path.starts_with("/proxy/") && self.proxy.is_some() {
            // Strip /proxy prefix and forward
            let mut req = req;
            *req.uri_mut() = path.trim_start_matches("/proxy").parse().unwrap();
            return self.proxy.as_ref().unwrap().handle_request(req).await;
        }

        // Handle normal routes
        let routes = self.routes.lock().await;
        if let Some(handler) = routes.get(&(method, path)) {
            handler(req).await
        } else {
            Response::builder()
                .status(hyper::StatusCode::NOT_FOUND)
                .body(Body::from("Not Found"))
                .unwrap()
        }
    }

    /// Create JSON response (core functionality)
    pub async fn json_response_direct(&self, json: &Value) -> Result<Response<Body>, Box<dyn std::error::Error + Send + Sync>> {
        let body = serde_json::to_vec(json)?;
        Ok(Response::builder()
            .header("Content-Type", "application/json")
            .body(Body::from(body))?)
    }

    /// Get proxy statistics if enabled
    pub async fn get_proxy_stats(&self) -> Option<String> {
        if let Some(proxy) = &self.proxy {
            proxy.get_stats().await.ok()
        } else {
            None
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}
