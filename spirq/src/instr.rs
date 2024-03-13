use spirq_core::error::anyhow;
use std::convert::TryFrom;

use crate::{parse::Instr, spirv::*};

type InstrId = u32;
type FunctionId = InstrId;
type TypeId = InstrId;
type VariableId = InstrId;
type ConstantId = InstrId;
type SpecConstantId = InstrId;

type MemberIdx = u32;

#[macro_export]
macro_rules! define_ops {
    (read_enum: $type:ty: $operands:expr) => {
        {
            <$type>::from_u32($operands.read_u32()?)
                .ok_or_else(|| anyhow!("invalid enum value"))?
        }
    };
    ($read_fn:ident: $type:ty: $operands:expr) => {
        {
            $operands.$read_fn()?
        }
    };
    ($($opcode:ident { $($field:ident: $type:ty = $read_fn:tt(),)+ })+) => {
        $(
            pub struct $opcode<'a> {
                $( pub $field: $type, )*
                _ph: ::std::marker::PhantomData<&'a ()>,
            }
            impl<'a> TryFrom<&'a Instr> for $opcode<'a> {
                type Error = ::spirq_core::error::Error;
                fn try_from(instr: &'a Instr) -> ::spirq_core::error::Result<Self> {
                    let mut operands = instr.operands();
                    let op = $opcode {
                        $( $field: define_ops!($read_fn: $type: operands), )+
                        _ph: ::std::marker::PhantomData,
                    };
                    Ok(op)
                }
            }
        )+
    };
}

