use std::collections::HashMap;

use anyhow::Result;
use spirq_core::parse::{Instr, SpirvBinary, Instrs, Operands};
use crate::generated;
use super::auto_name;

pub struct Disassembler {
    print_header: bool,
    name_ids: bool,
    name_type_ids: bool,
    name_const_ids: bool,
}
impl Disassembler {
    pub fn new() -> Self {
        Self {
            print_header: true,
            name_ids: false,
            name_type_ids: false,
            name_const_ids: false,
        }
    }

    pub fn print_header(mut self, value: bool) -> Self {
        self.print_header = value;
        return self
    }
    /// Reference intermediate values by their names rather than numeric IDs if
    /// it has been annotated by OpName.
    ///
    /// If enabled, the input SPIR-V MUST follow the standard physical layout as
    /// described in Section 2.3 in SPIR-V specification.
    pub fn name_ids(mut self, value: bool) -> Self {
        self.name_ids = value;
        self
    }
    /// Reference type definitions by their names rather than numeric IDs. A
    /// human-friendly name will be generated if it's not annotated by OpName.
    ///
    /// If enabled, the input SPIR-V MUST follow the standard physical layout as
    /// described in Section 2.3 in SPIR-V specification.
    pub fn name_type_ids(mut self, value: bool) -> Self {
        self.name_type_ids = value;
        self
    }
    /// Reference constant value by names rather than numeric IDs. A human-
    /// friendly name will be generated if it's not annotated by OpName.
    ///
    /// If enabled, the input SPIR-V MUST follow the standard physical layout as
    /// described in Section 2.3 in SPIR-V specification.
    pub fn name_const_ids(mut self, value: bool) -> Self {
        self.name_const_ids = value;
        self
    }

    fn print_id(&self, id: u32, id_names: &HashMap<u32, String>) -> Result<String> {
        if let Some(name) = id_names.get(&id) {
            return Ok(format!("%{}", name));
        }
        Ok(format!("%{}", id))
    }
    fn print_operands(&self, opcode: u32, operands: &mut Operands<'_>, id_names: &HashMap<u32, String>) -> Result<String> {
        let operands = generated::print_operand(opcode, operands, id_names)?;
        let out = operands.join(" ");
        Ok(out)
    }
    fn print_opcode(&self, opcode: u32) -> Result<String> {
        let opname = generated::op_to_str(opcode)?.to_owned();
        Ok(opname)
    }

    fn print_line<'a>(&self, instr: &'a Instr, id_names: &HashMap<u32, String>) -> Result<String> {
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
            out.push_str(&self.print_id(result_id, id_names)?);
            out.push_str(" = ");
        }
        out.push_str(&self.print_opcode(opcode)?);
        if let Some(result_type_id) = result_type_id {
            out.push_str(&format!(" {}", &self.print_id(result_type_id, id_names)?));
        }
        let operands_ = self.print_operands(opcode, &mut operands, id_names)?;
        if !operands_.is_empty() {
            out.push(' ');
            out.push_str(&operands_);
        }

        Ok(out)
    }
    fn print_lines<'a>(&self, instrs: &'a mut Instrs, id_names: HashMap<u32, String>) -> Result<Vec<String>> {
        let mut out = Vec::new();
        while let Some(instr) = instrs.next()? {
            out.push(self.print_line(instr, &id_names)?);
        }
        Ok(out)
    }

    fn print<'a>(&self, spv: &'a SpirvBinary, id_names: HashMap<u32, String>) -> Result<Vec<String>> {
        self.print_lines(&mut spv.instrs()?, id_names)
    }

    pub fn disassemble(&self, spv: &SpirvBinary) -> Result<String> {
        let mut out = Vec::new();

        if self.print_header {
            if let Some(header) = spv.header() {
                out.push(format!("; SPIR-V"));
                let major_version = header.version >> 16;
                let minor_version = (header.version >> 8) & 0xff;
                out.push(format!("; Version: {}.{}", major_version, minor_version));
                // FIXME: (penguinliong) This is a hack to match the spirv-dis
                // output.
                let generator = header.generator >> 16;
                let generator_version = header.generator & 0xffff;
                if generator == 8 {
                    out.push(format!("; Generator: Khronos Glslang Reference Front End; {}", generator_version));
                } else {
                    out.push(format!("; Generator: {}; {}", generator, generator_version));
                }
                out.push(format!("; Bound: {}", header.bound));
                out.push(format!("; Schema: {:x}", header.schema));
            }
        }

        let id_names = if self.name_ids || self.name_type_ids || self.name_const_ids {
            auto_name::collect_names(spv, self.name_ids, self.name_type_ids, self.name_const_ids)?
        } else {
            HashMap::new()
        };

        let instrs = self.print(spv, id_names)?;
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
            0x07230203, 0x00010000, 0x00000000, 0x0000001, 0x00000000
        ].iter().map(|x| *x as u32).collect::<Vec<_>>();
        let spv = SpirvBinary::from(spv);
        let out = Disassembler::new().disassemble(&spv).unwrap();
        assert_eq!(out, "; SPIR-V\n; Version: 1.0\n; Generator: 0; 0\n; Bound: 1\n; Schema: 0");
    }

    #[test]
    fn test_nop() {
        let spv = [
            0x07230203, 0x00010000, 0x00000000, 0x0000001, 0x00000000,
            0x00010000
        ].iter().map(|x| *x as u32).collect::<Vec<_>>();
        let spv = SpirvBinary::from(spv);
        let out = Disassembler::new().disassemble(&spv).unwrap();
        assert_eq!(out, "; SPIR-V\n; Version: 1.0\n; Generator: 0; 0\n; Bound: 1\n; Schema: 0\nOpNop");
    }
}
