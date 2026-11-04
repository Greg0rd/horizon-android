# Where HZD diverges from the fork

Every edit outside `hzd/` is listed here: the file, what changed and why. New code in
`crates/hzd-*` and everything under `hzd/` is HZD's own and is not listed. Parity harnesses
and vanilla tests are never deleted; harnesses a divergence makes obsolete are listed at the
bottom.

## Engine files edited (all marked `// HZD:` or `<!-- HZD: -->` in place)

| File | Change | Module |
|---|---|---|
| `Cargo.toml` | `crates/hzd-survival` added to the workspace members | F00 |
| `crates/horizon-client/Cargo.toml` | depends on `hzd-survival` | F00 |
| `crates/horizon-client/src/main.rs` | logs `HZD layer active (hzd-survival <version>)` after the build banner | F00 |
| `crates/horizon-client/src/game.rs` (`Game::open_world`) | writes the built-in data pack into the world before it opens | F00 |
| `crates/horizon-client/src/game/debug_keys.rs` (`debug_system_lines`, `debug_performance_lines` area) | F3 `system` group shows the HZD version and `hzd_survival::perf::lines()` | F00 |
| `packaging/android/AndroidManifest.xml` | `android:label="HZD"` (package id unchanged) | F00 |
| `tools/android_prepare.py` (`download_game`) | when Mojang has republished the 26.3 version JSON, fetch the pinned one from its content address instead of refusing to prepare (see below) | F00 |
| `.github/workflows/android-dev.yml` (and its export `packaging/android/dev-builds.yml`) | the APK build's output goes through `tee build.log`, the log tail lands in the run summary and the file is uploaded on failure; a JDK 17 is installed for the Gradle step, because Gradle 8.14 cannot run on the JDK 25 the Mojang exporters need and the image no longer ships the `JAVA_HOME_17_X64` alias the workflow passed | F00 |
| `AGENTS.md` | one line pointing at `HZD.md` | F00 |
| `.github/workflows/hzd-check.yml` | new workflow (not an edit of an existing one) | F00 |

## Behaviour that differs from the fork

- Every local world gets `datapacks/hzd` on open and has it enabled, unless its
  `datapacks.json` disables `file/hzd`. The pack currently holds one function,
  `hzd:ping`; later modules fill it.
- The app's launcher name is `HZD`.
- The log and the F3 overlay name the HZD layer's version.
- `tools/android_prepare.py` no longer stops when Mojang republishes a release's version JSON
  (a new asset index, new translations): it downloads the copy the lock pins, by its hash, as
  `horizon-prepare`'s own error message instructs. The jars and libraries it then verifies are
  the pinned ones; nothing about the pin's meaning changes. This unblocks the APK workflow,
  which failed at that step on `6eca278` too.

## Parity harnesses made obsolete

None yet. F00 changes no vanilla behaviour that a harness pins: the data pack is additive
(a `file/` pack a world's selection does not mention is enabled), the banner is log text, and
F3's own group is outside vanilla's entries.
