use crate::app::App;
use crate::config::Uniform;
use crate::core::shader::Shader;
use crate::core::shader_var::{UniformBindingResource, UniformVar};
use crate::Color3;
use serde_json::json;
use zerocopy::{AsBytes, FromBytes};
pub struct Material {
    pub shader: Option<Shader>,
    pub color: Color3,
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
    pub fn add_shader(&mut self, app: &App) -> &Shader {
        if self.shader.is_some() {
            return self.shader.as_ref().expect("msg");
        } else {
            let vec: Vec<f32> = vec![self.color.x, self.color.y, self.color.z];
            let uniform_buf = app.device.create_buffer_with_data(
                vec.as_bytes(),
                wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            );
            let mut s = Shader::new(app);
            s.set_uniform_vars(
                Uniform::Color0,
                UniformVar {
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                    resource: UniformBindingResource::Buffer {
                        buffer: uniform_buf,
                        range: 0..64,
                    },
                },
            );
            self.shader = Some(s);
            return self.shader.as_ref().unwrap();
        }
    }
}
