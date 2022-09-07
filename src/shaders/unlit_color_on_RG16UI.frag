#version 300 es
precision highp float;
precision highp int;

in vec2 v_uv;

out uvec2 outColor;

void main() {
    vec2 uv = v_uv - vec2(0.5, 0.5);
    if(uv.x * uv.x + uv.y * uv.y < 0.5 * 0.5) {
        outColor = uvec2(0, 65535);
    } else {
        discard;
    }
}
