use clap::Parser;
use spirq_spvasm::{SpirvBinary, Disassembler};
use std::{
    fs::File,
    io::{stderr, Write, Read},
    path::Path,
    process::exit,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "Input SPIR-V file path.")]
    in_path: String,

    #[arg(
        short,
        long,
        help = "Output SPIR-V assembly file path. The output is printed to \
        stdout if this path is not given."
    )]
    out_path: Option<String>,

    #[arg(
        long,
        help = "Don't output the header as leading comments."
    )]
    no_header: bool,

    #[arg(
        long,
        help = "Show raw Id values instead of friendly names."
    )]
    raw_id: bool,
}

fn main() {
    let args = Args::parse();

    let in_path = Path::new(&args.in_path);

    let mut in_file = File::open(in_path).unwrap_or_else(|e| {
        writeln!(stderr(), "error: failed to open input file: {}", e).unwrap();
        exit(1);
    });

    let mut spv = Vec::new();
    in_file.read_to_end(&mut spv)
        .unwrap_or_else(|e| {
            writeln!(stderr(), "error: failed to read input file: {}", e).unwrap();
            exit(1);
        });

    let dis = Disassembler::new()
        .print_header(!args.no_header)
        .name_ids(!args.raw_id)
        .name_type_ids(!args.raw_id)
        .name_const_ids(!args.raw_id);
    let mut spvasm = dis.disassemble(&SpirvBinary::from(spv)).unwrap_or_else(|e| {
        writeln!(stderr(), "error: failed to read input file: {}", e).unwrap();
        exit(1);
    });
    spvasm.push('\n');

    if let Some(out_path) = args.out_path {
        let out_path = Path::new(&out_path).to_owned();
        let mut out_file = File::create(out_path)
            .unwrap_or_else(|e| {
                writeln!(stderr(), "error: failed to open output file: {}", e).unwrap();
                exit(1);
            });
        out_file.write_all(&spvasm.into_bytes())
            .unwrap_or_else(|e| {
                writeln!(stderr(), "error: failed to write output file: {}", e).unwrap();
                exit(1);
            });
    } else {
        println!("{}", spvasm);
    };
}
