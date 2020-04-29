#![allow(unused)]
#[macro_use]
extern crate lazy_static;
use wgpu_learn::{
    app,
    config::{Attrib, Config, Event, Uniform},
    console_log,
    core::shader::Shader,
    core::shader_var::{UniformBindingResource, UniformVar},
    core::vertex_buffer::VertexBuffer,
    core::vertex_format::{VertexFormat, VertexType},
    time, Matrix4, Vector2, Vector3,
};
use zerocopy::{AsBytes, FromBytes};

struct Vertex {
    position: Vector3,
    tex_coord: Vector2,
}
async fn run() {
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

    let gvf = VertexFormat::new(vec![
        VertexType {
            attrib: Attrib::POSITION,
            size: 4,
        },
        VertexType {
            attrib: Attrib::TEXCOORD0,
            size: 2,
        },
    ]);
    // dbg!(gvf.elements[0].size);
    let gvb = VertexBuffer::new(
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
            .as_bytes()
            .to_vec(),
        gvf,
    );

    let mut app = app::App::new("123", Config::PowerHighPerformance).await;
    let mut shader = Shader::new(
        &app,
        include_str!("./projection_camera.vert"),
        include_str!("./projection_camera.frag"),
    );
    shader.set_vertex_buffer(gvb);
    let mx_projection = cgmath::perspective(
        cgmath::Deg(45f32),
        app.size.width as f32 / app.size.height as f32,
        1.0,
        10.0,
    );
    let mx_view = cgmath::Matrix4::look_at(
        cgmath::Point3::new(0.0, 0.0, 2.0),
        cgmath::Point3::new(0.0001, 0.0, 0.0),
        cgmath::Vector3::unit_z(),
    );

    let mx_model: Matrix4 = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );

    let model_view_projection_matrix = mx_model * mx_projection * mx_view;
    let mx_ref: &[f32; 16] = model_view_projection_matrix.as_ref();
    let uniform_buf = app.device.create_buffer_with_data(
        mx_ref.as_bytes(),
        wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    );
    shader.set_uniform_vars(
        Uniform::ModelViewProjectionMatrix,
        UniformVar {
            visibility: wgpu::ShaderStage::VERTEX,
            ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            resource: UniformBindingResource::Buffer {
                buffer: uniform_buf,
                range: 0..64,
            },
        },
    );
    let texture_view = shader.create_texture();
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
    });
    shader.set_uniform_vars(
        Uniform::Texture0,
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
    shader.set_uniform_vars(
        Uniform::Sampler0,
        UniformVar {
            visibility: wgpu::ShaderStage::FRAGMENT,
            ty: wgpu::BindingType::Sampler { comparison: false },
            resource: UniformBindingResource::Sampler(sampler),
        },
    );
    dbg!(shader.get_shader_head());
    shader.get_bind_group();
    let vertex_buf = app.device.create_buffer_with_data(
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

    let index_buf = app
        .device
        .create_buffer_with_data(index_data.as_bytes(), wgpu::BufferUsage::INDEX);

    app.on(Event::Update, move |app| unsafe {
        let frame = app
            .swap_chain
            .get_next_texture()
            .expect("Timeout when acquiring next swap chain texture");
        let mut encoder = app
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::BLACK,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&shader.render_pipeline.as_ref().expect("error1"));
            rpass.set_bind_group(0, &shader.bind_group.as_ref().expect("error2"), &[]);
            rpass.set_index_buffer(&index_buf, 0, 0);
            rpass.set_vertex_buffer(0, &vertex_buf, 0, 0);
            // rpass.draw(0..3, 0..1);
            rpass.draw_indexed(0..6 as u32, 0, 0..1);
        }

        app.queue.submit(Some(encoder.finish()));
    });

    app.start();
}

fn main() {
    async_std::task::block_on(run());
}
