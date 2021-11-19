#version 450

// Specialization constants walking demo.
layout(constant_id=233) const int w = 1;
layout(constant_id=234) const int h = 1;
//

layout(location=0) in vec4 color1;
layout(location=1) in vec4 multiplier;
layout(location=2) noperspective in vec4 color2;

layout(location=0) out vec4 color;

struct S {
    bool b;
    vec4 v[5];
    int i;
};

struct Object {
    vec3 position;
    uint type;
    vec4 rotation;
    vec3 half_size;
    uint dummy;
};

struct Scene {
    uint object_count;
    Object object; // this works fine
//    Object objects[10]; // this causes an error: "cannot find a suitable type"
};


layout(binding=0) uniform blockName {
    S s;
    bool cond;
};

layout(binding=1) buffer atomics {
    uint x;
};

void main()
{
    vec4 scale = vec4(1.0, 1.0, 2.0, 1.0);

    Scene scene;
    scene.object_count = 5;

    atomicAdd(x, 5);

    vec4 pixel = vec4(0, 0, 0, 1);
    pixel.r = scene.object_count * 0.1;

    if (cond)
        color = color1 + s.v[2];
    else
        color = sqrt(color2) * scale;

    for (int i = 0; i < 4; ++i)
        color *= multiplier + pixel;
}
