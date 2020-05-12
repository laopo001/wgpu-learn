use crate::ecs::components::camera::CameraComponent;
use crate::ecs::components::mesh::MeshComponent;
use std::cell::RefCell;
use std::rc::Rc;
pub struct ComponentSystem<T> {
    components: Vec<Rc<RefCell<T>>>,
}
impl<T> ComponentSystem<T> {
    pub fn new() -> Self {
        return Self { components: vec![] };
    }
}

pub struct System {
    mesh: ComponentSystem<MeshComponent>,
    camera: ComponentSystem<CameraComponent>,
}

impl System {
    pub fn new() -> Self {
        return Self {
            mesh: ComponentSystem::<MeshComponent>::new(),
            camera: ComponentSystem::<CameraComponent>::new(),
        };
    }
    pub fn set_mesh_component(&mut self, c: Rc<RefCell<MeshComponent>>) {
        self.mesh.components.push(c)
    }
}
