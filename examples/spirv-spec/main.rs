use std::collections::HashMap;
use spirq::SpirvBinary;
use spirq::sym::Sym;
use log::info;
use std::path::Path;

fn main() {
    env_logger::init();

    let spvs = collect_spirv_binaries("assets/effects/spirv-spec");
    info!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let entries = spvs["referential.frag"].reflect().unwrap();
    info!("{:#?}", entries);

    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0")).unwrap();
    info!("0.0: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0.s")).unwrap();
    info!("0.0.s: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0.cond")).unwrap();
    info!("0.0.cond: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0.s.b")).unwrap();
    info!("0.0.s.b: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0.s.v")).unwrap();
    info!("0.0.s.v: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0.s.v.0")).unwrap();
    info!("0.0.s.v.0: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0.s.v.1")).unwrap();
    info!("0.0.s.v.1: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0.s.v.2")).unwrap();
    info!("0.0.s.v.2: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0.s.v.3")).unwrap();
    info!("0.0.s.v.3: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0.s.v.4")).unwrap();
    info!("0.0.s.v.4: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = entries[0].resolve_desc(Sym::new("0.0.s.i")).unwrap();
    info!("0.0.s.i: offset={:?}, ty={:?}", offset, var_ty);
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
