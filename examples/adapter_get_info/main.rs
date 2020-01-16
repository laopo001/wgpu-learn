fn main() {
    // HighPerformance 如果是笔记本则是独立显卡，默认是集显
    let adapter = wgpu::Adapter::request(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
        },
        wgpu::BackendBit::PRIMARY,
    )
    .unwrap();

    println!("{:?}", adapter.get_info())
}
