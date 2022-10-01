use std::ffi::CString;
use std::os::raw::{c_char, c_void};
use std::convert::TryFrom;

use spirq::ConstantValue;

pub type Bool = u32;
pub type SpecId = u32;

pub type Reflection = *mut c_void;
pub type EntryPoint = *mut c_void;

#[repr(i32)]
pub enum Error {
    Success = 0,
    ArgumentNull = -1,
    ArgumentOutOfRange = -2,
    InvalidArgument = -3,
    CorruptedSpirv = -4,
    UnsupportedSpirv = -5,
    InvalidSpecialization = -6,
}

#[repr(C)]
pub struct Specialization {
    pub spec_id: SpecId,
    pub value_size: usize,
    pub value: *const c_void,
}

#[repr(C)]
pub struct ReflectConfig {
    pub spirv_size: usize,
    pub spirv: *const u32,
    pub reference_all_resources: Bool,
    pub combine_image_samplers: Bool,
    pub generate_unique_names: Bool,
    pub specialization_count: u32,
    pub specializations: *const Specialization,
}

#[repr(u32)]
pub enum AccessType {
    Unspecified = 0,
    ReadOnly = 1,
    WriteOnly = 2,
    ReadWrite = 3,
}

#[repr(u32)]
pub enum PrimitiveType {
    I8 = 1,
    I16 = 2,
    I32 = 3,
    I64 = 4,
    U8 = 5,
    U16 = 6,
    U32 = 7,
    U64 = 8,
    F16 = 9,
    F32 = 10,
    F64 = 11,
}

#[repr(u32)]
enum MatrixAxisOrder {
    RowMajor = 1,
    ColumnMajor = 2,
}

#[repr(C)]
enum Type {
    Scalar {
        primitive_type: PrimitiveType,
    },
    Vector {
        primitive_type: PrimitiveType,
        component_count: u32,
    },
    Matrix {
        primitive_type: PrimitiveType,
        row_count: u32,
        column_count: u32,
        axis_order: MatrixAxisOrder,
    },
    Array {
        element_type: *const Type,
    },
    Struct {
        member_count: usize,
        member_types: *const Type,
    },
}

#[repr(C)]
pub enum Variable {
    Input {
        location: u32,
        component: u32,
        type_index: usize,
    },
    Output {
        location: u32,
        component: u32,
        type_index: usize,
    },
    UniformBuffer {
        set: u32,
        binding: u32,
        type_index: usize,
        count: u32,
    },
    StorageBuffer {
        set: u32,
        binding: u32,
        access: AccessType,
        type_index: usize,
        count: u32,
    },
    PushConstant {
        type_index: usize,
    },
    SpecConstant {
        spec_id: SpecId,
        type_index: usize,
    }
}

#[repr(C)]
pub struct EntryPointProfile {
    pub name: [c_char; 256],
    pub variable_count: u32,
    pub variables: *mut Variable,
}

struct ReflectionData {
    entry_points: Vec<spirq::EntryPoint>,
}

#[no_mangle]
pub unsafe extern "C" fn create_reflection(config: &ReflectConfig, reflection: *mut Reflection) -> Error {
    let mut cfg = spirq::ReflectConfig::new();
    cfg.spv(std::slice::from_raw_parts(config.spirv, config.spirv_size))
        .ref_all_rscs(config.reference_all_resources != 0)
        .combine_img_samplers(config.combine_image_samplers != 0)
        .gen_unique_names(config.generate_unique_names != 0);

    let specs = std::slice::from_raw_parts(config.specializations, config.specialization_count as usize);
    for spec in specs {
        let data = std::slice::from_raw_parts(spec.value as *const u8, spec.value_size);
        if let Ok(spec_value) = ConstantValue::try_from(data) {
            cfg.specialize(spec.spec_id, spec_value);
        } else {
            return Error::InvalidSpecialization;
        }
    }

    match cfg.reflect() {
        Ok(entry_points) => {
            let data = Box::new(ReflectionData { entry_points });
            *reflection = Box::leak(data) as *mut ReflectionData as *mut c_void;
            Error::Success
        },
        Err(spirq::Error::CorruptedSpirv(_)) => Error::CorruptedSpirv,
        Err(spirq::Error::UnsupportedSpirv(_)) => Error::UnsupportedSpirv,
    }
}
#[no_mangle]
pub unsafe extern "C" fn destroy_reflection(reflection: *mut Reflection) {
    Box::from_raw(reflection);
}

#[no_mangle]
pub unsafe extern "C" fn enumerate_entry_points(reflection: *const Reflection, entry_point_count: *mut u32, entry_points: *mut EntryPoint) -> Error {
    let data = &*(reflection as *const ReflectionData);

    if reflection == std::ptr::null_mut() {
        return Error::ArgumentNull;
    }
    if entry_point_count == std::ptr::null_mut() {
        return Error::ArgumentNull;
    }

    if entry_points == std::ptr::null_mut() {
        *entry_point_count = data.entry_points.len() as u32;
    } else {
        let n = (*entry_point_count as usize).min(data.entry_points.len());
        *entry_point_count = n as u32;
        let entry_points = std::slice::from_raw_parts_mut(entry_points, n);
        for (i, entry_point) in entry_points.iter_mut().enumerate() {
            *entry_point = &data.entry_points[i] as *const spirq::EntryPoint as *mut c_void;
        }
    }

    Error::Success
}

fn str2bufstr(dst: &mut [c_char; 256], src: &[u8]) {
    let n = src.len().min(255);
    for i in 0..n {
        dst[i] = src[i] as c_char;
    }
    dst[n] = 0;
}

#[no_mangle]
pub unsafe extern "C" fn get_entry_point_profile(entry_point: EntryPoint, profile: *mut EntryPointProfile) -> Error {
    if entry_point == std::ptr::null_mut() {
        return Error::ArgumentNull;
    }
    if profile == std::ptr::null_mut() {
        return Error::ArgumentNull;
    }

    let entry_point = &*(entry_point as *const spirq::EntryPoint);
    let profile = &mut *(profile);

    str2bufstr(&mut profile.name, entry_point.name.as_bytes());

    let n = (profile.variable_count as usize).min(entry_point.vars.len());
    profile.variable_count = n;
    for (i, var) in entry_point.vars.iter().enumerate() {
        match var {
            spirq::Variable::Input { name, location, ty } => {
                Variable::Input { name, location: location.loc(), component: location.comp(), type_index:  }
            }
            spirq::Variable::Output { name, location, ty } => todo!(),
            spirq::Variable::Descriptor { name, desc_bind, desc_ty, ty, nbind } => todo!(),
            spirq::Variable::PushConstant { name, ty } => todo!(),
            spirq::Variable::SpecConstant { name, spec_id, ty } => todo!(),
        }
    }

    Error::Success
}
