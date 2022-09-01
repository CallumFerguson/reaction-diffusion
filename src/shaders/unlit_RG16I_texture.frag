#version 300 es
precision highp float;

uniform highp usampler2D u_texture;

in vec2 v_uv;

out vec4 outColor;

void main() {
    highp uvec2 utexel = texture(u_texture, v_uv).rg;
    float A = float(utexel.r) / 65535.0;
    float B = float(utexel.g) / 65535.0;
    outColor = vec4(A, B, 0, 1);
}
