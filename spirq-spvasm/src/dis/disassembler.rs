use anyhow::Result;
use spirq_core::{parse::{Instr, SpirvBinary, Instrs, Operands}, spirv::Op};
use crate::generated;

pub struct Disassembler {
    print_header: bool
}
impl Disassembler {
    pub fn new() -> Self {
        Self {
            print_header: true,
        }
    }

    pub fn print_header(mut self, value: bool) -> Self {
        self.print_header = value;
        return self
    }

    fn print_operands(&self, opcode: u32, operands: &mut Operands<'_>) -> Result<String> {
        let operands = generated::print_operand(opcode, operands)?;
        let out = operands.join(" ");
        Ok(out)
    }
    fn print_opcode(&self, opcode: u32) -> Result<String> {
        let opname = generated::op_to_str(opcode)?.to_owned();
        Ok(opname)
    }

    fn print_line(&self, instr: &Instr) -> Result<String> {
        let mut operands = instr.operands();
        let opcode = instr.opcode();
        let result_type_id = if generated::op_has_result_type_id(opcode)? {
            Some(operands.read_id()?)
        } else {
            None
        };
        let result_id = if generated::op_has_result_id(opcode)? {
            Some(operands.read_id()?)
        } else {
            None
        };

        let mut out = String::new();
        if let Some(result_id) = result_id {
            out = format!("%{} = ", result_id);
        }
        out.push_str(&self.print_opcode(opcode)?);
        if let Some(result_type_id) = result_type_id {
            out.push_str(&format!(" %{}", result_type_id));
        }
        let operands_ = self.print_operands(opcode, &mut operands)?;
        if !operands_.is_empty() {
            out.push(' ');
            out.push_str(&operands_);
        }

        dbg!(&out);
        Ok(out)
    }
    fn print_lines(&self, instrs: &mut Instrs) -> Result<Vec<String>> {
        let mut out = Vec::new();
        while let Some(instr) = instrs.next()? {
            out.push(self.print_line(instr)?);
        }
        Ok(out)
    }
    pub fn disassemble(self, spv: &SpirvBinary) -> Result<String> {
        let mut out = Vec::new();

        if let Some(header) = spv.header() {
            out.push(format!("; SPIR-V"));
            let major_version = header.version >> 16;
            let minor_version = header.version & 0xffff;
            out.push(format!("; Version: {}.{}", major_version, minor_version));
            out.push(format!("; Generator: {:x}", header.generator));
            out.push(format!("; Bound: {:x}", header.bound));
            out.push(format!("; Schema: {:x}", header.schema));
        }

        let instrs = self.print_lines(&mut spv.instrs()?)?;
        out.extend(instrs);
        Ok(out.join("\n"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let spv = [
            0x07230203, 0x00010000, 0x00000008, 0x0000001, 0x00000000
        ].iter().map(|x| *x as u32).collect::<Vec<_>>();
        let spv = SpirvBinary::from(spv);
        let out = Disassembler::new().disassemble(&spv).unwrap();
        assert_eq!(out, "; SPIR-V\n; Version: 1.0\n; Generator: 8\n; Bound: 1\n; Schema: 0");
    }

    #[test]
    fn test_nop() {
        let spv = [
            0x07230203, 0x00010000, 0x00000008, 0x0000001, 0x00000000,
            0x00010000
        ].iter().map(|x| *x as u32).collect::<Vec<_>>();
        let spv = SpirvBinary::from(spv);
        let out = Disassembler::new().disassemble(&spv).unwrap();
        assert_eq!(out, "; SPIR-V\n; Version: 1.0\n; Generator: 8\n; Bound: 1\n; Schema: 0\nOpNop");
    }
}
