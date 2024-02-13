use std::{io::{Read, stderr, Write}, path::Path, fs::File, process::exit, collections::HashMap};

use anyhow::{anyhow, bail, Result};
use spirq::{prelude::*, entry_point};
use clap::Parser;
use screen_13::prelude::*;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Input SPIR-V assembly file path. Or read from stdin if input \
        file path is not provided.")]
    in_path: Option<String>,

    #[arg(
        short,
        long,
        help = "Output SPIR-V assembly file path. The output is printed to \
        stdout if this path is not given."
    )]
    out_path: Option<String>,

    #[arg(long, help = "Specialization constant value.")]
    spec: Option<String>,

    #[arg(long, help = "Workgroup count in XYZ separated in comma. Missing dimensions are defaulted to 1. e.g. 1,2,3 or 4,5.")]
    workgroup_count: Option<String>,

    #[arg(long, help = "Entry point name", default_value_t = "main".to_string())]
    entry_point: String,

    #[arg(short = 'I', help = "Input buffer path.")]
    input_buffer_paths: Vec<String>,

    #[arg(short = 'O', help = "Output buffer path.")]
    output_buffer_paths: Vec<String>,

    #[arg(short = 'U', help = "Uniform value assignment. e.g. x=1")]
    uniform_values: Vec<String>,
}

fn guarded_main() -> Result<()> {
    let args = Args::parse();

    let mut in_file: Box<dyn Read> = if let Some(in_path) = &args.in_path {
        let in_path = Path::new(in_path);
        let in_file = File::open(in_path)
            .map_err(|e| anyhow!("failed to open input file: {}", e))?;
        let in_file: Box<dyn Read> = Box::new(in_file);
        in_file
    } else {
        let stdin = std::io::stdin();
        let in_file = Box::new(stdin);
        in_file
    };

    let mut spv = Vec::new();
    in_file.read_to_end(&mut spv)
        .map_err(|e| anyhow!("failed to read input file: {}", e))?;

    // Reflect to find out what resources are needed.
    let entries = ReflectConfig::new()
        .spv(SpirvBinary::from(spv))
        .combine_img_samplers(true)
        .reflect()
        .map_err(|e| anyhow!("failed to reflect input file: {}", e))?;

    // Find the compute shader entry point.
    let entry_point_name = args.entry_point.unwrap_or_else(|| "main".to_owned());
    let entry = entries.into_iter()
        .find(|entry| entry.name == entry_point_name)
        .ok_or_else(|| anyhow!("failed to find entry point: {}", entry_point_name))?;
    if entry.exec_model != spirq::spirv::ExecutionModel::GLCompute {
        bail!("entry point {} must be a compute shader", entry_point_name);
    }

    // Load the input buffers.
    let mut in_bufs = Vec::new();
    for path in args.input_buffer_paths {
        let path = Path::new(&path);
        let mut file = File::open(path)
            .map_err(|e| anyhow!("failed to open input buffer file: {}", e))?;
        let mut buf = Vec::new();
        file.read_to_end(&mut buf)
            .map_err(|e| anyhow!("failed to read input buffer file: {}", e))?;
        in_bufs.push(buf);
    }

    let mut uniform_buf: Option<> = None;
    let mut in_bufs = Vec::new();
    let mut out_bufs = Vec::new();
    for var in entry.vars {
        match var {
            Variable::Input { .. } => bail!("unexpected pipeline input"),
            Variable::Output { .. } => bail!("unexpected pipeline output"),
            Variable::Descriptor { desc_bind, desc_ty, ty, nbind, .. } => {
                match desc_ty {
                    DescriptorType::Sampler() => bail!("sampler is not supported"),
                    DescriptorType::CombinedImageSampler() => bail!("image is not supported"),
                    DescriptorType::SampledImage() => bail!("image is not supported"),
                    DescriptorType::StorageImage(_) => bail!("image is not supported"),
                    DescriptorType::UniformTexelBuffer() => bail!("image is not supported"),
                    DescriptorType::StorageTexelBuffer(_) => bail!("image is not supported"),
                    DescriptorType::UniformBuffer() => {
                        ty.walk()
                    },
                    DescriptorType::StorageBuffer(_) => {

                    },
                    DescriptorType::InputAttachment(_) => bail!("input attachment is not supported"),
                    DescriptorType::AccelStruct() => bail!("acceleration structure is not supported"),
                }
            },
            Variable::PushConstant { .. } => bail!("push constant is not supported"),
            Variable::SpecConstant { .. } => bail!("specialization constant is not supported"),
        }
    }


    Ok(())
}

fn main() {
    guarded_main().unwrap_or_else(|e| {
        writeln!(stderr(), "error: {}", e).unwrap();
        exit(1);
    });
}
