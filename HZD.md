# HZD

HZD is this fork's own game: a grounded, DayZ-like survival game on top of the Horizon
engine (Minecraft: Java Edition 26.3 in Rust), for one owner's phone. The engine, the
Android packaging and the vanilla parity work stay as they are; everything HZD adds lives
in `hzd/` and `crates/hzd-*`.

Start here:

- [`hzd/PROGRESS.md`](hzd/PROGRESS.md) — the modules (F00–F13), what is done, what is next.
- [`hzd/docs/DECISIONS.md`](hzd/docs/DECISIONS.md) — every significant decision, its reason
  and the alternatives; the upstream commit the fork froze at.
- [`hzd/docs/BASELINE.md`](hzd/docs/BASELINE.md) — the phone's measured baseline.
- [`hzd/docs/CONSOLE.md`](hzd/docs/CONSOLE.md) — `/hzd` commands and the console on a phone.
- [`hzd/docs/CI.md`](hzd/docs/CI.md) — the two workflows, what each proves, and what neither does.
- [`hzd/docs/HZD_DIVERGED.md`](hzd/docs/HZD_DIVERGED.md) — every file outside `hzd/` that
  HZD edits, and why.
- [`hzd/docs/testcards/`](hzd/docs/testcards/) — what to try on the phone for each module.

Two rules that matter more than any other:

- **Data first.** A feature starts as a data pack, then a resource pack, then a content mod,
  then new code in `crates/hzd-*`, and only then an edit inside existing engine code — which
  is marked `// HZD:` and listed in `HZD_DIVERGED.md`.
- **Server-authoritative.** Survival state is computed on the server thread; the client only
  draws and sends input. Single-player already runs through that thread.

The build is cloud-only: GitHub Actions builds the APK (`android-dev.yml` on `android`,
`hzd-check.yml` on `hzd/**` branches). Nothing here is built on the owner's PC.
