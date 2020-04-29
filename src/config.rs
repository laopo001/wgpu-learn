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
}

lazy_static! {
    pub static ref UNIFORMNAMES: Vec<(String, String)> = {
        let res = vec![
            ("ModelViewProjectionMatrix", "mat4"),
            ("Texture0", "texture2D"),
            ("Sampler0", "sampler"),
        ];
        res.iter()
            .map(|x| (x.0.to_string(), x.1.to_string()))
            .collect()
    };
    pub static ref ATTRIBNAMES: Vec<(String, String)> = {
        let res = vec![
            ("POSITION", "vec4"),
            ("NORMAL", "vec3"),
            ("COLOR", "vec3"),
            ("TEXCOORD0", "vec2"),
        ];
        res.iter()
            .map(|x| (x.0.to_string(), x.1.to_string()))
            .collect()
    };
}
