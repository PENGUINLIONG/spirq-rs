# spirq

[![Crate](https://img.shields.io/crates/v/spirq)](https://crates.io/crates/spirq)
[![Documentation](https://docs.rs/spirq/badge.svg)](https://docs.rs/spirq)

`spirq` is a shader reflection tool to help you process SPIR-V binary and assembly for Vulkan. You can use `spirq` to query host-shader interfaces including descriptor bindings, pipeline inputs and outputs, specialization constants.

You can also use the commandline (CLI) tool [`shader-reflect`](shader-reflect/README.md) to use `spirq` without programming in Rust.

## Usage

See [the crate level readme](spirq/README.md) for detail.

## What's different from other crates?

A lot of my works stand in an overlapping field of compilers and graphics systems, so I often have to work with weird or even corrupted SPIR-V binaries. Obviously, existing tools like `rspirv` and `spirv-reflect` are not designed for this. I then decided to develop my own toolkit, which is now the spirq family.

Compared with spirq, `rspirv` has more strict requirements on SPIR-V physical layout, which makes it impossible to process bad test cases for other projects. `spirv-reflect` is a broadly used reflection tool and it's a wrapper crate of Khronos' official [SPIRV-Reflect](https://github.com/KhronosGroup/SPIRV-Reflect) tool. `SPIRV-Reflect`, however, was developed in pretty early days and it has some legacy bad designs (like a limit of 16 descriptors). [SPIRV-Tools](https://github.com/KhronosGroup/SPIRV-Reflect) provides Khronos' official assembler and disassembler, while it's hard to be integrated to other Rust projects.

On the other hand, the tools in spirq are more tolerant of the input quality. They don't check the semantics strictly to the spec. They won't stop processing unless there is a fatal structural problem making the input totally indecipherable. As a result, you might have to be familiar with the SPIR-V specification so that it serves you well, if you are developing other tools based on spirq.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
