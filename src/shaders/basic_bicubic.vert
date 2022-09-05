#version 300 es

//uniform mat4 u_model;
//uniform mat4 u_view;
//uniform mat4 u_projection;

in vec4 a_position;
in vec2 a_uv;

out vec2 v_uv;

void main() {
    v_uv = a_uv;
//    gl_Position = u_projection * u_view * u_model * a_position;
    gl_Position = a_position;
}
