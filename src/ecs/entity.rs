use crate::scene::node::Node;
#[derive(Debug)]
pub struct Entity {
    pub __node: Node,
    name: String,
    tags: Vec<String>,
    pub parent: *mut Entity,
    pub children: Vec<*mut Entity>,
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
    pub fn new(name: &str) -> Self {
        return Entity {
            __node: Node::new().name(name),
            name: name.to_string(),
            tags: vec![],
            parent: std::ptr::null_mut(),
            children: vec![],
        };
    }
    pub fn parent(&mut self) -> Option<&mut Self> {
        unsafe {
            if (self.parent.is_null()) {
                return None;
            }
            return Some(&mut *self.parent as &mut Entity);
        }
    }
    pub fn add_child(&mut self, child: &mut Self) {
        self.__node.add_child(&mut child.__node);
        child.parent = self;
        self.children.push(child);
    }
    pub fn get_by_name(&mut self, name: &str) -> Option<&mut Self> {
        if (self.name == name) {
            return Some(self);
        }
        unsafe {
            for x in self.children.iter_mut() {
                return (**x).get_by_name(name);
            }
            return None;
        }
    }
}
