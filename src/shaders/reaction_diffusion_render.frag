#version 300 es
precision highp float;

uniform highp usampler2D u_texture;

in vec2 v_uv;

out vec4 outColor;

void main() {
    highp uvec2 utexel = texture(u_texture, v_uv).rg;
    float value = float(utexel.g) / 65535.0;

    vec4 color1 = vec4(0.1, 0.1, 0.1, 0);
    vec4 color2 = vec4(0.5, 0.5, 0.5, 0.2);
    vec4 color3 = vec4(0.5, 0.5, 0.75, 0.21);
    vec4 color4 = vec4(0, 0, 1, 0.4);
    vec4 color5 = vec4(1, 1, 1, 0.6);

    float a;
    vec3 col;

    if (value <= color1.a) {
        col = color1.rgb;
    }
    if (value > color1.a && value <= color2.a)
    {
        a = (value - color1.a) / (color2.a - color1.a);
        col = mix(color1.rgb, color2.rgb, a);
    }
    if (value > color2.a && value <= color3.a)
    {
        a = (value - color2.a) / (color3.a - color2.a);
        col = mix(color2.rgb, color3.rgb, a);
    }
    if (value > color3.a && value <= color4.a)
    {
        a = (value - color3.a) / (color4.a - color3.a);
        col = mix(color3.rgb, color4.rgb, a);
    }
    if (value > color4.a && value <= color5.a)
    {
        a = (value - color4.a) / (color5.a - color4.a);
        col = mix(color4.rgb, color5.rgb, a);
    }
    if (value > color5.a) {
        col = color5.rgb;
    }

    outColor = vec4(col, 1.0);
}
