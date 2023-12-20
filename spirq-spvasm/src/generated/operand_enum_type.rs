use anyhow::{bail, Result};


fn unknown_operand_index(i: usize) -> Result<&'static str> {
    bail!("Unknown operand index: {}", i)
}

pub fn operand_enum_type(opcode: u32, i: usize) -> Result<&'static str> {
    let out: &'static str = match opcode {
        3 => match i {
            0 => "SourceLanguage",
            _ => return unknown_operand_index(i),
        }
        14 => match i {
            0 => "AddressingModel",
            1 => "MemoryModel",
            _ => return unknown_operand_index(i),
        }
        15 => match i {
            0 => "ExecutionModel",
            _ => return unknown_operand_index(i),
        }
        16 => match i {
            1 => "ExecutionMode",
            _ => return unknown_operand_index(i),
        }
        17 => match i {
            0 => "Capability",
            _ => return unknown_operand_index(i),
        }
        25 => match i {
            1 => "Dim",
            6 => "ImageFormat",
            7 => "AccessQualifier",
            _ => return unknown_operand_index(i),
        }
        32 => match i {
            0 => "StorageClass",
            _ => return unknown_operand_index(i),
        }
        38 => match i {
            0 => "AccessQualifier",
            _ => return unknown_operand_index(i),
        }
        39 => match i {
            1 => "StorageClass",
            _ => return unknown_operand_index(i),
        }
        45 => match i {
            0 => "SamplerAddressingMode",
            2 => "SamplerFilterMode",
            _ => return unknown_operand_index(i),
        }
        54 => match i {
            0 => "FunctionControl",
            _ => return unknown_operand_index(i),
        }
        59 => match i {
            0 => "StorageClass",
            _ => return unknown_operand_index(i),
        }
        61 => match i {
            1 => "MemoryAccess",
            _ => return unknown_operand_index(i),
        }
        62 => match i {
            2 => "MemoryAccess",
            _ => return unknown_operand_index(i),
        }
        63 => match i {
            2 => "MemoryAccess",
            3 => "MemoryAccess",
            _ => return unknown_operand_index(i),
        }
        64 => match i {
            3 => "MemoryAccess",
            4 => "MemoryAccess",
            _ => return unknown_operand_index(i),
        }
        71 => match i {
            1 => "Decoration",
            _ => return unknown_operand_index(i),
        }
        72 => match i {
            2 => "Decoration",
            _ => return unknown_operand_index(i),
        }
        87 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        88 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        89 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        90 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        91 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        92 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        93 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        94 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        95 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        96 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        97 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        98 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        99 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        123 => match i {
            1 => "StorageClass",
            _ => return unknown_operand_index(i),
        }
        246 => match i {
            2 => "LoopControl",
            _ => return unknown_operand_index(i),
        }
        247 => match i {
            1 => "SelectionControl",
            _ => return unknown_operand_index(i),
        }
        264 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        265 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        266 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        267 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        268 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        269 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        270 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        271 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        305 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        306 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        307 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        308 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        309 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        310 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        311 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        312 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        313 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        314 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        315 => match i {
            3 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        320 => match i {
            2 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        331 => match i {
            1 => "ExecutionMode",
            _ => return unknown_operand_index(i),
        }
        332 => match i {
            1 => "Decoration",
            _ => return unknown_operand_index(i),
        }
        342 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        349 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        350 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        351 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        352 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        353 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        354 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        355 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        356 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        357 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        358 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        359 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        360 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        361 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        362 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        363 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        364 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        4450 => match i {
            2 => "PackedVectorFormat",
            _ => return unknown_operand_index(i),
        }
        4451 => match i {
            2 => "PackedVectorFormat",
            _ => return unknown_operand_index(i),
        }
        4452 => match i {
            2 => "PackedVectorFormat",
            _ => return unknown_operand_index(i),
        }
        4453 => match i {
            3 => "PackedVectorFormat",
            _ => return unknown_operand_index(i),
        }
        4454 => match i {
            3 => "PackedVectorFormat",
            _ => return unknown_operand_index(i),
        }
        4455 => match i {
            3 => "PackedVectorFormat",
            _ => return unknown_operand_index(i),
        }
        4457 => match i {
            3 => "MemoryAccess",
            _ => return unknown_operand_index(i),
        }
        4458 => match i {
            4 => "MemoryAccess",
            _ => return unknown_operand_index(i),
        }
        4459 => match i {
            3 => "CooperativeMatrixOperands",
            _ => return unknown_operand_index(i),
        }
        5000 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        5001 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        5002 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        5003 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        5004 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        5005 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        5006 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        5007 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        5283 => match i {
            4 => "ImageOperands",
            _ => return unknown_operand_index(i),
        }
        5359 => match i {
            3 => "MemoryAccess",
            _ => return unknown_operand_index(i),
        }
        5360 => match i {
            4 => "MemoryAccess",
            _ => return unknown_operand_index(i),
        }
        5632 => match i {
            1 => "Decoration",
            _ => return unknown_operand_index(i),
        }
        5633 => match i {
            2 => "Decoration",
            _ => return unknown_operand_index(i),
        }
        6086 => match i {
            0 => "AccessQualifier",
            _ => return unknown_operand_index(i),
        }
        6401 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        6402 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        6403 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        6404 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        6405 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        6406 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        6407 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        6408 => match i {
            1 => "GroupOperation",
            _ => return unknown_operand_index(i),
        }
        _ => bail!("{}-th operand of opcode {} is not a enum", i, opcode),
    };
    Ok(out)
}
