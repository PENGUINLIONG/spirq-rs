//! Reflection procedures and types.
use std::convert::{TryFrom};
use std::collections::{HashMap, HashSet};
use std::iter::Peekable;
use std::ops::RangeInclusive;
use spirv_headers::{Decoration, Dim, StorageClass};
use crate::ty::*;
use crate::consts::*;
use crate::{Location, DescriptorBinding, SpirvBinary, Instrs, Instr, Manifest,
    ResourceLocator, ExecutionModel, EntryPoint};
use crate::error::{Error, Result};
use crate::instr::*;

// Intermediate types used in reflection.

#[derive(Debug, Clone)]
struct Constant<'a> {
    ty: InstrId,
    value: &'a [u32],
}
#[derive(Clone)]
enum Variable {
    Input(Location, Type),
    Output(Location, Type),
    Descriptor(DescriptorBinding, DescriptorType),
}
#[derive(Default, Debug, Clone)]
struct Function {
    accessed_vars: HashSet<InstrId>,
    calls: HashSet<InstrId>,
}
struct EntryPointDeclartion<'a> {
    func_id: u32,
    name: &'a str,
    exec_model: ExecutionModel,
}


type ObjectId = u32;
type TypeId = ObjectId;
type VariableId = ObjectId;
type ConstantId = ObjectId;
type FunctionId = ObjectId;

// The actual reflection to take place.

