#version 450

layout(location=0) in vec4 color1;
layout(location=1) in vec4 multiplier;
layout(location=2) noperspective in vec4 color2;

layout(location=0) out vec4 color;

struct S {
    bool b;
    vec4 v[5];
    int i;
};

layout(binding=0) uniform blockName {
    S s;
    bool cond;
};

void main()
{
    vec4 scale = vec4(1.0, 1.0, 2.0, 1.0);

    if (cond)
        color = color1 + s.v[2];
    else
        color = sqrt(color2) * scale;

    for (int i = 0; i < 4; ++i)
        color *= multiplier;
}
