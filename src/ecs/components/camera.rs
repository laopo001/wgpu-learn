use crate::ecs::component::BaseComponent;
use crate::ecs::entity::Entity;
use crate::scene::camera::Camera;

pub struct CameraComponent {
    __super: BaseComponent,
    pub camera: Camera,
    entity: *mut Entity,
}

impl CameraComponent {
    pub fn new(entity: &mut Entity, camera: Camera) -> CameraComponent {
        return CameraComponent {
            __super: BaseComponent::new(),
            camera: camera,
            entity: entity,
        };
    }
    pub unsafe fn entity(&mut self) -> &mut Entity {
        return &mut *self.entity;
    }
}

extends2!(CameraComponent, BaseComponent);
