#version 450
#extension GL_GOOGLE_include_directive : require

#include "math.glsl"

struct Transform {
    mat4 transform;
};

layout(location = 0) in vec2 position;

layout(push_constant) uniform Constants {
    Transform transform;
    vec2 u_resolution;
    uint rgba;
} pc;

layout(location = 0) out vec2 tex_coords;

void main()
{
	mat4 matrix = pixelMatrixToNDC(pc.transform.transform, pc.u_resolution);
	gl_Position = matrix * vec4(position, 0.0, 1.0);
    tex_coords = position * 0.5 + 0.5;
}
