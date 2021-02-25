use std::collections::HashMap;
use spirq::SpirvBinary;
use log::info;
use std::path::Path;
use spirv_headers::Op;
use num_traits::FromPrimitive;

fn main() {
    env_logger::init();

    let spvs = collect_spirv_binaries("assets/effects/uniform-pbr");

    info!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let mut cur_func_name = String::new();
    let mut nfunc = 0;
    let mut nload = 0;
    let mut nstore = 0;
    let _vert = spvs["uniform-pbr.vert"]
        .reflect_vec_inspect(|itm, instr| {
            match Op::from_u32(instr.opcode()).unwrap() {
                Op::Function => {
                    let mut operands = instr.operands();
                    let _ty_id = operands.read_u32().unwrap();
                    let func_id = operands.read_u32().unwrap();
                    cur_func_name = itm.get_name(func_id).unwrap().to_owned();
                    info!("entered function {}", cur_func_name);
                    nfunc += 1;
                },
                Op::Load => {
                    info!("found a load instruction");
                    nload += 1;
                },
                Op::Store => {
                    info!("found a store instruction");
                    nstore += 1;
                },
                Op::FunctionEnd => {
                    info!("left function {}", cur_func_name);
                    cur_func_name = String::new();
                }
                _ => {},
            }
        })
        .unwrap();

    info!("{} load instructions and {} store instructions in {} functions of this shader file", nload, nstore, nfunc);
}


fn collect_spirv_binaries<P: AsRef<Path>>(path: P) -> HashMap<String, SpirvBinary> {
    use std::ffi::OsStr;
    use std::fs::{read_dir, File};
    use std::io::Read;
    use log::warn;

    read_dir(path).unwrap()
        .filter_map(|x| match x {
            Ok(rv) => Some(rv.path()),
            Err(err) => {
                warn!("cannot access to filesystem item: {}", err);
                None
            },
        })
        .filter_map(|x| {
            let mut buf = Vec::new();
            dbg!(&x);
            if !x.is_file() ||
                x.extension() != Some(OsStr::new("spv")) ||
                File::open(&x).and_then(|mut x| x.read_to_end(&mut buf)).is_err() ||
                buf.len() & 3 != 0 {
                return None;
            }
            let spv = buf.into();
            let name = x.file_stem()
                .and_then(OsStr::to_str)
                .map(ToOwned::to_owned)
                .unwrap();
            Some((name, spv))
        })
        .collect::<HashMap<_, _>>()
}
