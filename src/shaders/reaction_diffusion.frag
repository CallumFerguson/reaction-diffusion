#version 300 es
precision highp float;

uniform highp usampler2D u_texture;

in vec2 v_uv;

out highp uvec2 outColor;

void main() {
    highp uvec2 utexel = texture(u_texture, v_uv).rg;
    float A = float(utexel.r) / 65535.0;
    float B = float(utexel.g) / 65535.0;

    A += 0.01;
    B += 0.01;

    outColor = uvec2(int(round(clamp(A, 0.0, 1.0) * 65535.0)), int(round(clamp(B, 0.0, 1.0) * 65535.0)));
}
