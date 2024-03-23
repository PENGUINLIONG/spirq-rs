use std::collections::HashMap;

use super::auto_name;
use crate::{dis::utils::to_hexadecimal_float, generated};
use anyhow::{bail, Result};
use half::f16;
use spirq::{reflect::ReflectIntermediate, ReflectConfig};
use spirq_core::{
    parse::{Instr, Instrs, Operands, SpirvBinary},
    spirv::Op,
    ty::{self, Type},
};

/// SPIR-V disassembler.
pub struct Disassembler {
    print_header: bool,
    name_ids: bool,
    name_type_ids: bool,
    name_const_ids: bool,
    indent: bool,
}
impl Disassembler {
    /// Create a new SPIR-V disassembler.
    pub fn new() -> Self {
        Self {
            print_header: true,
            name_ids: false,
            name_type_ids: false,
            name_const_ids: false,
            indent: false,
        }
    }

    /// Print the metadata in SPIR-V header which looks like this:
    ///
    /// ```plaintext
    /// ; SPIR-V
    /// ; Version: 1.5
    /// ; Generator: Khronos Glslang Reference Front End; 11
    /// ; Bound: 406
    /// ; Schema: 0
    /// ```
    pub fn print_header(mut self, value: bool) -> Self {
        self.print_header = value;
        return self;
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
    /// Indent the output.
    pub fn indent(mut self, value: bool) -> Self {
        self.indent = value;
        self
    }

    fn print_id(&self, id: u32, id_names: &HashMap<u32, String>) -> Result<String> {
        if let Some(name) = id_names.get(&id) {
            return Ok(format!("%{}", name));
        }
        Ok(format!("%{}", id))
    }
    fn print_operands(
        &self,
        opcode: u32,
        operands: &mut Operands<'_>,
        id_names: &HashMap<u32, String>,
    ) -> Result<String> {
        let out = generated::print_operand(opcode, operands, id_names)?.join(" ");
        assert_eq!(operands.len(), 0);
        Ok(out)
    }
    fn print_opcode(&self, opcode: u32) -> Result<String> {
        let opname = generated::op_to_str(opcode)?.to_owned();
        Ok(opname)
    }

    // SPIR-V Tools emit numbers of any length as a single context dependent
    // literal for OpConstant. But don't fail if we failed to make a guess
    // of the type.
    fn print_constant_op_operand<'a>(
        &self,
        result_type_id: Option<u32>,
        operands: &mut Operands<'a>,
        itm: &ReflectIntermediate,
    ) -> Result<String> {
        let mut operands2 = operands.clone();

        let out = if let Some(result_type_id) = result_type_id {
            let ty = itm.ty_reg.get(result_type_id)?;
            match ty {
                Type::Scalar(scalar_ty) => match scalar_ty {
                    ty::ScalarType::Integer {
                        bits: 8,
                        is_signed: true,
                    } => {
                        let x = operands2.read_u32()?.to_le_bytes();
                        format!(" {}", i8::from_le_bytes([x[0]]))
                    }
                    ty::ScalarType::Integer {
                        bits: 16,
                        is_signed: true,
                    } => {
                        let x = operands2.read_u32()?.to_le_bytes();
                        format!(" {}", i16::from_le_bytes([x[0], x[1]]))
                    }
                    ty::ScalarType::Integer {
                        bits: 32,
                        is_signed: true,
                    } => {
                        let x = operands2.read_u32()?.to_le_bytes();
                        format!(" {}", i32::from_le_bytes([x[0], x[1], x[2], x[3]]))
                    }
                    ty::ScalarType::Integer {
                        bits: 64,
                        is_signed: true,
                    } => {
                        let x = operands2.read_u32()?.to_le_bytes();
                        let y = operands2.read_u32()?.to_le_bytes();
                        format!(
                            " {}",
                            i64::from_le_bytes([x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3]])
                        )
                    }
                    ty::ScalarType::Integer {
                        bits: 8,
                        is_signed: false,
                    } => {
                        let x = operands2.read_u32()?.to_le_bytes();
                        format!(" {}", u8::from_le_bytes([x[0]]))
                    }
                    ty::ScalarType::Integer {
                        bits: 16,
                        is_signed: false,
                    } => {
                        let x = operands2.read_u32()?.to_le_bytes();
                        format!(" {}", u16::from_le_bytes([x[0], x[1]]))
                    }
                    ty::ScalarType::Integer {
                        bits: 32,
                        is_signed: false,
                    } => {
                        let x = operands2.read_u32()?.to_le_bytes();
                        format!(" {}", u32::from_le_bytes([x[0], x[1], x[2], x[3]]))
                    }
                    ty::ScalarType::Integer {
                        bits: 64,
                        is_signed: false,
                    } => {
                        let x = operands2.read_u32()?.to_le_bytes();
                        let y = operands2.read_u32()?.to_le_bytes();
                        format!(
                            " {}",
                            u64::from_le_bytes([x[0], x[1], x[2], x[3], y[0], y[1], y[2], y[3]])
                        )
                    }
                    ty::ScalarType::Float { bits: 16 } => {
                        let x = operands2.read_u32()?.to_le_bytes();
                        let f = f16::from_bits(u16::from_le_bytes([x[0], x[1]]));
                        format!(" {}", to_hexadecimal_float(f))
                    }
                    ty::ScalarType::Float { bits: 32 } => {
                        let x = operands2.read_u32()?.to_le_bytes();
                        format!(" {}", f32::from_le_bytes([x[0], x[1], x[2], x[3]]))
                    }
                    ty::ScalarType::Float { bits: 64 } => {
                        let x0 = operands2.read_u32()?.to_le_bytes();
                        let x1 = operands2.read_u32()?.to_le_bytes();
                        format!(
                            " {}",
                            f64::from_le_bytes([
                                x0[0], x0[1], x0[2], x0[3], x1[0], x1[1], x1[2], x1[3]
                            ])
                        )
                    }
                    _ => bail!("unsupported scalar type for opconstant"),
                },
                _ => bail!("opconstant cannot have a non-scalar type"),
            }
        } else {
            bail!("opconstant must have a result type")
        };

