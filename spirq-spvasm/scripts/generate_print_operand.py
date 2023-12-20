import json
from typing import List, Optional

with open("assets/spirv/spirv.core.grammar.json") as f:
    j = json.load(f)

ops = {}
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
    ops[opcode] = opname, op_operand_kinds

operand_parameters = {}
for operand_kind in j["operand_kinds"]:
    category = operand_kind["category"]
    kind = operand_kind["kind"]

    enum_parameters = {}
    if category == "BitEnum":
        enumerants = operand_kind["enumerants"]
        for enumerant in enumerants:
            value = enumerant["value"]
            parameter_name = enumerant["enumerant"]
            if "parameters" in enumerant:
                parameters = [p["kind"] for p in enumerant["parameters"]]
            else:
                parameters = []
            enum_parameters[value] = parameter_name, parameters
    elif category == "ValueEnum":
        enumerants = operand_kind["enumerants"]
        for enumerant in enumerants:
            value = enumerant["value"]
            parameter_name = enumerant["enumerant"]
            if "parameters" in enumerant:
                parameters = [p["kind"] for p in enumerant["parameters"]]
            else:
                parameters = []
            enum_parameters[value] = parameter_name, parameters
    else:
        continue
    operand_parameters[kind] = category, enum_parameters


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
    "#[allow(dead_code)]",
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
]


def print_operand(kind: str, quantifier: Optional[str], indent: int) -> List[str]:
    padding = " " * (indent * 4)

    out = []
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
        out += [padding + f"out.extend(print_enum_{kind}(operands)?);"]

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

    return out


for kind, (category, parameters) in operand_parameters.items():
    out += [
        "#[allow(non_snake_case)]",
        "#[allow(dead_code)]",
        f"fn print_enum_{kind}(operands: &mut Operands) -> Result<Vec<String>> {{",
        "    let value = operands.read_u32()?;",
        "    #[allow(unused_mut)]",
        f'    let mut out = vec![enum_to_str(&"{kind}", value)?];',
    ]
    if category == "ValueEnum":
        out += [
            "    match value {",
        ]
        for value, (parameter_name, params) in parameters.items():
            out += [
                f"        // {parameter_name}",
                f"        {value} => {{",
            ]
            for i, param in enumerate(params):
                out += print_operand(param, None, 3)

            out += [
                "        }",
            ]
        out += [
            "        _ => {},",
            "    }",
        ]
    elif category == "BitEnum":
        for value, (parameter_name, params) in parameters.items():
            out += [
                f"    // {parameter_name}",
                f"    if value & {value} != 0 {{",
            ]
            for i, param in enumerate(params):
                out += print_operand(param, None, 2)

            out += [
                "    }",
            ]
    else:
        raise RuntimeError(f"unsupported enum category: {category}")

    out += [
        "    Ok(out)",
        "}",
        "",
    ]

out += [
    "pub fn print_operand(opcode: u32, operands: &mut Operands) -> Result<Vec<String>> {",
    "    let mut out: Vec<String> = Vec::new();",
    "    match opcode {",
]

for opcode, (opname, op_operand_kinds) in ops.items():
    out += [
        f"        // {opname}",
        f"        {opcode} => {{",
    ]
    for kind, quantifier in op_operand_kinds:
        if kind in ["IdResult", "IdResultType"]:
            continue

        out += print_operand(kind, quantifier, 3)
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
