//! Reflection procedures and types.
use std::convert::{TryFrom};
use std::iter::Peekable;
use std::ops::RangeInclusive;
use std::fmt;
use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use crate::ty::*;
use crate::consts::*;
use crate::{EntryPoint, Specialization};
use crate::parse::{Instrs, Instr};
use crate::error::{Error, Result};
use crate::instr::*;
use crate::inspect::Inspector;

use spirv_headers::Dim;
pub use spirv_headers::{ExecutionModel, Decoration, StorageClass};

// Public types.

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

pub type SpecId = u32;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Locator {
    Input(InterfaceLocation),
    Output(InterfaceLocation),
    Descriptor(DescriptorBinding),
    PushConstant,
    SpecConstant(SpecId),
}


// Intermediate types used in reflection.

#[derive(Debug, Clone)]
pub struct SpecConstant<'a> {
    /// Type of specialization constant.
    pub ty_id: TypeId,
    /// Default value of specialization constant.
    pub value: &'a [u32],
    /// Specialization constant ID, notice that this is NOT an instruction ID.
    /// It is used to identify specialization constants for graphics libraries.
    pub spec_id: SpecId,
}
#[derive(Debug, Clone)]
pub struct Constant<'a> {
    /// Type of constant.
    pub ty_id: InstrId,
    /// Defined value of constant.
    pub value: &'a [u32],
}

/// Descriptor type matching `VkDescriptorType`.
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum DescriptorType {
    /// `VK_DESCRIPTOR_TYPE_SAMPLER`
    Sampler(),
    /// `VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER`
    CombinedImageSampler(),
    /// `VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE`
    SampledImage(),
    /// `VK_DESCRIPTOR_TYPE_STORAGE_IMAGE`
    StorageImage(AccessType),
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER`.
    UniformTexelBuffer(),
    /// `VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER`.
    StorageTexelBuffer(AccessType),
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER` or
    /// `VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC` depending on how you gonna
    /// use it.
    UniformBuffer(),
    /// `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER` or
    /// `VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC` depending on how you gonna
    /// use it.
    StorageBuffer(AccessType),
    /// `VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT` and its input attachment index.
    InputAttachment(u32),
    /// `VK_DESCRIPTOR_TYPE_ACCELERATION_STRUCTURE_KHR`
    AccelStruct(),
}

