//! Reflection procedures and types.
use std::collections::BTreeMap;
use std::convert::TryFrom;

use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use num_traits::FromPrimitive;

use crate::{
    annotation::{DecorationRegistry, NameRegistry},
    constant::{Constant, ConstantValue},
    entry_point::{EntryPoint, ExecutionModel},
    error::{anyhow, Error, Result},
    evaluator::Evaluator,
    func::{ExecutionMode, Function, FunctionRegistry},
    inspect::Inspector,
    instr::*,
    parse::{Instr, Instrs},
    reflect_cfg::ReflectConfig,
    spirv::{self, Op},
    ty::{
        AccelStructType, AccessType, ArrayType, CombinedImageSamplerType, DescriptorType,
        DeviceAddressType, ImageType, MatrixAxisOrder, MatrixType, PointerType, RayQueryType,
        SampledImageType, SamplerType, ScalarType, StorageClass, StorageImageType, StructMember,
        StructType, SubpassDataType, Type, TypeRegistry, VectorType,
    },
    var::{Variable, VariableAlloc, VariableRegistry},
};

type ConstantId = u32;
type FunctionId = u32;
type TypeId = u32;
type VariableId = u32;

// Intermediate types used in reflection.

struct EntryPointDeclartion<'a> {
    name: &'a str,
    exec_model: ExecutionModel,
    exec_modes: Vec<ExecutionModeDeclaration>,
}
enum ExecutionModeOperand {
    Literal(u32),
    Id(ConstantId),
}
struct ExecutionModeDeclaration {
    func_id: FunctionId,
    exec_mode: spirv::ExecutionMode,
    operands: Vec<ExecutionModeOperand>,
}

// The actual reflection to take place.

fn is_ty_op(op: Op) -> bool {
    match op {
        Op::TypeVoid => true,
        Op::TypeBool => true,
        Op::TypeInt => true,
        Op::TypeFloat => true,
        Op::TypeVector => true,
        Op::TypeMatrix => true,
        Op::TypeImage => true,
        Op::TypeSampler => true,
        Op::TypeSampledImage => true,
        Op::TypeArray => true,
        Op::TypeRuntimeArray => true,
        Op::TypeStruct => true,
        Op::TypeOpaque => true,
        Op::TypePointer => true,
        Op::TypeFunction => true,
        Op::TypeEvent => true,
        Op::TypeDeviceEvent => true,
        Op::TypeReserveId => true,
        Op::TypeQueue => true,
        Op::TypePipe => true,
        Op::TypeForwardPointer => true,
        Op::TypePipeStorage => true,
        Op::TypeNamedBarrier => true,
        Op::TypeRayQueryKHR => true,
        Op::TypeAccelerationStructureKHR => true,
        Op::TypeCooperativeMatrixNV => true,
        Op::TypeVmeImageINTEL => true,
        Op::TypeAvcImePayloadINTEL => true,
        Op::TypeAvcRefPayloadINTEL => true,
        Op::TypeAvcSicPayloadINTEL => true,
        Op::TypeAvcMcePayloadINTEL => true,
        Op::TypeAvcMceResultINTEL => true,
        Op::TypeAvcImeResultINTEL => true,
        Op::TypeAvcImeResultSingleReferenceStreamoutINTEL => true,
        Op::TypeAvcImeResultDualReferenceStreamoutINTEL => true,
        Op::TypeAvcImeSingleReferenceStreaminINTEL => true,
        Op::TypeAvcImeDualReferenceStreaminINTEL => true,
        Op::TypeAvcRefResultINTEL => true,
        Op::TypeAvcSicResultINTEL => true,
        _ => false,
    }
}
fn is_const_op(op: Op) -> bool {
    match op {
        Op::ConstantTrue => true,
        Op::ConstantFalse => true,
        Op::Constant => true,
        Op::ConstantComposite => true,
        Op::ConstantSampler => true,
        Op::ConstantNull => true,
        Op::ConstantPipeStorage => true,
        Op::SpecConstantTrue => true,
        Op::SpecConstantFalse => true,
        Op::SpecConstant => true,
        Op::SpecConstantComposite => true,
        Op::SpecConstantOp => true,
        _ => false,
    }
}
fn is_atomic_load_op(op: Op) -> bool {
    match op {
        Op::AtomicLoad => true,
        Op::AtomicExchange => true,
        Op::AtomicCompareExchange => true,
        Op::AtomicCompareExchangeWeak => true,
        Op::AtomicIIncrement => true,
        Op::AtomicIDecrement => true,
        Op::AtomicIAdd => true,
        Op::AtomicISub => true,
        Op::AtomicSMin => true,
        Op::AtomicUMin => true,
        Op::AtomicSMax => true,
        Op::AtomicUMax => true,
        Op::AtomicAnd => true,
        Op::AtomicOr => true,
        Op::AtomicXor => true,
        _ => false,
    }
}
fn is_atomic_store_op(op: Op) -> bool {
    match op {
        Op::AtomicStore => true,
        _ => false,
    }
}

