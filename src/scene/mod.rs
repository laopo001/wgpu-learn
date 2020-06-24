pub mod camera;
pub mod node;
use crate::app::App;
use crate::config::Uniform;
use crate::core::shader_var::{UniformBindingResource, UniformVar};
use crate::ecs::component::Component;
use crate::ecs::components::camera::CameraComponent;
use crate::ecs::entity::Entity;
use crate::ecs::system::System;
use camera::Camera;
use cgmath::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use zerocopy::AsBytes;
pub struct Scene {
    pub root: Box<Entity>,
    pub systems: System,
    app: *const App,
}
impl Scene {
    pub fn new() -> Self {
        let root = Entity::new("root");
        let mut s = Self {
            root,
            systems: System::new(),
            app: std::ptr::null(),
        };
        let p = &mut s as *mut Scene;
        s.root.set_scene(p);
        return s;
    }
    fn initialize_system(&mut self, entity: &mut Entity) {
        entity.initialized = true;
        if entity.camera_component.is_some() {
            // entity
            //     .camera_component
            //     .as_mut()
            //     .unwrap()
            //     .borrow_mut()
            //     .initialize();
            self.systems
                .camera
                .components
                .push(entity.camera_component.clone().unwrap());
        }
        if entity.mesh_component.is_some() {
            self.systems
                .mesh
                .components
                .push(entity.mesh_component.clone().unwrap());
        }
        for x in entity.children.iter_mut() {
            self.initialize_system(x);
        }
    }
    pub fn initialize(&mut self) {
        let p = self.root.as_mut() as *mut Entity;
        unsafe {
            self.initialize_system(&mut *p as &mut Entity);
        };
    }
    pub fn draw(&mut self, app: &mut App) {
        unsafe {
            let camera = self.active_camera();
            let view = camera
                .borrow_mut()
                .entity()
                .get_world_transform()
                .clone()
                .invert()
                .unwrap();

            let projection = camera.borrow_mut().camera.get_perspective().clone();
            let view_projection = (projection * view);

            for mesh_c in self.systems.mesh.components.iter_mut() {
                let model = *(mesh_c.borrow_mut().entity().get_world_transform());
                // let model_view_projection_matrix = view_projection * model;
                // let mx_ref: &[f32; 16] = model_view_projection_matrix.as_ref();
                let view_projection_matrix_ref: &[f32; 16] = view_projection.as_ref();
                let model_matrix_ref: &[f32; 16] = model.as_ref();
                let mut mx_ref: Vec<f32> = view_projection_matrix_ref.clone().to_vec();
                mx_ref.extend_from_slice(model_matrix_ref);

                let uniform_buf = app.device.create_buffer_with_data(
                    mx_ref.as_bytes(),
                    wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
                );
                mesh_c.borrow_mut().mesh.material.set_uniform_vars(
                    Uniform::ModelViewProjectionMatrix,
                    UniformVar {
                        resource: UniformBindingResource::Buffer {
                            buffer: uniform_buf,
                            range: 0..64,
                        },
                        ty: wgpu::BindingType::UniformBuffer { dynamic: false },
                        visibility: wgpu::ShaderStage::VERTEX,
                    },
                );
                app.draw_mesh(&mut mesh_c.borrow_mut().mesh);
            }
        }
    }
    pub fn active_camera(&mut self) -> Rc<RefCell<CameraComponent>> {
        if (self.systems.camera.components.len() == 0) {
            panic!("添加一个相机组件");
        }
        return self.systems.camera.components[0].clone();
    }
}
