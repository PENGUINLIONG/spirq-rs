# SPIR-Q Reflection Tool

[![Crate](https://img.shields.io/crates/v/spirq-reflect)](https://crates.io/crates/spirq-reflect)

[`spirq-reflect`](https://crates.io/crates/spirq-reflect) is a CLI frontend of the shader reflection library [`spirq`](https://github.com/PENGUINLIONG/spirq-rs). It generates reflection JSONs from SPIR-V shader binaries. You can install `spirq-reflect` with:

```bash
cargo install spirq-reflect
```

## Usage

Run the following command to reflect a SPIR-V binary and conclude a JSON report:

```bash
spirq-reflect assets/spirv-spec.frag.spv
```

or the following if you want all declared resources to be reflected even when they are never used by the shader.

```bash
spirq-reflect assets/spirv-spec.frag.spv --ref-all-rscs
```

Please run `spirq-reflect -h` to get a detailed description of all the available command-line options.

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
