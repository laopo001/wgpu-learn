#![allow(non_snake_case)]
#![allow(unused)]

use wgpu;
use wgpu_learn::core::vertex::Vertex;
use wgpu_learn::ecs::entity::{Component, Entity};
use wgpu_learn::model::material::texture::Texture;
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

async fn run() {
    let mut app = app::App::new("123", Config::PowerHighPerformance).await;
    let mut mesh = Mesh::new(&app);

    let vertex_data = [
        Vertex {
            position: [-0.5, 0.5, 0.0],
            tex_coord: Some([0.0, 1.0]),
            color: None,
            normal: None,
        },
        Vertex {
            position: [-0.5, -0.5, 0.0],
            tex_coord: Some([0.0, 0.0]),
            color: None,
            normal: None,
        },
        Vertex {
            position: [0.5, 0.5, 0.0],
            tex_coord: Some([1.0, 1.0]),
            color: None,
            normal: None,
        },
        Vertex {
            position: [0.5, -0.5, 0.0],
            tex_coord: Some([1.0, 0.0]),
            color: None,
            normal: None,
        },
    ];
    let vertex_data = vertex_data
        .iter()
        .map(|x| {
            return x.data();
        })
        .collect::<Vec<Box<[f32]>>>()
        .concat();

    let format = VertexFormat::new(vec![
        VertexType {
            attrib: Attrib::POSITION,
            size: 3,
        },
        VertexType {
            attrib: Attrib::TEXCOORD0,
            size: 2,
        },
    ]);
    let vertex_buffer = VertexBuffer::new(vertex_data.as_bytes().to_vec(), format);
    mesh.set_vertex_buffer(vertex_buffer);
    let index_data: Vec<u16> = vec![0, 1, 2, 2, 1, 3];
    let index_buffer = IndexBuffer::new(index_data.as_bytes().to_vec(), index_data.len());
    mesh.set_index_buffer(index_buffer);

    let texture = Texture::new_for_png(include_bytes!("./negz.png"));
    let mut face = Entity::new("a");
    mesh.material.texture = Some(texture);
    face.set_component(Component::Mesh { mesh });
    face.set_position(0.0001, 0.0, 0.0);

    let mut camera = Entity::new("camera");
    camera.set_position(0.0, 0.0, 2.0);
    camera.set_component(Component::Camera {
        fov: 45.0,
        aspect: app.size.width as f32 / app.size.height as f32,
        near: 1.0,
        far: 10.0,
    });
    camera.lookat(&mut face);
    app.scene.root.add_child(face);
    app.scene.root.add_child(camera);

    app.on(Event::Update, move |app| unsafe {
        // app.draw_mesh(&mut mesh);
    });

    app.start();
}

fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    async_std::task::block_on(run());
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(run());
    }
}
