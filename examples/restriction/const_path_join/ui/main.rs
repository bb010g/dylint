// run-rustfix

fn main() {
    let _ = std::path::Path::new("..").join("target");
    let _ = std::path::PathBuf::from("..").join("target");
    let _ = std::path::PathBuf::from("..").join("target").as_path();
    let _ = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("target");
}
