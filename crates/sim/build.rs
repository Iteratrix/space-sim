//! Bundles every storylet under `data/storylets` into the crate.
use std::fmt::Write as _;
use std::path::Path;

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../data/storylets");
    println!("cargo:rerun-if-changed={}", dir.display());
    let mut files: Vec<_> = std::fs::read_dir(&dir)
        .map(|rd| rd.filter_map(Result::ok).map(|e| e.path()).collect())
        .unwrap_or_default();
    files.retain(|p| p.extension().is_some_and(|x| x == "toml"));
    files.sort();
    let mut out = String::from("[\n");
    for path in files {
        println!("cargo:rerun-if-changed={}", path.display());
        let name = path.file_name().unwrap().to_string_lossy();
        let abs = path.canonicalize().unwrap();
        writeln!(out, "    ({name:?}, include_str!({:?})),", abs.display()).unwrap();
    }
    out.push(']');
    let dest = Path::new(&std::env::var("OUT_DIR").unwrap()).join("bundled_storylets.rs");
    std::fs::write(dest, out).unwrap();
}
