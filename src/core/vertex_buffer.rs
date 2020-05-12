use crate::app::App;
use crate::core::vertex_format::VertexFormat;

pub struct VertexBuffer {
    pub buffer: Vec<u8>,
    pub format: VertexFormat,
    pub wgpu_vertex_buffer: Option<wgpu::Buffer>,
}
impl VertexBuffer {
    pub fn new(buffer: Vec<u8>, format: VertexFormat) -> Self {
        return VertexBuffer {
            buffer,
            format,
            wgpu_vertex_buffer: None,
        };
    }
    pub fn get_wgpu_vertex_buffer(&mut self, app: &App) -> &wgpu::Buffer {
        if self.wgpu_vertex_buffer.is_some() {
            return self.wgpu_vertex_buffer.as_ref().unwrap();
        }
        let b = app
            .device
            .create_buffer_with_data(self.buffer.as_slice(), wgpu::BufferUsage::VERTEX);
        self.wgpu_vertex_buffer = Some(b);
        return self.wgpu_vertex_buffer.as_ref().unwrap();
    }
}