// Be aware that the order of the read methods is important.
define_ops! {
    OpExtInstImport {
        instr_set_id: InstrId = read_u32(),
        name: &'a str = read_str(),
    }

    OpMemoryModel {
        addr_model: AddressingModel = read_enum(),
        mem_model: MemoryModel = read_enum(),
    }

    OpEntryPoint {
        exec_model: ExecutionModel = read_enum(),
        func_id: FunctionId = read_u32(),
        name: &'a str = read_str(),
    }

    OpExecutionModeCommonSPQ {
        func_id: FunctionId = read_u32(),
        execution_mode: ExecutionMode = read_enum(),
        params: &'a [u32] = read_list(),
    }

    OpName {
        target_id: InstrId = read_u32(),
        name: &'a str = read_str(),
    }
    OpMemberName {
        target_id: InstrId = read_u32(),
        member_idx: MemberIdx = read_u32(),
        name: &'a str = read_str(),
    }

    OpDecorate {
        target_id: InstrId = read_u32(),
        deco: Decoration = read_enum(),
        params: &'a [u32] = read_list(),
    }
    OpMemberDecorate {
        target_id: InstrId = read_u32(),
        member_idx: MemberIdx = read_u32(),
        deco: Decoration = read_enum(),
        params: &'a [u32] = read_list(),
    }

    OpTypeVoid {
        ty_id: TypeId = read_u32(),
    }
    OpTypeBool {
        ty_id: TypeId = read_u32(),
    }
    OpTypeInt {
        ty_id: TypeId = read_u32(),
        bits: u32 = read_u32(),
        is_signed: bool = read_bool(),
    }
    OpTypeFloat {
        ty_id: TypeId = read_u32(),
        bits: u32 = read_u32(),
    }
    OpTypeVector {
        ty_id: TypeId = read_u32(),
        scalar_ty_id: TypeId = read_u32(),
        nscalar: u32 = read_u32(),
    }
    OpTypeMatrix {
        ty_id: TypeId = read_u32(),
        vector_ty_id: TypeId = read_u32(),
        nvector: u32 = read_u32(),
    }
    OpTypeImage {
        ty_id: TypeId = read_u32(),
        scalar_ty_id: TypeId = read_u32(),
        dim: Dim = read_enum(),
        is_depth: u32 = read_u32(),
        is_array: bool = read_bool(),
        is_multisampled: bool = read_bool(),
        is_sampled: u32 = read_u32(),
        color_fmt: ImageFormat = read_enum(),
    }
    OpTypeSampler {
        ty_id: TypeId = read_u32(),
    }
    OpTypeSampledImage {
        ty_id: TypeId = read_u32(),
        image_ty_id: TypeId = read_u32(),
    }
    OpTypeArray {
        ty_id: TypeId = read_u32(),
        element_ty_id: TypeId = read_u32(),
        nelement_const_id: ConstantId = read_u32(),
    }
    OpTypeRuntimeArray {
        ty_id: TypeId = read_u32(),
        element_ty_id: TypeId = read_u32(),
    }
    OpTypeStruct {
        ty_id: TypeId = read_u32(),
        member_ty_ids: &'a [TypeId] = read_list(),
    }
    OpTypePointer {
        ty_id: TypeId = read_u32(),
        store_cls: StorageClass = read_enum(),
        target_ty_id: TypeId = read_u32(),
    }
    OpTypeForwardPointer {
        ty_id: TypeId = read_u32(),
        store_cls: StorageClass = read_enum(),
    }
    OpConstantTrue {
        ty_id: TypeId = read_u32(),
        const_id: ConstantId = read_u32(),
    }
    OpConstantFalse {
        ty_id: TypeId = read_u32(),
        const_id: ConstantId = read_u32(),
    }
    OpConstant {
        ty_id: TypeId = read_u32(),
        const_id: ConstantId = read_u32(),
        value: &'a [u32] = read_list(),
    }
    OpSpecConstantTrue {
        ty_id: TypeId = read_u32(),
        spec_const_id: SpecConstantId = read_u32(),
    }
    OpSpecConstantFalse {
        ty_id: TypeId = read_u32(),
        spec_const_id: SpecConstantId = read_u32(),
    }
    OpSpecConstant {
        ty_id: TypeId = read_u32(),
        spec_const_id: SpecConstantId = read_u32(),
        value: &'a [u32] = read_list(),
    }
    OpSpecConstantComposite {
        ty_id: TypeId = read_u32(),
        spec_const_id: SpecConstantId = read_u32(),
        value: &'a [SpecConstantId] = read_list(),
    }
    OpVariable {
        ty_id: TypeId = read_u32(),
        var_id: VariableId = read_u32(),
        store_cls: StorageClass = read_enum(),
    }

    OpFunction {
        return_ty_id: TypeId = read_u32(),
        func_id: TypeId = read_u32(),
    }
    OpFunctionCall {
        return_ty_id: TypeId = read_u32(),
        return_id: InstrId = read_u32(),
        func_id: FunctionId = read_u32(),
    }
    OpLoad {
        return_ty_id: TypeId = read_u32(),
        return_id: InstrId = read_u32(),
        var_id: VariableId = read_u32(),
    }
    OpStore {
        var_id: VariableId = read_u32(),
    }
    OpAccessChain {
        var_ty_id: TypeId = read_u32(),
        var_id: VariableId = read_u32(),
        accessed_var_id: VariableId = read_u32(),
    }
    OpTypeAccelerationStructureKHR {
        ty_id: TypeId = read_u32(),
    }

    OpConstantScalarCommonSPQ {
        ty_id: TypeId = read_u32(),
        const_id: ConstantId = read_u32(),
        value: &'a [u32] = read_list(),
    }
    OpSpecConstantHeadSPQ {
        ty_id: TypeId = read_u32(),
        spec_const_id: SpecConstantId = read_u32(),
        opcode: u32 = read_u32(),
    }
    OpSpecConstantUnaryOpCommonSPQ {
        ty_id: TypeId = read_u32(),
        spec_const_id: SpecConstantId = read_u32(),
        opcode: u32 = read_u32(),
        a_id: SpecConstantId = read_u32(),
    }
    OpSpecConstantBinaryOpCommonSPQ {
        ty_id: TypeId = read_u32(),
        spec_const_id: SpecConstantId = read_u32(),
        opcode: u32 = read_u32(),
        a_id: SpecConstantId = read_u32(),
        b_id: SpecConstantId = read_u32(),
    }
    OpSpecConstantTertiaryOpCommonSPQ {
        ty_id: TypeId = read_u32(),
        spec_const_id: SpecConstantId = read_u32(),
        opcode: u32 = read_u32(),
        a_id: SpecConstantId = read_u32(),
        b_id: SpecConstantId = read_u32(),
        c_id: SpecConstantId = read_u32(),
    }
    OpTypeRayQueryKHR {
        ty_id: TypeId = read_u32(),
    }
}
