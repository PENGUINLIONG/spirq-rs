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
use num_derive::FromPrimitive;
use fnv::FnvHashMap as HashMap;
use nohash_hasher::IntMap;
use reflect::ReflectIntermediate;
use inspect::{NopInspector, FnInspector};

use parse::{Instrs, Instr};
pub use ty::{Type, DescriptorType};
pub use sym::*;
pub use error::*;
pub use spirv_headers::ExecutionModel;

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
    fn from_iter<I: IntoIterator<Item=u32>>(iter: I) -> Self { SpirvBinary(iter.into_iter().collect::<Vec<u32>>()) }
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
        reflect::reflect_spirv(&self, NopInspector())
    }
    /// Similar to `reflect_vec` while you can inspect each instruction during
    /// the parse.
    pub fn reflect_vec_inspect<F: FnMut(&ReflectIntermediate<'_>, &Instr<'_>)>(&self, inspector: F) -> Result<Vec<EntryPoint>> {
        reflect::reflect_spirv(&self, FnInspector::<F>(inspector))
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


// Resource locationing.

type SpecId = u32;

/// Interface variable location and component.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
pub struct InterfaceLocation(u32, u32);
impl InterfaceLocation {
    pub fn new(loc: u32, comp: u32) -> Self { InterfaceLocation(loc, comp) }

    pub fn loc(&self) -> u32 { self.0 }
    pub fn comp(&self) -> u32 { self.1 }
    pub fn into_inner(self) -> (u32, u32) { (self.0, self.1) }
}
impl fmt::Display for InterfaceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(loc={}, comp={})", self.0, self.1)
    }
}
impl fmt::Debug for InterfaceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { (self as &dyn fmt::Display).fmt(f) }
}
impl From<InterfaceLocation> for InterfaceLocationCode {
    fn from(x: InterfaceLocation) -> InterfaceLocationCode {
        ((x.0 as u64) << 32) | (x.1 as u64)
    }
}
impl From<InterfaceLocationCode> for InterfaceLocation {
    fn from(x: InterfaceLocationCode) -> InterfaceLocation {
        InterfaceLocation((x >> 32) as u32, (x & 0xFFFFFFFF) as u32)
    }
}
type InterfaceLocationCode = u64;

/// Descriptor set and binding point carrier.
#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Default, Clone, Copy)]
pub struct DescriptorBinding(u32, u32);
impl DescriptorBinding {
    pub fn new(desc_set: u32, bind_point: u32) -> Self { DescriptorBinding(desc_set, bind_point) }

