use crate::config::Attrib;
use crate::core::shader_var::{VertexVar, VertexVars};
use std::ops::{Deref, DerefMut};
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct VertexType {
    pub attrib: Attrib,
    pub size: u32,
}

pub struct VertexAttribData {
    pub vertex_type: VertexType,
    pub offset: u32,
    // pub stride: u32,
    pub length: u32,
}

impl Deref for VertexAttribData {
    type Target = VertexType;
    fn deref<'a>(&'a self) -> &'a VertexType {
        &self.vertex_type
    }
}

pub struct VertexFormat {
    pub elements: Vec<VertexAttribData>,
    pub stride: u32,
    pub hasUv0: bool,
    pub vertex_vars: VertexVars,
}

impl VertexFormat {
    pub fn new(vertex_types: Vec<VertexType>) -> Self {
        let mut offset = 0;
        let mut hasUv0 = false;
        let mut elements = vec![];
        let mut vertex_vars = VertexVars::new();
        for item in vertex_types {
            let mem_size = std::mem::align_of::<f32>() as u32;

            let vertex_data = VertexAttribData {
                length: item.size * mem_size,
                vertex_type: item,
                offset: offset,
                // stride: 0,
            };
            offset += vertex_data.length;
            if vertex_data.attrib == Attrib::TEXCOORD0 {
                hasUv0 = true;
            }
            elements.push(vertex_data);
            let f: wgpu::VertexFormat = match item.size {
                1 => wgpu::VertexFormat::Float,
                2 => wgpu::VertexFormat::Float2,
                3 => wgpu::VertexFormat::Float3,
                4 => wgpu::VertexFormat::Float4,
                _ => panic!("错误"),
            };
            vertex_vars.set(item.attrib, VertexVar { offset, format: f });
        }
        let stride = offset;
        // elements.iter_mut().for_each(|x| {
        //     x.stride = stride;
        // });
        return VertexFormat {
            elements,
            stride,
            hasUv0,
            vertex_vars,
        };
    }
}
