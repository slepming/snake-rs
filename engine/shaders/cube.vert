#version 450

layout(location = 0) in vec2 position;
layout(push_constant) uniform Transform {
    mat4 transform;
} pc;

void main() {
    gl_Position = vec4(position, 0.0, 1.0) * pc.transform;
}