    pub fn set(&self) -> u32 { self.0 }
    pub fn bind(&self) -> u32 { self.1 }
    pub fn into_inner(self) -> (u32, u32) { (self.0, self.1) }
}
impl fmt::Display for DescriptorBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "(set={}, bind={})", self.0, self.1)
    }
}
impl fmt::Debug for DescriptorBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { (self as &dyn fmt::Display).fmt(f) }
}
impl From<DescriptorBinding> for DescriptorBindingCode {
    fn from(x: DescriptorBinding) -> DescriptorBindingCode {
        ((x.0 as u64) << 32) | (x.1 as u64)
    }
}
impl From<DescriptorBindingCode> for DescriptorBinding {
    fn from(x: DescriptorBindingCode) -> DescriptorBinding {
        DescriptorBinding((x >> 32) as u32, (x & 0xFFFFFFFF) as u32)
    }
}
type DescriptorBindingCode = u64;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub(crate) enum ResourceLocator {
    Input(InterfaceLocation),
    Output(InterfaceLocation),
    Descriptor(DescriptorBinding),
    PushConstant,
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

/// Access type of a variable.
#[repr(u32)]
#[derive(Debug, FromPrimitive, Clone, Copy, PartialEq, Eq)]
pub enum AccessType {
    /// The variable can be accessed by read.
    ReadOnly = 1,
    /// The variable can be accessed by write.
    WriteOnly = 2,
    /// The variable can be accessed by read or by write.
    ReadWrite = 3,
}
impl std::ops::BitOr<AccessType> for AccessType {
    type Output = AccessType;
    fn bitor(self, rhs: AccessType) -> AccessType {
        use num_traits::FromPrimitive;
        AccessType::from_u32((self as u32) | (rhs as u32)).unwrap()
    }
}
impl std::ops::BitAnd<AccessType> for AccessType {
    type Output = AccessType;
    fn bitand(self, rhs: AccessType) -> AccessType {
        use num_traits::FromPrimitive;
        AccessType::from_u32((self as u32) & (rhs as u32)).unwrap()
    }
}

/// A set of information used to describe variable typing and routing.
#[derive(Default, Clone)]
pub struct Manifest {
    push_const_ty: Option<Type>,
    input_map: IntMap<InterfaceLocationCode, Type>,
    output_map: IntMap<InterfaceLocationCode, Type>,
    desc_map: IntMap<DescriptorBindingCode, DescriptorType>,
    var_name_map: HashMap<String, ResourceLocator>,
    desc_access_map: IntMap<DescriptorBindingCode, AccessType>
}
impl Manifest {
    fn merge_ivars(
        self_ivar_map: &mut IntMap<InterfaceLocationCode, Type>,
        other_ivar_map: &IntMap<InterfaceLocationCode, Type>,
    ) -> Result<()> {
        use std::collections::hash_map::Entry::{Vacant, Occupied};
        for (location, ty) in other_ivar_map.iter() {
            match self_ivar_map.entry(*location) {
                Vacant(entry) => { entry.insert(ty.clone()); },
                Occupied(entry) => if hash(entry.get()) != hash(ty) {
                    return Err(Error::MismatchedManifest);
                }
            }
        }
        Ok(())
    }
    fn merge_push_const(&mut self, other: &Manifest) -> Result<()> {
        if let Some(Type::Struct(dst_struct_ty)) = self.push_const_ty.as_mut() {
            // Merge push constants scattered in different stages. This match
            // must success.
            if let Some(Type::Struct(src_struct_ty)) = other.push_const_ty.as_ref() {
                dst_struct_ty.merge(&src_struct_ty)?;
            }
            // It's guaranteed to be interface uniform so we don't have to check
            // the hash.
        } else {
            self.push_const_ty = other.push_const_ty.clone();
        }
        Ok(())
    }
    fn merge_descs(&mut self, other: &Manifest) -> Result<()> {
        use std::collections::hash_map::Entry::{Vacant, Occupied};
        for (desc_bind, desc_ty) in other.desc_map.iter() {
            match self.desc_map.entry(*desc_bind) {
                Vacant(entry) => { entry.insert(desc_ty.clone()); },
                Occupied(entry) => {
                    // Just regular descriptor types. Simply match the hashes.
                    if hash(entry.get()) != hash(&desc_ty) {
                        return Err(Error::MismatchedManifest);
                    }
                }
            }
        }
        Ok(())
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
    fn merge_accesses(&mut self, other: &Manifest) -> Result<()> {
        for (desc_bind, access) in other.desc_access_map.iter() {
            if let Some(acc) = self.desc_access_map.get_mut(&desc_bind) {
                use num_traits::FromPrimitive;
                let access = *acc as u32 | *access as u32;
                *acc = AccessType::from_u32(access).unwrap();
            } else {
                self.desc_access_map.insert(*desc_bind, *access);
            }
        }
        Ok(())
    }
    /// Merge metadata records in another manifest into the current one. If the
    /// type bound to a interface location, a descriptor binding point or an
    /// offset position in push constant block mismatches, the merge will fail
    /// and the `self` manifest will be corrupted.
    pub fn merge(&mut self, other: &Manifest) -> Result<()> {
        Self::merge_ivars(&mut self.input_map, &other.input_map)?;
        Self::merge_ivars(&mut self.output_map, &other.output_map)?;
        self.merge_push_const(other)?;
        self.merge_descs(other)?;
        self.merge_names(other)?;
        self.merge_accesses(other)?;
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
            self.input_map.clear();
            self.input_map.extend(
                other.input_map.iter()
                    .map(|(x, y)| (*x, y.clone()))
            );
        }
        if replace_out {
            self.output_map.clear();
            self.output_map.extend(
                other.output_map.iter()
                    .map(|(x, y)| (*x, y.clone()))
            );
        }
        self.merge_push_const(other)?;
        self.merge_descs(other)?;
        self.merge_accesses(other)?;
        Ok(())
    }
    /// Get the push constant type.
    pub fn get_push_const<'a>(&'a self) -> Option<&'a Type> {
        self.push_const_ty.as_ref()
    }
    /// Get the input interface variable type.
    pub fn get_input<'a>(&'a self, location: InterfaceLocation) -> Option<&'a Type> {
        self.input_map.get(&location.into())
    }
    /// Get the output interface variable type.
    pub fn get_output<'a>(&'a self, location: InterfaceLocation) -> Option<&'a Type> {
        self.output_map.get(&location.into())
    }
    /// Get the descriptor type at the given descriptor binding point.
    pub fn get_desc<'a>(&'a self, desc_bind: DescriptorBinding) -> Option<&'a DescriptorType> {
        self.desc_map.get(&desc_bind.into())
    }
    /// Get the name that also refers to the push constant block.
    pub fn get_push_const_name<'a>(&'a self) -> Option<&'a str> {
        self.var_name_map.iter()
            .find_map(|x| if let ResourceLocator::PushConstant = x.1 {
                Some(x.0.as_ref())
            } else { None })
    }
    /// Get the name that also refers to the input at the given location.
    pub fn get_input_name<'a>(&'a self, location: InterfaceLocation) -> Option<&'a str> {
        self.var_name_map.iter()
            .find_map(|x| if let ResourceLocator::Input(loc) = x.1 {
                if *loc == location { Some(x.0.as_ref()) } else { None }
            } else { None })
    }
    /// Get the name that also refers to the output at the given location.
    pub fn get_output_name<'a>(&'a self, location: InterfaceLocation) -> Option<&'a str> {
        self.var_name_map.iter()
            .find_map(|x| if let ResourceLocator::Output(loc) = x.1 {
                if *loc == location { Some(x.0.as_ref()) } else { None }
            } else { None })
    }
    /// Get the name that also refers to the descriptor at the given descriptor
    /// binding.
    pub fn get_desc_name<'a>(&'a self, desc_bind: DescriptorBinding) -> Option<&'a str> {
        self.var_name_map.iter()
            .find_map(|x| if let ResourceLocator::Descriptor(db) = x.1 {
                if *db == desc_bind { Some(x.0.as_ref()) } else { None }
            } else { None })
    }
    /// Get the valid access patterns of the descriptor at the given binding
    /// point. Currently only storage buffers and storage images can be accessed
    /// by write.
    ///
    /// Note that the returned access type is the nominal access type declared
    /// in SPIR-V. If a storage image is declared as `ReadWrite` but is only
    /// accessed by write, it is still considered a `ReadWrite` descriptor.
    pub fn get_desc_access(&self, desc_bind: DescriptorBinding) -> Option<AccessType> {
        self.desc_access_map
            .get(&desc_bind.into())
            .map(|x| *x)
    }
    fn resolve_ivar<'a>(&self, map: &'a IntMap<InterfaceLocationCode, Type>, sym: &Sym, kind: ResolveKind) -> Option<InterfaceVariableResolution<'a>> {
        let mut segs = sym.segs();
        let location = match segs.next() {
            Some(Seg::Index(loc)) => {
                if let Some(Seg::Index(comp)) = segs.next() {
                    InterfaceLocation::new(loc as u32, comp as u32)
                } else { return None; }
            },
            Some(Seg::Name(name)) => match self.var_name_map.get(name) {
                Some(ResourceLocator::Input(location)) =>
                    if kind == ResolveKind::Input { *location } else { return None; },

                Some(ResourceLocator::Output(location)) =>
                    if kind == ResolveKind::Output { *location } else { return None; },

                _ => return None,
            },
            _ => return None,
        };
        if segs.next().is_some() { return None }
        let ty = map.get(&location.into())?;
        let ivar_res = InterfaceVariableResolution { location, ty };
        Some(ivar_res)
    }
    /// Get the metadata of a input variable identified by a symbol.
    pub fn resolve_input<S: AsRef<Sym>>(&self, sym: S) -> Option<InterfaceVariableResolution> {
        self.resolve_ivar(&self.input_map, sym.as_ref(), ResolveKind::Input)
    }
    /// Get the metadata of a output variable identified by a symbol.
    pub fn resolve_output<S: AsRef<Sym>>(&self, sym: S) -> Option<InterfaceVariableResolution> {
        self.resolve_ivar(&self.output_map, sym.as_ref(), ResolveKind::Output)
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
                } else { return None; }
            },
            Some(Seg::Name(name)) => {
                if let Some(ResourceLocator::Descriptor(desc_bind)) = self.var_name_map.get(name) {
                    *desc_bind
                } else { return None; }
            },
            _ => return None,
        };
        let desc_ty = self.desc_map.get(&desc_bind.into())?;
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
                if let Some(ResourceLocator::PushConstant) = self.var_name_map.get(name) {
                } else { return None; }
            },
            _ => return None,
        };
        let ty = self.push_const_ty.as_ref()?;
        let rem_sym = segs.remaining();
        let member_var_res = ty.resolve(rem_sym);
        let push_const_res = PushConstantResolution { ty, member_var_res };
        Some(push_const_res)
    }
    /// List all input locations.
    pub fn inputs<'a>(&'a self) -> impl Iterator<Item=InterfaceVariableResolution<'a>> {
        self.input_map.iter()
            .map(|(&location, ty)| {
                InterfaceVariableResolution {
                    location: location.into(),
                    ty
                }
            })
    }
    /// List all output locations in this manifest.
    pub fn outputs<'a>(&'a self) -> impl Iterator<Item=InterfaceVariableResolution<'a>> {
        self.output_map.iter()
            .map(|(&location, ty)|  {
                InterfaceVariableResolution {
                    location: location.into(),
                    ty
                }
            })
    }
    /// List all descriptors in this manifest. In case of a descriptor pointing
    /// to a buffer block, the outermost structure type will be filled in
    /// `member_var_res`.
    pub fn descs<'a>(&'a self) -> impl Iterator<Item=DescriptorResolution<'a>> {
        self.desc_map.iter()
            .map(|(&desc_bind, desc_ty)| {
                DescriptorResolution {
                    desc_bind: desc_bind.into(),
                    desc_ty,
                    member_var_res: desc_ty.resolve(""),
                }
            })
    }

    pub(crate) fn insert_rsc_name(&mut self, name: &str, rsc_locator: ResourceLocator) -> Result<()> {
        if self.var_name_map.insert(name.to_owned(), rsc_locator).is_some() {
            Err(Error::NAME_COLLISION)
        } else { Ok(()) }
    }
    pub(crate) fn insert_input(&mut self, location: InterfaceLocation, ivar_ty: Type) -> Result<()> {
        // Input variables can share locations (aliasing).
        self.input_map.insert(location.into(), ivar_ty);
        Ok(())
    }
    pub(crate) fn insert_output(&mut self, location: InterfaceLocation, ivar_ty: Type) -> Result<()> {
        // Ouput variables can share locations (aliasing).
        self.output_map.insert(location.into(), ivar_ty);
        Ok(())
    }
    pub(crate) fn insert_desc(
        &mut self,
        desc_bind: DescriptorBinding,
        desc_ty: DescriptorType,
        access: AccessType,
    ) -> Result<()> {
        use std::collections::hash_map::Entry::{Vacant, Occupied};
        fn combine_img_sampler(
            nbind_samp: u32,
            nbind_img: u32,
            img_ty: &Type,
            access_samp: AccessType,
            access_img: AccessType,
        ) -> Vec<(DescriptorType, AccessType)> {
            use std::cmp::Ordering;
            match nbind_samp.cmp(&nbind_img) {
                Ordering::Equal => vec![
                    (DescriptorType::SampledImage(nbind_img, img_ty.clone()), access_samp | access_img)
                ],
                Ordering::Less => vec![
                    (DescriptorType::SampledImage(nbind_samp, img_ty.clone()), access_samp | access_img),
                    (DescriptorType::Image(nbind_img - nbind_samp, img_ty.clone()), access_img),
                ],
                Ordering::Greater => vec![
                    (DescriptorType::SampledImage(nbind_img, img_ty.clone()), access_samp | access_img),
                    (DescriptorType::Sampler(nbind_samp - nbind_img), access_samp),
                ],
            }
        }
        // Allow override of resource access...?
        self.desc_access_map.insert(desc_bind.into(), access);
        // Descriptors cannot share bindings, but separate image and
        // sampler can be fused implicitly into a
        // `CombinedImageSampler` by sharing bindings.
        let replaces = match self.desc_map.entry(desc_bind.into()) {
            Vacant(entry) => {
                entry.insert(desc_ty);
                Vec::new()
            },
            Occupied(entry) => {
                let replaces = match (entry.get(), &desc_ty) {
                    (DescriptorType::Sampler(nbind_samp), DescriptorType::Image(nbind_img, img_ty)) => {
                        let access_samp = self.desc_access_map[&desc_bind.into()];
                        combine_img_sampler(*nbind_samp, *nbind_img, &img_ty, access_samp, access)
                    },
                    (DescriptorType::Image(nbind_img, img_ty), DescriptorType::Sampler(nbind_samp)) => {
                        let access_img = self.desc_access_map[&desc_bind.into()];
                        combine_img_sampler(*nbind_samp, *nbind_img, &img_ty, access, access_img)
                    },
                    _ => return Err(Error::DESC_BIND_COLLISION),
                };
                entry.remove();
                replaces
            },
        };
        // Insert replace items back to the manifest.
        let mut replace_bind = desc_bind.bind();
        // `replaces`'s base binding MUST BE monotonically increamental.
        for (desc_ty, access) in replaces {
            let nbind = desc_ty.nbind();
            self.insert_desc(DescriptorBinding(desc_bind.set(), replace_bind), desc_ty, access)?;
            replace_bind += nbind;
        }
        Ok(())
    }
    pub(crate) fn insert_push_const(&mut self, push_const_ty: Type) -> Result<()> {
        if self.push_const_ty.is_none() {
            self.push_const_ty = Some(push_const_ty);
            Ok(())
        } else { Err(Error::MULTI_PUSH_CONST) }
    }
}

