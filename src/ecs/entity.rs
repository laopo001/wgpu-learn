use crate::ecs::components::mesh::MeshComponent;
use crate::model::mesh::Mesh;
use crate::scene::node::Node;
pub struct Entity {
    pub __node: Node,
    name: String,
    tags: Vec<String>,
    pub parent: *mut Entity,
    pub children: Vec<Box<Entity>>,
    pub mesh_component: Option<MeshComponent>,
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

impl Entity {
    pub fn new(name: &str) -> Box<Self> {
        return Box::new(Entity {
            __node: Node::new(),
            name: name.to_string(),
            tags: vec![],
            parent: std::ptr::null_mut(),
            children: vec![],
            mesh_component: None,
        });
    }
    pub fn set_mesh_component(&mut self, mesh: Mesh) {
        let mut c = MeshComponent::new(mesh);
        self.mesh_component = Some(c);
    }

    pub fn parent(&mut self) -> Option<&mut Self> {
        unsafe {
            if (self.parent.is_null()) {
                return None;
            }
            return Some(&mut *self.parent as &mut Entity);
        }
    }
    pub fn add_child(&mut self, mut child: Box<Self>) {
        self.__node.add_child(&mut child.__node);
        child.parent = self as *mut Entity;
        self.children.push(child);
        if (!self.enabled) {
            self.enabled = false;
        } else {
            self.enabled = true;
        }
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
