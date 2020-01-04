use log::info;
use spirq::SpirvBinary;
use std::collections::HashMap;
use std::path::Path;

fn main() {
    env_logger::init();

    let spvs = collect_spirv_binaries("assets/effects/uniform-pbr");

    info!(
        "collected spirvs: {:?}",
        spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>()
    );
    let vert = spvs["uniform-pbr.vert"].reflect().unwrap();
    let vert = &vert[0];
    info!("{:#?}", vert);

    let check_vert = |sym: &str| {
        let desc_res = vert.resolve_desc(sym).unwrap();
        info!("{}: {:?}", sym, desc_res);
    };
    check_vert(".model_view");
    check_vert(".view_proj");

    let frag = spvs["uniform-pbr.frag"].reflect().unwrap();
    let frag = &frag[0];
    info!("{:#?}", frag);
    let check_frag = |sym: &str| {
        let desc_res = frag.resolve_desc(sym).unwrap();
        info!("{}: {:?}", sym, desc_res);
    };
    check_frag("mat.fdsa.1");
    check_frag("someImage");
    check_frag("imgggg");
}

fn collect_spirv_binaries<P: AsRef<Path>>(path: P) -> HashMap<String, SpirvBinary> {
    use log::warn;
    use std::ffi::OsStr;
    use std::fs::{read_dir, File};
    use std::io::Read;

    read_dir(path)
        .unwrap()
        .filter_map(|x| match x {
            Ok(rv) => Some(rv.path()),
            Err(err) => {
                warn!("cannot access to filesystem item: {}", err);
                None
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
            let spv = buf.into();
            let name = x
                .file_stem()
                .and_then(OsStr::to_str)
                .map(ToOwned::to_owned)
                .unwrap();
            Some((name, spv))
        })
        .collect::<HashMap<_, _>>()
}
