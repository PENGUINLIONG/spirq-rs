# spq Assembler

[![Crate](https://img.shields.io/crates/v/spq-as)](https://crates.io/crates/spq-as)

spq Assembler (`spq-as`) is a SPIR-V assembler written in pure Rust. It is a drop-in replacement of the official assembler `spirv-as` with the same commandline arguments.

## Install

You can install `spq-as` from cargo with:

```bash
cargo install spq-as
```

## Usage

To assemble SPIR-V binary from SPIR-V assembly, you can either pass the source file path by argument or pipe the code in.

```bash
spq-as [INPUT].spvasm -o [OUTPUT].spv
# - or -
cat [INPUT].spvasm | spq-as -o [OUTPUT].spv
```

`spq-as` is a CLI tool for end users. You can also integrate the assembler to your application from the library crate [`spq-spvasm`](../spq-spvasm/README.md).

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
