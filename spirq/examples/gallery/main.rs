use spirq::ReflectConfig;
use std::path::Path;

fn main() {
    let spv = build_spirv_binary("assets/gallery.frag.spv").unwrap();
    let entry_points = ReflectConfig::new()
        .spv(spv)
        .ref_all_rscs(true)
        .reflect()
        .unwrap();
    println!("{:#?}", &entry_points);
}

fn build_spirv_binary<P: AsRef<Path>>(path: P) -> Option<Vec<u8>> {
    use std::ffi::OsStr;
    use std::fs::File;
    use std::io::Read;
    let path = path.as_ref();
    if !path.is_file() || path.extension() != Some(OsStr::new("spv")) {
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
