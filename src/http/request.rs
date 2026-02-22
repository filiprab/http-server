

use super::shared::{HttpVersion, HttpHeaders};

pub enum HttpMethod {
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
    method: HttpMethod,
    uri: Vec<u8>,
    version: HttpVersion,
    headers: ,
    body: Option<Vec<u8>>,
}

