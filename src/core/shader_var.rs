use crate::config::{Attrib, Uniform};
use std::ops::Range;
pub enum UniformBindingResource {
    Buffer {
        buffer: wgpu::Buffer,
        range: Range<wgpu::BufferAddress>,
    },
    Sampler(wgpu::Sampler),
    TextureView(wgpu::TextureView),
}

pub struct UniformVar {
    pub resource: UniformBindingResource,
    pub visibility: wgpu::ShaderStage,
    pub ty: wgpu::BindingType,
}

pub struct UniformVars {
    pub vars: Vec<Option<UniformVar>>,
}
impl UniformVars {
    pub fn new() -> Self {
        let LEN = 4;
        let mut vars = vec![];
        for i in 0..4 {
            vars.push(None);
        }
        UniformVars { vars }
    }
    pub fn set(&mut self, t: Uniform, var: UniformVar) {
        self.vars[t as usize] = Some(var);
    }
}

#[derive(Debug)]
pub struct VertexVar {
    pub format: wgpu::VertexFormat,
    pub offset: u32,
}
#[derive(Debug)]
pub struct VertexVars {
    pub vars: Vec<Option<VertexVar>>,
}
impl VertexVars {
    pub fn new() -> Self {
        let LEN = 4;
        let mut vars = vec![];
        for i in 0..4 {
            vars.push(None);
        }
        VertexVars { vars }
    }
    pub fn set(&mut self, t: Attrib, var: VertexVar) {
        self.vars[t as usize] = Some(var);
    }
}
