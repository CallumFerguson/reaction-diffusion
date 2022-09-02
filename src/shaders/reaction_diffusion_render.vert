#version 300 es

in vec4 a_position;
in vec2 a_uv;

out vec2 v_uv;

void main() {
    v_uv = a_uv;
    gl_Position = a_position;
}
