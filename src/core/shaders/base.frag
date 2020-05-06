// #define Texture 

layout(location = 0) in vec2 v_TEXCOORD0;

void main() {
    #ifdef use_color 
    outColor = u_Color0;
    #else
    outColor =  texture(sampler2D(u_Texture0, u_Sampler0), v_TEXCOORD0);
    #endif
}