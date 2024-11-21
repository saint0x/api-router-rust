use hyper::{Body, Request};
use super::types::{ProxyError, ProxyRequest, ProxyResult};

pub struct RequestParser;

impl RequestParser {
    pub fn parse(req: Request<Body>) -> ProxyResult<ProxyRequest> {
        Ok(ProxyRequest { request: req })
    }
}
