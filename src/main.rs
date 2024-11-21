use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use api_router_rust::proxy::ZapDecorator;

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Health check endpoint
    if req.uri().path() == "/health" {
        return Ok(Response::new(Body::from("OK")));
    }

    // Create proxy with basic configuration
    let proxy = ZapDecorator::basic();

    // Forward the request through our proxy
    match proxy.apply(req).await {
        Ok(response) => Ok(response),
        Err(_) => Ok(Response::builder()
            .status(500)
            .body(Body::from("Proxy error"))
            .unwrap()),
    }
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Proxy server address
    let addr = SocketAddr::from(([127, 0, 0, 1], 3003));

    // Create service
    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Proxy server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
