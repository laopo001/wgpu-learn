#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Attrib {
    POSITION = 0,
    NORMAL = 1,
    COLOR = 2,
    TEXCOORD0 = 3,
    TEXCOORD1 = 4,
}
#[repr(C)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
pub enum Uniform {
    ModelViewProjectionMatrix = 0,
    Sampler = 1,
    Args = 2,
    pbrBaseColorTexture = 3,
    pbrMetallicRoughnessTexture = 4,
    pbrNormalTexture = 5,
    pbrOcclusionTexture = 6,
    pbrEmissiveTexture = 7,
    pbrInfo = 8,
    pbrAlbedoTexture = 9,
}
