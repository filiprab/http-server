pub enum HttpVersion {
    HTTP0_9,
    HTTP1_0,
    HTTP1_1,
    HTTP2,
    HTTP3,
}

#[derive(Debug, Clone, Copy)]
pub struct HeaderIndex {
    pub name: (usize, usize),
    pub value: (usize, usize),
}