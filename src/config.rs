#![allow(non_camel_case_types)]

use serde_json::json;
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Config {
    PowerDefault,
    PowerLowPower,
    PowerHighPerformance,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Event {
    Start,
    Update,
    End,
}
// #[repr(C)]
// #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
// pub enum Attrib {
//     POSITION = 0,
//     NORMAL = 1,
//     COLOR = 2,
//     TEXCOORD0 = 3,
// }

// #[repr(C)]
// #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
// pub enum Uniform {
//     ModelViewProjectionMatrix = 0,
//     Sampler = 1,
//     Args = 2,
//     pbrBaseColorTexture = 3,
//     pbrMetallicRoughnessInfo = 4,
//     pbrNormalTexture = 5,
//     pbrNormalTextureInfo = 6,
//     pbrOcclusionTexture = 7,
//     pbrOcclusionTextureInfo = 8,
//     pbrEmissiveTexture = 9,
//     pbrEmissiveTextureInfo = 10,
//     pbrOther = 11,
// }
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
pub static TextureFormat: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
#[allow(non_upper_case_globals)]
#[cfg(not(target_arch = "wasm32"))]
pub static TextureFormat: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
lazy_static! {
    pub static ref UNIFORMNAMES: serde_json::Value = json!([
        {
            "name": "POSITION",
            "vary": false
        },
        {
            "name": "NORMAL",
            "vary": false
        },
        {
            "name": "COLOR",
            "vary": false
        },
        {
            "name": "TEXCOORD0",
            "vary": true
        },
        {
            "name": "TEXCOORD1",
            "vary": true
        }
    ]);
    pub static ref ATTRIBNAMES: serde_json::Value = json!([
        {
            "name": "ModelViewProjectionMatrix",
            "visibility": "vert"
        },
        {
            "name": "Sampler",
            "visibility": "frag"
        },
        {
            "name": "Args",
            "visibility": "frag"
        },
        {
            "name": "pbrBaseColorTexture",
            "visibility": "frag"
        },
        {
            "name": "pbrMetallicRoughnessInfo",
            "visibility": "frag"
        },
        {
            "name": "pbrNormalTexture",
            "visibility": "frag"
        },
        {
            "name": "pbrNormalTextureInfo",
            "visibility": "frag"
        },
        {
            "name": "pbrOcclusionTexture",
            "visibility": "frag"
        },
        {
            "name": "pbrOcclusionTextureInfo",
            "visibility": "frag"
        },
        {
            "name": "pbrEmissiveTexture",
            "visibility": "frag"
        },
        {
            "name": "pbrEmissiveTextureInfo",
            "visibility": "frag"
        },
        {
            "name": "pbrOther",
            "visibility": "frag"
        },
        {
            "name": "MeshPart",
            "visibility": "frag"
        }
    ]);
}

include!("./other.rs");
