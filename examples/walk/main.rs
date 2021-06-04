use std::collections::HashMap;
use spirq::SpirvBinary;
use std::path::Path;

fn main() {
    let spvs = collect_spirv_binaries("assets/effects/spirv-spec");
    println!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let frag = spvs["referential.frag"].reflect_vec().unwrap();
    let frag = &frag[0];
    for push_const in frag.get_push_const().iter() {
        println!("push_const:");
        for route in push_const.walk() {
            println!("{:>4} {:>20}: {:?}", route.offset, route.sym, route.ty);
        }
    }
    for spec_const in frag.spec.spec_consts() {
        let name = frag.spec.get_spec_const_name(spec_const.spec_id).unwrap_or("unnamed");
        println!("spec_const#{} ({}): {:?}", spec_const.spec_id, name, spec_const.ty);
    }
    for input in frag.inputs() {
        let name = frag.get_input_name(input.location).unwrap_or("unnamed");
        println!("input#{} ({}): {:?}", input.location, name, input.ty);
    }
    for output in frag.outputs() {
        let name = frag.get_output_name(output.location).unwrap_or("unnamed");
        println!("output#{} ({}): {:?}", output.location, name, output.ty);
    }
    for desc in frag.descs() {
        let name = frag.get_desc_name(desc.desc_bind).unwrap_or("unnamed");
        println!("descriptor{} ({}):", desc.desc_bind, name);
        for route in desc.desc_ty.walk() {
            println!("{:>4} {:>20}: {:?}", route.offset, route.sym, route.ty);
        }
    }
}


fn collect_spirv_binaries<P: AsRef<Path>>(path: P) -> HashMap<String, SpirvBinary> {
    use std::ffi::OsStr;
    use std::fs::{read_dir, File};
    use std::io::Read;

    read_dir(path).unwrap()
        .filter_map(|x| match x {
            Ok(rv) => Some(rv.path()),
            Err(err) => {
                panic!("cannot access to filesystem item: {}", err);
            },
        })
        .filter_map(|x| {
            let mut buf = Vec::new();
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
