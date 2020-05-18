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
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn error(s: &str);
}

#[cfg(target_arch = "wasm32")]
pub fn load_glsl(code: &str, stage: ShaderStage) -> Vec<u32> {
    match stage {
        ShaderStage::VERTEX => return return compileGLSL(code, "vertex"),
        ShaderStage::FRAGMENT => return compileGLSL(code, "fragment"),
        _ => panic!("error"),
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn console_log<T: std::fmt::Debug>(s: T) {
    println!("{:?}", s);
}
#[cfg(not(target_arch = "wasm32"))]
pub fn console_error<T: std::fmt::Debug>(s: T) {
    println!("{:?}", s);
    panic!("console error");
}

#[cfg(target_arch = "wasm32")]
pub fn console_log<T: std::fmt::Debug>(s: T) {
    let arr = js_sys::Array::new();
    let v = wasm_bindgen::JsValue::from_str(&format!("{:?}", s));
    arr.push(&v);
    web_sys::console::log(&arr);
    // log(&format!("{:?}", s));
}
#[cfg(target_arch = "wasm32")]
pub fn console_error<T: std::fmt::Debug>(s: T) {
    let arr = js_sys::Array::new();
    let v = wasm_bindgen::JsValue::from_str(&format!("{:?}", s));
    arr.push(&v);
    web_sys::console::error(&arr);
    // error(&format!("{:?}", s));
}
