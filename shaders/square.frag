#version 450

struct Transform {
    mat4 transform;
};

layout(location = 0) in vec2 uv;
layout(location = 0) out vec4 f_color;

layout(push_constant) uniform Constants {
    Transform transform;
    vec2 u_resolution;
    uint rgba;
} pc;

layout(set = 0, binding = 0) uniform SquareData {
    float corner_radius;
} data;

float sdRoundedRect(vec2 p, vec2 size, float radius) {
    vec2 d = abs(p) - size + radius;
    return length(max(d, 0.0)) + min(max(d.x, d.y), 0.0) - radius;
}

void main() {
    float r = ((pc.rgba >> 0) & 0xFF) / 255.0;
    float g = ((pc.rgba >> 8) & 0xFF) / 255.0;
    float b = ((pc.rgba >> 16) & 0xFF) / 255.0;
    float a = ((pc.rgba >> 24) & 0xFF) / 255.0;

    if (data.corner_radius > 0.0) {
        float d = sdRoundedRect(uv, vec2(1.0), data.corner_radius);
        float mask = 1.0 - smoothstep(0.0, fwidth(d) * 1.5, d);
        if (mask <= 0.0) {
            discard;
        }
        f_color = vec4(r, g, b, a * mask);
    } else {
        f_color = vec4(r, g, b, a);
    }
}
