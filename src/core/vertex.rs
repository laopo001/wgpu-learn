use crate::config::Attrib;
use crate::core::vertex_format::VertexFormat;
pub struct Vertex {
    pub position: [f32; 3],
    pub color: Option<[f32; 3]>,
    pub tex_coord: Option<[f32; 2]>,
    pub normal: Option<[f32; 3]>,
}

impl Vertex {
    pub fn data(&self, format: &VertexFormat) -> Box<[f32]> {
        let mut res = vec![];
        format.vertex_vars.vars.iter().for_each(|o_v| {
            o_v.as_ref().map(|v| match v.data_type {
                Attrib::POSITION => {
                    res.extend_from_slice(&self.position);
                }
                Attrib::TEXCOORD0 => {
                    if let Some(data) = &self.tex_coord {
                        res.extend_from_slice(data);
                    }
                }
                Attrib::COLOR => {
                    if let Some(data) = &self.color {
                        res.extend_from_slice(data);
                    }
                }
                Attrib::NORMAL => {
                    if let Some(data) = &self.normal {
                        res.extend_from_slice(data);
                    }
                }
            });
        });
        // res.extend_from_slice(&self.position);
        // if let Some(data) = &self.color {
        //     res.extend_from_slice(data);
        // }
        // if let Some(data) = &self.tex_coord {
        //     res.extend_from_slice(data);
        // }
        // if let Some(data) = &self.normal {
        //     res.extend_from_slice(data);
        // }
        return res.into_boxed_slice();
    }
}
