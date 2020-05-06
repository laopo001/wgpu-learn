use crate::config::{Uniform, ATTRIBNAMES, UNIFORMNAMES};
use crate::core::shader_var::{UniformBindingResource, UniformVar, UniformVars};
use crate::core::vertex_buffer::VertexBuffer;
use crate::core::vertex_format::VertexFormat;
use crate::{app::App, util::load_glsl};
use glsl_to_spirv::ShaderType;
use std::cell::RefCell;
use std::rc::Rc;
fn create_texels(size: usize) -> Vec<u8> {
    use std::iter;

    (0..size * size)
        .flat_map(|id| {
            // get high five for recognizing this ;)
            let cx = 3.0 * (id % size) as f32 / (size - 1) as f32 - 2.0;
            let cy = 2.0 * (id / size) as f32 / (size - 1) as f32 - 1.0;
            let (mut x, mut y, mut count) = (cx, cy, 0);
            while count < 0xFF && x * x + y * y < 4.0 {
                let old_x = x;
                x = x * x - y * y + cx;
                y = 2.0 * old_x * y + cy;
                count += 1;
            }
            iter::once(0xFF - (count * 5) as u8)
                .chain(iter::once(0xFF - (count * 15) as u8))
                .chain(iter::once(0xFF - (count * 50) as u8))
                .chain(iter::once(1))
        })
        .collect()
}
pub struct Shader<'a> {
    pub bind_group_layout: Option<wgpu::BindGroupLayout>,
    pub render_pipeline: Option<wgpu::RenderPipeline>,
    pub bind_group: Option<wgpu::BindGroup>,
    pub pipeline_layout: Option<wgpu::PipelineLayout>,
    pub layouts: Vec<wgpu::BindGroupLayoutEntry>,
    pub bindings: Vec<wgpu::Binding<'a>>,
    pub attributes: Vec<wgpu::VertexAttributeDescriptor>,
    pub vertex_desc: Vec<wgpu::VertexBufferDescriptor<'a>>,

    pub uniform_vars: UniformVars,
    pub vertex_buffer: Option<Rc<RefCell<VertexBuffer>>>,
    pub app: *const App,
    pub vs_module: Option<wgpu::ShaderModule>,
    pub fs_module: Option<wgpu::ShaderModule>,
}
impl<'a> Shader<'a> {
    pub fn new(app: &App) -> Self {
        let uniform_vars = UniformVars::new();
        Shader {
            bind_group_layout: None,
            render_pipeline: None,
            bind_group: None,
            pipeline_layout: None,
            layouts: vec![],
            bindings: vec![],
            attributes: vec![],
            vertex_desc: vec![],

            uniform_vars,
            vertex_buffer: None,
            app: app as *const App,
            vs_module: None,
            fs_module: None,
        }
    }
    pub fn new_by_code(app: &App, vs_code: &str, fs_code: &str) -> Self {
        let vs_bytes = load_glsl(vs_code, ShaderType::Vertex);
        let fs_bytes = load_glsl(fs_code, ShaderType::Fragment);
        let vs_module = app.device.create_shader_module(&vs_bytes);
        let fs_module = app.device.create_shader_module(&fs_bytes);
        let uniform_vars = UniformVars::new();
        Shader {
            bind_group_layout: None,
            render_pipeline: None,
            bind_group: None,
            pipeline_layout: None,
            layouts: vec![],
            bindings: vec![],
            attributes: vec![],
            vertex_desc: vec![],
            uniform_vars,
            vertex_buffer: None,
            app: app as *const App,
            vs_module: Some(vs_module),
            fs_module: Some(fs_module),
        }
    }
    pub fn set_uniform_vars(&mut self, t: Uniform, var: UniformVar) {
        self.uniform_vars.set(t, var);
    }
    pub fn get_uniform_shader_head(&self) -> (String, String) {
        let mut vert = "".to_string();
        let mut frag = "".to_string();
        for (i, item) in self.uniform_vars.vars.iter().enumerate() {
            if let Some(uniform_var) = item {
                match uniform_var.visibility {
                    wgpu::ShaderStage::VERTEX => {
                        frag += &format!(
                            r#"
                            layout(set = 0, binding = {}) uniform Locals {{
                                {} u_{};
                            }};
                           "#,
                            i,
                            UNIFORMNAMES[i]["type"].as_str().unwrap(),
                            UNIFORMNAMES[i]["name"].as_str().unwrap()
                        );
                    } // Fragment,
                    wgpu::ShaderStage::FRAGMENT => {
                        vert += &format!(
                            r#"
                            layout(set = 0, binding = {}) uniform {} u_{};
                           "#,
                            i,
                            UNIFORMNAMES[i]["type"].as_str().unwrap(),
                            UNIFORMNAMES[i]["name"].as_str().unwrap()
                        );
                    }
                    _ => panic!("错误"),
                }
            }
        }
        return (vert, frag);
    }
    pub fn get_shader_head(&self) -> (String, String) {
        let (vert, frag) = self.get_uniform_shader_head();
        let (vert2, frag2) = self.get_attrib_shader_head();
        (vert + &vert2, frag + &frag2)
    }
    pub fn set_vertex_buffer(&mut self, buffer: VertexBuffer) {
        self.vertex_buffer = Some(Rc::new(RefCell::new(buffer)));
    }
    pub fn get_attrib_shader_head(&self) -> (String, String) {
        let mut vert = "".to_string();
        let mut frag = "".to_string();
        for (i, item) in self
            .vertex_buffer
            .as_ref()
            .expect("请设置vertex_buffer")
            .borrow()
            .format
            .vertex_vars
            .vars
            .iter()
            .enumerate()
        {
            if let Some(vertex_var) = item {
                vert += &format!(
                    "layout (location = {}) in {} a_{};\n",
                    i,
                    ATTRIBNAMES[i]["type"].as_str().unwrap(),
                    ATTRIBNAMES[i]["name"].as_str().unwrap()
                )
            }
        }
        for (i, item) in self
            .vertex_buffer
            .as_ref()
            .expect("请设置vertex_buffer")
            .borrow()
            .format
            .vertex_vars
            .vars
            .iter()
            .enumerate()
        {
            if let Some(vertex_var) = item {
                if ATTRIBNAMES[i]["vary"].as_bool().expect("vary 转 bool") {
                    vert += &format!(
                        "layout (location = {}) out {} v_{};\n",
                        i,
                        ATTRIBNAMES[i]["type"].as_str().unwrap(),
                        ATTRIBNAMES[i]["name"].as_str().unwrap()
                    );
                    frag += &format!(
                        "layout (location = {}) in {} v_{};\n",
                        i,
                        ATTRIBNAMES[i]["type"].as_str().unwrap(),
                        ATTRIBNAMES[i]["name"].as_str().unwrap()
                    );
                }
            }
        }
        return (vert, frag);
    }
    pub fn create_texture(&self) -> wgpu::TextureView {
        unsafe {
            let size = 256u32;
            let texels = create_texels(size as usize);
            let texture_extent = wgpu::Extent3d {
                width: size,
                height: size,
                depth: 1,
            };
            let texture = (*self.app).device.create_texture(&wgpu::TextureDescriptor {
                size: texture_extent,
                // array_layer_count: 1,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
                label: None,
            });
            let texture_view = texture.create_default_view();
            let temp_buf = (*self.app)
                .device
                .create_buffer_with_data(texels.as_slice(), wgpu::BufferUsage::COPY_SRC);
            let mut init_encoder = (*self.app)
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            init_encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &temp_buf,
                    offset: 0,
                    bytes_per_row: 4 * size,
                    rows_per_image: size,
                },
                wgpu::TextureCopyView {
                    texture: &texture,
                    mip_level: 0,
                    array_layer: 0,
                    origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
                },
                texture_extent,
            );
            (*self.app).queue.submit(Some(init_encoder.finish()));
            texture_view
        }
    }
    pub fn set_index_buffer(&self, index_buffer: &[u8]) -> wgpu::Buffer {
        unsafe {
            let index_buf = (*self.app)
                .device
                .create_buffer_with_data(index_buffer, wgpu::BufferUsage::INDEX);
            return index_buf;
        }
    }
    pub fn get_bind(&mut self) {
        unsafe {
            for (i, o_var) in self.uniform_vars.vars.iter().enumerate() {
                if let Some(var) = o_var {
                    self.layouts.push(wgpu::BindGroupLayoutEntry {
                        binding: i as u32,
                        visibility: var.visibility,
                        ty: var.ty,
                    });
                    match &var.resource {
                        UniformBindingResource::Buffer { buffer, range } => {
                            self.bindings.push(std::mem::transmute(wgpu::Binding {
                                binding: i as u32,
                                resource: wgpu::BindingResource::Buffer {
                                    buffer,
                                    range: range.clone(),
                                },
                            }));
                        }
                        UniformBindingResource::TextureView(texture_view) => {
                            self.bindings.push(std::mem::transmute(wgpu::Binding {
                                binding: i as u32,
                                resource: wgpu::BindingResource::TextureView(&texture_view),
                            }));
                        }
                        UniformBindingResource::Sampler(sampler) => {
                            self.bindings.push(std::mem::transmute(wgpu::Binding {
                                binding: i as u32,
                                resource: wgpu::BindingResource::Sampler(&sampler),
                            }));
                        }
                        _ => panic!("error"),
                    }
                }
            }
            let bind_group_layout =
                (*self.app)
                    .device
                    .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                        bindings: self.layouts.as_slice(),
                        label: None,
                    });
            let bind_group = (*self.app)
                .device
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    layout: &bind_group_layout,
                    bindings: self.bindings.as_slice(),
                    label: None,
                });
            let pipeline_layout =
                (*self.app)
                    .device
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        bind_group_layouts: &[&bind_group_layout],
                    });

            if self.vertex_buffer.is_some() {
                for (i, o_var) in self
                    .vertex_buffer
                    .as_ref()
                    .expect("can not get vertex_buffer")
                    .borrow()
                    .format
                    .vertex_vars
                    .vars
                    .iter()
                    .enumerate()
                {
                    if let Some(var) = o_var {
                        self.attributes.push(wgpu::VertexAttributeDescriptor {
                            format: var.format,
                            offset: var.offset as u64,
                            shader_location: i as u32,
                        });
                    }
                }

                self.vertex_desc.push(wgpu::VertexBufferDescriptor {
                    stride: self
                        .vertex_buffer
                        .as_ref()
                        .expect("can not get vertex_buffer")
                        .borrow()
                        .format
                        .stride as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: std::mem::transmute::<
                        &[wgpu::VertexAttributeDescriptor],
                        &[wgpu::VertexAttributeDescriptor],
                    >(self.attributes.as_slice())
                        as &'a [wgpu::VertexAttributeDescriptor],
                });
            } else {
                println!("not set_vertex_buffer");
            }
            let render_pipeline =
                (*self.app)
                    .device
                    .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                        layout: &pipeline_layout,
                        vertex_stage: wgpu::ProgrammableStageDescriptor {
                            module: self.vs_module.as_ref().unwrap(),
                            entry_point: "main",
                        },
                        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                            module: self.vs_module.as_ref().unwrap(),
                            entry_point: "main",
                        }),
                        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                            front_face: wgpu::FrontFace::Ccw,
                            cull_mode: wgpu::CullMode::None,
                            depth_bias: 0,
                            depth_bias_slope_scale: 0.0,
                            depth_bias_clamp: 0.0,
                        }),
                        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                        color_states: &[wgpu::ColorStateDescriptor {
                            format: wgpu::TextureFormat::Bgra8UnormSrgb,
                            color_blend: wgpu::BlendDescriptor::REPLACE,
                            alpha_blend: wgpu::BlendDescriptor::REPLACE,
                            write_mask: wgpu::ColorWrite::ALL,
                        }],
                        depth_stencil_state: None,
                        vertex_state: wgpu::VertexStateDescriptor {
                            index_format: wgpu::IndexFormat::Uint16,
                            vertex_buffers: self.vertex_desc.as_slice(),
                        },
                        sample_count: 1,
                        sample_mask: !0,
                        alpha_to_coverage_enabled: false,
                    });
            self.bind_group_layout = Some(bind_group_layout);
            self.bind_group = Some(bind_group);
            self.render_pipeline = Some(render_pipeline);
            self.pipeline_layout = Some(pipeline_layout);
        }
    }
}
