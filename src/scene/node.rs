use crate::trait_help::{Into, Matrix4Plus, QuatPlus, Vector3Plus, Vector4Plus};
use crate::Matrix4 as Mat4;
use crate::Point3;
use crate::Quat;
use crate::Vector3 as Vec3;
use cgmath::prelude::SquareMatrix;
use cgmath::prelude::Transform;
use cgmath::Zero;
use std::cell::UnsafeCell;

#[derive(Debug)]
pub struct Node {
    pub(crate) local_position: Vec3,
    pub(crate) local_rotation: Quat,
    pub(crate) local_scale: Vec3,
    pub(crate) local_euler_angle: Vec3,
    pub(crate) local_transform: Mat4,
    pub(crate) world_position: Vec3,
    pub(crate) world_rotation: Quat,
    pub(crate) world_euler_angle: Vec3,
    pub(crate) world_transform: Mat4,
    pub parent: *mut Node,
    pub children: Vec<*mut Node>,
    _dirty_local: bool,
    _dirty_world: bool,
    enabled: bool,
    name: String,
}

impl Node {
    pub fn new() -> Self {
        return Node {
            local_position: Vec3::zero(),
            local_rotation: Quat::zero(),
            local_euler_angle: Vec3::zero(),
            local_scale: Vec3::new(1.0, 1.0, 1.0),
            local_transform: Mat4::one(),
            world_position: Vec3::zero(),
            world_rotation: Quat::zero(),
            world_euler_angle: Vec3::zero(),
            world_transform: Mat4::one(),
            parent: std::ptr::null_mut(),
            children: vec![],
            _dirty_local: false,
            _dirty_world: false,
            enabled: true,
            name: "".to_string(),
        };
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn up(&mut self) -> Vec3 {
        self.get_world_transform().get_x()
    }
    pub fn forward(&mut self) -> Vec3 {
        self.get_world_transform().get_y()
    }
    pub fn right(&mut self) -> Vec3 {
        self.get_world_transform().get_z()
    }
    pub fn lookat(&mut self, target: &mut Node) {
        let up = target.up();
        let target_location = target.get_position();
        let mat4 = Mat4::look_at(self.get_position().into2(), target_location.into2(), up);
        let mut quat = Quat::zero();
        quat.set_from_mat4(&mat4);
        self.set_rotation(&quat);
    }
    pub fn add_child(&mut self, child: &mut Node) {
        child.parent = self;
        self.children.push(child);
    }
    fn set_rotation(&mut self, rotation: &Quat) {
        unsafe {
            if (self.parent.is_null()) {
                self.local_rotation = rotation.clone();
            } else {
                let mut inv_parent_rot = (*self.parent).get_rotation().clone();
                inv_parent_rot.invert();
                self.local_rotation = inv_parent_rot;
                self.local_rotation = self.local_rotation * rotation;
            }

            if (!self._dirty_local) {
                self._dirtify(true);
            }
        }
    }
    fn get_rotation(&mut self) -> &Quat {
        unsafe {
            let world_transform_ptr = self.get_world_transform_ptr();
            self.world_rotation.set_from_mat4(&*world_transform_ptr);
            return &self.world_rotation;
        }
    }
    pub fn set_local_position(&mut self, x: f32, y: f32, z: f32) {
        self.local_position.set(x, y, z);
        if !self._dirty_local {
            self._dirtify(true);
        }
    }
    pub fn get_local_position(&self) -> &Vec3 {
        &self.local_position
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            if self.parent.is_null() {
                self.local_position.set(x, y, z);
            } else {
                let inv_parent_transform = (*self.parent).get_world_transform();
                let t_inv_parent_transform = inv_parent_transform.invert().unwrap();
                self.local_position = t_inv_parent_transform
                    .transform_point(Point3::new(x, y, z))
                    .into2();
            }
            if !self._dirty_local {
                self._dirtify(true);
            }
        }
    }
    pub fn get_position(&mut self) -> &Vec3 {
        unsafe {
            self.world_position = self.get_world_transform().get_translate();
        }
        return &self.world_position;
    }

    pub fn set_local_euler_angles(&mut self, x: f32, y: f32, z: f32) {
        self.local_rotation.set_from_euler_angles(x, y, z);
        if !self._dirty_local {
            self._dirtify(true);
        }
    }
    pub fn get_local_euler_angles(&mut self) -> &Vec3 {
        self.local_rotation
            .get_euler_angles(&mut self.local_euler_angle);
        return &self.local_euler_angle;
    }

    pub fn set_euler_angles(&mut self, x: f32, y: f32, z: f32) {
        self.local_rotation.set_from_euler_angles(x, y, z);
        unsafe {
            if !self.parent.is_null() {
                let mut inv_parent_rotation = (*self.parent).get_rotation().clone();
                inv_parent_rotation.invert();
                self.local_rotation = self.local_rotation * &inv_parent_rotation;
            }
            if !self._dirty_local {
                self._dirtify(true);
            }
        }
    }
    pub(crate) fn get_euler_angles(&mut self) -> &Vec3 {
        unsafe {
            let world_transform = self.get_world_transform_ptr();
            (*world_transform).get_euler_angles(&mut self.world_euler_angle);
            return &self.world_euler_angle;
        }
    }

    pub fn get_world_transform(&mut self) -> &mut Mat4 {
        unsafe {
            if self._dirty_local == false && self._dirty_world == false {
                return &mut self.world_transform as &mut Mat4;
            }
            if !self.parent.is_null() {
                (*self.parent).get_world_transform();
            }
            self._sync();
            return &mut self.world_transform as &mut Mat4;
        }
    }
    pub fn get_world_transform_ptr(&mut self) -> *mut Mat4 {
        unsafe {
            if self._dirty_local == false && self._dirty_world == false {
                return &mut self.world_transform as *mut Mat4;
            }
            if !self.parent.is_null() {
                (*self.parent).get_world_transform();
            }
            self._sync();
            return &mut self.world_transform as *mut Mat4;
        }
    }