/// SPIR-V reflection intermediate.
pub struct ReflectIntermediate<'a> {
    pub cfg: &'a ReflectConfig,
    pub name_reg: NameRegistry<'a>,
    pub deco_reg: DecorationRegistry<'a>,
    pub ty_reg: TypeRegistry,
    pub var_reg: VariableRegistry,
    pub func_reg: FunctionRegistry,
    pub interp: Evaluator,
}
impl<'a> ReflectIntermediate<'a> {
    pub fn new(cfg: &'a ReflectConfig) -> Self {
        ReflectIntermediate {
            cfg,

            name_reg: Default::default(),
            deco_reg: Default::default(),
            ty_reg: Default::default(),
            var_reg: Default::default(),
            func_reg: Default::default(),
            interp: Default::default(),
        }
    }
}
fn broken_nested_ty(id: TypeId) -> Error {
    Error::msg(format!("broken nested type: {}", id))
}
impl<'a> ReflectIntermediate<'a> {
    fn populate_one_ty(&mut self, instr: &Instr) -> Result<()> {
        match instr.op() {
            Op::TypeFunction => {}
            Op::TypeVoid => {
                let op = OpTypeVoid::try_from(instr)?;
                let scalar_ty = ScalarType::Void;
                self.ty_reg.set(op.ty_id, Type::Scalar(scalar_ty))?;
            }
            Op::TypeBool => {
                let op = OpTypeBool::try_from(instr)?;
                let scalar_ty = ScalarType::Boolean;
                self.ty_reg.set(op.ty_id, Type::Scalar(scalar_ty))?;
            }
            Op::TypeInt => {
                let op = OpTypeInt::try_from(instr)?;
                let scalar_ty = ScalarType::Integer {
                    bits: op.bits,
                    is_signed: op.is_signed,
                };
                self.ty_reg.set(op.ty_id, Type::Scalar(scalar_ty))?;
            }
            Op::TypeFloat => {
                let op = OpTypeFloat::try_from(instr)?;
                let scalar_ty = ScalarType::Float { bits: op.bits };
                self.ty_reg.set(op.ty_id, Type::Scalar(scalar_ty))?;
            }
            Op::TypeVector => {
                let op = OpTypeVector::try_from(instr)?;
                if let Type::Scalar(scalar_ty) = self.ty_reg.get(op.scalar_ty_id)? {
                    let vector_ty = VectorType {
                        scalar_ty: scalar_ty.clone(),
                        scalar_count: op.scalar_count,
                    };
                    self.ty_reg.set(op.ty_id, Type::Vector(vector_ty))?;
                } else {
                    return Err(broken_nested_ty(op.ty_id));
                }
            }
            Op::TypeMatrix => {
                let op = OpTypeMatrix::try_from(instr)?;
                if let Type::Vector(vector_ty) = self.ty_reg.get(op.vector_ty_id)? {
                    let mat_ty = MatrixType {
                        vector_ty: vector_ty.clone(),
                        vector_count: op.vector_count,
                        axis_order: None,
                        stride: None,
                    };
                    self.ty_reg.set(op.ty_id, Type::Matrix(mat_ty))?;
                } else {
                    return Err(broken_nested_ty(op.ty_id));
                }
            }
            Op::TypeImage => {
                let op = OpTypeImage::try_from(instr)?;
                let scalar_ty = match self.ty_reg.get(op.scalar_ty_id)? {
                    Type::Scalar(scalar_ty) => scalar_ty.clone(),
                    _ => return Err(broken_nested_ty(op.ty_id)),
                };
                if op.dim == spirv::Dim::DimSubpassData {
                    let subpass_data_ty = SubpassDataType {
                        scalar_ty,
                        is_multisampled: op.is_multisampled,
                    };
                    self.ty_reg
                        .set(op.ty_id, Type::SubpassData(subpass_data_ty))?;
                } else {
                    // Only unit types allowed to be stored in storage images
                    // can have given format.
                    let is_sampled = match op.is_sampled {
                        0 => None,
                        1 => Some(true),
                        2 => Some(false),
                        _ => return Err(anyhow!("unsupported image sampling type")),
                    };
                    let is_depth = match op.is_depth {
                        0 => Some(false),
                        1 => Some(true),
                        2 => None,
                        _ => return Err(anyhow!("unsupported image depth type")),
                    };
                    let image_ty = ImageType {
                        scalar_ty,
                        dim: op.dim,
                        is_depth,
                        is_array: op.is_array,
                        is_multisampled: op.is_multisampled,
                        is_sampled,
                        fmt: op.color_fmt,
                    };
                    self.ty_reg.set(op.ty_id, Type::Image(image_ty))?;
                }
            }
            Op::TypeSampler => {
                let op = OpTypeSampler::try_from(instr)?;
                // Note that SPIR-V doesn't discriminate color and depth/stencil
                // samplers. `sampler` and `samplerShadow` means the same thing.
                self.ty_reg.set(op.ty_id, Type::Sampler(SamplerType {}))?;
            }
            Op::TypeSampledImage => {
                let op = OpTypeSampledImage::try_from(instr)?;
                if let Type::Image(image_ty) = self.ty_reg.get(op.image_ty_id)? {
                    let sampled_image_ty = SampledImageType {
                        scalar_ty: image_ty.scalar_ty.clone(),
                        dim: image_ty.dim,
                        is_array: image_ty.is_array,
                        is_multisampled: image_ty.is_multisampled,
                    };
                    let combined_img_sampler_ty = CombinedImageSamplerType { sampled_image_ty };
                    self.ty_reg.set(
                        op.ty_id,
                        Type::CombinedImageSampler(combined_img_sampler_ty),
                    )?;
                } else {
                    return Err(broken_nested_ty(op.ty_id));
                }
            }
            Op::TypeArray => {
                let op = OpTypeArray::try_from(instr)?;
                // FIXME: Workaround old storage buffers.
                if self
                    .deco_reg
                    .contains(op.element_ty_id, spirv::Decoration::BufferBlock)
                {
                    let _ = self.deco_reg.set(
                        op.ty_id,
                        spirv::Decoration::BufferBlock,
                        &[] as &'static [u32],
                    );
                }
                let element_ty = if let Ok(x) = self.ty_reg.get(op.element_ty_id) {
                    x
                } else {
                    return Ok(());
                };

                // Some notes about specialization constants.
                //
                // Using specialization constants for array sizes might lead
                // to UNDEFINED BEHAVIOR because structure size MUST be
                // definitive at compile time and CANNOT be specialized at
                // runtime according to Khronos members, but the default
                // behavior of `glslang` is to treat the specialization
                // constants as normal constants, then I would say...
                // probably it's fine to size array with them?
                let element_count = match self.interp.get_value(op.element_count_const_id)? {
                    ConstantValue::S32(x) if *x > 0 => *x as u32,
                    ConstantValue::U32(x) if *x > 0 => *x,
                    _ => return Err(anyhow!("invalid array size")),
                };
                let stride = self
                    .deco_reg
                    .get_u32(op.ty_id, spirv::Decoration::ArrayStride)
                    .map(|x| x as usize);

                let arr_ty = if let Ok(stride) = stride {
                    // Sized data arrays.
                    ArrayType {
                        element_ty: Box::new(element_ty.clone()),
                        element_count: Some(element_count),
                        stride: Some(stride),
                    }
                } else {
                    // Multiple descriptor binding points grouped into an array.
                    ArrayType {
                        element_ty: Box::new(element_ty.clone()),
                        element_count: Some(element_count),
                        stride: None,
                    }
                };
                self.ty_reg.set(op.ty_id, Type::Array(arr_ty))?;
            }
            Op::TypeRuntimeArray => {
                let op = OpTypeRuntimeArray::try_from(instr)?;
                let element_ty = if let Ok(x) = self.ty_reg.get(op.element_ty_id) {
                    x
                } else {
                    return Ok(());
                };
                let stride = self
                    .deco_reg
                    .get_u32(op.ty_id, spirv::Decoration::ArrayStride)
                    .map(|x| x as usize);
                let arr_ty = if let Ok(stride) = stride {
                    // Unsized data arrays.
                    ArrayType {
                        element_ty: Box::new(element_ty.clone()),
                        element_count: None,
                        stride: Some(stride),
                    }
                } else {
                    // Multiple descriptor binding points grouped into an array
                    // whose size is unknown at compile time.
                    ArrayType {
                        element_ty: Box::new(element_ty.clone()),
                        element_count: None,
                        stride: None,
                    }
                };
                self.ty_reg.set(op.ty_id, Type::Array(arr_ty))?;
            }
            Op::TypeStruct => {
                let op = OpTypeStruct::try_from(instr)?;
                let struct_name =
                    self.name_reg
                        .get(op.ty_id)
                        .map(ToOwned::to_owned)
                        .or_else(|| {
                            if self.cfg.gen_unique_names {
                                Some(format!("type_{}", op.ty_id))
                            } else {
                                None
                            }
                        });
                let mut members = Vec::new();
                for (i, &member_ty_id) in op.member_ty_ids.iter().enumerate() {
                    let i = i as u32;
                    let mut member_ty = if let Ok(member_ty) = self.ty_reg.get(member_ty_id) {
                        member_ty.clone()
                    } else {
                        return Ok(());
                    };
                    let mut element_ty = &mut member_ty;
                    while let Type::Array(arr_ty) = element_ty {
                        element_ty = &mut *arr_ty.element_ty;
                    }
                    if let Type::Matrix(ref mut mat_ty) = element_ty {
                        let mat_stride = self.deco_reg.get_member_u32(
                            op.ty_id,
                            i,
                            spirv::Decoration::MatrixStride,
                        );
                        if let Ok(mat_stride) = mat_stride {
                            mat_ty.stride = Some(mat_stride as usize);
                        }

                        let is_row_major =
                            self.deco_reg
                                .contains_member(op.ty_id, i, spirv::Decoration::RowMajor);
                        let is_col_major =
                            self.deco_reg
                                .contains_member(op.ty_id, i, spirv::Decoration::ColMajor);

                        mat_ty.axis_order = if is_row_major {
                            Some(MatrixAxisOrder::RowMajor)
                        } else if is_col_major {
                            Some(MatrixAxisOrder::ColumnMajor)
                        } else {
                            None
                        };
                    }
                    let name = self
                        .name_reg
                        .get_member(op.ty_id, i)
                        .map(ToOwned::to_owned)
                        .or_else(|| {
                            if self.cfg.gen_unique_names {
                                Some(format!("type_{}_member_{}", op.ty_id, i))
                            } else {
                                None
                            }
                        });
                    // For shader input/output blocks there are no offset
                    // decoration. Since these variables are not externally
                    // accessible we don't have to worry about them.
                    let offset = self
                        .deco_reg
                        .get_member_u32(op.ty_id, i, spirv::Decoration::Offset)
                        .map(|x| x as usize)
                        .ok();
                    let access_ty = self
                        .deco_reg
                        .get_member_access_ty_from_deco(op.ty_id, i)
                        .ok_or_else(|| anyhow!("missing access type"))?;
                    let member = StructMember {
                        name,
                        offset,
                        ty: member_ty.clone(),
                        access_ty,
                    };
                    members.push(member);
                }
                let struct_ty = StructType {
                    name: struct_name,
                    members: members,
                };
                // Don't have to shrink-to-fit because the types in `ty_map`
                // won't be used directly and will be cloned later.
                self.ty_reg.set(op.ty_id, Type::Struct(struct_ty))?;
            }
            Op::TypePointer => {
                let op = OpTypePointer::try_from(instr)?;
                if let Ok(pointee_ty) = self.ty_reg.get(op.target_ty_id) {
                    // Before SPIR-V 1.3, there is no `StorageBuffer` storage
                    // class. And from a pointer perspective you can't tell if
                    // it's a uniform block or a buffer block.
                    let is_storage_buffer = self
                        .deco_reg
                        .contains(op.target_ty_id, spirv::Decoration::BufferBlock);
                    let store_cls = if op.store_cls == StorageClass::Uniform && is_storage_buffer {
                        StorageClass::StorageBuffer
                    } else {
                        op.store_cls
                    };
                    let pointer_ty = PointerType {
                        pointee_ty: Box::new(pointee_ty.clone()),
                        store_cls,
                    };
                    self.ty_reg.set(op.ty_id, Type::DevicePointer(pointer_ty))?;
                } else {
                    // Ignore unknown types. Currently only funtion pointers can
                    // step into this.
                    return Ok(());
                }
            }
            Op::TypeForwardPointer => {
                let op = OpTypeForwardPointer::try_from(instr)?;
                self.ty_reg
                    .set(op.ty_id, Type::DeviceAddress(DeviceAddressType {}))?;
            }
            Op::TypeAccelerationStructureKHR => {
                let op = OpTypeAccelerationStructureKHR::try_from(instr)?;
                self.ty_reg
                    .set(op.ty_id, Type::AccelStruct(AccelStructType {}))?;
            }
            Op::TypeRayQueryKHR => {
                let op = OpTypeRayQueryKHR::try_from(instr)?;
                self.ty_reg.set(op.ty_id, Type::RayQuery(RayQueryType {}))?;
            }
            _ => return Err(anyhow!("unexpected opcode {:?}", instr.op())),
        }
        Ok(())
    }
    fn populate_one_const(&mut self, instr: &Instr) -> Result<()> {
        let opcode = instr.op();
        match opcode {
            Op::ConstantTrue | Op::ConstantFalse | Op::Constant => {
                let op = OpConstantScalarCommonSPQ::try_from(instr)?;
                let ty = self.ty_reg.get(op.ty_id)?.clone();
                let value = match instr.op() {
                    Op::ConstantTrue => ConstantValue::from(true),
                    Op::ConstantFalse => ConstantValue::from(false),
                    Op::Constant => ConstantValue::try_from_dwords(op.value, &ty)?,
                    _ => return Ok(()),
                };
                let name = self
                    .name_reg
                    .get(op.const_id)
                    .map(ToOwned::to_owned)
                    .or_else(|| {
                        if self.cfg.gen_unique_names {
                            Some(format!("const_{}", op.const_id))
                        } else {
                            None
                        }
                    });
                let constant = Constant::new(name, ty, value);
                self.interp.set(op.const_id, constant)?;
                Ok(())
            }
            Op::ConstantComposite
            | Op::ConstantSampler
            | Op::ConstantNull
            | Op::ConstantPipeStorage => Ok(()),
            Op::SpecConstantTrue | Op::SpecConstantFalse | Op::SpecConstant => {
                let op = OpConstantScalarCommonSPQ::try_from(instr)?;
                let name = self.name_reg.get(op.const_id).map(ToString::to_string);
                let spec_id = self
                    .deco_reg
                    .get_u32(op.const_id, spirv::Decoration::SpecId)?;
                let ty = self.ty_reg.get(op.ty_id)?.clone();
                let constant = if let Some(user_value) = self.cfg.spec_values.get(&spec_id) {
                    Constant::new(name, ty, user_value.clone())
                } else {
                    let value = match opcode {
                        Op::SpecConstantTrue => ConstantValue::from(true),
                        Op::SpecConstantFalse => ConstantValue::from(false),
                        Op::SpecConstant => ConstantValue::try_from_dwords(op.value, &ty)?,
                        _ => unreachable!(),
                    };
                    Constant::new_spec(name, ty, value, spec_id)
                };
                self.interp.set(op.const_id, constant)?;
                Ok(())
            }
            // `SpecId` decorations will be specified to each of the constituents so we don't have to register a `Constant` for the composite of them. `Constant` is registered only for those will be interacting with Vulkan.
            Op::SpecConstantComposite => Ok(()),
            Op::SpecConstantOp => {
                let op = OpSpecConstantHeadSPQ::try_from(instr)?;
                let opcode = Op::from_u32(op.opcode)
                    .ok_or_else(|| anyhow!("invalid specialization constant op opcode"))?;
                let result_id = op.spec_const_id;
                let result_ty = self.ty_reg.get(op.ty_id)?;
                self.interp
                    .interpret(opcode, result_id, result_ty, &instr.as_ref()[4..])?;
                Ok(())
            }
            _ => Err(anyhow!("unexpected opcode {:?}", instr.op())),
        }
    }
    fn populate_one_var(&mut self, instr: &Instr) -> Result<()> {
        let op = OpVariable::try_from(instr)?;
        let ptr_ty = if let Ok(ty) = self.ty_reg.get(op.ty_id) {
            match ty {
                Type::DevicePointer(ptr_ty) => ptr_ty.clone(),
                _ => return Err(broken_nested_ty(op.ty_id)),
            }
        } else {
            return Ok(());
        };
        let name = self.name_reg.get(op.var_id).map(ToString::to_string);
        let var = VariableAlloc {
            name,
            ptr_ty,
            store_cls: op.store_cls,
        };
        self.var_reg.set(op.var_id, var)?;
        Ok(())
    }
}

