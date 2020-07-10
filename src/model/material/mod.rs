use crate::app::App;
use crate::config::Uniform;
use crate::core::shader::Shader;
use crate::core::shader_var::{UniformBindingResource, UniformVar, UniformVars};
use crate::Color3;
use serde_json::json;
use std::ptr::NonNull;
pub mod texture;
use zerocopy::{AsBytes, FromBytes};
#[derive(DebugStub)]
pub struct Material {
    #[debug_stub = "Shader"]
    pub shader: Shader,
    pub color: Color3,
    pub texture: Option<texture::Texture>,
    app: *const App,
}
impl Material {
    fn app(&self) -> &App {
        if self.app.is_null() {
            panic!("app get not");
        }
        unsafe {
            return &*self.app;
        }
    }
    pub fn new() -> Self {
        return Material {
            color: Color3::new(0.0, 1.0, 0.0),
            shader: Shader::new(),
            texture: None,
            app: std::ptr::null() as *const App,
        };
    }
    pub fn set_uniform_vars(&mut self, t: Uniform, var: UniformVar) {
        self.shader.set_uniform_vars(t, var);
    }
    pub fn update_shader(&mut self, app: &App) {
        self.shader.set_app(app);
        let vec: Vec<f32> = vec![self.color.x, self.color.y, self.color.z];
        let uniform_buf = app.device.create_buffer_with_data(
            vec.as_bytes(),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );
        if let Some(t) = self.texture.as_ref() {
            if let Some(data) = t.img_data.as_ref() {
                let texture_view = app.create_wgpu_texture(data, t.size.0, t.size.1);
                self.shader.set_uniform_vars(
                    Uniform::pbrBaseColorTexture,
                    UniformVar {
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::SampledTexture {
                            multisampled: false,
                            component_type: wgpu::TextureComponentType::Float,
                            dimension: wgpu::TextureViewDimension::D2,
                        },
                        resource: UniformBindingResource::TextureView(texture_view),
                    },
                );
                let sampler = app.device.create_sampler(&wgpu::SamplerDescriptor {
                    address_mode_u: wgpu::AddressMode::ClampToEdge,
                    address_mode_v: wgpu::AddressMode::ClampToEdge,
                    address_mode_w: wgpu::AddressMode::ClampToEdge,
                    mag_filter: wgpu::FilterMode::Nearest,
                    min_filter: wgpu::FilterMode::Linear,
                    mipmap_filter: wgpu::FilterMode::Nearest,
                    lod_min_clamp: -100.0,
                    lod_max_clamp: 100.0,
                    compare: wgpu::CompareFunction::Always,
                    label: None,
                });
                self.shader.set_uniform_vars(
                    Uniform::Sampler,
                    UniformVar {
                        visibility: wgpu::ShaderStage::FRAGMENT,
                        ty: wgpu::BindingType::Sampler { comparison: false },
                        resource: UniformBindingResource::Sampler(sampler),
                    },
                );
            }
        }
        self.shader.set_uniform_vars(
            Uniform::pbrInfo,
            UniformVar {
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                resource: UniformBindingResource::Buffer {
                    buffer: uniform_buf,
                    range: 0..(vec.len() * std::mem::size_of::<f32>()) as u64,
                },
            },
        );
        self.shader.get_base_material();
    }
}
