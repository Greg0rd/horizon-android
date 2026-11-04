# HZD's CI

Two workflows build this fork. Nothing else does: the owner's PC is not a build machine.

| Workflow | When | What it proves | Time |
|---|---|---|---|
| `.github/workflows/hzd-check.yml` | push to `hzd/**`, PRs touching `hzd/`, `crates/hzd-*`, `Cargo.*`, `rust-toolchain.toml` | HZD's crate formats, lints (`-D warnings`) and passes its tests; the tree holds no Mojang files; `horizon-client` still compiles with the `// HZD:` hooks in place | minutes |
| `.github/workflows/android-dev.yml` | push to `android` touching code, packaging or tools | the arm64 dev APK builds, is signed and passes the release audit; the artifact is what the owner installs | up to ~2.5 h |

A module is done when both are green (the fast check on its branch, the APK after the merge into
`android`) and the owner has run the module's test card.

## What the fast check does not prove

- **It does not build the APK.** Android-only code (`crates/horizon-android`, the packaging, the
  NDK target) is compiled only by `android-dev.yml`, and only after a merge into `android`.
- **It does not run the game.** There is no headless game in CI: no world is opened, no block is
  broken, no datapack is loaded. Anything that only shows at runtime (a world getting the pack,
  the console on touch, an F3 line) is the owner's test card, labelled IMPLEMENTED BUT NOT
  EXECUTED until he reports it.
- **It does not run the parity harnesses.** The harness runner needs the prepared 26.3
  resources in `.local/`, which CI does not produce for `hzd-check`. When a change touches
  engine gameplay, name the harnesses in `HZD_DIVERGED.md` and run them through
  `android-dev.yml`'s environment or locally.
- **The engine job's cache hides how much was rebuilt.** Its `cargo check -p horizon-client`
  can finish in a couple of minutes on a warm cache; that is still a real compile of the
  touched crates, but it is not a full build from scratch.
- **It says nothing about performance or heat.** Those need the phone (`BASELINE.md`).

## Reading a run

The fast check has two jobs: *HZD layer · fmt, clippy, tests* (the crate and the public-tree
check) and *Engine builds with the HZD hooks* (`cargo check -p horizon-client`). A red
*engine* job with a green *layer* job almost always means a `// HZD:` hook stopped compiling
and needs the file's real API, not the crate.
