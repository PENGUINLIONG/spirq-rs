import json

op_has_result_id = {}
with open("assets/spirv/spirv.core.grammar.json") as f:
    j = json.load(f)

for instr in j["instructions"]:
    opcode = instr["opcode"]

    has_result_id = False
    if "operands" in instr:
        operands = instr["operands"]
        for operand in operands:
            if operand["kind"] == "IdResult":
                has_result_id = True
                break

    op_has_result_id[opcode] = has_result_id

out = []

out += [
    "use anyhow::{bail, Result};",
    "",
    "pub fn op_has_result_id(opcode: u32) -> Result<bool> {",
    "    let out: bool = match opcode {",
]

for opcode, has_result_id in op_has_result_id.items():
    out += [f'        {opcode} => {"true" if has_result_id else "false"},']

out += [
    '        _ => bail!("Unknown opcode: {}", opcode),',
    "    };",
    "    Ok(out)",
    "}",
    "",
]

with open("spirq-spvasm/src/generated/op_has_result_id.rs", "w") as f:
    f.write("\n".join(out))
