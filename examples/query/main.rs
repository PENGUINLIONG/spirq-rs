use std::collections::HashMap;
use spirq::SpirvBinary;
use spirq::Sym;
use log::info;
use std::path::Path;

fn main() {
    env_logger::init();

    let spvs = collect_spirv_binaries("assets/effects/uniform-pbr");
    info!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let entries = spvs["uniform-pbr.vert"].reflect().unwrap();
    info!("{:#?}", entries);
    let desc_res = entries[0].resolve_desc(Sym::new(".model_view")).unwrap();
    info!("push_constant[model_view]: {:?}", desc_res);
    let desc_res = entries[0].resolve_desc(Sym::new(".view_proj")).unwrap();
    info!("push_constant[view_proj]: {:?}", desc_res);

    let entries = spvs["uniform-pbr.frag"].reflect().unwrap();
    info!("{:#?}", entries);
    let desc_res = entries[0].resolve_desc(Sym::new("mat.fdsa.1")).unwrap();
    info!("mat.fdsa.1: {:?}", desc_res);
    let desc_res = entries[0].resolve_desc(Sym::new("someImage")).unwrap();
    info!("someImage: {:?}", desc_res);
    let desc_res = entries[0].resolve_desc(Sym::new("imgggg")).unwrap();
    info!("imgggg: {:?}", desc_res);
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
