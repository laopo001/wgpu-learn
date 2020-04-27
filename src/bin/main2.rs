#![allow(unused)]
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use wgpu_learn::{
    app,
    config::{Config, Event},
    console_log,
    core::shader::Shader,
    time, Matrix4F32,
};

async fn run() {
    let mut app = app::App::new("123", Config::PowerHighPerformance).await;
    let shader = Shader::new(
        &app,
        include_str!("./main2.vert"),
        include_str!("./main2.frag"),
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
