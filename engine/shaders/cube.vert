#version 450

struct Transform {
	mat4 transform;
};

layout(location = 0) in vec2 position;
layout(push_constant) uniform Constants {
	Transform transform;
	uint rgba;
} pc;

void main() {
    gl_Position = vec4(position, 0.0, 1.0) * pc.transform.transform;
}

