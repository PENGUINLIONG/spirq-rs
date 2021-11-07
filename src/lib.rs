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
pub mod sym;
pub mod error;
pub mod ty;

use std::convert::TryInto;
use std::fmt;
use std::iter::FromIterator;
use std::ops::Deref;
use fnv::FnvHashMap as HashMap;
use reflect::{ReflectIntermediate};
use inspect::{NopInspector, FnInspector};

use parse::{Instrs, Instr};
pub use ty::{Type, DescriptorType};
pub use sym::{Seg, Segs, Sym, Symbol};
pub use error::{Error, Result};
pub use reflect::{AccessType, InterfaceLocation, DescriptorBinding, SpecId,
    Locator, Variable, ExecutionModel};

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
    /// Type of the descriptor.
    pub desc_ty: &'a DescriptorType,
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

/// A set of information used to describe variable typing and routing.
#[derive(Default, Clone)]
pub struct Manifest {
    var_map: HashMap<Locator, Variable>,
    var_name_map: HashMap<String, Locator>,
}
impl Manifest {
    fn clear_inputs(&mut self) {
        self.var_map.retain(|k, _| if let Locator::Input(_) = k { false } else { true });
    }
    fn clear_outputs(&mut self) {
        self.var_map.retain(|k, _| if let Locator::Output(_) = k { false } else { true });
    }
    fn merge_names(&mut self, other: &Manifest) -> Result<()> {
        use std::collections::hash_map::Entry::{Vacant, Occupied};
        for (name, locator) in other.var_name_map.iter() {
            match self.var_name_map.entry(name.to_owned()) {
                Vacant(entry) => { entry.insert(locator.clone()); },
                Occupied(entry) => if entry.get() != locator {
                    // Mismatched names are not allowed.
                    return Err(Error::MismatchedManifest);
                },
            }
        }
        Ok(())
    }
    fn merge_var(dst_var: &mut Variable, src_var: &Variable) -> Result<()> {
        match (dst_var, src_var) {
            (
                Variable::PushConstant(Type::Struct(dst_struct_ty)),
                Variable::PushConstant(Type::Struct(src_struct_ty)),
            ) => {
                // Merge push constants scattered in different stages. This
                // match must succeed.
                dst_struct_ty.merge(&src_struct_ty)?;
                // It's guaranteed to be interface uniform so we don't have to
                // check the hash.
            },
            (
                Variable::Input(_, dst_ty),
                Variable::Input(_, src_ty),
            ) => {
                if hash(dst_ty) != hash(src_ty) {
                    return Err(Error::MismatchedManifest);
                }
            },
            (
                Variable::Output(_, dst_ty),
                Variable::Output(_, src_ty),
            ) => {
                if hash(dst_ty) != hash(src_ty) {
                    return Err(Error::MismatchedManifest);
                }
            },
            (
                Variable::Descriptor(_, dst_desc_ty, dst_access),
                Variable::Descriptor(_, src_desc_ty, src_access),
            ) => {
                // Just regular descriptor types. Simply match the hashes.
                if hash(dst_desc_ty) != hash(src_desc_ty) || dst_access != src_access {
                    return Err(Error::MismatchedManifest);
                }
            },
            _ => unreachable!(),
        }
        Ok(())
    }
    fn merge_vars(&mut self, other: &Manifest) -> Result<()> {
        use std::collections::hash_map::Entry;
        for src_var in other.var_map.values() {
            let locator = src_var.locator();
            match self.var_map.entry(locator) {
                Entry::Vacant(entry) => {
                    // Doesn't have a push constant in this manifest, so we
                    // directly copy it.
                    entry.insert(src_var.clone());
                },
                Entry::Occupied(mut entry) => {
                    Self::merge_var(entry.get_mut(), src_var)?;
                },
            }
        }
        Ok(())
    }
    /// Merge metadata records in another manifest into the current one. If the
    /// type bound to a interface location, a descriptor binding point or an
    /// offset position in push constant block mismatches, the merge will fail
    /// and the `self` manifest will be corrupted.
    pub fn merge(&mut self, other: &Manifest) -> Result<()> {
        self.merge_vars(other)?;
        self.merge_names(other)?;
        Ok(())
    }
    /// Similar to `merge` but optionally the current input interface variables
    /// can be kept alive, and the output interface variables can be cleared and
    /// replaced with entries in `other`; and names are all discarded. This can
    /// be used to merge pipeline stages.
    pub fn merge_pipe(
        &mut self,
        other: &Manifest,
        replace_in: bool,
        replace_out: bool,
    ) -> Result<()> {
        if replace_in {
            self.clear_inputs();
        }
        if replace_out {
            self.clear_outputs();
        }
        self.merge_vars(other)?;
        Ok(())
    }
    pub fn get_var<'a>(&'a self, locator: Locator) -> Option<&'a Variable> {
        self.var_map.get(&locator)
    }
    /// Get the push constant type.
    pub fn get_push_const<'a>(&'a self) -> Option<&'a Type> {
        self.get_var(Locator::PushConstant)
            .map(|x| {
                if let Variable::PushConstant(ty) = x {
                    ty
                } else {
                    unreachable!();
                }
            })
    }
    /// Get the input interface variable type.
    pub fn get_input<'a>(&'a self, location: InterfaceLocation) -> Option<&'a Type> {
        self.get_var(Locator::Input(location))
            .map(|x| {
                if let Variable::Input(_, ty) = x {
                    ty
                } else {
                    unreachable!();
                }
            })
    }
    /// Get the output interface variable type.
    pub fn get_output<'a>(&'a self, location: InterfaceLocation) -> Option<&'a Type> {
        self.get_var(Locator::Output(location))
            .map(|x| {
                if let Variable::Output(_, ty) = x {
                    ty
                } else {
                    unreachable!();
                }
            })
    }
    /// Get the descriptor type at the given descriptor binding point.
    pub fn get_desc<'a>(&'a self, desc_bind: DescriptorBinding) -> Option<&'a DescriptorType> {
        self.get_var(Locator::Descriptor(desc_bind))
            .map(|x| {
                if let Variable::Descriptor(_, desc_ty, _) = x {
                    desc_ty
                } else {
                    unreachable!();
                }
            })
    }
    pub fn get_var_name<'a>(&'a self, locator: Locator) -> Option<&'a str> {
        self.var_name_map.iter()
            .find_map(|x| {
                if *x.1 == locator {
                    Some(x.0.as_ref())
                } else {
                    None
                }
            })
    }
    /// Get the name that also refers to the push constant block.
    pub fn get_push_const_name<'a>(&'a self) -> Option<&'a str> {
        self.get_var_name(Locator::PushConstant)
    }
    /// Get the name that also refers to the input at the given location.
    pub fn get_input_name<'a>(&'a self, location: InterfaceLocation) -> Option<&'a str> {
        self.get_var_name(Locator::Input(location))
    }
    /// Get the name that also refers to the output at the given location.
    pub fn get_output_name<'a>(&'a self, location: InterfaceLocation) -> Option<&'a str> {
        self.get_var_name(Locator::Output(location))
    }
    /// Get the name that also refers to the descriptor at the given descriptor
    /// binding.
    pub fn get_desc_name<'a>(&'a self, desc_bind: DescriptorBinding) -> Option<&'a str> {
        self.get_var_name(Locator::Descriptor(desc_bind))
    }
    /// Get the valid access patterns of the descriptor at the given binding
    /// point. Currently only storage buffers and storage images can be accessed
    /// by write.
    ///
    /// Note that the returned access type is the nominal access type declared
    /// in SPIR-V. If a storage image is declared as `ReadWrite` but is only
    /// accessed by write, it is still considered a `ReadWrite` descriptor.
    pub fn get_desc_access(&self, desc_bind: DescriptorBinding) -> Option<AccessType> {
        self.get_var(Locator::Descriptor(desc_bind))
            .map(|x| {
                if let Variable::Descriptor(_, _, access) = x {
                    *access
                } else {
                    unreachable!();
                }
            })
    }
    fn resolve_ivar<'a>(&'a self, sym: &Sym, kind: ResolveKind) -> Option<InterfaceVariableResolution<'a>> {
        let mut segs = sym.segs();
        let location = match segs.next() {
            Some(Seg::Index(loc)) => {
                if let Some(Seg::Index(comp)) = segs.next() {
                    InterfaceLocation::new(loc as u32, comp as u32)
                } else {
                    // Component must be specified, to be in consistent with
                    // descriptor resolution. 
                    return None;
                }
            },
            Some(Seg::Name(name)) => {
                match self.var_name_map.get(name) {
                    Some(Locator::Input(location)) if kind == ResolveKind::Input => *location,
                    Some(Locator::Output(location)) if kind == ResolveKind::Output => *location,
                    _ => return None,
                }
            },
            _ => return None,
        };
        let ty = match kind {
            ResolveKind::Input => self.get_input(location)?,
            ResolveKind::Output => self.get_output(location)?
        };
        let ivar_res = InterfaceVariableResolution { location, ty };
        if segs.next().is_some() { return None }
        Some(ivar_res)
    }
    /// Get the metadata of a input variable identified by a symbol.
    pub fn resolve_input<S: AsRef<Sym>>(&self, sym: S) -> Option<InterfaceVariableResolution> {
        self.resolve_ivar(sym.as_ref(), ResolveKind::Input)
    }
    /// Get the metadata of a output variable identified by a symbol.
    pub fn resolve_output<S: AsRef<Sym>>(&self, sym: S) -> Option<InterfaceVariableResolution> {
        self.resolve_ivar(sym.as_ref(), ResolveKind::Output)
    }
    /// Get the metadata of a descriptor variable identified by a symbol.
    /// If the exact variable cannot be resolved, the descriptor part of the
    /// resolution will still be returned, if possible.
    pub fn resolve_desc<S: AsRef<Sym>>(&self, sym: S) -> Option<DescriptorResolution> {
        let mut segs = sym.as_ref().segs();
        let desc_bind = match segs.next() {
            Some(Seg::Index(desc_set)) => {
                if let Some(Seg::Index(bind_point)) = segs.next() {
                    DescriptorBinding::new(desc_set as u32, bind_point as u32)
                } else {
                    // Binding point must be specified so we know there are
                    // always two segments leading structure field segments.
                    return None;
                }
            },
            Some(Seg::Name(name)) => {
                if let Some(Locator::Descriptor(desc_bind)) = self.var_name_map.get(name) {
                    *desc_bind
                } else {
                    return None;
                }
            },
            _ => return None,
        };
        let desc_ty = self.get_desc(desc_bind)?;
        let rem_sym = segs.remaining();
        let member_var_res = desc_ty.resolve(rem_sym);
        let desc_res = DescriptorResolution { desc_bind, desc_ty, member_var_res };
        Some(desc_res)
    }
    /// Get the metadata of a descriptor variable identified by a symbol. If the
    /// exact variable cannot be resolved, the descriptor part of the resolution
    /// will still be returned, if possible.
    pub fn resolve_push_const<S: AsRef<Sym>>(&self, sym: S) -> Option<PushConstantResolution> {
        let mut segs = sym.as_ref().segs();
        match segs.next() {
            Some(Seg::Empty) => {
                // Symbols started with an empty head, like ".modelView", is
                // used to identify push constants.
            },
            Some(Seg::Name(name)) => {
                if let Some(Locator::PushConstant) = self.var_name_map.get(name) {
                } else { return None; }
            },
            _ => return None,
        };
        let ty = self.get_push_const()?;
        let rem_sym = segs.remaining();
        let member_var_res = ty.resolve(rem_sym);
        let push_const_res = PushConstantResolution { ty, member_var_res };
        Some(push_const_res)
    }

    // List all variables.
    pub fn vars<'a>(&'a self) -> impl Iterator<Item=&'a Variable> {
        self.var_map.values()
    }
    /// List all input locations.
    pub fn inputs<'a>(&'a self) -> impl Iterator<Item=InterfaceVariableResolution<'a>> {
        self.vars()
            .filter_map(|x| {
                if let Variable::Input(location, ty) = x {
                    let ivar_res = InterfaceVariableResolution {
                        location: *location,
                        ty,
                    };
                    Some(ivar_res)
                } else {
                    None
                }
            })
    }
    /// List all output locations in this manifest.
    pub fn outputs<'a>(&'a self) -> impl Iterator<Item=InterfaceVariableResolution<'a>> {
        self.vars()
            .filter_map(|x| {
                if let Variable::Output(location, ty) = x {
                    let ivar_res = InterfaceVariableResolution {
                        location: *location,
                        ty,
                    };
                    Some(ivar_res)
                } else {
                    None
                }
            })
    }
    /// List all descriptors in this manifest. In case of a descriptor pointing
    /// to a buffer block, the outermost structure type will be filled in
    /// `member_var_res`.
    pub fn descs<'a>(&'a self) -> impl Iterator<Item=DescriptorResolution<'a>> {
        self.vars()
            .filter_map(|x| {
                if let Variable::Descriptor(desc_bind, desc_ty, _) = x {
                    let ivar_res = DescriptorResolution {
                        desc_bind: *desc_bind,
                        desc_ty,
                        member_var_res: desc_ty.resolve(""),
                    };
                    Some(ivar_res)
                } else {
                    None
                }
            })
    }

    fn insert_var_name(&mut self, name: &str, locator: Locator) -> Result<()> {
        if self.var_name_map.insert(name.to_owned(), locator).is_some() {
            Err(Error::NAME_COLLISION)
        } else { Ok(()) }
    }
    fn insert_var(&mut self, var: Variable, name: Option<&str>) -> Result<()> {
        use std::collections::hash_map::{Entry};
        let locator = var.locator();
        match &var {
            Variable::Input(_, _) => {
                // Input variables can share locations (aliasing).
                match self.var_map.entry(locator) {
                    Entry::Occupied(_) => return Err(Error::LOCATION_COLLISION),
                    Entry::Vacant(entry) => { entry.insert(var); },
                }
            },
            Variable::Output(_, _) => {
                // Output variables can share locations (aliasing).
                match self.var_map.entry(locator) {
                    Entry::Occupied(_) => return Err(Error::LOCATION_COLLISION),
                    Entry::Vacant(entry) => { entry.insert(var); },
                }
            },
            Variable::Descriptor(desc_bind, desc_ty, access) => {
                // Descriptors cannot share bindings, but separate image and
                // sampler can be fused implicitly into a
                // `CombinedImageSampler` by sharing bindings.
                match self.var_map.entry(locator) {
                    Entry::Occupied(mut entry) => {
                        if let Variable::Descriptor(_, cur_desc_ty, _) = entry.get() {
                            let replace_desc_ty = match (cur_desc_ty, &desc_ty) {
                                (DescriptorType::Sampler(nbind_samp), DescriptorType::Image(nbind_img, img_ty)) => {
                                    if let Type::Image(img_ty) = img_ty {
                                        if nbind_samp != nbind_img {
                                            return Err(Error::SAMPLER_IMG_NBIND_MISMATCH);
                                        }
                                        let sampled_img_ty = ty::SampledImageType::new(img_ty.clone());
                                        DescriptorType::SampledImage(*nbind_samp, Type::SampledImage(sampled_img_ty))
                                    } else {
                                        unreachable!();
                                    }
                                },
                                (DescriptorType::Image(nbind_img, img_ty), DescriptorType::Sampler(nbind_samp)) => {
                                    if let Type::Image(img_ty) = img_ty {
                                        if nbind_samp != nbind_img {
                                            return Err(Error::SAMPLER_IMG_NBIND_MISMATCH);
                                        }
                                        let sampled_img_ty = ty::SampledImageType::new(img_ty.clone());
                                        DescriptorType::SampledImage(*nbind_samp, Type::SampledImage(sampled_img_ty))
                                    } else {
                                        unreachable!();
                                    }
                                },
                                _ => return Err(Error::DESC_BIND_COLLISION),
                            };
                            let var = Variable::Descriptor(*desc_bind, replace_desc_ty, *access);
                            entry.insert(var);
                        }
                    },
                    Entry::Vacant(entry) => { entry.insert(var); },
                }
            },
            Variable::PushConstant(_) => {
                match self.var_map.entry(locator) {
                    Entry::Occupied(_) => return Err(Error::MULTI_PUSH_CONST),
                    Entry::Vacant(entry) => { entry.insert(var); },
                }
            },
        }
        if let Some(name) = name {
            self.insert_var_name(name, locator)?;
        }
        Ok(())
    }
}

