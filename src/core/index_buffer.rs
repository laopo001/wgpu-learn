use crate::app::App;
pub struct IndexBuffer {
    pub buffer: Vec<u8>,
    pub length: usize,
    pub wgpu_index_buffer: Option<wgpu::Buffer>,
}
impl IndexBuffer {
    pub fn new(buffer: Vec<u8>, length: usize) -> Self {
        return IndexBuffer {
            buffer,
            wgpu_index_buffer: None,
            length,
        };
    }
    pub fn get_wgpu_index_buffer<'a>(&'a mut self, app: &App) -> &'a wgpu::Buffer {
        if self.wgpu_index_buffer.is_some() {
            return self.wgpu_index_buffer.as_ref().unwrap();
        }
        let b = app
            .device
            .create_buffer_with_data(self.buffer.as_slice(), wgpu::BufferUsage::INDEX);
        self.wgpu_index_buffer = Some(b);
        return self.wgpu_index_buffer.as_ref().unwrap();
    }
}
