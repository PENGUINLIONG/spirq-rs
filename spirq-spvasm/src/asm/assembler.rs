use std::collections::{HashMap, HashSet, hash_map::Entry};

use anyhow::{anyhow, bail, Result, Ok};
use half::f16;
use num_traits::FromPrimitive;
use spirq_core::{spirv::Op, parse::{InstructionBuilder, bin::SpirvHeader, SpirvBinary}, ty::ScalarType};
use super::tokenizer::{Token, Tokenizer, Lit};
use crate::generated;


#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum IdRef {
    Name(String),
    Id(u32),
}
#[derive(Debug, Clone)]
enum Operand {
    IdRef(IdRef),
    Literal(Lit),
    Ident(String),
}

#[derive(Debug, Clone)]
struct Instruction {
    result_id: Option<IdRef>,
    opcode: u32,
    operands: Vec<Operand>,
}

struct TokenStream<'a> {
    tokenizer: Tokenizer<'a>,
    cache: Option<Token>,
}
impl<'a> TokenStream<'a> {
    fn new(tokenizer: Tokenizer<'a>) -> Result<Self> {
        let mut out = Self {
            tokenizer,
            cache: None,
        };
        out.load_next()?;
        Ok(out)
    }

    fn load_next(&mut self) -> Result<()> {
        self.cache = self.tokenizer.next().transpose()?;
        Ok(())
    }

    fn peek(&mut self) -> Option<&Token> {
        self.cache.as_ref()
    }
    fn next(&mut self) -> Result<Option<Token>> {
        let last_cache = self.cache.take();
        self.load_next()?;
        Ok(last_cache)
    }
}

#[derive(Default)]
pub struct Assembler {
    name2id: HashMap<String, u32>,
    used_ids: HashSet<u32>,
    next_id: u32,
    bound: u32,
    // The LieteralContextDependentNumber in OpConstant depends on the type of
    // the constant. So we need to keep track of the type of each constant.
    scalar_tys: HashMap<u32, ScalarType>,
}
impl Assembler {
    pub fn new() -> Self {
        Self::default()
    }

    fn parse_opcode(&self, s: &mut TokenStream) -> Result<u32> {
        let token = s.next()?.ok_or_else(|| anyhow!("expected opcode"))?;
        match token {
            Token::Ident(ident) => {
                generated::op_from_str(&ident)
            }
            _ => Err(anyhow!("expected opcode")),
        }
    }

    fn str2idref(&self, id: String) -> IdRef {
        if let Some(id) = id.parse::<u32>().ok() {
            IdRef::Id(id)
        } else {
            IdRef::Name(id)
        }
    }
    fn parse_idref(&self, s: &mut TokenStream) -> Result<IdRef> {
        let token = s.next()?.ok_or_else(|| anyhow!("expected idref"))?;
        let idref = match token {
            Token::IdRef(id) => self.str2idref(id),
            _ => unreachable!(),
        };
        Ok(idref)
    }

    fn parse_operand(&self, s: &mut TokenStream) -> Result<Operand> {
        let token = s.next()?.ok_or_else(|| anyhow!("expected operand"))?;
        match token {
            Token::IdRef(id) => {
                let idref = self.str2idref(id);
                Ok(Operand::IdRef(idref))
            }
            Token::Literal(lit) => {
                Ok(Operand::Literal(lit.clone()))
            }
            Token::Ident(ident) => {
                Ok(Operand::Ident(ident.clone()))
            }
            _ => Err(anyhow!("expected operand, but {:?}", token)),
        }
    }

