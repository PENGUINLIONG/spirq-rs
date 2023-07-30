use spirq::{spirv::Op, ReflectConfig};
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let spvs = collect_spirv_binaries("assets");

    println!(
        "collected spirvs: {:?}",
        spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>()
    );
    let mut cur_func_name = String::new();
    let mut nfunc = 0;
    let mut nload = 0;
    let mut nstore = 0;
    ReflectConfig::new()
        .spv(spvs.get("spirv-spec.frag").unwrap() as &[u8])
        .ref_all_rscs(true)
        .reflect_inspect_by(|itm, instr| match instr.op() {
            Op::Function => {
                let mut operands = instr.operands();
                let _ty_id = operands.read_u32().unwrap();
                let func_id = operands.read_u32().unwrap();
                cur_func_name = itm.name_reg.get(func_id).unwrap().to_owned();
                println!("entered function {}", cur_func_name);
                nfunc += 1;
            }
            Op::Load => {
                println!("found a load instruction");
                nload += 1;
            }
            Op::Store => {
                println!("found a store instruction");
                nstore += 1;
            }
            Op::FunctionEnd => {
                println!("left function {}", cur_func_name);
                cur_func_name = String::new();
            }
            _ => {}
        })
        .unwrap();

    println!(
        "{} load instructions and {} store instructions in {} functions of this shader file",
        nload, nstore, nfunc
    );
}

fn collect_spirv_binaries<P: AsRef<Path>>(path: P) -> BTreeMap<String, Vec<u8>> {
    use std::ffi::OsStr;
    use std::fs::{read_dir, File};
    use std::io::Read;

    read_dir(path)
        .unwrap()
        .filter_map(|x| match x {
            Ok(rv) => Some(rv.path()),
            Err(err) => {
                panic!("cannot access to filesystem item: {}", err);
            }
        })
        .filter_map(|x| {
            let mut buf = Vec::new();
            if !x.is_file()
                || x.extension() != Some(OsStr::new("spv"))
                || File::open(&x)
                    .and_then(|mut x| x.read_to_end(&mut buf))
                    .is_err()
                || buf.len() & 3 != 0
            {
                return None;
            }
            let name = x
                .file_stem()
                .and_then(OsStr::to_str)
                .map(ToOwned::to_owned)
                .unwrap();
            Some((name, buf))
        })
        .collect::<BTreeMap<_, _>>()
}
