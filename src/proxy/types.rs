use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ProxyError {
    InvalidUri(String),
    RequestBuildError(String),
    ForwardError(String),
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyError::InvalidUri(msg) => write!(f, "Invalid URI: {}", msg),
            ProxyError::RequestBuildError(msg) => write!(f, "Failed to build request: {}", msg),
            ProxyError::ForwardError(msg) => write!(f, "Failed to forward request: {}", msg),
        }
    }
}

impl Error for ProxyError {}

pub type ProxyResult<T> = Result<T, ProxyError>;
