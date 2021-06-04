use std::collections::HashMap;
use spirq::SpirvBinary;
use std::path::Path;

fn main() {
    let spvs = collect_spirv_binaries("assets/effects/ray-tracing");
    println!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    for (name, spv) in spvs {
        let module = spv.reflect_vec().unwrap();
        println!("{}: {:#?}", name, &module[0]);
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
