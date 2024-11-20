use hyper::{Body, Request, Uri};
use std::str::FromStr;
use crate::proxy::types::{ProxyError, ProxyRequest, ProxyResult};

/// Extracts destination information from the request URL
pub struct UrlParser;

impl UrlParser {
    /// Parse a request URL and extract the destination
    /// Format: /destination-url/path
    /// Example: /https://api.example.com/data -> forwards to https://api.example.com/data
    pub fn parse(req: Request<Body>) -> ProxyResult<ProxyRequest> {
        let uri = req.uri();
        let path = uri.path();

        // Remove leading slash
        let path = path.trim_start_matches('/');

        // Find the first occurrence of the next slash
        let (destination, remaining_path) = match path.find('/') {
            Some(idx) => {
                let (dest, path) = path.split_at(idx);
                (dest, path)
            }
            None => return Err(ProxyError::InvalidDestination(
                "No destination path provided".to_string()
            )),
        };

        // Parse the destination into a URI
        let destination = Uri::from_str(destination)
            .map_err(|e| ProxyError::InvalidDestination(e.to_string()))?;

        Ok(ProxyRequest {
            request: req,
            destination,
            original_path: remaining_path.to_string(),
        })
    }

    /// Reconstruct the full destination URL
    pub fn build_destination_url(proxy_req: &ProxyRequest) -> ProxyResult<Uri> {
        let dest = proxy_req.destination.clone();
        
        // Combine destination with original path
        let full_url = format!(
            "{}{}{}",
            dest,
            if proxy_req.original_path.starts_with('/') { "" } else { "/" },
            proxy_req.original_path
        );

        Uri::from_str(&full_url)
            .map_err(|e| ProxyError::InvalidDestination(e.to_string()))
    }

    /// Validate the destination URL
    pub fn validate_destination(uri: &Uri) -> ProxyResult<()> {
        // Ensure scheme is present
        if uri.scheme().is_none() {
            return Err(ProxyError::InvalidDestination(
                "Missing scheme (http/https)".to_string()
            ));
        }

        // Ensure host is present
        if uri.host().is_none() {
            return Err(ProxyError::InvalidDestination(
                "Missing host".to_string()
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hyper::Request;

    #[test]
    fn test_parse_valid_url() {
        let req = Request::builder()
            .uri("/https://api.example.com/data")
            .body(Body::empty())
            .unwrap();

        let result = UrlParser::parse(req);
        assert!(result.is_ok());

        let proxy_req = result.unwrap();
        assert_eq!(proxy_req.destination.host().unwrap(), "api.example.com");
        assert_eq!(proxy_req.original_path, "/data");
    }

    #[test]
    fn test_parse_invalid_url() {
        let req = Request::builder()
            .uri("/invalid-url")
            .body(Body::empty())
            .unwrap();

        let result = UrlParser::parse(req);
        assert!(result.is_err());
    }
}
