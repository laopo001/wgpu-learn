use gltf::{buffer::Data as BufferData, image::Data as ImageData, Node, Semantic};
use std::error::Error;
use wgpu_learn::app;
use wgpu_learn::config::Config;
use wgpu_learn::ecs::entity::Entity;
use wgpu_learn::model::mesh::Mesh;

fn each_node(node: &Node, buffers: &Vec<BufferData>, images: &Vec<ImageData>) -> Box<Entity> {
    let mut entity = Entity::new(node.name().unwrap_or(""));
    node.mesh().map(|mesh| {
        let m = Mesh::new();
        // dbg!(std::mem::size_of::<f32>());
        mesh.primitives().for_each(|primitive| {
            let mut positions: Vec<f32> = vec![];
            let mut normals: Option<Vec<f32>> = None;
            let mut colors: Option<Vec<f32>> = None;
            let mut uvs: Option<Vec<f32>> = None;
            let mut indices: Option<Vec<f32>> = None;
            primitive
                .attributes()
                .for_each(|(semantic, accessor)| unsafe {
                    let index = accessor.view().unwrap().buffer().index();
                    // dbg!(&accessor.dimensions(), &accessor.data_type());
                    match semantic {
                        Semantic::Positions => {
                            let buffer = buffers[index].0[accessor.offset()
                                ..(accessor.offset() + accessor.count() * accessor.size())]
                                .to_vec();

                            positions = std::slice::from_raw_parts(
                                buffer.as_ptr() as *const f32,
                                buffer.len() * std::mem::size_of::<u8>()
                                    / std::mem::size_of::<f32>(),
                            )
                            .to_vec();
                        }
                        Semantic::Normals => {
                            let buffer = buffers[index].0[accessor.offset()
                                ..(accessor.offset() + accessor.count() * accessor.size())]
                                .to_vec();

                            let b = std::slice::from_raw_parts(
                                buffer.as_ptr() as *const f32,
                                buffer.len() * std::mem::size_of::<u8>()
                                    / std::mem::size_of::<f32>(),
                            )
                            .to_vec();
                            normals = Some(b);
                        }
                        Semantic::TexCoords(size) => {
                            assert_eq!(accessor.size(), size as usize);
                            let buffer = buffers[index].0[accessor.offset()
                                ..(accessor.offset() + accessor.count() * accessor.size())]
                                .to_vec();

                            let b = std::slice::from_raw_parts(
                                buffer.as_ptr() as *const f32,
                                buffer.len() * std::mem::size_of::<u8>()
                                    / std::mem::size_of::<f32>(),
                            )
                            .to_vec();
                            uvs = Some(b);
                        }
                        Semantic::Colors(size) => {
                            assert_eq!(accessor.size(), size as usize);
                            let buffer = buffers[index].0[accessor.offset()
                                ..(accessor.offset() + accessor.count() * accessor.size())]
                                .to_vec();

                            let b = std::slice::from_raw_parts(
                                buffer.as_ptr() as *const f32,
                                buffer.len() * std::mem::size_of::<u8>()
                                    / std::mem::size_of::<f32>(),
                            )
                            .to_vec();
                            colors = Some(b);
                        }
                        _ => panic!("暂时未实现"),
                    }
                });
            primitive.indices();
        });
    });
    for node in node.children() {
        entity.add_child(each_node(&node, buffers, images));
    }
    entity
}

async fn run() {
    let mut app = app::App::new("123", Config::PowerHighPerformance).await;
    // let gltf = gltf::Gltf::open("Box.gltf").unwrap();
    let (document, buffers, images) = gltf::import("Box.gltf").unwrap();
    // dbg!(
    //     document.accessors().collect::<Vec<gltf::Accessor>>(),
    //     images.len()
    // );

    for scene in document.default_scene() {
        let mut entity = Entity::new("root");
        for node in scene.nodes() {
            entity.add_child(each_node(&node, &buffers, &images));
        }
    }
}
fn main() -> Result<(), Box<dyn Error>> {
    async_std::task::block_on(run());
    Ok(())
}
