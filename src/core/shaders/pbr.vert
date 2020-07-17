#version 450
#define use_NORMAL 1;
#define use_COLOR 1;
#define use_TEXCOORD0 1;
//split
#pragma vscode_glsllint_stage : vert 
layout(set = 0, binding = 0) uniform Locals0 {
    mat4 u_ViewProjectionMatrix;
    mat4 u_ModelMatrix;
    mat4 u_NormalMatrix;
};
layout (location = 0) in vec4 a_POSITION;
layout (location = 0) out vec3 v_POSITION;
#if defined (use_NORMAL)
layout (location = 1) in vec3 a_NORMAL;
layout (location = 1) out vec3 v_NORMAL;
#endif

#if defined (use_COLOR)
layout (location = 2) in vec4 a_COLOR;
layout (location = 2) out vec4 v_COLOR;
#endif

#if defined (use_TEXCOORD0)
layout (location = 3) in vec2 a_TEXCOORD0;
layout (location = 3) out vec2 v_TEXCOORD0;
#endif
#if defined (use_TEXCOORD1)
layout (location = 3) in vec2 a_TEXCOORD1;
layout (location = 3) out vec2 v_TEXCOORD1;
#endif
void main() {
    vec4 posW = u_ModelMatrix * a_POSITION;
    v_POSITION = vec3(posW);
    gl_Position = u_ViewProjectionMatrix * posW;
    #if defined (use_NORMAL)
    v_NORMAL = normalize(vec3(u_NormalMatrix * vec4(a_NORMAL,1)));
    #endif

    #if defined (use_COLOR)
    v_COLOR = a_COLOR;
    #endif

    #if defined (use_TEXCOORD0)
    v_TEXCOORD0 = a_TEXCOORD0;
    #endif

    #if defined (use_TEXCOORD1)
    v_TEXCOORD1 = a_TEXCOORD1;
    #endif
}