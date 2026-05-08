#version 450

struct Transform {
    mat4 transform;
};

layout(location = 0) in vec2 st;
layout(location = 0) out vec4 outColor;

layout(push_constant) uniform Constants {
    Transform transform;
    vec2 u_resolution;
    uint rgba;
} pc;

layout(set = 0, binding = 0) uniform CircleData {
    float radius;
    float thickness;
} data;

void circle()
{
    float d = length(st);

    float mask = smoothstep(data.radius, data.radius - data.thickness, d);

    if (mask <= 0.0) {
        discard;
    }

    float r = ((pc.rgba >> 0) & 0xFF) / 255.0;
    float g = ((pc.rgba >> 8) & 0xFF) / 255.0;
    float b = ((pc.rgba >> 16) & 0xFF) / 255.0;
    float a = ((pc.rgba >> 24) & 0xFF) / 255.0;

    vec4 circleColor = vec4(r, g, b, a);
    circleColor.a *= mask;

    outColor = circleColor;
}

void main() {
    circle();
}