    fn parse_instr_with_result_id(&self, s: &mut TokenStream) -> Result<Instruction> {
        let result_id = self.parse_idref(s)?;
        let eq_token = s.next()?.ok_or_else(|| anyhow!("expected '='"))?;
        if !matches!(eq_token, Token::Eq) {
            bail!("expected '='");
        }
        let opcode = self.parse_opcode(s)?;

        let mut operands = Vec::new();
        while let Some(token) = s.peek() {
            match token {
                Token::Comment(_) => {
                    s.next()?;
                },
                Token::NewLine => {
                    s.next()?;
                    break;
                },
                _ => {
                    let operand = self.parse_operand(s)?;
                    operands.push(operand);
                },
            };
        }

        let out = Instruction {
            result_id: Some(result_id),
            opcode,
            operands,
        };
        Ok(out)
    }
    fn parse_instr_without_result_id(&self, s: &mut TokenStream) -> Result<Instruction> {
        let opcode = self.parse_opcode(s)?;
        let mut operands = Vec::new();

        while let Some(token) = s.peek() {
            match token {
                Token::Comment(_) => {
                    s.next()?;
                },
                Token::NewLine => {
                    s.next()?;
                    break;
                },
                _ => {
                    let operand = self.parse_operand(s)?;
                    operands.push(operand);
                },
            };
        }

        let out = Instruction {
            result_id: None,
            opcode,
            operands,
        };
        Ok(out)
    }

    fn parse_instr(&self, s: &mut TokenStream) -> Result<Option<Instruction>> {
        while let Some(token) = s.peek() {
            match token {
                Token::Comment(_) => {
                    s.next()?;
                }
                Token::NewLine => {
                    s.next()?;
                },
                Token::Ident(_) => {
                    let instr = self.parse_instr_without_result_id(s)?;
                    return Ok(Some(instr));
                }
                Token::IdRef(_) => {
                    let instr = self.parse_instr_with_result_id(s)?;
                    return Ok(Some(instr));
                }
                _ => {
                    bail!("unexpected token {:?}", token);
                }
            }
        }
        Ok(None)
    }

    fn parse_instrs(&self, s: &mut TokenStream) -> Result<Vec<Instruction>> {
        let mut instrs = Vec::new();
        while let Some(instr) = self.parse_instr(s)? {
            instrs.push(instr);
        }
        Ok(instrs)
    }

    fn parse(&self, input: &str) -> Result<Vec<Instruction>> {
        let tokenizer = Tokenizer::new(input);
        let mut s = TokenStream::new(tokenizer)?;
        self.parse_instrs(&mut s)
    }

    fn mark_id(&mut self, id: u32) {
        self.used_ids.insert(id);
    }
    fn acquire_id(&mut self, name: &str) -> u32 {
        if let Some(id) = self.name2id.get(name) {
            return *id;
        }
        let mut id = self.next_id;
        while self.used_ids.contains(&id) {
            id += 1;
        }
        self.next_id = id + 1;
        self.name2id.insert(name.to_owned(), id);
        self.used_ids.insert(id);
        id
    }
    fn process_idref(&mut self, idref: &IdRef) -> Result<IdRef> {
        let out = match idref {
            IdRef::Name(name) => {
                let id = self.acquire_id(name);
                IdRef::Id(id)
            }
            IdRef::Id(id) => {
                IdRef::Id(*id)
            }
        };
        Ok(out)
    }

