#![allow(dead_code)]
use spirv_headers::Op;
use std::ops::RangeInclusive;

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
pub const OP_TYPE_FORWARD_POINTER: OpCode = Op::TypeForwardPointer as u32;
pub const OP_TYPE_FUNCTION: OpCode = Op::TypeFunction as u32;
pub const OP_TYPE_ACCELERATION_STRUCTURE_KHR: OpCode = Op::TypeAccelerationStructureKHR as u32;

pub fn is_ty_op(op: u32) -> bool {
    match op {
        OP_TYPE_VOID..=OP_TYPE_FUNCTION => true,
        OP_TYPE_ACCELERATION_STRUCTURE_KHR => true,
        OP_TYPE_FORWARD_POINTER => true,
        _ => false,
    }
}

pub const OP_CONSTANT_TRUE: OpCode = Op::ConstantTrue as u32;
pub const OP_CONSTANT_FALSE: OpCode = Op::ConstantFalse as u32;
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

// Integral ops used in specialization.
pub const OP_SCONVERT: OpCode = Op::SConvert as u32;
pub const OP_UCONVERT: OpCode = Op::UConvert as u32;
pub const OP_FCONVERT: OpCode = Op::FConvert as u32;
pub const OP_SNEGATE: OpCode = Op::SNegate as u32;
pub const OP_NOT: OpCode = Op::Not as u32;
pub const OP_IADD: OpCode = Op::IAdd as u32;
pub const OP_ISUB: OpCode = Op::ISub as u32;
pub const OP_IMUL: OpCode = Op::IMul as u32;
pub const OP_UDIV: OpCode = Op::UDiv as u32;
pub const OP_SDIV: OpCode = Op::SDiv as u32;
pub const OP_UMOD: OpCode = Op::UMod as u32;
pub const OP_SREM: OpCode = Op::SRem as u32;
pub const OP_SMOD: OpCode = Op::SMod as u32;
pub const OP_SHIFT_RIGHT_LOGICAL: OpCode = Op::ShiftRightLogical as u32;
pub const OP_SHIFT_RIGHT_ARITHMETIC: OpCode = Op::ShiftRightArithmetic as u32;
pub const OP_SHIFT_LEFT_LOGICAL: OpCode = Op::ShiftLeftLogical as u32;
pub const OP_BITWISE_OR: OpCode = Op::BitwiseOr as u32;
pub const OP_BITWISE_XOR: OpCode = Op::BitwiseXor as u32;
pub const OP_BITWISE_AND: OpCode = Op::BitwiseAnd as u32;
pub const OP_VECTOR_SHUFFLE: OpCode = Op::VectorShuffle as u32;
pub const OP_COMPOSITE_EXTRACT: OpCode = Op::CompositeExtract as u32;
pub const OP_COMPOSITE_INSERT: OpCode = Op::CompositeInsert as u32;
pub const OP_LOGICAL_OR: OpCode = Op::LogicalOr as u32;
pub const OP_LOGICAL_AND: OpCode = Op::LogicalAnd as u32;
pub const OP_LOGICAL_NOT: OpCode = Op::LogicalNot as u32;
pub const OP_LOGICAL_EQUAL: OpCode = Op::LogicalEqual as u32;
pub const OP_LOGICAL_NOT_EQUAL: OpCode = Op::LogicalNotEqual as u32;
pub const OP_SELECT: OpCode = Op::Select as u32;
pub const OP_IEQUAL: OpCode = Op::IEqual as u32;
pub const OP_INOT_EQUAL: OpCode = Op::INotEqual as u32;
pub const OP_ULESS_THAN: OpCode = Op::ULessThan as u32;
pub const OP_SLESS_THAN: OpCode = Op::SLessThan as u32;
pub const OP_UGREATER_THAN: OpCode = Op::UGreaterThan as u32;
pub const OP_SGREATER_THAN: OpCode = Op::SGreaterThan as u32;
pub const OP_ULESS_THAN_EQUAL: OpCode = Op::ULessThanEqual as u32;
pub const OP_SLESS_THAN_EQUAL: OpCode = Op::SLessThanEqual as u32;
pub const OP_UGREATER_THAN_EQUAL: OpCode = Op::UGreaterThanEqual as u32;
pub const OP_SGREATER_THAN_EQUAL: OpCode = Op::SGreaterThanEqual as u32;
pub const OP_QUANTIZE_TO_F16: OpCode = Op::QuantizeToF16 as u32;

pub const OP_ATOMIC_LOAD: OpCode = Op::AtomicLoad as u32;
pub const OP_ATOMIC_STORE: OpCode = Op::AtomicStore as u32;
pub const OP_ATOMIC_EXCHANGE: OpCode = Op::AtomicExchange as u32;
pub const OP_ATOMIC_COMPARE_EXCHANGE: OpCode = Op::AtomicCompareExchange as u32;
pub const OP_ATOMIC_IINCREMENT: OpCode = Op::AtomicIIncrement as u32;
pub const OP_ATOMIC_IDECREMENT: OpCode = Op::AtomicIDecrement as u32;
pub const OP_ATOMIC_IADD: OpCode = Op::AtomicIAdd as u32;
pub const OP_ATOMIC_ISUB: OpCode = Op::AtomicISub as u32;
pub const OP_ATOMIC_SMIN: OpCode = Op::AtomicSMin as u32;
pub const OP_ATOMIC_UMIN: OpCode = Op::AtomicUMin as u32;
pub const OP_ATOMIC_SMAX: OpCode = Op::AtomicSMax as u32;
pub const OP_ATOMIC_UMAX: OpCode = Op::AtomicUMax as u32;
pub const OP_ATOMIC_AND: OpCode = Op::AtomicAnd as u32;
pub const OP_ATOMIC_OR: OpCode = Op::AtomicOr as u32;
pub const OP_ATOMIC_XOR: OpCode = Op::AtomicXor as u32;
