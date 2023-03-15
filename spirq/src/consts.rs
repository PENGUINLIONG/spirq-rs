#![allow(dead_code)]
use spirv::Op;
use std::ops::RangeInclusive;

pub type OpCode = u32;
pub const OP_ENTRY_POINT: OpCode = Op::EntryPoint as u32;
pub const ENTRY_POINT_RANGE: RangeInclusive<OpCode> = OP_ENTRY_POINT..=OP_ENTRY_POINT;

pub fn is_debug_op(op: Op) -> bool {
    match op {
        Op::SourceContinued => true,
        Op::Source => true,
        Op::SourceExtension => true,
        Op::Name => true,
        Op::MemberName => true,
        Op::String => true,
        Op::ModuleProcessed => true,
        _ => false,
    }
}
pub fn is_deco_op(op: Op) -> bool {
    match op {
        Op::Decorate => true,
        Op::MemberDecorate => true,
        Op::DecorationGroup => true,
        Op::GroupDecorate => true,
        Op::GroupMemberDecorate => true,
        Op::DecorateId => true,
        Op::DecorateString => true,
        Op::MemberDecorateString => true,
        _ => false,
    }
}
pub fn is_ty_op(op: Op) -> bool {
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

pub fn is_const_op(op: Op) -> bool {
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
pub fn is_spec_const_op(op: Op) -> bool {
    match op {
        _ => false,
    }
}
pub fn is_spec_const_op_op(op: Op) -> bool {
    match op {
        Op::SConvert => true,
        Op::UConvert => true,
        Op::FConvert => true,
        Op::SNegate => true,
        Op::Not => true,
        Op::IAdd => true,
        Op::ISub => true,
        Op::IMul => true,
        Op::UDiv => true,
        Op::SDiv => true,
        Op::UMod => true,
        Op::SRem => true,
        Op::SMod => true,
        Op::ShiftRightLogical => true,
        Op::ShiftRightArithmetic => true,
        Op::ShiftLeftLogical => true,
        Op::BitwiseOr => true,
        Op::BitwiseXor => true,
        Op::BitwiseAnd => true,
        Op::VectorShuffle => true,
        Op::CompositeExtract => true,
        Op::CompositeInsert => true,
        Op::LogicalOr => true,
        Op::LogicalAnd => true,
        Op::LogicalNot => true,
        Op::LogicalEqual => true,
        Op::LogicalNotEqual => true,
        Op::Select => true,
        Op::IEqual => true,
        Op::INotEqual => true,
        Op::ULessThan => true,
        Op::SLessThan => true,
        Op::UGreaterThan => true,
        Op::SGreaterThan => true,
        Op::ULessThanEqual => true,
        Op::SLessThanEqual => true,
        Op::UGreaterThanEqual => true,
        Op::SGreaterThanEqual => true,
        Op::QuantizeToF16 => true,
        _ => false,
    }
}

pub fn is_atomic_load_op(op: Op) -> bool {
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
pub fn is_atomic_store_op(op: Op) -> bool {
    match op {
        Op::AtomicStore => true,
        _ => false,
    }
}
