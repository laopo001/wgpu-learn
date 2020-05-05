layout (location = 0) out vec2 v_TEXCOORD0;

void main() {
    gl_Position = a_ModelViewProjectionMatrix * a_POSITION;
    v_TEXCOORD0 = a_TEXCOORD0;
}