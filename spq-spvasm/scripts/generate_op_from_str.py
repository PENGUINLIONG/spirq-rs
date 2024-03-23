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
    "use anyhow::{bail, Result};",
    "",
    "pub fn op_from_str(opname: &str) -> Result<u32> {",
    "    let out: u32 = match opname {",
]

for opname, opcode in name2op.items():
    out += [f'        "{opname}" => {opcode},']

out += [
    '        _ => bail!("Unknown opname: {}", opname),',
    "    };",
    "    Ok(out)",
    "}",
    "",
]

with open("spq-spvasm/src/generated/op_from_str.rs", "w") as f:
    f.write("\n".join(out))