#[derive(Debug, Clone)]
pub enum Variable {
    /// Input interface variable.
    Input {
        name: Option<String>,
        // Interface location of input.
        location: InterfaceLocation,
        /// The concrete SPIR-V type definition of descriptor resource.
        ty: Type,
    },
    /// Output interface variable.
    Output {
        name: Option<String>,
        // Interface location of output.
        location: InterfaceLocation,
        /// The concrete SPIR-V type definition of descriptor resource.
        ty: Type,
    },
    /// Descriptor resource.
    Descriptor {
        name: Option<String>,
        desc_bind: DescriptorBinding,
        /// Descriptor resource type matching `VkDescriptorType`.
        desc_ty: DescriptorType,
        /// The concrete SPIR-V type definition of descriptor resource.
        ty: Type,
        /// Number of bindings at the binding point. All descriptors can have
        /// multiple binding points. If the multi-binding is dynamic, 0 will be
        /// returned.
        ///
        /// For more information about dynamic multi-binding, please refer to
        /// Vulkan extension `VK_EXT_descriptor_indexing`, GLSL extension
        /// `GL_EXT_nonuniform_qualifier` and SPIR-V extension
        /// `SPV_EXT_descriptor_indexing`. Dynamic multi-binding is only supported
        /// in Vulkan 1.2.
        nbind: u32,
    },
    /// Push constant.
    PushConstant {
        name: Option<String>,
        /// The concrete SPIR-V type definition of descriptor resource.
        ty: Type,
    },
}
impl Variable {
    pub fn locator(&self) -> Locator {
        match self {
            Variable::Input { location, .. } => Locator::Input(*location),
            Variable::Output { location, .. } => Locator::Output(*location),
            Variable::Descriptor { desc_bind, .. } => Locator::Descriptor(*desc_bind),
            Variable::PushConstant { .. } => Locator::PushConstant,
        }
    }
    pub fn ty(&self) -> &Type {
        match self {
            Variable::Input { ty, .. } => ty,
            Variable::Output { ty, .. } => ty,
            Variable::Descriptor { ty, .. } => ty,
            Variable::PushConstant { ty, .. } => ty,
        }
    }
    pub fn nbind(&self) -> Option<u32> {
        if let Variable::Descriptor { nbind, .. } = self { Some(*nbind) } else { None }
    }
    pub fn walk<'a>(&'a self) -> Walk<'a> {
        Walk::new(self.ty())
    }
}
#[derive(Default, Debug, Clone)]
pub struct Function {
    pub accessed_vars: HashSet<VariableId>,
    pub callees: HashSet<InstrId>,
}
pub struct EntryPointDeclartion<'a> {
    pub func_id: FunctionId,
    pub name: &'a str,
    pub exec_model: ExecutionModel,
}
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ExecutionMode {
    /// Number of times to invoke the geometry stage for each input primitive received.
    /// The default is to run once for each input primitive. It is invalid to specify
    /// a value greater than the target-dependent maximum.
    ///
    /// Only valid with the geometry execution model.
    Invocations(u32),
    /// Requests the tesselation primitive generator to divide edges into a collection
    /// of equal-sized segments.
    ///
    /// Only valid with one of the tessellation execution models.
    SpacingEqual,
    /// Requests the tessellation primitive generator to divide edges into an even number
    /// of equal-length segments plus two additional shorter fractional segments.
    ///
    /// Only valid with one of the tessellation execution models.
    SpacingFractionalEven,
    /// Requests the tessellation primitive generator to divide edges into an odd number
    /// of equal-length segments plus two additional shorter fractional segments.
    ///
    /// Only valid with one of the tessellation execution models.
    SpacingFractionalOdd,
    /// Requests the tessellation primitive generator to generate triangles in clockwise
    /// order.
    ///
    /// Only valid with one of the tessellation execution models.
    VertexOrderCw,
    /// Requests the tessellation primitive generator to generate triangles in
    /// counter-clockwise order.
    ///
    /// Only valid with one of the tessellation execution models.
    VertexOrderCcw,
    /// Pixels appear centered on whole-number pixel offsets. E.g., the coordinate (0.5, 0.5)
    /// appears to move to (0.0, 0.0).
    ///
    /// Only valid with the fragment execution model.
    /// If a fragment entry point does not have this set, pixels appear centered at offsets
    /// of (0.5, 0.5) from whole numbers.
    PixelCenterInteger,
    /// Pixel coordinates appear to originate in the upper left, and increase toward the right
    /// and downward.
    ///
    /// Only valid with the fragment execution model.
    OriginUpperLeft,
    /// Pixel coordinates appear to originate in the lower left, and increase toward the right
    /// and upward.
    ///
    /// Only valid with the fragment execution model.
    OriginLowerLeft,
    /// Fragment tests are to be performed before fragment shader execution.
    ///
    /// Only valid with the fragment execution model.
    EarlyFragmentTests,
    /// Requests the tessellation primitive generator to generate a point for each distinct vertex
    /// in the subdivided primitive, rather than to generate lines or triangles.
    ///
    /// Only valid with one of the tessellation execution models.
    PointMode,
    /// This stage will run in transform feedback-capturing mode and this module is responsible
    /// for describing the transform-feedback setup.
    ///
    /// See the XfbBuffer, Offset, and XfbStride decorations.
    Xfb,
    /// This mode must be declared if this module potentially changes the fragment’s depth.
    ///
    /// Only valid with the fragment execution model.
    DepthReplacing,
    /// External optimizations may assume depth modifications will leave the fragment’s depth
    /// as greater than or equal to the fragment’s interpolated depth value (given by the z
    /// component of the FragCoord BuiltIn decorated variable).
    ///
    /// Only valid with the fragment execution model.
    DepthGreater,
    /// External optimizations may assume depth modifications leave the fragment’s depth less
    /// than the fragment’s interpolated depth value, (given by the z component of the FragCoord
    /// BuiltIn decorated variable).
    ///
    /// Only valid with the fragment execution model.
    DepthLess,
    /// External optimizations may assume this stage did not modify the fragment’s depth. However,
    /// DepthReplacing mode must accurately represent depth modification.
    ///
    /// Only valid with the fragment execution model.
    DepthUnchanged,
    /// Indicates the work-group size in the x, y, and z dimensions.
    ///
    /// Only valid with the GLCompute or Kernel execution models.
    LocalSize { x: u32, y: u32, z: u32 },
    /// Stage input primitive is points.
    ///
    /// Only valid with the geometry execution model.
    InputPoints,
    /// Stage input primitive is lines.
    ///
    /// Only valid with the geometry execution model.
    InputLines,
    /// Stage input primitive is lines adjacency.
    ///
    /// Only valid with the geometry execution model.
    InputLinesAdjacency,
    /// For a geometry stage, input primitive is triangles. For a tessellation stage,
    /// requests the tessellation primitive generator to generate triangles.
    ///
    /// Only valid with the geometry or one of the tessellation execution models.
    Triangles,
    /// Geometry stage input primitive is triangles adjacency.
    ///
    /// Only valid with the geometry execution model.
    InputTrianglesAdjacency,
    /// Requests the tessellation primitive generator to generate quads.
    ///
    /// Only valid with one of the tessellation execution models.
    Quads,
    /// Requests the tessellation primitive generator to generate isolines.
    ///
    /// Only valid with one of the tessellation execution models.
    Isolines,
    /// For a geometry stage, the maximum number of vertices the shader will ever
    /// emit in a single invocation. For a tessellation-control stage, the number
    /// of vertices in the output patch produced by the tessellation control shader,
    /// which also specifies the number of times the tessellation control shader is invoked.
    ///
    /// Only valid with the geometry or one of the tessellation execution models.
    OutputVertices(u32),
    /// Stage output primitive is points.
    ///
    /// Only valid with the geometry execution model.
    OutputPoints,
    /// Stage output primitive is line strip.
    ///
    /// Only valid with the geometry execution model.
    OutputLineStrip,
    /// Stage output primitive is triangle strip.
    ///
    /// Only valid with the geometry execution model.
    OutputTriangleStrip,
    /// Indicates that this entry point is a module initializer.
    Initializer,
    /// Indicates that this entry point is a module finalizer.
    Finalizer,
    /// Indicates that this entry point requires the specified Subgroup Size.
    SubgroupSize(u32),
    /// Indicates that this entry point requires the specified number of Subgroups Per
    /// Workgroup.
    SubgroupsPerWorkgroup(u32),
    /// Indicates that this entry point requires the specified number of Subgroups Per
    /// Workgroup.
    ///
    /// Specified as an Id.
    SubgroupsPerWorkgroupId(SpecId),
    /// Indicates the work-group size in the x, y, and z dimensions.
    ///
    /// Only valid with the GLCompute or Kernel execution models.
    ///
    /// Specified as Ids.
    LocalSizeId { x: SpecId, y: SpecId, z: SpecId },
    PostDepthCoverage,
    StencilRefReplacingEXT,
}
pub struct ExecutionModeDeclaration {
    pub func_id: FunctionId,
    pub execution_mode: ExecutionMode,
}


// The actual reflection to take place.

