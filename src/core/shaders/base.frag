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

layout(std140, set = 0, binding = 0) uniform Args {
    layout(offset = 0) mat4 proj_view;
    layout(offset = 64) vec3 camera_pos;
    layout(offset = 76) int point_light_count;
    layout(offset = 80) PointLight point_lights[32];
    layout(offset = 1104) int spot_light_count;
    layout(offset = 1120) SpotLight spot_lights[32];
};

layout(set = 2, binding = 0) uniform MeshPart {
    layout(offset = 0) vec4 in_diffuse;
    layout(offset = 16) float metal_factor;
    layout(offset = 32) float rough_factor;
    layout(offset = 48) vec3 emissive_factor;
    layout(offset = 64) vec3 extra_emissive;
};
layout(set = 0, binding = 1) uniform texture2D u_DiffuseTexture;
layout(set = 0, binding = 2) uniform sampler u_DiffuseSampler;
layout(set = 0, binding = 3) uniform Locals0 {
    vec3 u_DiffuseColor;
};
layout(set = 0, binding = 4) uniform Locals1 {
    vec3 u_CameraPosition;
};


layout(location = 0) out vec4 outColor;
#if defined (use_NORMAL)
layout (location = 1) in vec3 a_NORMAL;
#endif
#if defined (use_COLOR)
layout (location = 2) in vec4 a_COLOR;
#endif
#if defined (use_TEXCOORD0)
layout (location = 3) in vec2 v_TEXCOORD0; 
#endif

void main() {
    // outColor = vec4(0.5, 0.0, 0.0, 1.0); 
    #if defined (use_DiffuseTexture) && defined (use_DiffuseSampler)  && defined (use_TEXCOORD0)
    outColor =  texture(sampler2D(u_DiffuseTexture, u_DiffuseSampler), v_TEXCOORD0);
    #else
    outColor = vec4(u_DiffuseColor, 1.0);
    #endif
}