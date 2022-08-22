#version 300 es
precision highp float;

uniform vec3 u_color;

out vec4 outColor;

void main() {
    outColor = vec4(u_color.rgb, 1);
}
