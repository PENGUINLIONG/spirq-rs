use crate::{
    constant::{Constant, ConstantValue},
    error::{anyhow, Error, Result},
    spirv::Op,
    ty::{ScalarType, Type},
};
use fnv::FnvHashMap as HashMap;

type InstrId = u32;

fn const_id_not_found(id: InstrId) -> Error {
    anyhow!("constant id {} not found", id)
}
fn evaluation_failed(op: Op, result_ty: &Type, operands: &[ConstantValue]) -> Error {
    anyhow!("cannot evaluate {op:?} with {operands:?} as {result_ty:?}")
}
fn broken_expr_tree(id: InstrId) -> Error {
    anyhow!("broken expression tree at id {}", id)
}

#[derive(Default)]
pub struct Evaluator {
    ext_instr_set: HashMap<InstrId, String>,
    values: HashMap<InstrId, Constant>,
}
impl Evaluator {
    pub fn new() -> Self {
        Self {
            ext_instr_set: HashMap::default(),
            values: HashMap::default(),
        }
    }

    pub fn import_ext_instr_set(&mut self, id: InstrId, name: String) -> Result<()> {
        use std::collections::hash_map::Entry;
        match self.ext_instr_set.entry(id) {
            Entry::Vacant(entry) => {
                entry.insert(name);
                Ok(())
            }
            Entry::Occupied(_) => Err(anyhow!("extended instruction set id {} already exists", id)),
        }
    }
    pub fn get_ext_instr_set_name(&mut self, id: InstrId) -> Result<&str> {
        self.ext_instr_set
            .get(&id)
            .ok_or(anyhow!("missing extended instruction set id {}", id))
            .map(|s| s.as_str())
    }

    pub fn set(&mut self, id: InstrId, constant: Constant) -> Result<&Constant> {
        use std::collections::hash_map::Entry;
        match self.values.entry(id) {
            Entry::Vacant(entry) => Ok(entry.insert(constant)),
            Entry::Occupied(_) => Err(anyhow!("constant id {} already exists", id)),
        }
    }
    pub fn get(&self, id: InstrId) -> Result<&Constant> {
        self.values.get(&id).ok_or(const_id_not_found(id))
    }

    pub fn get_value(&self, id: InstrId) -> Result<&ConstantValue> {
        self.get(id).map(|c| &c.value)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&InstrId, &Constant)> {
        self.values.iter()
    }

