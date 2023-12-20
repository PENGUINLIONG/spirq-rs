//! Constant and specialization constant representations.
use std::convert::TryFrom;

use half::f16;
use ordered_float::OrderedFloat;

use crate::{
    error::{anyhow, Result},
    ty::{ScalarType, Type},
    var::SpecId,
};

/// Typed constant value.
#[non_exhaustive]
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ConstantValue {
    Typeless(Box<[u8]>),
    Bool(bool),
    S8(i8),
    S16(i16),
    S32(i32),
    S64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F16(OrderedFloat<f16>),
    F32(OrderedFloat<f32>),
    F64(OrderedFloat<f64>),
}
impl From<&[u32]> for ConstantValue {
    fn from(x: &[u32]) -> Self {
        let bytes = x.iter().flat_map(|x| x.to_le_bytes()).collect();
        ConstantValue::Typeless(bytes)
    }
}
impl From<&[u8]> for ConstantValue {
    fn from(x: &[u8]) -> Self {
        let bytes = x.to_owned().into_boxed_slice();
        ConstantValue::Typeless(bytes)
    }
}
impl From<[u8; 4]> for ConstantValue {
    fn from(x: [u8; 4]) -> Self {
        ConstantValue::try_from(&x as &[u8]).unwrap()
    }
}
impl From<[u8; 8]> for ConstantValue {
    fn from(x: [u8; 8]) -> Self {
        ConstantValue::try_from(&x as &[u8]).unwrap()
    }
}
impl From<bool> for ConstantValue {
    fn from(x: bool) -> Self {
        Self::Bool(x)
    }
}
impl From<u32> for ConstantValue {
    fn from(x: u32) -> Self {
        Self::U32(x)
    }
}
impl From<i32> for ConstantValue {
    fn from(x: i32) -> Self {
        Self::S32(x)
    }
}
impl From<f32> for ConstantValue {
    fn from(x: f32) -> Self {
        Self::F32(OrderedFloat(x))
    }
}
impl ConstantValue {
    pub fn to_typed(&self, ty: &Type) -> Result<Self> {
        let x = match self {
            Self::Typeless(x) => x,
            _ => return Err(anyhow!("{self:?} is already typed")),
        };

        if let Some(scalar_ty) = ty.as_scalar() {
            match scalar_ty {
                ScalarType::Boolean => Ok(ConstantValue::Bool(x.iter().any(|x| x != &0))),
                ScalarType::Integer {
                    bits: 8,
                    is_signed: true,
                } if x.len() == 4 => {
                    let x = i8::from_le_bytes([x[0]]);
                    Ok(ConstantValue::S8(x))
                }
                ScalarType::Integer {
                    bits: 8,
                    is_signed: false,
                } if x.len() == 4 => {
                    let x = u8::from_le_bytes([x[0]]);
                    Ok(ConstantValue::U8(x))
                }
                ScalarType::Integer {
                    bits: 16,
                    is_signed: true,
                } if x.len() == 4 => {
                    let x = i16::from_le_bytes([x[0], x[1]]);
                    Ok(ConstantValue::S16(x))
                }
                ScalarType::Integer {
                    bits: 16,
                    is_signed: false,
                } if x.len() == 4 => {
                    let x = u16::from_le_bytes([x[0], x[1]]);
                    Ok(ConstantValue::U16(x))
                }
                ScalarType::Integer {
                    bits: 32,
                    is_signed: true,
                } if x.len() == 4 => {
                    let x = i32::from_le_bytes([x[0], x[1], x[2], x[3]]);
                    Ok(ConstantValue::S32(x))
                }
                ScalarType::Integer {
                    bits: 32,
                    is_signed: false,
                } if x.len() == 4 => {
                    let x = u32::from_le_bytes([x[0], x[1], x[2], x[3]]);
                    Ok(ConstantValue::U32(x))
                }
                ScalarType::Integer {
                    bits: 64,
                    is_signed: true,
                } if x.len() == 8 => {
                    let x = i64::from_le_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]]);
                    Ok(ConstantValue::S64(x))
                }
                ScalarType::Integer {
                    bits: 64,
                    is_signed: false,
                } if x.len() == 8 => {
                    let x = u64::from_le_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]]);
                    Ok(ConstantValue::U64(x))
                }
                ScalarType::Float { bits: 16 } if x.len() == 4 => {
                    let x = f16::from_le_bytes([x[0], x[1]]);
                    Ok(ConstantValue::F16(OrderedFloat(x)))
                },
                ScalarType::Float { bits: 32 } if x.len() == 4 => {
                    let x = f32::from_le_bytes([x[0], x[1], x[2], x[3]]);
                    Ok(ConstantValue::F32(OrderedFloat(x)))
                }
                ScalarType::Float { bits: 64 } if x.len() == 8 => {
                    let x = f64::from_le_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]]);
                    Ok(ConstantValue::F64(OrderedFloat(x)))
                }
                _ => Err(anyhow!(
                    "cannot parse {:?} from {} bytes",
                    scalar_ty,
                    x.len()
                )),
            }
        } else {
            Err(anyhow!("cannot parse {:?} as a constant value", ty))
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(x) => Some(*x),
            _ => None,
        }
    }
    pub fn to_s32(&self) -> Option<i32> {
        match self {
            Self::S32(x) => Some(*x),
            _ => None,
        }
    }
    pub fn to_u32(&self) -> Option<i32> {
        match self {
            Self::S32(x) => Some(*x),
            _ => None,
        }
    }
    pub fn to_f32(&self) -> Option<f32> {
        match self {
            Self::F32(x) => Some((*x).into()),
            _ => None,
        }
    }

    pub fn to_typeless(&self) -> Option<Box<[u8]>> {
        match self {
            Self::Typeless(x) => Some(x.clone()),
            Self::S8(x) => Some(Box::new(x.to_le_bytes())),
            Self::S16(x) => Some(Box::new(x.to_le_bytes())),
            Self::S32(x) => Some(Box::new(x.to_le_bytes())),
            Self::S64(x) => Some(Box::new(x.to_le_bytes())),
            Self::U8(x) => Some(Box::new(x.to_le_bytes())),
            Self::U16(x) => Some(Box::new(x.to_le_bytes())),
            Self::U32(x) => Some(Box::new(x.to_le_bytes())),
            Self::U64(x) => Some(Box::new(x.to_le_bytes())),
            Self::F16(x) => Some(Box::new(x.to_le_bytes())),
            Self::F32(x) => Some(Box::new(x.to_le_bytes())),
            Self::F64(x) => Some(Box::new(x.to_le_bytes())),
            Self::Bool(x) => Some(Box::new([*x as u8])),
        }
    }
}

