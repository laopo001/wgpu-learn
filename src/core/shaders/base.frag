layout(set = 0, binding = 1) uniform texture2D u_DiffuseTexture;
layout(set = 0, binding = 2) uniform sampler u_DiffuseSampler;
layout(set = 0, binding = 3) uniform Locals3 {
    vec3 u_DiffuseColor;
};
#if defined (use_TEXCOORD0)
layout (location = 3) in vec2 v_TEXCOORD0; 
#endif
layout(location = 0) out vec4 outColor;
void main() {
    // outColor = vec4(0.5, 0.0, 0.0, 1.0); 
    #if defined (use_DiffuseTexture) && defined (use_DiffuseSampler)  && defined (use_TEXCOORD0)
    outColor =  texture(sampler2D(u_DiffuseTexture, u_DiffuseSampler), v_TEXCOORD0);
    #else
    outColor = vec4(u_DiffuseColor, 1.0);
    #endif
}