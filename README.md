# SPIR-Q

[![Crate](https://img.shields.io/crates/v/spirq)](https://crates.io/crates/spirq)
[![Documentation](https://docs.rs/spirq/badge.svg)](https://docs.rs/spirq)

SPIR-Q is a light weight library for SPIR-V pipeline metadata query, which can be very useful for dynamic graphics/compute pipeline construction, shader debugging and so on. SPIR-Q is currently compatible with a subset of SPIR-V 1.5, with most of graphics capabilities but no OpenCL kernel capabilities covered.

## Usage

The project is still in progress, but some of the functionalities are already in good shape. Please refer to the attached examples:

* [query](examples/query/main.rs): Query separate entry points in SPIR-V binaries.
* [pipeline](examples/pipeline/main.rs): Query a (conceptual) pipeline built from multiple shader modules.
* [spirv-spec](examples/spirv-spec/main.rs): Reflection of an example fragment shader program, which can be found in section 1.10 of the SPIR-V specification.

Sample output are attached in the same directories as the code files.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
