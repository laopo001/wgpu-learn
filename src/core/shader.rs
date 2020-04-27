use crate::{app::App, util::load_glsl, ShaderStage};

pub struct Shader {
    pub render_pipeline: wgpu::RenderPipeline,
    pub bind_group: wgpu::BindGroup,
}
impl Shader {
    pub fn new(app: &App, vs_code: &str, fs_code: &str) -> Shader {
        let vs_bytes = load_glsl(vs_code, ShaderStage::Vertex);
        let fs_bytes = load_glsl(fs_code, ShaderStage::Fragment);
        let vs_module = app.device.create_shader_module(&vs_bytes);
        let fs_module = app.device.create_shader_module(&fs_bytes);
        let bind_group_layout =
            app.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    bindings: &[],
                    label: None,
                });
        let bind_group = app.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[],
            label: None,
        });
        let pipeline_layout = app
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[&bind_group_layout],
            });

        let render_pipeline = app
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                layout: &pipeline_layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor {
                    module: &vs_module,
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                    module: &fs_module,
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
                vertex_state: wgpu::VertexStateDescriptor {
                    index_format: wgpu::IndexFormat::Uint16,
                    vertex_buffers: &[],
                },
                depth_stencil_state: None,
                // index_format: wgpu::IndexFormat::Uint16,
                // vertex_buffers: &[],
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            });
        Shader {
            render_pipeline,
            bind_group,
        }
    }
}
