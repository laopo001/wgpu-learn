#version 450
layout (location = 0) in vec4 a_Pos;
layout (location = 1) in vec2 a_TexCoord;
layout (location = 0) out vec2 v_TexCoord;
layout(set = 0, binding = 0) uniform Locals {
    mat4 u_ModelViewProjectionMatrix;
};

void main() {
    gl_Position = u_ModelViewProjectionMatrix * a_Pos;
    v_TexCoord = a_TexCoord;
}