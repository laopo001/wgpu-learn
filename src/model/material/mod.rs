use crate::Color3;

pub struct Material {
    color: Color3,
}
impl Material {
    pub fn default() -> Material {
        return Material {
            color: Color3::new(0.0, 0.0, 0.0),
        };
    }
}
