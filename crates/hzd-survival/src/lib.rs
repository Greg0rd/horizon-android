//! HZD's own layer on the fork's engine: everything the game adds to Horizon
//! lives behind this crate's small, testable API, so the game loop keeps only
//! tiny `// HZD:` hooks (`HZD.md`, `hzd/docs/DECISIONS.md`).
//!
//! F00 ships two things here: the built-in data pack's delivery into a world
//! ([`packs`]) and the version the log banner and the F3 overlay show
//! ([`VERSION`]).
#![forbid(unsafe_code)]

/// The HZD layer's version, as the log banner and the F3 overlay print it.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// The folder name of HZD's built-in data pack inside a world's `datapacks/`.
pub const PACK_FOLDER: &str = "hzd";

/// The pack's id in the game's pack list (`file/<folder>`, `datapacks.json`).
pub const PACK_ID: &str = "file/hzd";

include!(concat!(env!("OUT_DIR"), "/builtin_pack.rs"));

/// HZD's built-in data pack: what the build ships, and how a world gets it.
pub mod packs {
    use std::{
        fs, io,
        path::{Path, PathBuf},
    };

    use crate::{FILES, PACK_FOLDER, PACK_ID};

    /// What installing the built-in pack into a world did.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Installed {
        /// The world had no HZD pack yet; it now holds every file.
        Created,
        /// The folder was there (an older build's pack) and a file was written.
        Updated,
        /// Every file was already present with its current contents.
        Unchanged,
        /// The world's `datapacks.json` turns HZD's pack off: the owner's choice wins.
        Disabled,
    }

    /// Write HZD's built-in data pack into `world`'s `datapacks/hzd` folder.
    ///
    /// The game's pack list enables every `file/` pack that a world's
    /// `datapacks.json` does not mention, so a world that never heard of HZD
    /// gets the pack on its next open, while one that disables it keeps it off
    /// ([`Installed::Disabled`]). Worlds from an older build are upgraded file
    /// by file, and an unchanged pack is not rewritten.
    pub fn install_builtin_datapack(world: &Path) -> io::Result<Installed> {
        if world_disables_pack(world) {
            return Ok(Installed::Disabled);
        }
        let folder = pack_folder(world);
        let existed = folder.is_dir();
        let mut wrote = false;
        for (path, bytes) in FILES {
            let target = folder.join(path);
            if fs::read(&target).is_ok_and(|old| old == *bytes) {
                continue;
            }
            if let Some(parent) = target.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::write(&target, bytes)?;
            wrote = true;
        }
        Ok(match (existed, wrote) {
            (_, false) => Installed::Unchanged,
            (true, true) => Installed::Updated,
            (false, true) => Installed::Created,
        })
    }

    /// Where the built-in pack lives in `world`.
    pub fn pack_folder(world: &Path) -> PathBuf {
        world.join("datapacks").join(PACK_FOLDER)
    }

    /// Whether the world's `datapacks.json` lists HZD's pack as disabled.
    fn world_disables_pack(world: &Path) -> bool {
        let Ok(bytes) = fs::read(world.join("datapacks.json")) else {
            return false;
        };
        let Ok(json) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
            return false;
        };
        json["disabled"]
            .as_array()
            .is_some_and(|ids| ids.iter().any(|id| id.as_str() == Some(PACK_ID)))
    }
}

/// HZD's counters for the F3 overlay and the performance recording.
pub mod perf {
    /// The lines HZD's own systems add to the F3 overlay's "performance"
    /// report, one per counter.
    ///
    /// Empty in F00: the printing hook is in place
    /// (`crates/horizon-client/src/game/debug_keys.rs`), and the survival and
    /// world systems of later modules push their numbers here.
    pub fn lines() -> Vec<String> {
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path_of(wanted: &str) -> &'static [u8] {
        FILES
            .iter()
            .find(|(path, _)| *path == wanted)
            .map(|(_, bytes)| *bytes)
            .unwrap_or_else(|| panic!("{wanted} is not part of the built-in pack"))
    }

    #[test]
    fn the_pack_meta_declares_the_pinned_data_format() {
        let meta: serde_json::Value =
            serde_json::from_slice(path_of("pack.mcmeta")).expect("pack.mcmeta is JSON");
        let (major, _minor) = horizon_core::version::DATA_PACK_FORMAT;
        assert_eq!(meta["pack"]["min_format"], major);
        assert_eq!(meta["pack"]["max_format"], major);
        let description = meta["pack"]["description"].as_str().unwrap_or_default();
        assert!(
            !description.trim().is_empty(),
            "the pack needs a description the Data Packs screen can show"
        );
    }

    #[test]
    fn the_pack_holds_a_function_the_phone_can_call() {
        assert!(
            FILES.iter().any(|(path, bytes)| {
                *path == "data/hzd/function/ping.mcfunction" && !bytes.is_empty()
            }),
            "F00's phone check runs /function hzd:ping"
        );
    }
}
