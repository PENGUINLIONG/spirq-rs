# spq

`spq` is a family of crates to help you process SPIR-V binary and assembly for Vulkan.

> Note: The prefix `spirq-` of several sub-crates has been replaced with `spq-` in the v1.2.0 release.

| Crate | Purpose |
|-|-|
|[spirq](spirq/README.md) [![Crate](https://img.shields.io/crates/v/spirq)](https://crates.io/crates/spirq)| Shader resource reflection, including descriptor bindings, pipeline inputs and outputs, specialization constants. |
|[spq-core](spq-core/README.md) [![Crate](https://img.shields.io/crates/v/spq-core)](https://crates.io/crates/spq-core)| Common structures and routines for SPIR-V IR analysis. |
|[spq-spvasm](spq-spvasm/README.md) [![Crate](https://img.shields.io/crates/v/spq-spvasm)](https://crates.io/crates/spq-spvasm)| SPIR-V assembler and disassembler. |

Commandline (CLI) tools are also provided for general use.

| Crate | Purpose |
|-|-|
|[shader-reflect](shader-reflect/README.md) [![Crate](https://img.shields.io/crates/v/shader-reflect)](https://crates.io/crates/shader-reflect)| Shader resource declaration reflector. |
|[spq-dis](spq-dis/README.md) [![Crate](https://img.shields.io/crates/v/spq-dis)](https://crates.io/crates/spq-dis)| SPIR-V disassembler frontend. Drop-in replacement of `spirv-dis`. |
|[spq-as](spq-as/README.md) [![Crate](https://img.shields.io/crates/v/spq-as)](https://crates.io/crates/spq-as)| SPIR-V assembler frontend. Drop-in replacement of `spirv-as`. |

## What's different from other crates?

A lot of my works stand in an overlapping field of compilers and graphics systems, so I often have to work with weird or even corrupted SPIR-V binaries. Obviously, existing tools like `rspirv` and `spirv-reflect` are not designed for this. I then decided to develop my own toolkit, which is now the spirq family.

Compared with spirq, `rspirv` has more strict requirements on SPIR-V physical layout, which makes it impossible to process bad test cases for other projects. `spirv-reflect` is a broadly used reflection tool and it's a wrapper crate of Khronos' official [SPIRV-Reflect](https://github.com/KhronosGroup/SPIRV-Reflect) tool. `SPIRV-Reflect`, however, was developed in pretty early days and it has some legacy bad designs (like a limit of 16 descriptors). [SPIRV-Tools](https://github.com/KhronosGroup/SPIRV-Reflect) provides Khronos' official assembler and disassembler, while it's hard to be integrated to other Rust projects.

On the other hand, the tools in spirq are more tolerant of the input quality. They don't check the semantics strictly to the spec. They won't stop processing unless there is a fatal structural problem making the input totally indecipherable. As a result, you might have to be familiar with the SPIR-V specification so that it serves you well, if you are developing other tools based on spirq.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
