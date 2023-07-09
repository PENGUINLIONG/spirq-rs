use std::collections::{hash_map::Entry, HashMap};

use anyhow::{anyhow, Error, Result};
use spirq_interface::{Constant, ConstantValue};
use spirq_parse::Instr;
use spirq_types::{ScalarType, Type, TypeRegistry};
use spirv::Op;

type InstrId = u32;

fn const_id_not_found(id: InstrId) -> Error {
    anyhow!("constant id {} not found", id)
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
            ext_instr_set: HashMap::new(),
            values: HashMap::new(),
        }
    }

    pub fn import_ext_instr_set(&mut self, id: InstrId, name: String) -> Result<()> {
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

    fn get_s32(&self, id: InstrId) -> Result<i32> {
        match self.get_value(id) {
            Ok(ConstantValue::S32(x)) => Ok(*x),
            _ => Err(const_id_not_found(id)),
        }
    }
    fn get_u32(&self, id: InstrId) -> Result<u32> {
        match self.get_value(id) {
            Ok(ConstantValue::U32(x)) => Ok(*x),
            _ => Err(const_id_not_found(id)),
        }
    }
    fn get_f32(&self, id: InstrId) -> Result<f32> {
        match self.get_value(id) {
            Ok(ConstantValue::F32(x)) => Ok((*x).into()),
            _ => Err(const_id_not_found(id)),
        }
    }

    pub fn interpret(&mut self, ty_reg: &TypeRegistry, instr: &Instr) -> Result<&Constant> {
        let mut operands = instr.operands();
        let result_ty_id = operands.read_u32()?;
        let result_ty = ty_reg.get(result_ty_id)?;
        let result_id = operands.read_u32()?;

        let value = match instr.op() {
            // Convert ops.
            Op::ConvertFToS => match result_ty {
                Type::Scalar(ScalarType::Signed(4)) => {
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::F32(x) => ConstantValue::S32((*x).0 as i32),
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::ConvertFToU => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::F32(x) => ConstantValue::U32((*x).0 as u32),
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::ConvertSToF => match result_ty {
                Type::Scalar(ScalarType::Float(4)) => {
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::S32(x) => ConstantValue::F32((*x as f32).into()),
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::ConvertUToF => match result_ty {
                Type::Scalar(ScalarType::Float(4)) => {
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::U32(x) => ConstantValue::F32(((*x) as f32).into()),
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::SConvert => match result_ty {
                Type::Scalar(ScalarType::Signed(4)) => {
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::S32(x) => ConstantValue::S32(*x),
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::UConvert => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::U32(x) => ConstantValue::U32(*x),
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::FConvert => match result_ty {
                Type::Scalar(ScalarType::Float(4)) => {
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::F32(x) => ConstantValue::F32((*x).into()),
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::Bitcast => match result_ty {
                Type::Scalar(ScalarType::Signed(4)) => {
                    let bytes = match self.get_value(operands.read_u32()?)? {
                        ConstantValue::U32(x) => x.to_ne_bytes(),
                        ConstantValue::S32(x) => x.to_ne_bytes(),
                        ConstantValue::F32(x) => x.to_ne_bytes(),
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    ConstantValue::S32(i32::from_ne_bytes(bytes))
                }
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let bytes = match self.get_value(operands.read_u32()?)? {
                        ConstantValue::U32(x) => x.to_ne_bytes(),
                        ConstantValue::S32(x) => x.to_ne_bytes(),
                        ConstantValue::F32(x) => x.to_ne_bytes(),
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    ConstantValue::U32(u32::from_ne_bytes(bytes))
                }
                Type::Scalar(ScalarType::Float(4)) => {
                    let bytes = match self.get_value(operands.read_u32()?)? {
                        ConstantValue::U32(x) => x.to_ne_bytes(),
                        ConstantValue::S32(x) => x.to_ne_bytes(),
                        ConstantValue::F32(x) => x.to_ne_bytes(),
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    ConstantValue::F32(f32::from_ne_bytes(bytes).into())
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            // 3.42.13. Arithmetic Instructions
            Op::SNegate => match result_ty {
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = self.get_s32(operands.read_u32()?)?;
                    ConstantValue::S32(-x)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::FNegate => match result_ty {
                Type::Scalar(ScalarType::Float(4)) => {
                    let x = self.get_f32(operands.read_u32()?)?;
                    ConstantValue::F32((-x).into())
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::IAdd => match result_ty {
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::S32(x)) => *x,
                        Ok(ConstantValue::U32(x)) => *x as i32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    let y = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::S32(y)) => *y,
                        Ok(ConstantValue::U32(y)) => *y as i32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    ConstantValue::S32(x + y)
                }
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let x = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::U32(x)) => *x,
                        Ok(ConstantValue::S32(x)) => *x as u32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    let y = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::U32(y)) => *y,
                        Ok(ConstantValue::S32(y)) => *y as u32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    ConstantValue::U32(x + y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::FAdd => match result_ty {
                Type::Scalar(ScalarType::Float(4)) => {
                    let x = self.get_f32(operands.read_u32()?)?;
                    let y = self.get_f32(operands.read_u32()?)?;
                    ConstantValue::F32((x + y).into())
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::ISub => match result_ty {
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::S32(x)) => *x,
                        Ok(ConstantValue::U32(x)) => *x as i32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    let y = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::S32(y)) => *y,
                        Ok(ConstantValue::U32(y)) => *y as i32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    ConstantValue::S32(x - y)
                }
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let x = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::U32(x)) => *x,
                        Ok(ConstantValue::S32(x)) => *x as u32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    let y = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::U32(y)) => *y,
                        Ok(ConstantValue::S32(y)) => *y as u32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    ConstantValue::U32(x - y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::FSub => match result_ty {
                Type::Scalar(ScalarType::Float(4)) => {
                    let x = self.get_f32(operands.read_u32()?)?;
                    let y = self.get_f32(operands.read_u32()?)?;
                    ConstantValue::F32((x - y).into())
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::IMul => match result_ty {
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::S32(x)) => *x,
                        Ok(ConstantValue::U32(x)) => *x as i32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    let y = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::S32(y)) => *y,
                        Ok(ConstantValue::U32(y)) => *y as i32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    ConstantValue::S32(x * y)
                }
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let x = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::U32(x)) => *x,
                        Ok(ConstantValue::S32(x)) => *x as u32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    let y = match self.get_value(operands.read_u32()?) {
                        Ok(ConstantValue::U32(y)) => *y,
                        Ok(ConstantValue::S32(y)) => *y as u32,
                        _ => return Err(broken_expr_tree(result_id)),
                    };
                    ConstantValue::U32(x * y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::FMul => match result_ty {
                Type::Scalar(ScalarType::Float(4)) => {
                    let x = self.get_f32(operands.read_u32()?)?;
                    let y = self.get_f32(operands.read_u32()?)?;
                    ConstantValue::F32((x * y).into())
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::UDiv => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let x = self.get_u32(operands.read_u32()?)?;
                    let y = self.get_u32(operands.read_u32()?)?;
                    ConstantValue::U32(x / y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::SDiv => match result_ty {
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = self.get_s32(operands.read_u32()?)?;
                    let y = self.get_s32(operands.read_u32()?)?;
                    ConstantValue::S32(x / y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::FDiv => match result_ty {
                Type::Scalar(ScalarType::Float(4)) => {
                    let x = self.get_f32(operands.read_u32()?)?;
                    let y = self.get_f32(operands.read_u32()?)?;
                    ConstantValue::F32((x / y).into())
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::UMod => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let x = self.get_u32(operands.read_u32()?)?;
                    let y = self.get_u32(operands.read_u32()?)?;
                    ConstantValue::U32(x % y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::SRem => match result_ty {
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = self.get_s32(operands.read_u32()?)?;
                    let y = self.get_s32(operands.read_u32()?)?;
                    ConstantValue::S32(x % y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::SMod => match result_ty {
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = self.get_s32(operands.read_u32()?)?;
                    let y = self.get_s32(operands.read_u32()?)?;
                    ConstantValue::S32(x % y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::FRem => match result_ty {
                Type::Scalar(ScalarType::Float(4)) => {
                    let x = self.get_f32(operands.read_u32()?)?;
                    let y = self.get_f32(operands.read_u32()?)?;
                    ConstantValue::F32((x % y).into())
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::FMod => match result_ty {
                Type::Scalar(ScalarType::Float(4)) => {
                    let x = self.get_f32(operands.read_u32()?)?;
                    let y = self.get_f32(operands.read_u32()?)?;
                    ConstantValue::F32((x % y).into())
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            // 3.42.14. Bit Instructions
            Op::ShiftRightLogical => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let base = self.get_u32(operands.read_u32()?)?;
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::S32(x) => {
                            let shift = *x as u32;
                            ConstantValue::U32(base >> shift)
                        }
                        ConstantValue::U32(x) => {
                            let shift = x;
                            ConstantValue::U32(base >> shift)
                        }
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                Type::Scalar(ScalarType::Signed(4)) => {
                    let base = self.get_s32(operands.read_u32()?)? as u32;
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::S32(x) => {
                            let shift = *x as u32;
                            ConstantValue::S32((base >> shift) as i32)
                        }
                        ConstantValue::U32(x) => {
                            let shift = x;
                            ConstantValue::S32((base >> shift) as i32)
                        }
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::ShiftRightArithmetic => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let base = self.get_u32(operands.read_u32()?)? as i32;
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::S32(x) => {
                            let shift = *x as u32;
                            ConstantValue::U32((base >> shift) as u32)
                        }
                        ConstantValue::U32(x) => {
                            let shift = x;
                            ConstantValue::U32((base >> shift) as u32)
                        }
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                Type::Scalar(ScalarType::Signed(4)) => {
                    let base = self.get_s32(operands.read_u32()?)?;
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::S32(x) => {
                            let shift = *x as u32;
                            ConstantValue::S32(base >> shift)
                        }
                        ConstantValue::U32(x) => {
                            let shift = x;
                            ConstantValue::S32(base >> shift)
                        }
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::ShiftLeftLogical => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let base = self.get_u32(operands.read_u32()?)?;
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::S32(x) => {
                            let shift = *x as u32;
                            ConstantValue::U32(base << shift)
                        }
                        ConstantValue::U32(x) => {
                            let shift = x;
                            ConstantValue::U32(base << shift)
                        }
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                Type::Scalar(ScalarType::Signed(4)) => {
                    let base = self.get_s32(operands.read_u32()?)? as u32;
                    match self.get_value(operands.read_u32()?)? {
                        ConstantValue::S32(x) => {
                            let shift = *x as u32;
                            ConstantValue::S32((base << shift) as i32)
                        }
                        ConstantValue::U32(x) => {
                            let shift = x;
                            ConstantValue::S32((base << shift) as i32)
                        }
                        _ => return Err(broken_expr_tree(result_id)),
                    }
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::BitwiseOr => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let x = self.get_u32(operands.read_u32()?)?;
                    let y = self.get_u32(operands.read_u32()?)?;
                    ConstantValue::U32(x | y)
                }
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = self.get_s32(operands.read_u32()?)?;
                    let y = self.get_s32(operands.read_u32()?)?;
                    ConstantValue::S32(x | y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::BitwiseXor => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let x = self.get_u32(operands.read_u32()?)?;
                    let y = self.get_u32(operands.read_u32()?)?;
                    ConstantValue::U32(x ^ y)
                }
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = self.get_s32(operands.read_u32()?)?;
                    let y = self.get_s32(operands.read_u32()?)?;
                    ConstantValue::S32(x ^ y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::BitwiseAnd => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let x = self.get_u32(operands.read_u32()?)?;
                    let y = self.get_u32(operands.read_u32()?)?;
                    ConstantValue::U32(x & y)
                }
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = self.get_s32(operands.read_u32()?)?;
                    let y = self.get_s32(operands.read_u32()?)?;
                    ConstantValue::S32(x & y)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            Op::Not => match result_ty {
                Type::Scalar(ScalarType::Unsigned(4)) => {
                    let x = self.get_u32(operands.read_u32()?)?;
                    ConstantValue::U32(!x)
                }
                Type::Scalar(ScalarType::Signed(4)) => {
                    let x = self.get_s32(operands.read_u32()?)?;
                    ConstantValue::S32(!x)
                }
                _ => return Err(broken_expr_tree(result_id)),
            },
            _ => return Err(broken_expr_tree(result_id)),
        };
        self.set(result_id, Constant::new_itm(result_ty.clone(), value))
    }

    pub fn constants(&self) -> Vec<Constant> {
        let mut out: Vec<_> = self.values.iter().collect();
        out.sort_by_key(|c| c.0);
        out.into_iter().map(|c| c.1.clone()).collect()
    }
}
