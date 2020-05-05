use crate::app::App;
use crate::core::shader::Shader;
use crate::Color3;

pub struct Material {
    shader: Option<Shader>,
    color: Color3,
}
impl Material {
    pub fn new(app: &App) -> Self {
        return Material {
            color: Color3::new(0.0, 0.0, 0.0),
            shader: Some(Shader::new(app)),
        };
    }
    pub fn default() -> Self {
        return Material {
            color: Color3::new(0.0, 0.0, 0.0),
            shader: None,
        };
    }
}
