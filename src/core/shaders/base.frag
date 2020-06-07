layout(set = 0, binding = 1) uniform texture2D u_Texture0;
layout(set = 0, binding = 2) uniform sampler u_Sampler0;
layout(set = 0, binding = 3) uniform Locals3 {
    vec3 u_Color0;
};
#if defined (use_TEXCOORD0)
layout (location = 3) in vec2 v_TEXCOORD0; 
#endif
layout(location = 0) out vec4 outColor;
void main() {
    // outColor = vec4(0.5, 0.0, 0.0, 1.0); 
    #if defined (use_Texture0) && defined (use_Sampler0)  && defined (use_TEXCOORD0)
    outColor =  texture(sampler2D(u_Texture0, u_Sampler0), v_TEXCOORD0);
    #else
    outColor = vec4(u_Color0, 1.0);
    #endif
}