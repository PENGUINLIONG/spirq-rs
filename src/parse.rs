use num_traits::FromPrimitive;
use super::{Error, Result};

pub struct Instrs<'a>(&'a [u32]);
impl<'a> Instrs<'a> {
    pub fn new(spv: &'a [u32]) -> Instrs<'a> {
        const HEADER_LEN: usize = 5;
        if spv.len() < HEADER_LEN { return Instrs(&[] as &[u32]) }
        Instrs(&spv[HEADER_LEN..])
    }
}
impl<'a> Iterator for Instrs<'a> {
    type Item = Instr<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(head) = self.0.first() {
            let len = ((*head as u32) >> 16) as usize;
            if len <= self.0.len() {
                let opcode = head & 0xFFFF;
                let instr = Instr {
                    // Force break the instruction stream if any invalid opcode
                    // is spotted.
                    opcode: opcode,
                    operands: &self.0[1..len],
                };
                self.0 = &self.0[len..];
                return Some(instr);
            }
        }
        None
    }
}


#[derive(Debug)]
pub struct Instr<'a> {
    opcode: u32,
    operands: &'a [u32],
}
impl<'a> Instr<'a> {
    /// Get the opcode of the instruction.
    pub fn opcode(&self) -> u32 { self.opcode }
    /// Get the word count of the instruction, including the first word
    /// containing the word count and opcode.
    pub fn _word_count(&self) -> usize { self.operands.len() + 1 }
    /// Get an instruction operand reader. The reader does NO boundary checking
    /// so the user code MUST make sure the implementation follows the
    /// specification.
    pub fn operands(&self) -> Operands<'a> {
        Operands(self.operands)
    }
}

pub struct Operands<'a>(&'a [u32]);
impl<'a> Operands<'a> {
    pub fn read_bool(&mut self) -> Result<bool> { self.read_u32().map(|x| x != 0) }
    pub fn read_u32(&mut self) -> Result<u32> {
        if let Some(x) = self.0.first() {
            self.0 = &self.0[1..];
            Ok(*x)
        } else { Err(Error::CorruptedSpirv) }
    }
    pub fn read_str(&mut self) -> Result<&'a str> {
        use std::os::raw::c_char;
        use std::ffi::CStr;
        let ptr = self.0.as_ptr() as *const c_char;
        let char_slice = unsafe { std::slice::from_raw_parts(ptr, self.0.len() * 4) };
        if let Some(nul_pos) = char_slice.into_iter().position(|x| *x == 0) {
            let nword = nul_pos / 4 + 1;
            self.0 = &self.0[nword..];
            if let Ok(string) = unsafe { CStr::from_ptr(ptr) }.to_str() {
                return Ok(string);
            }
        }
        Err(Error::CorruptedSpirv)
    }
    pub fn read_enum<E: FromPrimitive>(&mut self) -> Result<E> {
        self.read_u32()
            .and_then(|x| {
                FromPrimitive::from_u32(x).ok_or(Error::CorruptedSpirv)
            })
    }
    pub fn read_list(&mut self) -> Result<&'a [u32]> {
        let rv = self.0;
        self.0 = &[];
        Ok(rv)
    }
}
