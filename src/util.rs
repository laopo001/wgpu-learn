#[cfg(not(target_arch = "wasm32"))]
pub fn load_glsl(code: &str, stage: glsl_to_spirv::ShaderType) -> Vec<u32> {
    wgpu::read_spirv(glsl_to_spirv::compile(&code, stage).unwrap()).unwrap()
}

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = glslang)]
    fn compileGLSL(s: &str, t: &str) -> Vec<u32>;
}

#[cfg(target_arch = "wasm32")]
pub fn load_glsl(code: &str, stage: glsl_to_spirv::ShaderType) -> Vec<u32> {
    match stage {
        glsl_to_spirv::ShaderType::Vertex => return compileGLSL(code, "vertex"),
        glsl_to_spirv::ShaderType::Fragment => return compileGLSL(code, "fragment"),
    }
}
