use crate::app::App;
use crate::core::index_buffer::IndexBuffer;
use crate::core::vertex_buffer::VertexBuffer;
use crate::model::material::Material;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Mesh {
    pub material: Material,
    pub vertex_buffer: Option<VertexBuffer>,
    pub index_buffer: Option<Rc<RefCell<IndexBuffer>>>,
}
impl Mesh {
    pub fn new(app: &App) -> Self {
        return Mesh {
            vertex_buffer: None,
            index_buffer: None,
            material: Material::new(app),
        };
    }
    pub fn set_vertex_buffer(&mut self, mut vertex_buffer: VertexBuffer) {
        // let v = Rc::new(RefCell::new(vertex_buffer));
        self.material.shader.vertex_buffer =
            std::ptr::NonNull::new(&mut vertex_buffer as *mut VertexBuffer);
        self.vertex_buffer = Some(vertex_buffer);
    }
    pub fn set_index_buffer(&mut self, index_buffer: IndexBuffer) {
        self.index_buffer = Some(Rc::new(RefCell::new(index_buffer)));
    }
}
