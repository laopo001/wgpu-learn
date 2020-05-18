use crate::core::shader::Shader;
use crate::model::mesh::Mesh;
use crate::scene::Scene;
use crate::{
    config::{Config, Event},
    Color,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub trait FnBox {
    fn call_box(&mut self, v: &mut App);
}

impl<F: FnMut(&mut App)> FnBox for F {
    fn call_box(&mut self, v: &mut App) {
        (*self)(v)
    }
}

type Task = Box<dyn FnBox + 'static>;

// type BoxFnOnce = Box<dyn FnOnce() + 'static>;

pub struct App {
    pub window: winit::window::Window,
    pub size: winit::dpi::PhysicalSize<u32>,
    event_loop: winit::event_loop::EventLoop<()>,
    pub adapter: wgpu::Adapter,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain: wgpu::SwapChain,
    pub event: HashMap<Event, Vec<Task>>,
    pub array: Vec<i32>,
    _clear_color: Color,
    pub scene: Scene,
}

impl Into<wgpu::PowerPreference> for Config {
    fn into(self) -> wgpu::PowerPreference {
        match self {
            Config::PowerDefault => wgpu::PowerPreference::Default,
            Config::PowerLowPower => wgpu::PowerPreference::LowPower,
            Config::PowerHighPerformance => wgpu::PowerPreference::HighPerformance,
            _ => panic!("Config into wgpu::PowerPreference fail"),
        }
    }
}

// use wgpu::PowerPreference as AppPower;
use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

impl App {
    pub async fn new(title: &str, power: Config) -> Self {
        let event_loop = EventLoop::new();
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder.with_title(title);
        let window = builder.build(&event_loop).unwrap();
        #[cfg(not(target_arch = "wasm32"))]
        {
            env_logger::init();
        }
        #[cfg(target_arch = "wasm32")]
        {
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            console_log::init().expect("could not initialize logger");
            use winit::platform::web::WindowExtWebSys;
            // On wasm, append the canvas to the document body
            web_sys::window()
                .and_then(|win| win.document())
                .and_then(|doc| doc.body())
                .and_then(|body| {
                    body.append_child(&web_sys::Element::from(window.canvas()))
                        .ok()
                })
                .expect("couldn't append canvas to document body");
        }

        let size = window.inner_size();
        let instance = wgpu::Instance::new();
        // crate::util::console_log("1");
        let surface = unsafe { instance.create_surface(&window) };
        // crate::util::console_log("2");
        let adapter = instance
            .request_adapter(
                &wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::Default,
                    compatible_surface: Some(&surface),
                },
                wgpu::BackendBit::PRIMARY,
            )
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    extensions: wgpu::Extensions {
                        anisotropic_filtering: false,
                    },
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .unwrap();
        let mut swap_chain = device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: crate::config::TextureFormat,
                width: size.width as u32,
                height: size.height as u32,
                present_mode: wgpu::PresentMode::Mailbox,
            },
        );
        return App {
            event_loop,
            window,
            size,
            adapter,
            surface,
            device,
            queue,
            swap_chain,
            event: HashMap::new(),
            array: vec![],
            _clear_color: Color::BLACK,
            scene: Scene::new(),
        };
    }
    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_info_adapter(&self) -> wgpu::AdapterInfo {
        return self.adapter.get_info();
    }
    pub fn set_clear_color(&mut self, color: Color) {
        self._clear_color = color;
    }
    // pub fn set_shader(&self, shader: Shader) {

    // }
    pub fn start(mut self) {
        unsafe {
            let p_app = &mut self as *mut App;
            // let event_loop = self.event_loop;
            let App {
                event_loop,
                window,
                size,
                adapter,
                surface,
                device,
                queue,
                swap_chain,
                mut event,
                array,
                _clear_color,
                mut scene,
            } = self;
            scene.initialize();
            scene.draw(&mut *p_app as &mut App);
            event
                .get_mut(&Event::Start)
                .get_or_insert(&mut vec![])
                .iter_mut()
                .for_each(|e| unsafe {
                    (e).call_box(std::mem::transmute::<*mut App, &mut App>(p_app));
                });
            event_loop.run(move |e, _, control_flow| {
                *control_flow = winit::event_loop::ControlFlow::Poll;
                match e {
                    winit::event::Event::MainEventsCleared => window.request_redraw(),
                    // 更新
                    winit::event::Event::RedrawRequested(_) => {
                        event
                            .get_mut(&Event::Update)
                            .get_or_insert(&mut vec![])
                            .iter_mut()
                            .for_each(|e| unsafe {
                                (e).call_box(std::mem::transmute::<*mut App, &mut App>(p_app));
                            });
                        // scene.draw(&mut *p_app as &mut App); // TODO
                    }
                    // 关闭
                    winit::event::Event::WindowEvent {
                        event: winit::event::WindowEvent::CloseRequested,
                        ..
                    } => {
                        event
                            .get_mut(&Event::End)
                            .get_or_insert(&mut vec![])
                            .iter_mut()
                            .for_each(|e| unsafe {
                                (e).call_box(std::mem::transmute::<*mut App, &mut App>(p_app));
                            });
                        *control_flow = winit::event_loop::ControlFlow::Exit
                    }
                    _ => {}
                }
            });
        }
    }
    pub fn on<F>(&mut self, e: Event, task: F)
    where
        F: FnMut(&mut App) + 'static,
    {
        if let Some(v) = self.event.get_mut(&e) {
            v.push(Box::new(task));
        } else {
            self.event.insert(e, vec![Box::new(task)]);
        }
    }
    pub fn create_wgpu_texture(
        &self,
        img_data: &Vec<u8>,
        width: u32,
        height: u32,
    ) -> wgpu::TextureView {
        log::info!(
            "create_wgpu_texture: Copying image of size {},{} to gpu",
            width,
            height,
        );
        unsafe {
            let texels = img_data;
            let texture_extent = wgpu::Extent3d {
                width: width,
                height: height,
                depth: 1,
            };
            let texture = self.device.create_texture(&wgpu::TextureDescriptor {
                size: texture_extent,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: crate::config::TextureFormat,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
                label: None,
            });
            let texture_view = texture.create_default_view();
            let temp_buf = self
                .device
                .create_buffer_with_data(texels, wgpu::BufferUsage::COPY_SRC);
            let mut init_encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

            init_encoder.copy_buffer_to_texture(
                wgpu::BufferCopyView {
                    buffer: &temp_buf,
                    offset: 0,
                    bytes_per_row: 4 * width,
                    rows_per_image: 0,
                },
                wgpu::TextureCopyView {
                    texture: &texture,
                    mip_level: 0,
                    array_layer: 0 as u32,
                    origin: wgpu::Origin3d::ZERO,
                },
                wgpu::Extent3d {
                    width: width,
                    height: height,
                    depth: 1,
                },
            );
            self.queue.submit(Some(init_encoder.finish()));
            texture_view
        }
    }
    pub fn draw_mesh(&mut self, mesh: &mut Mesh) {
        let index_buffer = mesh
            .index_buffer
            .as_mut()
            .expect("get index_buffer")
            .get_wgpu_index_buffer(&self);
        let vertex_buffer = mesh
            .vertex_buffer
            .as_mut()
            .expect("get vertex_buffer")
            .get_wgpu_vertex_buffer(&self);
        mesh.material.update_shader(self);
        // mesh.material
        //     .shader
        //     .set_vertex_buffer(mesh.vertex_buffer.clone().expect("").clone());
        mesh.material.shader.get_bind();
        let frame = self
            .swap_chain
            .get_next_texture()
            .expect("Timeout when acquiring next swap chain texture");
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::BLACK,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(
                mesh.material
                    .shader
                    .render_pipeline
                    .as_ref()
                    .expect("get render_pipeline"),
            );
            rpass.set_bind_group(
                0,
                mesh.material
                    .shader
                    .bind_group
                    .as_ref()
                    .expect("get bind_group"),
                &[],
            );

            rpass.set_index_buffer(index_buffer.slice(..));
            rpass.set_vertex_buffer(0, vertex_buffer.slice(..));
            // rpass.draw(0..3, 0..1);
            rpass.draw_indexed(0..6 as u32, 0, 0..1);
        }
        self.queue.submit(Some(encoder.finish()));
    }
}
