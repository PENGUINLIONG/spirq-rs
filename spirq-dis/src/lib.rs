use spirq_core::parse::{Instr, SpirvBinary, Instrs};

struct Disassembler {
}
impl Disassembler {
    pub fn apply(self, spv: &SpirvBinary) -> String {
        let mut out = String::new();

        if let Some(header) = spv.header() {
            out.push_str(&format!("; SPIR-V\n"));
            let major_version = header.version >> 16;
            let minor_version = header.version & 0xffff;
            out.push_str(&format!("; Version: {}.{}\n", major_version, minor_version));
            out.push_str(&format!("; Generator: {:x}\n", header.generator));
            out.push_str(&format!("; Bound: {:x}\n", header.bound));
            out.push_str(&format!("; Schema: {:x}\n", header.schema));
        }

        for instr in Instrs::new(spv.words()) {
            //result.push_str(&format!("{:?}\n", instr));
        }
        out
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
        let out = Disassembler{}.apply(&spv);
        assert_eq!(out, "; SPIR-V\n; Version: 1.0\n; Generator: 8\n; Bound: 1\n; Schema: 0\n");
    }
}
