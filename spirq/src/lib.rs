//! # SPIR-Q: Light Weight SPIR-V Query Utility for Graphics.
//!
//! SPIR-Q is a light weight library for SPIR-V pipeline metadata query, which
//! can be very useful for dynamic graphics/compute pipeline construction,
//! shader debugging and so on. SPIR-Q is currently compatible with a subset of
//! SPIR-V 1.5, with most of graphics capabilities but no OpenCL kernel
//! capabilities covered.
//!
//! ## How-to
//!
//! ```ignore
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
//! files. With these information SPIR-Q deduce the minimal size required for
//! to contain an instance of a type. However, SPIR-Q cannot handle dynamically-
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
mod consts;
mod instr;
mod inspect;
#[cfg(test)]
mod tests;
pub mod reflect;
pub mod parse;
pub mod error;
pub mod ty;
pub mod walk;

use std::convert::TryInto;
use std::fmt;
use std::iter::FromIterator;

pub use error::{Error, Result};
pub use reflect::{ReflectConfig, InterfaceLocation, DescriptorBinding,
    DescriptorType, Variable, Locator, AccessType, ExecutionMode,
    ExecutionModel, ConstantValue};

/// SPIR-V program binary.
#[derive(Debug, Default, Clone)]
pub struct SpirvBinary(Vec<u32>);
impl From<Vec<u32>> for SpirvBinary {
    fn from(x: Vec<u32>) -> Self { SpirvBinary(x) }
}
impl From<&[u32]> for SpirvBinary {
    fn from(x: &[u32]) -> Self { SpirvBinary(x.to_owned()) }
}
impl FromIterator<u32> for SpirvBinary {
    fn from_iter<I: IntoIterator<Item=u32>>(iter: I) -> Self {
        SpirvBinary(iter.into_iter().collect::<Vec<u32>>())
    }
}
impl From<&[u8]> for SpirvBinary {
    fn from(x: &[u8]) -> Self {
        if x.len() == 0 { return SpirvBinary::default(); }
        x.chunks_exact(4)
            .map(|x| x.try_into().unwrap())
            .map(match x[0] {
                0x03 => u32::from_le_bytes,
                0x07 => u32::from_be_bytes,
                _ => return SpirvBinary::default(),
            })
            .collect::<SpirvBinary>()
    }
}
impl From<Vec<u8>> for SpirvBinary {
    fn from(x: Vec<u8>) -> Self { SpirvBinary::from(x.as_ref() as &[u8]) }
}

impl SpirvBinary {
    pub fn words(&self) -> &[u32] {
        &self.0
    }
    pub fn into_words(self) -> Vec<u32> { self.0 }
}



// SPIR-V program entry points.

/// Representing an entry point described in a SPIR-V.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EntryPoint {
    /// Entry point execution model.
    pub exec_model: ExecutionModel,
    /// Name of the entry point.
    pub name: String,
    /// Variables that contains specialization constant, input, output and
    /// descriptor type information.
    ///
    /// Note that it is possible that multiple resources are bound to a same
    /// `Locator` so this is not a map.
    pub vars: Vec<Variable>,
    /// Execution modes the entry point will execute in, including predefined
    /// compute shader local sizes and specialization constant IDs of local
    /// sizes.
    pub exec_modes: Vec<ExecutionMode>,
}
impl fmt::Debug for EntryPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct(&self.name)
            .field("exec_model", &self.exec_model)
            .field("name", &self.name)
            .field("vars", &self.vars)
            .field("exec_modes", &self.exec_modes)
            .finish()
    }
}
