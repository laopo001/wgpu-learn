#version 450
layout(location = 0) in vec2 v_TexCoord;
layout(set = 0, binding = 1) uniform texture2D u_Texture;
layout(set = 0, binding = 2) uniform sampler u_Sampler;
layout(location = 0) out vec4 outColor;

void main() {
    outColor =  texture(sampler2D(u_Texture, u_Sampler), v_TexCoord);
    // outColor =  vec4 (1.0, 0.0, 0.0 , 1.0);
}