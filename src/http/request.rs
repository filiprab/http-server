

use super::shared::{HttpVersion, HeaderIndex};

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


pub struct HttpRequest {
    buffer: Vec<u8>,
    method: HttpMethod,
    uri: (usize, usize),
    version: HttpVersion,
    headers: Vec<HeaderIndex>,
    body: (usize, usize),
    content_length: usize,
}

impl HttpRequest<'a> {
    pub fn get_header_values(&self, key_name: &str) -> impl Iterator<Item = &'a str> + 'a {
        let key_name_bytes = key_name.as_bytes();
        
        self.headers.iter().filter_map(move |header| {
            let name_raw = &self.buffer[header.name.0 .. header.name.1];
            
            if key_name_bytes.eq_ignore_ascii_case(name_raw) {
                let value_raw = &self.buffer[header.value.0 .. header.value.1];
                
                std::str::from_utf8(value_raw).ok()
            } else {
                None
            }
        })
    }
}

