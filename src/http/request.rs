use super::shared::{HttpVersion, HeaderIndex};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct HttpRequest {
    buffer: Vec<u8>,
    method: HttpMethod,
    uri: (usize, usize),
    version: HttpVersion,
    headers: Vec<HeaderIndex>,
    body: (usize, usize),
    content_length: usize,
}

impl HttpRequest {
    pub fn get_header_values<'a>(&'a self, key_name: &'a str) -> impl Iterator<Item = &'a str> + 'a {
        let key_name_bytes = key_name.as_bytes();
        
        self.headers.iter().filter_map(move |header| {
            let name_raw = &self.buffer[header.name().0 .. header.name().1];
            
            if key_name_bytes.eq_ignore_ascii_case(name_raw) {
                let value_raw = &self.buffer[header.value().0 .. header.value().1];
                
                std::str::from_utf8(value_raw).ok()
            } else {
                None
            }
        })
    }
}

#[derive(Debug)]
pub enum ParseResult {
    Complete(HttpRequest),
    Partial,
    Error(String), // Or a custom error enum
}

pub enum ParserState {
    RequestLine,
    Headers,
    Body,
    Finished,
}

pub struct HttpRequestParser {
    buffer: Vec<u8>,
    state: ParserState,
    cursor: usize,
}

impl HttpRequestParser {
    pub fn new() -> Self {
        Self {
            buffer: Vec::with_capacity(4096),
            state: ParserState::RequestLine,
            cursor: 0,
        }
    }
    
    pub fn push_bytes(&mut self, data: &[u8]) -> ParseResult {
        self.buffer.extend_from_slice(data);
        
        //TODO
    }
}
