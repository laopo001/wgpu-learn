#version 450
#extension GL_ARB_separate_shader_objects : enable

layout (location = 0) out vec3 fragColor;

const vec2 pos[3] = vec2[](vec2(-0.5f, -0.5f), vec2(0.0f, 0.5f), vec2(0.5f, -0.5f));
const vec3 colors[3] = vec3[](vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0));

void main() {
     
    vec2 position = vec2(gl_VertexIndex, (gl_VertexIndex & 1) * 2) - 1;
    gl_Position = vec4(position / 2, 0.0, 1.0);
    // gl_Position = vec4(pos[gl_VertexIndex], 0.0, 1.0);  
    if (gl_VertexIndex == 0){
        fragColor = colors[gl_VertexIndex];
    }
    if (gl_VertexIndex== 1){
        fragColor = colors[gl_VertexIndex];
    }
    if (gl_VertexIndex == 2){
        fragColor = colors[gl_VertexIndex];
    }
    // fragColor = colors[gl_VertexIndex];
    // fragColor = vec3(position, 1.0); 
}