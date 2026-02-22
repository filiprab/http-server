pub enum HttpVersion {
    HTTP0_9,
    HTTP1_0,
    HTTP1_1,
    HTTP2,
    HTTP3,
}

#[derive(Debug, Clone, Copy)]
struct HeaderIndex {
    name: (usize, usize),
    value: (usize, usize),
}