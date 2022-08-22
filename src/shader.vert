#version 300 es

in vec4 position;

void main() {
    gl_Position = vec4(position.xyz / 10.0, 1.0);
    gl_PointSize = 10.0;
}
