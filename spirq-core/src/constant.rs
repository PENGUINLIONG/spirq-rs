use crate::ty::{ScalarType, Type};
use anyhow::{anyhow, Result};
use ordered_float::OrderedFloat;

use crate::locator::SpecId;

#[non_exhaustive]
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ConstantValue {
    Bool(bool),
    S32(i32),
    U32(u32),
    F32(OrderedFloat<f32>),
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
    pub fn try_from_dwords(x: &[u32], ty: &Type) -> Result<Self> {
        match x.len() {
            1 => {
                let bytes = u32::to_ne_bytes(x[0]);
                Self::try_from_bytes(&bytes, ty)
            }
            2 => {
                let mut bytes: [u8; 8] = [0; 8];
                let lower_bytes = u32::to_ne_bytes(x[0]);
                let upper_bytes = u32::to_ne_bytes(x[1]);
                (&mut bytes[0..4]).copy_from_slice(&lower_bytes);
                (&mut bytes[4..8]).copy_from_slice(&upper_bytes);
                Self::try_from_bytes(&bytes, ty)
            }
            _ => {
                return Err(anyhow!(
                    "cannot parse constant value from {} dwords",
                    x.len()
                ))
            }
        }
    }
    pub fn try_from_bytes(x: &[u8], ty: &Type) -> Result<Self> {
        if let Some(scalar_ty) = ty.as_scalar() {
            match scalar_ty {
                ScalarType::Boolean => Ok(ConstantValue::Bool(x.iter().any(|x| x != &0))),
                ScalarType::Integer {
                    bits: 32,
                    is_signed: true,
                } if x.len() == 4 => {
                    let x = i32::from_ne_bytes([x[0], x[1], x[2], x[3]]);
                    Ok(ConstantValue::S32(x))
                }
                ScalarType::Integer {
                    bits: 32,
                    is_signed: false,
                } if x.len() == 4 => {
                    let x = u32::from_ne_bytes([x[0], x[1], x[2], x[3]]);
                    Ok(ConstantValue::U32(x))
                }
                ScalarType::Float { bits: 32 } if x.len() == 4 => {
                    let x = f32::from_ne_bytes([x[0], x[1], x[2], x[3]]);
                    Ok(ConstantValue::F32(OrderedFloat(x)))
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
}

/// Reflection intermediate of constants and specialization constant.
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
    pub fn new(name: Option<String>, ty: Type, value: ConstantValue) -> Self {
        Self {
            name,
            ty,
            value,
            spec_id: None,
        }
    }
    pub fn new_itm(ty: Type, value: ConstantValue) -> Self {
        Self {
            name: None,
            ty,
            value,
            spec_id: None,
        }
    }
    pub fn new_spec(name: Option<String>, ty: Type, value: ConstantValue, spec_id: SpecId) -> Self {
        Self {
            name,
            ty,
            value: value,
            spec_id: Some(spec_id),
        }
    }
}
