import json

name2op = {}
with open("assets/spirv/spirv.core.grammar.json") as f:
    j = json.load(f)

for instr in j["instructions"]:
    opname = instr["opname"]
    opcode = instr["opcode"]

    assert opname not in name2op
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

with open("spirq-spvasm/src/generated/op_to_str.rs", "w") as f:
    f.write("\n".join(out))
