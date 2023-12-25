use super::enum_to_str::enum_to_str;
use anyhow::{bail, Result};
use spirq_core::parse::Operands;
use std::collections::HashMap;

fn print_id(operands: &mut Operands, id_names: &HashMap<u32, String>) -> Result<String> {
    let id = operands.read_u32()?;
    if let Some(name) = id_names.get(&id) {
        Ok(format!("%{}", name))
    } else {
        Ok(format!("%{}", id))
    }
}
fn print_u32(operands: &mut Operands) -> Result<String> {
    Ok(operands.read_u32()?.to_string())
}
#[allow(dead_code)]
fn print_f32(operands: &mut Operands) -> Result<String> {
    Ok(operands.read_f32()?.to_string())
}
fn print_str(operands: &mut Operands) -> Result<String> {
    Ok(format!(r#""{}""#, operands.read_str()?))
}
fn print_list(operands: &mut Operands) -> Result<Vec<String>> {
    let out = operands
        .read_list()?
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>();
    Ok(out)
}
fn print_pair_id_id_list(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let mut out = Vec::new();
    out.push(print_id(operands, id_names)?);
    out.push(print_id(operands, id_names)?);
    Ok(out)
}
fn print_pair_id_u32_list(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let mut out = Vec::new();
    out.push(print_id(operands, id_names)?);
    out.push(print_u32(operands)?);
    Ok(out)
}
fn print_pair_u32_id_list(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let mut out = Vec::new();
    out.push(print_u32(operands)?);
    out.push(print_id(operands, id_names)?);
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_ImageOperands(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"ImageOperands", value)?];
    // None
    if value & 0x0000 != 0 {}
    // Bias
    if value & 0x0001 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    // Lod
    if value & 0x0002 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    // Grad
    if value & 0x0004 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    // ConstOffset
    if value & 0x0008 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    // Offset
    if value & 0x0010 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    // ConstOffsets
    if value & 0x0020 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    // Sample
    if value & 0x0040 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    // MinLod
    if value & 0x0080 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    // MakeTexelAvailableKHR
    if value & 0x0100 != 0 {
        // IdScope
        out.push(print_id(operands, id_names)?);
    }
    // MakeTexelVisibleKHR
    if value & 0x0200 != 0 {
        // IdScope
        out.push(print_id(operands, id_names)?);
    }
    // NonPrivateTexelKHR
    if value & 0x0400 != 0 {}
    // VolatileTexelKHR
    if value & 0x0800 != 0 {}
    // SignExtend
    if value & 0x1000 != 0 {}
    // ZeroExtend
    if value & 0x2000 != 0 {}
    // Nontemporal
    if value & 0x4000 != 0 {}
    // Offsets
    if value & 0x10000 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_FPFastMathMode(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"FPFastMathMode", value)?];
    // None
    if value & 0x0000 != 0 {}
    // NotNaN
    if value & 0x0001 != 0 {}
    // NotInf
    if value & 0x0002 != 0 {}
    // NSZ
    if value & 0x0004 != 0 {}
    // AllowRecip
    if value & 0x0008 != 0 {}
    // Fast
    if value & 0x0010 != 0 {}
    // AllowContractFastINTEL
    if value & 0x10000 != 0 {}
    // AllowReassocINTEL
    if value & 0x20000 != 0 {}
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_SelectionControl(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"SelectionControl", value)?];
    // None
    if value & 0x0000 != 0 {}
    // Flatten
    if value & 0x0001 != 0 {}
    // DontFlatten
    if value & 0x0002 != 0 {}
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_LoopControl(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"LoopControl", value)?];
    // None
    if value & 0x0000 != 0 {}
    // Unroll
    if value & 0x0001 != 0 {}
    // DontUnroll
    if value & 0x0002 != 0 {}
    // DependencyInfinite
    if value & 0x0004 != 0 {}
    // DependencyLength
    if value & 0x0008 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // MinIterations
    if value & 0x0010 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // MaxIterations
    if value & 0x0020 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // IterationMultiple
    if value & 0x0040 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // PeelCount
    if value & 0x0080 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // PartialCount
    if value & 0x0100 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // InitiationIntervalINTEL
    if value & 0x10000 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // MaxConcurrencyINTEL
    if value & 0x20000 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // DependencyArrayINTEL
    if value & 0x40000 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // PipelineEnableINTEL
    if value & 0x80000 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // LoopCoalesceINTEL
    if value & 0x100000 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // MaxInterleavingINTEL
    if value & 0x200000 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // SpeculatedIterationsINTEL
    if value & 0x400000 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // NoFusionINTEL
    if value & 0x800000 != 0 {}
    // LoopCountINTEL
    if value & 0x1000000 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // MaxReinvocationDelayINTEL
    if value & 0x2000000 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_FunctionControl(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"FunctionControl", value)?];
    // None
    if value & 0x0000 != 0 {}
    // Inline
    if value & 0x0001 != 0 {}
    // DontInline
    if value & 0x0002 != 0 {}
    // Pure
    if value & 0x0004 != 0 {}
    // Const
    if value & 0x0008 != 0 {}
    // OptNoneINTEL
    if value & 0x10000 != 0 {}
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_MemorySemantics(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"MemorySemantics", value)?];
    // None
    if value & 0x0000 != 0 {}
    // Acquire
    if value & 0x0002 != 0 {}
    // Release
    if value & 0x0004 != 0 {}
    // AcquireRelease
    if value & 0x0008 != 0 {}
    // SequentiallyConsistent
    if value & 0x0010 != 0 {}
    // UniformMemory
    if value & 0x0040 != 0 {}
    // SubgroupMemory
    if value & 0x0080 != 0 {}
    // WorkgroupMemory
    if value & 0x0100 != 0 {}
    // CrossWorkgroupMemory
    if value & 0x0200 != 0 {}
    // AtomicCounterMemory
    if value & 0x0400 != 0 {}
    // ImageMemory
    if value & 0x0800 != 0 {}
    // OutputMemoryKHR
    if value & 0x1000 != 0 {}
    // MakeAvailableKHR
    if value & 0x2000 != 0 {}
    // MakeVisibleKHR
    if value & 0x4000 != 0 {}
    // Volatile
    if value & 0x8000 != 0 {}
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_MemoryAccess(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"MemoryAccess", value)?];
    // None
    if value & 0x0000 != 0 {}
    // Volatile
    if value & 0x0001 != 0 {}
    // Aligned
    if value & 0x0002 != 0 {
        // LiteralInteger
        out.push(print_u32(operands)?);
    }
    // Nontemporal
    if value & 0x0004 != 0 {}
    // MakePointerAvailableKHR
    if value & 0x0008 != 0 {
        // IdScope
        out.push(print_id(operands, id_names)?);
    }
    // MakePointerVisibleKHR
    if value & 0x0010 != 0 {
        // IdScope
        out.push(print_id(operands, id_names)?);
    }
    // NonPrivatePointerKHR
    if value & 0x0020 != 0 {}
    // AliasScopeINTELMask
    if value & 0x10000 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    // NoAliasINTELMask
    if value & 0x20000 != 0 {
        // IdRef
        out.push(print_id(operands, id_names)?);
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_KernelProfilingInfo(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"KernelProfilingInfo", value)?];
    // None
    if value & 0x0000 != 0 {}
    // CmdExecTime
    if value & 0x0001 != 0 {}
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_RayFlags(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"RayFlags", value)?];
    // NoneKHR
    if value & 0x0000 != 0 {}
    // OpaqueKHR
    if value & 0x0001 != 0 {}
    // NoOpaqueKHR
    if value & 0x0002 != 0 {}
    // TerminateOnFirstHitKHR
    if value & 0x0004 != 0 {}
    // SkipClosestHitShaderKHR
    if value & 0x0008 != 0 {}
    // CullBackFacingTrianglesKHR
    if value & 0x0010 != 0 {}
    // CullFrontFacingTrianglesKHR
    if value & 0x0020 != 0 {}
    // CullOpaqueKHR
    if value & 0x0040 != 0 {}
    // CullNoOpaqueKHR
    if value & 0x0080 != 0 {}
    // SkipTrianglesKHR
    if value & 0x0100 != 0 {}
    // SkipAABBsKHR
    if value & 0x0200 != 0 {}
    // ForceOpacityMicromap2StateEXT
    if value & 0x0400 != 0 {}
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_FragmentShadingRate(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"FragmentShadingRate", value)?];
    // Vertical2Pixels
    if value & 0x0001 != 0 {}
    // Vertical4Pixels
    if value & 0x0002 != 0 {}
    // Horizontal2Pixels
    if value & 0x0004 != 0 {}
    // Horizontal4Pixels
    if value & 0x0008 != 0 {}
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_SourceLanguage(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"SourceLanguage", value)?];
    match value {
        // Unknown
        0 => {}
        // ESSL
        1 => {}
        // GLSL
        2 => {}
        // OpenCL_C
        3 => {}
        // OpenCL_CPP
        4 => {}
        // HLSL
        5 => {}
        // CPP_for_OpenCL
        6 => {}
        // SYCL
        7 => {}
        // HERO_C
        8 => {}
        // NZSL
        9 => {}
        // WGSL
        10 => {}
        // Slang
        11 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_ExecutionModel(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"ExecutionModel", value)?];
    match value {
        // Vertex
        0 => {}
        // TessellationControl
        1 => {}
        // TessellationEvaluation
        2 => {}
        // Geometry
        3 => {}
        // Fragment
        4 => {}
        // GLCompute
        5 => {}
        // Kernel
        6 => {}
        // TaskNV
        5267 => {}
        // MeshNV
        5268 => {}
        // RayGenerationKHR
        5313 => {}
        // IntersectionKHR
        5314 => {}
        // AnyHitKHR
        5315 => {}
        // ClosestHitKHR
        5316 => {}
        // MissKHR
        5317 => {}
        // CallableKHR
        5318 => {}
        // TaskEXT
        5364 => {}
        // MeshEXT
        5365 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_AddressingModel(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"AddressingModel", value)?];
    match value {
        // Logical
        0 => {}
        // Physical32
        1 => {}
        // Physical64
        2 => {}
        // PhysicalStorageBuffer64EXT
        5348 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_MemoryModel(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"MemoryModel", value)?];
    match value {
        // Simple
        0 => {}
        // GLSL450
        1 => {}
        // OpenCL
        2 => {}
        // VulkanKHR
        3 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_ExecutionMode(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"ExecutionMode", value)?];
    match value {
        // Invocations
        0 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // SpacingEqual
        1 => {}
        // SpacingFractionalEven
        2 => {}
        // SpacingFractionalOdd
        3 => {}
        // VertexOrderCw
        4 => {}
        // VertexOrderCcw
        5 => {}
        // PixelCenterInteger
        6 => {}
        // OriginUpperLeft
        7 => {}
        // OriginLowerLeft
        8 => {}
        // EarlyFragmentTests
        9 => {}
        // PointMode
        10 => {}
        // Xfb
        11 => {}
        // DepthReplacing
        12 => {}
        // DepthGreater
        14 => {}
        // DepthLess
        15 => {}
        // DepthUnchanged
        16 => {}
        // LocalSize
        17 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // LocalSizeHint
        18 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // InputPoints
        19 => {}
        // InputLines
        20 => {}
        // InputLinesAdjacency
        21 => {}
        // Triangles
        22 => {}
        // InputTrianglesAdjacency
        23 => {}
        // Quads
        24 => {}
        // Isolines
        25 => {}
        // OutputVertices
        26 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OutputPoints
        27 => {}
        // OutputLineStrip
        28 => {}
        // OutputTriangleStrip
        29 => {}
        // VecTypeHint
        30 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // ContractionOff
        31 => {}
        // Initializer
        33 => {}
        // Finalizer
        34 => {}
        // SubgroupSize
        35 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // SubgroupsPerWorkgroup
        36 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // SubgroupsPerWorkgroupId
        37 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // LocalSizeId
        38 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // LocalSizeHintId
        39 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // NonCoherentColorAttachmentReadEXT
        4169 => {}
        // NonCoherentDepthAttachmentReadEXT
        4170 => {}
        // NonCoherentStencilAttachmentReadEXT
        4171 => {}
        // SubgroupUniformControlFlowKHR
        4421 => {}
        // PostDepthCoverage
        4446 => {}
        // DenormPreserve
        4459 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // DenormFlushToZero
        4460 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // SignedZeroInfNanPreserve
        4461 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // RoundingModeRTE
        4462 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // RoundingModeRTZ
        4463 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // EarlyAndLateFragmentTestsAMD
        5017 => {}
        // StencilRefReplacingEXT
        5027 => {}
        // CoalescingAMDX
        5069 => {}
        // MaxNodeRecursionAMDX
        5071 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // StaticNumWorkgroupsAMDX
        5072 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // ShaderIndexAMDX
        5073 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // MaxNumWorkgroupsAMDX
        5077 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // StencilRefUnchangedFrontAMD
        5079 => {}
        // StencilRefGreaterFrontAMD
        5080 => {}
        // StencilRefLessFrontAMD
        5081 => {}
        // StencilRefUnchangedBackAMD
        5082 => {}
        // StencilRefGreaterBackAMD
        5083 => {}
        // StencilRefLessBackAMD
        5084 => {}
        // OutputLinesEXT
        5269 => {}
        // OutputPrimitivesEXT
        5270 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // DerivativeGroupQuadsNV
        5289 => {}
        // DerivativeGroupLinearNV
        5290 => {}
        // OutputTrianglesEXT
        5298 => {}
        // PixelInterlockOrderedEXT
        5366 => {}
        // PixelInterlockUnorderedEXT
        5367 => {}
        // SampleInterlockOrderedEXT
        5368 => {}
        // SampleInterlockUnorderedEXT
        5369 => {}
        // ShadingRateInterlockOrderedEXT
        5370 => {}
        // ShadingRateInterlockUnorderedEXT
        5371 => {}
        // SharedLocalMemorySizeINTEL
        5618 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // RoundingModeRTPINTEL
        5620 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // RoundingModeRTNINTEL
        5621 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // FloatingPointModeALTINTEL
        5622 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // FloatingPointModeIEEEINTEL
        5623 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // MaxWorkgroupSizeINTEL
        5893 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // MaxWorkDimINTEL
        5894 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // NoGlobalOffsetINTEL
        5895 => {}
        // NumSIMDWorkitemsINTEL
        5896 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // SchedulerTargetFmaxMhzINTEL
        5903 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // StreamingInterfaceINTEL
        6154 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // RegisterMapInterfaceINTEL
        6160 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // NamedBarrierCountINTEL
        6417 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_StorageClass(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"StorageClass", value)?];
    match value {
        // UniformConstant
        0 => {}
        // Input
        1 => {}
        // Uniform
        2 => {}
        // Output
        3 => {}
        // Workgroup
        4 => {}
        // CrossWorkgroup
        5 => {}
        // Private
        6 => {}
        // Function
        7 => {}
        // Generic
        8 => {}
        // PushConstant
        9 => {}
        // AtomicCounter
        10 => {}
        // Image
        11 => {}
        // StorageBuffer
        12 => {}
        // TileImageEXT
        4172 => {}
        // NodePayloadAMDX
        5068 => {}
        // NodeOutputPayloadAMDX
        5076 => {}
        // CallableDataKHR
        5328 => {}
        // IncomingCallableDataKHR
        5329 => {}
        // RayPayloadKHR
        5338 => {}
        // HitAttributeKHR
        5339 => {}
        // IncomingRayPayloadKHR
        5342 => {}
        // ShaderRecordBufferKHR
        5343 => {}
        // PhysicalStorageBufferEXT
        5349 => {}
        // HitObjectAttributeNV
        5385 => {}
        // TaskPayloadWorkgroupEXT
        5402 => {}
        // CodeSectionINTEL
        5605 => {}
        // DeviceOnlyINTEL
        5936 => {}
        // HostOnlyINTEL
        5937 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_Dim(operands: &mut Operands, id_names: &HashMap<u32, String>) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"Dim", value)?];
    match value {
        // 1D
        0 => {}
        // 2D
        1 => {}
        // 3D
        2 => {}
        // Cube
        3 => {}
        // Rect
        4 => {}
        // Buffer
        5 => {}
        // SubpassData
        6 => {}
        // TileImageDataEXT
        4173 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_SamplerAddressingMode(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"SamplerAddressingMode", value)?];
    match value {
        // None
        0 => {}
        // ClampToEdge
        1 => {}
        // Clamp
        2 => {}
        // Repeat
        3 => {}
        // RepeatMirrored
        4 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_SamplerFilterMode(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"SamplerFilterMode", value)?];
    match value {
        // Nearest
        0 => {}
        // Linear
        1 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_ImageFormat(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"ImageFormat", value)?];
    match value {
        // Unknown
        0 => {}
        // Rgba32f
        1 => {}
        // Rgba16f
        2 => {}
        // R32f
        3 => {}
        // Rgba8
        4 => {}
        // Rgba8Snorm
        5 => {}
        // Rg32f
        6 => {}
        // Rg16f
        7 => {}
        // R11fG11fB10f
        8 => {}
        // R16f
        9 => {}
        // Rgba16
        10 => {}
        // Rgb10A2
        11 => {}
        // Rg16
        12 => {}
        // Rg8
        13 => {}
        // R16
        14 => {}
        // R8
        15 => {}
        // Rgba16Snorm
        16 => {}
        // Rg16Snorm
        17 => {}
        // Rg8Snorm
        18 => {}
        // R16Snorm
        19 => {}
        // R8Snorm
        20 => {}
        // Rgba32i
        21 => {}
        // Rgba16i
        22 => {}
        // Rgba8i
        23 => {}
        // R32i
        24 => {}
        // Rg32i
        25 => {}
        // Rg16i
        26 => {}
        // Rg8i
        27 => {}
        // R16i
        28 => {}
        // R8i
        29 => {}
        // Rgba32ui
        30 => {}
        // Rgba16ui
        31 => {}
        // Rgba8ui
        32 => {}
        // R32ui
        33 => {}
        // Rgb10a2ui
        34 => {}
        // Rg32ui
        35 => {}
        // Rg16ui
        36 => {}
        // Rg8ui
        37 => {}
        // R16ui
        38 => {}
        // R8ui
        39 => {}
        // R64ui
        40 => {}
        // R64i
        41 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_ImageChannelOrder(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"ImageChannelOrder", value)?];
    match value {
        // R
        0 => {}
        // A
        1 => {}
        // RG
        2 => {}
        // RA
        3 => {}
        // RGB
        4 => {}
        // RGBA
        5 => {}
        // BGRA
        6 => {}
        // ARGB
        7 => {}
        // Intensity
        8 => {}
        // Luminance
        9 => {}
        // Rx
        10 => {}
        // RGx
        11 => {}
        // RGBx
        12 => {}
        // Depth
        13 => {}
        // DepthStencil
        14 => {}
        // sRGB
        15 => {}
        // sRGBx
        16 => {}
        // sRGBA
        17 => {}
        // sBGRA
        18 => {}
        // ABGR
        19 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_ImageChannelDataType(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"ImageChannelDataType", value)?];
    match value {
        // SnormInt8
        0 => {}
        // SnormInt16
        1 => {}
        // UnormInt8
        2 => {}
        // UnormInt16
        3 => {}
        // UnormShort565
        4 => {}
        // UnormShort555
        5 => {}
        // UnormInt101010
        6 => {}
        // SignedInt8
        7 => {}
        // SignedInt16
        8 => {}
        // SignedInt32
        9 => {}
        // UnsignedInt8
        10 => {}
        // UnsignedInt16
        11 => {}
        // UnsignedInt32
        12 => {}
        // HalfFloat
        13 => {}
        // Float
        14 => {}
        // UnormInt24
        15 => {}
        // UnormInt101010_2
        16 => {}
        // UnsignedIntRaw10EXT
        19 => {}
        // UnsignedIntRaw12EXT
        20 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_FPRoundingMode(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"FPRoundingMode", value)?];
    match value {
        // RTE
        0 => {}
        // RTZ
        1 => {}
        // RTP
        2 => {}
        // RTN
        3 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_FPDenormMode(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"FPDenormMode", value)?];
    match value {
        // Preserve
        0 => {}
        // FlushToZero
        1 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_QuantizationModes(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"QuantizationModes", value)?];
    match value {
        // TRN
        0 => {}
        // TRN_ZERO
        1 => {}
        // RND
        2 => {}
        // RND_ZERO
        3 => {}
        // RND_INF
        4 => {}
        // RND_MIN_INF
        5 => {}
        // RND_CONV
        6 => {}
        // RND_CONV_ODD
        7 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_FPOperationMode(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"FPOperationMode", value)?];
    match value {
        // IEEE
        0 => {}
        // ALT
        1 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_OverflowModes(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"OverflowModes", value)?];
    match value {
        // WRAP
        0 => {}
        // SAT
        1 => {}
        // SAT_ZERO
        2 => {}
        // SAT_SYM
        3 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_LinkageType(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"LinkageType", value)?];
    match value {
        // Export
        0 => {}
        // Import
        1 => {}
        // LinkOnceODR
        2 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_AccessQualifier(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"AccessQualifier", value)?];
    match value {
        // ReadOnly
        0 => {}
        // WriteOnly
        1 => {}
        // ReadWrite
        2 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_HostAccessQualifier(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"HostAccessQualifier", value)?];
    match value {
        // NoneINTEL
        0 => {}
        // ReadINTEL
        1 => {}
        // WriteINTEL
        2 => {}
        // ReadWriteINTEL
        3 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_FunctionParameterAttribute(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"FunctionParameterAttribute", value)?];
    match value {
        // Zext
        0 => {}
        // Sext
        1 => {}
        // ByVal
        2 => {}
        // Sret
        3 => {}
        // NoAlias
        4 => {}
        // NoCapture
        5 => {}
        // NoWrite
        6 => {}
        // NoReadWrite
        7 => {}
        // RuntimeAlignedINTEL
        5940 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_Decoration(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"Decoration", value)?];
    match value {
        // RelaxedPrecision
        0 => {}
        // SpecId
        1 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // Block
        2 => {}
        // BufferBlock
        3 => {}
        // RowMajor
        4 => {}
        // ColMajor
        5 => {}
        // ArrayStride
        6 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // MatrixStride
        7 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // GLSLShared
        8 => {}
        // GLSLPacked
        9 => {}
        // CPacked
        10 => {}
        // BuiltIn
        11 => {
            // BuiltIn
            out.extend(print_enum_BuiltIn(operands, id_names)?);
        }
        // NoPerspective
        13 => {}
        // Flat
        14 => {}
        // Patch
        15 => {}
        // Centroid
        16 => {}
        // Sample
        17 => {}
        // Invariant
        18 => {}
        // Restrict
        19 => {}
        // Aliased
        20 => {}
        // Volatile
        21 => {}
        // Constant
        22 => {}
        // Coherent
        23 => {}
        // NonWritable
        24 => {}
        // NonReadable
        25 => {}
        // Uniform
        26 => {}
        // UniformId
        27 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
        }
        // SaturatedConversion
        28 => {}
        // Stream
        29 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // Location
        30 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // Component
        31 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // Index
        32 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // Binding
        33 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // DescriptorSet
        34 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // Offset
        35 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // XfbBuffer
        36 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // XfbStride
        37 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // FuncParamAttr
        38 => {
            // FunctionParameterAttribute
            out.extend(print_enum_FunctionParameterAttribute(operands, id_names)?);
        }
        // FPRoundingMode
        39 => {
            // FPRoundingMode
            out.extend(print_enum_FPRoundingMode(operands, id_names)?);
        }
        // FPFastMathMode
        40 => {
            // FPFastMathMode
            out.extend(print_enum_FPFastMathMode(operands, id_names)?);
        }
        // LinkageAttributes
        41 => {
            // LiteralString
            out.push(print_str(operands)?);
            // LinkageType
            out.extend(print_enum_LinkageType(operands, id_names)?);
        }
        // NoContraction
        42 => {}
        // InputAttachmentIndex
        43 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // Alignment
        44 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // MaxByteOffset
        45 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // AlignmentId
        46 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // MaxByteOffsetId
        47 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // NoSignedWrap
        4469 => {}
        // NoUnsignedWrap
        4470 => {}
        // WeightTextureQCOM
        4487 => {}
        // BlockMatchTextureQCOM
        4488 => {}
        // ExplicitInterpAMD
        4999 => {}
        // NodeSharesPayloadLimitsWithAMDX
        5019 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // NodeMaxPayloadsAMDX
        5020 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // TrackFinishWritingAMDX
        5078 => {}
        // PayloadNodeNameAMDX
        5091 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OverrideCoverageNV
        5248 => {}
        // PassthroughNV
        5250 => {}
        // ViewportRelativeNV
        5252 => {}
        // SecondaryViewportRelativeNV
        5256 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // PerPrimitiveEXT
        5271 => {}
        // PerViewNV
        5272 => {}
        // PerTaskNV
        5273 => {}
        // PerVertexNV
        5285 => {}
        // NonUniformEXT
        5300 => {}
        // RestrictPointerEXT
        5355 => {}
        // AliasedPointerEXT
        5356 => {}
        // HitObjectShaderRecordBufferNV
        5386 => {}
        // BindlessSamplerNV
        5398 => {}
        // BindlessImageNV
        5399 => {}
        // BoundSamplerNV
        5400 => {}
        // BoundImageNV
        5401 => {}
        // SIMTCallINTEL
        5599 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // ReferencedIndirectlyINTEL
        5602 => {}
        // ClobberINTEL
        5607 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // SideEffectsINTEL
        5608 => {}
        // VectorComputeVariableINTEL
        5624 => {}
        // FuncParamIOKindINTEL
        5625 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // VectorComputeFunctionINTEL
        5626 => {}
        // StackCallINTEL
        5627 => {}
        // GlobalVariableOffsetINTEL
        5628 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // HlslCounterBufferGOOGLE
        5634 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // HlslSemanticGOOGLE
        5635 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // UserTypeGOOGLE
        5636 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // FunctionRoundingModeINTEL
        5822 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // FPRoundingMode
            out.extend(print_enum_FPRoundingMode(operands, id_names)?);
        }
        // FunctionDenormModeINTEL
        5823 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // FPDenormMode
            out.extend(print_enum_FPDenormMode(operands, id_names)?);
        }
        // RegisterINTEL
        5825 => {}
        // MemoryINTEL
        5826 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // NumbanksINTEL
        5827 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // BankwidthINTEL
        5828 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // MaxPrivateCopiesINTEL
        5829 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // SinglepumpINTEL
        5830 => {}
        // DoublepumpINTEL
        5831 => {}
        // MaxReplicatesINTEL
        5832 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // SimpleDualPortINTEL
        5833 => {}
        // MergeINTEL
        5834 => {
            // LiteralString
            out.push(print_str(operands)?);
            // LiteralString
            out.push(print_str(operands)?);
        }
        // BankBitsINTEL
        5835 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // ForcePow2DepthINTEL
        5836 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // StridesizeINTEL
        5883 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // WordsizeINTEL
        5884 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // TrueDualPortINTEL
        5885 => {}
        // BurstCoalesceINTEL
        5899 => {}
        // CacheSizeINTEL
        5900 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // DontStaticallyCoalesceINTEL
        5901 => {}
        // PrefetchINTEL
        5902 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // StallEnableINTEL
        5905 => {}
        // FuseLoopsInFunctionINTEL
        5907 => {}
        // MathOpDSPModeINTEL
        5909 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // AliasScopeINTEL
        5914 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // NoAliasINTEL
        5915 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // InitiationIntervalINTEL
        5917 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // MaxConcurrencyINTEL
        5918 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // PipelineEnableINTEL
        5919 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // BufferLocationINTEL
        5921 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // IOPipeStorageINTEL
        5944 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // FunctionFloatingPointModeINTEL
        6080 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // FPOperationMode
            out.extend(print_enum_FPOperationMode(operands, id_names)?);
        }
        // SingleElementVectorINTEL
        6085 => {}
        // VectorComputeCallableFunctionINTEL
        6087 => {}
        // MediaBlockIOINTEL
        6140 => {}
        // StallFreeINTEL
        6151 => {}
        // FPMaxErrorDecorationINTEL
        6170 => {
            // LiteralFloat
            out.push(print_f32(operands)?);
        }
        // LatencyControlLabelINTEL
        6172 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // LatencyControlConstraintINTEL
        6173 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // ConduitKernelArgumentINTEL
        6175 => {}
        // RegisterMapKernelArgumentINTEL
        6176 => {}
        // MMHostInterfaceAddressWidthINTEL
        6177 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // MMHostInterfaceDataWidthINTEL
        6178 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // MMHostInterfaceLatencyINTEL
        6179 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // MMHostInterfaceReadWriteModeINTEL
        6180 => {
            // AccessQualifier
            out.extend(print_enum_AccessQualifier(operands, id_names)?);
        }
        // MMHostInterfaceMaxBurstINTEL
        6181 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // MMHostInterfaceWaitRequestINTEL
        6182 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // StableKernelArgumentINTEL
        6183 => {}
        // HostAccessINTEL
        6188 => {
            // HostAccessQualifier
            out.extend(print_enum_HostAccessQualifier(operands, id_names)?);
            // LiteralString
            out.push(print_str(operands)?);
        }
        // InitModeINTEL
        6190 => {
            // InitializationModeQualifier
            out.extend(print_enum_InitializationModeQualifier(operands, id_names)?);
        }
        // ImplementInRegisterMapINTEL
        6191 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // CacheControlLoadINTEL
        6442 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LoadCacheControl
            out.extend(print_enum_LoadCacheControl(operands, id_names)?);
        }
        // CacheControlStoreINTEL
        6443 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
            // StoreCacheControl
            out.extend(print_enum_StoreCacheControl(operands, id_names)?);
        }
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_BuiltIn(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"BuiltIn", value)?];
    match value {
        // Position
        0 => {}
        // PointSize
        1 => {}
        // ClipDistance
        3 => {}
        // CullDistance
        4 => {}
        // VertexId
        5 => {}
        // InstanceId
        6 => {}
        // PrimitiveId
        7 => {}
        // InvocationId
        8 => {}
        // Layer
        9 => {}
        // ViewportIndex
        10 => {}
        // TessLevelOuter
        11 => {}
        // TessLevelInner
        12 => {}
        // TessCoord
        13 => {}
        // PatchVertices
        14 => {}
        // FragCoord
        15 => {}
        // PointCoord
        16 => {}
        // FrontFacing
        17 => {}
        // SampleId
        18 => {}
        // SamplePosition
        19 => {}
        // SampleMask
        20 => {}
        // FragDepth
        22 => {}
        // HelperInvocation
        23 => {}
        // NumWorkgroups
        24 => {}
        // WorkgroupSize
        25 => {}
        // WorkgroupId
        26 => {}
        // LocalInvocationId
        27 => {}
        // GlobalInvocationId
        28 => {}
        // LocalInvocationIndex
        29 => {}
        // WorkDim
        30 => {}
        // GlobalSize
        31 => {}
        // EnqueuedWorkgroupSize
        32 => {}
        // GlobalOffset
        33 => {}
        // GlobalLinearId
        34 => {}
        // SubgroupSize
        36 => {}
        // SubgroupMaxSize
        37 => {}
        // NumSubgroups
        38 => {}
        // NumEnqueuedSubgroups
        39 => {}
        // SubgroupId
        40 => {}
        // SubgroupLocalInvocationId
        41 => {}
        // VertexIndex
        42 => {}
        // InstanceIndex
        43 => {}
        // CoreIDARM
        4160 => {}
        // CoreCountARM
        4161 => {}
        // CoreMaxIDARM
        4162 => {}
        // WarpIDARM
        4163 => {}
        // WarpMaxIDARM
        4164 => {}
        // SubgroupEqMaskKHR
        4416 => {}
        // SubgroupGeMaskKHR
        4417 => {}
        // SubgroupGtMaskKHR
        4418 => {}
        // SubgroupLeMaskKHR
        4419 => {}
        // SubgroupLtMaskKHR
        4420 => {}
        // BaseVertex
        4424 => {}
        // BaseInstance
        4425 => {}
        // DrawIndex
        4426 => {}
        // PrimitiveShadingRateKHR
        4432 => {}
        // DeviceIndex
        4438 => {}
        // ViewIndex
        4440 => {}
        // ShadingRateKHR
        4444 => {}
        // BaryCoordNoPerspAMD
        4992 => {}
        // BaryCoordNoPerspCentroidAMD
        4993 => {}
        // BaryCoordNoPerspSampleAMD
        4994 => {}
        // BaryCoordSmoothAMD
        4995 => {}
        // BaryCoordSmoothCentroidAMD
        4996 => {}
        // BaryCoordSmoothSampleAMD
        4997 => {}
        // BaryCoordPullModelAMD
        4998 => {}
        // FragStencilRefEXT
        5014 => {}
        // CoalescedInputCountAMDX
        5021 => {}
        // ShaderIndexAMDX
        5073 => {}
        // ViewportMaskNV
        5253 => {}
        // SecondaryPositionNV
        5257 => {}
        // SecondaryViewportMaskNV
        5258 => {}
        // PositionPerViewNV
        5261 => {}
        // ViewportMaskPerViewNV
        5262 => {}
        // FullyCoveredEXT
        5264 => {}
        // TaskCountNV
        5274 => {}
        // PrimitiveCountNV
        5275 => {}
        // PrimitiveIndicesNV
        5276 => {}
        // ClipDistancePerViewNV
        5277 => {}
        // CullDistancePerViewNV
        5278 => {}
        // LayerPerViewNV
        5279 => {}
        // MeshViewCountNV
        5280 => {}
        // MeshViewIndicesNV
        5281 => {}
        // BaryCoordNV
        5286 => {}
        // BaryCoordNoPerspNV
        5287 => {}
        // FragmentSizeNV
        5292 => {}
        // InvocationsPerPixelNV
        5293 => {}
        // PrimitivePointIndicesEXT
        5294 => {}
        // PrimitiveLineIndicesEXT
        5295 => {}
        // PrimitiveTriangleIndicesEXT
        5296 => {}
        // CullPrimitiveEXT
        5299 => {}
        // LaunchIdKHR
        5319 => {}
        // LaunchSizeKHR
        5320 => {}
        // WorldRayOriginKHR
        5321 => {}
        // WorldRayDirectionKHR
        5322 => {}
        // ObjectRayOriginKHR
        5323 => {}
        // ObjectRayDirectionKHR
        5324 => {}
        // RayTminKHR
        5325 => {}
        // RayTmaxKHR
        5326 => {}
        // InstanceCustomIndexKHR
        5327 => {}
        // ObjectToWorldKHR
        5330 => {}
        // WorldToObjectKHR
        5331 => {}
        // HitTNV
        5332 => {}
        // HitKindKHR
        5333 => {}
        // CurrentRayTimeNV
        5334 => {}
        // HitTriangleVertexPositionsKHR
        5335 => {}
        // HitMicroTriangleVertexPositionsNV
        5337 => {}
        // HitMicroTriangleVertexBarycentricsNV
        5344 => {}
        // IncomingRayFlagsKHR
        5351 => {}
        // RayGeometryIndexKHR
        5352 => {}
        // WarpsPerSMNV
        5374 => {}
        // SMCountNV
        5375 => {}
        // WarpIDNV
        5376 => {}
        // SMIDNV
        5377 => {}
        // HitKindFrontFacingMicroTriangleNV
        5405 => {}
        // HitKindBackFacingMicroTriangleNV
        5406 => {}
        // CullMaskKHR
        6021 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_Scope(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"Scope", value)?];
    match value {
        // CrossDevice
        0 => {}
        // Device
        1 => {}
        // Workgroup
        2 => {}
        // Subgroup
        3 => {}
        // Invocation
        4 => {}
        // QueueFamilyKHR
        5 => {}
        // ShaderCallKHR
        6 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_GroupOperation(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"GroupOperation", value)?];
    match value {
        // Reduce
        0 => {}
        // InclusiveScan
        1 => {}
        // ExclusiveScan
        2 => {}
        // ClusteredReduce
        3 => {}
        // PartitionedReduceNV
        6 => {}
        // PartitionedInclusiveScanNV
        7 => {}
        // PartitionedExclusiveScanNV
        8 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_KernelEnqueueFlags(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"KernelEnqueueFlags", value)?];
    match value {
        // NoWait
        0 => {}
        // WaitKernel
        1 => {}
        // WaitWorkGroup
        2 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_Capability(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"Capability", value)?];
    match value {
        // Matrix
        0 => {}
        // Shader
        1 => {}
        // Geometry
        2 => {}
        // Tessellation
        3 => {}
        // Addresses
        4 => {}
        // Linkage
        5 => {}
        // Kernel
        6 => {}
        // Vector16
        7 => {}
        // Float16Buffer
        8 => {}
        // Float16
        9 => {}
        // Float64
        10 => {}
        // Int64
        11 => {}
        // Int64Atomics
        12 => {}
        // ImageBasic
        13 => {}
        // ImageReadWrite
        14 => {}
        // ImageMipmap
        15 => {}
        // Pipes
        17 => {}
        // Groups
        18 => {}
        // DeviceEnqueue
        19 => {}
        // LiteralSampler
        20 => {}
        // AtomicStorage
        21 => {}
        // Int16
        22 => {}
        // TessellationPointSize
        23 => {}
        // GeometryPointSize
        24 => {}
        // ImageGatherExtended
        25 => {}
        // StorageImageMultisample
        27 => {}
        // UniformBufferArrayDynamicIndexing
        28 => {}
        // SampledImageArrayDynamicIndexing
        29 => {}
        // StorageBufferArrayDynamicIndexing
        30 => {}
        // StorageImageArrayDynamicIndexing
        31 => {}
        // ClipDistance
        32 => {}
        // CullDistance
        33 => {}
        // ImageCubeArray
        34 => {}
        // SampleRateShading
        35 => {}
        // ImageRect
        36 => {}
        // SampledRect
        37 => {}
        // GenericPointer
        38 => {}
        // Int8
        39 => {}
        // InputAttachment
        40 => {}
        // SparseResidency
        41 => {}
        // MinLod
        42 => {}
        // Sampled1D
        43 => {}
        // Image1D
        44 => {}
        // SampledCubeArray
        45 => {}
        // SampledBuffer
        46 => {}
        // ImageBuffer
        47 => {}
        // ImageMSArray
        48 => {}
        // StorageImageExtendedFormats
        49 => {}
        // ImageQuery
        50 => {}
        // DerivativeControl
        51 => {}
        // InterpolationFunction
        52 => {}
        // TransformFeedback
        53 => {}
        // GeometryStreams
        54 => {}
        // StorageImageReadWithoutFormat
        55 => {}
        // StorageImageWriteWithoutFormat
        56 => {}
        // MultiViewport
        57 => {}
        // SubgroupDispatch
        58 => {}
        // NamedBarrier
        59 => {}
        // PipeStorage
        60 => {}
        // GroupNonUniform
        61 => {}
        // GroupNonUniformVote
        62 => {}
        // GroupNonUniformArithmetic
        63 => {}
        // GroupNonUniformBallot
        64 => {}
        // GroupNonUniformShuffle
        65 => {}
        // GroupNonUniformShuffleRelative
        66 => {}
        // GroupNonUniformClustered
        67 => {}
        // GroupNonUniformQuad
        68 => {}
        // ShaderLayer
        69 => {}
        // ShaderViewportIndex
        70 => {}
        // UniformDecoration
        71 => {}
        // CoreBuiltinsARM
        4165 => {}
        // TileImageColorReadAccessEXT
        4166 => {}
        // TileImageDepthReadAccessEXT
        4167 => {}
        // TileImageStencilReadAccessEXT
        4168 => {}
        // FragmentShadingRateKHR
        4422 => {}
        // SubgroupBallotKHR
        4423 => {}
        // DrawParameters
        4427 => {}
        // WorkgroupMemoryExplicitLayoutKHR
        4428 => {}
        // WorkgroupMemoryExplicitLayout8BitAccessKHR
        4429 => {}
        // WorkgroupMemoryExplicitLayout16BitAccessKHR
        4430 => {}
        // SubgroupVoteKHR
        4431 => {}
        // StorageUniformBufferBlock16
        4433 => {}
        // StorageUniform16
        4434 => {}
        // StoragePushConstant16
        4435 => {}
        // StorageInputOutput16
        4436 => {}
        // DeviceGroup
        4437 => {}
        // MultiView
        4439 => {}
        // VariablePointersStorageBuffer
        4441 => {}
        // VariablePointers
        4442 => {}
        // AtomicStorageOps
        4445 => {}
        // SampleMaskPostDepthCoverage
        4447 => {}
        // StorageBuffer8BitAccess
        4448 => {}
        // UniformAndStorageBuffer8BitAccess
        4449 => {}
        // StoragePushConstant8
        4450 => {}
        // DenormPreserve
        4464 => {}
        // DenormFlushToZero
        4465 => {}
        // SignedZeroInfNanPreserve
        4466 => {}
        // RoundingModeRTE
        4467 => {}
        // RoundingModeRTZ
        4468 => {}
        // RayQueryProvisionalKHR
        4471 => {}
        // RayQueryKHR
        4472 => {}
        // RayTraversalPrimitiveCullingKHR
        4478 => {}
        // RayTracingKHR
        4479 => {}
        // TextureSampleWeightedQCOM
        4484 => {}
        // TextureBoxFilterQCOM
        4485 => {}
        // TextureBlockMatchQCOM
        4486 => {}
        // Float16ImageAMD
        5008 => {}
        // ImageGatherBiasLodAMD
        5009 => {}
        // FragmentMaskAMD
        5010 => {}
        // StencilExportEXT
        5013 => {}
        // ImageReadWriteLodAMD
        5015 => {}
        // Int64ImageEXT
        5016 => {}
        // ShaderClockKHR
        5055 => {}
        // ShaderEnqueueAMDX
        5067 => {}
        // SampleMaskOverrideCoverageNV
        5249 => {}
        // GeometryShaderPassthroughNV
        5251 => {}
        // ShaderViewportIndexLayerNV
        5254 => {}
        // ShaderViewportMaskNV
        5255 => {}
        // ShaderStereoViewNV
        5259 => {}
        // PerViewAttributesNV
        5260 => {}
        // FragmentFullyCoveredEXT
        5265 => {}
        // MeshShadingNV
        5266 => {}
        // ImageFootprintNV
        5282 => {}
        // MeshShadingEXT
        5283 => {}
        // FragmentBarycentricNV
        5284 => {}
        // ComputeDerivativeGroupQuadsNV
        5288 => {}
        // ShadingRateNV
        5291 => {}
        // GroupNonUniformPartitionedNV
        5297 => {}
        // ShaderNonUniformEXT
        5301 => {}
        // RuntimeDescriptorArrayEXT
        5302 => {}
        // InputAttachmentArrayDynamicIndexingEXT
        5303 => {}
        // UniformTexelBufferArrayDynamicIndexingEXT
        5304 => {}
        // StorageTexelBufferArrayDynamicIndexingEXT
        5305 => {}
        // UniformBufferArrayNonUniformIndexingEXT
        5306 => {}
        // SampledImageArrayNonUniformIndexingEXT
        5307 => {}
        // StorageBufferArrayNonUniformIndexingEXT
        5308 => {}
        // StorageImageArrayNonUniformIndexingEXT
        5309 => {}
        // InputAttachmentArrayNonUniformIndexingEXT
        5310 => {}
        // UniformTexelBufferArrayNonUniformIndexingEXT
        5311 => {}
        // StorageTexelBufferArrayNonUniformIndexingEXT
        5312 => {}
        // RayTracingPositionFetchKHR
        5336 => {}
        // RayTracingNV
        5340 => {}
        // RayTracingMotionBlurNV
        5341 => {}
        // VulkanMemoryModelKHR
        5345 => {}
        // VulkanMemoryModelDeviceScopeKHR
        5346 => {}
        // PhysicalStorageBufferAddressesEXT
        5347 => {}
        // ComputeDerivativeGroupLinearNV
        5350 => {}
        // RayTracingProvisionalKHR
        5353 => {}
        // CooperativeMatrixNV
        5357 => {}
        // FragmentShaderSampleInterlockEXT
        5363 => {}
        // FragmentShaderShadingRateInterlockEXT
        5372 => {}
        // ShaderSMBuiltinsNV
        5373 => {}
        // FragmentShaderPixelInterlockEXT
        5378 => {}
        // DemoteToHelperInvocationEXT
        5379 => {}
        // DisplacementMicromapNV
        5380 => {}
        // RayTracingOpacityMicromapEXT
        5381 => {}
        // ShaderInvocationReorderNV
        5383 => {}
        // BindlessTextureNV
        5390 => {}
        // RayQueryPositionFetchKHR
        5391 => {}
        // RayTracingDisplacementMicromapNV
        5409 => {}
        // SubgroupShuffleINTEL
        5568 => {}
        // SubgroupBufferBlockIOINTEL
        5569 => {}
        // SubgroupImageBlockIOINTEL
        5570 => {}
        // SubgroupImageMediaBlockIOINTEL
        5579 => {}
        // RoundToInfinityINTEL
        5582 => {}
        // FloatingPointModeINTEL
        5583 => {}
        // IntegerFunctions2INTEL
        5584 => {}
        // FunctionPointersINTEL
        5603 => {}
        // IndirectReferencesINTEL
        5604 => {}
        // AsmINTEL
        5606 => {}
        // AtomicFloat32MinMaxEXT
        5612 => {}
        // AtomicFloat64MinMaxEXT
        5613 => {}
        // AtomicFloat16MinMaxEXT
        5616 => {}
        // VectorComputeINTEL
        5617 => {}
        // VectorAnyINTEL
        5619 => {}
        // ExpectAssumeKHR
        5629 => {}
        // SubgroupAvcMotionEstimationINTEL
        5696 => {}
        // SubgroupAvcMotionEstimationIntraINTEL
        5697 => {}
        // SubgroupAvcMotionEstimationChromaINTEL
        5698 => {}
        // VariableLengthArrayINTEL
        5817 => {}
        // FunctionFloatControlINTEL
        5821 => {}
        // FPGAMemoryAttributesINTEL
        5824 => {}
        // FPFastMathModeINTEL
        5837 => {}
        // ArbitraryPrecisionIntegersINTEL
        5844 => {}
        // ArbitraryPrecisionFloatingPointINTEL
        5845 => {}
        // UnstructuredLoopControlsINTEL
        5886 => {}
        // FPGALoopControlsINTEL
        5888 => {}
        // KernelAttributesINTEL
        5892 => {}
        // FPGAKernelAttributesINTEL
        5897 => {}
        // FPGAMemoryAccessesINTEL
        5898 => {}
        // FPGAClusterAttributesINTEL
        5904 => {}
        // LoopFuseINTEL
        5906 => {}
        // FPGADSPControlINTEL
        5908 => {}
        // MemoryAccessAliasingINTEL
        5910 => {}
        // FPGAInvocationPipeliningAttributesINTEL
        5916 => {}
        // FPGABufferLocationINTEL
        5920 => {}
        // ArbitraryPrecisionFixedPointINTEL
        5922 => {}
        // USMStorageClassesINTEL
        5935 => {}
        // RuntimeAlignedAttributeINTEL
        5939 => {}
        // IOPipesINTEL
        5943 => {}
        // BlockingPipesINTEL
        5945 => {}
        // FPGARegINTEL
        5948 => {}
        // DotProductInputAllKHR
        6016 => {}
        // DotProductInput4x8BitKHR
        6017 => {}
        // DotProductInput4x8BitPackedKHR
        6018 => {}
        // DotProductKHR
        6019 => {}
        // RayCullMaskKHR
        6020 => {}
        // CooperativeMatrixKHR
        6022 => {}
        // BitInstructions
        6025 => {}
        // GroupNonUniformRotateKHR
        6026 => {}
        // AtomicFloat32AddEXT
        6033 => {}
        // AtomicFloat64AddEXT
        6034 => {}
        // LongCompositesINTEL
        6089 => {}
        // OptNoneINTEL
        6094 => {}
        // AtomicFloat16AddEXT
        6095 => {}
        // DebugInfoModuleINTEL
        6114 => {}
        // BFloat16ConversionINTEL
        6115 => {}
        // SplitBarrierINTEL
        6141 => {}
        // FPGAClusterAttributesV2INTEL
        6150 => {}
        // FPGAKernelAttributesv2INTEL
        6161 => {}
        // FPMaxErrorINTEL
        6169 => {}
        // FPGALatencyControlINTEL
        6171 => {}
        // FPGAArgumentInterfacesINTEL
        6174 => {}
        // GlobalVariableHostAccessINTEL
        6187 => {}
        // GlobalVariableFPGADecorationsINTEL
        6189 => {}
        // GroupUniformArithmeticKHR
        6400 => {}
        // CacheControlsINTEL
        6441 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_RayQueryIntersection(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"RayQueryIntersection", value)?];
    match value {
        // RayQueryCandidateIntersectionKHR
        0 => {}
        // RayQueryCommittedIntersectionKHR
        1 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_RayQueryCommittedIntersectionType(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"RayQueryCommittedIntersectionType", value)?];
    match value {
        // RayQueryCommittedIntersectionNoneKHR
        0 => {}
        // RayQueryCommittedIntersectionTriangleKHR
        1 => {}
        // RayQueryCommittedIntersectionGeneratedKHR
        2 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_RayQueryCandidateIntersectionType(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"RayQueryCandidateIntersectionType", value)?];
    match value {
        // RayQueryCandidateIntersectionTriangleKHR
        0 => {}
        // RayQueryCandidateIntersectionAABBKHR
        1 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_PackedVectorFormat(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"PackedVectorFormat", value)?];
    match value {
        // PackedVectorFormat4x8BitKHR
        0 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_CooperativeMatrixOperands(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"CooperativeMatrixOperands", value)?];
    // NoneKHR
    if value & 0x0000 != 0 {}
    // MatrixASignedComponentsKHR
    if value & 0x0001 != 0 {}
    // MatrixBSignedComponentsKHR
    if value & 0x0002 != 0 {}
    // MatrixCSignedComponentsKHR
    if value & 0x0004 != 0 {}
    // MatrixResultSignedComponentsKHR
    if value & 0x0008 != 0 {}
    // SaturatingAccumulationKHR
    if value & 0x0010 != 0 {}
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_CooperativeMatrixLayout(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"CooperativeMatrixLayout", value)?];
    match value {
        // RowMajorKHR
        0 => {}
        // ColumnMajorKHR
        1 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_CooperativeMatrixUse(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"CooperativeMatrixUse", value)?];
    match value {
        // MatrixAKHR
        0 => {}
        // MatrixBKHR
        1 => {}
        // MatrixAccumulatorKHR
        2 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_InitializationModeQualifier(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"InitializationModeQualifier", value)?];
    match value {
        // InitOnDeviceReprogramINTEL
        0 => {}
        // InitOnDeviceResetINTEL
        1 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_LoadCacheControl(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"LoadCacheControl", value)?];
    match value {
        // UncachedINTEL
        0 => {}
        // CachedINTEL
        1 => {}
        // StreamingINTEL
        2 => {}
        // InvalidateAfterReadINTEL
        3 => {}
        // ConstCachedINTEL
        4 => {}
        _ => {}
    }
    Ok(out)
}