    // Call this after you sanitized named refs to ID refs.
    fn assemble_op_type_int(&mut self, instr: &Instruction) -> Result<Vec<u32>> {
        if instr.operands.len() != 2 {
            bail!("OpTypeInt expected 2 operands");
        }
        let width = match instr.operands[0] {
            Operand::Literal(Lit::Int(i)) => i as u32,
            _ => bail!("OpTypeInt width expected literal integer"),
        };
        let signedness = match instr.operands[1] {
            Operand::Literal(Lit::Int(i)) => i as u32,
            _ => bail!("OpTypeInt signedness expected literal integer"),
        };

        let result_id = instr.result_id.as_ref()
            .and_then(|idref| match idref {
                IdRef::Id(id) => Some(*id),
                _ => None,
            })
            .ok_or_else(|| anyhow!("OpTypeInt expected result id"))?;

        match self.scalar_tys.entry(result_id) {
            Entry::Vacant(entry) => {
                let scalar_ty = ScalarType::Integer {
                    bits: width,
                    is_signed: signedness != 0,
                };
                entry.insert(scalar_ty);
            }
            Entry::Occupied(_) => bail!("OpTypeInt result id already exists")
        }

        let instr = InstructionBuilder::new(Op::TypeInt)
            .push(result_id)
            .push(width)
            .push(signedness)
            .build();
        Ok(instr.into_words())
    }
    // Call this after you sanitized named refs to ID refs.
    fn assemble_op_type_float(&mut self, instr: &Instruction) -> Result<Vec<u32>> {
        if instr.operands.len() != 1 {
            bail!("OpTypeFloat expected 1 operand");
        }
        let width = match instr.operands[0] {
            Operand::Literal(Lit::Int(i)) => i as u32,
            _ => bail!("OpTypeFloat width expected literal integer"),
        };

        let result_id = instr.result_id.as_ref()
            .and_then(|idref| match idref {
                IdRef::Id(id) => Some(*id),
                _ => None,
            })
            .ok_or_else(|| anyhow!("OpTypeFloat expected result id"))?;

        match self.scalar_tys.entry(result_id) {
            Entry::Vacant(entry) => {
                let scalar_ty = ScalarType::Float {
                    bits: width,
                };
                entry.insert(scalar_ty);
            }
            Entry::Occupied(_) => bail!("OpTypeFloat result id already exists")
        }

        let instr = InstructionBuilder::new(Op::TypeFloat)
            .push(result_id)
            .push(width)
            .build();
        Ok(instr.into_words())
    }
    fn assemble_op_constant(&mut self, instr: &Instruction) -> Result<Vec<u32>> {
        if instr.operands.len() != 2 {
            bail!("OpConstant expected 2 operands");
        }
        let result_type_id = match instr.operands[0] {
            Operand::IdRef(IdRef::Id(id)) => id,
            _ => bail!("OpConstant expected result type id"),
        };
        let result_id = match instr.result_id.as_ref() {
            Some(IdRef::Id(id)) => *id,
            _ => bail!("OpConstant expected result id"),
        };

        dbg!(result_type_id, &self.scalar_tys);
        let scalar_ty = self.scalar_tys.get(&result_type_id)
            .ok_or_else(|| anyhow!("OpConstant result type id not found"))?;

        fn lit2int(lit: &Lit) -> Result<i64> {
            match lit {
                Lit::Int(i) => Ok(*i),
                Lit::Float(f, exponent_bias) => {
                    let f = (*f as f32) * 2.0f32.powi(*exponent_bias);
                    Ok(f as i64)
                },
                Lit::String(_) => bail!("OpConstant expected a int or float literal"),
            }
        }
        fn lit2float(lit: &Lit) -> Result<f64> {
            match lit {
                Lit::Int(i) => Ok(*i as f64),
                Lit::Float(f, exponent_bias) => {
                    let f = (*f as f32) * 2.0f32.powi(*exponent_bias);
                    Ok(f as f64)
                },
                Lit::String(_) => bail!("OpConstant expected a int or float literal"),
            }
        }

        let value = match &instr.operands[1] {
            Operand::Literal(lit) => lit,
            _ => bail!("OpConstant expected a literal value"),
        };

        let mut value_buf = [0u32; 2];
        let value: &[u32] = match scalar_ty {
            ScalarType::Integer { bits: 8, is_signed: true } => {
                let value = lit2int(value)?;
                if let Some(value) = i8::from_i64(value) {
                    value_buf[0] = value as u32;
                    &value_buf[..1]
                } else {
                    bail!("expected a i8 literal in range [-128, 127]");
                }
            }
            ScalarType::Integer { bits: 16, is_signed: true } => {
                let value = lit2int(value)?;
                if let Some(value) = i16::from_i64(value) {
                    value_buf[0] = value as u32;
                    &value_buf[..1]
                } else {
                    bail!("expected a i16 literal in range [-32768, 32767]");
                }
            }
            ScalarType::Integer { bits: 32, is_signed: true } => {
                let value = lit2int(value)?;
                if let Some(value) = i32::from_i64(value) {
                    value_buf[0] = value as u32;
                    &value_buf[..1]
                } else {
                    bail!("expected a i32 literal in range [-2147483648, 2147483647]");
                }
            }
            ScalarType::Integer { bits: 64, is_signed: true } => {
                let value = lit2int(value)?;
                let x = value.to_le_bytes();
                value_buf[0] = u32::from_le_bytes([x[0], x[1], x[2], x[3]]);
                value_buf[1] = u32::from_le_bytes([x[4], x[5], x[6], x[7]]);
                &value_buf[..2]
            }
            ScalarType::Integer { bits: 8, is_signed: false } => {
                let value = lit2int(value)?;
                if let Some(value) = u8::from_i64(value) {
                    value_buf[0] = value as u32;
                    &value_buf[..1]
                } else {
                    bail!("expected a u8 literal in range [0, 255]");
                }
            }
            ScalarType::Integer { bits: 16, is_signed: false } => {
                let value = lit2int(value)?;
                if let Some(value) = u16::from_i64(value) {
                    value_buf[0] = value as u32;
                    &value_buf[..1]
                } else {
                    bail!("expected a u16 literal in range [0, 65535]");
                }
            }
            ScalarType::Integer { bits: 32, is_signed: false } => {
                let value = lit2int(value)?;
                if let Some(value) = u32::from_i64(value) {
                    value_buf[0] = value as u32;
                    &value_buf[..1]
                } else {
                    bail!("expected a u32 literal in range [0, 4294967295]");
                }
            }
            ScalarType::Integer { bits: 64, is_signed: false } => {
                let value = lit2int(value)?;
                let x = value.to_le_bytes();
                value_buf[0] = u32::from_le_bytes([x[0], x[1], x[2], x[3]]);
                value_buf[1] = u32::from_le_bytes([x[4], x[5], x[6], x[7]]);
                &value_buf[..2]
            }
            ScalarType::Float { bits: 16 } => {
                let value = f16::from_f64(lit2float(value)?);
                let x = value.to_bits();
                value_buf[0] = x as u32;
                &value_buf[..1]
            }
            ScalarType::Float { bits: 32 } => {
                let value = lit2float(value)? as f32;
                let x = value.to_bits();
                value_buf[0] = x;
                &value_buf[..1]
            }
            ScalarType::Float { bits: 64 } => {
                let value = lit2float(value)?;
                let x = value.to_bits().to_le_bytes();
                value_buf[0] = u32::from_le_bytes([x[0], x[1], x[2], x[3]]);
                value_buf[1] = u32::from_le_bytes([x[4], x[5], x[6], x[7]]);
                &value_buf[..2]
            }
            _ => bail!("OpConstant unsupported result type"),
        };

        let instr = InstructionBuilder::new(Op::Constant)
            .push(result_type_id)
            .push(result_id)
            .push_list(value)
            .build();
        Ok(instr.into_words())
    }

