use clap::Parser;
use spirq_spvasm::{Disassembler, SpirvBinary};
use std::{
    fs::File,
    io::{stderr, Read, Write},
    path::Path,
    process::exit,
};

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

    #[arg(long, help = "Don't indent instructions.")]
    no_indent: bool,

    #[arg(long, help = "Don't output the header as leading comments.")]
    no_header: bool,

    #[arg(long, help = "Show raw Id values instead of friendly names.")]
    raw_id: bool,
}

fn main() {
    let args = Args::parse();

    let mut in_file: Box<dyn Read> = if let Some(in_path) = &args.in_path {
        let in_path = Path::new(in_path);
        let in_file = File::open(in_path).unwrap_or_else(|e| {
            writeln!(stderr(), "error: failed to open input file: {}", e).unwrap();
            exit(1);
        });
        let in_file: Box<dyn Read> = Box::new(in_file);
        in_file
    } else {
        let stdin = std::io::stdin();
        let in_file = Box::new(stdin);
        in_file
    };

    let mut spv = Vec::new();
    in_file.read_to_end(&mut spv).unwrap_or_else(|e| {
        writeln!(stderr(), "error: failed to read input file: {}", e).unwrap();
        exit(1);
    });

    let dis = Disassembler::new()
        .print_header(!args.no_header)
        .indent(!args.no_indent)
        .name_ids(!args.raw_id)
        .name_type_ids(!args.raw_id)
        .name_const_ids(!args.raw_id);
    let spvasm = dis
        .disassemble(&SpirvBinary::from(spv))
        .unwrap_or_else(|e| {
            writeln!(stderr(), "error: failed to read input file: {}", e).unwrap();
            exit(1);
        });

    if let Some(out_path) = args.out_path {
        let out_path = Path::new(&out_path).to_owned();
        let mut out_file = File::create(out_path).unwrap_or_else(|e| {
            writeln!(stderr(), "error: failed to open output file: {}", e).unwrap();
            exit(1);
        });
        out_file
            .write_all(&spvasm.into_bytes())
            .unwrap_or_else(|e| {
                writeln!(stderr(), "error: failed to write output file: {}", e).unwrap();
                exit(1);
            });
    } else {
        println!("{}", spvasm);
    };
}
