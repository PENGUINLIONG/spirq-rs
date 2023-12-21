use clap::Parser;
use spirq_spvasm::{Assembler, SpirvHeader};
use std::{
    borrow::Borrow,
    fs::File,
    io::{stderr, Read, Write},
    path::Path,
    process::exit,
};

const SPIRV_VERSION_1_0: u32 = 0x0001_0000;
const SPIRV_VERSION_1_1: u32 = 0x0001_0100;
const SPIRV_VERSION_1_2: u32 = 0x0001_0200;
const SPIRV_VERSION_1_3: u32 = 0x0001_0300;
const SPIRV_VERSION_1_4: u32 = 0x0001_0400;
const SPIRV_VERSION_1_5: u32 = 0x0001_0500;
const SPIRV_VERSION_1_6: u32 = 0x0001_0600;

// TODO: (penguinliong) Get ourselves a generator ID.
const GENERATOR: u32 = 0;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Input SPIR-V assembly file path.")]
    in_path: String,

    #[arg(
        short,
        long,
        help = "Output SPIR-V file path. The output file is defaulted to \
        {in_path}.spv if this path is not given."
    )]
    out_path: Option<String>,

    #[arg(
        long,
        help = "{vulkan1.1spv1.4|vulkan1.0|vulkan1.1|vulkan1.2|vulkan1.3 \
        |spv1.0|spv1.1|spv1.2|spv1.3|spv1.4|spv1.5|spv1.6} \
        Use specified environment."
    )]
    target_env: Option<String>,
}

fn main() {
    let args = Args::parse();

    let in_path = Path::new(&args.in_path);
    let out_path = if let Some(out_path) = args.out_path {
        Path::new(&out_path).to_owned()
    } else {
        Path::new(&format!("{}.spv", args.in_path)).to_owned()
    };

    let mut in_file = File::open(in_path).unwrap_or_else(|e| {
        writeln!(stderr(), "error: failed to open input file: {}", e).unwrap();
        exit(1);
    });

    let mut code = String::new();
    in_file.read_to_string(&mut code).unwrap_or_else(|e| {
        writeln!(stderr(), "error: failed to read input file: {}", e).unwrap();
        exit(1);
    });

    let header = match args.target_env.as_ref().map(Borrow::borrow) {
        Some("vulken1.1spv1.4") => SpirvHeader::new(SPIRV_VERSION_1_4, GENERATOR),
        Some("vulkan1.0") => SpirvHeader::new(SPIRV_VERSION_1_0, GENERATOR),
        Some("vulkan1.1") => SpirvHeader::new(SPIRV_VERSION_1_1, GENERATOR),
        Some("vulkan1.2") => SpirvHeader::new(SPIRV_VERSION_1_2, GENERATOR),
        Some("vulkan1.3") => SpirvHeader::new(SPIRV_VERSION_1_3, GENERATOR),
        Some("spv1.0") => SpirvHeader::new(SPIRV_VERSION_1_0, GENERATOR),
        Some("spv1.1") => SpirvHeader::new(SPIRV_VERSION_1_1, GENERATOR),
        Some("spv1.2") => SpirvHeader::new(SPIRV_VERSION_1_2, GENERATOR),
        Some("spv1.3") => SpirvHeader::new(SPIRV_VERSION_1_3, GENERATOR),
        Some("spv1.4") => SpirvHeader::new(SPIRV_VERSION_1_4, GENERATOR),
        Some("spv1.5") => SpirvHeader::new(SPIRV_VERSION_1_5, GENERATOR),
        Some("spv1.6") => SpirvHeader::new(SPIRV_VERSION_1_6, GENERATOR),
        None => SpirvHeader::new(SPIRV_VERSION_1_4, GENERATOR),
        _ => {
            writeln!(
                stderr(),
                "error: unknown target environment: {}",
                args.target_env.unwrap()
            )
            .unwrap();
            exit(1);
        }
    };

    let spv = Assembler::new()
        .assemble(&code, header)
        .unwrap_or_else(|e| {
            writeln!(stderr(), "error: failed to read input file: {}", e).unwrap();
            exit(1);
        });

    let mut out_file = File::create(out_path).unwrap_or_else(|e| {
        writeln!(stderr(), "error: failed to open output file: {}", e).unwrap();
        exit(1);
    });

    out_file.write_all(&spv.into_bytes()).unwrap_or_else(|e| {
        writeln!(stderr(), "error: failed to write output file: {}", e).unwrap();
        exit(1);
    });
}
