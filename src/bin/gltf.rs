use gltf::{
    accessor::DataType, buffer::Data as BufferData, image::Data as ImageData, Node, Semantic,
};
use std::error::Error;
use wgpu_learn::config::Config;
use wgpu_learn::ecs::entity::{Component, Entity};
use wgpu_learn::model::create_mesh::{create_mesh, CreateMeshParam};
use wgpu_learn::model::mesh::Mesh;
use wgpu_learn::trait_help::*;
use wgpu_learn::{app, Matrix4};

fn each_node(node: &Node, buffers: &Vec<BufferData>, images: &Vec<ImageData>) -> Box<Entity> {
    let mut entity = Entity::new(node.name().unwrap_or(""));

    let data = node.transform().matrix();
    let mat = Matrix4::from(data);
    let scale = mat.get_scale();
    let euler = mat.get_euler_angles();
    let transform = mat.get_translate();
    entity.set_position(transform.x, transform.y, transform.z);
    entity.set_euler_angles(euler.x, euler.y, euler.z);
    entity.set_local_scale(scale.x, scale.y, scale.z);

    node.mesh().map(|gltf_mesh| {
        // let m = Mesh::new();
        // dbg!(std::mem::size_of::<f32>());
        let mut positions: Vec<f32> = vec![];
        let mut normals: Option<Vec<f32>> = None;
        let mut colors: Option<Vec<f32>> = None;
        let mut uvs: Option<Vec<f32>> = None;
        let mut indices: Option<Vec<u32>> = None;
        gltf_mesh.primitives().for_each(|primitive| {
            primitive
                .attributes()
                .for_each(|(semantic, accessor)| unsafe {
                    let index = accessor.view().unwrap().buffer().index();
                    let data_type_size = match accessor.data_type() {
                        DataType::I8 | DataType::U8 => 1,
                        DataType::I16 | DataType::U16 => 2,
                        DataType::U32 | DataType::F32 => 4,
                    };
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
            primitive.indices().map(|accessor| unsafe {
                let index = accessor.view().unwrap().buffer().index();
                // dbg!(&accessor.data_type());
                let buffer = buffers[index].0
                    [accessor.offset()..(accessor.offset() + accessor.count() * accessor.size())]
                    .to_vec();

                match accessor.data_type() {
                    DataType::U16 => {
                        let b = std::slice::from_raw_parts(
                            buffer.as_ptr() as *const u16,
                            buffer.len() * std::mem::size_of::<u8>() / std::mem::size_of::<u16>(),
                        )
                        .to_vec();
                        indices = Some(b.into_iter().map(|x| x as u32).collect());
                    }
                    DataType::U32 => {
                        indices = Some(
                            std::slice::from_raw_parts(
                                buffer.as_ptr() as *const u32,
                                buffer.len() * std::mem::size_of::<u8>()
                                    / std::mem::size_of::<u32>(),
                            )
                            .to_vec(),
                        );
                    }
                    _ => panic!("error"),
                };
            });
        });
        let mesh = create_mesh(CreateMeshParam {
            positions,
            normals,
            colors,
            uvs,
            uvs1: None,
            indices,
        });
        entity.set_component(Component::Mesh { mesh })
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
        let mut entity = Entity::new("gltf_root");
        for node in scene.nodes() {
            entity.add_child(each_node(&node, &buffers, &images));
        }
        app.scene.root.add_child(entity);
    }
    app.start();
}
fn main() -> Result<(), Box<dyn Error>> {
    async_std::task::block_on(run());
    Ok(())
}
