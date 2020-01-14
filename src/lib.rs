#![allow(non_snake_case)]

pub enum ShaderStage {
    Vertex,
    Fragment,
    Compute,
}

pub fn load_glsl(code: &str, stage: ShaderStage) -> Vec<u32> {
    let ty = match stage {
        ShaderStage::Vertex => glsl_to_spirv::ShaderType::Vertex,
        ShaderStage::Fragment => glsl_to_spirv::ShaderType::Fragment,
        ShaderStage::Compute => glsl_to_spirv::ShaderType::Compute,
    };

    wgpu::read_spirv(glsl_to_spirv::compile(&code, ty).unwrap()).unwrap()
}

pub fn Texture2TextureCopyView<'a>(t: &'a wgpu::Texture) -> wgpu::TextureCopyView<'a> {
    wgpu::TextureCopyView {
        texture: t,
        mip_level: 0,
        array_layer: 0,
        origin: wgpu::Origin3d { x: 0, y: 0, z: 0 },
    }
}
