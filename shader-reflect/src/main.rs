use clap::Parser;
use serde_json::json;
use spirq::prelude::*;
use spirq::ty;
use std::{
    borrow::Borrow,
    fs::File,
    io::{stderr, Write},
    path::{Path, PathBuf},
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

    #[arg(
        short = 'I',
        help = "The base directories of standard includes (`#include <...>`) \
        in compilation of GLSL or HLSL shader sources."
    )]
    include_directories: Vec<String>,

    #[arg(
        short = 'D',
        help = "Compiler definitions in compilation of GLSL or HLSL shader \
        sources."
    )]
    definitions: Vec<String>,

    #[arg(
        short,
        long,
        help = "Shader entry point function name in compilation of GLSL or \
        HLSL shader."
    )]
    entry_point: Option<String>,
}

fn read_spirv_bianry(path: &str) -> SpirvBinary {
    let spv = match std::fs::read(&path) {
        Ok(x) => x,
        Err(e) => {
            writeln!(stderr(), "{}", e.to_string()).unwrap();
            writeln!(stderr(), "cannot read from SPIR-V binary: {}", path).unwrap();
            exit(-1);
        }
    };
    if spv.len() % 4 != 0 {
        // Misaligned input.
        writeln!(stderr(), "spirv binary must align to 4 bytes: {}", path).unwrap();
        exit(-1);
    }
    SpirvBinary::from(spv)
}

fn compile_shader_source(
    path: &str,
    args: &Args,
    src_lang: shaderc::SourceLanguage,
    shader_kind: shaderc::ShaderKind,
) -> SpirvBinary {
    let src = match std::fs::read_to_string(path) {
        Ok(x) => x,
        Err(e) => {
            writeln!(stderr(), "{}", e.to_string()).unwrap();
            writeln!(stderr(), "cannot read from input shader source: {}", path).unwrap();
            exit(-1);
        }
    };

    let mut opt = match shaderc::CompileOptions::new() {
        Some(x) => x,
        None => {
            writeln!(stderr(), "cannot create shaderc compile option").unwrap();
            exit(-1);
        }
    };
    opt.set_target_env(
        shaderc::TargetEnv::Vulkan,
        shaderc::EnvVersion::Vulkan1_2 as u32,
    );
    opt.set_source_language(src_lang);
    opt.set_auto_bind_uniforms(false);
    opt.set_optimization_level(shaderc::OptimizationLevel::Zero);
    opt.set_include_callback(|name, ty, src_path, _depth| {
        use shaderc::{IncludeType, ResolvedInclude};
        let path = match ty {
            IncludeType::Relative => {
                let cur_dir = Path::new(src_path).parent()
                    .ok_or("the shader source is not living in a filesystem, but attempts to include a relative path")?;
                cur_dir.join(name)
            },
            IncludeType::Standard => {
                args.include_directories.iter()
                    .find_map(|incl_dir| {
                        let path = PathBuf::from(incl_dir).join(name);
                        if path.exists() { Some(path) } else { None }
                    })
                    .ok_or(format!("cannot find \"{}\" in include directories", name))?
            },
        };

        let path_lit = path.to_string_lossy().to_string();
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("cannot read from \"{}\": {}", path_lit, e.to_string()))?;
        let incl = ResolvedInclude { resolved_name: path_lit, content };
        Ok(incl)
    });
    for (k, v) in args.definitions.iter().map(|x| {
        if let Some((a, b)) = x.split_once('=') {
            (a, Some(b))
        } else {
            (x.as_ref(), None)
        }
    }) {
        opt.add_macro_definition(k, v);
    }
    opt.set_generate_debug_info();

    let entry_point = args
        .entry_point
        .as_ref()
        .map(|x| x.borrow())
        .unwrap_or("main");

    let mut compiler = match shaderc::Compiler::new() {
        Some(x) => x,
        None => {
            writeln!(stderr(), "cannot create compiler instance").unwrap();
            exit(-1);
        }
    };
    let art = match compiler.compile_into_spirv(&src, shader_kind, &path, entry_point, Some(&opt)) {
        Ok(x) => x,
        Err(e) => {
            writeln!(stderr(), "{}", e.to_string()).unwrap();
            writeln!(stderr(), "cannot compile shader source: {}", path).unwrap();
            exit(-1);
        }
    };

    SpirvBinary::from(art.as_binary())
}

