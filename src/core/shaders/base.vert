layout(set = 0, binding = 0) uniform Locals0 {
    mat4 u_ModelViewProjectionMatrix;
};
layout (location = 0) in vec4 a_POSITION;
layout (location = 3) in vec2 a_TEXCOORD0;
layout (location = 3) out vec2 v_TEXCOORD0;

void main() {
    gl_Position = u_ModelViewProjectionMatrix * a_POSITION;
    v_TEXCOORD0 = a_TEXCOORD0;
}