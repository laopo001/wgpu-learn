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
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Attrib {
    POSITION = 0,
    NORMAL = 1,
    COLOR = 2,
    TEXCOORD0 = 3,
    // TANGENT,
    // BLENDWEIGHT,
    // BLENDINDICES,
    // TEXCOORD1,
    // TEXCOORD2,
    // TEXCOORD3,
    // TEXCOORD4,
    // TEXCOORD5,
    // TEXCOORD6,
    // TEXCOORD7,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Uniform {
    ModelViewProjectionMatrix = 0,
    Texture0 = 1,
    Sampler0 = 2,
    Color0 = 3,
}
#[allow(non_upper_case_globals)]
#[cfg(target_arch = "wasm32")]
pub static TextureFormat: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
#[allow(non_upper_case_globals)]
#[cfg(not(target_arch = "wasm32"))]
pub static TextureFormat: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
lazy_static! {
    pub static ref UNIFORMNAMES: serde_json::Value = json!([{
        "name": "ModelViewProjectionMatrix",
        "type": "mat4",
        "is_base": true,
        "visibility": "vert"
    },{
        "name": "Texture0",
        "type": "texture2D",
        "is_base": false,
        "visibility": "frag"
    },{
        "name": "Sampler0",
        "type": "sampler",
        "is_base": false,
        "visibility": "frag"
    },{
        "name": "Color0",
        "type": "vec3",
        "is_base": true,
        "visibility": "frag"
    }]);
    pub static ref ATTRIBNAMES: serde_json::Value = json!([{
        "name": "POSITION",
        "type": "vec4",
        "vary": false,
    },{
        "name": "NORMAL",
        "type": "vec3",
        "vary": false,
    },{
        "name": "COLOR",
        "type": "vec3",
        "vary": false,
    },{
        "name": "TEXCOORD0",
        "type": "vec2",
        "vary": true,
    }]);
}
