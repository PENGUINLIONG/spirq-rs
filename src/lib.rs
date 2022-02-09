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
//! // Load SPIR-V data into `[u32]` buffer `spv_words`.
//! let spv: SpirvBinary = spv_words.into();
//! let entries = spv.reflect_vec().unwrap();
//! // All extracted entry point data are available in `entries`.
//! ```
//!
//! By calling [`reflect`] of the wrapper type [`SpirvBinary`], every entry
//! point in the binary are analyzed and reported as one or more
//! [`EntryPoint`]s. Each entry point has a [`Manifest`] that supports queries
//! from allocation requirement to fine-grained typing details.
//!
//! ## Size calculation
//!
//! The struct member offsets and array/matrix strides are specified in SPIR-V
//! files. With these information SPIR-Q deduce the minimal size required for
//! to contain an instance of a type. However, SPIR-Q cannot handle dynamically-
//! sized arrays, and it will treat such arrays as zero-sized. The user has to
//! handle such SSBO-like themselves via [`Type`] APIs.
//!
//! ## Symbol resolution
//!
//! SPIR-Q uses a very simple solution to help you locate any metadata including
//! input/output variables, descriptors and variables defined inside those
//! descriptors. We call it a [`Symbol`]. A symbol is a dot-separated list of
//! identifiers. Identifiers can be an index or a name literal (or empty for the
//! push constant block.)
//!
//! Input/output variables are referred to by their locations. The following
//! are examples of input/output variable symbols:
//!
//! ```ignore
//! 1
//! aTexCoord
//! vWorldPosition
//! 1.2 // ERROR: I/O variables cannot be nested.
//! gl_Position // WARNING: Built-in variables are ignored during reflection.
//! ```
//!
//! Descriptors have to be referred to with both the descriptor set number and
//! its binding point number specified. The following are valid symbols for
//! descriptor variables:
//!
//! ```ignore
//! 0.1 // Refering to the descriptor at set 0 on binding 1.
//! light.0 // Refering to the first member of block 'light'.
//! 1.0.bones.4 // Refering to the 5th element of array member `bones` in descriptor `1.0`.
//! .modelview // Push constants can be referred to by either an empty identifier or its variable name.
//! ```
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
//! [`reflect`]: struct.SpirvBinary.html#method.reflect
//! [`Manifest`]: struct.Manifest.html
//! [`Type`]: ty/enum.Type.html
//! [`Symbol`]: sym/struct.Symbol.html
mod consts;
mod instr;
mod inspect;
#[cfg(test)]
mod tests;
pub mod reflect;
pub mod parse;
pub mod error;
pub mod ty;

use std::convert::TryInto;
use std::fmt;
use std::iter::FromIterator;
use fnv::FnvHashMap as HashMap;
use reflect::{ReflectIntermediate};
use inspect::{NopInspector, FnInspector};

