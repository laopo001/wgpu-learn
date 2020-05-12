use crate::ecs::components::camera::CameraComponent;
use crate::ecs::components::mesh::MeshComponent;
use crate::model::mesh::Mesh;
use crate::scene::camera::Camera;
use crate::scene::node::Node;
use crate::scene::Scene;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Entity {
    pub __node: Node,
    name: String,
    tags: Vec<String>,
    scene: *mut Scene,
    pub parent: *mut Entity,
    pub children: Vec<Box<Entity>>,
    pub mesh_component: Option<Rc<RefCell<MeshComponent>>>,
    pub camera_component: Option<Rc<RefCell<CameraComponent>>>,
    pub initialized: bool,
}
use core::ops::{Deref, DerefMut};
impl Deref for Entity {
    type Target = Node;
    fn deref<'a>(&'a self) -> &'a Self::Target {
        &self.__node
    }
}
impl DerefMut for Entity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.__node
    }
}

pub enum Component {
    Mesh {
        mesh: Mesh,
    },
    Camera {
        fov: f32,
        aspect: f32,
        near: f32,
        far: f32,
    },
}

impl Entity {
    pub fn new(name: &str) -> Box<Self> {
        return Box::new(Entity {
            __node: Node::new(),
            name: name.to_string(),
            tags: vec![],
            parent: std::ptr::null_mut(),
            children: vec![],
            mesh_component: None,
            camera_component: None,
            scene: std::ptr::null_mut(),
            initialized: false,
        });
    }
    pub fn set_component(&mut self, q: Component) {
        match q {
            Component::Mesh { mesh } => {
                let c = Rc::new(RefCell::new(MeshComponent::new(self, mesh)));
                if let Some(s) = self.scene() {
                    s.systems.add_mesh_component(c.clone());
                }

                self.mesh_component = Some(c);
            }
            Component::Camera {
                fov,
                aspect,
                near,
                far,
            } => {
                let mut c = Rc::new(RefCell::new(CameraComponent::new(
                    self,
                    Camera::new(fov, aspect, near, far),
                )));
                if let Some(s) = self.scene() {
                    s.systems.add_camera_component(c.clone());
                }

                self.camera_component = Some(c);
            }
            _ => panic!("Component"),
        }
    }
    pub(crate) fn set_scene(&mut self, scene: *mut Scene) {
        self.scene = scene;
    }
    pub(crate) fn scene(&mut self) -> Option<&mut Scene> {
        unsafe {
            if self.scene_ptr().is_null() {
                return None;
            }
            return Some(&mut *self.scene_ptr() as &mut Scene);
        }
    }
    pub(crate) fn scene_ptr(&mut self) -> *mut Scene {
        if self.root().scene.is_null() {}
        return self.root().scene;
    }
    pub fn parent(&mut self) -> Option<&mut Self> {
        unsafe {
            if (self.parent.is_null()) {
                return None;
            }
            return Some(&mut *self.parent as &mut Entity);
        }
    }
    pub fn root(&mut self) -> &mut Self {
        unsafe {
            if (self.parent().is_none()) {
                return self;
            } else {
                return self.parent().unwrap().root();
            }
        }
    }
    pub fn add_child(&mut self, mut child: Box<Self>) {
        self.__node.add_child(&mut child.__node);
        child.parent = self as *mut Entity;
        // if (self.enabled) {
        //     child.enabled = true;
        // } else {
        //     child.enabled = false;
        // }
        self.children.push(child);
    }
    pub fn get_by_name(&mut self, name: &str) -> Option<&mut Self> {
        if (self.name == name) {
            return Some(self);
        }
        for x in self.children.iter_mut() {
            return x.get_by_name(name);
        }
        return None;
    }
}
