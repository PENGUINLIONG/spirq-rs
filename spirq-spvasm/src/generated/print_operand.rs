use anyhow::{bail, Result};
use spirq_core::parse::Operands;
use super::enum_to_str::enum_to_str;

fn print_id(operands: &mut Operands) -> Result<String> {
    Ok(format!("%{}", operands.read_u32()?))
}
fn print_u32(operands: &mut Operands) -> Result<String> {
    Ok(operands.read_u32()?.to_string())
}
fn print_f32(operands: &mut Operands) -> Result<String> {
    Ok(operands.read_f32()?.to_string())
}
fn print_str(operands: &mut Operands) -> Result<String> {
    Ok(format!(r#""{}""#, operands.read_str()?))
}
fn print_list(operands: &mut Operands) -> Result<Vec<String>> {
    let out = operands.read_list()?
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    Ok(out)
}
fn print_pair_id_id_list(operands: &mut Operands) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for pair in operands.read_list()?.chunks(2) {
        if pair.len() != 2 {
            bail!("operands does not pair up");
        }
        let seg = format!("%{} %{}", pair[0], pair[1]);
        out.push(seg);
    }
    Ok(out)
}
fn print_pair_id_u32_list(operands: &mut Operands) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for pair in operands.read_list()?.chunks(2) {
        if pair.len() != 2 {
            bail!("operands does not pair up");
        }
        let seg = format!("%{} {}", pair[0], pair[1]);
        out.push(seg);
    }
    Ok(out)
}
fn print_pair_u32_id_list(operands: &mut Operands) -> Result<Vec<String>> {
    let mut out = Vec::new();
    for pair in operands.read_list()?.chunks(2) {
        if pair.len() != 2 {
            bail!("operands does not pair up");
        }
        let seg = format!("{} %{}", pair[0], pair[1]);
        out.push(seg);
    }
    Ok(out)
}

