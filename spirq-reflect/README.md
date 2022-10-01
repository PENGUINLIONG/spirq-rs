# SPIR-Q Reflection Tool

`spirq-reflect` is a CLI tool to generate reflection JSON from SPIR-V shader binaries. You can install `spirq-reflect` with:

```bash
cargo install spirq-reflect
```

To reflect a shader:

```bash
spirq-reflect --in-path assets/spirv-spec.frag.spv
```

You will get the following output:

```
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
