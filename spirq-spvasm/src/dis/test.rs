use super::Disassembler;
use pretty_assertions::assert_eq;
use spirq_core::parse::SpirvBinary;

#[test]
fn test_disassembler() {
    let actual = Disassembler::new();
    let spv = include_bytes!("../../../assets/gallery.frag.spv");
    let spvasm = actual
        .name_ids(true)
        .name_type_ids(true)
        .name_const_ids(true)
        .disassemble(&SpirvBinary::from(spv.as_ref()))
        .unwrap();
    let expect = include_str!("../../../assets/gallery.frag.spvasm")
        .lines()
        .map(|x| x.trim().to_owned() + "\n")
        .collect::<Vec<_>>()
        .concat();
    assert_eq!(expect, spvasm);
}
