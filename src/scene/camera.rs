use crate::Matrix4;
use cgmath::prelude::Transform;
use cgmath::Rad;
pub struct Camera {
    pub far_clip: f32,
    pub near_clip: f32,
    pub projection_matrix: Matrix4,
}

impl Camera {
    pub fn new() -> Self {
        return Camera {
            far_clip: 1000.0,
            near_clip: 0.1,
            projection_matrix: Matrix4::one(),
        };
    }
    pub fn set_perspective(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
        self.near_clip = near;
        self.far_clip = far;
        self.projection_matrix = cgmath::perspective(Rad(fov), aspect, near, far);
        // self.projection_matrix
        //     .set_perspective(fov, aspect, near, far);
    }
}
