use std::env::args;
use std::path::Path;
use spirq::ReflectConfig;

fn build_spirv_binary<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
    use std::fs::File;
    use std::io::Read;
    let path = path.as_ref();
    if !path.is_file() {
        return None;
    }
    let mut buf = Vec::new();
    if let Ok(mut f) = File::open(&path) {
        if buf.len() & 3 != 0 {
            // Misaligned input.
            return None;
        }
        f.read_to_end(&mut buf).ok()?;
    }
    Some(buf)
}

fn main() {
    for arg in args().skip(1) {
        println!("[{}]", arg);

        let spv = build_spirv_binary(arg).unwrap();
        let entry_points = ReflectConfig::new()
            .spv(spv)
            .ref_all_rscs(true)
            .gen_unique_names(true)
            .reflect()
            .unwrap();
        println!("{:#?}", &entry_points);
    }
}
