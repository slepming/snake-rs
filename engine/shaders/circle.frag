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

void main() {
    vec2 st = st;
    float aspect = pc.u_resolution.x / pc.u_resolution.y;
    st.x *= aspect;

    vec2 center = vec2(0.0);
    float radius = 0.5;
    float thickness = 0.02;

    float d = distance(st, center);

    float mask = smoothstep(radius, radius - thickness, d);

    vec3 circleColor = vec3(0.2, 0.6, 1.0);
    vec3 backgroundColor = vec3(1.0);

    vec3 finalColor = mix(backgroundColor, circleColor, mask);

    outColor = vec4(finalColor, 1.0);
}
