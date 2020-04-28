use crate::{
    config::{Config, Event},
    Color,
};
use std::collections::HashMap;

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
        let window = winit::window::Window::new(&event_loop).unwrap();
        let size = window.inner_size();
        let instance = wgpu::Instance::new();
        let surface = unsafe { instance.create_surface(&window) };
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
            .request_device(&wgpu::DeviceDescriptor {
                extensions: wgpu::Extensions {
                    anisotropic_filtering: false,
                },
                limits: wgpu::Limits::default(),
            })
            .await
            .unwrap();
        let mut swap_chain = device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
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
        };
    }
    pub fn get_info_adapter(&self) -> wgpu::AdapterInfo {
        return self.adapter.get_info();
    }
    pub fn set_clear_color(&mut self, color: Color) {
        self._clear_color = color;
    }
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
            } = self;
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
                        // !todo
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
}
