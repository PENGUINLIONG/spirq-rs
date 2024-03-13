//!  SPIR-V instruction parser.
use anyhow::bail;
use num_traits::FromPrimitive;
use spirv::Op;
use std::{borrow::Borrow, fmt, ops::Deref};

use crate::error::{anyhow, Result};

pub struct Instrs<'a> {
    inner: &'a [u32],
    cache: Option<&'a Instr>,
}
impl<'a> Instrs<'a> {
    pub fn new(spv: &'a [u32]) -> Result<Instrs<'a>> {
        let mut out = Instrs {
            inner: &spv,
            cache: None,
        };
        out.load_next()?;
        Ok(out)
    }

    fn load_next(&mut self) -> Result<()> {
        let mut new_cache = None;
        while let Some(head) = self.inner.first() {
            let len = ((*head as u32) >> 16) as usize;
            // Report zero-length instructions.
            if len == 0 {
                bail!("instruction length is zero");
            }

            if len <= self.inner.len() {
                let instr = Instr::new(&self.inner[..len])?;
                self.inner = &self.inner[len..];
                new_cache = Some(instr);
            } else {
                if len < self.inner.len() {
                    bail!("instruction is truncated");
                }
            }
            break;
        }

        self.cache = new_cache;
        Ok(())
    }

    pub fn peek(&self) -> Option<&'a Instr> {
        self.cache.clone()
    }
    pub fn next(&mut self) -> Result<Option<&'a Instr>> {
        let last_cache = self.cache.take();
        self.load_next()?;
        return Ok(last_cache);
    }
    pub fn next_non_nop(&mut self) -> Result<Option<&'a Instr>> {
        while let Some(instr) = self.next()? {
            if instr.opcode() != Op::Nop as u32 {
                return Ok(Some(instr));
            }
        }
        Ok(None)
    }
}

pub struct Instr {
    inner: [u32],
}
impl Instr {
    pub fn new(x: &[u32]) -> Result<&Instr> {
        if x.len() >= 1 {
            Ok(unsafe { std::mem::transmute(x) })
        } else {
            Err(anyhow!("instruction is too short"))
        }
    }

    /// Get the instruction opcode.
    pub fn opcode(&self) -> u32 {
        self.inner[0] & 0xFFFF
    }
    /// Get the instruction op.
    pub fn op(&self) -> Op {
        Op::from_u32(self.opcode()).unwrap()
    }
    /// Get the word count of the instruction, including the first word
    /// containing the word count and opcode.
    pub fn word_count(&self) -> usize {
        self.inner.len()
    }
    /// Get an instruction operand reader. The reader does NO boundary checking
    /// so the user code MUST make sure the implementation follows the
    /// specification.
    pub fn operands(&self) -> Operands<'_> {
        Operands(&self.inner[1..])
    }
}
impl AsRef<[u32]> for Instr {
    fn as_ref(&self) -> &[u32] {
        &self.inner
    }
}
impl ToOwned for Instr {
    type Owned = Instruction;
    fn to_owned(&self) -> Self::Owned {
        Instruction::from(&self.inner)
    }
}
impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?}", self.op(), &self.inner[1..])
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    inner: Vec<u32>,
}
impl From<Vec<u32>> for Instruction {
    fn from(x: Vec<u32>) -> Instruction {
        Instruction { inner: x }
    }
}
impl From<&[u32]> for Instruction {
    fn from(x: &[u32]) -> Instruction {
        Instruction::from(x.to_owned())
    }
}
impl AsRef<[u32]> for Instruction {
    fn as_ref(&self) -> &[u32] {
        &self.inner
    }
}
impl Borrow<Instr> for Instruction {
    fn borrow(&self) -> &Instr {
        Instr::new(self.inner.as_ref()).unwrap()
    }
}
impl Deref for Instruction {
    type Target = Instr;
    fn deref(&self) -> &Instr {
        self.borrow()
    }
}
impl Instruction {
    pub fn builder(op: Op) -> InstructionBuilder {
        InstructionBuilder::new(op)
    }
    pub fn into_words(self) -> Vec<u32> {
        self.inner
    }
}

pub struct InstructionBuilder {
    inner: Vec<u32>,
}
impl InstructionBuilder {
    pub fn new(op: Op) -> InstructionBuilder {
        InstructionBuilder {
            inner: vec![(op as u32) & 0xFFFF],
        }
    }
    pub fn push(mut self, x: u32) -> Self {
        self.inner.push(x);
        self
    }
    pub fn push_list(mut self, x: &[u32]) -> Self {
        self.inner.extend_from_slice(x);
        self
    }
    pub fn push_str(mut self, x: &str) -> Self {
        use std::ffi::CString;
        let cstr = CString::new(x).unwrap();
        let bytes = cstr.as_bytes();
        let words = bytes.len() / 4 + 1;
        // Pad the string with zeros.
        let bytes = {
            let mut out = bytes.to_owned();
            out.resize(words * 4, 0);
            out
        };
        let slice: &[u32] = bytemuck::cast_slice(&bytes);
        self.inner.extend_from_slice(slice);
        self
    }
    pub fn build(mut self) -> Instruction {
        self.inner[0] |= (self.inner.len() as u32) << 16;
        Instruction::from(self.inner)
    }
}

#[derive(Clone)]
pub struct Operands<'a>(&'a [u32]);
impl<'a> Operands<'a> {
    pub fn read_bool(&mut self) -> Result<bool> {
        self.read_u32().map(|x| x != 0)
    }
    pub fn read_u32(&mut self) -> Result<u32> {
        if let Some(x) = self.0.first() {
            self.0 = &self.0[1..];
            Ok(*x)
        } else {
            Err(anyhow!("operand is too short"))
        }
    }
    pub fn read_f32(&mut self) -> Result<f32> {
        self.read_u32().map(|x| f32::from_bits(x))
    }
    pub fn read_id(&mut self) -> Result<u32> {
        self.read_u32()
    }
    pub fn read_str(&mut self) -> Result<&'a str> {
        use std::ffi::CStr;
        // Find the word with a trailing zero.
        let ieos = self
            .0
            .iter()
            .position(|x| (x >> 24) == 0)
            .ok_or(anyhow!("string is not null-terminated"))?;

        let slice: &[u32] = &self.0[..ieos + 1];
        self.0 = &self.0[ieos + 1..];
        let bytes: &[u8] = bytemuck::cast_slice(slice);
        let cstr = CStr::from_bytes_until_nul(bytes)?;
        Ok(cstr.to_str()?)
    }
    pub fn read_list(&mut self) -> Result<&'a [u32]> {
        let rv = self.0;
        self.0 = &[];
        Ok(rv)
    }
}
impl<'a> Iterator for Operands<'a> {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.read_u32().ok()
    }
}
impl<'a> ExactSizeIterator for Operands<'a> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