    // Call this after you sanitized named refs to ID refs.
    fn assemble_special_instr(&mut self, instr: &Instruction) -> Result<Option<Vec<u32>>> {
        const OP_TYPE_INT: u32 = Op::TypeInt as u32;
        const OP_TYPE_FLOAT: u32 = Op::TypeFloat as u32;
        const OP_CONSTANT: u32 = Op::Constant as u32;

        let out = match instr.opcode {
            OP_TYPE_INT => {
                Some(self.assemble_op_type_int(instr)?)
            }
            OP_TYPE_FLOAT => {
                Some(self.assemble_op_type_float(instr)?)
            }
            OP_CONSTANT => {
                Some(self.assemble_op_constant(instr)?)
            }
            _ => None,
        };
        Ok(out)
    }
    fn assemble_general_instr(&mut self, instr: &Instruction) -> Result<Vec<u32>> {
        let opcode = Op::from_u32(instr.opcode)
            .ok_or_else(|| anyhow!("unknown opcode {}", instr.opcode))?;
        let mut builder = InstructionBuilder::new(opcode);

        let mut operands = instr.operands.iter();
        if generated::op_has_result_type_id(instr.opcode)? {
            // The first operand in spvasm is the result type id (if the op
            // has one).
            match operands.next() {
                Some(Operand::IdRef(IdRef::Id(id))) => {
                    builder = builder.push(*id);
                    self.bound = self.bound.max(*id + 1);
                }
                _ => bail!("expected result type id"),
            }
        }

        if generated::op_has_result_id(instr.opcode)? {
            // The second operand in spvasm is the result id (if the op has one).
            match instr.result_id {
                Some(IdRef::Id(id)) => {
                    builder = builder.push(id);
                }
                _ => bail!("expected result id"),
            }
        } else {
            if instr.result_id.is_some() {
                bail!("unexpected result id");
            }
        }

        for (i, operand) in operands.enumerate() {
            match operand {
                Operand::IdRef(IdRef::Name(_)) => unreachable!(),
                Operand::IdRef(IdRef::Id(id)) => {
                    builder = builder.push(*id);
                }
                Operand::Literal(lit) => {
                    match lit {
                        Lit::Int(i) => {
                            if *i < 0 {
                                if let Some(i) = i32::from_i64(*i) {
                                    builder = builder.push(i as u32);
                                } else {
                                    bail!("literal integer out of range");
                                }
                            } else {
                                if let Some(i) = u32::from_i64(*i) {
                                    builder = builder.push(i);
                                } else {
                                    bail!("literal integer out of range");
                                }
                            }
                        },
                        Lit::Float(f, exponent_bias) => {
                            // First cast to f32.
                            let f = (*f as f32) * 2.0f32.powi(*exponent_bias);
                            // Then bit cast to u32.
                            let u = f.to_bits();
                            builder = builder.push(u);
                        },
                        Lit::String(s) => {
                            builder = builder.push_str(&s);
                        },
                    }
                }
                Operand::Ident(ident) => {
                    let ety = generated::operand_enum_type(instr.opcode, i)?;
                    let e = generated::enum_from_str(ety, &ident)?;
                    builder = builder.push(e);
                }
            }
        }

        let instr = builder.build();
        Ok(instr.into_words())
    }

