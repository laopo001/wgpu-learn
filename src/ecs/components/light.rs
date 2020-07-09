use crate::ecs::component::BaseComponent;
use crate::ecs::entity::Entity;
use crate::scene::light::{DirectionalLight, PointLight, SpotLight};

pub enum Light {
    DirectionalLight(DirectionalLight),
    PointLight(PointLight),
    SpotLight(SpotLight),
}

pub struct LightComponent {
    __super: BaseComponent,
    pub light: Light,
    entity: *mut Entity,
}

impl LightComponent {
    pub fn new(entity: &mut Entity, light: Light) -> LightComponent {
        return LightComponent {
            __super: BaseComponent::new(),
            light: light,
            entity: entity,
        };
    }
    pub unsafe fn entity(&mut self) -> &mut Entity {
        return &mut *self.entity;
    }
}

extends2!(LightComponent, BaseComponent);
