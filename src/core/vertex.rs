pub struct Vertex {
    pub position: [f32; 3],
    pub color: Option<[f32; 3]>,
    pub tex_coord: Option<[f32; 2]>,
    pub normal: Option<[f32; 3]>,
}

impl Vertex {
    pub fn data(&self) -> Box<[f32]> {
        let mut res = vec![];
        res.extend_from_slice(&self.position);
        if let Some(data) = &self.color {
            res.extend_from_slice(data);
        }
        if let Some(data) = &self.tex_coord {
            res.extend_from_slice(data);
        }
        if let Some(data) = &self.normal {
            res.extend_from_slice(data);
        }
        return res.into_boxed_slice();
    }
}