pub struct FunctionInspector {
    cur_func: Option<(FunctionId, Function)>,
    access_chain_map: HashMap<VariableId, VariableId>,
}
impl FunctionInspector {
    pub fn new() -> Self {
        Self {
            cur_func: None,
            access_chain_map: HashMap::default(),
        }
    }
}
impl Inspector for FunctionInspector {
    fn inspect(&mut self, itm: &mut ReflectIntermediate<'_>, instr: &Instr) -> Result<()> {
        let opcode = instr.op();
        match opcode {
            Op::Function => {
                let op = OpFunction::try_from(instr)?;
                let func_id = op.func_id;
                self.cur_func = Some((func_id, Function::default()));
            }
            Op::FunctionEnd => {
                if let Some((func_id, func)) = self.cur_func.take() {
                    itm.func_reg.set(func_id, func)?;
                } else {
                    return Err(anyhow!("unexpected OpFunctionEnd"));
                }
                self.cur_func = None;
            }
            Op::FunctionCall => {
                let op = OpFunctionCall::try_from(instr)?;
                let func_id = op.func_id;
                let func = itm.func_reg.get_mut(func_id)?;
                func.callees.insert(func_id);
            }
            _ => {
                if let Some((_func_id, func)) = self.cur_func.as_mut() {
                    let op = instr.op();
                    if op == Op::AccessChain {
                        let op = OpAccessChain::try_from(instr)?;
                        if self
                            .access_chain_map
                            .insert(op.var_id, op.accessed_var_id)
                            .is_some()
                        {
                            return Err(anyhow!("duplicate access chain at a same id"));
                        }
                    } else if op == Op::Load || is_atomic_load_op(op) {
                        let op = OpLoad::try_from(instr)?;
                        let mut var_id = op.var_id;
                        // Resolve access chain.
                        if let Some(&x) = self.access_chain_map.get(&var_id) {
                            var_id = x
                        }
                        func.accessed_vars.insert(var_id);
                    } else if op == Op::Store || is_atomic_store_op(op) {
                        let op = OpStore::try_from(instr)?;
                        let mut var_id = op.var_id;
                        // Resolve access chain.
                        if let Some(&x) = self.access_chain_map.get(&var_id) {
                            var_id = x
                        }
                        func.accessed_vars.insert(var_id);
                    }
                } else {
                    return Err(anyhow!("unexpected opcode {:?}", instr.op()));
                }
            }
        }
        Ok(())
    }
}

