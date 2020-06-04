use std::error::Error;
use wgpu_learn::app;
use wgpu_learn::config::Config;
use wgpu_learn::ecs::entity::Entity;
use wgpu_learn::model::mesh::Mesh;
fn each_node(node: &gltf::Node) -> Box<Entity> {
    let mut entity = Entity::new(node.name().unwrap_or(""));
    node.mesh().map(|mesh| {
        // Mesh::new(app)
    });
    for node in node.children() {
        entity.add_child(each_node(&node));
    }
    entity
}

async fn run() {
    let mut app = app::App::new("123", Config::PowerHighPerformance).await;
    let gltf = gltf::Gltf::open("Box.gltf").unwrap();
    let (document, buffers, images) = gltf::import("Box.gltf").unwrap();
    for scene in gltf.scenes() {
        let mut entity = Entity::new("root");
        for node in scene.nodes() {
            entity.add_child(each_node(&node));
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    async_std::task::block_on(run());
    Ok(())
}
