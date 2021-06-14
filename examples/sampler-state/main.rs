use std::collections::BTreeMap;
use spirq::SpirvBinary;
use std::path::Path;

fn main() {
    let spvs = collect_spirv_binaries("assets/effects/hlsl");
    println!("collected spirvs: {:?}", spvs.iter().map(|x| x.0.as_ref()).collect::<Vec<&str>>());
    let frag = spvs["sampler-state.frag.hlsl"].reflect_vec().unwrap();
    println!("{:#?}", frag);
    let frag = &frag[0];
    let check = |sym :&str| {
        let desc_res = frag.resolve_desc(sym).unwrap();
        println!("{}: {:?}", sym, desc_res);
    };
    check("SampleType");
    check("shaderTexture");
}


fn collect_spirv_binaries<P: AsRef<Path>>(path: P) -> BTreeMap<String, SpirvBinary> {
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
        .collect::<BTreeMap<_, _>>()
}
