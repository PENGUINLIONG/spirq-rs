use fnv::FnvHashMap as HashMap;

use crate::{
    constant::ConstantValue,
    entry_point::EntryPoint,
    error::Result,
    inspect::{FnInspector, Inspector},
    locator::SpecId,
    parse::{Instr, SpirvBinary},
    reflect::{reflect, FunctionInspector, ReflectIntermediate},
};

/// Reflection configuration builder.
#[derive(Default, Clone)]
pub struct ReflectConfig {
    pub(crate) spv: SpirvBinary,
    pub(crate) ref_all_rscs: bool,
    pub(crate) combine_img_samplers: bool,
    pub(crate) gen_unique_names: bool,
    pub(crate) spec_values: HashMap<SpecId, ConstantValue>,
}
impl ReflectConfig {
    pub fn new() -> Self {
        Default::default()
    }

    /// SPIR-V binary to be reflected.
    pub fn spv<Spv: Into<SpirvBinary>>(&mut self, x: Spv) -> &mut Self {
        self.spv = x.into();
        self
    }
    /// Reference all defined resources even the resource is not used by an
    /// entry point. Otherwise and by default, only the referenced resources are
    /// assigned to entry points.
    ///
    /// Can be faster for modules with only entry point; slower for multiple
    /// entry points.
    pub fn ref_all_rscs(&mut self, x: bool) -> &mut Self {
        self.ref_all_rscs = x;
        self
    }
    /// Combine images and samplers sharing a same binding point to combined
    /// image sampler descriptors.
    ///
    /// Faster when disabled, but useful for modules derived from HLSL.
    pub fn combine_img_samplers(&mut self, x: bool) -> &mut Self {
        self.combine_img_samplers = x;
        self
    }
    /// Generate unique names for types and struct fields to help further
    /// processing of the reflection data. Otherwise, the debug names are
    /// assigned.
    pub fn gen_unique_names(&mut self, x: bool) -> &mut Self {
        self.gen_unique_names = x;
        self
    }
    /// Use the provided value for specialization constant at `spec_id`.
    pub fn specialize(&mut self, spec_id: SpecId, value: ConstantValue) -> &mut Self {
        self.spec_values.insert(spec_id, value);
        self
    }

    /// Reflect the SPIR-V binary and extract all entry points.
    pub fn reflect(&self) -> Result<Vec<EntryPoint>> {
        let mut itm = ReflectIntermediate::new(self);
        let inspector = FunctionInspector::new();
        reflect(&mut itm, inspector)
    }
    /// Reflect the SPIR-V binary and extract all entry points with an inspector
    /// for customized reflection subroutines.
    pub fn reflect_inspect<I: Inspector>(&self, inspector: &mut I) -> Result<Vec<EntryPoint>> {
        let mut itm = ReflectIntermediate::new(self);
        let mut func_inspector = FunctionInspector::new();
        reflect(&mut itm, func_inspector.chain(inspector))
    }
    /// Reflect the SPIR-V binary and extract all entry points with an inspector
    /// function for customized reflection subroutines.
    pub fn reflect_inspect_by<F: FnMut(&mut ReflectIntermediate<'_>, &Instr)>(
        &self,
        inspector: F,
    ) -> Result<Vec<EntryPoint>> {
        let mut inspector = FnInspector::<F>(inspector);
        self.reflect_inspect(&mut inspector)
    }
}
