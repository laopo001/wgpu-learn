use crate::ecs::components::camera::CameraComponent;
use crate::ecs::components::light::LightComponent;
use crate::ecs::components::mesh::MeshComponent;
use std::cell::RefCell;
use std::rc::Rc;
pub struct BaseComponentSystem<T> {
    pub components: Vec<Rc<RefCell<T>>>,
}
impl<T> BaseComponentSystem<T> {
    pub fn new() -> Self {
        return Self { components: vec![] };
    }
}

pub struct System {
    pub mesh: BaseComponentSystem<MeshComponent>,
    pub camera: BaseComponentSystem<CameraComponent>,
    pub light: BaseComponentSystem<LightComponent>,
}

impl System {
    pub fn new() -> Self {
        return Self {
            mesh: BaseComponentSystem::<MeshComponent>::new(),
            camera: BaseComponentSystem::<CameraComponent>::new(),
            light: BaseComponentSystem::<LightComponent>::new(),
        };
    }
    pub fn add_mesh_component(&mut self, c: Rc<RefCell<MeshComponent>>) {
        self.mesh.components.push(c)
    }
    pub fn add_camera_component(&mut self, c: Rc<RefCell<CameraComponent>>) {
        self.camera.components.push(c)
    }
    pub fn add_light_component(&mut self, c: Rc<RefCell<LightComponent>>) {
        self.light.components.push(c)
    }
}