use parse::{Instrs, Instr};
pub use ty::{AccessType, Type};
pub use error::{Error, Result};
pub use reflect::{InterfaceLocation, DescriptorBinding, DescriptorType, SpecId,
    Locator, Variable, ExecutionMode, ExecutionModel};

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
    pub(crate) fn instrs<'a>(&'a self) -> Instrs<'a> { Instrs::new(&self.0) }
    /// Reflect the SPIR-V binary and extract all the entry points. It's
    /// the same as `refelct_vec` while it returns a boxed slice. You may find
    /// `reflect_vec` more handy but this is kept for API compatibility.
    #[deprecated(since="0.4.6", note="please use `reflect_vec` instead")]
    pub fn reflect(&self) -> Result<Box<[EntryPoint]>> {
        self.reflect_vec()
            .map(|x| x.into_boxed_slice())
    }
    /// Reflect the SPIR-V binary and extract all the entry points.
    pub fn reflect_vec(&self) -> Result<Vec<EntryPoint>> {
        let inspector = NopInspector();
        reflect::ReflectIntermediate::reflect(self.instrs(), inspector)?
            .collect_entry_points()
    }
    /// Similar to `reflect_vec` while you can inspect each instruction during
    /// the parse.
    pub fn reflect_vec_inspect<F: FnMut(&ReflectIntermediate<'_>, &Instr<'_>)>(
        &self,
        inspector: F
    ) -> Result<Vec<EntryPoint>> {
        let inspector = FnInspector::<F>(inspector);
        reflect::ReflectIntermediate::reflect(self.instrs(), inspector)?
            .collect_entry_points()
    }
    // Reflect the SPIR-V binary fast. This method returns the only entry point
    // in the SPIR-V binary, with all declared descriptors and interface
    // variables enumerated. Unlike `reflect_vec`, the resources are not
    // filtered based on references in entry points.
    //
    // It can be a faster option in case the SPIR-V only contains one single
    // entry point, and no descriptor or variable have conflicting binding
    // points or locations.
    pub fn reflect_fast(&self) -> Result<EntryPoint> {
        let inspector = NopInspector();
        reflect::ReflectIntermediate::reflect(self.instrs(), inspector)?
            .collect_module_as_entry_point()
    }
    /// Similar to `reflect_fast` while you can inspect each instruction during
    /// the parse.
    pub fn reflect_fast_inspect<F: FnMut(&ReflectIntermediate<'_>, &Instr<'_>)>(
        &self,
        inspector: F
    ) -> Result<EntryPoint> {
        let inspector = FnInspector::<F>(inspector);
        reflect::ReflectIntermediate::reflect(self.instrs(), inspector)?
            .collect_module_as_entry_point()
    }
    pub fn words(&self) -> &[u32] {
        &self.0
    }
    pub fn bytes(&self) -> &[u8] {
        unsafe {
            let len = self.0.len() * std::mem::size_of::<u32>();
            let ptr = self.0.as_ptr() as *const u8;
            std::slice::from_raw_parts(ptr, len)
        }
    }
    pub fn into_words(self) -> Vec<u32> { self.0 }
}


/// Internal hasher for type equality check.
pub(crate) fn hash<H: std::hash::Hash>(h: &H) -> u64 {
    use std::hash::{BuildHasher, Hasher};
    let mut hasher = fnv::FnvBuildHasher::default().build_hasher();
    h.hash(&mut hasher);
    hasher.finish()
}


#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub(crate) enum ResolveKind {
    Input,
    Output,
}

// Resolution results.

/// Specialization constant resolution result.
#[derive(PartialEq, Eq, Debug)]
pub struct SpecConstantResolution<'a> {
    /// Specialization ID, aka the `constant_id` layout property in GLSL.
    pub spec_id: SpecId,
    /// Type of the specialization constant.
    pub ty: &'a Type,
}

/// Interface variables resolution result.
#[derive(PartialEq, Eq, Debug)]
pub struct InterfaceVariableResolution<'a> {
    /// Location of the current interface variable. It should be noted that
    /// matrix types can take more than one location.
    pub location: InterfaceLocation,
    /// Type of the resolution target.
    pub ty: &'a Type,
}

/// Push constant resolution result.
#[derive(PartialEq, Eq, Debug)]
pub struct PushConstantResolution<'a> {
    /// Type of the push constant block. This is expected to be struct.
    pub ty: &'a Type,
    /// Resolution of a variable in the push constant block, if the resolution
    /// doesn't end at the block.
    pub member_var_res: Option<MemberVariableResolution<'a>>,
}
/// Descriptor variable resolution result.
#[derive(PartialEq, Eq, Debug)]
pub struct DescriptorResolution<'a> {
    /// Descriptor set and binding point of the descriptor.
    pub desc_bind: DescriptorBinding,
    /// SPIR-V Type of descriptor resource.
    pub ty: &'a Type,
    /// Descriptor resource type.
    pub desc_type: DescriptorType,
    /// Resolution of a variable in the descriptor, if the resolution doesn't
    /// end at a descriptor type.
    pub member_var_res: Option<MemberVariableResolution<'a>>,
}
/// Member variable resolution result.
#[derive(PartialEq, Eq, Debug)]
pub struct MemberVariableResolution<'a> {
    /// Offset to the resolution target from the beginning of buffer.
    pub offset: usize,
    /// Type of the resolution target.
    pub ty: &'a Type,
}

