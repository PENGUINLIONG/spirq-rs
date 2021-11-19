//! Reflection procedures and types.
use std::convert::{TryFrom};
use std::iter::Peekable;
use std::ops::RangeInclusive;
use std::fmt;
use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use crate::ty::*;
use crate::consts::*;
use crate::{Manifest, EntryPoint, Specialization};
use crate::parse::{Instrs, Instr};
use crate::error::{Error, Result};
use crate::instr::*;
use crate::inspect::Inspector;

pub use spirv_headers::{ExecutionModel, Decoration, StorageClass};

// Public types.

/// Access type of a variable.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
        return match (self, rhs) {
            (Self::ReadOnly, Self::ReadOnly) => Self::ReadOnly,
            (Self::WriteOnly, Self::WriteOnly) => Self::WriteOnly,
            _ => Self::ReadWrite,
        }
    }
}
impl std::ops::BitAnd<AccessType> for AccessType {
    type Output = Option<AccessType>;
    fn bitand(self, rhs: AccessType) -> Option<AccessType> {
        return match (self, rhs) {
            (Self::ReadOnly, Self::ReadWrite) |
                (Self::ReadWrite, Self::ReadOnly) |
                (Self::ReadOnly, Self::ReadOnly) => Some(Self::ReadOnly),
            (Self::WriteOnly, Self::ReadWrite) |
                (Self::ReadWrite, Self::WriteOnly) |
                (Self::WriteOnly, Self::WriteOnly) => Some(Self::WriteOnly),
            (Self::ReadWrite, Self::ReadWrite) => Some(Self::ReadWrite),
            (_, _) => None,
        }
    }
}

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
#[derive(Debug, Clone)]
pub enum Variable {
    /// Input interface variable.
    Input(InterfaceLocation, Type),
    /// Output interface variable.
    Output(InterfaceLocation, Type),
    /// Descriptor resource.
    Descriptor(DescriptorBinding, DescriptorType, AccessType),
    /// Push constant.
    PushConstant(Type),
}
impl Variable {
    pub fn locator(&self) -> Locator {
        match self {
            Variable::Input(location, ..) => Locator::Input(*location),
            Variable::Output(location, ..) => Locator::Output(*location),
            Variable::Descriptor(desc_bind, ..) => Locator::Descriptor(*desc_bind),
            Variable::PushConstant(..) => Locator::PushConstant,
        }
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



// The actual reflection to take place.

#[derive(Default)]
pub struct ReflectIntermediate<'a> {
    entry_point_declrs: Vec<EntryPointDeclartion<'a>>,
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
            OP_TYPE_VOID | OP_TYPE_FUNCTION => { return Ok(()) },
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
                if let Some(Type::Scalar(scalar_ty)) = self.get_ty(op.scalar_ty_id) {
                    let img_ty = if op.dim == Dim::DimSubpassData {
                        let arng = SubpassDataArrangement::from_spv_def(op.is_multisampled)?;
                        let subpass_data_ty = SubpassDataType::new(scalar_ty.clone(), arng);
                        Type::SubpassData(subpass_data_ty)
                    } else {
                        // Only unit types allowed to be stored in storage images
                        // can have given format.
                        let unit_fmt = ImageUnitFormat::from_spv_def(
                            op.is_sampled, op.is_depth, op.color_fmt)?;
                        let arng = ImageArrangement::from_spv_def(
                            op.dim, op.is_array, op.is_multisampled)?;
                        let img_ty = ImageType::new(scalar_ty.clone(), unit_fmt, arng);
                        Type::Image(img_ty)
                    };
                    (op.ty_id, img_ty)
                } else { return Err(Error::TY_NOT_FOUND); }
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
                                return Some(constant.value[0]);
                            }
                        }
                        None
                    })
                    .ok_or(Error::CONST_NOT_FOUND)?;
                let stride = self.get_deco_u32(op.ty_id, Decoration::ArrayStride)
                    .map(|x| x as usize);
                let arr_ty = if let Some(stride) = stride {
                    ArrayType::new(&proto_ty, nrepeat, stride)
                } else {
                    ArrayType::new_multibind(&proto_ty, nrepeat)
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
                        struct_ty.push_member(member)?;
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
                (op.spec_const_id, constant, spec_const)
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
                (op.spec_const_id, constant, spec_const)
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
                (op.spec_const_id, constant, spec_const)
            },
            // `SpecId` decorations will be specified to each of the
            // constituents so we don't have to worry about the composite of
            // them.
            OP_SPEC_CONSTANT_COMPOSITE => return Ok(()),
            _ => return Err(Error::UNSUPPORTED_SPEC),
        };

        if let Vacant(entry) = self.const_map.entry(spec_const_id) {
            entry.insert(constant);
        } else { return Err(Error::ID_COLLISION) }
        let locator = Locator::SpecConstant(spec_const.spec_id);
        self.declr_map.insert(locator, spec_const_id);
        self.spec_consts.push(spec_const);

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
        let var = match op.store_cls {
            StorageClass::Input => {
                if let Some(location) = self.get_var_location(op.var_id) {
                    let var = Variable::Input(location, ty.clone());
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
                    let var = Variable::Output(location, ty.clone());
                    Some(var)
                } else {
                    None
                }
            },
            StorageClass::PushConstant => {
                // Push constants have no global offset. Offsets are applied to
                // members.
                if let Type::Struct(_) = ty {
                    let var = Variable::PushConstant(ty.clone());
                    Some(var)
                } else {
                    return Err(Error::TY_NOT_FOUND);
                }
            },
            StorageClass::Uniform => {
                let (nbind, ty) = extract_proto_ty(ty)?;
                let desc_bind = self.get_var_desc_bind_or_default(op.var_id);
                let var = if self.contains_deco(ty_id, None, Decoration::BufferBlock) {
                    let desc_ty = DescriptorType::StorageBuffer(nbind, ty.clone());
                    let access = self.get_desc_access(op.var_id)
                        .ok_or(Error::ACCESS_CONFLICT)?;
                    Variable::Descriptor(desc_bind, desc_ty, access)
                } else {
                    let desc_ty = DescriptorType::UniformBuffer(nbind, ty.clone());
                    let access = AccessType::ReadOnly;
                    Variable::Descriptor(desc_bind, desc_ty, access)
                };
                Some(var)
            },
            StorageClass::StorageBuffer => {
                let (nbind, ty) = extract_proto_ty(ty)?;
                let desc_bind = self.get_var_desc_bind_or_default(op.var_id);
                let desc_ty = DescriptorType::StorageBuffer(nbind, ty.clone());
                let access = self.get_desc_access(op.var_id)
                    .ok_or(Error::ACCESS_CONFLICT)?;
                let var = Variable::Descriptor(desc_bind, desc_ty, access);
                Some(var)
            },
            StorageClass::UniformConstant => {
                let (nbind, ty) = extract_proto_ty(ty)?;
                let desc_bind = self.get_var_desc_bind_or_default(op.var_id);
                let var = match ty {
                    Type::Image(_) => {
                        let desc_ty = DescriptorType::Image(nbind, ty.clone());
                        let access = self.get_desc_access(op.var_id)
                            .ok_or(Error::ACCESS_CONFLICT)?;
                        Variable::Descriptor(desc_bind, desc_ty, access)
                    },
                    Type::Sampler() => {
                        let desc_ty = DescriptorType::Sampler(nbind);
                        let access = AccessType::ReadOnly;
                        Variable::Descriptor(desc_bind, desc_ty, access)
                    },
                    Type::SampledImage(_) => {
                        let desc_ty = DescriptorType::SampledImage(nbind, ty.clone());
                        let access = AccessType::ReadOnly;
                        Variable::Descriptor(desc_bind, desc_ty, access)
                    },
                    Type::SubpassData(_) => {
                        let input_attm_idx = self
                            .get_deco_u32(op.var_id, Decoration::InputAttachmentIndex)
                            .ok_or(Error::MISSING_DECO)?;
                        let desc_ty = DescriptorType::InputAttachment(nbind, ty.clone(), input_attm_idx);
                        let access = AccessType::ReadOnly;
                        Variable::Descriptor(desc_bind, desc_ty, access)
                    },
                    Type::AccelStruct() => {
                        let desc_ty = DescriptorType::AccelStruct(nbind);
                        let access = AccessType::ReadOnly;
                        Variable::Descriptor(desc_bind, desc_ty, access)
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
    fn collect_entry_point_manifest(&self, func_id: FunctionId) -> Result<Manifest> {
        let mut manifest = Manifest::default();
        let accessed_var_ids = self.collect_fn_vars(func_id);
        let accessed_var_ids = accessed_var_ids.into_iter().collect::<HashSet<_>>();
        for accessed_var_id in accessed_var_ids {
            // Sometimes this process would meet interface variables without
            // locations. These are should built-ins otherwise the SPIR-V is
            // corrupted. Since we assume the SPIR-V is valid and we don't
            // collect built-in variable as useful information, we simply ignore
            // such null-references.
            if let Some(accessed_var) = self.get_var(accessed_var_id) {
                let name = self.get_name(accessed_var_id);
                manifest.insert_var(accessed_var.clone(), name)?;
            }
        }
        Ok(manifest)
    }
    fn collect_entry_point_spec(&self) -> Result<Specialization> {
        // TODO: (penguinlion) Report only specialization constants that have
        // been refered to by the specified function. (Do we actually need this?
        // It might not be an optimization in mind of engineering.)
        let mut spec = Specialization::default();
        for spec_const in self.spec_consts().iter() {
            let ty = self.get_ty(spec_const.ty_id)
                .ok_or(Error::TY_NOT_FOUND)?;
            let locator = Locator::SpecConstant(spec_const.spec_id);
            let name = self.get_var_name(locator);
            spec.insert_spec_const(spec_const.spec_id, ty.clone(), name)?;
        }
        Ok(spec)
    }
    pub(crate) fn collect_entry_points(&self) -> Result<Vec<EntryPoint>> {
        let mut entry_points = Vec::with_capacity(self.entry_point_declrs().len());
        for entry_point_declr in self.entry_point_declrs().iter() {
            let manifest = self.collect_entry_point_manifest(entry_point_declr.func_id)?;
            let spec = self.collect_entry_point_spec()?;
            let entry_point = EntryPoint {
                name: entry_point_declr.name.to_owned(),
                exec_model: entry_point_declr.exec_model,
                manifest,
                spec,
            };
            entry_points.push(entry_point);
        }
        Ok(entry_points)
    }
    /// Collect resources discovered in this reflection session, no matter if
    /// it's been used by any entry point.
    pub(crate) fn collect_module_as_entry_point(&self) -> Result<EntryPoint> {
        let mut manifest = Manifest::default();
        for accessed_var in self.vars().iter() {
            let locator = accessed_var.locator();
            let name = self.get_var_name(locator);
            manifest.insert_var(accessed_var.clone(), name)?;
        }
        let spec = self.collect_entry_point_spec()?;

        let entry_point_declrs = self.entry_point_declrs();
        if entry_point_declrs.len() != 1 {
            return Err(Error::MULTI_ENTRY_POINTS);
        }
        let entry_point_declr = &entry_point_declrs[0];
        let entry_point = EntryPoint {
            name: entry_point_declr.name.to_owned(),
            exec_model: entry_point_declr.exec_model,
            manifest,
            spec,
        };
        Ok(entry_point)
    }
}