/// Entry point specialization descriptions.
#[derive(Default, Clone)]
pub struct Specialization {
    /// Mapping from specialization constant names to their IDs.
    spec_const_name_map: HashMap<String, SpecId>,
    /// Mapping from specialization IDs to specialization constant types.
    spec_const_map: IntMap<SpecId, Type>,
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

    // TODO: (penguinliong) This should not be exposed. Hide it in next larger
    // release.
    pub fn insert_spec_const(&mut self, spec_id: SpecId, ty: Type) -> Result<()> {
        if self.spec_const_map.insert(spec_id, ty).is_some() {
            Err(Error::SPEC_ID_COLLISION)
        } else { Ok(()) }
    }
    // TODO: (penguinliong) This should not be exposed. Hide it in next larger
    // release.
    pub fn insert_spec_const_name(&mut self, name: &str, spec_id: SpecId) -> Result<()>{
        if self.spec_const_name_map.insert(name.to_owned(), spec_id).is_some() {
            Err(Error::NAME_COLLISION)
        } else { Ok(()) }
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
}
impl Deref for EntryPoint {
    type Target = Manifest;
    fn deref(&self) -> &Self::Target { &self.manifest }
}
impl fmt::Debug for EntryPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct InterfaceLocationDebugHelper<'a>(&'a IntMap<InterfaceLocationCode, Type>);
        impl<'a> fmt::Debug for InterfaceLocationDebugHelper<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_map()
                    .entries(self.0.iter().map(|(k, v)| (InterfaceLocation::from(*k as InterfaceLocationCode), v)))
                    .finish()
            }
        }
        struct DescriptorBindingDebugHelper<'a>(&'a IntMap<DescriptorBindingCode, DescriptorType>);
        impl<'a> fmt::Debug for DescriptorBindingDebugHelper<'a> {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_map()
                    .entries(self.0.iter().map(|(k, v)| (DescriptorBinding::from(*k as DescriptorBindingCode), v)))
                    .finish()
            }
        }
        f.debug_struct(&self.name)
            .field("push_const", &self.manifest.push_const_ty)
            .field("inputs", &InterfaceLocationDebugHelper(&self.manifest.input_map))
            .field("outputs", &InterfaceLocationDebugHelper(&self.manifest.output_map))
            .field("descriptors", &DescriptorBindingDebugHelper(&self.manifest.desc_map))
            .field("spec_consts", &self.spec.spec_const_map)
            .finish()
    }
}