fn get_spirv_bianry(path: &str, args: &Args) -> SpirvBinary {
    // Ensure the source file exists.
    if !Path::new(path).is_file() {
        writeln!(stderr(), "input file doesn't exist").unwrap();
        exit(-1);
    }

    // Extension names to shader types.
    let ext_map = &[
        (
            ".vert",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Vertex,
        ),
        (
            ".tesc",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::TessControl,
        ),
        (
            ".tese",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::TessEvaluation,
        ),
        (
            ".geom",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Geometry,
        ),
        (
            ".frag",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Fragment,
        ),
        (
            ".comp",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Compute,
        ),
        (
            ".mesh",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Mesh,
        ),
        (
            ".task",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Task,
        ),
        (
            ".rgen",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::RayGeneration,
        ),
        (
            ".rint",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Intersection,
        ),
        (
            ".rahit",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::AnyHit,
        ),
        (
            ".rchit",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::ClosestHit,
        ),
        (
            ".rmiss",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Miss,
        ),
        (
            ".rcall",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Callable,
        ),
        (
            ".vert.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Vertex,
        ),
        (
            ".tesc.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::TessControl,
        ),
        (
            ".tese.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::TessEvaluation,
        ),
        (
            ".geom.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Geometry,
        ),
        (
            ".frag.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Fragment,
        ),
        (
            ".comp.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Compute,
        ),
        (
            ".mesh.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Mesh,
        ),
        (
            ".task.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Task,
        ),
        (
            ".rgen.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::RayGeneration,
        ),
        (
            ".rint.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Intersection,
        ),
        (
            ".rahit.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::AnyHit,
        ),
        (
            ".rchit.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::ClosestHit,
        ),
        (
            ".rmiss.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Miss,
        ),
        (
            ".rcall.glsl",
            shaderc::SourceLanguage::GLSL,
            shaderc::ShaderKind::Callable,
        ),
        (
            ".vert.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::Vertex,
        ),
        (
            ".tesc.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::TessControl,
        ),
        (
            ".tese.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::TessEvaluation,
        ),
        (
            ".geom.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::Geometry,
        ),
        (
            ".frag.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::Fragment,
        ),
        (
            ".comp.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::Compute,
        ),
        (
            ".mesh.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::Mesh,
        ),
        (
            ".task.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::Task,
        ),
        (
            ".rgen.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::RayGeneration,
        ),
        (
            ".rint.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::Intersection,
        ),
        (
            ".rahit.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::AnyHit,
        ),
        (
            ".rchit.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::ClosestHit,
        ),
        (
            ".rmiss.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::Miss,
        ),
        (
            ".rcall.hlsl",
            shaderc::SourceLanguage::HLSL,
            shaderc::ShaderKind::Callable,
        ),
    ];

    // Discovered as shader source files.
    for (ext, src_lang2, shader_kind2) in ext_map {
        if path.ends_with(ext) {
            return compile_shader_source(path, args, *src_lang2, *shader_kind2);
        }
    }

    // Otherwise it's considered be a compiled SPIR-V binary.
    read_spirv_bianry(path)
}

