import json

name2op = {}
with open("assets/spirv/spirv.core.grammar.json") as f:
    j = json.load(f)

for instr in j["instructions"]:
    opname = instr["opname"]
    opcode = instr["opcode"]

    # Third party extension names should be suppressed so that they don't show
    # up when disassembling.
    is_khr_op = ("extensions" not in instr) or (
        opname.endswith("KHR") or opname.endswith("EXT")
    )
    is_official_ext = ("extensions" not in instr) or any(
        x.startswith("SPV_KHR") or x.startswith("SPV_EXT") for x in instr["extensions"]
    )
    if is_khr_op and is_official_ext:
        name2op[opname] = opcode

out = []

out += [
    "#![allow(unreachable_patterns)]",
    "use anyhow::{bail, Result};",
    "",
    "pub fn op_to_str(opcode: u32) -> Result<&'static str> {",
    "    let out: &'static str = match opcode {",
]

for opname, opcode in name2op.items():
    out += [f'        {opcode} => "{opname}",']

out += [
    '        _ => bail!("Unknown opcode: {}", opcode),',
    "    };",
    "    Ok(out)",
    "}",
    "",
]

with open("spq-spvasm/src/generated/op_to_str.rs", "w") as f:
    f.write("\n".join(out))