/// Entry point specialization descriptions.
#[derive(Clone, Hash, Debug)]
pub struct Specialization {
    name: Option<String>,
    // Specialization constant ID.
    spec_id: SpecId,
    // Type of specialization constant.
    ty: Type,
}


// SPIR-V program entry points.

/// Representing an entry point described in a SPIR-V.
#[derive(Clone)]
pub struct EntryPoint {
    /// Entry point execution model.
    pub exec_model: ExecutionModel,
    /// Name of the entry point.
    pub name: String,
    /// Variables that contains input, output and descriptor type information.
    /// Note that it is possible that multiple resources are bound to a same
    /// `Locator` so this is not a map.
    pub vars: Vec<Variable>,
    /// Specialization description of the entry point.
    pub specs: Vec<Specialization>,
    /// Execution modes the entry point will execute in, including predefined
    /// compute shader local sizes and specialization constant IDs of local
    /// sizes.
    pub exec_modes: Vec<ExecutionMode>,
}
impl fmt::Debug for EntryPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct InterfaceLocationDebugHelper<'a>(Vec<(InterfaceLocation, &'a Type)>);
        impl<'a> fmt::Debug for InterfaceLocationDebugHelper<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_map()
                    .entries(self.0.iter().cloned())
                    .finish()
            }
        }
        struct DescriptorBindingDebugHelper<'a>(Vec<(DescriptorBinding, &'a DescriptorType)>);
        impl<'a> fmt::Debug for DescriptorBindingDebugHelper<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_map()
                    .entries(self.0.iter().cloned())
                    .finish()
            }
        }
        f.debug_struct(&self.name)
            .field("exec_model", &self.exec_model)
            .field("name", &self.name)
            .field("vars", &self.vars)
            .field("specs", &self.specs)
            .field("exec_modes", &self.exec_modes)
            .finish()
    }
}
impl EntryPoint {
    /// Merge `DescriptorType::SampledImage` and `DescriptorType::Sampler` if
    /// they are bound to a same binding point with a same number of bindings.
    pub fn merge_combined_image_sampler(&mut self) {
        let mut samplers = Vec::<Variable>::new();
        let mut imgs = Vec::<Variable>::new();
        let mut out_vars = Vec::<Variable>::new();

        for var in self.vars.drain(..) {
            if let Variable::Descriptor { desc_ty, .. } = &var {
                match desc_ty {
                    DescriptorType::Sampler() => {
                        samplers.push(var);
                        continue;
                    },
                    DescriptorType::SampledImage() => {
                        imgs.push(var);
                        continue;
                    },
                    _ => {},
                }
            } 
            out_vars.push(var);
        }

        for sampler_var in samplers {
            let (sampler_desc_bind, sampler_nbind) = {
                if let Variable::Descriptor { desc_bind, nbind, .. } = sampler_var {
                    (desc_bind, nbind)
                } else { unreachable!(); }
            };

            let mut combined_imgs = Vec::new();
            imgs = imgs.drain(..)
                .filter_map(|var| {
                    let succ =
                        var.locator() == Locator::Descriptor(sampler_desc_bind) &&
                        var.nbind() == Some(sampler_nbind);
                    if succ {
                        Some(var)
                    } else {
                        combined_imgs.push(var);
                        None
                    }
                })
                .collect();

            if combined_imgs.is_empty() {
                // If the sampler can be combined with no texture, just put it
                // back.
                out_vars.push(sampler_var);
            } else {
                // For any texture that can be combined with this sampler,
                // create a new combined image sampler.
                for img_var in combined_imgs {
                    if let Variable::Descriptor { name, ty, .. } = img_var {
                        if let Type::Image(img_ty) = ty {
                            let out_var = Variable::Descriptor {
                                name,
                                desc_bind: sampler_desc_bind,
                                desc_ty: DescriptorType::CombinedImageSampler(),
                                ty: Type::SampledImage(ty::SampledImageType::new(img_ty)),
                                nbind: sampler_nbind,
                            };
                            out_vars.push(out_var);
                        } else { unreachable!(); }
                    } else { unreachable!(); }
                }
            }
        }
    }
}
