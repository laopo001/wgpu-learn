// #define use_color 

layout(location = 0) out vec4 outColor;
void main() {
    // outColor = vec4(0.5, 0.0, 0.0, 1.0);
    #ifdef use_color 
    outColor = vec4(u_Color0, 1.0);
    #else
    outColor =  texture(sampler2D(u_Texture0, u_Sampler0), v_TEXCOORD0);
    #endif
}