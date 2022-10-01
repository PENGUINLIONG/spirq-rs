use spirq::ReflectConfig;
use std::collections::BTreeMap;
use std::path::Path;

fn main() {
    let spvs = collect_spirv_binaries("assets");

    println!(
        "collected spirvs: {:?}",
        spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>()
    );
    let entry_points = ReflectConfig::new()
        .spv(spvs.get("spirv-spec.frag").unwrap() as &[u8])
        .ref_all_rscs(true)
        .reflect()
        .unwrap();
    println!("{:?}", entry_points);
    for var in entry_points[0].vars.iter() {
        println!("{:?}", var);
        for route in var.walk() {
            println!("- {:?}", route);
        }
    }
}

fn collect_spirv_binaries<P: AsRef<Path>>(path: P) -> BTreeMap<String, Vec<u8>> {
    use std::ffi::OsStr;
    use std::fs::{read_dir, File};
    use std::io::Read;

    read_dir(path)
        .unwrap()
        .filter_map(|x| match x {
            Ok(rv) => Some(rv.path()),
            Err(err) => {
                panic!("cannot access to filesystem item: {}", err);
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
            let name = x
                .file_stem()
                .and_then(OsStr::to_str)
                .map(ToOwned::to_owned)
                .unwrap();
            Some((name, buf))
        })
        .collect::<BTreeMap<_, _>>()
}
