//! Embeds `hzd/datapack` into `hzd-survival`: the game writes that pack into a
//! world's `datapacks/` folder when the world opens, so HZD content ships
//! inside the build without a zip and without a copy step in the packaging
//! scripts (F00, `hzd/docs/DECISIONS.md`).
//!
//! Each file becomes one entry of [`FILES`] (path inside the pack, bytes), in
//! name order. Hidden files (a `.DS_Store`, an editor's swap file) are left
//! out: the game would read them as pack files.
use std::{
    env,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};

/// The pack's source folder, relative to this crate's manifest.
const PACK_DIR: &str = "../../hzd/datapack";

fn main() {
    let manifest = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let root = manifest.join(PACK_DIR);
    println!("cargo:rerun-if-changed={}", root.display());
    let mut files = Vec::new();
    collect(&root, &root, &mut files);
    files.sort();
    assert!(
        files.iter().any(|path| path == "pack.mcmeta"),
        "{}/pack.mcmeta is missing: the game ignores a data pack without it",
        PACK_DIR
    );
    let mut out = String::from(
        "/// Every file of `hzd/datapack`: its path inside the pack (`/`-separated) and its bytes,\n\
         /// in name order.\n\
         pub const FILES: &[(&str, &[u8])] = &[\n",
    );
    for path in &files {
        writeln!(
            out,
            "    ({path:?}, include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{PACK_DIR}/{path}\"))),"
        )
        .expect("writing to a String");
    }
    out.push_str("];\n");
    let target = PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR")).join("builtin_pack.rs");
    fs::write(&target, out).expect("writing the embedded pack table");
}

/// Push every pack file under `dir` into `out`, as its `/`-separated path
/// below `root`.
fn collect(root: &Path, dir: &Path, out: &mut Vec<String>) {
    let entries = fs::read_dir(dir).unwrap_or_else(|e| panic!("reading {}: {e}", dir.display()));
    for entry in entries.flatten() {
        let path = entry.path();
        let name = entry.file_name();
        if name.to_string_lossy().starts_with('.') {
            continue;
        }
        if path.is_dir() {
            collect(root, &path, out);
        } else {
            let relative = path
                .strip_prefix(root)
                .unwrap_or_else(|_| panic!("{} is outside {}", path.display(), root.display()));
            out.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}
