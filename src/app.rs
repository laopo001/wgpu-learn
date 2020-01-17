use crate::config::{Config, Event};
use std::collections::HashMap;

pub trait FnBox {
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}
type BoxFnOnce = Box<dyn FnOnce()>;

type Task = Box<dyn FnBox + Send + 'static>;

pub struct App {
    pub window: winit::window::Window,
    pub size: winit::dpi::PhysicalSize,
    pub event_loop: winit::event_loop::EventLoop<()>,
    pub adapter: wgpu::Adapter,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub swap_chain: wgpu::SwapChain,
    pub event: HashMap<Event, Vec<BoxFnOnce>>,
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

impl App {
    pub fn new(title: &str, power: Config) -> Self {
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: power.into(),
            },
            wgpu::BackendBit::PRIMARY,
        )
        .expect("获取adapter");
        let event_loop = winit::event_loop::EventLoop::new();
        let (window, size, surface) = {
            let window = winit::window::Window::new(&event_loop).unwrap();
            let size = window.inner_size().to_physical(window.hidpi_factor());
            let surface = wgpu::Surface::create(&window);
            (window, size, surface)
        };
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        });
        let mut swap_chain = device.create_swap_chain(
            &surface,
            &wgpu::SwapChainDescriptor {
                usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
                format: wgpu::TextureFormat::Bgra8UnormSrgb,
                width: size.width.round() as u32,
                height: size.height.round() as u32,
                present_mode: wgpu::PresentMode::Vsync,
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
        };
    }
    pub fn get_info_adapter(&self) -> wgpu::AdapterInfo {
        return self.adapter.get_info();
    }
    pub fn start(self) {
        let App {
            event_loop,
            window,
            size,
            adapter,
            surface,
            device,
            queue,
            mut swap_chain,
            event,
        } = self;
        event_loop.run(move |event, _, control_flow| {
            *control_flow = winit::event_loop::ControlFlow::Poll;
            match event {
                winit::event::Event::MainEventsCleared => window.request_redraw(),
                winit::event::Event::RedrawRequested(_) => {
                    // !todo
                }
                winit::event::Event::WindowEvent {
                    event: winit::event::WindowEvent::CloseRequested,
                    ..
                } => {
                    dbg!("exit");
                    *control_flow = winit::event_loop::ControlFlow::Exit
                }
                _ => {}
            }
        });
    }
    pub fn on(&mut self, e: Event, task: BoxFnOnce) {
        self.event.get_mut(&e).get_or_insert(&mut vec![]).push(task);
    }
}
