mod bin;
mod instr;

pub use bin::SpirvBinary;
pub use instr::{Instr, Instrs, Instruction, InstructionBuilder, Operands};

pub mod error {
    pub use anyhow::{anyhow, Error, Result};
}