/// Constant or specialization constant record.
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Constant {
    pub name: Option<String>,
    /// Type of constant.
    pub ty: Type,
    /// Defined value of constant, or default value of specialization constant.
    pub value: ConstantValue,
    /// Specialization constant ID, notice that this is NOT an instruction ID.
    /// It is used to identify specialization constants for graphics libraries.
    pub spec_id: Option<SpecId>,
}
impl Constant {
    /// Create a constant record with name, type and value. `ty` must be a
    /// `ScalarType`.
    pub fn new(name: Option<String>, ty: Type, value: ConstantValue) -> Self {
        Self {
            name,
            ty,
            value,
            spec_id: None,
        }
    }
    /// Create an intermediate constant record with type and value. Intermediate
    /// constants don't have names because they contribute to subexpressions in
    /// arithmetic.
    pub fn new_itm(ty: Type, value: ConstantValue) -> Self {
        Self {
            name: None,
            ty,
            value,
            spec_id: None,
        }
    }
    /// Create a specialization constant record with name, type, default value
    /// and a `SpecId`.
    pub fn new_spec(name: Option<String>, ty: Type, value: ConstantValue, spec_id: SpecId) -> Self {
        Self {
            name,
            ty,
            value: value,
            spec_id: Some(spec_id),
        }
    }
}
