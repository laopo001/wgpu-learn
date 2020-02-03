#version 450
layout (location = 0) out vec3 fragColor;
const vec2 pos[3] = vec2[3](vec2(0.0f, 0.5f), vec2(-0.5f, -0.5f), vec2(0.5f, -0.5f));
vec3 colors [3] = vec3 [] (vec3 (1.0, 0.0, 0.0), vec3 (0.0, 1.0, 0.0), vec3 (0.0, 0.0, 1.0));
void main() {
    gl_Position = vec4(pos[gl_VertexIndex], 0.0, 1.0);
    fragColor = colors[gl_VertexIndex];
}