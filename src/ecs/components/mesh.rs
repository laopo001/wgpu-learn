use crate::ecs::component::BaseComponent;
use crate::model::mesh::Mesh;

pub struct MeshComponent {
    __super: BaseComponent,
    mesh: Mesh,
}

impl MeshComponent {
    pub fn new(m: Mesh) -> MeshComponent {
        return MeshComponent {
            __super: BaseComponent::new(),
            mesh: m,
        };
    }
}

extends2!(MeshComponent, BaseComponent);
