use crate::core::index_buffer::IndexBuffer;
use crate::core::vertex_buffer::VertexBuffer;
use crate::model::material::Material;
pub struct Mesh {
    pub material: Material,
    pub vertex_buffer: Option<VertexBuffer>,
    pub index_buffer: Option<IndexBuffer>,
}
impl Mesh {
    pub fn new() -> Self {
        return Mesh {
            vertex_buffer: None,
            index_buffer: None,
            material: Material::default(),
        };
    }
}

pub struct Model {
    pub meshs: Vec<Mesh>,
}
impl Model {
    pub fn new() -> Self {
        return Model { meshs: vec![] };
    }
}
