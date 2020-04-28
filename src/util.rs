use crate::ShaderStage;

#[cfg(not(target_arch = "wasm32"))]
pub fn load_glsl(code: &str, stage: ShaderStage) -> Vec<u32> {
    wgpu::read_spirv(glsl_to_spirv::compile(&code, stage).unwrap()).unwrap()
}
