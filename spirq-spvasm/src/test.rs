use pretty_assertions::assert_eq;
use spirq::spirv;
use spirq_core::parse::bin::SpirvHeader;

use crate::asm::Assembler;
use crate::dis::Disassembler;

#[test]
fn test_asm_dis_roundtrip() {
    let code = r#"
; SPIR-V
; Version: SPIRV_VERSION
; Generator: 0; 0
; Bound: 13
; Schema: 0
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
"#
    .trim_start();
    let code = code.replace("SPIRV_VERSION", &format!("{}.{}", spirv::MAJOR_VERSION, spirv::MINOR_VERSION));
    let header = SpirvHeader::default();
    let spv = Assembler::new().assemble(&code, header).unwrap();
    let spvasm = Disassembler::new()
        .name_type_ids(true)
        .disassemble(&spv.into())
        .unwrap();
    assert_eq!(code, spvasm);
}

#[test]
fn test_gallery_roundtrip() {
    let code = include_str!("../../assets/gallery.frag.spvasm")
        .lines()
        // (penguinliong) For some reason our reassembled SPIR-V use less IDs
        // than the GLSLang output. Workaround here.
        .skip(5)
        .map(|x| x.trim().to_owned() + "\n")
        .collect::<Vec<_>>()
        .concat();
    let header = SpirvHeader::new(0x00010500, 0x0008000b);
    let spv = Assembler::new().assemble(&code, header).unwrap();
    let spvasm = Disassembler::new()
        .print_header(false)
        .name_ids(true)
        .name_type_ids(true)
        .name_const_ids(true)
        .disassemble(&spv.into())
        .unwrap();
    assert_eq!(code, spvasm);
}
