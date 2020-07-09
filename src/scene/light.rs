use crate::core::color::Color;
use crate::scene::camera::Camera;
use crate::scene::node::Node;
use crate::trait_help::*;
use crate::Vector3;
pub struct DirectionalLight {
    pub cast_shadows: bool,
    pub shadow_map_size: u32,
    pub shadow_bias: f32,
    pub color: Color,
    pub direction: Vector3,
    pub camera: Camera,
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

#[derive(Debug)]
pub struct PointLight {
    pub cast_shadows: bool,
    pub shadow_map_size: u32,
    pub shadow_bias: f32,
    pub color: Color,
    pub range: f32,
    pub cameras: Vec<Camera>,
    pub camera_nodes: Vec<Box<Node>>,
    pub intensity: f32,
}

impl PointLight {
    pub fn new(range: f32, color: Color) -> Self {
        let mut cameras = vec![];
        let mut camera_nodes = vec![];
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
            let mut camera_node = Box::new(Node::new());
            camera_node.lookat2(&v, &up);
            // let node_pos = node.get_position();
            // camera_node.set_position(node_pos.x, node_pos.y, node_pos.z);
            cameras.push(camera);
            camera_nodes.push(camera_node);
        }
        // node.set_sync_node(
        //     camera_nodes
        //         .iter_mut()
        //         .map(|x| return (*x).as_mut() as *mut Node)
        //         .collect(),
        // );
        return PointLight {
            cameras,
            cast_shadows: true,
            shadow_map_size: 1000,
            shadow_bias: 0.001,
            color,
            range,
            camera_nodes,
            intensity: 1.0,
        };
    }
    pub fn tobase(&mut self) -> PointLightBase {
        PointLightBase {
            intensity: self.intensity,
            color: self.color.into(),
            pos: *self.camera_nodes[0].get_position().as_ref(),
        }
    }
}

pub struct SpotLight {
    pub cast_shadows: bool,
    pub shadow_map_size: u32,
    pub shadow_bias: f32,
    pub color: Color,
    pub range: f32,
    pub cameras: Vec<Camera>,
    pub camera_nodes: Vec<Box<Node>>,
    pub angle: f32,
    pub intensity: f32,
    pub smoothness: f32,
}
impl SpotLight {
    pub fn new() -> Self {
        let mut cameras = vec![];
        let mut camera_nodes = vec![];
        let cone_angle = 75.0;

        let near = 0.01;
        let far = 20.0;
        let camera = Camera::new_perspective(90.0, 1.0, near, far);
        let mut camera_node = Box::new(Node::new());

        cameras.push(camera);
        camera_nodes.push(camera_node);

        return Self {
            cameras,
            cast_shadows: true,
            shadow_map_size: 512,
            shadow_bias: 0.001,
            color: Color::new(1.0, 1.0, 1.0, 1.0),
            range: 20.0,
            camera_nodes,
            angle: cone_angle,
            intensity: 1.0,
            smoothness: 1.0,
        };
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PointLightBase {
    pub pos: [f32; 3],
    pub intensity: f32,
    pub color: [f32; 3],
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct SpotLightBase {
    pub pos: [f32; 3],
    pub angle: f32,
    pub color: [f32; 3],
    pub range: f32,
    pub dir: [f32; 3],
    pub smoothness: f32,
    pub intensity: f32,
}
