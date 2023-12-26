# SPIR-Q

[![Crate](https://img.shields.io/crates/v/spirq)](https://crates.io/crates/spirq)
[![Documentation](https://docs.rs/spirq/badge.svg)](https://docs.rs/spirq)

SPIR-Q is a family of crates to help you process SPIR-V binary and assembly for Vulkan.

| Crate | Purpose |
|-|-|
|[spirq](spirq/README.md) [![Crate](https://img.shields.io/crates/v/spirq)](https://crates.io/crates/spirq)| Shader resource reflection, including descriptor bindings, pipeline inputs and outputs, specialization constants. |
|[spirq-core](spirq-core/README.md) [![Crate](https://img.shields.io/crates/v/spirq-core)](https://crates.io/crates/spirq-core)| Common structures and routines for SPIR-V IR analysis. |
|[spirq-spvasm](spirq-spvasm/README.md) [![Crate](https://img.shields.io/crates/v/spirq-spvasm)](https://crates.io/crates/spirq-spvasm)| SPIR-V assembler and disassembler. |

Commandline (CLI) tools are also provided for general use.

| Crate | Purpose |
|-|-|
|[shader-reflect](shader-reflect/README.md) [![Crate](https://img.shields.io/crates/v/shader-reflect)](https://crates.io/crates/shader-reflect)| Shader resource declaration reflector. |
|[spirq-dis](spirq-dis/README.md) [![Crate](https://img.shields.io/crates/v/spirq-dis)](https://crates.io/crates/spirq-dis)| SPIR-V disassembler frontend. Drop-in replacement of `spirv-dis`. |
|[spirq-as](spirq-as/README.md) [![Crate](https://img.shields.io/crates/v/spirq-as)](https://crates.io/crates/spirq-as)| SPIR-V assembler frontend. Drop-in replacement of `spirv-as`. |

## What's different from other crates?

A lot of my works stand in an overlapping field of compilers and graphics systems, so I often have to work with weird or even corrupted SPIR-V binaries. Obviously, existing tools like `rspirv` and `spirv-reflect` are not designed for this. I then decided to develop my own toolkit, which is now the SPIR-Q family.

Compared with SPIR-Q, `rspirv` has more strict requirements on SPIR-V physical layout, which makes it impossible to process bad test cases for other projects. `spirv-reflect` is a broadly used reflection tool and it's a wrapper crate of Khronos' official [SPIRV-Reflect](https://github.com/KhronosGroup/SPIRV-Reflect) tool. `SPIRV-Reflect`, however, was developed in pretty early days and it has some legacy bad designs (like a limit of 16 descriptors). [SPIRV-Tools](https://github.com/KhronosGroup/SPIRV-Reflect) provides Khronos' official assembler and disassembler, while it's hard to be integrated to other Rust projects.

On the other hand, the tools in SPIR-Q are more tolerant of the input quality. They don't check the semantics strictly to the spec. They won't stop processing unless there is a fatal structural problem making the input totally indecipherable. As a result, you might have to be familiar with the SPIR-V specification so that it serves you well, if you are developing other tools based on SPIR-Q.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
