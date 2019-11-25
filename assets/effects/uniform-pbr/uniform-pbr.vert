#version 450 core

// # Input
//
// Vertex position in object space.
layout(location=0) in vec4 pos;
// Normal direction vector.
layout(location=1) in vec4 normal;
// Texture UV coordinates.
layout(location=2) in vec2 uv;
//

// # Output
//
// Raw fragment representation.
struct Repr {
    vec4 pos;
    vec4 normal;
    vec2 uv;
};
layout(location=0) out Repr repr;
//

// # Uniforms
//
// Transform matrices pushed directly via command buffer.
layout(std140, push_constant)
uniform Transform {
    mat4x4 model_view;
    mat4x4 view_proj;
};
//

void main() {
    repr.pos = model_view * pos;
    repr.normal = model_view * normal;
    repr.uv = uv;
    gl_Position = view_proj * pos;
}
