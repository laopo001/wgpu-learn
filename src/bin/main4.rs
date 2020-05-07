#![allow(non_snake_case)]
#![allow(unused)]

use wgpu;
use wgpu_learn::model::mesh::Mesh;
use wgpu_learn::{
    app,
    config::{Attrib, Config, Event, Uniform},
    console_log,
    core::index_buffer::IndexBuffer,
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

async fn run() {
    let mut app = app::App::new("123", Config::PowerHighPerformance).await;
    let mut mesh = Mesh::new(&app);

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
    let vertex_data = vertex_data
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
        .concat();

    let format = VertexFormat::new(vec![
        VertexType {
            attrib: Attrib::POSITION,
            size: 4,
        },
        VertexType {
            attrib: Attrib::TEXCOORD0,
            size: 2,
        },
    ]);
    let vertex_buffer = VertexBuffer::new(vertex_data.as_bytes().to_vec(), format);
    mesh.set_vertex_buffer(vertex_buffer);
    let index_data: Vec<u16> = vec![0, 1, 2, 2, 1, 3];
    let index_buffer = IndexBuffer::new(index_data.as_bytes().to_vec());
    mesh.set_index_buffer(index_buffer);
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

    let mx_model: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    let model_view_projection_matrix = mx_model * mx_projection * mx_view;
    let mx_ref: &[f32; 16] = model_view_projection_matrix.as_ref();
    let uniform_buf = app.device.create_buffer_with_data(
        mx_ref.as_bytes(),
        wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    );
    mesh.material.set_uniform_vars(
        Uniform::ModelViewProjectionMatrix,
        UniformVar {
            resource: UniformBindingResource::Buffer {
                buffer: uniform_buf,
                range: 0..64,
            },
            ty: wgpu::BindingType::UniformBuffer { dynamic: false },
            visibility: wgpu::ShaderStage::VERTEX,
        },
    );
    app.on(Event::Update, move |app| unsafe {
        app.draw_mesh(&mut mesh);
    });

    app.start();
}

fn main() {
    async_std::task::block_on(run());
}
