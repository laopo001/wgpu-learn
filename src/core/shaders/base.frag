#version 450
#define use_NORMAL 1;
#define use_COLOR 1;
#define use_TEXCOORD0 1;
//split
#extension GL_ARB_separate_shader_objects : enable
#pragma vscode_glsllint_stage : frag //pragma to set STAGE to 'frag'

struct PointLight {
    vec3 position;
    float intensity; // 光的强度
    vec3 color;
};

struct SpotLight {
    vec3 position;
    float angle;
    vec3 color;
    float range;
    vec3 direction;
    float smoothness; // 平稳 平滑
    float intensity; // 光的强度
};
layout(set = 0, binding = 1) uniform sampler u_Sampler;
// pbrMetallicRoughness
layout(std140, set = 0, binding = 2) uniform Args {
    layout(offset = 0) mat4 proj_view;
    layout(offset = 64) vec3 camera_pos;
    layout(offset = 76) int point_light_count;
    layout(offset = 80) PointLight point_lights[32];
    layout(offset = 1104) int spot_light_count;
    layout(offset = 1120) SpotLight spot_lights[32];
};
layout(set = 0, binding = 3) uniform texture2D u_pbrBaseColorTexture;
layout(set = 0, binding = 4) uniform pbrMetallicRoughnessInfo {
    vec4 u_pbrBaseColorFactor;
    float u_pbrMetallicFactor;
    float u_pbrRoughnessFactor;
    uint u_pbrBaseColorTextureTexCoord;
};
// normalTexture
layout(set = 0, binding = 5) uniform texture2D u_pbrNormalTexture;
layout(set = 0, binding = 6) uniform pbrNormalTextureInfo {
    uint u_pbrNormalTextureTexCoord;
    float u_pbrNormalTextureScale;
};
// occlusionTexture
layout(set = 0, binding = 7) uniform texture2D u_pbrOcclusionTexture;
layout(set = 0, binding = 8) uniform pbrOcclusionTextureInfo {
    uint u_pbrOcclusionTextureTexCoord;
    float u_pbrOcclusionTextureStrength;
};
// emissiveTexture
layout(set = 0, binding = 9) uniform texture2D u_pbrEmissiveTexture;
layout(set = 0, binding = 10) uniform pbrEmissiveTextureInfo {
    uint u_pbrEmissiveTextureTexCoord;
};
// pbrOther
layout(set = 0, binding = 11) uniform pbrOther {
    vec3 u_pbrEmissiveFactor;
    uint u_pbrAlphaMode;
    float u_pbrAlphaCutoff;
    bool u_pbrDoubleSided;
};
layout(set = 2, binding = 12) uniform MeshPart {
    layout(offset = 0) vec4 in_diffuse;
    layout(offset = 16) float metal_factor;
    layout(offset = 32) float rough_factor;
    layout(offset = 48) vec3 emissive_factor;
    layout(offset = 64) vec3 extra_emissive;
};

layout (location = 0) in vec3 v_POSITION;
#if defined (use_NORMAL)
layout (location = 1) in vec3 a_NORMAL;
#endif
#if defined (use_COLOR)
layout (location = 2) in vec4 a_COLOR;
#endif
#if defined(use_TEXCOORD0)
layout (location = 3) in vec2 v_TEXCOORD0; 
#endif
#if defined(use_TEXCOORD1)
layout (location = 4) in vec2 v_TEXCOORD1; 
#endif
layout(location = 5) out vec4 outColor;

vec2 getCurrTEXCOORD(uint index) {
    #if defined(use_TEXCOORD0)
        if(index==0) {
            return v_TEXCOORD0;
        }
    #endif
    #if defined(use_TEXCOORD1)
        if(index==1) {
            return v_TEXCOORD1;
        }
    #endif
}

vec4 getBaseColor() {
    vec4 baseColor = vec4(1.0, 1.0, 1.0, 1.0);
    #if defined(use_COLOR)
        baseColor = a_COLOR;
    #endif
    #if defined(use_pbrMetallicRoughnessInfo)
        baseColor = u_pbrBaseColorFactor;
    #endif
    #if defined(use_pbrBaseColorTexture) && defined(use_Sampler)  && defined(use_TEXCOORD0)
        outColor =  texture(sampler2D(u_pbrBaseColorTexture, u_Sampler), v_TEXCOORD0);
    #endif
    return baseColor;
}


void main() {
    vec4 baseColor = getBaseColor(); 
    vec3 camera_v = camera_pos - v_POSITION;
    // vec3 light_v = camera_pos - v_POSITION;
    if (baseColor.a == 0.0) discard;
    #if defined(has_pbrNormalTexture)
        vec3 normal = texture(sampler2D(u_pbrNormalTexture, u_Sampler), getCurrTEXCOORD(u_pbrNormalTextureTexCoord)).rgb;
        normal = normalize((normal * 2 - 1) * vec3(u_pbrNormalTextureScale, u_pbrNormalTextureScale, 1.0)); // Convert [0, 1] to [-1, 1] and scale
    #else
        vec3 normal = a_NORMAL;
    #endif
    outColor = baseColor;
}