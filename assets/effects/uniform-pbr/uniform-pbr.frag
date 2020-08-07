#version 450 core

// # Input
//
// Interpolated fragment representation.
struct Repr {
    vec4 pos;
    vec4 normal;
    vec2 uv;
};
layout(location=0) in Repr repr;
//

// # Output
//
// Color value.
layout(location=0) out vec4 color;
//

// # Uniforms
//
// Light source information. Note that pos is a directional light when the w-
// component is 0 and a point light when the w-component is 1.
struct Light {
    vec3 pos;
    vec3 color;
};
layout(std430, set=1, binding=0) readonly
buffer Lighting {
    vec4 cam_pos;
    Light[] lights;
} l[2];
//
// Material information.
layout(std140, binding=1)
uniform Material {
    vec3 albedo;
    float metalicity;
    float roughness;
    mat3 fdsa[2];
} mat[2];
//

layout(input_attachment_index=0, binding=2)
uniform subpassInput someImage;

layout(binding=3)uniform sampler2D imgggg;

layout(binding=50)
uniform sampler hahano;
layout(binding=51)
uniform texture2D hahano1;

layout(binding=40)
uniform sampler2D hahayes[69];


layout(push_constant)
uniform YetAnotherPushConstantBlock {
    vec4 randomColor;
};


const float PI = 3.1415926;
const float TAU = PI * 2;

void main() {
    vec4 randcolor = randomColor;
    float x = mat[0].metalicity;
    vec4 ccc = l[0].cam_pos;
    color = vec4(0.0, 0.0, 1.0, 1.0) + subpassLoad(someImage).rgba + texture(imgggg, vec2(0.0, 0.0))+ texture(hahayes[5], vec2(0.0, 0.0))+ texture(sampler2D(hahano1, hahano), vec2(0.0, 0.0));
}