#[derive(Default)]
pub struct ReflectIntermediate<'a> {
    entry_point_declrs: Vec<EntryPointDeclartion<'a>>,
    execution_mode_declrs: Vec<ExecutionModeDeclaration>,
    spec_consts: Vec<SpecConstant<'a>>,
    vars: Vec<Variable>,

    name_map: HashMap<(InstrId, Option<u32>), &'a str>,
    deco_map: HashMap<(InstrId, Option<u32>, u32), &'a [u32]>,
    ty_map: HashMap<TypeId, Type>,
    var_map: HashMap<VariableId, usize>,
    const_map: HashMap<ConstantId, Constant<'a>>,
    ptr_map: HashMap<TypeId, TypeId>,
    func_map: HashMap<FunctionId, Function>,
    declr_map: HashMap<Locator, InstrId>,
}
impl<'a> ReflectIntermediate<'a> {
    /// Check if a result (like a variable declaration result) or a memeber of a
    /// result (like a structure definition result) has the given decoration.
    pub fn contains_deco(&self, id: InstrId, member_idx: Option<u32>, deco: Decoration) -> bool {
        self.deco_map.contains_key(&(id, member_idx, deco as u32))
    }
    /// Get the single-word decoration of an instruction result.
    pub fn get_deco_u32(&self, id: InstrId, deco: Decoration) -> Option<u32> {
        self.get_deco_list(id, deco)
            .and_then(|x| x.get(0))
            .cloned()
    }
    /// Get the single-word decoration of a member of an instruction result.
    pub fn get_member_deco_u32(
        &self,
        id: InstrId,
        member_idx: u32,
        deco: Decoration,
    ) -> Option<u32> {
        self.get_member_deco_list(id, member_idx, deco)
            .and_then(|x| x.get(0))
            .cloned()
    }
    /// Get the multi-word declaration of a instruction result.
    pub fn get_deco_list(&self, id: InstrId, deco: Decoration) -> Option<&'a [u32]> {
        self.deco_map.get(&(id, None, deco as u32))
            .cloned()
    }
    /// Get the multi-word declaration of a member of an instruction result.
    pub fn get_member_deco_list(
        &self,
        id: InstrId,
        member_idx: u32,
        deco: Decoration,
    ) -> Option<&'a [u32]> {
        self.deco_map.get(&(id, Some(member_idx), deco as u32))
            .cloned()
    }
    /// Get the location-component pair of an interface variable.
    pub fn get_var_location(&self, var_id: VariableId) -> Option<InterfaceLocation> {
        let comp = self.get_deco_u32(var_id, Decoration::Component)
            .unwrap_or(0);
        self.get_deco_u32(var_id, Decoration::Location)
            .map(|loc| InterfaceLocation(loc, comp))
    }
    /// Get the set-binding pair of a descriptor resource.
    pub fn get_var_desc_bind(&self, var_id: VariableId) -> Option<DescriptorBinding> {
        let desc_set = self.get_deco_u32(var_id, Decoration::DescriptorSet)
            .unwrap_or(0);
        self.get_deco_u32(var_id, Decoration::Binding)
            .map(|bind_point| DescriptorBinding::new(desc_set, bind_point))
    }
    /// Get the set-binding pair of a descriptor resource, but the binding point
    /// is forced to 0 if it's not specified in SPIR-V source.
    pub fn get_var_desc_bind_or_default(&self, var_id: VariableId) -> DescriptorBinding {
        self.get_var_desc_bind(var_id)
            .unwrap_or(DescriptorBinding(0, 0))
    }
    /// Get the type identified by `ty_id`.
    pub fn get_ty(&self, ty_id: TypeId) -> Option<&Type> {
        self.ty_map.get(&ty_id)
    }
    /// Get the variable identified by `var_id`.
    pub fn get_var(&self, var_id: VariableId) -> Option<&Variable> {
        let ivar = *self.var_map.get(&var_id)?;
        let var = &self.vars[ivar];
        Some(var)
    }
    /// Get the constant identified by `const_id`. Specialization constants are
    /// also stored as constants. Array extents specified by specialization
    /// constants are not statically known.
    pub fn get_const(&self, const_id: ConstantId) -> Option<&Constant> {
        self.const_map.get(&const_id)
    }
    /// Get the human-friendly name of an instruction result.
    pub fn get_name(&self, id: InstrId) -> Option<&'a str> {
        self.name_map.get(&(id, None)).copied()
    }
    /// Get the human-friendly name of a member of an instruction result.
    pub fn get_member_name(&self, id: InstrId, member_idx: u32) -> Option<&'a str> {
        self.name_map.get(&(id, Some(member_idx))).copied()
    }
    pub fn get_func(&self, func_id: FunctionId) -> Option<&Function> {
        self.func_map.get(&func_id)
    }
    pub fn get_var_name(&self, locator: Locator) -> Option<&'a str> {
        let instr_id = *self.declr_map.get(&locator)?;
        self.get_name(instr_id)
    }
    pub fn entry_point_declrs(&self) -> &[EntryPointDeclartion<'a>] {
        &self.entry_point_declrs
    }
    pub fn execution_mode_declrs(&self) -> &[ExecutionModeDeclaration] {
        &self.execution_mode_declrs
    }
    pub fn spec_consts(&self) -> &[SpecConstant<'a>] {
        &self.spec_consts
    }
    pub fn vars(&self) -> &[Variable] {
        &self.vars
    }
    fn get_desc_access(&self, var_id: VariableId) -> Option<AccessType> {
        let read_only = self.contains_deco(var_id, None, Decoration::NonWritable);
        let write_only = self.contains_deco(var_id, None, Decoration::NonReadable);
        match (read_only, write_only) {
            (true, true) => None,
            (true, false) => Some(AccessType::ReadOnly),
            (false, true) => Some(AccessType::WriteOnly),
            (false, false) => Some(AccessType::ReadWrite),
        }
    }
    /// Resolve one recurring layer of pointers to the pointer that refer to the
    /// data directly. `ty_id` should be refer to a pointer type. Returns the ID
    /// of the type the pointer points to.
    pub fn access_chain(&self, ty_id: TypeId) -> Option<TypeId> {
        self.ptr_map.get(&ty_id).cloned()
    }
}
impl<'a> ReflectIntermediate<'a> {
    fn populate_entry_points(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        while let Some(instr) = instrs.peek() {
            if instr.opcode() != OP_ENTRY_POINT { break; }
            let op = OpEntryPoint::try_from(instr)?;
            let entry_point_declr = EntryPointDeclartion {
                exec_model: op.exec_model,
                func_id: op.func_id,
                name: op.name,
            };
            self.entry_point_declrs.push(entry_point_declr);
            instrs.next();
        }
        Ok(())
    }
    fn populate_execution_modes(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        while let Some(instr) = instrs.peek() {
            if instr.opcode() != OP_EXECUTION_MODE { break; }
            let op = OpExecutionMode::try_from(instr)?;
            let execution_mode = match op.execution_mode {
                spirv_headers::ExecutionMode::Invocations => {
                    ExecutionMode::Invocations(op.params[0])
                },
                spirv_headers::ExecutionMode::SpacingEqual => {
                    ExecutionMode::SpacingEqual
                },
                spirv_headers::ExecutionMode::SpacingFractionalEven => {
                    ExecutionMode::SpacingFractionalEven
                },
                spirv_headers::ExecutionMode::SpacingFractionalOdd => {
                    ExecutionMode::SpacingFractionalOdd
                },
                spirv_headers::ExecutionMode::VertexOrderCw => {
                    ExecutionMode::VertexOrderCw
                },
                spirv_headers::ExecutionMode::VertexOrderCcw => {
                    ExecutionMode::VertexOrderCcw
                },
                spirv_headers::ExecutionMode::PixelCenterInteger => {
                    ExecutionMode::PixelCenterInteger
                },
                spirv_headers::ExecutionMode::OriginUpperLeft => {
                    ExecutionMode::OriginUpperLeft
                },
                spirv_headers::ExecutionMode::OriginLowerLeft => {
                    ExecutionMode::OriginLowerLeft
                },
                spirv_headers::ExecutionMode::EarlyFragmentTests => {
                    ExecutionMode::EarlyFragmentTests
                },
                spirv_headers::ExecutionMode::PointMode => {
                    ExecutionMode::PointMode
                },
                spirv_headers::ExecutionMode::Xfb => {
                    ExecutionMode::Xfb
                },
                spirv_headers::ExecutionMode::DepthReplacing => {
                    ExecutionMode::DepthReplacing
                },
                spirv_headers::ExecutionMode::DepthGreater => {
                    ExecutionMode::DepthGreater
                },
                spirv_headers::ExecutionMode::DepthLess => {
                    ExecutionMode::DepthLess
                },
                spirv_headers::ExecutionMode::DepthUnchanged => {
                    ExecutionMode::DepthUnchanged
                },
                spirv_headers::ExecutionMode::LocalSize => {
                    ExecutionMode::LocalSize {
                        x: op.params[0],
                        y: op.params[1],
                        z: op.params[2]
                    }
                },
                spirv_headers::ExecutionMode::InputPoints => {
                    ExecutionMode::InputPoints
                },
                spirv_headers::ExecutionMode::InputLines => {
                    ExecutionMode::InputLines
                },
                spirv_headers::ExecutionMode::InputLinesAdjacency => {
                    ExecutionMode::InputLinesAdjacency
                },
                spirv_headers::ExecutionMode::Triangles => {
                    ExecutionMode::Triangles
                },
                spirv_headers::ExecutionMode::InputTrianglesAdjacency => {
                    ExecutionMode::InputTrianglesAdjacency
                },
                spirv_headers::ExecutionMode::Quads => {
                    ExecutionMode::Quads
                },
                spirv_headers::ExecutionMode::Isolines => {
                    ExecutionMode::Isolines
                },
                spirv_headers::ExecutionMode::OutputVertices => {
                    ExecutionMode::OutputVertices(op.params[0])
                },
                spirv_headers::ExecutionMode::OutputPoints => {
                    ExecutionMode::OutputPoints
                },
                spirv_headers::ExecutionMode::OutputLineStrip => {
                    ExecutionMode::OutputLineStrip
                },
                spirv_headers::ExecutionMode::OutputTriangleStrip => {
                    ExecutionMode::OutputTriangleStrip
                },
                spirv_headers::ExecutionMode::Initializer => {
                    ExecutionMode::Initializer
                },
                spirv_headers::ExecutionMode::Finalizer => {
                    ExecutionMode::Finalizer
                },
                spirv_headers::ExecutionMode::SubgroupSize => {
                    ExecutionMode::SubgroupSize(op.params[0])
                },
                spirv_headers::ExecutionMode::SubgroupsPerWorkgroup => {
                    ExecutionMode::SubgroupsPerWorkgroup(op.params[0])
                },
                spirv_headers::ExecutionMode::SubgroupsPerWorkgroupId => {
                    ExecutionMode::SubgroupsPerWorkgroupId(op.params[0])
                },
                spirv_headers::ExecutionMode::LocalSizeId => {
                    ExecutionMode::LocalSizeId {
                        x: op.params[0],
                        y: op.params[1],
                        z: op.params[2]
                    }
                },
                spirv_headers::ExecutionMode::PostDepthCoverage => {
                    ExecutionMode::PostDepthCoverage
                },
                spirv_headers::ExecutionMode::StencilRefReplacingEXT => {
                    ExecutionMode::StencilRefReplacingEXT
                },
                _ => { return Err(Error::UNSUPPORTED_EXEC_MODE); }
            };
            let execution_mode_declr = ExecutionModeDeclaration {
                func_id: op.func_id,
                execution_mode
            };
            self.execution_mode_declrs.push(execution_mode_declr);
            instrs.next();
        }
        Ok(())
    }
    fn populate_names(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        // Extract naming. Names are generally produced as debug information by
        // `glslValidator` but it might be in absence.
        while let Some(instr) = instrs.peek() {
            let (key, value) = match instr.opcode() {
                OP_NAME => {
                    let op = OpName::try_from(instr)?;
                    ((op.target_id, None), op.name)
                },
                OP_MEMBER_NAME => {
                    let op = OpMemberName::try_from(instr)?;
                    ((op.target_id, Some(op.member_idx)), op.name)
                },
                _ => break,
            };
            if !value.is_empty() {
                let collision = self.name_map.insert(key, value);
                if collision.is_some() { return Err(Error::NAME_COLLISION); }
            }
            instrs.next();
        }
        Ok(())
    }
    fn populate_decos(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        while let Some(instr) = instrs.peek() {
            let (key, value) = match instr.opcode() {
                OP_DECORATE => {
                    let op = OpDecorate::try_from(instr)?;
                    ((op.target_id, None, op.deco), op.params)
                }
                OP_MEMBER_DECORATE => {
                    let op = OpMemberDecorate::try_from(instr)?;
                    ((op.target_id, Some(op.member_idx), op.deco), op.params)
                },
                x => if is_deco_op(x) { instrs.next(); continue } else { break },
            };
            let collision = self.deco_map.insert(key, value);
            if collision.is_some() { return Err(Error::DECO_COLLISION); }
            instrs.next();
        }
        Ok(())
    }
    fn populate_one_ty(&mut self, instr: &Instr<'a>) -> Result<()> {
        use std::collections::hash_map::Entry::Vacant;
        let (key, value) = match instr.opcode() {
            OP_TYPE_FUNCTION => { return Ok(()) },
            OP_TYPE_VOID => {
                let op = OpTypeVoid::try_from(instr)?;
                (op.ty_id, Type::Void())
            },
            OP_TYPE_BOOL => {
                let op = OpTypeBool::try_from(instr)?;
                let scalar_ty = ScalarType::boolean();
                (op.ty_id, Type::Scalar(scalar_ty))
            },
            OP_TYPE_INT => {
                let op = OpTypeInt::try_from(instr)?;
                let scalar_ty = ScalarType::int(op.nbyte >> 3, op.is_signed);
                (op.ty_id, Type::Scalar(scalar_ty))
            },
            OP_TYPE_FLOAT => {
                let op = OpTypeFloat::try_from(instr)?;
                let scalar_ty = ScalarType::float(op.nbyte >> 3);
                (op.ty_id, Type::Scalar(scalar_ty))
            },
            OP_TYPE_VECTOR => {
                let op = OpTypeVector::try_from(instr)?;
                if let Some(Type::Scalar(scalar_ty)) = self.get_ty(op.scalar_ty_id) {
                    let vec_ty = VectorType::new(scalar_ty.clone(), op.nscalar);
                    (op.ty_id, Type::Vector(vec_ty))
                } else { return Err(Error::TY_NOT_FOUND); }
            },
            OP_TYPE_MATRIX => {
                let op = OpTypeMatrix::try_from(instr)?;
                if let Some(Type::Vector(vec_ty)) = self.get_ty(op.vec_ty_id) {
                    let mat_ty = MatrixType::new(vec_ty.clone(), op.nvec);
                    (op.ty_id, Type::Matrix(mat_ty))
                } else { return Err(Error::TY_NOT_FOUND); }
            },
            OP_TYPE_IMAGE => {
                let op = OpTypeImage::try_from(instr)?;
                let scalar_ty = match self.get_ty(op.scalar_ty_id) {
                    Some(Type::Scalar(scalar_ty)) => Some(scalar_ty.clone()),
                    Some(Type::Void()) => None,
                    _ => return Err(Error::TY_NOT_FOUND),
                };
                let img_ty = if op.dim == Dim::DimSubpassData {
                    let arng = SubpassDataArrangement::from_spv_def(op.is_multisampled)?;
                    let subpass_data_ty = SubpassDataType::new(scalar_ty, arng);
                    Type::SubpassData(subpass_data_ty)
                } else {
                    // Only unit types allowed to be stored in storage images
                    // can have given format.
                    let unit_fmt = ImageUnitFormat::from_spv_def(
                        op.is_sampled, op.is_depth, op.color_fmt)?;
                    let arng = ImageArrangement::from_spv_def(
                        op.dim, op.is_array, op.is_multisampled)?;
                    let img_ty = ImageType::new(scalar_ty, unit_fmt, arng);
                    Type::Image(img_ty)
                };
                (op.ty_id, img_ty)
            },
            OP_TYPE_SAMPLER => {
                let op = OpTypeSampler::try_from(instr)?;
                // Note that SPIR-V doesn't discriminate color and depth/stencil
                // samplers. `sampler` and `samplerShadow` means the same thing.
                (op.ty_id, Type::Sampler())
            },
            OP_TYPE_SAMPLED_IMAGE => {
                let op = OpTypeSampledImage::try_from(instr)?;
                if let Some(Type::Image(img_ty)) = self.get_ty(op.img_ty_id) {
                    let sampled_img_ty = SampledImageType::new(img_ty.clone());
                    (op.ty_id, Type::SampledImage(sampled_img_ty))
                } else { return Err(Error::TY_NOT_FOUND); }
            },
            OP_TYPE_ARRAY => {
                let op = OpTypeArray::try_from(instr)?;
                let proto_ty = if let Some(proto_ty) = self.get_ty(op.proto_ty_id) {
                    proto_ty
                } else {
                    return Ok(());
                };

                let nrepeat = self.const_map.get(&op.nrepeat_const_id)
                    // Some notes about specialization constants.
                    //
                    // Using specialization constants for array sizes might lead
                    // to UNDEFINED BEHAVIOR because structure size MUST be
                    // definitive at compile time and CANNOT be specialized at
                    // runtime according to Khronos members, but the default
                    // behavior of `glslang` is to treat the specialization
                    // constants as normal constants, then I would say...
                    // probably it's fine to size array with them?
                    .and_then(|constant| {
                        if let Some(Type::Scalar(scalar_ty)) = self.get_ty(constant.ty_id) {
                            if scalar_ty.nbyte() == 4 && scalar_ty.is_uint() {
                                return constant.value.iter().next().cloned();
                            }
                        }
                        None
                    });
                let stride = self.get_deco_u32(op.ty_id, Decoration::ArrayStride)
                    .map(|x| x as usize);

                let arr_ty = if let Some(nrepeat) = nrepeat {
                    if let Some(stride) = stride {
                        ArrayType::new(&proto_ty, nrepeat, stride)
                    } else {
                        ArrayType::new_multibind(&proto_ty, nrepeat)
                    }
                } else {
                    // We expect the constant is registered but we failed to
                    // find it. It's possible that the SPIR-V generated a
                    // forward reference to a result of `OpCompositeExtract` to
                    // get a component of a specialized composite. One example
                    // is to use workgroup size to declare arrays.
                    //
                    // Such behavior is observed in `glslangValidator`, built
                    // with glslang 11.1; but AFAIK no longer with glslang 11.5
                    // in which `OpSpecConstantOp` with `OpCompositeExtract`
                    // is generated instead.
                    //
                    // Either way, if array size is a specialized value, we have
                    // no idea about the actual size of the array through static
                    // analysis. To the most possible extent, we assume the
                    // SPIR-V input is valid so we also assume any missing
                    // reference points to a forward-referenced instruction.
                    if let Some(stride) = stride {
                        ArrayType::new_unsized(&proto_ty, stride)
                    } else {
                        ArrayType::new_unsized_multibind(&proto_ty)
                    }
                };
                (op.ty_id, Type::Array(arr_ty))
            },
            OP_TYPE_RUNTIME_ARRAY => {
                let op = OpTypeRuntimeArray::try_from(instr)?;
                let proto_ty = if let Some(proto_ty) = self.get_ty(op.proto_ty_id) {
                    proto_ty
                } else {
                    return Ok(());
                };
                let stride = self.get_deco_u32(op.ty_id, Decoration::ArrayStride)
                    .map(|x| x as usize);
                let arr_ty = if let Some(stride) = stride {
                    ArrayType::new_unsized(&proto_ty, stride)
                } else {
                    ArrayType::new_unsized_multibind(&proto_ty)
                };
                (op.ty_id, Type::Array(arr_ty))
            },
            OP_TYPE_STRUCT => {
                let op = OpTypeStruct::try_from(instr)?;
                let struct_name = self.get_name(op.ty_id).map(|n| n.to_string());
                let mut struct_ty = StructType::new(struct_name);
                for (i, &member_ty_id) in op.member_ty_ids.iter().enumerate() {
                    let i = i as u32;
                    let mut member_ty = if let Some(member_ty) = self.get_ty(member_ty_id) {
                        member_ty.clone()
                    } else {
                        return Ok(());
                    };
                    let mut proto_ty = &mut member_ty;
                    while let Type::Array(arr_ty) = proto_ty {
                        proto_ty = &mut *arr_ty.proto_ty;
                    }
                    if let Type::Matrix(ref mut mat_ty) = proto_ty {
                        let mat_stride = self
                            .get_member_deco_u32(op.ty_id, i, Decoration::MatrixStride)
                            .map(|x| x as usize)
                            .ok_or(Error::MISSING_DECO)?;
                        let row_major = self.contains_deco(op.ty_id, Some(i), Decoration::RowMajor);
                        let col_major = self.contains_deco(op.ty_id, Some(i), Decoration::ColMajor);
                        let major = match (row_major, col_major) {
                            (true, false) => MatrixAxisOrder::RowMajor,
                            (false, true) => MatrixAxisOrder::ColumnMajor,
                            _ => return Err(Error::UNENCODED_ENUM),
                        };
                        mat_ty.decorate(mat_stride, major);
                    }
                    let name = if let Some(nm) = self.get_member_name(op.ty_id, i) {
                        if nm.is_empty() { None } else { Some(nm.to_owned()) }
                    } else { None };
                    if let Some(offset) = self.get_member_deco_u32(op.ty_id, i, Decoration::Offset)
                        .map(|x| x as usize) {
                        let member = StructMember {
                            name,
                            offset,
                            ty: member_ty.clone()
                        };
                        struct_ty.members.push(member);
                    } else {
                        // For shader input/output blocks there are no offset
                        // decoration. Since these variables are not externally
                        // accessible we don't have to worry about them.
                        return Ok(())
                    }
                }
                // Don't have to shrink-to-fit because the types in `ty_map`
                // won't be used directly and will be cloned later.
                (op.ty_id, Type::Struct(struct_ty))
            },
            OP_TYPE_POINTER => {
                let op = OpTypePointer::try_from(instr)?;
                if self.ptr_map.insert(op.ty_id, op.target_ty_id).is_some() {
                    return Err(Error::ID_COLLISION)
                } else { return Ok(()) }
            },
            OP_TYPE_ACCELERATION_STRUCTURE_KHR => {
                let op = OpTypeAccelerationStructureKHR::try_from(instr)?;
                (op.ty_id, Type::AccelStruct())
            },
            _ => return Err(Error::UNSUPPORTED_TY),
        };
        if let Vacant(entry) = self.ty_map.entry(key) {
            entry.insert(value); Ok(())
        } else { Err(Error::ID_COLLISION) }
    }
    fn populate_one_const(&mut self, instr: &Instr<'a>) -> Result<()> {
        use std::collections::hash_map::Entry::Vacant;
        if instr.opcode() == OP_CONSTANT {
            let op = OpConstant::try_from(instr)?;
            if let Vacant(entry) = self.const_map.entry(op.const_id) {
                let constant = Constant {
                    ty_id: op.ty_id,
                    value: op.value,
                };
                entry.insert(constant);
                Ok(())
            } else { Err(Error::ID_COLLISION) }
        } else {
            Ok(())
        }
    }
    fn populate_one_spec_const(&mut self, instr: &Instr<'a>) -> Result<()> {
        use std::collections::hash_map::Entry::Vacant;
        let (spec_const_id, constant, spec_const) = match instr.opcode() {
            OP_SPEC_CONSTANT_TRUE => {
                let op = OpSpecConstantTrue::try_from(instr)?;
                let constant = Constant {
                    ty_id: op.ty_id,
                    value: &[1],
                };
                let spec_id = self.get_deco_u32(op.spec_const_id, Decoration::SpecId)
                    .ok_or(Error::MISSING_DECO)?;
                let spec_const = SpecConstant {
                    ty_id: constant.ty_id,
                    value: &[1],
                    spec_id,
                };
                (op.spec_const_id, constant, Some(spec_const))
            },
            OP_SPEC_CONSTANT_FALSE => {
                let op = OpSpecConstantFalse::try_from(instr)?;
                let constant = Constant {
                    ty_id: op.ty_id,
                    value: &[0],
                };
                let spec_id = self.get_deco_u32(op.spec_const_id, Decoration::SpecId)
                    .ok_or(Error::MISSING_DECO)?;
                let spec_const = SpecConstant {
                    ty_id: constant.ty_id,
                    value: &[0],
                    spec_id,
                };
                (op.spec_const_id, constant, Some(spec_const))
            },
            OP_SPEC_CONSTANT => {
                let op = OpSpecConstant::try_from(instr)?;
                let constant = Constant {
                    ty_id: op.ty_id,
                    value: op.value,
                };
                let spec_id = self.get_deco_u32(op.spec_const_id, Decoration::SpecId)
                    .ok_or(Error::MISSING_DECO)?;
                let spec_const = SpecConstant {
                    ty_id: constant.ty_id,
                    value: op.value,
                    spec_id,
                };
                (op.spec_const_id, constant, Some(spec_const))
            },
            // `SpecId` decorations will be specified to each of the
            // constituents so we don't have to register a `SpecConstant` for
            // the composite of them. `SpecConstant` is registered only for
            // those will be interacting with Vulkan.
            OP_SPEC_CONSTANT_COMPOSITE => {
                let op = OpSpecConstantComposite::try_from(instr)?;
                let constant = Constant {
                    ty_id: op.ty_id,
                    // Empty value to annotate a specialization constant. We
                    // have nothing like a `SpecId` to access such
                    // specialization constant so it's unnecesary to resolve
                    // it's default value. Same applies to `OpSpecConstantOp`.
                    value: &[] as &'static [u32],
                };
                (op.spec_const_id, constant, None)
            },
            // Similar to `OpConstantComposite`, we don't register
            // specialization constants for `OpSpecConstantOp` results, neither
            // the validity of the operations because they are out of SPIR-Q's
            // duty.
            //
            // NOTE: In some cases you might want to use specialized workgroup
            // size to allocate shared memory or other on-chip memory with this,
            // that's possible, but still be aware that specialization constants
            // CANNOT be used to specify any STRUCTURED memory objects like UBO
            // and SSBO, because the stride and offset decorations are
            // precompiled as a part of the SPIR-V binary meta.
            OP_SPEC_CONSTANT_OP => {
                let op = OpSpecConstantOp::try_from(instr)?;
                let constant = Constant {
                    ty_id: op.ty_id,
                    value: &[] as &'static [u32],
                };
                (op.spec_const_id, constant, None)
            },
            _ => return Err(Error::UNSUPPORTED_SPEC),
        };

        if let Vacant(entry) = self.const_map.entry(spec_const_id) {
            entry.insert(constant);
        } else { return Err(Error::ID_COLLISION) }
        if let Some(spec_const) = spec_const {
            let locator = Locator::SpecConstant(spec_const.spec_id);
            self.declr_map.insert(locator, spec_const_id);
            self.spec_consts.push(spec_const);
        }

        Ok(())
    }
    fn populate_one_var(&mut self, instr: &Instr<'a>) -> Result<()> {
        fn extract_proto_ty<'a>(ty: &Type) -> Result<(u32, Type)> {
            match ty {
                Type::Array(arr_ty) => {
                    // `nrepeat=None` is no longer considered invalid because of
                    // the adoption of `SPV_EXT_descriptor_indexing`. This
                    // shader extension has been supported in Vulkan 1.2.
                    let nrepeat = arr_ty.nrepeat()
                        .unwrap_or(0);
                    let proto_ty = arr_ty.proto_ty();
                    Ok((nrepeat, proto_ty.clone()))
                },
                _ => Ok((1, ty.clone())),
            }
        }

