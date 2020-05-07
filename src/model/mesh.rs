use crate::app::App;
use crate::core::index_buffer::IndexBuffer;
use crate::core::vertex_buffer::VertexBuffer;
use crate::model::material::Material;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Mesh {
    pub material: Material,
    pub vertex_buffer: Option<Rc<RefCell<VertexBuffer>>>,
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
    pub fn set_vertex_buffer(&mut self, vertex_buffer: VertexBuffer) {
        let v = Rc::new(RefCell::new(vertex_buffer));
        self.vertex_buffer = Some(v.clone());
        self.material.shader.vertex_buffer = Some(v);
    }
    pub fn set_index_buffer(&mut self, index_buffer: IndexBuffer) {
        self.index_buffer = Some(Rc::new(RefCell::new(index_buffer)));
    }
}
