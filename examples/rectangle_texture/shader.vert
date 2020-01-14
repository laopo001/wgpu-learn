#version 450
layout (location = 0) in vec4 a_Pos;
layout (location = 1) in vec2 a_TexCoord;
layout (location = 0) out vec2 v_TexCoord;

void main() {
    gl_Position =   a_Pos;
    v_TexCoord = a_TexCoord;
}