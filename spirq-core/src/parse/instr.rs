//!  SPIR-V instruction parser.
use num_traits::FromPrimitive;
use spirv::Op;
use std::{borrow::Borrow, fmt, ops::Deref};

use crate::error::{anyhow, Result};

pub struct Instrs<'a>(&'a [u32]);
impl<'a> Instrs<'a> {
    pub fn new(spv: &'a [u32]) -> Instrs<'a> {
        const HEADER_LEN: usize = 5;
        if spv.len() < HEADER_LEN {
            return Instrs(&[] as &[u32]);
        }
        Instrs(&spv[HEADER_LEN..])
    }
}
impl<'a> Iterator for Instrs<'a> {
    type Item = &'a Instr;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(head) = self.0.first() {
            // Ignore nops.
            let opcode = head & 0xFFFF;
            if opcode == 0 {
                continue;
            }

            let len = ((*head as u32) >> 16) as usize;
            if len <= self.0.len() {
                let instr = Instr::new(&self.0[..len]);
                self.0 = &self.0[len..];
                return Some(instr.unwrap());
            } else {
                return None;
            }
        }
        None
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
        // FIXME: (penguinliong) Avoid unsafe code.
        use std::ffi::CString;
        let cstr = CString::new(x).unwrap();
        let bytes = cstr.as_bytes_with_nul();
        let words = bytes.len() / 4 + 1;
        let ptr = cstr.as_ptr() as *const u32;
        let slice = unsafe { std::slice::from_raw_parts(ptr, words) };
        self.inner.extend_from_slice(slice);
        self
    }
    pub fn build(mut self) -> Instruction {
        self.inner[0] |= (self.inner.len() as u32) << 16;
        Instruction::from(self.inner)
    }
}

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
    pub fn read_str(&mut self) -> Result<&'a str> {
        // FIXME: (penguinliong) Avoid unsafe code.
        use std::ffi::CStr;
        use std::os::raw::c_char;
        let ptr = self.0.as_ptr() as *const c_char;
        let char_slice = unsafe { std::slice::from_raw_parts(ptr, self.0.len() * 4) };
        if let Some(nul_pos) = char_slice.into_iter().position(|x| *x == 0) {
            let nword = nul_pos / 4 + 1;
            self.0 = &self.0[nword..];
            if let Ok(string) = unsafe { CStr::from_ptr(ptr) }.to_str() {
                return Ok(string);
            }
        }
        Err(anyhow!("string is not null-terminated"))
    }
    pub fn read_enum<E: FromPrimitive>(&mut self) -> Result<E> {
        self.read_u32()
            .and_then(|x| FromPrimitive::from_u32(x).ok_or(anyhow!("invalid enum value")))
    }
    pub fn read_list(&mut self) -> Result<&'a [u32]> {
        let rv = self.0;
        self.0 = &[];
        Ok(rv)
    }
}
