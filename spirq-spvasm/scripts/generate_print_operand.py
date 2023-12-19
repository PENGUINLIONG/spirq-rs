import json

with open("assets/spirv/spirv.core.grammar.json") as f:
    j = json.load(f)

operand_kinds = {}
for instr in j["instructions"]:
    opcode = instr["opcode"]
    opname = instr["opname"]

    op_operand_kinds = []
    if "operands" in instr:
        operands = instr["operands"]
        for operand in operands:
            kind = operand["kind"]
            quantifier = operand["quantifier"] if "quantifier" in operand else None
            op_operand_kinds.append((kind, quantifier))
    operand_kinds[opcode] = opname, op_operand_kinds

out = []

out += [
    "use anyhow::{bail, Result};",
    "use spirq_core::parse::Operands;",
    "use super::enum_to_str::enum_to_str;",
    "",
    "fn print_id(operands: &mut Operands) -> Result<String> {",
    '    Ok(format!("%{}", operands.read_u32()?))',
    "}",
    "fn print_u32(operands: &mut Operands) -> Result<String> {",
    "    Ok(operands.read_u32()?.to_string())",
    "}",
    "fn print_f32(operands: &mut Operands) -> Result<String> {",
    "    Ok(operands.read_f32()?.to_string())",
    "}",
    "fn print_str(operands: &mut Operands) -> Result<String> {",
    '    Ok(format!(r#""{}""#, operands.read_str()?))',
    "}",
    "fn print_list(operands: &mut Operands) -> Result<Vec<String>> {",
    "    let out = operands.read_list()?",
    "        .iter()",
    "        .map(|x| x.to_string())",
    "        .collect::<Vec<_>>();",
    "    Ok(out)",
    "}",
    "fn print_pair_id_id_list(operands: &mut Operands) -> Result<Vec<String>> {",
    "    let mut out = Vec::new();",
    "    for pair in operands.read_list()?.chunks(2) {",
    "        if pair.len() != 2 {",
    '            bail!("operands does not pair up");',
    "        }",
    '        let seg = format!("%{} %{}", pair[0], pair[1]);',
    "        out.push(seg);",
    "    }",
    "    Ok(out)",
    "}",
    "fn print_pair_id_u32_list(operands: &mut Operands) -> Result<Vec<String>> {",
    "    let mut out = Vec::new();",
    "    for pair in operands.read_list()?.chunks(2) {",
    "        if pair.len() != 2 {",
    '            bail!("operands does not pair up");',
    "        }",
    '        let seg = format!("%{} {}", pair[0], pair[1]);',
    "        out.push(seg);",
    "    }",
    "    Ok(out)",
    "}",
    "fn print_pair_u32_id_list(operands: &mut Operands) -> Result<Vec<String>> {",
    "    let mut out = Vec::new();",
    "    for pair in operands.read_list()?.chunks(2) {",
    "        if pair.len() != 2 {",
    '            bail!("operands does not pair up");',
    "        }",
    '        let seg = format!("{} %{}", pair[0], pair[1]);',
    "        out.push(seg);",
    "    }",
    "    Ok(out)",
    "}",
    "",
    "pub fn print_operand(opcode: u32, operands: &mut Operands) -> Result<Vec<String>> {",
    "    let mut out: Vec<String> = Vec::new();",
    "    match opcode {",
]

for opcode, (opname, op_operand_kinds) in operand_kinds.items():
    if len(op_operand_kinds) == 0:
        continue

    out += [
        f"        // {opname}",
        f"        {opcode} => {{",
    ]
    for kind, quantifier in op_operand_kinds:
        padding = " " * 12

        if kind in ["IdResult", "IdResultType"]:
            continue

        out += [
            padding + f"// {kind}" + (f" {quantifier}" if quantifier else ""),
        ]

        if quantifier == "*":
            out += [
                padding + "while !operands.is_empty() {",
            ]
            padding += "    "
        elif quantifier == "?":
            out += [
                padding + "if !operands.is_empty() {",
            ]
            padding += "    "
        elif quantifier is None:
            pass
        else:
            raise RuntimeError(f"unknown quantifier {quantifier}")

        # Literal
        if kind == "LiteralInteger":
            out += [padding + "out.push(print_u32(operands)?);"]
        elif kind == "LiteralFloat":
            out += [padding + "out.push(print_f32(operands)?);"]
        elif kind == "LiteralString":
            out += [padding + "out.push(print_str(operands)?);"]
        elif kind == "LiteralContextDependentNumber":
            out += [padding + "out.extend(print_list(operands)?);"]
        elif kind.startswith("Literal"):
            out += [padding + "out.push(print_u32(operands)?);"]
        # Id
        elif kind.startswith("Id"):
            out += [padding + "out.push(print_id(operands)?);"]
        # Pair
        elif kind == "PairIdRefIdRef":
            out += [padding + "out.extend(print_pair_id_id_list(operands)?);"]
        elif kind == "PairIdRefLiteralInteger":
            out += [padding + "out.extend(print_pair_id_u32_list(operands)?);"]
        elif kind == "PairLiteralIntegerIdRef":
            out += [padding + "out.extend(print_pair_u32_id_list(operands)?);"]
        # Enum
        else:
            out += [
                padding + f'out.push(enum_to_str("{kind}", operands.read_u32()?)?);'
            ]

        if quantifier == "*":
            out += [
                padding[:-4] + "}",
            ]
        elif quantifier == "?":
            out += [
                padding[:-4] + "}",
            ]
        elif quantifier is None:
            pass
        else:
            raise RuntimeError(f"unknown quantifier {quantifier}")

    # Deal with extra operands. We don't know what they are but we can print them as u32 anyway.
    out += [
        "        }",
    ]

out += [
    '        _ => bail!("unsupported opcode {}", opcode),',
    "    };",
    "    while !operands.is_empty() {",
    '        out.push(format!("!{}", operands.read_u32()?));',
    "    }",
    "    Ok(out)",
    "}",
    "",
]

with open("spirq-spvasm/src/generated/print_operand.rs", "w") as f:
    f.write("\n".join(out))
