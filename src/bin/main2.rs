#![allow(unused)]
#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;
use wgpu_learn::{
    app,
    config::{Config, Event},
    console_log, time, Matrix4F32,
};

fn main() {
    let mut app = app::App::new("123", Config::PowerHighPerformance);
    let vs_bytes = wgpu_learn::util::load_glsl(
        include_str!("./main2.vert"),
        wgpu_learn::ShaderStage::Vertex,
    );
    let fs_bytes = wgpu_learn::util::load_glsl(
        include_str!("./main2.frag"),
        wgpu_learn::ShaderStage::Fragment,
    );
    let vs_module = app.device.create_shader_module(&vs_bytes);
    let fs_module = app.device.create_shader_module(&fs_bytes);
    let bind_group_layout = app
        .device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { bindings: &[] });
    let bind_group = app.device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[],
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
            depth_stencil_state: None,
            index_format: wgpu::IndexFormat::Uint16,
            vertex_buffers: &[],
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

    app.on(Event::Update, move |app| unsafe {
        let frame = app
            .swap_chain
            .get_next_texture()
            .expect("Timeout when acquiring next swap chain texture");
        let mut encoder = app
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
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
            rpass.set_pipeline(&render_pipeline);
            rpass.set_bind_group(0, &bind_group, &[]);
            rpass.draw(0..3, 0..1);
        }

        app.queue.submit(&[encoder.finish()]);
    });

    app.start();
}
