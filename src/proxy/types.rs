use hyper::{Body, Method, Request, Uri};
use std::error::Error;
use std::fmt;
use std::collections::HashMap;
use std::str::FromStr;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::{self, Visitor};
use serde_json::Value;

#[derive(Debug)]
pub enum ProxyError {
    InvalidPath(String),
    InvalidUri(String),
    RequestBuildError(String),
    ForwardError(String),
    InvalidDecorator(String),
    MiddlewareError(String),
    TransformError(String),
}

impl fmt::Display for ProxyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProxyError::InvalidPath(msg) => write!(f, "Invalid path: {}", msg),
            ProxyError::InvalidUri(msg) => write!(f, "Invalid URI: {}", msg),
            ProxyError::RequestBuildError(msg) => write!(f, "Failed to build request: {}", msg),
            ProxyError::ForwardError(msg) => write!(f, "Failed to forward request: {}", msg),
            ProxyError::InvalidDecorator(msg) => write!(f, "Invalid decorator: {}", msg),
            ProxyError::MiddlewareError(msg) => write!(f, "Middleware error: {}", msg),
            ProxyError::TransformError(msg) => write!(f, "Transform error: {}", msg),
        }
    }
}

impl Error for ProxyError {}

pub type ProxyResult<T> = Result<T, ProxyError>;

// Custom Method wrapper for serialization
#[derive(Debug, Clone)]
pub struct SerializableMethod(Method);

impl From<Method> for SerializableMethod {
    fn from(method: Method) -> Self {
        SerializableMethod(method)
    }
}

impl From<SerializableMethod> for Method {
    fn from(method: SerializableMethod) -> Self {
        method.0
    }
}

impl Serialize for SerializableMethod {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.0.as_str())
    }
}

struct MethodVisitor;

impl<'de> Visitor<'de> for MethodVisitor {
    type Value = SerializableMethod;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string representing an HTTP method")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Method::from_str(value)
            .map(SerializableMethod)
            .map_err(|_| E::custom(format!("invalid HTTP method: {}", value)))
    }
}

impl<'de> Deserialize<'de> for SerializableMethod {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(MethodVisitor)
    }
}

// Route configuration from @zap decorator
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    #[serde(rename = "method")]
    pub method_wrapper: SerializableMethod,
    pub path: String,
    pub middleware: Vec<String>,
    pub transforms: TransformConfig,
    pub cache: Option<CacheConfig>,
    pub rate_limit: Option<RateLimitConfig>,
}

impl RouteConfig {
    pub fn method(&self) -> Method {
        self.method_wrapper.clone().into()
    }
}

// Request/Response transformation configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransformConfig {
    pub request: Option<TransformRules>,
    pub response: Option<TransformRules>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformRules {
    pub headers: Option<HashMap<String, String>>,
    pub query: Option<HashMap<String, String>>,
    pub body: Option<Value>,
}

// Caching configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    pub ttl_seconds: u64,
    pub vary_by: Vec<String>,
}

// Rate limiting configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub requests_per_second: u32,
    pub burst: u32,
}

// Extracted path parameters
#[derive(Debug, Clone)]
pub struct PathParams {
    pub params: HashMap<String, String>,
    pub wildcard: Option<String>,
}

// Proxy request with additional context
#[derive(Debug)]
pub struct ProxyRequest {
    pub request: Request<Body>,
    pub destination: Uri,
    pub original_path: String,
    pub route_config: Option<RouteConfig>,
    pub path_params: Option<PathParams>,
}

// Proxy configuration
#[derive(Debug, Clone)]
pub struct ProxyConfig {
    pub pool_size: u32,
    pub pool_timeout_secs: u64,
    pub default_transforms: Option<TransformConfig>,
    pub global_middleware: Vec<String>,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            pool_size: 32,
            pool_timeout_secs: 60,
            default_transforms: None,
            global_middleware: Vec::new(),
        }
    }
}

// Middleware trait
#[async_trait::async_trait]
pub trait Middleware: Send + Sync {
    async fn handle(&self, req: &mut ProxyRequest) -> ProxyResult<()>;
}

// Transform trait
#[async_trait::async_trait]
pub trait Transform: Send + Sync {
    async fn transform_request(&self, req: &mut Request<Body>) -> ProxyResult<()>;
    async fn transform_response(&self, res: &mut hyper::Response<Body>) -> ProxyResult<()>;
}
