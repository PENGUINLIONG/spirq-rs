# spq Disassembler

[![Crate](https://img.shields.io/crates/v/spq-dis)](https://crates.io/crates/spq-dis)

spq Disassembler (`spq-dis`) is a SPIR-V disassembler written in pure Rust. It is a drop-in replacement of the official disassembler `spirv-dis` with the same commandline arguments.

## Install

You can install `spq-dis` from cargo with:

```bash
cargo install spq-dis
```

## Usage

To disassemble SPIR-V binary, you can either pass the SPIR-V file path by argument or pipe the content in.

```bash
spq-dis [INPUT].spv -o [OUTPUT].spvasm
# - or -
cat [INPUT].spv | spq-dis -o [OUTPUT].spvasm
```

`spq-dis` is a CLI tool for end users. You can also integrate the disassembler to your application from the library crate [`spq-spvasm`](../spq-spvasm/README.md).

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
