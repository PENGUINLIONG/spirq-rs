use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::ops::Deref;
use spirq::{Result, Error, SpirvBinary, Manifest, ExecutionModel};
use spirq::EntryPoint;
use std::path::Path;

#[derive(Clone, Default)]
pub struct Pipeline {
    pub manifest: Manifest,
}
impl TryFrom<&[EntryPoint]> for Pipeline {
    type Error = Error;

    fn try_from(entry_points: &[EntryPoint]) -> Result<Pipeline> {
        let mut found_stages = HashSet::<ExecutionModel>::default();
        let mut manifest = Manifest::default();
        for entry_point in entry_points.as_ref().iter() {
            if found_stages.insert(entry_point.exec_model) {
                manifest.merge(&entry_point.manifest)?;
            } else {
                // Reject stage collision.
                panic!("pipeline cannot have two stages of the same execution model");
            }
        }
        return Ok(Pipeline { manifest: manifest });
    }
}
impl Deref for Pipeline {
    type Target = Manifest;
    fn deref(&self) -> &Self::Target { &self.manifest }
}

fn main() {
    let spvs = collect_spirv_binaries("assets/effects/uniform-pbr");
    println!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let mut entry_points = spvs.values()
        .map(|x| x.reflect_vec().unwrap()[0].to_owned())
        .collect::<Vec<_>>();
    entry_points.sort_by_key(|x| x.exec_model as u32);
    let pl = Pipeline::try_from(entry_points.as_ref()).unwrap();

    let pcheck = |sym :&str| {
        let push_const_res = pl.resolve_push_const(sym).unwrap();
        println!("{}: {:?}", sym, push_const_res.member_var_res);
    };
    let check = |sym :&str| {
        let desc_res = pl.resolve_desc(sym).unwrap();
        println!("{}: {:?}", sym, desc_res.member_var_res);
    };

    pcheck(".model_view");
    pcheck(".view_proj");
    check("mat.fdsa.1");
    check("someImage");
    check("imgggg");

    println!("-- buffer sizing:");
    for desc_res in pl.descs() {
        println!("{:?}: nbyte={:?}", desc_res.desc_bind, desc_res.desc_ty.nbyte());
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
