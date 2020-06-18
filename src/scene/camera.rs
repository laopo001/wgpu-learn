use crate::Matrix4;
use cgmath::prelude::Transform;
use cgmath::Rad;
pub struct Camera {
    pub far_clip: f32,
    pub near_clip: f32,
    pub projection_matrix: Matrix4,
}

impl Camera {
    pub fn new(fov: f32, aspect: f32, mut near: f32, far: f32) -> Self {
        near = near + 0.000001;
        let f = cgmath::Deg(fov);
        return Camera {
            far_clip: far,
            near_clip: near,
            projection_matrix: cgmath::perspective(f, aspect, near, far),
        };
    }
    pub fn set_perspective(&mut self, fov: f32, aspect: f32, mut near: f32, far: f32) {
        near = near + 0.000001;
        self.near_clip = near;
        self.far_clip = far;
        let f = Rad::from(cgmath::Deg(fov));
        self.projection_matrix = cgmath::perspective(f, aspect, near, far);
    }
    pub fn get_perspective(&self) -> &Matrix4 {
        return &self.projection_matrix;
    }
    pub fn set_ortho(&mut self, left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) {
        self.near_clip = near;
        self.far_clip = far;
        self.projection_matrix = cgmath::ortho(left, right, bottom, top, near, far);
    }
}
