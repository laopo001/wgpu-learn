use crate::config::Attrib;
use crate::core::shader_var::{VertexVar, VertexVars};
use std::ops::{Deref, DerefMut};
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub struct VertexType {
    pub attrib: Attrib,
    pub size: u32,
}

// pub struct VertexAttribData {
//     pub vertex_type: VertexType,
//     pub offset: u32,
//     // pub stride: u32,
//     pub length: u32,
// }

// impl Deref for VertexAttribData {
//     type Target = VertexType;
//     fn deref<'a>(&'a self) -> &'a VertexType {
//         &self.vertex_type
//     }
// }
#[derive(Debug)]
pub struct VertexFormat {
    pub stride: u32,
    pub vertex_vars: VertexVars,
}

impl VertexFormat {
    pub fn new(vertex_types: Vec<VertexType>) -> Self {
        let mut hasUv0 = false;

        let mut vertex_vars = VertexVars::new();
        for item in vertex_types {
            let mem_size = std::mem::align_of::<f32>() as u32;
            let f: wgpu::VertexFormat = match item.size {
                1 => wgpu::VertexFormat::Float,
                2 => wgpu::VertexFormat::Float2,
                3 => wgpu::VertexFormat::Float3,
                4 => wgpu::VertexFormat::Float4,
                _ => panic!("错误"),
            };
            vertex_vars.set(
                item.attrib,
                VertexVar {
                    offset: item.size * mem_size,
                    format: f,
                    data_type: item.attrib,
                },
            );
        }
        let mut offset = 0;
        vertex_vars.vars.iter_mut().for_each(|v| {
            v.as_mut().map(|x| {
                let t = x.offset;
                x.offset = offset;
                offset += t;
            });
        });
        let stride = offset;

        return VertexFormat {
            stride,

            vertex_vars,
        };
    }
}
