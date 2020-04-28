use async_std::task;

async fn run() {
    // HighPerformance 如果是笔记本则是独立显卡，默认是集显
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    // let size = window.inner_size();
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

    println!("{:?}", adapter.get_info())
}

fn main() {
    task::block_on(run());
}
