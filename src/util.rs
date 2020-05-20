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
    // crate::console_log!(compileGLSL(code, "vertex"));
    match stage {
        ShaderStage::VERTEX => return compileGLSL(code, "vertex"),
        ShaderStage::FRAGMENT => return compileGLSL(code, "fragment"),
        _ => panic!("error"),
    }
}

#[macro_export]
macro_rules! console_log {
    ($val:expr) => {
        cfg_if::cfg_if!{
                // log::info!("[{}:{}] {:?}", std::file!(), std::line!(), $val);
                if #[cfg(not(target_arch = "wasm32"))] {
                    if std::env::var("RUST_LOG").is_err() {
                        dbg!($val);
                    } else {
                        log::info!("[{}:{}] {:?}", std::file!(), std::line!(), $val);
                    }
                } else {
                    let arr = js_sys::Array::new();
                    let v = wasm_bindgen::JsValue::from_str(&format!("[{}:{}] {:?}", std::file!(), std::line!(), $val));
                    arr.push(&v);
                    web_sys::console::log(&arr);
                }
            }

    };
    ( $( $x:expr ),* ) => {
        {
            $(
                crate::console_log!($x);
            )*
        }
    };
}

#[macro_export]
macro_rules! console_error {
    ($val:expr) => {
        cfg_if::cfg_if!{
            if #[cfg(not(target_arch = "wasm32"))] {
                log::error!("[{}:{}] {:?}", std::file!(), std::line!(), $val);
                // panic!("console error");
            } else {
                let arr = js_sys::Array::new();
                let v = wasm_bindgen::JsValue::from_str(&format!("[{}:{}] {:?}", std::file!(), std::line!(), $val));
                arr.push(&v);
                web_sys::console::error(&arr);
            }
        }
    };
    ( $( $x:expr ),* ) => {
        {
            $(
                crate::console_error!($x);
            )*
        }
    };
}

// #[cfg(not(target_arch = "wasm32"))]
// pub fn console_log<T: std::fmt::Debug>(s: T) {
//     log::info!("{:?}", s);
// }
// #[cfg(not(target_arch = "wasm32"))]
// pub fn console_error<T: std::fmt::Debug>(s: T) {
//     log::error!("{:?}", s);
//     panic!("console error");
// }

// #[cfg(target_arch = "wasm32")]
// pub fn console_log<T: std::fmt::Debug>(s: T) {
//     let arr = js_sys::Array::new();
//     let v = wasm_bindgen::JsValue::from_str(&format!("{:?}", s));
//     arr.push(&v);
//     web_sys::console::log(&arr);
//     // log(&format!("{:?}", s));
// }
// #[cfg(target_arch = "wasm32")]
// pub fn console_error<T: std::fmt::Debug>(s: T) {
//     let arr = js_sys::Array::new();
//     let v = wasm_bindgen::JsValue::from_str(&format!("{:?}", s));
//     arr.push(&v);
//     web_sys::console::error(&arr);
//     // error(&format!("{:?}", s));
// }
