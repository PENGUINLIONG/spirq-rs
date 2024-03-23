# spq Tools for SPIR-V Assembly

[![Crate](https://img.shields.io/crates/v/spq-spvasm)](https://crates.io/crates/spq-spvasm)
[![Documentation](https://docs.rs/spq-spvasm/badge.svg)](https://docs.rs/spq-spvasm)

spq tools for SPIR-V Assembly provides useful auxiliaries for shader and shader tool development. The toolkit currently provides the following tools:

- Assembler [`spq-as`](../spq-as/README.md) (drop-in replacement of `spirv-as`)
- Disassemlber [`spq-dis`](../spq-dis) (drop-in replacement of `spirv-dis`)

`spq-as` and `spq-dis` share the same commandline arguments as [the official tools](https://github.com/KhronosGroup/SPIRV-Tools). They consume the same SPIR-V assembly syntax as described [here](https://github.com/KhronosGroup/SPIRV-Tools/blob/main/docs/syntax.md).

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
