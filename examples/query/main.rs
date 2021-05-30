use std::collections::HashMap;
use spirq::SpirvBinary;
use log::info;
use std::path::Path;

fn main() {
    env_logger::init();

    let spvs = collect_spirv_binaries("assets/effects/uniform-pbr");

    info!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let vert = spvs["uniform-pbr.vert"].reflect_vec().unwrap();
    let vert = &vert[0];
    info!("{:#?}", vert);

    let check_vert = |sym :&str| {
        let push_const_res = vert.resolve_push_const(sym).unwrap();
        info!("{}: {:?}", sym, push_const_res);
    };
    check_vert(".model_view");
    check_vert(".view_proj");

    let frag = spvs["uniform-pbr.frag"].reflect_vec().unwrap();
    let frag = &frag[0];
    info!("{:#?}", frag);
    let check_frag = |sym :&str| {
        dbg!(sym);
        let desc_res = frag.resolve_desc(sym).expect("failed to resolve desc");
        info!("{}: {:?}", sym, desc_res);
    };
    check_frag("hahayes");
    check_frag("hahano");
    //check_frag("mat.fdsa");
    check_frag("someImage");
    check_frag("imgggg");

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
