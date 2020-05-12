use crate::ecs::component::BaseComponent;
use crate::scene::camera::Camera;

pub struct CameraComponent {
    __super: BaseComponent,
    camera: Camera,
}

impl CameraComponent {
    pub fn new() -> CameraComponent {
        return CameraComponent {
            __super: BaseComponent::new(),
            camera: Camera::new(),
        };
    }
}

extends2!(CameraComponent, BaseComponent);