    pub(crate) fn get_local_transform(&mut self) -> &mut Mat4 {
        if self._dirty_local {
            self._sync();
        }
        return &mut self.local_transform as &mut Mat4;
    }

    pub fn set_local_scale(&mut self, x: f32, y: f32, z: f32) {
        self.local_scale.set(x, y, z);
        if !self._dirty_local {
            self._dirtify(true);
        }
    }
    fn get_local_scale(&mut self) -> &Vec3 {
        &self.local_scale
    }

    fn _dirtify(&mut self, local: bool) {
        if local {
            self._dirty_local = true;
        }
        if !self._dirty_world {
            self._dirty_world = true;
            for item in self.children.iter() {
                unsafe {
                    (**item)._dirtify(false);
                }
            }
        }
    }
    pub fn sync_hierarchy(&mut self) {
        if !self.enabled {
            return;
        }
        if self._dirty_local || self._dirty_world {
            self._sync();
        }
        for i in 0..(self.children.len()) {
            unsafe {
                (*self.children[i]).sync_hierarchy();
            }
        }
    }
    pub fn _sync(&mut self) {
        unsafe {
            if self._dirty_local {
                self.local_transform.set_from_trs(
                    &self.local_position,
                    &self.local_rotation,
                    &self.local_scale,
                );
                self._dirty_local = false;
            }
            if self._dirty_world {
                if self.parent.is_null() {
                    self.world_transform = self.local_transform;
                } else {
                    self.world_transform = (*self.parent).world_transform * self.local_transform;
                }
                self._dirty_world = false;
            }
        }
    }
}

impl Clone for Node {
    fn clone(&self) -> Node {
        let mut c = Node::new();
        c.local_position = self.local_position.clone();
        c.local_rotation = self.local_rotation.clone();
        c.local_scale = self.local_scale.clone();
        // c._dirty_local = false;
        // c._dirty_world = false;
        for child in self.children.iter() {
            unsafe {
                let mut clone_child = (**child).clone();
                c.add_child(&mut clone_child);
            }
        }
        return c;
    }
}

#[test]
fn test_set_get_position() {
    let mut node = Node::new();
    node.set_position(1.0, 2.0, 3.0);
    assert_eq!(node.get_position().data(), Vec3::new(1.0, 2.0, 3.0).data());
}

#[test]
fn test_set_get_local_position() {
    let mut node = Node::new();
    node.set_local_position(1.0, 2.0, 3.0);
    assert_eq!(
        node.get_local_position().data(),
        Vec3::new(1.0, 2.0, 3.0).data()
    );
}

#[test]
fn test_child_set_get_position() {
    let mut node = Node::new();
    let mut child = Node::new();
    let mut grandson = Node::new();
    node.add_child(&mut child);
    child.add_child(&mut grandson);
    node.set_local_position(1.0, 2.0, 3.0);
    child.set_local_position(1.0, 2.0, 3.0);
    grandson.set_local_position(1.0, 2.0, 3.0);
    assert_eq!(
        grandson.get_position().data(),
        Vec3::new(3.0, 6.0, 9.0).data()
    );
    grandson.set_position(0.0, 0.0, 0.0);
    assert_eq!(
        grandson.get_position().data(),
        Vec3::new(0.0, 0.0, 0.0).data()
    );
    assert_eq!(
        grandson.get_local_position().data(),
        Vec3::new(-2.0, -4.0, -6.0).data()
    );
}

#[test]
fn test_child_set_get_local_angles() {
    let mut node = Node::new();
    node.set_local_euler_angles(1.0, 2.0, 3.0);
    assert_eq!(
        node.get_local_euler_angles()
            .data()
            .into_iter()
            .map(|x| x.round())
            .collect::<Box<[f32]>>(),
        Vec3::new(1.0, 2.0, 3.0).data()
    );
}

#[test]
fn test_child_set_get_angles() {
    let mut node = Node::new();
    let mut child = Node::new();
    let mut grandson = Node::new();
    node.add_child(&mut child);
    child.add_child(&mut grandson);
    node.set_local_euler_angles(1.0, 0.0, 0.0);
    child.set_local_euler_angles(1.0, 0.0, 0.0);
    grandson.set_local_euler_angles(1.0, 0.0, 0.0);
    assert_eq!(
        grandson
            .get_euler_angles()
            .data()
            .into_iter()
            .map(|x| x.round())
            .collect::<Box<[f32]>>(),
        Vec3::new(3.0, 0.0, 0.0).data()
    );
    grandson.set_euler_angles(0.0, 0.0, 0.0);
    assert_eq!(
        grandson
            .get_euler_angles()
            .data()
            .into_iter()
            .map(|x| x.round())
            .collect::<Box<[f32]>>(),
        Vec3::new(0.0, 0.0, 0.0).data()
    );
    assert_eq!(
        grandson
            .get_local_euler_angles()
            .data()
            .into_iter()
            .map(|x| x.round())
            .collect::<Box<[f32]>>(),
        Vec3::new(-2.0, 0.0, 0.0).data()
    );
}

#[test]
fn test_child_set_get_local_scale() {
    let mut node = Node::new();
    node.set_local_scale(1.0, 2.0, 3.0);
    assert_eq!(
        node.get_local_scale()
            .data()
            .into_iter()
            .map(|x| x.round())
            .collect::<Box<[f32]>>(),
        Vec3::new(1.0, 2.0, 3.0).data()
    );
}
