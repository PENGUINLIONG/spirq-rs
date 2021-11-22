#![allow(dead_code)]
use std::ops::RangeInclusive;
use spirv_headers::Op;

pub type OpCode = u32;
pub const OP_ENTRY_POINT: OpCode = Op::EntryPoint as u32;
pub const ENTRY_POINT_RANGE: RangeInclusive<OpCode> = OP_ENTRY_POINT..=OP_ENTRY_POINT;

pub const OP_EXECUTION_MODE: OpCode = Op::ExecutionMode as u32;

pub const OP_NAME: OpCode = Op::Name as u32;
pub const OP_MEMBER_NAME: OpCode = Op::MemberName as u32;
pub const NAME_RANGE: RangeInclusive<OpCode> = OP_NAME..=OP_MEMBER_NAME;

pub const OP_DECORATE: OpCode = Op::Decorate as u32;
pub const OP_MEMBER_DECORATE: OpCode = Op::MemberDecorate as u32;
pub const OP_DECORATION_GROUP: OpCode = Op::DecorationGroup as u32;
pub const OP_GROUP_DECORATE: OpCode = Op::GroupDecorate as u32;
pub const OP_GROUP_MEMBER_DECORATE: OpCode = Op::GroupMemberDecorate as u32;
pub const OP_DECORATE_ID: OpCode = Op::DecorateId as u32;
pub const OP_DECORATE_STRING: OpCode = Op::DecorateString as u32;
pub const OP_MEMBER_DECORATE_STRING: OpCode = Op::MemberDecorateString as u32;

pub fn is_deco_op(op: u32) -> bool {
    match op {
        OP_DECORATE..=OP_GROUP_MEMBER_DECORATE => true,
        OP_DECORATE_ID => true,
        OP_DECORATE_STRING..=OP_MEMBER_DECORATE_STRING => true,
        _ => false,
    }
}

// Don't need this: Not a resource type. But kept for the range.
pub const OP_TYPE_VOID: OpCode = Op::TypeVoid as u32;
pub const OP_TYPE_BOOL: OpCode = Op::TypeBool as u32;
pub const OP_TYPE_INT: OpCode = Op::TypeInt as u32;
pub const OP_TYPE_FLOAT: OpCode = Op::TypeFloat as u32;
pub const OP_TYPE_VECTOR: OpCode = Op::TypeVector as u32;
pub const OP_TYPE_MATRIX: OpCode = Op::TypeMatrix as u32;
pub const OP_TYPE_IMAGE: OpCode = Op::TypeImage as u32;
pub const OP_TYPE_SAMPLER: OpCode = Op::TypeSampler as u32;
pub const OP_TYPE_SAMPLED_IMAGE: OpCode = Op::TypeSampledImage as u32;
pub const OP_TYPE_ARRAY: OpCode = Op::TypeArray as u32;
pub const OP_TYPE_RUNTIME_ARRAY: OpCode = Op::TypeRuntimeArray as u32;
pub const OP_TYPE_STRUCT: OpCode = Op::TypeStruct as u32;
pub const OP_TYPE_POINTER: OpCode = Op::TypePointer as u32;
// Don't need this: Not a resource type. But kept for the range.
pub const OP_TYPE_FUNCTION: OpCode = Op::TypeFunction as u32;
pub const OP_TYPE_ACCELERATION_STRUCTURE_KHR: OpCode = Op::TypeAccelerationStructureKHR as u32;
pub const TYPE_RANGE: RangeInclusive<OpCode> = OP_TYPE_VOID..=OP_TYPE_FUNCTION;

pub const OP_CONSTANT_TRUE: OpCode = Op::ConstantTrue as u32;
// pub const OP_CONSTANT_FALSE: OpCode = Op::ConstantFalse as u32;
pub const OP_CONSTANT: OpCode = Op::Constant as u32;
// pub const OP_CONSTANT_COMPOSITE: OpCode = Op::ConstantComposite as u32;
// pub const OP_CONSTANT_SAMPLER: OpCode = Op::ConstantSampler as u32;
pub const OP_CONSTANT_NULL: OpCode = Op::ConstantNull as u32;
pub const CONST_RANGE: RangeInclusive<OpCode> = OP_CONSTANT_TRUE..=OP_CONSTANT_NULL;

pub const OP_SPEC_CONSTANT_TRUE: OpCode = Op::SpecConstantTrue as u32;
pub const OP_SPEC_CONSTANT_FALSE: OpCode = Op::SpecConstantFalse as u32;
pub const OP_SPEC_CONSTANT: OpCode = Op::SpecConstant as u32;
pub const OP_SPEC_CONSTANT_COMPOSITE: OpCode = Op::SpecConstantComposite as u32;
pub const OP_SPEC_CONSTANT_OP: OpCode = Op::SpecConstantOp as u32;
pub const SPEC_CONST_RANGE: RangeInclusive<OpCode> = OP_SPEC_CONSTANT_TRUE..=OP_SPEC_CONSTANT_OP;

pub const OP_VARIABLE: OpCode = Op::Variable as u32;

pub const OP_FUNCTION: OpCode = Op::Function as u32;
pub const OP_FUNCTION_END: OpCode = Op::FunctionEnd as u32;
pub const OP_FUNCTION_CALL: OpCode = Op::FunctionCall as u32;
pub const OP_ACCESS_CHAIN: OpCode = Op::AccessChain as u32;
pub const OP_LOAD: OpCode = Op::Load as u32;
pub const OP_STORE: OpCode = Op::Store as u32;
// pub const OP_IN_BOUNDS_ACCESS_CHAIN: OpCode = Op::InBoundsAccessChain as u32;
