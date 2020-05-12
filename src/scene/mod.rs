pub mod camera;
pub mod node;
use crate::ecs::entity::Entity;
use crate::ecs::system::System;
pub struct Scene {
    pub root: Box<Entity>,
    pub systems: System,
}
impl Scene {
    pub fn new() -> Self {
        let root = Entity::new("root");
        let mut s = Self {
            root,
            systems: System::new(),
        };
        let p = &mut s as *mut Scene;
        s.root.set_scene(p);
        return s;
    }
}
