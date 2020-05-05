use crate::core::vertex_format::VertexFormat;
pub struct VertexBuffer {
    pub buffer: Vec<u8>,
    pub format: VertexFormat,
}
impl VertexBuffer {
    pub fn new(buffer: Vec<u8>, format: VertexFormat) -> Self {
        return VertexBuffer { buffer, format };
    }
    pub fn get_wgpu_vertex_buffer() {}
}
