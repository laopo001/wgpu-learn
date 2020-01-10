#![allow(unused)]
use wgpu_learn;

fn main() {
    use winit::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoop},
        window::WindowBuilder,
    };
    let event_loop = EventLoop::new();
    let (window, size, surface) = {
        let window = winit::window::Window::new(&event_loop).unwrap();
        let size = window.inner_size().to_physical(window.hidpi_factor());

        let surface = wgpu::Surface::create(&window);
        (window, size, surface)
    };
    let adapter = wgpu::Adapter::request(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
        },
        wgpu::BackendBit::PRIMARY,
    )
    .unwrap();
    let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });
    let vs_bytes = wgpu_learn::load_glsl(
        include_str!("./shaders/helloTriangle.vert"),
        wgpu_learn::ShaderStage::Vertex,
    );
    let fs_bytes = wgpu_learn::load_glsl(
        include_str!("./shaders/helloTriangle.frag"),
        wgpu_learn::ShaderStage::Fragment,
    );
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared => {
                // Application update code.
                // Queue a RedrawRequested event.
                window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Redraw the application.
                //
                // It's preferrable to render in this event rather than in MainEventsCleared, since
                // rendering in here allows the program to gracefully handle redraws requested
                // by the OS.
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                *control_flow = ControlFlow::Exit
            }
            // ControlFlow::Poll continuously runs the event loop, even if the OS hasn't
            // dispatched any events. This is ideal for games and similar applications.
            _ => *control_flow = ControlFlow::Poll,
            // ControlFlow::Wait pauses the event loop if no events are available to process.
            // This is ideal for non-game applications that only update in response to user
            // input, and uses significantly less power/CPU time than ControlFlow::Poll.
            // _ => *control_flow = ControlFlow::Wait,
        }
    });
    println!("Hello, world!");
}
