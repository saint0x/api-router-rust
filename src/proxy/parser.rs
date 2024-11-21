use hyper::{Body, Method, Request, Uri};
use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;
use tracing::debug;

use super::types::{
    ProxyError, ProxyRequest, ProxyResult, RouteConfig,
    PathParams, TransformConfig, CacheConfig, RateLimitConfig,
    SerializableMethod,
};

pub struct RequestParser;

impl RequestParser {
    pub fn new() -> Self {
        RequestParser
    }

    pub fn parse(req: Request<Body>) -> ProxyResult<ProxyRequest> {
        let uri = req.uri().clone();
        let path = uri.path();
        
        // Extract the remaining path after /api/
        let remaining_path = path.strip_prefix("/api/")
            .ok_or_else(|| ProxyError::InvalidPath("Path must start with /api/".to_string()))?;

        // Parse route configuration from path
        let (route_config, path_params) = Self::parse_route_config(req.method(), remaining_path)?;

        // Create the destination URI with processed path
        let destination = Self::build_destination_uri(remaining_path, &path_params)?;

        // Create a new request with cloned parts
        let new_req = Request::builder()
            .method(req.method().clone())
            .uri(destination.clone())
            .body(req.into_body())
            .map_err(|e| ProxyError::RequestBuildError(e.to_string()))?;

        Ok(ProxyRequest {
            request: new_req,
            destination,
            original_path: remaining_path.to_string(),
            route_config: Some(route_config),
            path_params: Some(path_params),
        })
    }

    fn parse_route_config(method: &Method, path: &str) -> ProxyResult<(RouteConfig, PathParams)> {
        // Extract path parameters using regex
        let params = Self::extract_path_params(path)?;
        
        // Look for @zap decorator in comments above the route
        let transforms = Self::parse_transforms(path)?;
        let cache = Self::parse_cache_config(path)?;
        let rate_limit = Self::parse_rate_limit(path)?;
        let middleware = Self::parse_middleware(path)?;

        let config = RouteConfig {
            method_wrapper: SerializableMethod::from(method.clone()),
            path: path.to_string(),
            middleware,
            transforms,
            cache,
            rate_limit,
        };

        Ok((config, params))
    }

    fn extract_path_params(path: &str) -> ProxyResult<PathParams> {
        let param_regex = Regex::new(r"\[([^\]]+)\]").unwrap();
        let wildcard_regex = Regex::new(r"\[\.\.\.([^\]]+)\]").unwrap();
        
        let mut params = HashMap::new();
        let mut wildcard = None;

        // Extract named parameters
        for cap in param_regex.captures_iter(path) {
            let param_name = cap[1].to_string();
            if !param_name.starts_with("...") {
                params.insert(param_name, String::new()); // Will be filled during routing
            }
        }

        // Extract wildcard if present
        if let Some(cap) = wildcard_regex.captures(path) {
            wildcard = Some(cap[1].to_string());
        }

        Ok(PathParams { params, wildcard })
    }

    fn parse_transforms(path: &str) -> ProxyResult<TransformConfig> {
        // Look for @zap transform decorator
        let transform_regex = Regex::new(r"@zap\s*\(\s*transform\s*=\s*(\{[^}]+\})\s*\)").unwrap();
        if let Some(cap) = transform_regex.captures(path) {
            let config = cap[1].to_string();
            let value: Value = serde_json::from_str(&config)
                .map_err(|e| ProxyError::InvalidDecorator(format!("Invalid transform config: {}", e)))?;
            
            // Parse transform rules
            if let Value::Object(obj) = value {
                let request = obj.get("request").map(|v| serde_json::from_value(v.clone()))
                    .transpose()
                    .map_err(|e| ProxyError::InvalidDecorator(format!("Invalid request transform: {}", e)))?;
                
                let response = obj.get("response").map(|v| serde_json::from_value(v.clone()))
                    .transpose()
                    .map_err(|e| ProxyError::InvalidDecorator(format!("Invalid response transform: {}", e)))?;

                return Ok(TransformConfig { request, response });
            }
        }

        Ok(TransformConfig { request: None, response: None })
    }

    fn parse_cache_config(path: &str) -> ProxyResult<Option<CacheConfig>> {
        // Look for @zap cache decorator
        let cache_regex = Regex::new(r"@zap\s*\(\s*cache\s*=\s*(\{[^}]+\})\s*\)").unwrap();
        if let Some(cap) = cache_regex.captures(path) {
            let config = cap[1].to_string();
            let cache_config: CacheConfig = serde_json::from_str(&config)
                .map_err(|e| ProxyError::InvalidDecorator(format!("Invalid cache config: {}", e)))?;
            return Ok(Some(cache_config));
        }

        Ok(None)
    }

    fn parse_rate_limit(path: &str) -> ProxyResult<Option<RateLimitConfig>> {
        // Look for @zap rateLimit decorator
        let rate_limit_regex = Regex::new(r"@zap\s*\(\s*rateLimit\s*=\s*(\{[^}]+\})\s*\)").unwrap();
        if let Some(cap) = rate_limit_regex.captures(path) {
            let config = cap[1].to_string();
            let rate_limit_config: RateLimitConfig = serde_json::from_str(&config)
                .map_err(|e| ProxyError::InvalidDecorator(format!("Invalid rate limit config: {}", e)))?;
            return Ok(Some(rate_limit_config));
        }

        Ok(None)
    }

    fn parse_middleware(path: &str) -> ProxyResult<Vec<String>> {
        // Look for @zap middleware decorator
        let middleware_regex = Regex::new(r"@zap\s*\(\s*middleware\s*=\s*\[(.*?)\]\s*\)").unwrap();
        if let Some(cap) = middleware_regex.captures(path) {
            let middleware_list = cap[1].to_string();
            let middleware: Vec<String> = serde_json::from_str(&format!("[{}]", middleware_list))
                .map_err(|e| ProxyError::InvalidDecorator(format!("Invalid middleware list: {}", e)))?;
            return Ok(middleware);
        }

        Ok(Vec::new())
    }

    fn build_destination_uri(path: &str, params: &PathParams) -> ProxyResult<Uri> {
        // Replace path parameters with actual values
        let mut processed_path = path.to_string();
        for (name, value) in &params.params {
            processed_path = processed_path.replace(&format!("[{}]", name), value);
        }

        // Handle wildcard if present
        if let Some(wildcard) = &params.wildcard {
            processed_path = processed_path.replace(&format!("[...{}]", wildcard), "");
        }

        let destination = format!("http://localhost:3000/api/{}", processed_path);
        Uri::try_from(destination.as_str())
            .map_err(|e| ProxyError::InvalidUri(e.to_string()))
    }
}
