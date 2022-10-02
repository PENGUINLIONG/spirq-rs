use clap::Parser;
use serde_json::json;
use spirq::{
    ty::{StructMember, Type},
    EntryPoint, ReflectConfig,
};
use std::{
    fs::File,
    io::{stderr, Write},
    path::Path,
    process::exit,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Input SPIR-V file paths.")]
    in_path: String,

    #[arg(
        short,
        long,
        help = "Output JSON file path. The output is printed to stdout if this \
        path is not given."
    )]
    out_path: Option<String>,

    #[arg(
        long,
        help = "Reference all resources even they are never used by the entry \
        points. By default, only the referenced resources are reflected."
    )]
    reference_all_resources: bool,

    #[arg(
        long,
        help = "Combine separate sampled image and sampler at a same \
        descriptor set and binding. By default, they are listed as separate \
        objects."
    )]
    combine_image_samplers: bool,

    #[arg(
        long,
        help = "Generate unique names for every resource variable, structure \
        types, and type members. By default, the names are assigned with debug \
        annotations in the input SPIR-V."
    )]
    generate_unique_names: bool,
}

fn build_spirv_binary<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
    use std::io::Read;
    let path = path.as_ref();
    if !path.is_file() {
        return None;
    }
    let mut buf = Vec::new();
    if let Ok(mut f) = File::open(&path) {
        if buf.len() & 3 != 0 {
            // Misaligned input.
            return None;
        }
        f.read_to_end(&mut buf).ok()?;
    }
    Some(buf)
}

fn member2json(member: &StructMember) -> serde_json::Value {
    json!({
        "Name": member.name,
        "Offset": member.offset,
        "MemberType": ty2json(&member.ty)
    })
}
fn ty2json(ty: &Type) -> serde_json::Value {
    match ty {
        Type::Matrix(x) => json!({
            "Kind": "Matrix",
            "AxisOrder": x.major.map(|x| format!("{:?}", x)),
            "VectorType": x.vec_ty.to_string(),
            "Count": x.nvec,
            "Stride": x.stride,
        }),
        Type::Array(x) => json!({
            "Kind": "Array",
            "ElementType": ty2json(&*x.proto_ty),
            "Count": x.nrepeat,
            "Stride": x.stride
        }),
        Type::Struct(x) => json!({
            "Kind": "Struct",
            "Members": x.members.iter().map(member2json).collect::<Vec<_>>()
        }),
        Type::DevicePointer(x) => json!({
            "Kind": "Pointer",
            "TargetType": ty2json(&*x.pointee_ty)
        }),
        _ => json!(ty.to_string()),
    }
}
fn entry_point2json(entry_point: &EntryPoint) -> serde_json::Value {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut descs = Vec::new();
    let mut push_consts = Vec::new();
    let mut spec_consts = Vec::new();
    for var in entry_point.vars.iter() {
        use spirq::Variable::*;
        match var {
            Input { name, location, ty } => {
                let j = json!({
                    "Name": name.as_ref(),
                    "Location": location.loc(),
                    "Component": location.comp(),
                    "Type": ty2json(&ty),
                });
                inputs.push(j);
            }
            Output { name, location, ty } => {
                let j = json!({
                    "Name": name.as_ref(),
                    "Location": location.loc(),
                    "Component": location.comp(),
                    "Type": ty2json(&ty),
                });
                outputs.push(j);
            }
            Descriptor {
                name,
                desc_bind,
                desc_ty,
                ty,
                nbind,
            } => {
                let j = json!({
                    "Name": name.as_ref(),
                    "Set": desc_bind.set(),
                    "Binding": desc_bind.bind(),
                    "DescriptorType": format!("{desc_ty:?}"),
                    "Type": ty2json(&ty),
                    "Count": nbind,
                });
                descs.push(j);
            }
            PushConstant { name, ty } => {
                let j = json!({
                    "Name": name.as_ref(),
                    "Type": ty2json(&ty),
                });
                push_consts.push(j);
            }
            SpecConstant { name, spec_id, ty } => {
                let j = json!({
                    "Name": name.as_ref(),
                    "SpecId": spec_id,
                    "Type": ty2json(&ty),
                });
                spec_consts.push(j);
            }
        }
    }

    let mut exec_modes = Vec::new();
    for exec_mode in entry_point.exec_modes.iter() {
        let operands = exec_mode
            .operands
            .iter()
            .map(|operand| {
                json!({
                    "Value": operand.value.to_u32(),
                    "SpecId": operand.spec_id,
                })
            })
            .collect::<Vec<_>>();
        let j = json!({
            "ExecutionMode": format!("{:?}", exec_mode.exec_mode),
            "Operands": operands,
        });
        exec_modes.push(j);
    }

    json!({
        "EntryPoint": entry_point.name,
        "ExecutionModel": format!("{:?}", entry_point.exec_model),
        "ExecutionModes": exec_modes,
        "Variables": {
            "Inputs": inputs,
            "Outputs": outputs,
            "Descriptors": descs,
            "PushConstants": push_consts,
            "SpecConstants": spec_consts
        },
    })
}

fn main() {
    let args = Args::parse();

    let in_path: &str = &args.in_path;

    let spv = match build_spirv_binary(&in_path) {
        Some(x) => x,
        None => {
            writeln!(stderr(), "cannot read spirv: {in_path}").unwrap();
            exit(-1);
        }
    };
    let mut reflect_cfg = ReflectConfig::new();
    reflect_cfg
        .spv(spv)
        .ref_all_rscs(args.reference_all_resources)
        .combine_img_samplers(args.combine_image_samplers)
        .gen_unique_names(args.generate_unique_names);
    let entry_points = match reflect_cfg.reflect() {
        Ok(x) => x,
        Err(e) => {
            writeln!(stderr(), "{e}").unwrap();
            writeln!(stderr(), "cannot reflect spirv: {in_path}").unwrap();
            exit(-1);
        }
    };

    for entry_point in entry_points {
        let j = entry_point2json(&entry_point);
        let json = serde_json::to_string_pretty(&j).unwrap();

        if let Some(ref out_path) = args.out_path {
            let mut f = match File::create(out_path) {
                Ok(x) => x,
                Err(e) => {
                    writeln!(stderr(), "{e}").unwrap();
                    writeln!(stderr(), "cannot create output file: {out_path}").unwrap();
                    exit(-1);
                }
            };
            if let Err(e) = f.write(json.as_bytes()) {
                writeln!(stderr(), "{e}").unwrap();
                writeln!(stderr(), "cannot write to output file: {out_path}").unwrap();
                exit(-1);
            };
        } else {
            println!("{}", serde_json::to_string_pretty(&j).unwrap());
        }
    }
}
