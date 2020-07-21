#![allow(non_snake_case)]
#![allow(unused)]

use wgpu;
use wgpu_learn::core::vertex::Vertex;
use wgpu_learn::ecs::entity::{Component, Entity};
use wgpu_learn::model::material::texture::Texture;
use wgpu_learn::model::mesh::Mesh;
use wgpu_learn::trait_help::*;
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
    let mut mesh = Mesh::create_box(&app, None, None, None, None);
    wgpu_learn::console_log!(&mesh.vertex_buffer.as_ref().unwrap().format);
    unsafe {
        wgpu_learn::console_log!(&mesh.material.shader.vertex_buffer.unwrap().as_ref().format);
    }

    let texture = Texture::new_for_png(include_bytes!("./negz.png"));
    let mut face = Entity::new("a");
    dbg!(&mesh);
    mesh.material.texture = Some(texture);
    face.set_component(Component::Mesh { mesh });
    face.set_position(0.0001, 0.0, 0.0);
    // face.set_euler_angles(0.0, 90.0, 0.0);

    let mut camera = Entity::new("camera");
    camera.set_position(0.0, 2.0, 0.0);
    camera.set_component(Component::Camera {
        fov: 45.0,
        aspect: app.size.width as f32 / app.size.height as f32,
        near: 0.001,
        far: 10.0,
    });
    camera.lookat(&mut face);

    wgpu_learn::console_log!(camera.get_world_transform().as_ref() as &[f32; 16]);
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
