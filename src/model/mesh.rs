use crate::app::App;
use crate::core::index_buffer::IndexBuffer;
use crate::core::vertex_buffer::VertexBuffer;
use crate::model::create_mesh::create_box;
use crate::model::material::Material;
use crate::Vector3;
use std::cell::RefCell;
use std::rc::Rc;
#[derive(Debug)]
pub struct Mesh {
    pub material: Material,
    pub vertex_buffer: Option<Box<VertexBuffer>>,
    pub index_buffer: Option<Box<IndexBuffer>>,
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
    pub fn new() -> Self {
        return Mesh {
            vertex_buffer: None,
            index_buffer: None,
            material: Material::new(),
            app: std::ptr::null() as *const App,
        };
    }
    pub fn set_vertex_buffer(&mut self, mut vertex_buffer: VertexBuffer) {
        // let v = Rc::new(RefCell::new(vertex_buffer));
        self.vertex_buffer = Some(Box::new(vertex_buffer));
        self.material.shader.vertex_buffer = std::ptr::NonNull::new(
            self.vertex_buffer.as_mut().unwrap().as_mut() as *mut VertexBuffer,
        );
    }
    pub fn set_index_buffer(&mut self, index_buffer: IndexBuffer) {
        self.index_buffer = Some(Box::new(index_buffer));
        self.material.shader.index_buffer = std::ptr::NonNull::new(
            self.index_buffer.as_mut().unwrap().as_mut() as *mut IndexBuffer,
        );
    }
    pub fn create_box(
        app: &App,
        half_extents: Option<Vector3>,
        width_segments: Option<u32>,
        length_segments: Option<u32>,
        height_segments: Option<u32>,
    ) -> Self {
        return create_box(
            app,
            half_extents,
            width_segments,
            length_segments,
            height_segments,
        );
    }
}
