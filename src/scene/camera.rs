use crate::Matrix4;
use cgmath::prelude::Transform;
use cgmath::Rad;
#[derive(Debug)]
pub struct Camera {
    pub far_clip: f32,
    pub near_clip: f32,
    pub view_matrix: Matrix4,
}

impl Camera {
    pub fn new_perspective(fov: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = cgmath::Deg(fov);
        return Camera {
            far_clip: far,
            near_clip: near,
            view_matrix: cgmath::perspective(f, aspect, near, far),
        };
    }
    pub fn new_ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        return Camera {
            far_clip: far,
            near_clip: near,
            view_matrix: cgmath::ortho(left, right, bottom, top, near, far),
        };
    }
    pub fn set_perspective(&mut self, fov: f32, aspect: f32, mut near: f32, far: f32) {
        self.near_clip = near;
        self.far_clip = far;
        let f = Rad::from(cgmath::Deg(fov));
        self.view_matrix = cgmath::perspective(f, aspect, near, far);
    }
    pub fn get_view_matrix(&self) -> &Matrix4 {
        return &self.view_matrix;
    }
    pub fn set_ortho(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
        self.near_clip = near;
        self.far_clip = far;
        self.view_matrix = cgmath::ortho(left, right, bottom, top, near, far);
    }
}