#[derive(Default)]
struct ReflectIntermediate<'a> {
    entry_point_declrs: Vec<EntryPointDeclartion<'a>>,
    name_map: HashMap<(InstrId, Option<u32>), &'a str>,
    deco_map: HashMap<(InstrId, Option<u32>, Decoration), &'a [u32]>,
    ty_map: HashMap<TypeId, Type>,
    var_map: HashMap<VariableId, Variable>,
    const_map: HashMap<ConstantId, Constant<'a>>,
    ptr_map: HashMap<TypeId, TypeId>,
    func_map: HashMap<FunctionId, Function>,
}
impl<'a> ReflectIntermediate<'a> {
    /// Resolve one recurring layer of pointers to the pointer that refer to the
    /// data directly.
    fn resolve_ref(&self, ty_id: TypeId) -> Option<(TypeId, &Type)> {
        self.ptr_map.get(&ty_id)
            .and_then(|ty_id| {
                self.ty_map.get(ty_id)
                    .map(|ty| (*ty_id, ty))
            })
    }
    fn contains_deco(&self, id: ObjectId, member_idx: Option<u32>, deco: Decoration) -> bool {
        self.deco_map.contains_key(&(id, member_idx, deco))
    }
    fn get_deco_u32(&self, id: InstrId, member_idx: Option<u32>, deco: Decoration) -> Option<u32> {
        self.deco_map.get(&(id, member_idx, deco))
            .and_then(|x| x.get(0))
            .cloned()
    }
    fn get_var_location_or_default(&self, var_id: VariableId) -> u32 {
        self.get_deco_u32(var_id, None, Decoration::Location)
            .unwrap_or(0)
    }
    fn get_var_desc_bind_or_default(&self, var_id: VariableId) -> DescriptorBinding {
        let desc_set = self.get_deco_u32(var_id, None, Decoration::DescriptorSet)
            .unwrap_or(0);
        let bind_point = self.get_deco_u32(var_id, None, Decoration::Binding)
            .unwrap_or(0);
        DescriptorBinding::desc_bind(desc_set, bind_point)
    }
    fn get_name(&self, id: InstrId, member_idx: Option<u32>) -> Option<&'a str> {
        self.name_map.get(&(id, member_idx))
            .map(|x| *x)
    }
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
            let collision = self.name_map.insert(key, value);
            if collision.is_some() { return Err(Error::CorruptedSpirv); }
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
                _ => break,
            };
            let collision = self.deco_map.insert(key, value);
            if collision.is_some() { return Err(Error::CorruptedSpirv); }
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
            }
            OP_TYPE_FLOAT => {
                let op = OpTypeFloat::try_from(instr)?;
                let scalar_ty = ScalarType::float(op.nbyte >> 3);
                (op.ty_id, Type::Scalar(scalar_ty))
            },
            OP_TYPE_VECTOR => {
                let op = OpTypeVector::try_from(instr)?;
                if let Some(Type::Scalar(scalar_ty)) = self.ty_map.get(&op.scalar_ty_id).cloned() {
                    let vec_ty = VectorType::new(scalar_ty, op.nscalar);
                    (op.ty_id, Type::Vector(vec_ty))
                } else { return Err(Error::CorruptedSpirv); }
            },
            OP_TYPE_MATRIX => {
                let op = OpTypeMatrix::try_from(instr)?;
                if let Some(Type::Vector(vec_ty)) = self.ty_map.get(&op.vec_ty_id).cloned() {
                    let mat_ty = MatrixType::new(vec_ty, op.nvec);
                    (op.ty_id, Type::Matrix(mat_ty))
                } else { return Err(Error::CorruptedSpirv); }
            },
            OP_TYPE_IMAGE => {
                let op = OpTypeImage::try_from(instr)?;
                let img_ty = if op.dim == Dim::DimSubpassData {
                    Type::SubpassData
                } else {
                    // Only unit types allowed to be stored in storage images can
                    // have given format.
                    let unit_fmt = ImageUnitFormat::from_spv_def(op.is_sampled, op.is_depth, op.color_fmt)?;
                    let arng = ImageArrangement::from_spv_def(op.dim, op.is_array, op.is_multisampled)?;
                    let img_ty = ImageType::new(unit_fmt, arng);
                    Type::Image(img_ty)
                };
                (op.ty_id, img_ty)
            },
            OP_TYPE_SAMPLED_IMAGE => {
                let op = OpTypeSampledImage::try_from(instr)?;
                if let Some(Type::Image(img_ty)) = self.ty_map.get(&op.img_ty_id) {
                    (op.ty_id, Type::Image(img_ty.clone()))
                } else { return Err(Error::CorruptedSpirv); }
            },
            OP_TYPE_ARRAY => {
                let op = OpTypeArray::try_from(instr)?;
                let proto_ty = self.ty_map.get(&op.proto_ty_id)
                    .ok_or(Error::CorruptedSpirv)?;
                let nrepeat = self.const_map.get(&op.nrepeat_const_id)
                    .and_then(|constant| {
                        if let Some(Type::Scalar(scalar_ty)) = self.ty_map.get(&constant.ty) {
                            if scalar_ty.nbyte() == 4 && scalar_ty.is_uint() {
                                return Some(constant.value[0]);
                            }
                        }
                        None
                    })
                    .ok_or(Error::CorruptedSpirv)?;
                let stride = self.get_deco_u32(op.ty_id, None, Decoration::ArrayStride)
                    .map(|x| x as usize);
                let arr_ty = if let Some(stride) = stride {
                    ArrayType::new(proto_ty, nrepeat, stride)
                } else {
                    ArrayType::new_multibind(proto_ty, nrepeat)
                };
                (op.ty_id, Type::Array(arr_ty))
            },
            OP_TYPE_RUNTIME_ARRAY => {
                let op = OpTypeRuntimeArray::try_from(instr)?;
                let proto_ty = self.ty_map.get(&op.proto_ty_id)
                    .ok_or(Error::CorruptedSpirv)?;
                let stride = self.get_deco_u32(op.ty_id, None, Decoration::ArrayStride)
                    .map(|x| x as usize)
                    .ok_or(Error::CorruptedSpirv)?;
                let arr_ty = ArrayType::new_unsized(proto_ty, stride);
                (op.ty_id, Type::Array(arr_ty))
            }
            OP_TYPE_STRUCT => {
                let op = OpTypeStruct::try_from(instr)?;
                let mut struct_ty = StructType::default();
                for (i, &member_ty_id) in op.member_ty_ids.iter().enumerate() {
                    let i = i as u32;
                    let mut member_ty = self.ty_map.get(&member_ty_id)
                        .cloned()
                        .ok_or(Error::CorruptedSpirv)?;
                    let mut proto_ty = &mut member_ty;
                    while let Type::Array(arr_ty) = proto_ty {
                        proto_ty = &mut *arr_ty.proto_ty;
                    }
                    if let Type::Matrix(ref mut mat_ty) = proto_ty {
                        let mat_stride = self.get_deco_u32(op.ty_id, Some(i), Decoration::MatrixStride)
                            .map(|x| x as usize)
                            .ok_or(Error::CorruptedSpirv)?;
                        let row_major = self.contains_deco(op.ty_id, Some(i), Decoration::RowMajor);
                        let col_major = self.contains_deco(op.ty_id, Some(i), Decoration::ColMajor);
                        let major = match (row_major, col_major) {
                            (true, false) => MatrixAxisOrder::RowMajor,
                            (false, true) => MatrixAxisOrder::ColumnMajor,
                            _ => return Err(Error::CorruptedSpirv),
                        };
                        mat_ty.decorate(mat_stride, major);
                    }
                    let name = if let Some(nm) = self.get_name(op.ty_id, Some(i)) {
                        if nm.is_empty() { None } else { Some(nm.to_owned()) }
                    } else { None };
                    if let Some(offset) = self.get_deco_u32(op.ty_id, Some(i), Decoration::Offset)
                        .map(|x| x as usize) {
                        let member = StructMember { name, offset, ty: member_ty };
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
                    return Err(Error::CorruptedSpirv)
                } else { return Ok(()) }
            },
            _ => return Err(Error::CorruptedSpirv),
        };
        if let Vacant(entry) = self.ty_map.entry(key) {
            entry.insert(value); Ok(())
        } else { Err(Error::CorruptedSpirv) }
    }
    fn populate_one_const(&mut self, instr: &Instr<'a>) -> Result<()> {
        use std::collections::hash_map::Entry::Vacant;
        if instr.opcode() != OP_CONSTANT { return Ok(()) }
        let op = OpConstant::try_from(instr)?;
        let constant = Constant { ty: op.ty_id, value: op.value };
        if let Vacant(entry) = self.const_map.entry(op.const_id) {
            entry.insert(constant); Ok(())
        } else { Err(Error::CorruptedSpirv) }
    }
    fn populate_one_var(&mut self, instr: &Instr<'a>) -> Result<()> {
        fn ty2buf(ty: &Type) -> Option<(u32, Type)> {
            match ty {
                Type::Array(arr_ty) => if let Type::Struct(struct_ty) = &*arr_ty.proto_ty() {
                    Some((arr_ty.nrepeat()?, Type::Struct(struct_ty.clone())))
                } else { return None },
                Type::Struct(_) => Some((1, ty.clone())),
                _ => return None,
            }
        }
        fn ty2uniform(buf_ty: &Type) -> Option<DescriptorType> {
            let (nbind, struct_ty) = ty2buf(buf_ty)?;
            let desc_ty = DescriptorType::UniformBuffer(nbind, struct_ty);
            Some(desc_ty)
        }
        fn ty2storage(buf_ty: &Type) -> Option<DescriptorType> {
            let (nbind, struct_ty) = ty2buf(buf_ty)?;
            let desc_ty = DescriptorType::StorageBuffer(nbind, struct_ty);
            Some(desc_ty)
        }

        let op = OpVariable::try_from(instr)?;
        let (ty_id, ty) = if let Some(x) = self.resolve_ref(op.ty_id) { x } else {
            // If a variable is declared based on a unregistered type, very
            // likely it's a input/output block passed between shader stages. We
            // can safely ignore them.
            return Ok(());
        };
        match op.store_cls {
            StorageClass::Input => {
                let location = self.get_var_location_or_default(op.alloc_id);
                let var = Variable::Input(location, ty.clone());
                if self.var_map.insert(op.alloc_id, var).is_some() {
                    return Err(Error::CorruptedSpirv);
                }
                // There can be interface blocks for input and output but there
                // won't be any for attribute inputs nor for attachment outputs,
                // so we just ignore structs and arrays or something else here.
            },
            StorageClass::Output => {
                let location = self.get_var_location_or_default(op.alloc_id);
                let var = Variable::Output(location, ty.clone());
                if self.var_map.insert(op.alloc_id, var).is_some() {
                    return Err(Error::CorruptedSpirv);
                }
            },
            StorageClass::PushConstant => {
                // Push constants have no global offset. Offsets are applied to
                // members.
                if let Type::Struct(_) = ty {
                    let desc_bind = DescriptorBinding::push_const();
                    let desc_ty = DescriptorType::PushConstant(ty.clone());
                    let var = Variable::Descriptor(desc_bind, desc_ty);
                    if self.var_map.insert(op.alloc_id, var).is_some() {
                        return Err(Error::CorruptedSpirv);
                    }
                } else { return Err(Error::CorruptedSpirv); }
            },
            StorageClass::Uniform => {
                let desc_ty = if self.contains_deco(ty_id, None, Decoration::BufferBlock) {
                    ty2storage(ty)
                } else {
                    ty2uniform(ty)
                }.ok_or(Error::CorruptedSpirv)?;
                let desc_bind = self.get_var_desc_bind_or_default(op.alloc_id);
                let var = Variable::Descriptor(desc_bind, desc_ty);
                if self.var_map.insert(op.alloc_id, var).is_some() {
                    return Err(Error::CorruptedSpirv);
                }
            },
            StorageClass::StorageBuffer => {
                let desc_ty = ty2storage(ty).ok_or(Error::CorruptedSpirv)?;
                let desc_bind = self.get_var_desc_bind_or_default(op.alloc_id);
                let var = Variable::Descriptor(desc_bind, desc_ty);
                if self.var_map.insert(op.alloc_id, var).is_some() {
                    return Err(Error::CorruptedSpirv);
                }
            },
            StorageClass::UniformConstant => {
                let desc_bind = self.get_var_desc_bind_or_default(op.alloc_id);
                let desc_ty = if let Type::Image(_) = ty {
                    DescriptorType::Image(ty.clone())
                } else if let Type::SubpassData = ty {
                    let input_attm_idx = self.get_deco_u32(op.alloc_id, None, Decoration::InputAttachmentIndex)
                        .ok_or(Error::CorruptedSpirv)?;
                    DescriptorType::InputAtatchment(input_attm_idx)
                } else {
                    return Err(Error::UnsupportedSpirv);
                };
                let var = Variable::Descriptor(desc_bind, desc_ty);
                if self.var_map.insert(op.alloc_id, var).is_some() {
                    return Err(Error::CorruptedSpirv);
                }
                // Leak out unknown types of uniform constants.
            },
            _ => {
                // Leak out unknown storage classes.
            },
        }
        Ok(())
    }
    fn populate_defs(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        // type definitions always follow decorations, so we don't skip
        // instructions here.
        while let Some(instr) = instrs.peek() {
            let opcode = instr.opcode();
            if TYPE_RANGE.contains(&opcode) {
                self.populate_one_ty(instr)?;
            } else if opcode == OP_VARIABLE {
                self.populate_one_var(instr)?;
            } else if CONST_RANGE.contains(&opcode) {
                self.populate_one_const(instr)?;
            } else if SPEC_CONST_RANGE.contains(&opcode) {
                // TODO: (penguinliong)
            } else { break; }
            instrs.next();
        }
        Ok(())
    }
    fn populate_access(&mut self, instrs: &'_ mut Peekable<Instrs<'a>>) -> Result<()> {
        while instrs.peek().is_some() {
            let mut access_chain_map = HashMap::new();
            let mut func: &mut Function = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            while let Some(instr) = instrs.peek() {
                if instr.opcode() == OP_FUNCTION {
                    let op = OpFunction::try_from(instr)?;
                    func = self.func_map.entry(op.func_id).or_default();
                    break;
                }
                instrs.next();
            }
            while let Some(instr) = instrs.peek() {
                match instr.opcode() {
                    OP_FUNCTION_CALL => {
                        let op = OpFunctionCall::try_from(instr)?;
                        if !func.calls.insert(op.func_id) {
                            return Err(Error::CorruptedSpirv);
                        }
                    },
                    OP_LOAD => {
                        let op = OpLoad::try_from(instr)?;
                        let mut rsc_id = op.rsc_id;
                        if let Some(&x) = access_chain_map.get(&rsc_id) { rsc_id = x }
                        func.accessed_vars.insert(rsc_id);
                    },
                    OP_STORE => {
                        let op = OpStore::try_from(instr)?;
                        let mut rsc_id = op.rsc_id;
                        if let Some(&x) = access_chain_map.get(&rsc_id) { rsc_id = x }
                        func.accessed_vars.insert(rsc_id);
                    },
                    OP_ACCESS_CHAIN => {
                        let op = OpAccessChain::try_from(instr)?;
                        if access_chain_map.insert(op.rsc_id, op.accessed_rsc_id).is_some() {
                            return Err(Error::CorruptedSpirv);
                        }
                    },
                    OP_FUNCTION_END => break,
                    _ => { },
                }
                instrs.next();
            }
        }
        Ok(())
    }
    fn collect_fn_vars_impl(&self, func: FunctionId, vars: &mut HashSet<VariableId>) {
        if let Some(func) = self.func_map.get(&func) {
            let it = func.accessed_vars.iter()
                .filter(|x| self.var_map.contains_key(x));
            vars.extend(it);
            for call in func.calls.iter() {
                self.collect_fn_vars_impl(*call, vars);
            }
        }
    }
    fn collect_fn_vars(&self, func: FunctionId) -> HashSet<VariableId> {
        let mut accessed_vars = HashSet::new();
        self.collect_fn_vars_impl(func, &mut accessed_vars);
        accessed_vars
    }
    fn collect_entry_points(&self) -> Result<Box<[EntryPoint]>> {
        let mut entry_points = Vec::with_capacity(self.entry_point_declrs.len());
        for entry_point_declr in self.entry_point_declrs.iter() {
            let mut entry_point = EntryPoint {
                name: entry_point_declr.name.to_owned(),
                exec_model: entry_point_declr.exec_model,
                manifest: Manifest::default(),
            };
            let accessed_var_ids = self.collect_fn_vars(entry_point_declr.func_id);
            for accessed_var_id in accessed_var_ids {
                let accessed_var = self.var_map.get(&accessed_var_id)
                    .cloned()
                    .ok_or(Error::CorruptedSpirv)?;
                let collision = match accessed_var {
                    Variable::Input(location, ivar_ty) => {
                        let mut collision = entry_point.manifest.input_map
                            .insert(location, ivar_ty).is_some();
                        if let Some(name) = self.get_name(accessed_var_id, None) {
                            collision |= entry_point.manifest.var_name_map
                                .insert(name.to_owned(), ResourceLocator::Input(location)).is_some();
                        }
                        collision
                    },
                    Variable::Output(location, ivar_ty) => {
                        let mut collision = entry_point.manifest.output_map.insert(location, ivar_ty).is_some();
                        if let Some(name) = self.get_name(accessed_var_id, None) {
                            collision |= entry_point.manifest.var_name_map
                                .insert(name.to_owned(), ResourceLocator::Output(location)).is_some();
                        }
                        collision
                    },
                    Variable::Descriptor(desc_bind, desc_ty) => {
                        let mut collision = entry_point.manifest.desc_map.insert(desc_bind, desc_ty).is_some();
                        if let Some(name) = self.get_name(accessed_var_id, None) {
                            collision |= entry_point.manifest.var_name_map
                                .insert(name.to_owned(), ResourceLocator::Descriptor(desc_bind)).is_some();
                        }
                        collision
                    },
                };
                if collision { return Err(Error::CorruptedSpirv); }
            }
            entry_points.push(entry_point);
        }
        Ok(entry_points.into_boxed_slice())
    }
}


pub(crate) fn reflect_spirv<'a>(module: &'a SpirvBinary) -> Result<Box<[EntryPoint]>> {
    fn skip_until_range_inclusive<'a>(instrs: &'_ mut Peekable<Instrs<'a>>, rng: RangeInclusive<u32>) {
        while let Some(instr) = instrs.peek() {
            if !rng.contains(&instr.opcode()) { instrs.next(); } else { break; }
        }
    }
    // Don't change the order. See _2.4 Logical Layout of a Module_ of the
    // SPIR-V specification for more information.
    let mut instrs = module.instrs().peekable();
    let mut itm = ReflectIntermediate::default();
    skip_until_range_inclusive(&mut instrs, ENTRY_POINT_RANGE);
    itm.populate_entry_points(&mut instrs)?;
    skip_until_range_inclusive(&mut instrs, NAME_RANGE);
    itm.populate_names(&mut instrs)?;
    skip_until_range_inclusive(&mut instrs, DECO_RANGE);
    itm.populate_decos(&mut instrs)?;
    itm.populate_defs(&mut instrs)?;
    itm.populate_access(&mut instrs)?;
    Ok(itm.collect_entry_points()?)
}
