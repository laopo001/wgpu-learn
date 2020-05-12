use crate::ecs::components::camera::CameraComponent;
use crate::ecs::components::mesh::MeshComponent;

pub struct ComponentSystem<T> {
    components: Vec<T>,
}
impl<T> ComponentSystem<T> {
    pub fn new() -> Self {
        return Self { components: vec![] };
    }
}

struct System {
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
}