#[allow(non_snake_case)]
#[allow(dead_code)]
#[allow(unused_variables)]
fn print_enum_StoreCacheControl(
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let value = operands.read_u32()?;
    #[allow(unused_mut)]
    let mut out = vec![enum_to_str(&"StoreCacheControl", value)?];
    match value {
        // UncachedINTEL
        0 => {}
        // WriteThroughINTEL
        1 => {}
        // WriteBackINTEL
        2 => {}
        // StreamingINTEL
        3 => {}
        _ => {}
    }
    Ok(out)
}

pub fn print_operand(
    opcode: u32,
    operands: &mut Operands,
    id_names: &HashMap<u32, String>,
) -> Result<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    match opcode {
        // OpNop
        0 => {}
        // OpUndef
        1 => {}
        // OpSourceContinued
        2 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpSource
        3 => {
            // SourceLanguage
            out.extend(print_enum_SourceLanguage(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
            // LiteralString ?
            if operands.len() != 0 {
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
            out.push(print_id(operands, id_names)?);
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpMemberName
        6 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralExtInstInteger
            out.push(print_u32(operands)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpMemoryModel
        14 => {
            // AddressingModel
            out.extend(print_enum_AddressingModel(operands, id_names)?);
            // MemoryModel
            out.extend(print_enum_MemoryModel(operands, id_names)?);
        }
        // OpEntryPoint
        15 => {
            // ExecutionModel
            out.extend(print_enum_ExecutionModel(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralString
            out.push(print_str(operands)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpExecutionMode
        16 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ExecutionMode
            out.extend(print_enum_ExecutionMode(operands, id_names)?);
        }
        // OpCapability
        17 => {
            // Capability
            out.extend(print_enum_Capability(operands, id_names)?);
        }
        // OpTypeVoid
        19 => {}
        // OpTypeBool
        20 => {}
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpTypeMatrix
        24 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpTypeImage
        25 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // Dim
            out.extend(print_enum_Dim(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // ImageFormat
            out.extend(print_enum_ImageFormat(operands, id_names)?);
            // AccessQualifier ?
            if operands.len() != 0 {
                out.extend(print_enum_AccessQualifier(operands, id_names)?);
            }
        }
        // OpTypeSampler
        26 => {}
        // OpTypeSampledImage
        27 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeArray
        28 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeRuntimeArray
        29 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeStruct
        30 => {
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
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
            out.extend(print_enum_StorageClass(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeFunction
        33 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpTypeEvent
        34 => {}
        // OpTypeDeviceEvent
        35 => {}
        // OpTypeReserveId
        36 => {}
        // OpTypeQueue
        37 => {}
        // OpTypePipe
        38 => {
            // AccessQualifier
            out.extend(print_enum_AccessQualifier(operands, id_names)?);
        }
        // OpTypeForwardPointer
        39 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // StorageClass
            out.extend(print_enum_StorageClass(operands, id_names)?);
        }
        // OpConstantTrue
        41 => {}
        // OpConstantFalse
        42 => {}
        // OpConstant
        43 => {
            // LiteralContextDependentNumber
            out.extend(print_list(operands)?);
        }
        // OpConstantComposite
        44 => {
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpConstantSampler
        45 => {
            // SamplerAddressingMode
            out.extend(print_enum_SamplerAddressingMode(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // SamplerFilterMode
            out.extend(print_enum_SamplerFilterMode(operands, id_names)?);
        }
        // OpConstantNull
        46 => {}
        // OpSpecConstantTrue
        48 => {}
        // OpSpecConstantFalse
        49 => {}
        // OpSpecConstant
        50 => {
            // LiteralContextDependentNumber
            out.extend(print_list(operands)?);
        }
        // OpSpecConstantComposite
        51 => {
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
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
            out.extend(print_enum_FunctionControl(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFunctionParameter
        55 => {}
        // OpFunctionEnd
        56 => {}
        // OpFunctionCall
        57 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpVariable
        59 => {
            // StorageClass
            out.extend(print_enum_StorageClass(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpImageTexelPointer
        60 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpLoad
        61 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // MemoryAccess ?
            if operands.len() != 0 {
                out.extend(print_enum_MemoryAccess(operands, id_names)?);
            }
        }
        // OpStore
        62 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // MemoryAccess ?
            if operands.len() != 0 {
                out.extend(print_enum_MemoryAccess(operands, id_names)?);
            }
        }
        // OpCopyMemory
        63 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // MemoryAccess ?
            if operands.len() != 0 {
                out.extend(print_enum_MemoryAccess(operands, id_names)?);
            }
            // MemoryAccess ?
            if operands.len() != 0 {
                out.extend(print_enum_MemoryAccess(operands, id_names)?);
            }
        }
        // OpCopyMemorySized
        64 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // MemoryAccess ?
            if operands.len() != 0 {
                out.extend(print_enum_MemoryAccess(operands, id_names)?);
            }
            // MemoryAccess ?
            if operands.len() != 0 {
                out.extend(print_enum_MemoryAccess(operands, id_names)?);
            }
        }
        // OpAccessChain
        65 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpInBoundsAccessChain
        66 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpPtrAccessChain
        67 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpArrayLength
        68 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpGenericPtrMemSemantics
        69 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpInBoundsPtrAccessChain
        70 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpDecorate
        71 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // Decoration
            out.extend(print_enum_Decoration(operands, id_names)?);
        }
        // OpMemberDecorate
        72 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // Decoration
            out.extend(print_enum_Decoration(operands, id_names)?);
        }
        // OpDecorationGroup
        73 => {}
        // OpGroupDecorate
        74 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupMemberDecorate
        75 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // PairIdRefLiteralInteger *
            while operands.len() != 0 {
                out.extend(print_pair_id_u32_list(operands, id_names)?);
            }
        }
        // OpVectorExtractDynamic
        77 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpVectorInsertDynamic
        78 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpVectorShuffle
        79 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger *
            while operands.len() != 0 {
                out.push(print_u32(operands)?);
            }
        }
        // OpCompositeConstruct
        80 => {
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpCompositeExtract
        81 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger *
            while operands.len() != 0 {
                out.push(print_u32(operands)?);
            }
        }
        // OpCompositeInsert
        82 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger *
            while operands.len() != 0 {
                out.push(print_u32(operands)?);
            }
        }
        // OpCopyObject
        83 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTranspose
        84 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSampledImage
        86 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageSampleImplicitLod
        87 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSampleExplicitLod
        88 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands
            out.extend(print_enum_ImageOperands(operands, id_names)?);
        }
        // OpImageSampleDrefImplicitLod
        89 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSampleDrefExplicitLod
        90 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands
            out.extend(print_enum_ImageOperands(operands, id_names)?);
        }
        // OpImageSampleProjImplicitLod
        91 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSampleProjExplicitLod
        92 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands
            out.extend(print_enum_ImageOperands(operands, id_names)?);
        }
        // OpImageSampleProjDrefImplicitLod
        93 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSampleProjDrefExplicitLod
        94 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands
            out.extend(print_enum_ImageOperands(operands, id_names)?);
        }
        // OpImageFetch
        95 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageGather
        96 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageDrefGather
        97 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageRead
        98 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageWrite
        99 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImage
        100 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageQueryFormat
        101 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageQueryOrder
        102 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageQuerySizeLod
        103 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageQuerySize
        104 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageQueryLod
        105 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageQueryLevels
        106 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageQuerySamples
        107 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertFToU
        109 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertFToS
        110 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertSToF
        111 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertUToF
        112 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUConvert
        113 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSConvert
        114 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFConvert
        115 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpQuantizeToF16
        116 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertPtrToU
        117 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSatConvertSToU
        118 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSatConvertUToS
        119 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertUToPtr
        120 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpPtrCastToGeneric
        121 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGenericCastToPtr
        122 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGenericCastToPtrExplicit
        123 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // StorageClass
            out.extend(print_enum_StorageClass(operands, id_names)?);
        }
        // OpBitcast
        124 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSNegate
        126 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFNegate
        127 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIAdd
        128 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFAdd
        129 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpISub
        130 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFSub
        131 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIMul
        132 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFMul
        133 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUDiv
        134 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSDiv
        135 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFDiv
        136 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUMod
        137 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSRem
        138 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSMod
        139 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFRem
        140 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFMod
        141 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpVectorTimesScalar
        142 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpMatrixTimesScalar
        143 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpVectorTimesMatrix
        144 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpMatrixTimesVector
        145 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpMatrixTimesMatrix
        146 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpOuterProduct
        147 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpDot
        148 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIAddCarry
        149 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpISubBorrow
        150 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUMulExtended
        151 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSMulExtended
        152 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAny
        154 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAll
        155 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIsNan
        156 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIsInf
        157 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIsFinite
        158 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIsNormal
        159 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSignBitSet
        160 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpLessOrGreater
        161 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpOrdered
        162 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUnordered
        163 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpLogicalEqual
        164 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpLogicalNotEqual
        165 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpLogicalOr
        166 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpLogicalAnd
        167 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpLogicalNot
        168 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSelect
        169 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIEqual
        170 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpINotEqual
        171 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUGreaterThan
        172 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSGreaterThan
        173 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUGreaterThanEqual
        174 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSGreaterThanEqual
        175 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpULessThan
        176 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSLessThan
        177 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpULessThanEqual
        178 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSLessThanEqual
        179 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFOrdEqual
        180 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFUnordEqual
        181 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFOrdNotEqual
        182 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFUnordNotEqual
        183 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFOrdLessThan
        184 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFUnordLessThan
        185 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFOrdGreaterThan
        186 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFUnordGreaterThan
        187 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFOrdLessThanEqual
        188 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFUnordLessThanEqual
        189 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFOrdGreaterThanEqual
        190 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFUnordGreaterThanEqual
        191 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpShiftRightLogical
        194 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpShiftRightArithmetic
        195 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpShiftLeftLogical
        196 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpBitwiseOr
        197 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpBitwiseXor
        198 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpBitwiseAnd
        199 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpNot
        200 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpBitFieldInsert
        201 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpBitFieldSExtract
        202 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpBitFieldUExtract
        203 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpBitReverse
        204 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpBitCount
        205 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpDPdx
        207 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpDPdy
        208 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFwidth
        209 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpDPdxFine
        210 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpDPdyFine
        211 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFwidthFine
        212 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpDPdxCoarse
        213 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpDPdyCoarse
        214 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFwidthCoarse
        215 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpEmitVertex
        218 => {}
        // OpEndPrimitive
        219 => {}
        // OpEmitStreamVertex
        220 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpEndStreamPrimitive
        221 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpControlBarrier
        224 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
        }
        // OpMemoryBarrier
        225 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicLoad
        227 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicStore
        228 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicExchange
        229 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicCompareExchange
        230 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicCompareExchangeWeak
        231 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicIIncrement
        232 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicIDecrement
        233 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicIAdd
        234 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicISub
        235 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicSMin
        236 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicUMin
        237 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicSMax
        238 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicUMax
        239 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicAnd
        240 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicOr
        241 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicXor
        242 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpPhi
        245 => {
            // PairIdRefIdRef *
            while operands.len() != 0 {
                out.extend(print_pair_id_id_list(operands, id_names)?);
            }
        }
        // OpLoopMerge
        246 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LoopControl
            out.extend(print_enum_LoopControl(operands, id_names)?);
        }
        // OpSelectionMerge
        247 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // SelectionControl
            out.extend(print_enum_SelectionControl(operands, id_names)?);
        }
        // OpLabel
        248 => {}
        // OpBranch
        249 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpBranchConditional
        250 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger *
            while operands.len() != 0 {
                out.push(print_u32(operands)?);
            }
        }
        // OpSwitch
        251 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // PairLiteralIntegerIdRef *
            while operands.len() != 0 {
                out.extend(print_pair_u32_id_list(operands, id_names)?);
            }
        }
        // OpKill
        252 => {}
        // OpReturn
        253 => {}
        // OpReturnValue
        254 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUnreachable
        255 => {}
        // OpLifetimeStart
        256 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpLifetimeStop
        257 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpGroupAsyncCopy
        259 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupWaitEvents
        260 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupAll
        261 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupAny
        262 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupBroadcast
        263 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupIAdd
        264 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupFAdd
        265 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupFMin
        266 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupUMin
        267 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupSMin
        268 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupFMax
        269 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupUMax
        270 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupSMax
        271 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpReadPipe
        274 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpWritePipe
        275 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpReservedReadPipe
        276 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpReservedWritePipe
        277 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpReserveReadPipePackets
        278 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpReserveWritePipePackets
        279 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpCommitReadPipe
        280 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpCommitWritePipe
        281 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIsValidReserveId
        282 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGetNumPipePackets
        283 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGetMaxPipePackets
        284 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupReserveReadPipePackets
        285 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupReserveWritePipePackets
        286 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupCommitReadPipe
        287 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupCommitWritePipe
        288 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpEnqueueMarker
        291 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpEnqueueKernel
        292 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGetKernelNDrangeSubGroupCount
        293 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGetKernelNDrangeMaxSubGroupSize
        294 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGetKernelWorkGroupSize
        295 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGetKernelPreferredWorkGroupSizeMultiple
        296 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRetainEvent
        297 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpReleaseEvent
        298 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpCreateUserEvent
        299 => {}
        // OpIsValidEvent
        300 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSetUserEventStatus
        301 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpCaptureEventProfilingInfo
        302 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGetDefaultQueue
        303 => {}
        // OpBuildNDRange
        304 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageSparseSampleImplicitLod
        305 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSparseSampleExplicitLod
        306 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands
            out.extend(print_enum_ImageOperands(operands, id_names)?);
        }
        // OpImageSparseSampleDrefImplicitLod
        307 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSparseSampleDrefExplicitLod
        308 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands
            out.extend(print_enum_ImageOperands(operands, id_names)?);
        }
        // OpImageSparseSampleProjImplicitLod
        309 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSparseSampleProjExplicitLod
        310 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands
            out.extend(print_enum_ImageOperands(operands, id_names)?);
        }
        // OpImageSparseSampleProjDrefImplicitLod
        311 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSparseSampleProjDrefExplicitLod
        312 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands
            out.extend(print_enum_ImageOperands(operands, id_names)?);
        }
        // OpImageSparseFetch
        313 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSparseGather
        314 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSparseDrefGather
        315 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpImageSparseTexelsResident
        316 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpNoLine
        317 => {}
        // OpAtomicFlagTestAndSet
        318 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicFlagClear
        319 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
        }
        // OpImageSparseRead
        320 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpSizeOf
        321 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypePipeStorage
        322 => {}
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
            out.push(print_id(operands, id_names)?);
        }
        // OpGetKernelLocalSizeForSubgroupCount
        325 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGetKernelMaxNumSubgroups
        326 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeNamedBarrier
        327 => {}
        // OpNamedBarrierInitialize
        328 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpMemoryNamedBarrier
        329 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
        }
        // OpModuleProcessed
        330 => {
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpExecutionModeId
        331 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ExecutionMode
            out.extend(print_enum_ExecutionMode(operands, id_names)?);
        }
        // OpDecorateId
        332 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // Decoration
            out.extend(print_enum_Decoration(operands, id_names)?);
        }
        // OpGroupNonUniformElect
        333 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformAll
        334 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformAny
        335 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformAllEqual
        336 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformBroadcast
        337 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformBroadcastFirst
        338 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformBallot
        339 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformInverseBallot
        340 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformBallotBitExtract
        341 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformBallotBitCount
        342 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformBallotFindLSB
        343 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformBallotFindMSB
        344 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformShuffle
        345 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformShuffleXor
        346 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformShuffleUp
        347 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformShuffleDown
        348 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformIAdd
        349 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformFAdd
        350 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformIMul
        351 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformFMul
        352 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformSMin
        353 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformUMin
        354 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformFMin
        355 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformSMax
        356 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformUMax
        357 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformFMax
        358 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformBitwiseAnd
        359 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformBitwiseOr
        360 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformBitwiseXor
        361 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformLogicalAnd
        362 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformLogicalOr
        363 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformLogicalXor
        364 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpGroupNonUniformQuadBroadcast
        365 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformQuadSwap
        366 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpCopyLogical
        400 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpPtrEqual
        401 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpPtrNotEqual
        402 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpPtrDiff
        403 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpColorAttachmentReadEXT
        4160 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpDepthAttachmentReadEXT
        4161 => {
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpStencilAttachmentReadEXT
        4162 => {
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpTerminateInvocation
        4416 => {}
        // OpSubgroupBallotKHR
        4421 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupFirstInvocationKHR
        4422 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAllKHR
        4428 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAnyKHR
        4429 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAllEqualKHR
        4430 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformRotateKHR
        4431 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpSubgroupReadInvocationKHR
        4432 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTraceRayKHR
        4445 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpExecuteCallableKHR
        4446 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertUToAccelerationStructureKHR
        4447 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIgnoreIntersectionKHR
        4448 => {}
        // OpTerminateRayKHR
        4449 => {}
        // OpSDotKHR
        4450 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // PackedVectorFormat ?
            if operands.len() != 0 {
                out.extend(print_enum_PackedVectorFormat(operands, id_names)?);
            }
        }
        // OpUDotKHR
        4451 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // PackedVectorFormat ?
            if operands.len() != 0 {
                out.extend(print_enum_PackedVectorFormat(operands, id_names)?);
            }
        }
        // OpSUDotKHR
        4452 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // PackedVectorFormat ?
            if operands.len() != 0 {
                out.extend(print_enum_PackedVectorFormat(operands, id_names)?);
            }
        }
        // OpSDotAccSatKHR
        4453 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // PackedVectorFormat ?
            if operands.len() != 0 {
                out.extend(print_enum_PackedVectorFormat(operands, id_names)?);
            }
        }
        // OpUDotAccSatKHR
        4454 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // PackedVectorFormat ?
            if operands.len() != 0 {
                out.extend(print_enum_PackedVectorFormat(operands, id_names)?);
            }
        }
        // OpSUDotAccSatKHR
        4455 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // PackedVectorFormat ?
            if operands.len() != 0 {
                out.extend(print_enum_PackedVectorFormat(operands, id_names)?);
            }
        }
        // OpTypeCooperativeMatrixKHR
        4456 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpCooperativeMatrixLoadKHR
        4457 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
            // MemoryAccess ?
            if operands.len() != 0 {
                out.extend(print_enum_MemoryAccess(operands, id_names)?);
            }
        }
        // OpCooperativeMatrixStoreKHR
        4458 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
            // MemoryAccess ?
            if operands.len() != 0 {
                out.extend(print_enum_MemoryAccess(operands, id_names)?);
            }
        }
        // OpCooperativeMatrixMulAddKHR
        4459 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // CooperativeMatrixOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_CooperativeMatrixOperands(operands, id_names)?);
            }
        }
        // OpCooperativeMatrixLengthKHR
        4460 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeRayQueryKHR
        4472 => {}
        // OpRayQueryInitializeKHR
        4473 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryTerminateKHR
        4474 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGenerateIntersectionKHR
        4475 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryConfirmIntersectionKHR
        4476 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryProceedKHR
        4477 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionTypeKHR
        4479 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageSampleWeightedQCOM
        4480 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageBoxFilterQCOM
        4481 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageBlockMatchSSDQCOM
        4482 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpImageBlockMatchSADQCOM
        4483 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupIAddNonUniformAMD
        5000 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupFAddNonUniformAMD
        5001 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupFMinNonUniformAMD
        5002 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupUMinNonUniformAMD
        5003 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupSMinNonUniformAMD
        5004 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupFMaxNonUniformAMD
        5005 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupUMaxNonUniformAMD
        5006 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupSMaxNonUniformAMD
        5007 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFragmentMaskFetchAMD
        5011 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFragmentFetchAMD
        5012 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpReadClockKHR
        5056 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
        }
        // OpFinalizeNodePayloadsAMDX
        5075 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFinishWritingNodePayloadAMDX
        5078 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpInitializeNodePayloadsAMDX
        5090 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectRecordHitMotionNV
        5249 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectRecordHitWithIndexMotionNV
        5250 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectRecordMissMotionNV
        5251 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetWorldToObjectNV
        5252 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetObjectToWorldNV
        5253 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetObjectRayDirectionNV
        5254 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetObjectRayOriginNV
        5255 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectTraceRayMotionNV
        5256 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetShaderRecordBufferHandleNV
        5257 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetShaderBindingTableRecordIndexNV
        5258 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectRecordEmptyNV
        5259 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectTraceRayNV
        5260 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectRecordHitNV
        5261 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectRecordHitWithIndexNV
        5262 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectRecordMissNV
        5263 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectExecuteShaderNV
        5264 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetCurrentTimeNV
        5265 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetAttributesNV
        5266 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetHitKindNV
        5267 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetPrimitiveIndexNV
        5268 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetGeometryIndexNV
        5269 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetInstanceIdNV
        5270 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetInstanceCustomIndexNV
        5271 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetWorldRayDirectionNV
        5272 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetWorldRayOriginNV
        5273 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetRayTMaxNV
        5274 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectGetRayTMinNV
        5275 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectIsEmptyNV
        5276 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectIsHitNV
        5277 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpHitObjectIsMissNV
        5278 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpReorderThreadWithHitObjectNV
        5279 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpReorderThreadWithHintNV
        5280 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeHitObjectNV
        5281 => {}
        // OpImageSampleFootprintNV
        5283 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // ImageOperands ?
            if operands.len() != 0 {
                out.extend(print_enum_ImageOperands(operands, id_names)?);
            }
        }
        // OpEmitMeshTasksEXT
        5294 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpSetMeshOutputsEXT
        5295 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupNonUniformPartitionNV
        5296 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpWritePackedPrimitiveIndices4x8NV
        5299 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFetchMicroTriangleVertexPositionNV
        5300 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFetchMicroTriangleVertexBarycentricNV
        5301 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpReportIntersectionKHR
        5334 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIgnoreIntersectionNV
        5335 => {}
        // OpTerminateRayNV
        5336 => {}
        // OpTraceNV
        5337 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTraceMotionNV
        5338 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTraceRayMotionNV
        5339 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionTriangleVertexPositionsKHR
        5340 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeAccelerationStructureKHR
        5341 => {}
        // OpExecuteCallableNV
        5344 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeCooperativeMatrixNV
        5358 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpCooperativeMatrixLoadNV
        5359 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // MemoryAccess ?
            if operands.len() != 0 {
                out.extend(print_enum_MemoryAccess(operands, id_names)?);
            }
        }
        // OpCooperativeMatrixStoreNV
        5360 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // MemoryAccess ?
            if operands.len() != 0 {
                out.extend(print_enum_MemoryAccess(operands, id_names)?);
            }
        }
        // OpCooperativeMatrixMulAddNV
        5361 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpCooperativeMatrixLengthNV
        5362 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpBeginInvocationInterlockEXT
        5364 => {}
        // OpEndInvocationInterlockEXT
        5365 => {}
        // OpDemoteToHelperInvocationEXT
        5380 => {}
        // OpIsHelperInvocationEXT
        5381 => {}
        // OpConvertUToImageNV
        5391 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertUToSamplerNV
        5392 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertImageToUNV
        5393 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertSamplerToUNV
        5394 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertUToSampledImageNV
        5395 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertSampledImageToUNV
        5396 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSamplerImageAddressingModeNV
        5397 => {
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpSubgroupShuffleINTEL
        5571 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupShuffleDownINTEL
        5572 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupShuffleUpINTEL
        5573 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupShuffleXorINTEL
        5574 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupBlockReadINTEL
        5575 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupBlockWriteINTEL
        5576 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupImageBlockReadINTEL
        5577 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupImageBlockWriteINTEL
        5578 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupImageMediaBlockReadINTEL
        5580 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupImageMediaBlockWriteINTEL
        5581 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUCountLeadingZerosINTEL
        5585 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUCountTrailingZerosINTEL
        5586 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAbsISubINTEL
        5587 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAbsUSubINTEL
        5588 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIAddSatINTEL
        5589 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUAddSatINTEL
        5590 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIAverageINTEL
        5591 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUAverageINTEL
        5592 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIAverageRoundedINTEL
        5593 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUAverageRoundedINTEL
        5594 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpISubSatINTEL
        5595 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUSubSatINTEL
        5596 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpIMul32x16INTEL
        5597 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpUMul32x16INTEL
        5598 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConstantFunctionPointerINTEL
        5600 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFunctionPointerCallINTEL
        5601 => {
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralString
            out.push(print_str(operands)?);
            // LiteralString
            out.push(print_str(operands)?);
        }
        // OpAsmCallINTEL
        5611 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpAtomicFMinEXT
        5614 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicFMaxEXT
        5615 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAssumeTrueKHR
        5630 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpExpectKHR
        5631 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpDecorateStringGOOGLE
        5632 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // Decoration
            out.extend(print_enum_Decoration(operands, id_names)?);
        }
        // OpMemberDecorateStringGOOGLE
        5633 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // Decoration
            out.extend(print_enum_Decoration(operands, id_names)?);
        }
        // OpVmeImageINTEL
        5699 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeVmeImageINTEL
        5700 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeAvcImePayloadINTEL
        5701 => {}
        // OpTypeAvcRefPayloadINTEL
        5702 => {}
        // OpTypeAvcSicPayloadINTEL
        5703 => {}
        // OpTypeAvcMcePayloadINTEL
        5704 => {}
        // OpTypeAvcMceResultINTEL
        5705 => {}
        // OpTypeAvcImeResultINTEL
        5706 => {}
        // OpTypeAvcImeResultSingleReferenceStreamoutINTEL
        5707 => {}
        // OpTypeAvcImeResultDualReferenceStreamoutINTEL
        5708 => {}
        // OpTypeAvcImeSingleReferenceStreaminINTEL
        5709 => {}
        // OpTypeAvcImeDualReferenceStreaminINTEL
        5710 => {}
        // OpTypeAvcRefResultINTEL
        5711 => {}
        // OpTypeAvcSicResultINTEL
        5712 => {}
        // OpSubgroupAvcMceGetDefaultInterBaseMultiReferencePenaltyINTEL
        5713 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceSetInterBaseMultiReferencePenaltyINTEL
        5714 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetDefaultInterShapePenaltyINTEL
        5715 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceSetInterShapePenaltyINTEL
        5716 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetDefaultInterDirectionPenaltyINTEL
        5717 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceSetInterDirectionPenaltyINTEL
        5718 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetDefaultIntraLumaShapePenaltyINTEL
        5719 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetDefaultInterMotionVectorCostTableINTEL
        5720 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetDefaultHighPenaltyCostTableINTEL
        5721 => {}
        // OpSubgroupAvcMceGetDefaultMediumPenaltyCostTableINTEL
        5722 => {}
        // OpSubgroupAvcMceGetDefaultLowPenaltyCostTableINTEL
        5723 => {}
        // OpSubgroupAvcMceSetMotionVectorCostFunctionINTEL
        5724 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetDefaultIntraLumaModePenaltyINTEL
        5725 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetDefaultNonDcLumaIntraPenaltyINTEL
        5726 => {}
        // OpSubgroupAvcMceGetDefaultIntraChromaModeBasePenaltyINTEL
        5727 => {}
        // OpSubgroupAvcMceSetAcOnlyHaarINTEL
        5728 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceSetSourceInterlacedFieldPolarityINTEL
        5729 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceSetSingleReferenceInterlacedFieldPolarityINTEL
        5730 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceSetDualReferenceInterlacedFieldPolaritiesINTEL
        5731 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceConvertToImePayloadINTEL
        5732 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceConvertToImeResultINTEL
        5733 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceConvertToRefPayloadINTEL
        5734 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceConvertToRefResultINTEL
        5735 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceConvertToSicPayloadINTEL
        5736 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceConvertToSicResultINTEL
        5737 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetMotionVectorsINTEL
        5738 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetInterDistortionsINTEL
        5739 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetBestInterDistortionsINTEL
        5740 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetInterMajorShapeINTEL
        5741 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetInterMinorShapeINTEL
        5742 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetInterDirectionsINTEL
        5743 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetInterMotionVectorCountINTEL
        5744 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetInterReferenceIdsINTEL
        5745 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcMceGetInterReferenceInterlacedFieldPolaritiesINTEL
        5746 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeInitializeINTEL
        5747 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeSetSingleReferenceINTEL
        5748 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeSetDualReferenceINTEL
        5749 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeRefWindowSizeINTEL
        5750 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeAdjustRefOffsetINTEL
        5751 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeConvertToMcePayloadINTEL
        5752 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeSetMaxMotionVectorCountINTEL
        5753 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeSetUnidirectionalMixDisableINTEL
        5754 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeSetEarlySearchTerminationThresholdINTEL
        5755 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeSetWeightedSadINTEL
        5756 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeEvaluateWithSingleReferenceINTEL
        5757 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeEvaluateWithDualReferenceINTEL
        5758 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminINTEL
        5759 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeEvaluateWithDualReferenceStreaminINTEL
        5760 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeEvaluateWithSingleReferenceStreamoutINTEL
        5761 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeEvaluateWithDualReferenceStreamoutINTEL
        5762 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeEvaluateWithSingleReferenceStreaminoutINTEL
        5763 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeEvaluateWithDualReferenceStreaminoutINTEL
        5764 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeConvertToMceResultINTEL
        5765 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetSingleReferenceStreaminINTEL
        5766 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetDualReferenceStreaminINTEL
        5767 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeStripSingleReferenceStreamoutINTEL
        5768 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeStripDualReferenceStreamoutINTEL
        5769 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeMotionVectorsINTEL
        5770 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeDistortionsINTEL
        5771 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetStreamoutSingleReferenceMajorShapeReferenceIdsINTEL
        5772 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeMotionVectorsINTEL
        5773 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeDistortionsINTEL
        5774 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetStreamoutDualReferenceMajorShapeReferenceIdsINTEL
        5775 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetBorderReachedINTEL
        5776 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetTruncatedSearchIndicationINTEL
        5777 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetUnidirectionalEarlySearchTerminationINTEL
        5778 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetWeightingPatternMinimumMotionVectorINTEL
        5779 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcImeGetWeightingPatternMinimumDistortionINTEL
        5780 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcFmeInitializeINTEL
        5781 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcBmeInitializeINTEL
        5782 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcRefConvertToMcePayloadINTEL
        5783 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcRefSetBidirectionalMixDisableINTEL
        5784 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcRefSetBilinearFilterEnableINTEL
        5785 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcRefEvaluateWithSingleReferenceINTEL
        5786 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcRefEvaluateWithDualReferenceINTEL
        5787 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcRefEvaluateWithMultiReferenceINTEL
        5788 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcRefEvaluateWithMultiReferenceInterlacedINTEL
        5789 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcRefConvertToMceResultINTEL
        5790 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicInitializeINTEL
        5791 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicConfigureSkcINTEL
        5792 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicConfigureIpeLumaINTEL
        5793 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicConfigureIpeLumaChromaINTEL
        5794 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicGetMotionVectorMaskINTEL
        5795 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicConvertToMcePayloadINTEL
        5796 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicSetIntraLumaShapePenaltyINTEL
        5797 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicSetIntraLumaModeCostFunctionINTEL
        5798 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicSetIntraChromaModeCostFunctionINTEL
        5799 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicSetBilinearFilterEnableINTEL
        5800 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicSetSkcForwardTransformEnableINTEL
        5801 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicSetBlockBasedRawSkipSadINTEL
        5802 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicEvaluateIpeINTEL
        5803 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicEvaluateWithSingleReferenceINTEL
        5804 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicEvaluateWithDualReferenceINTEL
        5805 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicEvaluateWithMultiReferenceINTEL
        5806 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicEvaluateWithMultiReferenceInterlacedINTEL
        5807 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicConvertToMceResultINTEL
        5808 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicGetIpeLumaShapeINTEL
        5809 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicGetBestIpeLumaDistortionINTEL
        5810 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicGetBestIpeChromaDistortionINTEL
        5811 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicGetPackedIpeLumaModesINTEL
        5812 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicGetIpeChromaModeINTEL
        5813 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicGetPackedSkcLumaCountThresholdINTEL
        5814 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicGetPackedSkcLumaSumThresholdINTEL
        5815 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSubgroupAvcSicGetInterRawSadsINTEL
        5816 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpVariableLengthArrayINTEL
        5818 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpSaveMemoryINTEL
        5819 => {}
        // OpRestoreMemoryINTEL
        5820 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpArbitraryFloatSinCosPiINTEL
        5840 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatGEINTEL
        5851 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatLTINTEL
        5852 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatLEINTEL
        5853 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatEQINTEL
        5854 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
        }
        // OpArbitraryFloatRecipINTEL
        5855 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // LiteralInteger
            out.push(print_u32(operands)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            while operands.len() != 0 {
                out.push(print_u32(operands)?);
            }
        }
        // OpAliasDomainDeclINTEL
        5911 => {
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpAliasScopeDeclINTEL
        5912 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef ?
            if operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpAliasScopeListDeclINTEL
        5913 => {
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpFixedSqrtINTEL
        5923 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
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
            out.push(print_id(operands, id_names)?);
        }
        // OpCrossWorkgroupCastToPtrINTEL
        5938 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpReadPipeBlockingINTEL
        5946 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpWritePipeBlockingINTEL
        5947 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpFPGARegINTEL
        5949 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetRayTMinKHR
        6016 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetRayFlagsKHR
        6017 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionTKHR
        6018 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionInstanceCustomIndexKHR
        6019 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionInstanceIdKHR
        6020 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionInstanceShaderBindingTableRecordOffsetKHR
        6021 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionGeometryIndexKHR
        6022 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionPrimitiveIndexKHR
        6023 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionBarycentricsKHR
        6024 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionFrontFaceKHR
        6025 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionCandidateAABBOpaqueKHR
        6026 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionObjectRayDirectionKHR
        6027 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionObjectRayOriginKHR
        6028 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetWorldRayDirectionKHR
        6029 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetWorldRayOriginKHR
        6030 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionObjectToWorldKHR
        6031 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpRayQueryGetIntersectionWorldToObjectKHR
        6032 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpAtomicFAddEXT
        6035 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpTypeBufferSurfaceINTEL
        6086 => {
            // AccessQualifier
            out.extend(print_enum_AccessQualifier(operands, id_names)?);
        }
        // OpTypeStructContinuedINTEL
        6090 => {
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpConstantCompositeContinuedINTEL
        6091 => {
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpSpecConstantCompositeContinuedINTEL
        6092 => {
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpCompositeConstructContinuedINTEL
        6096 => {
            // IdRef *
            while operands.len() != 0 {
                out.push(print_id(operands, id_names)?);
            }
        }
        // OpConvertFToBF16INTEL
        6116 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpConvertBF16ToFINTEL
        6117 => {
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpControlBarrierArriveINTEL
        6142 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
        }
        // OpControlBarrierWaitINTEL
        6143 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdScope
            out.push(print_id(operands, id_names)?);
            // IdMemorySemantics
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupIMulKHR
        6401 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupFMulKHR
        6402 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupBitwiseAndKHR
        6403 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupBitwiseOrKHR
        6404 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupBitwiseXorKHR
        6405 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupLogicalAndKHR
        6406 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupLogicalOrKHR
        6407 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        // OpGroupLogicalXorKHR
        6408 => {
            // IdScope
            out.push(print_id(operands, id_names)?);
            // GroupOperation
            out.extend(print_enum_GroupOperation(operands, id_names)?);
            // IdRef
            out.push(print_id(operands, id_names)?);
        }
        _ => bail!("unsupported opcode {}", opcode),
    };
    while operands.len() != 0 {
        out.push(format!("!{}", operands.read_u32()?));
    }
    Ok(out)
}
