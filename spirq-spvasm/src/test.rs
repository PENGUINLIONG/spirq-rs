use pretty_assertions::assert_eq;
use spirq_core::parse::bin::SpirvHeader;

use crate::asm::Assembler;
use crate::dis::Disassembler;

#[test]
fn test_asm_dis_roundtrip() {
    let code = r#"
; SPIR-V
; Version: 1.5
; Generator: 0; 0
; Bound: 0
; Schema: 0
OpMemoryModel Logical GLSL450
%void = OpTypeVoid
%void_0 = OpTypeVoid
%void_1 = OpTypeVoid
%void_2 = OpTypeVoid
%void_3 = OpTypeVoid
%void_4 = OpTypeVoid
%void_5 = OpTypeVoid
%void_6 = OpTypeVoid
%void_7 = OpTypeVoid
%void_8 = OpTypeVoid
%void_9 = OpTypeVoid
%void_10 = OpTypeVoid
"#.trim();
    let header = SpirvHeader::default();
    let spv = Assembler::new().assemble(code, header).unwrap();
    let spvasm = Disassembler::new().name_type_ids(true).disassemble(&spv.into()).unwrap();
    assert_eq!(code, spvasm);
}