fn member2json(member: &ty::StructMember) -> serde_json::Value {
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
            "AxisOrder": x.axis_order.map(|x| format!("{:?}", x)),
            "VectorType": x.vector_ty.to_string(),
            "Count": x.vector_count,
            "Stride": x.stride,
        }),
        Type::Array(x) => json!({
            "Kind": "Array",
            "ElementType": ty2json(&*x.element_ty),
            "Count": x.element_count,
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
fn desc_ty2json(desc_ty: &DescriptorType) -> serde_json::Value {
    match desc_ty {
        DescriptorType::Sampler => json!("Sampler"),
        DescriptorType::CombinedImageSampler => json!("CombinedImageSampler"),
        DescriptorType::SampledImage => json!("SampledImage"),
        DescriptorType::StorageImage {
            access_ty: AccessType::ReadOnly,
        } => json!("StorageImage(ReadOnly)"),
        DescriptorType::StorageImage {
            access_ty: AccessType::WriteOnly,
        } => json!("StorageImage(WriteOnly)"),
        DescriptorType::StorageImage {
            access_ty: AccessType::ReadWrite,
        } => json!("StorageImage(ReadWrite)"),
        DescriptorType::UniformTexelBuffer => json!("UniformTexelBuffer"),
        DescriptorType::StorageTexelBuffer {
            access_ty: AccessType::ReadOnly,
        } => json!("StorageTexelBuffer(ReadOnly)"),
        DescriptorType::StorageTexelBuffer {
            access_ty: AccessType::WriteOnly,
        } => json!("StorageTexelBuffer(WriteOnly)"),
        DescriptorType::StorageTexelBuffer {
            access_ty: AccessType::ReadWrite,
        } => json!("StorageTexelBuffer(ReadWrite)"),
        DescriptorType::UniformBuffer => json!("UniformBuffer"),
        DescriptorType::StorageBuffer {
            access_ty: AccessType::ReadOnly,
        } => json!("StorageBuffer(ReadOnly)"),
        DescriptorType::StorageBuffer {
            access_ty: AccessType::WriteOnly,
        } => json!("StorageBuffer(WriteOnly)"),
        DescriptorType::StorageBuffer {
            access_ty: AccessType::ReadWrite,
        } => json!("StorageBuffer(ReadWrite)"),
        DescriptorType::InputAttachment {
            input_attachment_index,
        } => json!(format!("InputAttachment({})", input_attachment_index)),
        DescriptorType::AccelStruct => json!("AccelStruct"),
    }
}
fn entry_point2json(entry_point: &EntryPoint) -> serde_json::Value {
    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut descs = Vec::new();
    let mut push_consts = Vec::new();
    let mut spec_consts = Vec::new();
    for var in entry_point.vars.iter() {
        match var {
            Variable::Input{ name, location, ty } => {
                let j = json!({
                    "Name": name.as_ref(),
                    "Location": location.loc(),
                    "Component": location.comp(),
                    "Type": ty2json(&ty),
                });
                inputs.push(j);
            }
            Variable::Output{ name, location, ty } => {
                let j = json!({
                    "Name": name.as_ref(),
                    "Location": location.loc(),
                    "Component": location.comp(),
                    "Type": ty2json(&ty),
                });
                outputs.push(j);
            }
            Variable::Descriptor{
                name,
                desc_bind,
                desc_ty,
                ty,
                bind_count,
            } => {
                let j = json!({
                    "Name": name.as_ref(),
                    "Set": desc_bind.set(),
                    "Binding": desc_bind.bind(),
                    "DescriptorType": desc_ty2json(&desc_ty),
                    "Type": ty2json(&ty),
                    "Count": bind_count,
                });
                descs.push(j);
            }
            Variable::PushConstant{ name, ty } => {
                let j = json!({
                    "Name": name.as_ref(),
                    "Type": ty2json(&ty),
                });
                push_consts.push(j);
            }
            Variable::SpecConstant{ name, spec_id, ty } => {
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
                let value = match operand.value {
                    ConstantValue::Bool(x) => x.to_string(),
                    ConstantValue::S32(x) => x.to_string(),
                    ConstantValue::U32(x) => x.to_string(),
                    ConstantValue::F32(x) => x.to_string(),
                    _ => todo!(),
                };
                json!({
                    "Value": value,
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

    let spv = get_spirv_bianry(in_path, &args);
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
            if let Err(e) = writeln!(f, "{json}") {
                writeln!(stderr(), "{e}").unwrap();
                writeln!(stderr(), "cannot write to output file: {out_path}").unwrap();
                exit(-1);
            };
        } else {
            println!("{}", serde_json::to_string_pretty(&j).unwrap());
        }
    }
}