pub fn print_operand(opcode: u32, operands: &mut Operands) -> Result<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    match opcode {
        // OpUndef
        1 => {
        }
        // OpSourceContinued
        2 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpSource
        3 => {
            // SourceLanguage
            out.push(enum_to_str("SourceLanguage", operands.read_u32()?)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
            // LiteralString ?
            if !operands.is_empty() {
                out.push(print_str(operands)?);
            }
        }
        // OpSourceExtension
        4 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpName
        5 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpMemberName
        6 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpString
        7 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpLine
        8 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpExtension
        10 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpExtInstImport
        11 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpExtInst
        12 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralExtInstInteger
            out.push(print_u32(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpMemoryModel
        14 => {
            // AddressingModel
            out.push(enum_to_str("AddressingModel", operands.read_u32()?)?);
            // MemoryModel
            out.push(enum_to_str("MemoryModel", operands.read_u32()?)?);
        }
        // OpEntryPoint
        15 => {
            // ExecutionModel
            out.push(enum_to_str("ExecutionModel", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralString
            out.push(print_str(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpExecutionMode
        16 => {
            // IdRef
            out.push(print_id(operands)?);
            // ExecutionMode
            out.push(enum_to_str("ExecutionMode", operands.read_u32()?)?);
        }
        // OpCapability
        17 => {
            // Capability
            out.push(enum_to_str("Capability", operands.read_u32()?)?);
        }
        // OpTypeVoid
        19 => {
        }
        // OpTypeBool
        20 => {
        }
        // OpTypeInt
        21 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpTypeFloat
        22 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpTypeVector
        23 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpTypeMatrix
        24 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpTypeImage
        25 => {
            // IdRef
            out.push(print_id(operands)?);
            // Dim
            out.push(enum_to_str("Dim", operands.read_u32()?)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // ImageFormat
            out.push(enum_to_str("ImageFormat", operands.read_u32()?)?);
            // AccessQualifier ?
            if !operands.is_empty() {
                out.push(enum_to_str("AccessQualifier", operands.read_u32()?)?);
            }
        }
        // OpTypeSampler
        26 => {
        }
        // OpTypeSampledImage
        27 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeArray
        28 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeRuntimeArray
        29 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeStruct
        30 => {
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpTypeOpaque
        31 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpTypePointer
        32 => {
            // StorageClass
            out.push(enum_to_str("StorageClass", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeFunction
        33 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpTypeEvent
        34 => {
        }
        // OpTypeDeviceEvent
        35 => {
        }
        // OpTypeReserveId
        36 => {
        }
        // OpTypeQueue
        37 => {
        }
        // OpTypePipe
        38 => {
            // AccessQualifier
            out.push(enum_to_str("AccessQualifier", operands.read_u32()?)?);
        }
        // OpTypeForwardPointer
        39 => {
            // IdRef
            out.push(print_id(operands)?);
            // StorageClass
            out.push(enum_to_str("StorageClass", operands.read_u32()?)?);
        }
        // OpConstantTrue
        41 => {
        }
        // OpConstantFalse
        42 => {
        }
        // OpConstant
        43 => {
            // LiteralContextDependentNumber
            out.extend(print_list(operands)?);
        }
        // OpConstantComposite
        44 => {
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpConstantSampler
        45 => {
            // SamplerAddressingMode
            out.push(enum_to_str("SamplerAddressingMode", operands.read_u32()?)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // SamplerFilterMode
            out.push(enum_to_str("SamplerFilterMode", operands.read_u32()?)?);
        }
        // OpConstantNull
        46 => {
        }
        // OpSpecConstantTrue
        48 => {
        }
        // OpSpecConstantFalse
        49 => {
        }
        // OpSpecConstant
        50 => {
            // LiteralContextDependentNumber
            out.extend(print_list(operands)?);
        }
        // OpSpecConstantComposite
        51 => {
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpSpecConstantOp
        52 => {
            // LiteralSpecConstantOpInteger
            out.push(print_u32(operands)?);
        }
        // OpFunction
        54 => {
            // FunctionControl
            out.push(enum_to_str("FunctionControl", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFunctionParameter
        55 => {
        }
        // OpFunctionCall
        57 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpVariable
        59 => {
            // StorageClass
            out.push(enum_to_str("StorageClass", operands.read_u32()?)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpImageTexelPointer
        60 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpLoad
        61 => {
            // IdRef
            out.push(print_id(operands)?);
            // MemoryAccess ?
            if !operands.is_empty() {
                out.push(enum_to_str("MemoryAccess", operands.read_u32()?)?);
            }
        }
        // OpStore
        62 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // MemoryAccess ?
            if !operands.is_empty() {
                out.push(enum_to_str("MemoryAccess", operands.read_u32()?)?);
            }
        }
        // OpCopyMemory
        63 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // MemoryAccess ?
            if !operands.is_empty() {
                out.push(enum_to_str("MemoryAccess", operands.read_u32()?)?);
            }
            // MemoryAccess ?
            if !operands.is_empty() {
                out.push(enum_to_str("MemoryAccess", operands.read_u32()?)?);
            }
        }
        // OpCopyMemorySized
        64 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // MemoryAccess ?
            if !operands.is_empty() {
                out.push(enum_to_str("MemoryAccess", operands.read_u32()?)?);
            }
            // MemoryAccess ?
            if !operands.is_empty() {
                out.push(enum_to_str("MemoryAccess", operands.read_u32()?)?);
            }
        }
        // OpAccessChain
        65 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpInBoundsAccessChain
        66 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpPtrAccessChain
        67 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpArrayLength
        68 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpGenericPtrMemSemantics
        69 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpInBoundsPtrAccessChain
        70 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpDecorate
        71 => {
            // IdRef
            out.push(print_id(operands)?);
            // Decoration
            out.push(enum_to_str("Decoration", operands.read_u32()?)?);
        }
        // OpMemberDecorate
        72 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // Decoration
            out.push(enum_to_str("Decoration", operands.read_u32()?)?);
        }
        // OpDecorationGroup
        73 => {
        }
        // OpGroupDecorate
        74 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupMemberDecorate
        75 => {
            // IdRef
            out.push(print_id(operands)?);
            // PairIdRefLiteralInteger *
            while !operands.is_empty() {
                out.extend(print_pair_id_u32_list(operands)?);
            }
        }
        // OpVectorExtractDynamic
        77 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpVectorInsertDynamic
        78 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpVectorShuffle
        79 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger *
            while !operands.is_empty() {
                out.push(print_u32(operands)?);
            }
        }
        // OpCompositeConstruct
        80 => {
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpCompositeExtract
        81 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger *
            while !operands.is_empty() {
                out.push(print_u32(operands)?);
            }
        }
        // OpCompositeInsert
        82 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger *
            while !operands.is_empty() {
                out.push(print_u32(operands)?);
            }
        }
        // OpCopyObject
        83 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTranspose
        84 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSampledImage
        86 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageSampleImplicitLod
        87 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSampleExplicitLod
        88 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands
            out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
        }
        // OpImageSampleDrefImplicitLod
        89 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSampleDrefExplicitLod
        90 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands
            out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
        }
        // OpImageSampleProjImplicitLod
        91 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSampleProjExplicitLod
        92 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands
            out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
        }
        // OpImageSampleProjDrefImplicitLod
        93 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSampleProjDrefExplicitLod
        94 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands
            out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
        }
        // OpImageFetch
        95 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageGather
        96 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageDrefGather
        97 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageRead
        98 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageWrite
        99 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImage
        100 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageQueryFormat
        101 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageQueryOrder
        102 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageQuerySizeLod
        103 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageQuerySize
        104 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageQueryLod
        105 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageQueryLevels
        106 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageQuerySamples
        107 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertFToU
        109 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertFToS
        110 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertSToF
        111 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertUToF
        112 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUConvert
        113 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSConvert
        114 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFConvert
        115 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpQuantizeToF16
        116 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertPtrToU
        117 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSatConvertSToU
        118 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSatConvertUToS
        119 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertUToPtr
        120 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpPtrCastToGeneric
        121 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGenericCastToPtr
        122 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGenericCastToPtrExplicit
        123 => {
            // IdRef
            out.push(print_id(operands)?);
            // StorageClass
            out.push(enum_to_str("StorageClass", operands.read_u32()?)?);
        }
        // OpBitcast
        124 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSNegate
        126 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFNegate
        127 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIAdd
        128 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFAdd
        129 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpISub
        130 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFSub
        131 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIMul
        132 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFMul
        133 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUDiv
        134 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSDiv
        135 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFDiv
        136 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUMod
        137 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSRem
        138 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSMod
        139 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFRem
        140 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFMod
        141 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpVectorTimesScalar
        142 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpMatrixTimesScalar
        143 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpVectorTimesMatrix
        144 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpMatrixTimesVector
        145 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpMatrixTimesMatrix
        146 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpOuterProduct
        147 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpDot
        148 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIAddCarry
        149 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpISubBorrow
        150 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUMulExtended
        151 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSMulExtended
        152 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAny
        154 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAll
        155 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIsNan
        156 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIsInf
        157 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIsFinite
        158 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIsNormal
        159 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSignBitSet
        160 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpLessOrGreater
        161 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpOrdered
        162 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUnordered
        163 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpLogicalEqual
        164 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpLogicalNotEqual
        165 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpLogicalOr
        166 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpLogicalAnd
        167 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpLogicalNot
        168 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSelect
        169 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIEqual
        170 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpINotEqual
        171 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUGreaterThan
        172 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSGreaterThan
        173 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUGreaterThanEqual
        174 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSGreaterThanEqual
        175 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpULessThan
        176 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSLessThan
        177 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpULessThanEqual
        178 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSLessThanEqual
        179 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFOrdEqual
        180 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFUnordEqual
        181 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFOrdNotEqual
        182 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFUnordNotEqual
        183 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFOrdLessThan
        184 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFUnordLessThan
        185 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFOrdGreaterThan
        186 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFUnordGreaterThan
        187 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFOrdLessThanEqual
        188 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFUnordLessThanEqual
        189 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFOrdGreaterThanEqual
        190 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFUnordGreaterThanEqual
        191 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpShiftRightLogical
        194 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpShiftRightArithmetic
        195 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpShiftLeftLogical
        196 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpBitwiseOr
        197 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpBitwiseXor
        198 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpBitwiseAnd
        199 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpNot
        200 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpBitFieldInsert
        201 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpBitFieldSExtract
        202 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpBitFieldUExtract
        203 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpBitReverse
        204 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpBitCount
        205 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpDPdx
        207 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpDPdy
        208 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFwidth
        209 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpDPdxFine
        210 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpDPdyFine
        211 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFwidthFine
        212 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpDPdxCoarse
        213 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpDPdyCoarse
        214 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFwidthCoarse
        215 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpEmitStreamVertex
        220 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpEndStreamPrimitive
        221 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpControlBarrier
        224 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
        }
        // OpMemoryBarrier
        225 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
        }
        // OpAtomicLoad
        227 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
        }
        // OpAtomicStore
        228 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicExchange
        229 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicCompareExchange
        230 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicCompareExchangeWeak
        231 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicIIncrement
        232 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
        }
        // OpAtomicIDecrement
        233 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
        }
        // OpAtomicIAdd
        234 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicISub
        235 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicSMin
        236 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicUMin
        237 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicSMax
        238 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicUMax
        239 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicAnd
        240 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicOr
        241 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicXor
        242 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpPhi
        245 => {
            // PairIdRefIdRef *
            while !operands.is_empty() {
                out.extend(print_pair_id_id_list(operands)?);
            }
        }
        // OpLoopMerge
        246 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LoopControl
            out.push(enum_to_str("LoopControl", operands.read_u32()?)?);
        }
        // OpSelectionMerge
        247 => {
            // IdRef
            out.push(print_id(operands)?);
            // SelectionControl
            out.push(enum_to_str("SelectionControl", operands.read_u32()?)?);
        }
        // OpLabel
        248 => {
        }
        // OpBranch
        249 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpBranchConditional
        250 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger *
            while !operands.is_empty() {
                out.push(print_u32(operands)?);
            }
        }
        // OpSwitch
        251 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // PairLiteralIntegerIdRef *
            while !operands.is_empty() {
                out.extend(print_pair_u32_id_list(operands)?);
            }
        }
        // OpReturnValue
        254 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpLifetimeStart
        256 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpLifetimeStop
        257 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpGroupAsyncCopy
        259 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupWaitEvents
        260 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupAll
        261 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupAny
        262 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupBroadcast
        263 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupIAdd
        264 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupFAdd
        265 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupFMin
        266 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupUMin
        267 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupSMin
        268 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupFMax
        269 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupUMax
        270 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupSMax
        271 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpReadPipe
        274 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpWritePipe
        275 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpReservedReadPipe
        276 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpReservedWritePipe
        277 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpReserveReadPipePackets
        278 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpReserveWritePipePackets
        279 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpCommitReadPipe
        280 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpCommitWritePipe
        281 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIsValidReserveId
        282 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGetNumPipePackets
        283 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGetMaxPipePackets
        284 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupReserveReadPipePackets
        285 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupReserveWritePipePackets
        286 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupCommitReadPipe
        287 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupCommitWritePipe
        288 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpEnqueueMarker
        291 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpEnqueueKernel
        292 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGetKernelNDrangeSubGroupCount
        293 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGetKernelNDrangeMaxSubGroupSize
        294 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGetKernelWorkGroupSize
        295 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGetKernelPreferredWorkGroupSizeMultiple
        296 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRetainEvent
        297 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpReleaseEvent
        298 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpCreateUserEvent
        299 => {
        }
        // OpIsValidEvent
        300 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSetUserEventStatus
        301 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpCaptureEventProfilingInfo
        302 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGetDefaultQueue
        303 => {
        }
        // OpBuildNDRange
        304 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageSparseSampleImplicitLod
        305 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSparseSampleExplicitLod
        306 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands
            out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
        }
        // OpImageSparseSampleDrefImplicitLod
        307 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSparseSampleDrefExplicitLod
        308 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands
            out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
        }
        // OpImageSparseSampleProjImplicitLod
        309 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSparseSampleProjExplicitLod
        310 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands
            out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
        }
        // OpImageSparseSampleProjDrefImplicitLod
        311 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSparseSampleProjDrefExplicitLod
        312 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands
            out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
        }
        // OpImageSparseFetch
        313 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSparseGather
        314 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSparseDrefGather
        315 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpImageSparseTexelsResident
        316 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicFlagTestAndSet
        318 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
        }
        // OpAtomicFlagClear
        319 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
        }
        // OpImageSparseRead
        320 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpSizeOf
        321 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypePipeStorage
        322 => {
        }
        // OpConstantPipeStorage
        323 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpCreatePipeFromPipeStorage
        324 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGetKernelLocalSizeForSubgroupCount
        325 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGetKernelMaxNumSubgroups
        326 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeNamedBarrier
        327 => {
        }
        // OpNamedBarrierInitialize
        328 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpMemoryNamedBarrier
        329 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
        }
        // OpModuleProcessed
        330 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpExecutionModeId
        331 => {
            // IdRef
            out.push(print_id(operands)?);
            // ExecutionMode
            out.push(enum_to_str("ExecutionMode", operands.read_u32()?)?);
        }
        // OpDecorateId
        332 => {
            // IdRef
            out.push(print_id(operands)?);
            // Decoration
            out.push(enum_to_str("Decoration", operands.read_u32()?)?);
        }
        // OpGroupNonUniformElect
        333 => {
            // IdScope
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformAll
        334 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformAny
        335 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformAllEqual
        336 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformBroadcast
        337 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformBroadcastFirst
        338 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformBallot
        339 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformInverseBallot
        340 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformBallotBitExtract
        341 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformBallotBitCount
        342 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformBallotFindLSB
        343 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformBallotFindMSB
        344 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformShuffle
        345 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformShuffleXor
        346 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformShuffleUp
        347 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformShuffleDown
        348 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformIAdd
        349 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformFAdd
        350 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformIMul
        351 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformFMul
        352 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformSMin
        353 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformUMin
        354 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformFMin
        355 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformSMax
        356 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformUMax
        357 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformFMax
        358 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformBitwiseAnd
        359 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformBitwiseOr
        360 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformBitwiseXor
        361 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformLogicalAnd
        362 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformLogicalOr
        363 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformLogicalXor
        364 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpGroupNonUniformQuadBroadcast
        365 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformQuadSwap
        366 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpCopyLogical
        400 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpPtrEqual
        401 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpPtrNotEqual
        402 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpPtrDiff
        403 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpColorAttachmentReadEXT
        4160 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpDepthAttachmentReadEXT
        4161 => {
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpStencilAttachmentReadEXT
        4162 => {
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpSubgroupBallotKHR
        4421 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupFirstInvocationKHR
        4422 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAllKHR
        4428 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAnyKHR
        4429 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAllEqualKHR
        4430 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformRotateKHR
        4431 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpSubgroupReadInvocationKHR
        4432 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTraceRayKHR
        4445 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpExecuteCallableKHR
        4446 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertUToAccelerationStructureKHR
        4447 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSDotKHR
        4450 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // PackedVectorFormat ?
            if !operands.is_empty() {
                out.push(enum_to_str("PackedVectorFormat", operands.read_u32()?)?);
            }
        }
        // OpUDotKHR
        4451 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // PackedVectorFormat ?
            if !operands.is_empty() {
                out.push(enum_to_str("PackedVectorFormat", operands.read_u32()?)?);
            }
        }
        // OpSUDotKHR
        4452 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // PackedVectorFormat ?
            if !operands.is_empty() {
                out.push(enum_to_str("PackedVectorFormat", operands.read_u32()?)?);
            }
        }
        // OpSDotAccSatKHR
        4453 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // PackedVectorFormat ?
            if !operands.is_empty() {
                out.push(enum_to_str("PackedVectorFormat", operands.read_u32()?)?);
            }
        }
        // OpUDotAccSatKHR
        4454 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // PackedVectorFormat ?
            if !operands.is_empty() {
                out.push(enum_to_str("PackedVectorFormat", operands.read_u32()?)?);
            }
        }
        // OpSUDotAccSatKHR
        4455 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // PackedVectorFormat ?
            if !operands.is_empty() {
                out.push(enum_to_str("PackedVectorFormat", operands.read_u32()?)?);
            }
        }
        // OpTypeCooperativeMatrixKHR
        4456 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpCooperativeMatrixLoadKHR
        4457 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
            // MemoryAccess ?
            if !operands.is_empty() {
                out.push(enum_to_str("MemoryAccess", operands.read_u32()?)?);
            }
        }
        // OpCooperativeMatrixStoreKHR
        4458 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
            // MemoryAccess ?
            if !operands.is_empty() {
                out.push(enum_to_str("MemoryAccess", operands.read_u32()?)?);
            }
        }
        // OpCooperativeMatrixMulAddKHR
        4459 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // CooperativeMatrixOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("CooperativeMatrixOperands", operands.read_u32()?)?);
            }
        }
        // OpCooperativeMatrixLengthKHR
        4460 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeRayQueryKHR
        4472 => {
        }
        // OpRayQueryInitializeKHR
        4473 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryTerminateKHR
        4474 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGenerateIntersectionKHR
        4475 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryConfirmIntersectionKHR
        4476 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryProceedKHR
        4477 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionTypeKHR
        4479 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageSampleWeightedQCOM
        4480 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageBoxFilterQCOM
        4481 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageBlockMatchSSDQCOM
        4482 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpImageBlockMatchSADQCOM
        4483 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupIAddNonUniformAMD
        5000 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupFAddNonUniformAMD
        5001 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupFMinNonUniformAMD
        5002 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupUMinNonUniformAMD
        5003 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupSMinNonUniformAMD
        5004 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupFMaxNonUniformAMD
        5005 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupUMaxNonUniformAMD
        5006 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupSMaxNonUniformAMD
        5007 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFragmentMaskFetchAMD
        5011 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFragmentFetchAMD
        5012 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpReadClockKHR
        5056 => {
            // IdScope
            out.push(print_id(operands)?);
        }
        // OpFinalizeNodePayloadsAMDX
        5075 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFinishWritingNodePayloadAMDX
        5078 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpInitializeNodePayloadsAMDX
        5090 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectRecordHitMotionNV
        5249 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectRecordHitWithIndexMotionNV
        5250 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectRecordMissMotionNV
        5251 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetWorldToObjectNV
        5252 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetObjectToWorldNV
        5253 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetObjectRayDirectionNV
        5254 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetObjectRayOriginNV
        5255 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectTraceRayMotionNV
        5256 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetShaderRecordBufferHandleNV
        5257 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetShaderBindingTableRecordIndexNV
        5258 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectRecordEmptyNV
        5259 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectTraceRayNV
        5260 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectRecordHitNV
        5261 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectRecordHitWithIndexNV
        5262 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectRecordMissNV
        5263 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectExecuteShaderNV
        5264 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetCurrentTimeNV
        5265 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetAttributesNV
        5266 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetHitKindNV
        5267 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetPrimitiveIndexNV
        5268 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetGeometryIndexNV
        5269 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetInstanceIdNV
        5270 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetInstanceCustomIndexNV
        5271 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetWorldRayDirectionNV
        5272 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetWorldRayOriginNV
        5273 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetRayTMaxNV
        5274 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectGetRayTMinNV
        5275 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectIsEmptyNV
        5276 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectIsHitNV
        5277 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpHitObjectIsMissNV
        5278 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpReorderThreadWithHitObjectNV
        5279 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpReorderThreadWithHintNV
        5280 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeHitObjectNV
        5281 => {
        }
        // OpImageSampleFootprintNV
        5283 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // ImageOperands ?
            if !operands.is_empty() {
                out.push(enum_to_str("ImageOperands", operands.read_u32()?)?);
            }
        }
        // OpEmitMeshTasksEXT
        5294 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpSetMeshOutputsEXT
        5295 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupNonUniformPartitionNV
        5296 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpWritePackedPrimitiveIndices4x8NV
        5299 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFetchMicroTriangleVertexPositionNV
        5300 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFetchMicroTriangleVertexBarycentricNV
        5301 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpReportIntersectionKHR
        5334 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTraceNV
        5337 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTraceMotionNV
        5338 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTraceRayMotionNV
        5339 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionTriangleVertexPositionsKHR
        5340 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeAccelerationStructureKHR
        5341 => {
        }
        // OpExecuteCallableNV
        5344 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeCooperativeMatrixNV
        5358 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpCooperativeMatrixLoadNV
        5359 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // MemoryAccess ?
            if !operands.is_empty() {
                out.push(enum_to_str("MemoryAccess", operands.read_u32()?)?);
            }
        }
        // OpCooperativeMatrixStoreNV
        5360 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // MemoryAccess ?
            if !operands.is_empty() {
                out.push(enum_to_str("MemoryAccess", operands.read_u32()?)?);
            }
        }
        // OpCooperativeMatrixMulAddNV
        5361 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpCooperativeMatrixLengthNV
        5362 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIsHelperInvocationEXT
        5381 => {
        }
        // OpConvertUToImageNV
        5391 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertUToSamplerNV
        5392 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertImageToUNV
        5393 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertSamplerToUNV
        5394 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertUToSampledImageNV
        5395 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertSampledImageToUNV
        5396 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSamplerImageAddressingModeNV
        5397 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpSubgroupShuffleINTEL
        5571 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupShuffleDownINTEL
        5572 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupShuffleUpINTEL
        5573 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupShuffleXorINTEL
        5574 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupBlockReadINTEL
        5575 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupBlockWriteINTEL
        5576 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupImageBlockReadINTEL
        5577 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupImageBlockWriteINTEL
        5578 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupImageMediaBlockReadINTEL
        5580 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupImageMediaBlockWriteINTEL
        5581 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUCountLeadingZerosINTEL
        5585 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUCountTrailingZerosINTEL
        5586 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAbsISubINTEL
        5587 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAbsUSubINTEL
        5588 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIAddSatINTEL
        5589 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUAddSatINTEL
        5590 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIAverageINTEL
        5591 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUAverageINTEL
        5592 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIAverageRoundedINTEL
        5593 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUAverageRoundedINTEL
        5594 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpISubSatINTEL
        5595 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUSubSatINTEL
        5596 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpIMul32x16INTEL
        5597 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpUMul32x16INTEL
        5598 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConstantFunctionPointerINTEL
        5600 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFunctionPointerCallINTEL
        5601 => {
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpAsmTargetINTEL
        5609 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpAsmINTEL
        5610 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralString
            out.push(print_str(operands)?);
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpAsmCallINTEL
        5611 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpAtomicFMinEXT
        5614 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicFMaxEXT
        5615 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAssumeTrueKHR
        5630 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpExpectKHR
        5631 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpDecorateStringGOOGLE
        5632 => {
            // IdRef
            out.push(print_id(operands)?);
            // Decoration
            out.push(enum_to_str("Decoration", operands.read_u32()?)?);
        }
        // OpMemberDecorateStringGOOGLE
        5633 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // Decoration
            out.push(enum_to_str("Decoration", operands.read_u32()?)?);
        }
        // OpVmeImageINTEL
        5699 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeVmeImageINTEL
        5700 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeAvcImePayloadINTEL
        5701 => {
        }
        // OpTypeAvcRefPayloadINTEL
        5702 => {
        }
        // OpTypeAvcSicPayloadINTEL
        5703 => {
        }
        // OpTypeAvcMcePayloadINTEL
        5704 => {
        }
        // OpTypeAvcMceResultINTEL
        5705 => {
        }
        // OpTypeAvcImeResultINTEL
        5706 => {
        }
        // OpTypeAvcImeResultSingleReferenceStreamoutINTEL
        5707 => {
        }
        // OpTypeAvcImeResultDualReferenceStreamoutINTEL
        5708 => {
        }
        // OpTypeAvcImeSingleReferenceStreaminINTEL
        5709 => {
        }
        // OpTypeAvcImeDualReferenceStreaminINTEL
        5710 => {
        }
        // OpTypeAvcRefResultINTEL
        5711 => {
        }
        // OpTypeAvcSicResultINTEL
        5712 => {
        }
        // OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL
        5713 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL
        5714 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL
        5715 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceSetInterShapePenaltyINTEL
        5716 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL
        5717 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceSetInterDirectionPenaltyINTEL
        5718 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL
        5719 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL
        5720 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL
        5721 => {
        }
        // OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL
        5722 => {
        }
        // OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL
        5723 => {
        }
        // OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL
        5724 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL
        5725 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL
        5726 => {
        }
        // OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL
        5727 => {
        }
        // OpSubgroupAvcMceSetAcOnlyHaarINTEL
        5728 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL
        5729 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL
        5730 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL
        5731 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceConvertToImePayloadINTEL
        5732 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceConvertToImeResultINTEL
        5733 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceConvertToRefPayloadINTEL
        5734 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceConvertToRefResultINTEL
        5735 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceConvertToSicPayloadINTEL
        5736 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceConvertToSicResultINTEL
        5737 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetMotionVectorsINTEL
        5738 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetInterDistortionsINTEL
        5739 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetBestInterDistortionsINTEL
        5740 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetInterMajorShapeINTEL
        5741 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetInterMinorShapeINTEL
        5742 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetInterDirectionsINTEL
        5743 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetInterMotionVectorCountINTEL
        5744 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetInterReferenceIdsINTEL
        5745 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL
        5746 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeInitializeINTEL
        5747 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeSetSingleReferenceINTEL
        5748 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeSetDualReferenceINTEL
        5749 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeRefWindowSizeINTEL
        5750 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeAdjustRefOffsetINTEL
        5751 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeConvertToMcePayloadINTEL
        5752 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeSetMaxMotionVectorCountINTEL
        5753 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL
        5754 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL
        5755 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeSetWeightedSadINTEL
        5756 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL
        5757 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeEvaluateWithDualReferenceINTEL
        5758 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL
        5759 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL
        5760 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL
        5761 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL
        5762 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL
        5763 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL
        5764 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeConvertToMceResultINTEL
        5765 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetSingleReferenceStreaminINTEL
        5766 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetDualReferenceStreaminINTEL
        5767 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL
        5768 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeStripDualReferenceStreamoutINTEL
        5769 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL
        5770 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL
        5771 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL
        5772 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL
        5773 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL
        5774 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL
        5775 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetBorderReachedINTEL
        5776 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL
        5777 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL
        5778 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL
        5779 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL
        5780 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcFmeInitializeINTEL
        5781 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcBmeInitializeINTEL
        5782 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcRefConvertToMcePayloadINTEL
        5783 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcRefSetBidirectionalMixDisableINTEL
        5784 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcRefSetBilinearFilterEnableINTEL
        5785 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL
        5786 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcRefEvaluateWithDualReferenceINTEL
        5787 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL
        5788 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL
        5789 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcRefConvertToMceResultINTEL
        5790 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicInitializeINTEL
        5791 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicConfigureSkcINTEL
        5792 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicConfigureIpeLumaINTEL
        5793 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicConfigureIpeLumaChromaINTEL
        5794 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicGetMotionVectorMaskINTEL
        5795 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicConvertToMcePayloadINTEL
        5796 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL
        5797 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL
        5798 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL
        5799 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicSetBilinearFilterEnableINTEL
        5800 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL
        5801 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL
        5802 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicEvaluateIpeINTEL
        5803 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL
        5804 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicEvaluateWithDualReferenceINTEL
        5805 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL
        5806 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL
        5807 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicConvertToMceResultINTEL
        5808 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicGetIpeLumaShapeINTEL
        5809 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL
        5810 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL
        5811 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicGetPackedIpeLumaModesINTEL
        5812 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicGetIpeChromaModeINTEL
        5813 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL
        5814 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL
        5815 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSubgroupAvcSicGetInterRawSadsINTEL
        5816 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpVariableLengthArrayINTEL
        5818 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpSaveMemoryINTEL
        5819 => {
        }
        // OpRestoreMemoryINTEL
        5820 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpArbitraryFloatSinCosPiINTEL
        5840 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatCastINTEL
        5841 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatCastFromIntINTEL
        5842 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatCastToIntINTEL
        5843 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatAddINTEL
        5846 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatSubINTEL
        5847 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatMulINTEL
        5848 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatDivINTEL
        5849 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatGTINTEL
        5850 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatGEINTEL
        5851 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatLTINTEL
        5852 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatLEINTEL
        5853 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatEQINTEL
        5854 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatRecipINTEL
        5855 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatRSqrtINTEL
        5856 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatCbrtINTEL
        5857 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatHypotINTEL
        5858 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatSqrtINTEL
        5859 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatLogINTEL
        5860 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatLog2INTEL
        5861 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatLog10INTEL
        5862 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatLog1pINTEL
        5863 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatExpINTEL
        5864 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatExp2INTEL
        5865 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatExp10INTEL
        5866 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatExpm1INTEL
        5867 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatSinINTEL
        5868 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatCosINTEL
        5869 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatSinCosINTEL
        5870 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatSinPiINTEL
        5871 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatCosPiINTEL
        5872 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatASinINTEL
        5873 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatASinPiINTEL
        5874 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatACosINTEL
        5875 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatACosPiINTEL
        5876 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatATanINTEL
        5877 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatATanPiINTEL
        5878 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatATan2INTEL
        5879 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatPowINTEL
        5880 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatPowRINTEL
        5881 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatPowNINTEL
        5882 => {
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpLoopControlINTEL
        5887 => {
            // LiteralInteger *
            while !operands.is_empty() {
                out.push(print_u32(operands)?);
            }
        }
        // OpAliasDomainDeclINTEL
        5911 => {
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpAliasScopeDeclINTEL
        5912 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef ?
            if !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpAliasScopeListDeclINTEL
        5913 => {
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpFixedSqrtINTEL
        5923 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpFixedRecipINTEL
        5924 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpFixedRsqrtINTEL
        5925 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpFixedSinINTEL
        5926 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpFixedCosINTEL
        5927 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpFixedSinCosINTEL
        5928 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpFixedSinPiINTEL
        5929 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpFixedCosPiINTEL
        5930 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpFixedSinCosPiINTEL
        5931 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpFixedLogINTEL
        5932 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpFixedExpINTEL
        5933 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpPtrCastToCrossWorkgroupINTEL
        5934 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpCrossWorkgroupCastToPtrINTEL
        5938 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpReadPipeBlockingINTEL
        5946 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpWritePipeBlockingINTEL
        5947 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpFPGARegINTEL
        5949 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetRayTMinKHR
        6016 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetRayFlagsKHR
        6017 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionTKHR
        6018 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionInstanceCustomIndexKHR
        6019 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionInstanceIdKHR
        6020 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR
        6021 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionGeometryIndexKHR
        6022 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionPrimitiveIndexKHR
        6023 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionBarycentricsKHR
        6024 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionFrontFaceKHR
        6025 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionCandidateAABBOpaqueKHR
        6026 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionObjectRayDirectionKHR
        6027 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionObjectRayOriginKHR
        6028 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetWorldRayDirectionKHR
        6029 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetWorldRayOriginKHR
        6030 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionObjectToWorldKHR
        6031 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpRayQueryGetIntersectionWorldToObjectKHR
        6032 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpAtomicFAddEXT
        6035 => {
            // IdRef
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpTypeBufferSurfaceINTEL
        6086 => {
            // AccessQualifier
            out.push(enum_to_str("AccessQualifier", operands.read_u32()?)?);
        }
        // OpTypeStructContinuedINTEL
        6090 => {
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpConstantCompositeContinuedINTEL
        6091 => {
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpSpecConstantCompositeContinuedINTEL
        6092 => {
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpCompositeConstructContinuedINTEL
        6096 => {
            // IdRef *
            while !operands.is_empty() {
                out.push(print_id(operands)?);
            }
        }
        // OpConvertFToBF16INTEL
        6116 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpConvertBF16ToFINTEL
        6117 => {
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpControlBarrierArriveINTEL
        6142 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
        }
        // OpControlBarrierWaitINTEL
        6143 => {
            // IdScope
            out.push(print_id(operands)?);
            // IdScope
            out.push(print_id(operands)?);
            // IdMemorySemantics
            out.push(print_id(operands)?);
        }
        // OpGroupIMulKHR
        6401 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupFMulKHR
        6402 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupBitwiseAndKHR
        6403 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupBitwiseOrKHR
        6404 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupBitwiseXorKHR
        6405 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupLogicalAndKHR
        6406 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupLogicalOrKHR
        6407 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        // OpGroupLogicalXorKHR
        6408 => {
            // IdScope
            out.push(print_id(operands)?);
            // GroupOperation
            out.push(enum_to_str("GroupOperation", operands.read_u32()?)?);
            // IdRef
            out.push(print_id(operands)?);
        }
        _ => bail!("unsupported opcode {}", opcode),
    };
    while !operands.is_empty() {
        out.push(format!("!{}", operands.read_u32()?));
    }
    Ok(out)
}
