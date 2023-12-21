#version 460 core
#extension GL_EXT_ray_tracing : enable
#extension GL_EXT_ray_query : enable
#extension GL_EXT_shader_explicit_arithmetic_types_int8 : require
#extension GL_EXT_shader_explicit_arithmetic_types_int16 : require
#extension GL_EXT_shader_explicit_arithmetic_types_int32 : require
#extension GL_EXT_shader_explicit_arithmetic_types_int64 : require
#extension GL_EXT_shader_explicit_arithmetic_types_float16 : require
#extension GL_EXT_shader_explicit_arithmetic_types_float32 : require
#extension GL_EXT_shader_explicit_arithmetic_types_float64 : require

struct Data {
    // Signed integer scalar and vector types.
    int   i0;
    ivec2 i1;
    ivec3 i2;
    ivec4 i3;

    // Unsigned integer scalar and vector types.
    uint  u0;
    uvec2 u1;
    uvec3 u2;
    uvec4 u3;

    // Single-precision floating-point scalar and vector types.
    float f0;
    vec2  f1;
    vec3  f2;
    vec4  f3;

    // Single-precision floating-point matrix types.
    mat2x2 fMat0;
    mat2x3 fMat1;
    mat2x4 fMat2;
    mat3x2 fMat3;
    mat3x3 fMat4;
    mat3x4 fMat5;
    mat4x2 fMat6;
    mat4x3 fMat7;
    mat4x4 fMat8;

    // Double-precision floating-point scalar and vector types.
    double d0;
    dvec2  d1;
    dvec3  d2;
    dvec4  d3;

    // Single-precision floating-point matrix types.
    dmat2x2 dMat0;
    dmat2x3 dMat1;
    dmat2x4 dMat2;
    dmat3x2 dMat3;
    dmat3x3 dMat4;
    dmat3x4 dMat5;
    dmat4x2 dMat6;
    dmat4x3 dMat7;
    dmat4x4 dMat8;
};

// Signed integer storage images.
layout(set=1, binding=0, rgba32i) readonly uniform iimage1D        iImg1d;
layout(set=1, binding=1, rgba16i) readonly uniform iimage2D        iImg2d;
layout(set=1, binding=2,  rgba8i) readonly uniform iimage3D        iImg3d;
layout(set=1, binding=3, rgba32i) readonly uniform iimage1DArray   iImg1dArr;
layout(set=1, binding=4, rgba32i) readonly uniform iimage2DArray   iImg2dArr;
layout(set=1, binding=5, rgba32i) readonly uniform iimageCube      iImgCube;
layout(set=1, binding=6, rgba32i) readonly uniform iimageCubeArray iImgCubeArr;
layout(set=1, binding=7, rgba32i) readonly uniform iimageBuffer    iImgBuf;

// Unsigned integer storage images.
layout(set=2, binding=0, rgba32ui) readonly uniform uimage1D        uImg1d;
layout(set=2, binding=1, rgba16ui) readonly uniform uimage2D        uImg2d;
layout(set=2, binding=2,  rgba8ui) readonly uniform uimage3D        uImg3d;
layout(set=2, binding=3, rgba32ui) readonly uniform uimage1DArray   uImg1dArr;
layout(set=2, binding=4, rgba32ui) readonly uniform uimage2DArray   uImg2dArr;
layout(set=2, binding=5, rgba32ui) readonly uniform uimageCube      uImgCube;
layout(set=2, binding=6, rgba32ui) readonly uniform uimageCubeArray uImgCubeArr;
layout(set=2, binding=7, rgba32ui) readonly uniform uimageBuffer    uImgBuf;

// Floating-point storage images.
layout(set=3, binding=0,      rgba32f)  writeonly uniform image1D        fImg1D;
layout(set=3, binding=1,      rgba16f)  writeonly uniform image2D        fImg2D;
layout(set=3, binding=2,         r32f)  writeonly uniform image3D        fImg3D;
layout(set=3, binding=3,        rgba8)  writeonly uniform imageCube      fImgCube;
layout(set=3, binding=4,  rgba8_snorm)  writeonly uniform image2DRect    fImg2DRect;
layout(set=3, binding=5,      rgba32f)  writeonly uniform image1DArray   fImg1DArray;
layout(set=3, binding=6,      rgba32f)  writeonly uniform image2DArray   fImg2DArray;
layout(set=3, binding=7,      rgba32f)  writeonly uniform imageCubeArray fImgCubeArray;
layout(set=3, binding=8,      rgba32f)  writeonly uniform imageBuffer    fImgBuffer;
layout(set=3, binding=9,      rgba32f)  writeonly uniform image2DMS      fImg2DMS;
layout(set=3, binding=10,     rgba32f)  writeonly uniform image2DMSArray fImg2DMSArray;

