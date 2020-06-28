use crate::scene::camera::Camera;
use crate::scene::node::Node;
use crate::trait_help::*;
use crate::{Color, Vector3};
pub struct DirectionalLight {
    cast_shadows: bool,
    shadow_map_size: u32,
    shadow_bias: f32,
    color: Color,
    direction: Vector3,
    camera: Camera,
}

impl DirectionalLight {
    pub fn new() -> Self {
        let height = 40.0;
        let width = 1.0 * height;
        let length = 1.0 * height;
        let camera = Camera::new_ortho(-width, width, -height, height, -length, length);
        return DirectionalLight {
            cast_shadows: true,
            shadow_map_size: 1000,
            shadow_bias: 0.001,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            direction: Vector3::new(-0.5, -0.70, 0.5),
            camera,
        };
    }
}

pub struct PointLight {
    cast_shadows: bool,
    shadow_map_size: u32,
    shadow_bias: f32,
    color: Color,
    range: u32,
    cameras: Vec<Camera>,
}

impl PointLight {
    fn new(node: &mut Node) -> Self {
        let mut cameras = vec![];
        for i in 0..6 {
            let mut v = Vector3::new(0.0, 0.0, 0.0);
            let a = i % 2;
            let mut up;
            match i {
                0 => up = Vector3::new(0.0, -1.0, 0.0),
                1 => up = Vector3::new(0.0, -1.0, 0.0),
                2 => up = Vector3::new(0.0, 0.0, 1.0),
                3 => up = Vector3::new(0.0, 0.0, -1.0),
                4 => up = Vector3::new(0.0, -1.0, 0.0),
                5 => up = Vector3::new(0.0, -1.0, 0.0),
                _ => panic!("error"),
            }
            let b = (i as f32 / 2.0).floor() as usize;
            let data: &mut [f32; 3] = v.as_mut();

            data[b] = if a == 0 { 1.0 } else { -1.0 };
            let near = 0.01;
            let far = 10.0;
            let camera = Camera::new_perspective(90.0, 1.0, near, far);
            let mut camera_node = Node::new();
            camera_node.lookat2(&v, &up);
            let node_pos = node.get_position();
            camera_node.set_position(node_pos.x, node_pos.y, node_pos.z);
            cameras.push(camera);
            // TODO
            // node.set_sync_callback(Box::new(|| unsafe {
            //     let pos = node.get_position();
            //     // camera_node.set_position(pos.x, pos.y, pos.z);
            //     // camera_node.set_position(1.0, 1.0, 1.0);
            // }));
            // event.on('sync', () => {
            //     camera.node.setPosition(node.getPosition());
            // });
        }
        return PointLight {
            cameras,
            cast_shadows: true,
            shadow_map_size: 1000,
            shadow_bias: 0.001,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            range: 10,
        };
    }
}