    pub fn evaluate(
        op: spirv::Op,
        result_ty: &Type,
        operands: &[ConstantValue],
    ) -> Result<ConstantValue> {
        let value = match op {
            // Convert ops.
            Op::ConvertFToS => {
                let x = match operands {
                    [ConstantValue::F32(x)] => *x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(x.0 as i32),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::ConvertFToU => {
                let x = match operands {
                    [ConstantValue::F32(x)] => *x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(x.0 as u32),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::ConvertSToF => {
                let x = match operands {
                    [ConstantValue::S32(x)] => *x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Float { bits: 32 }) => {
                        ConstantValue::F32((x as f32).into())
                    }
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::ConvertUToF => {
                let x = match operands {
                    [ConstantValue::U32(x)] => *x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Float { bits: 32 }) => {
                        ConstantValue::F32((x as f32).into())
                    }
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::SConvert => {
                let x = match operands {
                    [ConstantValue::S32(x)] => *x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(x),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::UConvert => {
                let x = match operands {
                    [ConstantValue::U32(x)] => *x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(x),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::FConvert => {
                let x = match operands {
                    [ConstantValue::F32(x)] => *x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Float { bits: 32 }) => ConstantValue::F32(x),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::Bitcast => {
                let bits = match operands {
                    [ConstantValue::U32(x)] => x.to_ne_bytes(),
                    [ConstantValue::S32(x)] => x.to_ne_bytes(),
                    [ConstantValue::F32(x)] => x.to_ne_bytes(),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(i32::from_ne_bytes(bits)),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(u32::from_ne_bytes(bits)),
                    Type::Scalar(ScalarType::Float { bits: 32 }) => {
                        ConstantValue::F32(f32::from_ne_bytes(bits).into())
                    }
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            // 3.42.13. Arithmetic Instructions
            Op::SNegate => {
                let a = match operands {
                    [ConstantValue::S32(x)] => *x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(-a),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::FNegate => {
                let a = match operands {
                    [ConstantValue::F32(x)] => *x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Float { bits: 32 }) => ConstantValue::F32(-a),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::IAdd => {
                let (a, b) = match operands {
                    [ConstantValue::S32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::S32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32((a + b) as i32),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32((a + b) as u32),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::FAdd => {
                let (a, b) = match operands {
                    [ConstantValue::F32(x), ConstantValue::F32(y)] => (*x, *y),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Float { bits: 32 }) => ConstantValue::F32(a + b),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::ISub => {
                let (a, b) = match operands {
                    [ConstantValue::S32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::S32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(((a - b) & 0xffffffff) as i32),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(((a - b) & 0xffffffff) as u32),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::FSub => {
                let (a, b) = match operands {
                    [ConstantValue::F32(x), ConstantValue::F32(y)] => (*x, *y),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Float { bits: 32 }) => ConstantValue::F32(a - b),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::IMul => {
                let (a, b) = match operands {
                    [ConstantValue::S32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::S32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(((a * b) & 0xffffffff) as i32),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(((a * b) & 0xffffffff) as u32),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::FMul => {
                let (a, b) = match operands {
                    [ConstantValue::F32(x), ConstantValue::F32(y)] => (*x, *y),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Float { bits: 32 }) => ConstantValue::F32(a * b),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::UDiv => {
                let (a, b) = match operands {
                    [ConstantValue::S32(x), ConstantValue::S32(y)] => (*x as u64, *y as u64),
                    [ConstantValue::S32(x), ConstantValue::U32(y)] => (*x as u64, *y as u64),
                    [ConstantValue::U32(x), ConstantValue::S32(y)] => (*x as u64, *y as u64),
                    [ConstantValue::U32(x), ConstantValue::U32(y)] => (*x as u64, *y as u64),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(((a / b) & 0xffffffff) as i32),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(((a / b) & 0xffffffff) as u32),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::SDiv => {
                let (a, b) = match operands {
                    [ConstantValue::S32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::S32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(((a / b) & 0xffffffff) as i32),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(((a / b) & 0xffffffff) as u32),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::FDiv => {
                let (a, b) = match operands {
                    [ConstantValue::F32(x), ConstantValue::F32(y)] => (*x, *y),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Float { bits: 32 }) => ConstantValue::F32(a / b),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::UMod => {
                let (a, b) = match operands {
                    [ConstantValue::S32(x), ConstantValue::S32(y)] => (*x as u64, *y as u64),
                    [ConstantValue::S32(x), ConstantValue::U32(y)] => (*x as u64, *y as u64),
                    [ConstantValue::U32(x), ConstantValue::S32(y)] => (*x as u64, *y as u64),
                    [ConstantValue::U32(x), ConstantValue::U32(y)] => (*x as u64, *y as u64),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(((a % b) & 0xffffffff) as i32),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(((a % b) & 0xffffffff) as u32),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::SRem => {
                let (a, b) = match operands {
                    [ConstantValue::S32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::S32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(((a % b) & 0xffffffff) as i32),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::SMod => {
                let (a, b) = match operands {
                    [ConstantValue::S32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::S32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::S32(y)] => (*x as i64, *y as i64),
                    [ConstantValue::U32(x), ConstantValue::U32(y)] => (*x as i64, *y as i64),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32((a.rem_euclid(b) & 0xffffffff) as i32),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::FRem => {
                let (a, b) = match operands {
                    [ConstantValue::F32(x), ConstantValue::F32(y)] => (*x, *y),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Float { bits: 32 }) => ConstantValue::F32(a % b),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::FMod => {
                let (a, b) = match operands {
                    [ConstantValue::F32(x), ConstantValue::F32(y)] => (*x, *y),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Float { bits: 32 }) => {
                        ConstantValue::F32(a.rem_euclid(*b).into())
                    }
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            // 3.42.14. Bit Instructions
            Op::ShiftRightLogical => {
                if operands.len() != 2 {
                    return Err(evaluation_failed(op, result_ty, operands));
                }
                let base = match operands[0] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                let shift = match operands[1] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(i32::from_ne_bytes((base >> shift).to_ne_bytes())),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(base >> shift),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::ShiftRightArithmetic => {
                if operands.len() != 2 {
                    return Err(evaluation_failed(op, result_ty, operands));
                }
                // See https://www.reddit.com/r/rust/comments/2lp3il/where_is_arithmetic_signed_rightshift.
                let base = match operands[0] {
                    ConstantValue::S32(x) => x,
                    ConstantValue::U32(x) => i32::from_ne_bytes(x.to_ne_bytes()),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                let shift = match operands[1] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(base >> shift),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(u32::from_ne_bytes((base >> shift).to_ne_bytes())),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::ShiftLeftLogical => {
                if operands.len() != 2 {
                    return Err(evaluation_failed(op, result_ty, operands));
                }
                let base = match operands[0] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                let shift = match operands[1] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(i32::from_ne_bytes((base << shift).to_ne_bytes())),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(base << shift),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::BitwiseOr => {
                if operands.len() != 2 {
                    return Err(evaluation_failed(op, result_ty, operands));
                }
                let x = match operands[0] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                let y = match operands[1] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(i32::from_ne_bytes((x | y).to_ne_bytes())),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(x | y),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::BitwiseXor => {
                if operands.len() != 2 {
                    return Err(evaluation_failed(op, result_ty, operands));
                }
                let x = match operands[0] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                let y = match operands[1] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(i32::from_ne_bytes((x ^ y).to_ne_bytes())),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(x ^ y),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::BitwiseAnd => {
                if operands.len() != 2 {
                    return Err(evaluation_failed(op, result_ty, operands));
                }
                let x = match operands[0] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                let y = match operands[1] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(i32::from_ne_bytes((x & y).to_ne_bytes())),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(x & y),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            Op::Not => {
                if operands.len() != 1 {
                    return Err(evaluation_failed(op, result_ty, operands));
                }
                let x = match operands[0] {
                    ConstantValue::S32(x) => u32::from_ne_bytes(x.to_ne_bytes()),
                    ConstantValue::U32(x) => x,
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                };
                match result_ty {
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    }) => ConstantValue::S32(i32::from_ne_bytes((!x).to_ne_bytes())),
                    Type::Scalar(ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    }) => ConstantValue::U32(!x),
                    _ => return Err(evaluation_failed(op, result_ty, operands)),
                }
            }
            _ => return Err(evaluation_failed(op, result_ty, operands)),
        };
        Ok(value)
    }

    pub fn interpret(
        &mut self,
        op: spirv::Op,
        result_id: InstrId,
        result_ty: &Type,
        operand_ids: &[InstrId],
    ) -> Result<&Constant> {
        let mut operands = Vec::new();
        for operand in operand_ids.iter() {
            if let Ok(operand) = self.get_value(*operand) {
                operands.push(operand.to_owned());
            } else {
                return Err(broken_expr_tree(result_id));
            }
        }

        let value = Self::evaluate(op, result_ty, &operands)?;
        self.set(result_id, Constant::new_itm(result_ty.clone(), value))
    }

    pub fn constants(&self) -> Vec<Constant> {
        let mut out: Vec<_> = self.values.iter().collect();
        out.sort_by_key(|c| c.0);
        out.into_iter().map(|c| c.1.clone()).collect()
    }
}