// Signed integer sampler images.
layout(set=4, binding=0)  uniform isampler1D        iSamp1D;
layout(set=4, binding=1)  uniform isampler2D        iSamp2D;
layout(set=4, binding=2)  uniform isampler3D        iSamp3D;
layout(set=4, binding=3)  uniform isamplerCube      iSampCube;
layout(set=4, binding=4)  uniform isampler2DRect    iSamp2DRect;
layout(set=4, binding=5)  uniform isampler1DArray   iSamp1DArray;
layout(set=4, binding=6)  uniform isampler2DArray   iSamp2DArray;
layout(set=4, binding=7)  uniform isamplerCubeArray iSampCubeArray;
layout(set=4, binding=8)  uniform isamplerBuffer    iSampBuffer;
layout(set=4, binding=9)  uniform isampler2DMS      iSamp2DMS;
layout(set=4, binding=10) uniform isampler2DMSArray iSamp2DMSArray;

// Unsigned integer sampler images.
layout(set=5, binding=0)  uniform usampler1D        uSamp1D;
layout(set=5, binding=1)  uniform usampler2D        uSamp2D;
layout(set=5, binding=2)  uniform usampler3D        uSamp3D;
layout(set=5, binding=3)  uniform usamplerCube      uSampCube;
layout(set=5, binding=4)  uniform usampler2DRect    uSamp2DRect;
layout(set=5, binding=5)  uniform usampler1DArray   uSamp1DArray;
layout(set=5, binding=6)  uniform usampler2DArray   uSamp2DArray;
layout(set=5, binding=7)  uniform usamplerCubeArray uSampCubeArray;
layout(set=5, binding=8)  uniform usamplerBuffer    uSampBuffer;
layout(set=5, binding=9)  uniform usampler2DMS      uSamp2DMS;
layout(set=5, binding=10) uniform usampler2DMSArray uSamp2DMSArray;

// Floating-point sampler images.
layout(set=6, binding=0)  uniform sampler1D        fSamp1D;
layout(set=6, binding=1)  uniform sampler2D        fSamp2D;
layout(set=6, binding=2)  uniform sampler3D        fSamp3D;
layout(set=6, binding=3)  uniform samplerCube      fSampCube;
layout(set=6, binding=4)  uniform sampler2DRect    fSamp2DRect;
layout(set=6, binding=5)  uniform sampler1DArray   fSamp1DArray;
layout(set=6, binding=6)  uniform sampler2DArray   fSamp2DArray;
layout(set=6, binding=7)  uniform samplerCubeArray fSampCubeArray;
layout(set=6, binding=8)  uniform samplerBuffer    fSampBuffer;
layout(set=6, binding=9)  uniform sampler2DMS      fSamp2DMS;
layout(set=6, binding=10) uniform sampler2DMSArray fSamp2DMSArray;

// Depth/stencil sampler images.
layout(set=7, binding=0) uniform sampler1DShadow        dsSamp1D;
layout(set=7, binding=1) uniform sampler2DShadow        dsSamp2D;
layout(set=7, binding=2) uniform samplerCubeShadow      dsSampCube;
layout(set=7, binding=3) uniform sampler2DRectShadow    dsSamp2DRect;
layout(set=7, binding=4) uniform sampler1DArrayShadow   dsSamp1DArray;
layout(set=7, binding=5) uniform sampler2DArrayShadow   dsSamp2DArray;
layout(set=7, binding=6) uniform samplerCubeArrayShadow dsSampCubeArray;

// Sampler states.
layout(set=8, binding=0) uniform sampler samp;
layout(set=8, binding=1) uniform samplerShadow sampShadow;

