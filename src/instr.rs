use super::parse::Instr;
use super::{Error, Result};
use spirv_headers::{Decoration, Dim, StorageClass};
use std::convert::TryFrom;
use std::marker::PhantomData;

pub use spirv_headers::{ExecutionModel, ImageFormat};

pub type InstrId = u32;
pub type FunctionId = u32;
pub type TypeId = u32;
pub type ResourceId = u32;
pub type ConstantId = u32;

pub type MemberIdx = u32;

macro_rules! define_ops {
    ($($opcode:ident { $($field:ident: $type:ty = $read_fn:ident(),)+ })+) => {
        $(
            pub struct $opcode<'a> {
                $( pub $field: $type, )*
                _ph: PhantomData<&'a ()>,
            }
            impl<'a> TryFrom<&Instr<'a>> for $opcode<'a> {
                type Error = Error;
                fn try_from(instr: &Instr<'a>) -> Result<Self> {
                    let mut operands = instr.operands();
                    let op = $opcode {
                        $( $field: operands.$read_fn()?, )+
                        _ph: PhantomData,
                    };
                    Ok(op)
                }
            }
        )+
    };
}

// Be aware that the order of the read methods is important.
define_ops! {
    OpEntryPoint {
        exec_model: ExecutionModel = read_enum(),
        func_id: FunctionId = read_u32(),
        name: &'a str = read_str(),
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

    OpTypeBool {
        ty_id: TypeId = read_u32(),
    }
    OpTypeInt {
        ty_id: TypeId = read_u32(),
        nbyte: u32 = read_u32(),
        is_signed: bool = read_bool(),
    }
    OpTypeFloat {
        ty_id: TypeId = read_u32(),
        nbyte: u32 = read_u32(),
    }
    OpTypeVector {
        ty_id: TypeId = read_u32(),
        scalar_ty_id: TypeId = read_u32(),
        nscalar: u32 = read_u32(),
    }
    OpTypeMatrix {
        ty_id: TypeId = read_u32(),
        vec_ty_id: TypeId = read_u32(),
        nvec: u32 = read_u32(),
    }
    OpTypeImage {
        ty_id: TypeId = read_u32(),
        unit_ty_id: TypeId = read_u32(),
        dim: Dim = read_enum(),
        is_depth: u32 = read_u32(),
        is_array: bool = read_bool(),
        is_multisampled: bool = read_bool(),
        is_sampled: u32 = read_u32(),
        color_fmt: ImageFormat = read_enum(),
    }
    OpTypeSampledImage {
        ty_id: TypeId = read_u32(),
        img_ty_id: TypeId = read_u32(),
    }
    OpTypeArray {
        ty_id: TypeId = read_u32(),
        proto_ty_id: TypeId = read_u32(),
        nrepeat_const_id: ConstantId = read_u32(),
    }
    OpTypeRuntimeArray {
        ty_id: TypeId = read_u32(),
        proto_ty_id: TypeId = read_u32(),
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
    OpConstant {
        ty_id: TypeId = read_u32(),
        const_id: ConstantId = read_u32(),
        value: &'a [u32] = read_list(),
    }
    OpVariable {
        ty_id: TypeId = read_u32(),
        alloc_id: ResourceId = read_u32(),
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
        rsc_id: ResourceId = read_u32(),
    }
    OpStore {
        rsc_id: ResourceId = read_u32(),
    }
    OpAccessChain {
        rsc_ty_id: TypeId = read_u32(),
        rsc_id: ResourceId = read_u32(),
        accessed_rsc_id: ResourceId = read_u32(),
    }
}
