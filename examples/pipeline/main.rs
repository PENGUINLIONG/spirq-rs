use std::collections::HashMap;
use std::convert::TryFrom;
use spirq::SpirvBinary;
use spirq::reflect::Pipeline;
use spirq::sym::Sym;
use log::info;
use std::path::Path;

fn main() {
    env_logger::init();

    let spvs = collect_spirv_binaries("assets/effects/uniform-pbr");
    info!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let entry_points = spvs.values()
        .map(|x| x.reflect().unwrap()[0].to_owned())
        .collect::<Vec<_>>();
    let pl = Pipeline::try_from(entry_points.as_ref()).unwrap();
    info!("{:#?}", pl);
    let (offset, var_ty) = pl.resolve_desc(Sym::new(".model_view")).unwrap();
    info!("push_constant[model_view]: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = pl.resolve_desc(Sym::new(".view_proj")).unwrap();
    info!("push_constant[view_proj]: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = pl.resolve_desc(Sym::new("mat.fdsa.1")).unwrap();
    info!("mat.fdsa.1: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = pl.resolve_desc(Sym::new("someImage")).unwrap();
    info!("someImage: offset={:?}, ty={:?}", offset, var_ty);
    let (offset, var_ty) = pl.resolve_desc(Sym::new("imgggg")).unwrap();
    info!("imgggg: offset={:?}, ty={:?}", offset, var_ty);

    info!("-- buffer sizing:");
    for (desc_bind, desc_ty) in pl.desc_binds() {
        use spirq::reflect::DescriptorType::*;
        let struct_ty = match desc_ty {
            PushConstant(struct_ty) => &struct_ty,
            Block(iblock_ty) => &iblock_ty.block_ty,
            _ => continue,
        };
        info!("{:?}: nbyte={:?}", desc_bind, struct_ty.nbyte());
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