// Signed integer sampler images.
layout(set=9, binding=0)  uniform itexture1D        iTex1D;
layout(set=9, binding=1)  uniform itexture2D        iTex2D;
layout(set=9, binding=2)  uniform itexture3D        iTex3D;
layout(set=9, binding=3)  uniform itextureCube      iTexCube;
layout(set=9, binding=4)  uniform itexture2DRect    iTex2DRect;
layout(set=9, binding=5)  uniform itexture1DArray   iTex1DArray;
layout(set=9, binding=6)  uniform itexture2DArray   iTex2DArray;
layout(set=9, binding=7)  uniform itextureCubeArray iTexCubeArray;
layout(set=9, binding=8)  uniform itextureBuffer    iTexBuffer;
layout(set=9, binding=9)  uniform itexture2DMS      iTex2DMS;
layout(set=9, binding=10) uniform itexture2DMSArray iTex2DMSArray;

// Unsigned integer sampler images.
layout(set=10, binding=0)  uniform utexture1D        uTex1D;
layout(set=10, binding=1)  uniform utexture2D        uTex2D;
layout(set=10, binding=2)  uniform utexture3D        uTex3D;
layout(set=10, binding=3)  uniform utextureCube      uTexCube;
layout(set=10, binding=4)  uniform utexture2DRect    uTex2DRect;
layout(set=10, binding=5)  uniform utexture1DArray   uTex1DArray;
layout(set=10, binding=6)  uniform utexture2DArray   uTex2DArray;
layout(set=10, binding=7)  uniform utextureCubeArray uTexCubeArray;
layout(set=10, binding=8)  uniform utextureBuffer    uTexBuffer;
layout(set=10, binding=9)  uniform utexture2DMS      uTex2DMS;
layout(set=10, binding=10) uniform utexture2DMSArray uTex2DMSArray;

// Floating-point texture images.
layout(set=11, binding=0)  uniform texture1D        fTex1D;
layout(set=11, binding=1)  uniform texture2D        fTex2D;
layout(set=11, binding=2)  uniform texture3D        fTex3D;
layout(set=11, binding=3)  uniform textureCube      fTexCube;
layout(set=11, binding=4)  uniform texture2DRect    fTex2DRect;
layout(set=11, binding=5)  uniform texture1DArray   fTex1DArray;
layout(set=11, binding=6)  uniform texture2DArray   fTex2DArray;
layout(set=11, binding=7)  uniform textureCubeArray fTexCubeArray;
layout(set=11, binding=8)  uniform textureBuffer    fTexBuffer;
layout(set=11, binding=9)  uniform texture2DMS      fTex2DMS;
layout(set=11, binding=10) uniform texture2DMSArray fTex2DMSArray;

// Uniform block with dynamic binding number.
layout(set=12, binding=0) uniform Ubo {
    Data ds[4];
} ubo[];

// Storage buffer block with dynamic size.
layout(set=13, binding=0) buffer Ssbo {
    int ds[];
} ssbo;

layout(set=14, binding=0, input_attachment_index=0) uniform isubpassInput   iAttm;
layout(set=14, binding=1, input_attachment_index=1) uniform isubpassInputMS iAttmMS;
layout(set=14, binding=2, input_attachment_index=2) uniform usubpassInput   uAttm;
layout(set=14, binding=3, input_attachment_index=3) uniform usubpassInputMS uAttmMS;
layout(set=14, binding=4, input_attachment_index=4) uniform subpassInput    fAttm;
layout(set=14, binding=5, input_attachment_index=5) uniform subpassInputMS  fAttmMS;

// Acceleration structure (for ray-tracing).
layout(set=15, binding=0) uniform accelerationStructureEXT acc;

const int8_t INT8 = int8_t(1);
const int16_t INT16 = int16_t(1);
const int32_t INT32 = int32_t(1);
const int64_t INT64 = int64_t(1);

const uint8_t UINT8 = uint8_t(1);
const uint16_t UINT16 = uint16_t(1);
const uint32_t UINT32 = uint32_t(1);
const uint64_t UINT64 = uint64_t(1);

// (penguinliong) Don't know why but SPIR-V Tools disassemble fp16 values to
// mantissa and exponent bias which is pretty much a special case I don't wanna
// work with atm.
const float16_t FLOAT16 = float16_t(0.0);
const float32_t FLOAT32 = float32_t(1.0);
const float64_t FLOAT64 = float64_t(1.0);

void main() {
    rayQueryEXT ray_query;
    rayQueryProceedEXT(ray_query);
}
