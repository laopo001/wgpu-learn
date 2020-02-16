#![allow(non_snake_case)]
#![allow(unused)]

use cgmath::{Matrix4, Vector2, Vector3};
use wgpu;
use wgpu_learn;
use winit::{
    event,
    event_loop::{ControlFlow, EventLoop},
};
use zerocopy::{AsBytes, FromBytes};

struct Vertex {
    position: Vector3<f32>,
    tex_coord: Vector2<f32>,
}

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

fn main() {
    let event_loop = EventLoop::new();
    let (window, window_size, surface) = {
        let window = winit::window::Window::new(&event_loop).unwrap();
        let size = window.inner_size().to_physical(window.hidpi_factor());

        let surface = wgpu::Surface::create(&window);
        (window, size, surface)
    };
    // data
    let vertex_data = [
        Vertex {
            position: Vector3::new(-0.5, 0.5, 0.0),
            tex_coord: Vector2::new(0.0, 1.0),
        },
        Vertex {
            position: Vector3::new(-0.5, -0.5, 0.0),
            tex_coord: Vector2::new(0.0, 0.0),
        },
        Vertex {
            position: Vector3::new(0.5, 0.5, 0.0),
            tex_coord: Vector2::new(1.0, 1.0),
        },
        Vertex {
            position: Vector3::new(0.5, -0.5, 0.0),
            tex_coord: Vector2::new(1.0, 0.0),
        },
    ];
    let index_data: Vec<u16> = vec![0, 1, 2, 2, 1, 3];
    let mx_projection = cgmath::perspective(
        cgmath::Deg(45f32),
        window_size.width as f32 / window_size.height as f32,
        1.0,
        10.0,
    );
    let mx_view = cgmath::Matrix4::look_at(
        cgmath::Point3::new(0.0, 0.0, 2.0),
        cgmath::Point3::new(0.0001, 0.0, 0.0),
        cgmath::Vector3::unit_z(),
    );

    let mx_model: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    dbg!(mx_view);
    let model_view_projection_matrix = mx_model * mx_projection * mx_view;
    dbg!(model_view_projection_matrix);
    let adapter = wgpu::Adapter::request(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
        },
        wgpu::BackendBit::PRIMARY,
    )
    .unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });
    let vs_bytes = wgpu_learn::util::load_glsl(
        include_str!("./shader.vert"),
        wgpu_learn::ShaderStage::Vertex,
    );
    let fs_bytes = wgpu_learn::util::load_glsl(
        include_str!("./shader.frag"),
        wgpu_learn::ShaderStage::Fragment,
    );
    let vs_module = device.create_shader_module(&vs_bytes);
    let fs_module = device.create_shader_module(&fs_bytes);
    let vertex_buf = device.create_buffer_with_data(
        vertex_data
            .iter()
            .map(|x| {
                return [
                    x.position.x,
                    x.position.y,
                    x.position.z,
                    1.0,
                    x.tex_coord.x,
                    x.tex_coord.y,
                ];
            })
            .collect::<Vec<[f32; 6]>>()
            .concat()
            .as_bytes(),
        wgpu::BufferUsage::VERTEX,
    );

    let index_buf = device.create_buffer_with_data(index_data.as_bytes(), wgpu::BufferUsage::INDEX);
    let mx_ref: &[f32; 16] = model_view_projection_matrix.as_ref();
    let uniform_buf = device.create_buffer_with_data(
        mx_ref.as_bytes(),
        wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    );
    // Create the texture
    let size = 256u32;
    let texels = create_texels(size as usize);
    let texture_extent = wgpu::Extent3d {
        width: size,
        height: size,
        depth: 1,
    };
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        size: texture_extent,
        array_layer_count: 1,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
    });
    let texture_view = texture.create_default_view();
    let temp_buf = device.create_buffer_with_data(texels.as_slice(), wgpu::BufferUsage::COPY_SRC);
    let mut init_encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
    init_encoder.copy_buffer_to_texture(
        wgpu::BufferCopyView {
            buffer: &temp_buf,
            offset: 0,
            row_pitch: 4 * size,
            image_height: size,
        },
        wgpu::TextureCopyView {
            texture: &texture,
            mip_level: 0,
            array_layer: 0,
            origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
        },
        texture_extent,
    );
    queue.submit(&[init_encoder.finish()]);
    // Create other resources
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Nearest,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::FilterMode::Nearest,
        lod_min_clamp: -100.0,
        lod_max_clamp: 100.0,
        compare_function: wgpu::CompareFunction::Always,
    });
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[
            wgpu::BindGroupLayoutBinding {
                binding: 0,
                visibility: wgpu::ShaderStage::VERTEX,
                ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            },
            wgpu::BindGroupLayoutBinding {
                binding: 1,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::SampledTexture {
                    multisampled: false,
                    dimension: wgpu::TextureViewDimension::D2,
                },
            },
            wgpu::BindGroupLayoutBinding {
                binding: 2,
                visibility: wgpu::ShaderStage::FRAGMENT,
                ty: wgpu::BindingType::Sampler,
            },
        ],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[
            wgpu::Binding {
                binding: 0,
                resource: wgpu::BindingResource::Buffer {
                    buffer: &uniform_buf,
                    range: 0..64,
                },
            },
            wgpu::Binding {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::Binding {
                binding: 2,
                resource: wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
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
        depth_stencil_state: None,
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[wgpu::VertexBufferDescriptor {
            stride: 6 * 4 as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float4,
                    offset: 0,
                    shader_location: 0,
                },
                wgpu::VertexAttributeDescriptor {
                    format: wgpu::VertexFormat::Float2,
                    offset: 4 * 4,
                    shader_location: 1,
                },
            ],
        }],
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });

    let mut swap_chain = device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: window_size.width.round() as u32,
            height: window_size.height.round() as u32,
            present_mode: wgpu::PresentMode::Vsync,
        },
    );

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            event::Event::MainEventsCleared => window.request_redraw(),
            event::Event::RedrawRequested(_) => {
                let frame = swap_chain
                    .get_next_texture()
                    .expect("Timeout when acquiring next swap chain texture");
                let mut encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
                {
                    let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                            attachment: &frame.view,
                            resolve_target: None,
                            load_op: wgpu::LoadOp::Clear,
                            store_op: wgpu::StoreOp::Store,
                            clear_color: wgpu::Color::GREEN,
                        }],
                        depth_stencil_attachment: None,
                    });
                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_bind_group(0, &bind_group, &[]);
                    rpass.set_index_buffer(&index_buf, 0);
                    rpass.set_vertex_buffers(0, &[(&vertex_buf, 0)]);
                    // rpass.draw(0..3, 0..1);
                    rpass.draw_indexed(0..6 as u32, 0, 0..1);
                }
                queue.submit(&[encoder.finish()]);
            }
            event::Event::WindowEvent {
                event: event::WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}
