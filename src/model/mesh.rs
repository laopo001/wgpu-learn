use crate::app::App;
use crate::core::index_buffer::IndexBuffer;
use crate::core::vertex_buffer::VertexBuffer;
use crate::model::material::Material;
use std::cell::RefCell;
use std::rc::Rc;
pub struct Mesh {
    pub material: Material,
    pub vertex_buffer: Option<VertexBuffer>,
    pub index_buffer: Option<IndexBuffer>,
    app: *const App,
}
impl Mesh {
    fn app(&self) -> &App {
        if self.app.is_null() {
            panic!("app get not")
        }
        unsafe {
            return &*self.app;
        }
    }
    pub fn new(app: &App) -> Self {
        return Mesh {
            vertex_buffer: None,
            index_buffer: None,
            material: Material::new(app),
            app: app as *const App,
        };
    }
    pub fn set_vertex_buffer(&mut self, mut vertex_buffer: VertexBuffer) {
        // let v = Rc::new(RefCell::new(vertex_buffer));
        self.material.shader.vertex_buffer =
            std::ptr::NonNull::new(&mut vertex_buffer as *mut VertexBuffer);
        self.vertex_buffer = Some(vertex_buffer);
    }
    pub fn set_index_buffer(&mut self, index_buffer: IndexBuffer) {
        self.index_buffer = Some(index_buffer);
    }
}