/// Entry point specialization descriptions.
#[derive(Default, Clone)]
pub struct Specialization {
    /// Mapping from specialization constant names to their IDs.
    spec_const_name_map: HashMap<String, SpecId>,
    /// Mapping from specialization IDs to specialization constant types.
    spec_const_map: HashMap<SpecId, Type>,
}
impl Specialization {
    pub fn resolve_spec_const<S: AsRef<Sym>>(&self, sym: S) -> Option<SpecConstantResolution> {
        let mut segs = sym.as_ref().segs();
        let spec_id = if let Some(Seg::Name(name)) = segs.next() {
            if let Some(spec_id) = self.spec_const_name_map.get(name) {
                *spec_id
            } else { return None }
        } else { return None };
        if segs.next().is_some() { return None }
        let ty = self.get_spec_const(spec_id)?;
        let spec_res = SpecConstantResolution { spec_id, ty };
        Some(spec_res)
    }
    /// Get the name that also refers to the specialization constant.
    pub fn get_spec_const_name<'a>(&'a self, spec_id: SpecId) -> Option<&'a str> {
        self.spec_const_name_map.iter()
            .find_map(|x| if spec_id == *x.1 { Some(x.0.as_ref()) } else { None })
    }
    /// Get the specialization constant type.
    pub fn get_spec_const<'a>(&'a self, spec_id: SpecId) -> Option<&'a Type> {
        self.spec_const_map.get(&spec_id)
    }
    /// List all specialization constants.
    pub fn spec_consts<'a>(&'a self) -> impl Iterator<Item=SpecConstantResolution<'a>> {
        self.spec_const_map.iter()
            .map(|(&spec_id, ty)| {
                SpecConstantResolution {
                    spec_id,
                    ty
                }
            })
    }

    fn insert_spec_const(
        &mut self,
        spec_id: SpecId,
        ty: Type,
        name: Option<&str>
    ) -> Result<()> {
        if self.spec_const_map.insert(spec_id, ty).is_some() {
            return Err(Error::SPEC_ID_COLLISION);
        }
        if let Some(name) = name {
            if self.spec_const_name_map.insert(name.to_owned(), spec_id).is_some() {
                return Err(Error::NAME_COLLISION);
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WorkgroupSize {
    x: u32,
    y: u32,
    z: u32
}
impl Default for WorkgroupSize {
    fn default() -> Self {
        Self {
            x: 1,
            y: 1,
            z: 1
        }
    }
}


// SPIR-V program entry points.

/// Representing an entry point described in a SPIR-V.
#[derive(Clone)]
pub struct EntryPoint {
    /// Entry point execution model.
    pub exec_model: ExecutionModel,
    /// Name of the entry point.
    pub name: String,
    /// Manifest object that contains input, output and descriptor type
    /// information.
    pub manifest: Manifest,
    /// Specialization description of the entry point.
    pub spec: Specialization,
    /// Compute shader workgroup size (if applicable).
    pub workgroup_size: Option<WorkgroupSize>,
}
impl Deref for EntryPoint {
    type Target = Manifest;
    fn deref(&self) -> &Self::Target { &self.manifest }
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
        let push_const_ty = self.manifest.get_push_const();
        let mut inputs = self.manifest.inputs()
            .map(|x| (x.location, x.ty))
            .collect::<Vec<_>>();
        inputs.sort_by_key(|x| x.0);
        let mut outputs = self.manifest.outputs()
            .map(|x| (x.location, x.ty))
            .collect::<Vec<_>>();
        outputs.sort_by_key(|x| x.0);
        let mut descs = self.manifest.descs()
            .map(|x| (x.desc_bind, x.desc_ty))
            .collect::<Vec<_>>();
        descs.sort_by_key(|x| x.0);
        f.debug_struct(&self.name)
            .field("push_const", &push_const_ty)
            .field("inputs", &InterfaceLocationDebugHelper(inputs))
            .field("outputs", &InterfaceLocationDebugHelper(outputs))
            .field("descriptors", &DescriptorBindingDebugHelper(descs))
            .field("spec_consts", &self.spec.spec_const_map)
            .field("workgroup_size", &self.workgroup_size)
            .finish()
    }
}
