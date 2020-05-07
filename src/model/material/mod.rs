use crate::app::App;
use crate::config::Uniform;
use crate::core::shader::Shader;
use crate::core::shader_var::{UniformBindingResource, UniformVar};
use crate::Color3;
use serde_json::json;

use zerocopy::{AsBytes, FromBytes};
pub struct Material {
    pub shader: Shader,
    pub color: Color3,
}
impl Material {
    pub fn new(app: &App) -> Self {
        return Material {
            color: Color3::new(0.0, 0.0, 0.0),
            shader: Shader::new(app),
        };
    }
    pub fn set_uniform_vars(&mut self, t: Uniform, var: UniformVar) {
        self.shader.set_uniform_vars(t, var);
    }
    pub fn update_shader(&mut self, app: &App) {
        let vec: Vec<f32> = vec![self.color.x, self.color.y, self.color.z];
        let uniform_buf = app.device.create_buffer_with_data(
            vec.as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        self.shader.get_base_material();
        self.shader.set_uniform_vars(
            Uniform::Color0,
            UniformVar {
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                resource: UniformBindingResource::Buffer {
                    buffer: uniform_buf,
                    range: 0..(vec.len() * std::mem::size_of::<f32>()) as u64,
                },
            },
        );
    }
}
