use anyhow::{bail, Result};
use num_traits::FromPrimitive;
use spirq_core::spirv::Op;

fn unknown_decorate_parameter_index(decoration: u32, i: usize) -> Result<&'static str> {
    let opname = Op::from_u32(decoration).map(|op| format!("{:?}", op)).unwrap_or("<unknown>".to_owned());
    bail!("Unknown op {} ({}) parameter index: {}", opname, decoration, i)
}

pub fn decorate_parameter_enum_type(decoration: u32, i: usize) -> Result<&'static str> {
    let out: &'static str = match decoration {
        11 => match i {
            0 => "BuiltIn",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        38 => match i {
            0 => "FunctionParameterAttribute",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        39 => match i {
            0 => "FPRoundingMode",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        40 => match i {
            0 => "FPFastMathMode",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        41 => match i {
            1 => "LinkageType",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        5822 => match i {
            1 => "FPRoundingMode",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        5823 => match i {
            1 => "FPDenormMode",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        6080 => match i {
            1 => "FPOperationMode",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        6180 => match i {
            0 => "AccessQualifier",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        6188 => match i {
            0 => "HostAccessQualifier",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        6190 => match i {
            0 => "InitializationModeQualifier",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        6442 => match i {
            1 => "LoadCacheControl",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        6443 => match i {
            1 => "StoreCacheControl",
            _ => return unknown_decorate_parameter_index(decoration, i),
        },
        _ => bail!("{}-th parameter of decoration {} is not a enum", i, decoration),
    };
    Ok(out)
}