pub fn reflect<'a, I: Inspector>(
    itm: &mut ReflectIntermediate<'a>,
    mut inspector: I,
) -> Result<Vec<EntryPoint>> {
    // Don't change the order. See _2.4 Logical Layout of a Module_ of the
    // SPIR-V specification for more information.
    let mut instrs = Instrs::new(itm.cfg.spv.words()).peekable();

    let mut entry_point_declrs = HashMap::default();

    // 1. All OpCapability instructions.
    while let Some(instr) = instrs.peek().cloned() {
        if instr.op() == Op::Capability {
            instrs.next();
        } else {
            break;
        }
    }
    // 2. Optional OpExtension instructions (extensions to SPIR-V).
    while let Some(instr) = instrs.peek().cloned() {
        if instr.op() == Op::Extension {
            instrs.next();
        } else {
            break;
        }
    }
    // 3. Optional OpExtInstImport instructions.
    while let Some(instr) = instrs.peek().cloned() {
        if instr.op() == Op::ExtInstImport {
            let op = OpExtInstImport::try_from(instr)?;
            itm.interp
                .import_ext_instr_set(op.instr_set_id, op.name.to_owned())?;
            instrs.next();
        } else {
            break;
        }
    }
    // 4. The single required OpMemoryModel instruction.
    if let Some(instr) = instrs.next() {
        if instr.op() == Op::MemoryModel {
            let op = OpMemoryModel::try_from(instr)?;
            match op.addr_model {
                spirv::AddressingModel::Logical => {}
                spirv::AddressingModel::PhysicalStorageBuffer64 => {}
                _ => return Err(anyhow!("unsupported addressing model")),
            }
            match op.mem_model {
                spirv::MemoryModel::GLSL450 => {}
                spirv::MemoryModel::Vulkan => {}
                _ => return Err(anyhow!("unsupported memory model")),
            }
        } else {
            return Err(anyhow!("expected OpMemoryModel, but got {:?}", instr.op()));
        }
    } else {
        return Err(anyhow!("expected OpMemoryModel, but got nothing"));
    }
    // 5. All entry point declarations, using OpEntryPoint.
    while let Some(instr) = instrs.peek().cloned() {
        if instr.op() == Op::EntryPoint {
            let op = OpEntryPoint::try_from(instr)?;
            let entry_point_declr = EntryPointDeclartion {
                exec_model: op.exec_model,
                name: op.name,
                exec_modes: Default::default(),
            };
            use std::collections::hash_map::Entry;
            match entry_point_declrs.entry(op.func_id) {
                Entry::Occupied(_) => return Err(anyhow!("duplicate entry point at a same id")),
                Entry::Vacant(e) => {
                    e.insert(entry_point_declr);
                }
            }
            instrs.next();
        } else {
            break;
        }
    }
    // 6. All execution-mode declarations, using OpExecutionMode or
    //    OpExecutionModeId.
    while let Some(instr) = instrs.peek().cloned() {
        let op = instr.op();
        match op {
            Op::ExecutionMode | Op::ExecutionModeId => {
                let mut operands = instr.operands();
                let operand_ctor = match op {
                    Op::ExecutionMode => |x: &u32| ExecutionModeOperand::Literal(*x),
                    Op::ExecutionModeId => |x: &u32| ExecutionModeOperand::Id(*x),
                    _ => unreachable!(),
                };

                let func_id = operands.read_u32()?;
                let exec_mode = operands.read_enum::<spirv::ExecutionMode>()?;
                let operands = operands
                    .read_list()?
                    .into_iter()
                    .map(operand_ctor)
                    .collect();
                let exec_mode_declr = ExecutionModeDeclaration {
                    func_id,
                    exec_mode,
                    operands,
                };
                entry_point_declrs
                    .get_mut(&func_id)
                    .ok_or(anyhow!("execution mode for non-existing entry point"))?
                    .exec_modes
                    .push(exec_mode_declr);
                instrs.next();
            }
            _ => break,
        }
    }
    // 7. These debug instructions, which must be grouped in the following
    //    order:
    //   a. All OpString, OpSourceExtension, OpSource, and
    //      OpSourceContinued, without forward references.
    //   b. All OpName and all OpMemberName.
    //   c. All OpModuleProcessed instructions.
    while let Some(instr) = instrs.peek().cloned() {
        match instr.op() {
            Op::String
            | Op::SourceExtension
            | Op::Source
            | Op::SourceContinued
            | Op::ModuleProcessed => {
                instrs.next();
            }
            Op::Name => {
                let op = OpName::try_from(instr)?;
                if !op.name.is_empty() {
                    // Ignore empty names.
                    itm.name_reg.set(op.target_id, op.name);
                }
                instrs.next();
            }
            Op::MemberName => {
                let op = OpMemberName::try_from(instr)?;
                if !op.name.is_empty() {
                    itm.name_reg
                        .set_member(op.target_id, op.member_idx, op.name);
                }
                instrs.next();
            }
            _ => break,
        }
    }
    // 8. All annotation instructions:
    //   a. All decoration instructions.
    while let Some(instr) = instrs.peek().cloned() {
        match instr.op() {
            Op::Decorate => {
                let op = OpDecorate::try_from(instr)?;
                let deco = op.deco;
                itm.deco_reg.set(op.target_id, deco, op.params)?;
                instrs.next();
            }
            Op::MemberDecorate => {
                let op = OpMemberDecorate::try_from(instr)?;
                let deco = op.deco;
                itm.deco_reg
                    .set_member(op.target_id, op.member_idx, deco, op.params)?;
                instrs.next();
            }
            Op::DecorationGroup
            | Op::GroupDecorate
            | Op::GroupMemberDecorate
            | Op::DecorateId
            | Op::DecorateString
            | Op::MemberDecorateString => {
                instrs.next();
            }
            _ => break,
        };
    }
    // 9. All type declarations (OpTypeXXX instructions), all constant
    //    instructions, and all global variable declarations (all OpVariable
    //    instructions whose Storage Class is not Function). This is the
    //    preferred location for OpUndef instructions, though they can also
    //    appear in function bodies. All operands in all these instructions
    //    must be declared before being used. Otherwise, they can be in any
    //    order. This section is the first section to allow use of:
    //   a. OpLine and OpNoLine debug information.
    //   b. Non-semantic instructions with OpExtInst.
    while let Some(instr) = instrs.peek().cloned() {
        let opcode = instr.op();
        if let Op::Line | Op::NoLine = opcode {
            instrs.next();
            continue;
        }
        if is_ty_op(opcode) {
            itm.populate_one_ty(instr)?;
        } else if opcode == Op::Variable {
            itm.populate_one_var(instr)?;
        } else if is_const_op(opcode) {
            itm.populate_one_const(instr)?;
        } else {
            break;
        }
        instrs.next();
    }
    // 10. All function declarations ("declarations" are functions without a
    //     body; there is no forward declaration to a function with a body).
    //     A function declaration is as follows.
    //   a. Function declaration, using OpFunction.
    //   b. Function parameter declarations, using OpFunctionParameter.
    //   c. Function end, using OpFunctionEnd.
    // 11. All function definitions (functions with a body). A function
    //     definition is as follows.
    //   a. Function definition, using OpFunction.
    //   b. Function parameter declarations, using OpFunctionParameter.
    //   c. Block.
    //   d. Block.
    //   e. ...
    //   f. Function end, using OpFunctionEnd.
    while let Some(instr) = instrs.peek().cloned() {
        let opcode = instr.op();
        if let Op::Line | Op::NoLine = opcode {
            instrs.next();
            continue;
        }
        inspector.inspect(itm, instr)?;
        instrs.next();
    }

    itm.collect_entry_points(entry_point_declrs)
}

