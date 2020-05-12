use crate::ecs::component::BaseComponent;
use crate::model::mesh::Mesh;

pub struct MeshComponent {
    __super: BaseComponent,
    mesh: Option<Mesh>,
}

impl MeshComponent {
    pub fn new() -> MeshComponent {
        return MeshComponent {
            __super: BaseComponent::new(),
            mesh: None,
        };
    }
    pub fn mesh(&mut self, mesh: Mesh) {
        self.mesh = Some(mesh);
    }
}

extends2!(MeshComponent, BaseComponent);
