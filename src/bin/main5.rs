#![allow(non_snake_case)]
#![allow(unused)]

use wgpu;
use wgpu_learn::ecs::entity::Entity;
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
    node.set_local_position(2.0, 0.0, 2.0);
    let mut node2 = Node::new();
    node2.set_local_position(0.0, 0.0, 0.0);
    dbg!(node.get_world_transform().data());
    node.lookat(&mut node2);
    dbg!(node.get_world_transform().data());
    dbg!("=================");
    return;
    let mut node = Entity::new("1");
    let mut child = Entity::new("2");
    let mut grandson = Entity::new("3");
    // dbg!(&node as *const Entity);
    // dbg!(&child as *const Entity);
    dbg!(&grandson as *const Box<Entity>);
    child.add_child(grandson);
    // dbg!(&child as *const Entity);
    dbg!(child.get_by_name("2").unwrap() as *const Entity);
    // unsafe {
    //     dbg!(&(*(child.get_by_name("3").unwrap().__node.parent)).name);
    // }
    node.add_child(child);
    // unsafe {
    //     dbg!(&(*(node.get_by_name("2").unwrap().__node.parent)).name);
    // }
    node.set_local_position(1.0, 2.0, 3.0);
    node.get_by_name("2")
        .unwrap()
        .set_local_position(1.0, 2.0, 3.0);
    node.get_by_name("3")
        .unwrap()
        .set_local_position(1.0, 2.0, 3.0);
    // unsafe {
    //     dbg!(&(node));
    // }
    dbg!(node.get_by_name("1").unwrap() as *const Entity);
    dbg!(node.get_by_name("2").unwrap() as *const Entity);
    dbg!(node.get_by_name("3").unwrap() as *const Entity);

    unsafe {
        dbg!((node.get_by_name("3").unwrap().parent as *const Entity));
        // dbg!(&node.children[0].children[0] as *const Entity);
        dbg!(&(*node.children[0].children[0].parent).parent.is_null());
    }
    dbg!(node.get_by_name("3").unwrap().get_position().data());
    assert_eq!(
        node.get_by_name("3").unwrap().get_position().data(),
        Vector3::new(3.0, 6.0, 9.0).data()
    );
}
