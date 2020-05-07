use crate::model::mesh::Mesh;

pub struct Model {
    pub meshs: Vec<Mesh>,
}
impl Model {
    pub fn new() -> Self {
        return Model { meshs: vec![] };
    }
}
