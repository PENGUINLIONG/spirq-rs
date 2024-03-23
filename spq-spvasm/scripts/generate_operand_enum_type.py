import json

with open("assets/spirv/spirv.core.grammar.json") as f:
    j = json.load(f)

operand_kinds = {}
for instr in j["instructions"]:
    opcode = instr["opcode"]

    op_operand_kinds = {}
    if "operands" in instr:
        operands = instr["operands"]
        i = 0
        for operand in operands:
            kind = operand["kind"]
            if kind in ["IdResultType", "IdResult"]:
                continue
            if (
                kind.startswith("Id")
                or kind.startswith("Literal")
                or kind.startswith("Pair")
            ):
                i += 1
                continue

            op_operand_kinds[i] = operand["kind"]
            i += 1
    operand_kinds[opcode] = op_operand_kinds

out = []

out += [
    "use anyhow::{bail, Result};",
    "",
    "",
    "fn unknown_operand_index(i: usize) -> Result<&'static str> {",
    '    bail!("Unknown operand index: {}", i)',
    "}",
    "",
    "pub fn operand_enum_type(opcode: u32, i: usize) -> Result<&'static str> {",
    "    let out: &'static str = match opcode {",
]

for opcode, op_operand_kinds in operand_kinds.items():
    if len(op_operand_kinds) == 0:
        continue

    out += [
        f"        {opcode} => match i {{",
    ]
    for i, kind in op_operand_kinds.items():
        out += [
            f'            {i} => "{kind}",',
        ]
    out += [
        "            _ => return unknown_operand_index(i),",
        "        }",
    ]

out += [
    '        _ => bail!("{}-th operand of opcode {} is not a enum", i, opcode),',
    "    };",
    "    Ok(out)",
    "}",
    "",
]

with open("spq-spvasm/src/generated/operand_enum_type.rs", "w") as f:
    f.write("\n".join(out))