        let op = OpVariable::try_from(instr)?;
        let ty_id = self.access_chain(op.ty_id)
            .ok_or(Error::BROKEN_ACCESS_CHAIN)?;
        let ty = if let Some(ty) = self.get_ty(ty_id) {
            ty
        } else {
            // If a variable is declared based on a unregistered type, very
            // likely it's a input/output block passed between shader stages. We
            // can safely ignore them.
            return Ok(());
        };
        let name = self.get_name(op.var_id).map(|x| x.to_owned());
        let var = match op.store_cls {
            StorageClass::Input => {
                if let Some(location) = self.get_var_location(op.var_id) {
                    let var = Variable::Input { name, location, ty: ty.clone() };
                    // There can be interface blocks for input and output but
                    // there won't be any for attribute inputs nor for
                    // attachment outputs, so we just ignore structs and arrays
                    // or something else here.
                    Some(var)
                } else {
                    // Ignore built-in interface varaibles whichh have no
                    // location assigned.
                    None
                }
            },
            StorageClass::Output => {
                if let Some(location) = self.get_var_location(op.var_id) {
                    let var = Variable::Output { name, location, ty: ty.clone() };
                    Some(var)
                } else {
                    None
                }
            },
            StorageClass::PushConstant => {
                // Push constants have no global offset. Offsets are applied to
                // members.
                if let Type::Struct(_) = ty {
                    let var = Variable::PushConstant { name, ty: ty.clone() };
                    Some(var)
                } else {
                    return Err(Error::TY_NOT_FOUND);
                }
            },
            StorageClass::Uniform => {
                let (nbind, ty) = extract_proto_ty(ty)?;
                let desc_bind = self.get_var_desc_bind_or_default(op.var_id);
                let var = if self.contains_deco(ty_id, None, Decoration::BufferBlock) {
                    let access = self.get_desc_access(op.var_id)
                        .ok_or(Error::ACCESS_CONFLICT)?;
                    let desc_ty = DescriptorType::StorageBuffer(access);
                    Variable::Descriptor { name, desc_bind, desc_ty, ty: ty.clone(), nbind }
                } else {
                    let desc_ty = DescriptorType::UniformBuffer();
                    Variable::Descriptor { name, desc_bind, desc_ty, ty: ty.clone(), nbind }
                };
                Some(var)
            },
            StorageClass::StorageBuffer => {
                let (nbind, ty) = extract_proto_ty(ty)?;
                let desc_bind = self.get_var_desc_bind_or_default(op.var_id);
                let access = self.get_desc_access(op.var_id)
                    .ok_or(Error::ACCESS_CONFLICT)?;
                let desc_ty = DescriptorType::StorageBuffer(access);
                let var = Variable::Descriptor { name, desc_bind, desc_ty, ty: ty.clone(), nbind };
                Some(var)
            },
            StorageClass::UniformConstant => {
                let (nbind, ty) = extract_proto_ty(ty)?;
                let desc_bind = self.get_var_desc_bind_or_default(op.var_id);
                let var = match &ty {
                    Type::Image(img_ty) => {
                        let desc_ty = match img_ty.unit_fmt {
                            ImageUnitFormat::Color(_) => {
                                let access = self.get_desc_access(op.var_id)
                                    .ok_or(Error::ACCESS_CONFLICT)?;
                                match img_ty.arng {
                                    ImageArrangement::ImageBuffer => DescriptorType::StorageTexelBuffer(access),
                                    _ => DescriptorType::StorageImage(access),
                                }
                            },
                            ImageUnitFormat::Sampled => match img_ty.arng {
                                ImageArrangement::ImageBuffer => DescriptorType::UniformTexelBuffer(),
                                _ => DescriptorType::SampledImage(),
                            },
                            ImageUnitFormat::Depth => DescriptorType::SampledImage(),
                        };
                        Variable::Descriptor { name, desc_bind, desc_ty, ty: ty.clone(), nbind }
                    },
                    Type::Sampler() => {
                        let desc_ty = DescriptorType::Sampler();
                        Variable::Descriptor { name, desc_bind, desc_ty, ty: ty.clone(), nbind }
                    },
                    Type::SampledImage(_) => {
                        let desc_ty = DescriptorType::CombinedImageSampler();
                        Variable::Descriptor { name, desc_bind, desc_ty, ty: ty.clone(), nbind }
                    },
                    Type::SubpassData(_) => {
                        let input_attm_idx = self
                            .get_deco_u32(op.var_id, Decoration::InputAttachmentIndex)
                            .ok_or(Error::MISSING_DECO)?;
                        let desc_ty = DescriptorType::InputAttachment(input_attm_idx);
                        Variable::Descriptor { name, desc_bind, desc_ty, ty: ty.clone(), nbind }
                    },
                    Type::AccelStruct() => {
                        let desc_ty = DescriptorType::AccelStruct();
                        Variable::Descriptor { name, desc_bind, desc_ty, ty: ty.clone(), nbind }
                    },
                    _ => return Err(Error::UNSUPPORTED_TY),
                };
                Some(var)
            },
            _ => {
                // Leak out unknown storage classes.
                None
            },
        };
        