        *operands = operands2;
        Ok(out)
    }

    fn print_line<'a>(
        &self,
        instr: &'a Instr,
        itm: &ReflectIntermediate,
        id_names: &HashMap<u32, String>,
    ) -> Result<String> {
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

        if opcode == (Op::Constant as u32) {
            if let Ok(operand) = self.print_constant_op_operand(result_type_id, &mut operands, itm)
            {
                out.push_str(&operand);
            } else {
                // Tolerate the error and print the operands as usual.
            }
        }

        let operands_ = self.print_operands(opcode, &mut operands, id_names)?;
        if !operands_.is_empty() {
            out.push(' ');
            out.push_str(&operands_);
        }

        Ok(out)
    }
    fn print_lines<'a>(
        &self,
        instrs: &'a mut Instrs,
        itm: &ReflectIntermediate,
        id_names: HashMap<u32, String>,
    ) -> Result<Vec<String>> {
        let mut out = Vec::new();
        while let Some(instr) = instrs.next()? {
            out.push(self.print_line(instr, itm, &id_names)?);
        }
        Ok(out)
    }

    fn print<'a>(
        &self,
        spv: &'a SpirvBinary,
        itm: &ReflectIntermediate,
        id_names: HashMap<u32, String>,
    ) -> Result<Vec<String>> {
        self.print_lines(&mut spv.instrs()?, itm, id_names)
    }

    /// Disamble SPIR-V binary into SPIR-V assembly code.
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
                    out.push(format!(
                        "; Generator: Khronos Glslang Reference Front End; {}",
                        generator_version
                    ));
                } else {
                    out.push(format!("; Generator: {}; {}", generator, generator_version));
                }
                out.push(format!("; Bound: {}", header.bound));
                out.push(format!("; Schema: {:x}", header.schema));
            }
        }

        let cfg = ReflectConfig::default();
        let itm = {
            let mut itm = ReflectIntermediate::new(&cfg)?;
            let mut instrs = spv.instrs()?;
            itm.parse_global_declrs(&mut instrs)?;
            itm
        };

        let id_names = if self.name_ids || self.name_type_ids || self.name_const_ids {
            auto_name::collect_names(&itm, self.name_ids, self.name_type_ids, self.name_const_ids)?
        } else {
            HashMap::new()
        };

        let mut instrs = self.print(spv, &itm, id_names)?;

        if self.indent {
            let max_eq_pos = instrs
                .iter()
                .filter_map(|instr| instr.find('=')) // Skip lines without an assignment.
                .max()
                .unwrap_or(0)
                .min(15);
            let mut instrs2 = Vec::new();
            for instr in instrs {
                let indent = if let Some(eq_pos) = instr.find('=') {
                    max_eq_pos - eq_pos.min(max_eq_pos)
                } else {
                    max_eq_pos + 2
                };
                instrs2.push(format!("{}{}", " ".repeat(indent), instr));
            }
            instrs = instrs2;
        }

        out.extend(instrs);
        out.push(String::new()); // Add a trailing zero to align with spirv-dis.

        Ok(out.join("\n"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let spv = [0x07230203, 0x00010000, 0x00000000, 0x0000001, 0x00000000]
            .iter()
            .map(|x| *x as u32)
            .collect::<Vec<_>>();
        let spv = SpirvBinary::from(spv);
        let out = Disassembler::new().disassemble(&spv).unwrap();
        assert_eq!(
            out,
            "; SPIR-V\n; Version: 1.0\n; Generator: 0; 0\n; Bound: 1\n; Schema: 0\n"
        );
    }

    #[test]
    fn test_nop() {
        let spv = [
            0x07230203, 0x00010000, 0x00000000, 0x0000001, 0x00000000, 0x00010000,
        ]
        .iter()
        .map(|x| *x as u32)
        .collect::<Vec<_>>();
        let spv = SpirvBinary::from(spv);
        let out = Disassembler::new().disassemble(&spv).unwrap();
        assert_eq!(
            out,
            "; SPIR-V\n; Version: 1.0\n; Generator: 0; 0\n; Bound: 1\n; Schema: 0\nOpNop\n"
        );
    }
}