    fn assemble_instr(&mut self, instr: &Instruction) -> Result<Vec<u32>> {
        if let Some(buf) = self.assemble_special_instr(instr)? {
            return Ok(buf);
        }
        let buf = self.assemble_general_instr(instr)?;
        Ok(buf)
    }

    pub fn assemble(&mut self, input: &str, header: SpirvHeader) -> Result<SpirvBinary> {
        let mut instrs = self.parse(input)?;

        // Mark all used IDs.
        for instr in &instrs {
            if let Some(result_id) = &instr.result_id {
                match result_id {
                    IdRef::Id(id) => {
                        self.mark_id(*id);
                    }
                    IdRef::Name(_) => {}
                }
            }
            for operand in &instr.operands {
                match operand {
                    Operand::IdRef(IdRef::Id(id)) => {
                        self.mark_id(*id);
                    }
                    _ => {}
                }
            }
        }

        // Transform name refs to id refs.
        for instr in &mut instrs {
            if let Some(result_id) = &mut instr.result_id {
                *result_id = self.process_idref(&result_id)?;
            };

            for operand in &mut instr.operands {
                match operand {
                    Operand::IdRef(idref) => {
                        let idref = self.process_idref(idref)?;
                        *operand = Operand::IdRef(idref);
                    }
                    _ => {}
                }
            }
        }

        // Collect instructions.
        let mut buf = Vec::new();
        for instr in instrs {
            let instr = self.assemble_instr(&instr)?;
            buf.extend(instr);
        }

        let mut spv = vec![
            0x07230203, // Magic number
            header.version, // Version
            header.generator, // Generator
            self.bound, // Bound
            0, // Reserved word
        ];
        spv.extend(buf);

        let out = SpirvBinary::from(spv);
        Ok(out)
    }
}