        if let Some(var) = var {
            // Register variable.
            if self.var_map.insert(op.var_id, self.vars.len()).is_some() {
                return Err(Error::ID_COLLISION);
            }
            let locator = var.locator();
            self.declr_map.insert(locator, op.var_id);
            self.vars.push(var);
        }


        Ok(())
    }
    fn populate_defs(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        // type definitions always follow decorations, so we don't skip
        // instructions here.
        while let Some(instr) = instrs.peek() {
            let opcode = instr.opcode();
            if TYPE_RANGE.contains(&opcode) || opcode == OP_TYPE_ACCELERATION_STRUCTURE_KHR {
                self.populate_one_ty(instr)?;
            } else if opcode == OP_VARIABLE {
                self.populate_one_var(instr)?;
            } else if CONST_RANGE.contains(&opcode) {
                self.populate_one_const(instr)?;
            } else if SPEC_CONST_RANGE.contains(&opcode) {
                self.populate_one_spec_const(instr)?;
            } else { break; }
            instrs.next();
        }
        Ok(())
    }
    fn populate_access<I: Inspector>(
        &mut self,
        instrs: &'_ mut Peekable<Instrs<'a>>,
        mut inspector: I
    ) -> Result<()> {
        let mut access_chain_map = HashMap::default();
        let mut func_id: InstrId = !0;

        while let Some(instr) = instrs.peek() {
            let mut notify_inspector = func_id != !0;
            // Do our works first.
            match instr.opcode() {
                OP_FUNCTION => {
                    let op = OpFunction::try_from(instr)?;
                    func_id = op.func_id;
                    let last = self.func_map.insert(func_id, Default::default());
                    if last.is_some() {
                        return Err(Error::ID_COLLISION);
                    }
                    notify_inspector = true;
                },
                OP_FUNCTION_CALL => {
                    let op = OpFunctionCall::try_from(instr)?;
                    let func = self.func_map.get_mut(&func_id)
                        .ok_or(Error::FUNC_NOT_FOUND)?;
                    func.callees.insert(op.func_id);
                },
                OP_LOAD | OP_ATOMIC_LOAD |  OP_ATOMIC_EXCHANGE..=OP_ATOMIC_XOR => {
                    let op = OpLoad::try_from(instr)?;
                    let mut var_id = op.var_id;
                    // Resolve access chain.
                    if let Some(&x) = access_chain_map.get(&var_id) { var_id = x }
                    let func = self.func_map.get_mut(&func_id)
                        .ok_or(Error::FUNC_NOT_FOUND)?;
                    func.accessed_vars.insert(var_id);
                },
                OP_STORE | OP_ATOMIC_STORE => {
                    let op = OpStore::try_from(instr)?;
                    let mut var_id = op.var_id;
                    // Resolve access chain.
                    if let Some(&x) = access_chain_map.get(&var_id) { var_id = x }
                    let func = self.func_map.get_mut(&func_id)
                        .ok_or(Error::FUNC_NOT_FOUND)?;
                    func.accessed_vars.insert(var_id);
                },
                OP_ACCESS_CHAIN => {
                    let op = OpAccessChain::try_from(instr)?;
                    if access_chain_map.insert(op.var_id, op.accessed_var_id).is_some() {
                        return Err(Error::ID_COLLISION);
                    }
                },
                OP_FUNCTION_END => {
                    func_id = !0;
                },
                _ => { },
            }
            // Then notify the inspector.
            if notify_inspector {
                inspector.inspect(&self, instr)
            }

            instrs.next();
        }
        Ok(())
    }
    pub(crate) fn reflect<I: Inspector>(instrs: Instrs<'a>, inspector: I) -> Result<Self> {
        fn skip_until_range_inclusive<'a>(
            instrs: &'_ mut Peekable<Instrs<'a>>,
            rng: RangeInclusive<u32>
        ) {
            while let Some(instr) = instrs.peek() {
                if !rng.contains(&instr.opcode()) { instrs.next(); } else { break; }
            }
        }
        fn skip_until<'a>(instrs: &'_ mut Peekable<Instrs<'a>>, pred: fn(u32) -> bool) {
            while let Some(instr) = instrs.peek() {
                if !pred(instr.opcode()) { instrs.next(); } else { break; }
            }
        }
        // Don't change the order. See _2.4 Logical Layout of a Module_ of the
        // SPIR-V specification for more information.
        let mut instrs = instrs.peekable();
        let mut itm = ReflectIntermediate::default();
        skip_until_range_inclusive(&mut instrs, ENTRY_POINT_RANGE);
        itm.populate_entry_points(&mut instrs)?;
        itm.populate_execution_modes(&mut instrs)?;
        skip_until_range_inclusive(&mut instrs, NAME_RANGE);
        itm.populate_names(&mut instrs)?;
        skip_until(&mut instrs, is_deco_op);
        itm.populate_decos(&mut instrs)?;
        itm.populate_defs(&mut instrs)?;
        itm.populate_access(&mut instrs, inspector)?;
        return Ok(itm);
    }
}

