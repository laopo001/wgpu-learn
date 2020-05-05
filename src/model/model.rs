use crate::model::material::Material;

pub struct Mesh {}

pub struct Model {
    pub material: Material,
    pub meshs: Vec<Mesh>,
}
impl Model {
    pub fn new() -> Self {
        return Model {
            material: Material::default(),
            meshs: vec![],
        };
    }
}
