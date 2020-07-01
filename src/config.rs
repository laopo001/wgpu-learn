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
    DiffuseTexture = 1,
    DiffuseSampler = 2,
    DiffuseColor = 3,
    CameraPosition = 4,
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
        "visibility": "vert"
    },{
        "name": "DiffuseTexture",
        "visibility": "frag"
    },{
        "name": "DiffuseSampler",
        "visibility": "frag"
    },{
        "name": "DiffuseColor",
        "visibility": "frag"
    },{
        "name": "CameraPosition",
        "visibility": "frag"
    }]);
    pub static ref ATTRIBNAMES: serde_json::Value = json!([{
        "name": "POSITION",
        // "type": "vec4",
        "vary": false,
    },{
        "name": "NORMAL",
        // "type": "vec3",
        "vary": false,
    },{
        "name": "COLOR",
        // "type": "vec3",
        "vary": false,
    },{
        "name": "TEXCOORD0",
        // "type": "vec2",
        "vary": true,
    }]);
}
