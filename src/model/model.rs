use crate::core::index_buffer::IndexBuffer;
use crate::core::vertex_buffer::VertexBuffer;
use crate::model::material::Material;
pub struct Mesh<'a> {
    pub material: Material<'a>,
    pub vertex_buffer: Option<VertexBuffer>,
    pub index_buffer: Option<IndexBuffer>,
}
impl<'a> Mesh<'a> {
    pub fn new() -> Self {
        return Mesh {
            vertex_buffer: None,
            index_buffer: None,
            material: Material::default(),
        };
    }
}

pub struct Model<'a> {
    pub meshs: Vec<Mesh<'a>>,
}
impl<'a> Model<'a> {
    pub fn new() -> Self {
        return Model { meshs: vec![] };
    }
}
