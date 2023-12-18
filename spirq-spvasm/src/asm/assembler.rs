use std::{iter::Peekable, collections::{HashMap, HashSet}};

use anyhow::{anyhow, bail, Result};
use num_traits::FromPrimitive;
use spirq_core::{spirv::Op, parse::{Instr, InstructionBuilder}};
use super::tokenizer::{Token, Tokenizer, Lit};
use super::generated;


enum IdRef {
    Name(String),
    Id(u32),
}
enum Operand {
    IdRef(IdRef),
    Literal(Lit),
    Ident(String),
}

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
    fn peek(&mut self) -> Option<&Token> {
        self.cache.as_ref()
    }
    fn next(&mut self) -> Result<Option<Token>> {
        self.cache = self.tokenizer.next().transpose()?;
        self.tokenizer.next().transpose()
    }
}

#[derive(Default)]
pub struct Assembler {
    name2id: HashMap<String, u32>,
    used_ids: HashSet<u32>,
    next_id: u32,
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
    fn parse_idref(&self, s: &mut TokenStream) -> Result<IdRef> {
        let token = s.next()?.ok_or_else(|| anyhow!("expected idref"))?;
        match token {
            Token::IdRef(id) => {
                if let Some(id) = id.parse::<u32>().ok() {
                    Ok(IdRef::Id(id))
                } else {
                    Ok(IdRef::Name(id))
                }
            }
            _ => Err(anyhow!("expected idref")),
        }
    }

    fn parse_operand(&self, s: &mut TokenStream) -> Result<Operand> {
        let token = s.next()?.ok_or_else(|| anyhow!("expected operand"))?;
        match token {
            Token::IdRef(id) => {
                let idref = self.parse_idref(s)?;
                Ok(Operand::IdRef(idref))
            }
            Token::Literal(lit) => {
                Ok(Operand::Literal(lit))
            }
            Token::Ident(ident) => {
                Ok(Operand::Ident(ident))
            }
            _ => Err(anyhow!("expected operand, but {:?}", s.peek())),
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
        while let Some(token) = s.next()? {
            match token {
                Token::Comment(_) => {},
                Token::NewLine => break,
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

        while let Some(token) = s.next()? {
            match token {
                Token::Comment(_) => {},
                Token::NewLine => break,
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
            match token.clone() {
                Token::Comment(_) => {}
                Token::NewLine => {},
                Token::Ident(ident) => {
                    let instr = self.parse_instr_without_result_id(s)?;
                    return Ok(Some(instr));
                }
                Token::IdRef(id) => {
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
        let mut tokenizer = Tokenizer::new(input);
        let mut s = TokenStream {
            tokenizer,
            cache: None,
        };
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
                self.mark_id(*id);
                IdRef::Id(*id)
            }
        };
        Ok(out)
    }

    pub fn assemble(&mut self, input: &str) -> Result<Vec<spirq_core::parse::Instruction>> {
        let mut instrs = self.parse(input)?;

        // Transform name refs to id refs.
        for instr in &mut instrs {
            let result_id = if let Some(result_id) = &mut instr.result_id {
                let idref = self.process_idref(result_id)?;
                Some(idref)
            } else {
                None
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

        let mut out = Vec::new();
        for instr in instrs {
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
                    Operand::IdRef(IdRef::Name(name)) => unreachable!(),
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
                                let id = self.acquire_id(&s);
                                builder = builder.push(id);
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

            out.push(builder.build());
        }
        Ok(out)
    }
}
