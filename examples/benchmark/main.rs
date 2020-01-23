use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use spirq::{ExecutionModel, Error, EntryPoint, SpirvBinary, Manifest, Result};
use std::ops::Deref;
use log::info;
use std::path::Path;

macro_rules! bench {
    ($task:expr, $inner:block) => {
        {
            const NREPEAT: u128 = 100;
            let tic = Instant::now();
            let mut i = 0;
            let x = loop {
                i += 1;
                let x = $inner;
                if (i == NREPEAT) { break x; }
            };
            let toc = Instant::now();
            let dur = toc.duration_since(tic).as_nanos();
            let avg_dur = dur / NREPEAT;
            info!("{} took {:.3}us ({} times avg)", $task, avg_dur as f64 / 1000., NREPEAT);
            x
        }
    }
}


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
    use std::time::Instant;

    env_logger::init();

    let spvs = collect_spirv_binaries("assets/effects/uniform-pbr");
    info!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let (vert, frag) = (&spvs["uniform-pbr.vert"], &spvs["uniform-pbr.frag"]);
    let (vert, frag) = bench!("reflection", {
        (vert.reflect().unwrap(), frag.reflect().unwrap())
    });
    let pipe = &[vert[0].to_owned(), frag[0].to_owned()];
    let pipe = bench!("merging manifests", {
        Pipeline::try_from(pipe as &[EntryPoint]).unwrap()
    });
    let pipe = &*pipe;

    bench!("enumerating input names", {
        for input in pipe.inputs() {
            let _name = pipe.get_input_name(input.location);
        }
    });
    bench!("enumerating output names", {
        for output in pipe.outputs() {
            let _name = pipe.get_output_name(output.location);
        }
    });
    bench!("walking descriptors", {
        for desc in pipe.descs() {
            for _route in desc.desc_ty.walk() {
            }
        }
    });
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
