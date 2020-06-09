use crate::app::App;
#[derive(DebugStub)]
pub struct IndexBuffer {
    pub buffer: Vec<u8>,
    pub type_size: usize,
    #[debug_stub = "wgpu::Buffer"]
    pub wgpu_index_buffer: Option<wgpu::Buffer>,
    pub size: wgpu::IndexFormat,
    pub length: usize,
}

impl IndexBuffer {
    pub fn new(buffer: Vec<u8>, type_size: usize) -> Self {
        let length = buffer.len() / type_size;
        let size = match type_size {
            2 => wgpu::IndexFormat::Uint16,
            4 => wgpu::IndexFormat::Uint32,
            _ => panic!("wgpu::IndexFormat 只支持 u16 和 u32"),
        };
        return IndexBuffer {
            buffer,
            wgpu_index_buffer: None,
            type_size,
            size,
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
