use hyper::{Body, Request, Response, Uri};
use std::error::Error;
use std::fmt;
use std::time::Duration;

/// Configuration for the proxy
#[derive(Clone, Debug)]
pub struct ProxyConfig {
    /// Connection timeout
    pub timeout: Duration,
    /// Maximum retries for failed requests
    pub max_retries: u32,
    /// Connection pool size
    pub pool_size: u32,
    /// Whether to preserve original headers
    pub preserve_headers: bool,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            max_retries: 3,
            pool_size: 100,
            preserve_headers: true,
        }
    }
}

/// Proxy-specific errors
#[derive(Debug)]
pub enum ProxyError {
    /// Error parsing the destination URL
    InvalidDestination(String),
    /// Error forwarding the request
    ForwardError(String),
    /// Timeout error
    Timeout(Duration),
    /// Connection error
    ConnectionError(String),
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyError::InvalidDestination(msg) => write!(f, "Invalid destination: {}", msg),
            ProxyError::ForwardError(msg) => write!(f, "Forward error: {}", msg),
            ProxyError::Timeout(duration) => write!(f, "Request timed out after {:?}", duration),
            ProxyError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
        }
    }
}

impl Error for ProxyError {}

/// Represents a proxied request with its destination
#[derive(Debug)]
pub struct ProxyRequest {
    /// Original request
    pub request: Request<Body>,
    /// Extracted destination URI
    pub destination: Uri,
    /// Original path without proxy prefix
    pub original_path: String,
}

/// Result type for proxy operations
pub type ProxyResult<T> = Result<T, ProxyError>;
