# HZD decisions

Newest last. Each entry: date, decision, reason, alternatives, and what it costs to undo.

## 2026-11-04 — upstream frozen at `6eca278f`

- **Decision:** the fork stops merging Horizon upstream at `android` @ `6eca278f`
  (`6eca278fd92e99ade83ae5823d090cb4bdc6d046`, the branch's head when F00 started).
- **Reason:** HZD diverges from vanilla on purpose (no Nether/End, no villagers, no fantasy
  blocks); every upstream merge would re-import what F01 removes and re-run parity work we
  do not want.
- **Alternative:** periodic merges of security-only upstream patches, rejected: nothing here
  is exposed to the network beyond the LAN server the fork already ships.
- **Undo:** delete this entry and re-add `upstream` as a remote.

## 2026-11-04 — branch per module, merged into `android` when green

- **Decision:** each module works on `hzd/<module>`; it merges (fast-forward where possible)
  into `android` when its CI is green, so `android` always builds an APK.
- **Reason:** `android-dev.yml` only runs on `android`; the owner installs from that
  workflow's artifact. A module branch keeps `android` buildable while a module is half done.
- **Alternative:** working directly on `android`, rejected: a half-finished module would ship
  a broken APK.
- **Undo:** none needed.

## 2026-11-04 — the built-in data pack ships embedded in `hzd-survival`

- **Decision:** `hzd/datapack` is compiled into `crates/hzd-survival` by its `build.rs`
  (`include_bytes!` per file) and written into `<world>/datapacks/hzd` each time a local
  world opens (`hzd_survival::packs::install_builtin_datapack`, called from
  `Game::open_world`).
- **Reason:** the APK's asset pipeline is engine-owned (`tools/android_build.py`,
  `.local/android/gradle-assets`) and HZD must not fork it; embedding keeps the pack versioned
  with the code that needs it, works offline, and needs no zip step. Writing it on world open
  gives **existing** worlds the pack too, which a create-time-only hook would not.
- **Why not a zip in the world folder:** one more unpack step and a second copy of the data.
- **Alternatives:** a copy step in `tools/android_build.py` (rejected: engine-owned script,
  breaks if upstream moves it); a `.hmod` content mod (rejected for F00: entity/item support
  is unproven — F08's question — and worldgen data belongs in a data pack).
- **The owner keeps the last word:** a world whose `datapacks.json` lists `file/hzd` under
  `disabled` is left alone (`Installed::Disabled`), so turning the pack off is permanent.
- **Undo:** remove the crate's `build.rs` walk and the `// HZD:` hook in `open_world`.

## 2026-11-04 — display name `HZD`, package id unchanged

- **Decision:** `packaging/android/AndroidManifest.xml` labels the app `HZD`; the package id
  stays `dev.horizon.android.dev` (set by `android_build.py`'s `-PappId`).
- **Reason:** the owner already installs the dev-channel APK and wants his worlds to survive
  updates; a new package id would install beside it with empty storage.
- **Alternative:** a separate `dev.hzd.android` id (an own app, installed beside the fork) —
  viable later for a parallel install; it costs the existing saves.
- **Undo:** restore `android:label="Horizon Android"`.

## 2026-11-04 — no resource pack auto-enable in F00

- **Decision:** F00 delivers the data pack only. Resource packs stay the owner's choice until
  F04 (the art pack) exists.
- **Reason:** resource packs are a global option (`options.txt`'s `resourcePacks`), not a
  world setting; forcing one on would overwrite the owner's list and is invisible until there
  is art to show. The pack list, the loader and the built-in `vanilla` pack are already there
  (`crates/horizon-client-render/src/packs.rs`, `startup_root`), so F04 can add a built-in
  entry and a default without new plumbing.
- **Undo:** none yet.

## 2026-11-04 — F3 and the log carry the HZD version

- **Decision:** `hzd_survival::VERSION` is logged next to the build banner
  (`crates/horizon-client/src/main.rs`) and drawn in the F3 overlay's `system` group
  (`game/debug_keys.rs`); `hzd_survival::perf::lines()` (empty today) is printed into the
  same overlay.
- **Reason:** every later module can be identified from one screenshot plus one log; the
  performance rule (a module may not cost more than ~10 % without a reason) needs a place to
  report our own counters, and this is it.
- **Undo:** remove the two hooks.

## 2026-11-04 — the trunk's APK workflow is repaired inside F00

- **Decision:** F00 fixes what kept the APK red, even though it is tooling rather than HZD
  gameplay: `tools/android_prepare.py` falls back to the pinned 26.3 version JSON, the
  workflow installs a JDK 17 for Gradle, `crates/horizon-android` depends on `hzd-survival`,
  and the APK build keeps its log (`build.log` -> run summary + artifact).
- **Reason:** the module's definition of done needs a green APK build, and without one the
  owner cannot run any test card. Each cause was read from the job log first (they were three
  different failures in a row), and the last green build predates all of them.
- **Alternative:** report the red trunk and stop, rejected: it would have left F00 unverified
  for an unrelated reason.
- **Undo:** revert those five edits; nothing in them changes gameplay, and each is listed in
  `HZD_DIVERGED.md`.