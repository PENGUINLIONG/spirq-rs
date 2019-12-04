use std::ops::RangeInclusive;

type OpCode = u32;
pub const OP_ENTRY_POINT: OpCode = 15;
pub const ENTRY_POINT_RANGE: RangeInclusive<OpCode> = OP_ENTRY_POINT..=OP_ENTRY_POINT;

pub const OP_NAME: OpCode = 5;
pub const OP_MEMBER_NAME: OpCode = 6;
pub const NAME_RANGE: RangeInclusive<OpCode> = OP_NAME..=OP_MEMBER_NAME;

pub const OP_DECORATE: OpCode = 71;
pub const OP_MEMBER_DECORATE: OpCode = 72;
pub const DECO_RANGE: RangeInclusive<OpCode> = OP_DECORATE..=OP_MEMBER_DECORATE;

// Don't need this: Not a resource type. But kept for the range.
pub const OP_TYPE_VOID: OpCode = 19;
pub const OP_TYPE_BOOL: OpCode = 20;
pub const OP_TYPE_INT: OpCode = 21;
pub const OP_TYPE_FLOAT: OpCode = 22;
pub const OP_TYPE_VECTOR: OpCode = 23;
pub const OP_TYPE_MATRIX: OpCode = 24;
pub const OP_TYPE_IMAGE: OpCode = 25;
// Not in GLSL.
// pub const OP_TYPE_SAMPLER: OpCode = 26;
pub const OP_TYPE_SAMPLED_IMAGE: OpCode = 27;
pub const OP_TYPE_ARRAY: OpCode = 28;
pub const OP_TYPE_RUNTIME_ARRAY: OpCode = 29;
pub const OP_TYPE_STRUCT: OpCode = 30;
pub const OP_TYPE_POINTER: OpCode = 32;
// Don't need this: Not a resource type. But kept for the range.
pub const OP_TYPE_FUNCTION: OpCode = 33;
pub const TYPE_RANGE: RangeInclusive<OpCode> = OP_TYPE_VOID..=OP_TYPE_FUNCTION;

pub const OP_CONSTANT_TRUE: OpCode = 41;
// pub const OP_CONSTANT_FALSE: OpCode = 42;
pub const OP_CONSTANT: OpCode = 43;
// pub const OP_CONSTANT_COMPOSITE: OpCode = 44;
// pub const OP_CONSTANT_SAMPLER: OpCode = 45;
pub const OP_CONSTANT_NULL: OpCode = 46;
pub const CONST_RANGE: RangeInclusive<OpCode> = OP_CONSTANT_TRUE..=OP_CONSTANT_NULL;

pub const OP_SPEC_CONSTANT_TRUE: OpCode = 48;
// pub const OP_SPEC_CONSTANT_FALSE: OpCode = 49;
// pub const OP_SPEC_CONSTANT: OpCode = 50;
// pub const OP_SPEC_CONSTANT_COMPOSITE: OpCode = 51;
pub const OP_SPEC_CONSTANT_OP: OpCode = 52;
pub const SPEC_CONST_RANGE: RangeInclusive<OpCode> = OP_SPEC_CONSTANT_TRUE..=OP_SPEC_CONSTANT_OP;

pub const OP_VARIABLE: OpCode = 59;

pub const OP_FUNCTION: OpCode = 54;
pub const OP_FUNCTION_END: OpCode = 56;
pub const OP_FUNCTION_CALL: OpCode = 57;
pub const OP_ACCESS_CHAIN: OpCode = 65;
pub const OP_LOAD: OpCode = 61;
pub const OP_STORE: OpCode = 62;
// pub const OP_IN_BOUNDS_ACCESS_CHAIN: OpCode = 66;

pub const EXEC_MODEL_VERTEX: u32 = 0;
pub const EXEC_MODEL_TESSELLATION_CONTROL: u32 = 1;
pub const EXEC_MODEL_TESSELLATION_EVALUATION: u32 = 2;
pub const EXEC_MODEL_GEOMETRY: u32 = 3;
pub const EXEC_MODEL_FRAGMENT: u32 = 4;
pub const EXEC_MODEL_GL_COMPUTE: u32 = 5;
pub const EXEC_MODEL_KERNEL: u32 = 6;

pub type Decoration = u32;
// pub const DECO_SPEC_ID: Decoration = 1;
pub const DECO_BLOCK: Decoration = 2;
pub const DECO_BUFFER_BLOCK: Decoration = 3;
pub const DECO_ROW_MAJOR: Decoration = 4;
pub const DECO_COL_MAJOR: Decoration = 5;
pub const DECO_ARRAY_STRIDE: Decoration = 6;
pub const DECO_MATRIX_STRIDE: Decoration = 7;
// Don't need this: Built-in variables will not be attribute nor attachment.
// pub const DECO_BUILT_IN: Decoration = 11;
pub const DECO_LOCATION: Decoration = 30;
pub const DECO_BINDING: Decoration = 33;
pub const DECO_DESCRIPTOR_SET: Decoration = 34;
pub const DECO_OFFSET: Decoration = 35;
pub const DECO_INPUT_ATTACHMENT_INDEX: Decoration = 43;


pub type StorageClass = u32;
pub const STORE_CLS_UNIFORM_CONSTANT: StorageClass = 0;
pub const STORE_CLS_INPUT: StorageClass = 1;
pub const STORE_CLS_UNIFORM: StorageClass = 2;
pub const STORE_CLS_OUTPUT: StorageClass = 3;
// Texture calls to sampler object will translate to function class.
// pub const STORE_CLS_FUNCTION: StorageClass = 7;
pub const STORE_CLS_PUSH_CONSTANT: StorageClass = 9;
pub const STORE_CLS_STORAGE_BUFFER: StorageClass = 12;


pub type Dimension = u32;
pub const DIM_IMAGE_1D: Dimension = 0;
pub const DIM_IMAGE_2D: Dimension = 1;
pub const DIM_IMAGE_3D: Dimension = 2;
pub const DIM_IMAGE_CUBE: Dimension = 3;
pub const DIM_IMAGE_SUBPASS_DATA: Dimension = 6;

pub type ColorFormat = u32;
pub const IMG_UNIT_FMT_RGBA32F: ColorFormat = 1;
pub const IMG_UNIT_FMT_R32F: ColorFormat = 3;
pub const IMG_UNIT_FMT_RGBA8: ColorFormat = 4;
