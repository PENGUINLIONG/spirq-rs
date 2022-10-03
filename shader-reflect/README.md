# Shader Reflect

[![Crate](https://img.shields.io/crates/v/shader-reflect)](https://crates.io/crates/shader-reflect)

[`shader-reflect`](https://crates.io/crates/shader-reflect) is a CLI frontend of the shader reflection library [`spirq`](https://github.com/PENGUINLIONG/spirq-rs). It generates reflection JSONs from SPIR-V shader binaries. You can install `shader-reflect` with:

```bash
cargo install shader-reflect
```

## Usage

Run the following command to reflect a **GLSL/HLSL shader source** or a **SPIR-V binary** to conclude a JSON report:

```bash
# GLSL Shader source.
shader-reflect assets/spirv-spec.frag
# SPIR-V binary.
shader-reflect assets/spirv-spec.frag.spv
```

or the following if you want all declared resources to be reflected even when they are never used by the shader.

```bash
shader-reflect assets/spirv-spec.frag.spv --reference-all-resources
```

Please run `shader-reflect -h` to get a detailed description of all the available command-line options.

```
Light weight SPIR-V query utility for graphics. (CLI)

Usage: shader-reflect [OPTIONS] <IN_PATH>

Arguments:
  <IN_PATH>  Input SPIR-V file paths.

Options:
  -o, --out-path <OUT_PATH>        Output JSON file path. The output is printed to stdout if this path is not given.
      --reference-all-resources    Reference all resources even they are never used by the entry points. By default, only the referenced resources are reflected.
      --combine-image-samplers     Combine separate sampled image and sampler at a same descriptor set and binding. By default, they are listed as separate objects.
      --generate-unique-names      Generate unique names for every resource variable, structure types, and type members. By default, the names are assigned with debug annotations in the input SPIR-V.
  -I <INCLUDE_DIRECTORIES>         The base directories of standard includes (`#include <...>`) in compilation of GLSL or HLSL shader sources.
  -D <DEFINITIONS>                 Compiler definitions in compilation of GLSL or HLSL shader sources.
  -e, --entry-point <ENTRY_POINT>  Shader entry point function name in compilation of GLSL or HLSL shader.
  -h, --help                       Print help information
  -V, --version                    Print version information
```

## Supported Shader Types

`shader-reflect` supports all shader types available in Vulkan including ray-tracing shaders and mesh shaders. Compiling from shader sources, the shader type is inferred from the extension names. The extension name follows the convention of `glslangValidator`.

|Extension|Shader Stage|
|-|-|
|`.vert`|Vertex shader|
|`.tesc`|Tessellation control shader (or hull shader)|
|`.tese`|Tessellation evaluation shader (or domain shader)|
|`.geom`|Geometry shader|
|`.frag`|Fragment shader|
|`.comp`|Compute shader|
|`.mesh`|Mesh shader|
|`.task`|Task shader|
|`.rgen`|Ray-generation shader|
|`.rint`|Intersection shader|
|`.rahit`|Any-hit shader|
|`.rchit`|Closest-hit shader|
|`.rmiss`|Miss shader|
|`.rcall`|Callable shader|

A suffix of `.glsl` or `.hlsl` can be appended to explicitly specify the shading lanugages. `shader-reflect` assumes GLSL if not given. For example, `foo.vert` and `bar.frag.glsl` are considered GLSL shaders; `baz.comp.hlsl` is considered a HLSL shader.

## Example Output

The [`spirv-spec.frag.spv`](https://github.com/PENGUINLIONG/spirq-rs/tree/master/assets/spirv-spec.frag) binary in the [`spirq`](https://github.com/PENGUINLIONG/spirq-rs) repository gives the following output:

```json
{
  "EntryPoint": "main",
  "ExecutionModel": "Fragment",
  "Variables": {
    "Inputs": [
      {
        "Name": "_42",
        "Location": 2,
        "Component": 0,
        "Type": "vec4<f32>"
      },
      {
        "Name": "_57",
        "Location": 1,
        "Component": 0,
        "Type": "vec4<f32>"
      },
      {
        "Name": "_33",
        "Location": 0,
        "Component": 0,
        "Type": "vec4<f32>"
      }
    ],
    "Outputs": [
      {
        "Name": "_31",
        "Location": 0,
        "Component": 0,
        "Type": "vec4<f32>"
      }
    ],
    "Descriptors": [
      {
        "Name": "_20",
        "Set": 0,
        "Binding": 0,
        "DescriptorType": "UniformBuffer",
        "Type": {
          "Kind": "Struct",
          "Members": [
            {
              "Name": "_18_0",
              "Offset": 0,
              "MemberType": {
                "Kind": "Struct",
                "Members": [
                  {
                    "Name": "_17_0",
                    "Offset": 0,
                    "MemberType": "u32"
                  },
                  {
                    "Name": "_17_1",
                    "Offset": 16,
                    "MemberType": {
                      "Kind": "Array",
                      "ElementType": "vec4<f32>",
                      "Count": 5,
                      "Stride": 16
                    }
                  },
                  {
                    "Name": "_17_2",
                    "Offset": 96,
                    "MemberType": "i32"
                  }
                ]
              }
            },
            {
              "Name": "_18_1",
              "Offset": 112,
              "MemberType": "u32"
            }
          ]
        },
        "Count": 1
      }
    ],
    "PushConstants": [],
    "SpecConstants": []
  }
}
```
