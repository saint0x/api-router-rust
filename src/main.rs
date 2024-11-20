mod router;

use std::convert::Infallible;
use std::net::SocketAddr;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use serde_json::json;
use tokio::time::sleep;
use std::time::Duration;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

use crate::router::Router;

async fn handle_request(
    router: Router,
    req: Request<Body>,
) -> Result<Response<Body>, Infallible> {
    Ok(router.handle_request(req).await)
}

#[tokio::main]
async fn main() {
    // Initialize logging with timestamp
    FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_target(false)
        .init();

    // Create router instance
    let router = Router::new();
    let router_clone = router.clone();

    // Simple route - baseline performance test
    // Using static str for route path to avoid allocations
    router.add_route("GET", "/ping", move |_req| {
        let router = router_clone.clone();
        async move {
            // Pre-compute response to minimize allocations
            router.json_response_direct(&json!({
                "message": "pong",
                "time": chrono::Utc::now()
            }))
            .await
            .unwrap_or_else(|_| Response::builder()
                .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap())
        }
    });

    // Medium complexity route - optimized for performance
    let router_clone = router.clone();
    router.add_route("GET", "/api/v1/data", move |_req| {
        let router = router_clone.clone();
        async move {
            // Pre-compute timestamp to avoid multiple calls
            let timestamp = chrono::Utc::now();
            
            // Using static values where possible to enable compiler optimizations
            static VERSION: &str = "2.0";
            static TAGS: [&str; 2] = ["performance", "optimized"];
            
            router.json_response_direct(&json!({
                "id": 123,
                "timestamp": timestamp,
                "data": {
                    "status": "active",
                    "metrics": {
                        "value": 42,
                        "unit": "ms"
                    },
                    "tags": TAGS,
                    "version": VERSION
                }
            }))
            .await
            .unwrap_or_else(|_| Response::builder()
                .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap())
        }
    });

    // Complex route - heavy processing simulation
    let router_clone = router.clone();
    router.add_route("POST", "/api/v1/process", move |mut req| {
        let router = router_clone.clone();
        async move {
            // Simulate complex processing
            sleep(Duration::from_millis(50)).await;

            // Parse request body with zero-copy where possible
            let body_bytes = hyper::body::to_bytes(req.body_mut())
                .await
                .unwrap_or_default();

            let request_data: serde_json::Value = match serde_json::from_slice(&body_bytes) {
                Ok(data) => data,
                Err(_) => {
                    return Response::builder()
                        .status(hyper::StatusCode::BAD_REQUEST)
                        .body(Body::from("Invalid request body"))
                        .unwrap();
                }
            };

            // Pre-compute timestamp
            let timestamp = chrono::Utc::now();
            let request_id = format!("req_{}", timestamp.timestamp_nanos_opt().unwrap_or(0));

            router.json_response_direct(&json!({
                "status": "processed",
                "timestamp": timestamp,
                "requestId": request_id,
                "processed": request_data
            }))
            .await
            .unwrap_or_else(|_| Response::builder()
                .status(hyper::StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Internal Server Error"))
                .unwrap())
        }
    });

    // Set up server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    info!("Starting server on {}", addr);

    let make_svc = make_service_fn(move |_conn| {
        let router = router.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req| {
                handle_request(router.clone(), req)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);

    // Run the server
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
