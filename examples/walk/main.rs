use std::collections::HashMap;
use spirq::SpirvBinary;
use log::info;
use std::path::Path;

fn main() {
    env_logger::init();

    let spvs = collect_spirv_binaries("assets/effects/spirv-spec");
    info!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let frag = spvs["referential.frag"].reflect().unwrap();
    let frag = &frag[0];
    for input in frag.inputs() {
        info!("input {:?}: {:?}", input.location, input.ty);
    }
    for output in frag.outputs() {
        info!("output {:?}: {:?}", output.location, output.ty);
    }
    for desc in frag.descs() {
        info!("descriptor {}:", desc.desc_bind);
        for route in desc.desc_ty.walk() {
            info!("{:>4} {:>10}: {:?}", route.offset, route.sym, route.ty);
        }
    }
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
