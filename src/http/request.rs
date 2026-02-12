

use super::shared::HTTPVersion;

pub enum HTTPMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}


pub struct HTTPRequest<'a> {
    method: HTTPMethod,
    uri: Vec<u8>,
    version: HTTPVersion,
    headers: HTTPHeaders,
    body: Option<Vec<u8>>,
}

