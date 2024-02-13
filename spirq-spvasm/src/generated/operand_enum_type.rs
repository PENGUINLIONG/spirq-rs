use anyhow::{bail, Result};
use num_traits::FromPrimitive;
use spirq_core::spirv::Op;

fn unknown_operand_index(opcode: u32, i: usize) -> Result<&'static str> {
    let opname = Op::from_u32(opcode).map(|op| format!("{:?}", op)).unwrap_or("<unknown>".to_owned());
    bail!("Unknown op {} ({}) operand index: {}", opname, opcode, i)
}

pub fn operand_enum_type(opcode: u32, i: usize) -> Result<&'static str> {
    let out: &'static str = match opcode {
        3 => match i {
            0 => "SourceLanguage",
            _ => return unknown_operand_index(opcode, i),
        },
        14 => match i {
            0 => "AddressingModel",
            1 => "MemoryModel",
            _ => return unknown_operand_index(opcode, i),
        },
        15 => match i {
            0 => "ExecutionModel",
            _ => return unknown_operand_index(opcode, i),
        },
        16 => match i {
            1 => "ExecutionMode",
            _ => return unknown_operand_index(opcode, i),
        },
        17 => match i {
            0 => "Capability",
            _ => return unknown_operand_index(opcode, i),
        },
        25 => match i {
            1 => "Dim",
            6 => "ImageFormat",
            7 => "AccessQualifier",
            _ => return unknown_operand_index(opcode, i),
        },
        32 => match i {
            0 => "StorageClass",
            _ => return unknown_operand_index(opcode, i),
        },
        38 => match i {
            0 => "AccessQualifier",
            _ => return unknown_operand_index(opcode, i),
        },
        39 => match i {
            1 => "StorageClass",
            _ => return unknown_operand_index(opcode, i),
        },
        45 => match i {
            0 => "SamplerAddressingMode",
            2 => "SamplerFilterMode",
            _ => return unknown_operand_index(opcode, i),
        },
        54 => match i {
            0 => "FunctionControl",
            _ => return unknown_operand_index(opcode, i),
        },
        59 => match i {
            0 => "StorageClass",
            _ => return unknown_operand_index(opcode, i),
        },
        61 => match i {
            1 => "MemoryAccess",
            _ => return unknown_operand_index(opcode, i),
        },
        62 => match i {
            2 => "MemoryAccess",
            _ => return unknown_operand_index(opcode, i),
        },
        63 => match i {
            2 => "MemoryAccess",
            3 => "MemoryAccess",
            _ => return unknown_operand_index(opcode, i),
        },
        64 => match i {
            3 => "MemoryAccess",
            4 => "MemoryAccess",
            _ => return unknown_operand_index(opcode, i),
        },
        71 => match i {
            1 => "Decoration",
            _ => return unknown_operand_index(opcode, i),
        },
        72 => match i {
            2 => "Decoration",
            _ => return unknown_operand_index(opcode, i),
        },
        87 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        88 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        89 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        90 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        91 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        92 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        93 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        94 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        95 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        96 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        97 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        98 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        99 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        123 => match i {
            1 => "StorageClass",
            _ => return unknown_operand_index(opcode, i),
        },
        246 => match i {
            2 => "LoopControl",
            _ => return unknown_operand_index(opcode, i),
        },
        247 => match i {
            1 => "SelectionControl",
            _ => return unknown_operand_index(opcode, i),
        },
        264 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        265 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        266 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        267 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        268 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        269 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        270 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        271 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        305 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        306 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        307 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        308 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        309 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        310 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        311 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        312 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        313 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        314 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        315 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        320 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        331 => match i {
            1 => "ExecutionMode",
            _ => return unknown_operand_index(opcode, i),
        },
        332 => match i {
            1 => "Decoration",
            _ => return unknown_operand_index(opcode, i),
        },
        342 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        349 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        350 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        351 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        352 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        353 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        354 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        355 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        356 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        357 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        358 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        359 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        360 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        361 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        362 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        363 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        364 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        4450 => match i {
            2 => "PackedVectorFormat",
            _ => return unknown_operand_index(opcode, i),
        },
        4451 => match i {
            2 => "PackedVectorFormat",
            _ => return unknown_operand_index(opcode, i),
        },
        4452 => match i {
            2 => "PackedVectorFormat",
            _ => return unknown_operand_index(opcode, i),
        },
        4453 => match i {
            3 => "PackedVectorFormat",
            _ => return unknown_operand_index(opcode, i),
        },
        4454 => match i {
            3 => "PackedVectorFormat",
            _ => return unknown_operand_index(opcode, i),
        },
        4455 => match i {
            3 => "PackedVectorFormat",
            _ => return unknown_operand_index(opcode, i),
        },
        4457 => match i {
            3 => "MemoryAccess",
            _ => return unknown_operand_index(opcode, i),
        },
        4458 => match i {
            4 => "MemoryAccess",
            _ => return unknown_operand_index(opcode, i),
        },
        4459 => match i {
            3 => "CooperativeMatrixOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        5000 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        5001 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        5002 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        5003 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        5004 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        5005 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        5006 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        5007 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        5283 => match i {
            4 => "ImageOperands",
            _ => return unknown_operand_index(opcode, i),
        },
        5359 => match i {
            3 => "MemoryAccess",
            _ => return unknown_operand_index(opcode, i),
        },
        5360 => match i {
            4 => "MemoryAccess",
            _ => return unknown_operand_index(opcode, i),
        },
        5632 => match i {
            1 => "Decoration",
            _ => return unknown_operand_index(opcode, i),
        },
        5633 => match i {
            2 => "Decoration",
            _ => return unknown_operand_index(opcode, i),
        },
        6086 => match i {
            0 => "AccessQualifier",
            _ => return unknown_operand_index(opcode, i),
        },
        6401 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        6402 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        6403 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        6404 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        6405 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        6406 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        6407 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        6408 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(opcode, i),
        },
        _ => bail!("{}-th operand of opcode {} is not a enum", i, opcode),
    };
    Ok(out)
}
