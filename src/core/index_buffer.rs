pub struct IndexBuffer {
    pub buffer: Vec<u8>,
}
impl IndexBuffer {
    pub fn new(buffer: Vec<u8>) -> Self {
        return IndexBuffer { buffer };
    }
    pub fn get_wgpu_index_buffer() {}
}
