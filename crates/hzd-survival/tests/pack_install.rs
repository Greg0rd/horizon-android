//! The built-in data pack on disk: what a world gets, when nothing is rewritten and when
//! the owner's "off" wins. The folder is `CARGO_TARGET_TMPDIR`, which cargo keeps inside the
//! build directory, so a test never touches a real world or the system temp directory.
use std::{
    fs,
    path::{Path, PathBuf},
};

use hzd_survival::{FILES, packs};

/// A fresh folder for one test, under the build directory.
struct World(PathBuf);

impl World {
    fn new(tag: &str) -> Self {
        let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join(tag);
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("creating the test's world folder");
        Self(root)
    }

    fn path(&self) -> &Path {
        &self.0
    }

    fn pack(&self) -> PathBuf {
        packs::pack_folder(&self.0)
    }
}

impl Drop for World {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn on_disk(dir: &Path, prefix: &str, out: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let name = match path.strip_prefix(dir) {
            Ok(relative) => format!("{prefix}{}", relative.to_string_lossy().replace('\\', "/")),
            Err(_) => continue,
        };
        if path.is_dir() {
            on_disk(&path, &format!("{name}/"), out);
        } else {
            out.push(name);
        }
    }
}

#[test]
fn a_new_world_gets_every_file_of_the_pack_and_nothing_else() {
    let world = World::new("created");
    assert_eq!(
        packs::install_builtin_datapack(world.path()).expect("installing the pack"),
        packs::Installed::Created
    );
    for (path, bytes) in FILES {
        let installed = world.pack().join(path);
        assert_eq!(
            fs::read(&installed).unwrap_or_else(|e| panic!("reading {}: {e}", installed.display())),
            *bytes,
            "{} differs from the embedded pack",
            installed.display()
        );
    }
    let mut files = Vec::new();
    on_disk(&world.pack(), "", &mut files);
    let mut expected: Vec<String> = FILES.iter().map(|(path, _)| (*path).to_owned()).collect();
    files.sort();
    expected.sort();
    assert_eq!(
        files, expected,
        "the world's pack folder holds files the build does not"
    );
}

#[test]
fn an_up_to_date_world_is_not_rewritten() {
    let world = World::new("unchanged");
    packs::install_builtin_datapack(world.path()).expect("first install");
    let meta = world.pack().join("pack.mcmeta");
    let before = fs::metadata(&meta).and_then(|m| m.modified()).ok();
    assert_eq!(
        packs::install_builtin_datapack(world.path()).expect("second install"),
        packs::Installed::Unchanged
    );
    assert_eq!(
        fs::metadata(&meta).and_then(|m| m.modified()).ok(),
        before,
        "an unchanged pack is not touched, so its files keep their timestamps"
    );
}

#[test]
fn an_old_worlds_stale_pack_is_replaced_file_by_file() {
    let world = World::new("updated");
    fs::create_dir_all(world.pack()).expect("creating the pack folder");
    fs::write(world.pack().join("pack.mcmeta"), b"{\"pack\":{}}").expect("writing a stale pack");
    assert_eq!(
        packs::install_builtin_datapack(world.path()).expect("installing the pack"),
        packs::Installed::Updated
    );
    let meta = FILES
        .iter()
        .find(|(path, _)| *path == "pack.mcmeta")
        .map(|(_, bytes)| *bytes)
        .expect("pack.mcmeta is part of the pack");
    assert_eq!(
        fs::read(world.pack().join("pack.mcmeta")).expect("reading the pack"),
        meta
    );
}

#[test]
fn a_world_that_turned_the_pack_off_keeps_it_off() {
    let world = World::new("disabled");
    fs::write(
        world.path().join("datapacks.json"),
        br#"{"enabled":["vanilla"],"disabled":["file/hzd"]}"#,
    )
    .expect("writing the world's pack list");
    assert_eq!(
        packs::install_builtin_datapack(world.path()).expect("installing the pack"),
        packs::Installed::Disabled
    );
    assert!(
        !world.pack().exists(),
        "a pack the owner disabled is not put back on disk"
    );
}

#[test]
fn a_world_with_other_packs_still_gets_hzd() {
    let world = World::new("other-packs");
    fs::write(
        world.path().join("datapacks.json"),
        br#"{"enabled":["vanilla","file/somebody-elses"],"disabled":[]}"#,
    )
    .expect("writing the world's pack list");
    assert_eq!(
        packs::install_builtin_datapack(world.path()).expect("installing the pack"),
        packs::Installed::Created
    );
    assert!(world.pack().join("pack.mcmeta").is_file());
}
