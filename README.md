# SPIR-Q

[![Build Status](https://travis-ci.com/PENGUINLIONG/spirq-rs.svg?branch=master)](https://travis-ci.com/PENGUINLIONG/spirq-rs)
[![Crate](https://img.shields.io/crates/v/spirq)](https://crates.io/crates/spirq)
[![Documentation](https://docs.rs/spirq/badge.svg)](https://docs.rs/spirq)

SPIR-Q is a light weight library for SPIR-V pipeline metadata query, supporting upto SPIR-V 1.5 specification aligned with Vulkan 1.2.

## Why SPIR-Q?

Back in days of OpenGL, we have `glGetActiveUniformsiv` and other APIs to get pipeline metadata, so that we can determine the sizes, names, array strides and other information dynamically at runtime. However, the next-gen API, Vulkan, was deisgned not to support shader reflection so that the driver can be kept as thin as possible. SPIR-Q is an attempt to fill this gap.

SPIR-Q can be very useful for scenarios where we want some dynamic in pipeline construction, so that we don't have to refill those redundantly long `VkXxxCreateInfo`s all the time. It can also be used to automate filler code generation at compile time.

It should be noted that SPIR-V is targeting at Vulkan so OpenCL binaries are not supported.

## Usage

```rust
use spirq::*;
let entry_points = ReflectConfig::new()
    // Load SPIR-V data into `[u32]` buffer `spv_words`.
    .spv(spv_words)
    // Set this true if you want to reflect all resources no matter it's
    // used by an entry point or not.
    .ref_all_rscs(true)
    // Combine sampled image and separated sampler states if they are bound
    // to the same binding point.
    .combine_img_samplers(true)
    // Specialize the constant at `SpecID=3` with unsigned integer 7. The
    // constants specialized here won't be listed in the result entry point's
    // variable list.
    .specialize(3, ConstantValue::U32(7))
    // Do the work.
    .reflect()
    .unwrap();
// All extracted entry point data are available in `entry_points` now.
```

Please also refer to the attached examples:

* [walk](examples/walk): Enumerate offsets, symbols and types of all descriptor variables.
* [inspect](examples/inspect): Customize shader reflection with your own inspector function.
* [gallery](examples/gallery): All data types in GLSL.

Sample output are attached in the same directories as the code files.

## License

This project is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
