#version 450
#define use_NORMAL 1;
#define use_COLOR 1;
#define use_TEXCOORD0 1;
//split
#extension GL_ARB_separate_shader_objects : enable
#pragma vscode_glsllint_stage : frag //pragma to set STAGE to 'frag'

layout(set = 0, binding = 1) uniform sampler u_Sampler;

layout(set = 0, binding = 3) uniform texture2D u_pbrBaseColorTexture;

layout(set = 0, binding = 8) uniform pbrInfo {
    vec3 u_pbrBaseColorFactor;
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
layout(location = 4) out vec4 outColor;

void main() {
    // outColor = vec4(0.5, 0.0, 0.0, 1.0); 
    #if defined(use_pbrBaseColorTexture) && defined(use_Sampler)  && defined(use_TEXCOORD0)
    outColor =  texture(sampler2D(u_pbrBaseColorTexture, u_Sampler), v_TEXCOORD0);
    #else
    outColor = vec4(u_pbrBaseColorFactor, 1.0);
    #endif
}