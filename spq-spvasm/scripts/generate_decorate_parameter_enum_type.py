import json

with open("assets/spirv/spirv.core.grammar.json") as f:
    j = json.load(f)

j = [
    parameter_kind
    for parameter_kind in j["operand_kinds"]
    if parameter_kind["kind"] == "Decoration"
][0]

parameter_kinds = {}
for instr in j["enumerants"]:
    value = instr["value"]

    op_parameter_kinds = {}
    if "parameters" in instr:
        parameters = instr["parameters"]
        i = 0
        for parameter in parameters:
            kind = parameter["kind"]
            if kind in ["IdResultType", "IdResult"]:
                continue
            if (
                kind.startswith("Id")
                or kind.startswith("Literal")
                or kind.startswith("Pair")
            ):
                i += 1
                continue

            op_parameter_kinds[i] = parameter["kind"]
            i += 1
    parameter_kinds[value] = op_parameter_kinds

out = []

out += [
    "use anyhow::{bail, Result};",
    "use spq_core::spirv::Op;",
    "",
    "fn unknown_decorate_parameter_index(decoration: u32, i: usize) -> Result<&'static str> {",
    '    let opname = Op::from_u32(decoration).map(|op| format!("{:?}", op)).unwrap_or("<unknown>".to_owned());',
    '    bail!("Unknown op {} ({}) parameter index: {}", opname, decoration, i)',
    "}",
    "",
    "pub fn decorate_parameter_enum_type(decoration: u32, i: usize) -> Result<&'static str> {",
    "    let out: &'static str = match decoration {",
]

for decoration, decorate_parameter_kinds in parameter_kinds.items():
    if len(decorate_parameter_kinds) == 0:
        continue

    out += [
        f"        {decoration} => match i {{",
    ]
    for i, kind in decorate_parameter_kinds.items():
        out += [
            f'            {i} => "{kind}",',
        ]
    out += [
        "            _ => return unknown_decorate_parameter_index(decoration, i),",
        "        },",
    ]

out += [
    '        _ => bail!("{}-th parameter of decoration {} is not a enum", i, decoration),',
    "    };",
    "    Ok(out)",
    "}",
    "",
]

with open("spq-spvasm/src/generated/decorate_parameter_enum_type.rs", "w") as f:
    f.write("\n".join(out))
