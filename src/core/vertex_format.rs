use crate::config::SEMANTIC;
use std::ops::{Deref, DerefMut};
pub struct VertexType {
    pub semantic: SEMANTIC,
    pub size: u32,
    pub normalize: bool,
    pub isF64: bool,
}

pub struct VertexAttribData {
    pub vertex_type: VertexType,
    pub offset: u32,
    pub stride: u32,
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
}

impl VertexFormat {
    pub fn new(vertex_types: Vec<VertexType>) -> Self {
        let mut offset = 0;
        let mut hasUv0 = false;
        let mut elements = vec![];

        for item in vertex_types {
            let mem_size = if item.isF64 {
                std::mem::align_of::<f64>() as u32
            } else {
                std::mem::align_of::<f32>() as u32
            };
            let vertex_data = VertexAttribData {
                length: item.size * mem_size,
                vertex_type: item,
                offset: offset,
                stride: 0,
            };
            offset += vertex_data.length;
            if vertex_data.semantic == SEMANTIC::TEXCOORD0 {
                hasUv0 = true;
            }
            elements.push(vertex_data);
        }
        let stride = offset;
        elements.iter_mut().for_each(|x| {
            x.stride = stride;
        });
        return VertexFormat {
            elements,
            stride,
            hasUv0,
        };
    }
}
