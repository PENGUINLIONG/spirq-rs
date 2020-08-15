# SPIR-Q

[![Build Status](https://travis-ci.org/PENGUINLIONG/spirq-rs.svg?branch=master)](https://travis-ci.org/PENGUINLIONG/spirq-rs)
[![Crate](https://img.shields.io/crates/v/spirq)](https://crates.io/crates/spirq)
[![Documentation](https://docs.rs/spirq/badge.svg)](https://docs.rs/spirq)

SPIR-Q is a light weight library for SPIR-V pipeline metadata query.

## Why SPIR-Q?

Back in days of OpenGL, we have `glGetActiveUniformsiv` and other APIs to get pipeline metadata, so that we can determine the sizes, names, array strides and other information dynamically at runtime. However, the next-gen API, Vulkan, was deisgned not to support shader reflection so that the driver can be kept as thin as possible. SPIR-Q is an attempt to fill this gap.

SPIR-Q can be very useful for scenarios where we want some dynamic in pipeline construction, so that we don't have to refill those redundantly long `VkXxxCreateInfo`s all the time. It can also be used to automate filler code generation at compile time.

It should be noted that SPIR-V is targeting at Vulkan so OpenCL binaries are not supported.

## Usage

Please refer to the attached examples:

* [query](examples/query/main.rs): Query separate entry points in SPIR-V binaries.
* [pipeline](examples/pipeline/main.rs): Query a (conceptual) pipeline built from multiple shader modules.
* [spirv-spec](examples/spirv-spec/main.rs): Reflection of an example fragment shader program, which can be found in section 1.10 of the SPIR-V specification.
* [walk](examples/walk/main.rs): Enumerate offsets, symbols and types of all descriptor variables.
* [sampler-state](examples/sampler-state/main.rs): Separable sampler state support for HLSL-sourced SPIR-Vs.
* [benchmark](examples/benchmark/main.rs): Feel how fast SPIR-Q can be. (The log was generated from a `debug` run.)

Sample output are attached in the same directories as the code files.

## To-do

We are looking forward to improve SPIR-Q with features specified in SPIR-V 1.4. E.g. we no longer need to scan through function implementations to see what variables are used by an entry point. But unfortunately we have been blocked by [`spirv_headers`](https://crates.io/crates/spirv_headers). SPIR-Q will follow up as soon as `spirv-headers` updates.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
