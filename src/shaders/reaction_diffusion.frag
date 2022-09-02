#version 300 es
precision highp float;
precision highp int;

const float D_A = 1.0;
const float D_B = 0.5;
const float F = 0.055;
const float K = 0.062;
const float DELTA_T = 1.0;

uniform highp usampler2D u_texture;
uniform float u_kernel[9];
uniform int u_texture_width;
uniform int u_texture_height;

in vec2 v_uv;

out uvec2 outColor;

void main() {
    vec2 onePixel = vec2(1) / vec2(textureSize(u_texture, 0));

    uvec2 utexel = texture(u_texture, v_uv).rg;
    float a = float(utexel.r) / 65535.0;
    float b = float(utexel.g) / 65535.0;

    float nabla_squared_a = 0.0;
    float nabla_squared_b = 0.0;

    uvec2 utexel0 = texture(u_texture, v_uv + onePixel * vec2(-1, 1)).rg;
    float a0 = float(utexel0.r) / 65535.0;
    float b0 = float(utexel0.g) / 65535.0;
    nabla_squared_a += a0 * u_kernel[0];
    nabla_squared_b += b0 * u_kernel[0];

    uvec2 utexel1 = texture(u_texture, v_uv + onePixel * vec2(0, 1)).rg;
    float a1 = float(utexel1.r) / 65535.0;
    float b1 = float(utexel1.g) / 65535.0;
    nabla_squared_a += a1 * u_kernel[1];
    nabla_squared_b += b1 * u_kernel[1];

    uvec2 utexel2 = texture(u_texture, v_uv + onePixel * vec2(1, 1)).rg;
    float a2 = float(utexel2.r) / 65535.0;
    float b2 = float(utexel2.g) / 65535.0;
    nabla_squared_a += a2 * u_kernel[2];
    nabla_squared_b += b2 * u_kernel[2];

    uvec2 utexel3 = texture(u_texture, v_uv + onePixel * vec2(-1, 0)).rg;
    float a3 = float(utexel3.r) / 65535.0;
    float b3 = float(utexel3.g) / 65535.0;
    nabla_squared_a += a3 * u_kernel[3];
    nabla_squared_b += b3 * u_kernel[3];

    uvec2 utexel4 = texture(u_texture, v_uv + onePixel * vec2(0, 0)).rg;
    float a4 = float(utexel4.r) / 65535.0;
    float b4 = float(utexel4.g) / 65535.0;
    nabla_squared_a += a4 * u_kernel[4];
    nabla_squared_b += b4 * u_kernel[4];

    uvec2 utexel5 = texture(u_texture, v_uv + onePixel * vec2(1, 0)).rg;
    float a5 = float(utexel5.r) / 65535.0;
    float b5 = float(utexel5.g) / 65535.0;
    nabla_squared_a += a5 * u_kernel[5];
    nabla_squared_b += b5 * u_kernel[5];

    uvec2 utexel6 = texture(u_texture, v_uv + onePixel * vec2(-1, -1)).rg;
    float a6 = float(utexel6.r) / 65535.0;
    float b6 = float(utexel6.g) / 65535.0;
    nabla_squared_a += a6 * u_kernel[6];
    nabla_squared_b += b6 * u_kernel[6];

    uvec2 utexel7 = texture(u_texture, v_uv + onePixel * vec2(0, -1)).rg;
    float a7 = float(utexel7.r) / 65535.0;
    float b7 = float(utexel7.g) / 65535.0;
    nabla_squared_a += a7 * u_kernel[7];
    nabla_squared_b += b7 * u_kernel[7];

    uvec2 utexel8 = texture(u_texture, v_uv + onePixel * vec2(1, -1)).rg;
    float a8 = float(utexel8.r) / 65535.0;
    float b8 = float(utexel8.g) / 65535.0;
    nabla_squared_a += a8 * u_kernel[8];
    nabla_squared_b += b8 * u_kernel[8];

    // math from https://karlsims.com/rd.html
    float a_prime = a + (D_A * nabla_squared_a - a * b * b + F * (1.0 - a)) * DELTA_T;
    float b_prime = b + (D_B * nabla_squared_b + a * b * b - (K + F) * b) * DELTA_T;

    int a_prime_int = int(round(clamp(a_prime, 0.0, 1.0) * 65535.0));
    int b_prime_int = int(round(clamp(b_prime, 0.0, 1.0) * 65535.0));

    outColor = uvec2(a_prime_int, b_prime_int);
}
