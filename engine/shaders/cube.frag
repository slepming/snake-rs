#version 450


struct Transform {
	mat4 transform;
};

layout(location = 0) out vec4 f_color;
layout(push_constant) uniform Constants {
	Transform transform;
	uint rgba;
} pc;

void main() {
	float r = ((pc.rgba >> 0) & 0xFF) / 255.0;
	float g = ((pc.rgba >> 8) & 0xFF) / 255.0;
	float b = ((pc.rgba >> 16) & 0xFF) / 255.0;
	float a = ((pc.rgba >> 24) & 0xFF) / 255.0;
    f_color = vec4(r, g, b, a);
}
