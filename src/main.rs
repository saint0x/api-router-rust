use std::net::SocketAddr;
use std::str::FromStr;
use std::env;
use std::collections::HashMap;
use std::sync::Arc;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use rust_router::proxy::{
    ProxyBuilder, LoggingMiddleware, HeaderTransform,
    RateLimiter, CacheMiddleware,
};

async fn handle_request(
    req: Request<Body>,
    handler: Arc<rust_router::proxy::RequestHandler>,
) -> Result<Response<Body>, hyper::Error> {
    Ok(handler.handle_request(req).await)
}

#[tokio::main]
async fn main() {
    // Initialize logging with better formatting
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_ansi(true)
        .compact()
        .init();

    // Get configuration from environment
    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let pool_size = env::var("POOL_SIZE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(32);
    let timeout_secs = env::var("TIMEOUT_SECS")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);

    // Configure standard headers
    let mut request_headers = HashMap::new();
    request_headers.insert(
        "X-Proxy-Request-ID".to_string(),
        uuid::Uuid::new_v4().to_string(),
    );

    let mut response_headers = HashMap::new();
    response_headers.insert(
        "X-Powered-By".to_string(),
        "Rust API Router".to_string(),
    );

    // Build proxy handler with all components
    let handler = Arc::new(ProxyBuilder::new()
        .with_pool_size(pool_size)
        .with_timeout(timeout_secs)
        // Add logging middleware
        .with_middleware(LoggingMiddleware::new("proxy"))
        // Add rate limiting
        .with_middleware(RateLimiter::new(60, 1000)) // 1000 req/min
        // Add caching for GET requests
        .with_middleware(CacheMiddleware::new(300)) // 5 min cache
        // Add header transform
        .with_transform(HeaderTransform::new(
            request_headers,
            response_headers,
        ))
        .build());

    // Create service
    let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();
    let make_svc = make_service_fn(move |_conn| {
        let handler = handler.clone();
        async move {
            Ok::<_, hyper::Error>(service_fn(move |req| {
                let handler = handler.clone();
                handle_request(req, handler)
            }))
        }
    });

    // Create and run server with proper configuration
    let server = Server::bind(&addr)
        .tcp_nodelay(true)
        .serve(make_svc);

    info!("Proxy server running on {}", addr);
    info!("Configuration:");
    info!("  Pool Size: {}", pool_size);
    info!("  Timeout: {}s", timeout_secs);
    info!("  Rate Limit: 1000 req/min");
    info!("  Cache TTL: 300s");

    if let Err(e) = server.await {
        eprintln!("Server error: {}", e);
    }
}
