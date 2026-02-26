#[derive(Debug)]
pub enum HttpVersion {
    HTTP0_9,
    HTTP1_0,
    HTTP1_1,
    HTTP2,
    HTTP3,
}

#[derive(Debug, Clone, Copy)]
pub struct HeaderIndex {
    name: (usize, usize),
    value: (usize, usize),
}

impl HeaderIndex {
    pub fn name(&self) -> (usize, usize){
        self.name
    }
    
    pub fn value(&self) -> (usize, usize){
        self.value
    }
}