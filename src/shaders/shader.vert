#version 300 es

uniform mat4 u_view;
uniform mat4 u_projection;
uniform int u_canvas_height;
uniform float u_orthographic_size;

in vec4 a_position;

void main() {
    gl_Position = u_projection * u_view * a_position;
    gl_PointSize = (1.0 / u_orthographic_size) * float(u_canvas_height) / 2.0;
}
