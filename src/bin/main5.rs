#![allow(non_snake_case)]
#![allow(unused)]

use wgpu;
use wgpu_learn::model::material::texture::Texture;
use wgpu_learn::model::mesh::Mesh;
use wgpu_learn::scene::camera::Camera;
use wgpu_learn::scene::node::Node;
use wgpu_learn::trait_help::{Into, Matrix4Plus, QuatPlus, Vector3Plus, Vector4Plus};
use wgpu_learn::Deg;
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
fn main() {
    let mut node = Node::new();
    let mut child = Node::new();
    let mut grandson = Node::new();
    dbg!(node.parent.is_null());
    node.add_child(&mut child);
    child.add_child(&mut grandson);
    node.set_local_position(1.0, 2.0, 3.0);
    child.set_local_position(1.0, 2.0, 3.0);
    grandson.set_local_position(1.0, 2.0, 3.0);
    // assert_eq!(
    //     grandson.get_position().data(),
    //     Vector3::new(3.0, 6.0, 9.0).data()
    // );
    grandson.set_position(0.0, 0.0, 0.0);

    assert_eq!(
        grandson.get_position().data(),
        Vector3::new(0.0, 0.0, 0.0).data()
    );

    assert_eq!(
        grandson.get_local_position().data(),
        Vector3::new(-2.0, -4.0, -6.0).data()
    );
    let mut q = Camera::new();
    q.set_perspective(90.0, 10.0 / 6.0, 0.1, 1000.0);
    dbg!(&q.projection_matrix);
}