impl<'a> ReflectIntermediate<'a> {
    fn collect_fn_vars_impl(&self, func: FunctionId, vars: &mut Vec<VariableId>) {
        if let Some(func) = self.get_func(func) {
            vars.extend(func.accessed_vars.iter());
            for call in func.callees.iter() {
                self.collect_fn_vars_impl(*call, vars);
            }
        }
    }
    fn collect_fn_vars(&self, func: FunctionId) -> Vec<VariableId> {
        let mut accessed_vars = Vec::new();
        self.collect_fn_vars_impl(func, &mut accessed_vars);
        accessed_vars
    }
    fn collect_entry_point_vars(&self, func_id: FunctionId) -> Result<Vec<Variable>> {
        let mut vars = Vec::new();
        for accessed_var_id in self.collect_fn_vars(func_id).into_iter().collect::<HashSet<_>>() {
            // Sometimes this process would meet interface variables without
            // locations. These are should built-ins otherwise the SPIR-V is
            // corrupted. Since we assume the SPIR-V is valid and we don't
            // collect built-in variable as useful information, we simply ignore
            // such null-references.
            if let Some(accessed_var) = self.get_var(accessed_var_id) {
                vars.push(accessed_var.clone());
            }
        }
        Ok(vars)
    }
    fn collect_entry_point_specs(&self) -> Result<Vec<Specialization>> {
        // TODO: (penguinlion) Report only specialization constants that have
        // been refered to by the specified function. (Do we actually need this?
        // It might not be an optimization in mind of engineering.)
        let mut specs = Vec::new();
        for spec_const in self.spec_consts().iter() {
            let ty = self.get_ty(spec_const.ty_id)
                .ok_or(Error::TY_NOT_FOUND)?;
            let locator = Locator::SpecConstant(spec_const.spec_id);
            let name = self.get_var_name(locator);
            let spec = Specialization {
                name: name.map(|x| x.to_owned()),
                spec_id: spec_const.spec_id,
                ty: ty.clone(),
            };
            specs.push(spec);
        }
        Ok(specs)
    }
    fn collect_exec_modes(&self, func_id: FunctionId) -> Vec<ExecutionMode> {
        self.execution_mode_declrs.iter()
            .filter_map(|declaration| {
                if declaration.func_id == func_id {
                    return Some(declaration.execution_mode.clone());
                }
                None
            })
            .collect()
    }
    pub(crate) fn collect_entry_points(&self) -> Result<Vec<EntryPoint>> {
        let mut entry_points = Vec::with_capacity(self.entry_point_declrs().len());
        for entry_point_declr in self.entry_point_declrs().iter() {
            let vars = self.collect_entry_point_vars(entry_point_declr.func_id)?;
            let specs = self.collect_entry_point_specs()?;
            let exec_modes = self.collect_exec_modes(entry_point_declr.func_id);
            let entry_point = EntryPoint {
                name: entry_point_declr.name.to_owned(),
                exec_model: entry_point_declr.exec_model,
                vars,
                specs,
                exec_modes,
            };
            entry_points.push(entry_point);
        }
        Ok(entry_points)
    }
    /// Collect resources discovered in this reflection session, no matter if
    /// it's been used by any entry point.
    pub(crate) fn collect_module_as_entry_point(&self) -> Result<EntryPoint> {
        let vars = self.vars().iter().cloned().collect();
        let specs = self.collect_entry_point_specs()?;

        let entry_point_declrs = self.entry_point_declrs();
        if entry_point_declrs.len() != 1 {
            return Err(Error::MULTI_ENTRY_POINTS);
        }
        let entry_point_declr = &entry_point_declrs[0];
        let exec_modes = self.collect_exec_modes(entry_point_declr.func_id);
        let entry_point = EntryPoint {
            name: entry_point_declr.name.to_owned(),
            exec_model: entry_point_declr.exec_model,
            vars,
            specs,
            exec_modes,
        };
        Ok(entry_point)
    }
}
