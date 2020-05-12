use crate::ecs::component::BaseComponent;
use crate::ecs::entity::Entity;
use crate::model::mesh::Mesh;
pub struct MeshComponent {
    __super: BaseComponent,
    pub(crate) mesh: Mesh,
    entity: *mut Entity,
}

impl MeshComponent {
    pub fn new(entity: &mut Entity, m: Mesh) -> MeshComponent {
        return MeshComponent {
            __super: BaseComponent::new(),
            mesh: m,
            entity,
        };
    }
    pub unsafe fn entity(&mut self) -> &mut Entity {
        return &mut *self.entity;
    }
}

extends2!(MeshComponent, BaseComponent);
