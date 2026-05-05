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
    uint padding;
} pc;

void circle()
{
    vec2 corr_st = st;
    float aspect = pc.u_resolution.x / pc.u_resolution.y;
    corr_st.x *= aspect;

    vec2 center = vec2(0.0);
    float radius = 0.05;
    float thickness = 0.005;

    float d = distance(corr_st, center);

    float mask = smoothstep(radius, radius - thickness, d);

    vec4 circleColor = vec4(1.0, 0.5, 0.0, 1.0);
    vec4 backgroundColor = aspect < 5 ? vec4(1.0, 0.1, 0.1, 1.0) : vec4(0.1, 0.1, 0.1, 1.0);

    vec4 finalColor = mix(backgroundColor, circleColor, mask);

    outColor = finalColor;
}

void main() {
    circle();
}
