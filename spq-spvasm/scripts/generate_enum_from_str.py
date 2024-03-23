from collections import defaultdict
import json

with open("assets/spirv/spirv.core.grammar.json") as f:
    j = json.load(f)

bit_enums = defaultdict(dict)
value_enums = defaultdict(dict)
for operand_kind in j["operand_kinds"]:
    category = operand_kind["category"]
    kind = operand_kind["kind"]

    if category == "BitEnum":
        enumerants = operand_kind["enumerants"]
        for enumerant in enumerants:
            bit_enums[kind][enumerant["enumerant"]] = enumerant["value"]
    elif category == "ValueEnum":
        enumerants = operand_kind["enumerants"]
        for enumerant in enumerants:
            value_enums[kind][enumerant["enumerant"]] = enumerant["value"]

out = []

out += [
    "use anyhow::{bail, Result};",
    "",
    "pub fn enum_from_str(ety: &str, name: &str) -> Result<u32> {",
    "    let out: u32 = match ety {",
]

# ValueEnum
for kind, enumerants in value_enums.items():
    if len(enumerants) == 0:
        continue

    out += [
        f'        "{kind}" => match name {{',
    ]
    for name, value in enumerants.items():
        out += [
            f'            "{name}" => {value},',
        ]
    out += [
        '            _ => bail!("unknown enum: {}::{}", ety, name)',
        "        }",
    ]

# BitEnum
for kind, enumerants in bit_enums.items():
    if len(enumerants) == 0:
        continue

    out += [
        f'        "{kind}" => match name {{',
    ]
    for name, value in enumerants.items():
        out += [
            f'            "{name}" => {value},',
        ]
    out += [
        '            _ => bail!("unknown enum: {}::{}", ety, name)',
        "        }",
    ]

out += [
    '        _ => bail!("unknown enum: {}::{}", ety, name),',
    "    };",
    "    Ok(out)",
    "}",
    "",
]

with open("spq-spvasm/src/generated/enum_from_str.rs", "w") as f:
    f.write("\n".join(out))
