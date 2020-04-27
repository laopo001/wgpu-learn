#![allow(unused)]
#[macro_use]
extern crate lazy_static;
use wgpu_learn::{
    app,
    config::{Config, Event},
    console_log,
    core::shader::Shader,
    core::vertex_buffer::VertexBuffer,
    core::vertex_format::VertexFormat,
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
    let gvf = VertexFormat::new(8);
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
    let shader = Shader::new(
        &app,
        include_str!("./projection_camera.vert"),
        include_str!("./projection_camera.frag"),
    );
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
            rpass.set_pipeline(&shader.render_pipeline);
            rpass.set_bind_group(0, &shader.bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }

        app.queue.submit(&[encoder.finish()]);
    });

    app.start();
}

fn main() {
    async_std::task::block_on(run());
}
