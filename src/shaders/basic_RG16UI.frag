#version 300 es
precision highp float;
precision highp int;

uniform highp usampler2D u_texture;

in vec2 v_uv;

out uvec2 outColor;

void main() {
    outColor = texture(u_texture, v_uv).rg;
}
