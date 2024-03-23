//! # spq: Light Weight SPIR-V Query Utility for Graphics.
//!
//! spq is a light weight library for SPIR-V pipeline metadata query, which
//! can be very useful for dynamic graphics/compute pipeline construction,
//! shader debugging and so on. spq is currently compatible with a subset of
//! SPIR-V 1.5, with most of graphics capabilities but no OpenCL kernel
//! capabilities covered.
//!
//! ## How-to
//!
//! ```ignore
//! use spirq::prelude::*;
//!
//! let entry_points = ReflectConfig::new()
//!     // Load SPIR-V data into `[u32]` buffer `spv_words`.
//!     .spv(spv_words)
//!     // Set this true if you want to reflect all resources no matter it's
//!     // used by an entry point or not.
//!     .ref_all_rscs(true)
//!     // Combine sampled image and separated sampler states if they are bound
//!     // to the same binding point.
//!     .combine_img_samplers(true)
//!     // Generate unique names for types and struct fields to help further
//!     // processing of the reflection data. Otherwise, the debug names are
//!     // assigned.
//!     .gen_unique_names(true)
//!     // Specialize the constant at `SpecID=3` with unsigned integer 7. The
//!     // constants specialized here won't be listed in the result entry point's
//!     // variable list.
//!     .specialize(3, ConstantValue::U32(7))
//!     // Do the work.
//!     .reflect()
//!     .unwrap();
//! // All extracted entry point data are available in `entry_points` now.
//! ```
//!
//! By calling [`reflect`] of the wrapper type [`SpirvBinary`], every entry
//! point in the binary are analyzed and reported as one or more
//! [`EntryPoint`]s with all the types and bindings/locations.
//!
//! ## Size calculation
//!
//! The struct member offsets and array/matrix strides are specified in SPIR-V
//! files. With these information spq deduce the minimal size required for
//! to contain an instance of a type. However, spq cannot handle dynamically-
//! sized arrays, and it will treat such arrays as zero-sized. The user has to
//! handle such SSBO-like themselves via [`Type`] APIs.
//!
//! Note: It should be noted that descriptor multibinds are treated like single-
//! binds because although they use the same syntax as arrays, they are not
//! actually arrays.
//!
//! Note: Although `spv` files generated directly from compilers normally keep
//! the nameing data, it should be noticed that names are debug information that
//! might be wiped out during compression.
//!
//! [`SpirvBinary`]: struct.SpirvBinary.html
//! [`EntryPoint`]: struct.EntryPoint.html
//! [`reflect`]: reflect/struct.ReflectConfig.html#method.reflect
//! [`Type`]: ty/enum.Type.html
mod instr;

pub mod entry_point;
pub mod inspect;
pub mod reflect;
pub mod reflect_cfg;

#[cfg(test)]
mod tests;

pub use spq_core::annotation;
pub use spq_core::constant;
pub use spq_core::error;
pub use spq_core::evaluator;
pub use spq_core::func;
pub use spq_core::parse;
pub use spq_core::spirv;
pub use spq_core::ty;
pub use spq_core::var;

pub use reflect_cfg::ReflectConfig;

// Re-exports.
pub mod prelude {
    pub use super::ReflectConfig;
    pub use super::{
        constant::ConstantValue,
        entry_point::{EntryPoint, ExecutionModel},
        error::{Error, Result},
        parse::SpirvBinary,
        ty::{AccessType, DescriptorType, SpirvType, Type},
        var::{DescriptorBinding, InterfaceLocation, SpecId, Variable},
    };
}
