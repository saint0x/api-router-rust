use std::net::SocketAddr;
use std::str::FromStr;
use std::env;

use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .compact()
        .init();

    // Get port from environment or use default
    let port = env::var("PORT").unwrap_or_else(|_| "3001".to_string());
    let addr = SocketAddr::from_str(&format!("0.0.0.0:{}", port)).unwrap();

    info!("Server running on {}", addr);

    // Start server
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            info!("Shutting down gracefully");
        }
    }
}
