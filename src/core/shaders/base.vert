void main() {
    gl_Position = u_ModelViewProjectionMatrix * a_POSITION;
    v_TEXCOORD0 = a_TEXCOORD0;
}