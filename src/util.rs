use crate::ShaderStage;

#[cfg(not(target_arch = "wasm32"))]
pub fn load_glsl(code: &str, stage: ShaderStage) -> Vec<u32> {
    match stage {
        ShaderStage::VERTEX => {
            return wgpu::read_spirv(
                glsl_to_spirv::compile(&code, glsl_to_spirv::ShaderType::Vertex).unwrap(),
            )
            .unwrap()
        }
        ShaderStage::FRAGMENT => {
            return wgpu::read_spirv(
                glsl_to_spirv::compile(&code, glsl_to_spirv::ShaderType::Fragment).unwrap(),
            )
            .unwrap()
        }
        _ => panic!("error"),
    }
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
pub fn load_glsl(code: &str, stage: ShaderStage) -> Vec<u32> {
    match stage {
        ShaderStage::VERTEX => return return compileGLSL(code, "vertex"),
        ShaderStage::FRAGMENT => return compileGLSL(code, "fragment"),
        _ => panic!("error"),
    }
}