fn make_desc_var(
    deco_reg: &DecorationRegistry,
    name: Option<String>,
    var_id: VariableId,
    ptr_ty: &PointerType,
    ty: &Type,
) -> Option<Variable> {
    // Unwrap multi-binding.
    let (bind_count, ty) = match ty {
        Type::Array(arr_ty) => {
            // `nrepeat=None` is no longer considered invalid because of
            // the adoption of `SPV_EXT_descriptor_indexing`. This
            // shader extension has been supported in Vulkan 1.2.
            let nrepeat = arr_ty.element_count.unwrap_or(0);
            (nrepeat, &*arr_ty.element_ty)
        }
        _ => (1, ty),
    };

    // Elevate image type to concrete storage/sampled image type.
    let ty = match ty {
        Type::Image(image_ty) => {
            if let Some(false) = image_ty.is_sampled {
                // Guaranteed a storage image.
                let storage_image_ty = StorageImageType {
                    dim: image_ty.dim,
                    is_array: image_ty.is_array,
                    is_multisampled: image_ty.is_multisampled,
                    fmt: image_ty.fmt,
                };
                Type::StorageImage(storage_image_ty)
            } else {
                // Potentially a sampled image.
                let sampled_image_ty = SampledImageType {
                    dim: image_ty.dim,
                    scalar_ty: image_ty.scalar_ty.clone(),
                    is_array: image_ty.is_array,
                    is_multisampled: image_ty.is_multisampled,
                };
                Type::SampledImage(sampled_image_ty)
            }
        }
        _ => ty.clone(),
    };

    let desc_bind = deco_reg.get_var_desc_bind_or_default(var_id);
    let desc_ty = match &ty {
        Type::Struct(_) => {
            // Compatibility for SPIR-V <= 1.3 is done when
            // extracting storage class deco for pointer types.
            if ptr_ty.store_cls == StorageClass::StorageBuffer {
                let access = deco_reg
                    .get_desc_access_ty(var_id, &ty)
                    .unwrap_or(AccessType::ReadWrite);
                DescriptorType::StorageBuffer(access)
            } else {
                DescriptorType::UniformBuffer
            }
        }
        Type::SampledImage(sampled_image_ty) => match sampled_image_ty.dim {
            spirv::Dim::DimBuffer => DescriptorType::UniformTexelBuffer,
            _ => DescriptorType::SampledImage,
        },
        Type::StorageImage(store_image_ty) => {
            let access = deco_reg
                .get_desc_access_ty(var_id, &ty)
                .unwrap_or(AccessType::ReadWrite);
            match store_image_ty.dim {
                spirv::Dim::DimBuffer => DescriptorType::StorageTexelBuffer(access),
                _ => DescriptorType::StorageImage(access),
            }
        }
        Type::Sampler(_) => DescriptorType::Sampler,
        Type::CombinedImageSampler(combined_img_sampler_ty) => {
            match combined_img_sampler_ty.sampled_image_ty.dim {
                spirv::Dim::DimBuffer => DescriptorType::UniformTexelBuffer,
                _ => DescriptorType::CombinedImageSampler,
            }
        }
        Type::SubpassData(_) => {
            let input_attm_idx = deco_reg.get_var_input_attm_idx(var_id).unwrap_or_default();
            DescriptorType::InputAttachment(input_attm_idx)
        }
        Type::AccelStruct(_) => DescriptorType::AccelStruct,
        _ => return None,
    };
    let var = Variable::Descriptor {
        name,
        desc_bind,
        desc_ty,
        ty,
        bind_count,
    };
    Some(var)
}
fn make_var<'a>(
    deco_reg: &DecorationRegistry<'a>,
    name: Option<String>,
    var_id: VariableId,
    var_alloc: &VariableAlloc,
) -> Option<Variable> {
    let ptr_ty = &var_alloc.ptr_ty;
    let ty = &*ptr_ty.pointee_ty;
    // Note that the storage class of a variable must be the same as the
    // pointer.
    match ptr_ty.store_cls {
        StorageClass::Input => {
            if let Ok(location) = deco_reg.get_var_location(var_id) {
                let var = Variable::Input {
                    name,
                    location,
                    ty: ty.clone(),
                };
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
        }
        StorageClass::Output => {
            if let Ok(location) = deco_reg.get_var_location(var_id) {
                let var = Variable::Output {
                    name,
                    location,
                    ty: ty.clone(),
                };
                Some(var)
            } else {
                None
            }
        }
        StorageClass::PushConstant => {
            // Push constants have no global offset. Offsets are applied to
            // members.
            if let Type::Struct(_) = ty {
                let var = Variable::PushConstant {
                    name,
                    ty: ty.clone(),
                };
                Some(var)
            } else {
                None
            }
        }
        StorageClass::Uniform | StorageClass::StorageBuffer | StorageClass::UniformConstant => {
            let var = make_desc_var(&deco_reg, name, var_id, &ptr_ty, ty)?;
            Some(var)
        }
        _ => {
            // Leak out unknown storage classes.
            None
        }
    }
}
impl<'a> ReflectIntermediate<'a> {
    fn collect_vars_impl(&self) -> BTreeMap<VariableId, Variable> {
        // `BTreeMap` to ensure a stable order.
        let mut vars = BTreeMap::new();
        for (var_id, var_alloc) in self.var_reg.iter() {
            let name = self
                .name_reg
                .get(*var_id)
                .map(ToOwned::to_owned)
                .or_else(|| {
                    if self.cfg.gen_unique_names {
                        Some(format!("var_{}", var_id))
                    } else {
                        None
                    }
                });
            if let Some(var) = make_var(&self.deco_reg, name, *var_id, var_alloc) {
                vars.insert(*var_id, var);
            }
        }
        vars
    }
    fn collect_vars(&self) -> Vec<Variable> {
        self.collect_vars_impl()
            .into_iter()
            .map(|(_, var)| var)
            .collect()
    }

    fn collect_entry_point_vars(&self, func_id: FunctionId) -> Vec<Variable> {
        let accessed_var_ids = self
            .func_reg
            .collect_fn_vars(func_id)
            .into_iter()
            .collect::<HashSet<_>>();
        let vars = self
            .collect_vars_impl()
            .into_iter()
            .filter_map(|(var_id, var)| {
                if accessed_var_ids.contains(&var_id) {
                    Some(var)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        vars
    }
    fn collect_entry_point_specs(&self) -> Result<Vec<Variable>> {
        // TODO: (penguinlion) Report only specialization constants that have
        // been refered to by the specified function. (Do we actually need this?
        // It might not be an optimization in mind of engineering.)
        let mut vars = Vec::new();
        for constant in self.interp.constants() {
            if let Some(spec_id) = constant.spec_id {
                let var = Variable::SpecConstant {
                    name: constant.name.clone(),
                    spec_id,
                    ty: constant.ty.clone(),
                };
                vars.push(var);
            }
        }
        Ok(vars)
    }
    fn collect_exec_modes(
        &self,
        func_id: FunctionId,
        exec_mode_declrs: &[ExecutionModeDeclaration],
    ) -> Result<Vec<ExecutionMode>> {
        let mut exec_modes = Vec::with_capacity(exec_mode_declrs.len());

        for declr in exec_mode_declrs.iter() {
            if declr.func_id != func_id {
                continue;
            }

            let mut operands = Vec::with_capacity(declr.operands.len());
            for operand in declr.operands.iter() {
                let operand = match operand {
                    ExecutionModeOperand::Literal(x) => {
                        let scalar_ty = ScalarType::u32();
                        let ty = Type::Scalar(scalar_ty);
                        let value = ConstantValue::from(*x);
                        Constant::new_itm(ty, value)
                    }
                    ExecutionModeOperand::Id(x) => self.interp.get(*x)?.clone(),
                };
                operands.push(operand);
            }

            let exec_mode = ExecutionMode {
                exec_mode: declr.exec_mode,
                operands,
            };
            exec_modes.push(exec_mode)
        }

        Ok(exec_modes)
    }
}

/// Merge `DescriptorType::SampledImage` and `DescriptorType::Sampler` if
/// they are bound to a same binding point with a same number of bindings.
fn combine_img_samplers(vars: Vec<Variable>) -> Vec<Variable> {
    let mut samplers = Vec::<Variable>::new();
    let mut imgs = Vec::<Variable>::new();
    let mut out_vars = Vec::<Variable>::new();

    for var in vars {
        match &var {
            Variable::Descriptor { desc_ty: DescriptorType::Sampler, .. } => {
                samplers.push(var.clone());
                continue;
            }
            Variable::Descriptor { desc_ty: DescriptorType::SampledImage, .. } => {
                imgs.push(var.clone());
                continue;
            }
            _ => {}
        }
        out_vars.push(var);
    }

    for sampler_var in samplers {
        let mut combined_imgs = Vec::new();
        imgs = imgs
            .drain(..)
            .filter_map(|image_var| {
                match (&sampler_var, &image_var) {
                    (
                        Variable::Descriptor { desc_bind: sampler_desc_bind, bind_count: sampler_bind_count, .. },
                        Variable::Descriptor { desc_bind: image_desc_bind, bind_count: image_bind_count, .. },
                    ) if sampler_desc_bind == image_desc_bind && sampler_bind_count == image_bind_count => {
                        combined_imgs.push(image_var.clone());
                        None
                    },
                    _ => {
                        Some(image_var)
                    }
                }
            })
            .collect();

        if combined_imgs.is_empty() {
            // If the sampler can be combined with no texture, just put it
            // back.
            out_vars.push(sampler_var.clone());
        } else {
            // For any texture that can be combined with this sampler,
            // create a new combined image sampler.
            for img_var in combined_imgs {
                match img_var {
                    Variable::Descriptor { name, ty: Type::SampledImage(image_ty), desc_bind, bind_count, .. } => {
                        let sampled_image_ty = SampledImageType {
                            scalar_ty: image_ty.scalar_ty.clone(),
                            dim: image_ty.dim,
                            is_array: image_ty.is_array,
                            is_multisampled: image_ty.is_multisampled,
                        };
                        let combined_img_sampler_ty = CombinedImageSamplerType { sampled_image_ty };
                        let out_var = Variable::Descriptor {
                            name: name.clone(),
                            desc_bind: desc_bind,
                            desc_ty: DescriptorType::CombinedImageSampler,
                            ty: Type::CombinedImageSampler(combined_img_sampler_ty.clone()),
                            bind_count: bind_count,
                        };
                        out_vars.push(out_var);
                    },
                    _ => unreachable!(),
                }
            }
        }
    }

    out_vars.extend(imgs);

    out_vars
}

impl<'a> ReflectIntermediate<'a> {
    fn collect_entry_points(
        &self,
        entry_point_declrs: HashMap<FunctionId, EntryPointDeclartion<'a>>,
    ) -> Result<Vec<EntryPoint>> {
        let mut entry_points = Vec::with_capacity(entry_point_declrs.len());
        for (id, entry_point_declr) in entry_point_declrs.iter() {
            let mut vars = if self.cfg.ref_all_rscs {
                self.collect_vars()
            } else {
                self.collect_entry_point_vars(*id)
            };
            if self.cfg.combine_img_samplers {
                vars = combine_img_samplers(vars);
            }
            let specs = self.collect_entry_point_specs()?;
            vars.extend(specs);
            let exec_modes = self.collect_exec_modes(*id, &entry_point_declr.exec_modes)?;
            let entry_point = EntryPoint {
                name: entry_point_declr.name.to_owned(),
                exec_model: entry_point_declr.exec_model,
                vars,
                exec_modes,
            };
            entry_points.push(entry_point);
        }
        Ok(entry_points)
    }
}
