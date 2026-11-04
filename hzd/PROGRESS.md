# HZD progress

Modules run in order; a module starts when the one before it is done (CI green, checked on
the phone). Status labels: **VERIFIED** (seen running, CI green or the owner's phone result),
**IMPLEMENTED NOT EXECUTED**, **NOT IMPLEMENTED**.

| Module | Scope | Status | Report |
|---|---|---|---|
| F00 | own layer, fast CI, baseline, built-in data pack | IMPLEMENTED NOT EXECUTED | `hzd/docs/reports/F00.md` |
| F01 | trim the game (dimensions, mobs, structures, fantasy blocks, PC settings) | NOT IMPLEMENTED | — |
| F02 | realistic landscape and biomes, fields, half blocks | NOT IMPLEMENTED | — |
| F03 | villages, towns, farms, roads | NOT IMPLEMENTED | — |
| F04 | 16×16 art pack drawn for HZD | NOT IMPLEMENTED | — |
| F05 | survival: vitals, weight, items, loot, fire, crafting | NOT IMPLEMENTED | — |
| F06 | atmosphere: grey look, dark nights, silence, sound, effects instead of HUD | NOT IMPLEMENTED | — |
| F07 | zombies (7 kinds), hearing, sight, spawning | NOT IMPLEMENTED | — |
| F08 | animals: deer, boar, wolves, bears, cattle, hunting | NOT IMPLEMENTED | — |
| F09 | combat: melee, firearms, ballistics, bows | NOT IMPLEMENTED | — |
| F10 | base, locks, garden, fishing, events (helicopter, infection) | NOT IMPLEMENTED | — |
| F11 | JSON models, leg IK, animation, first person, appearance, falling trees | NOT IMPLEMENTED | — |
| F12 | vehicles | NOT IMPLEMENTED | — |
| F13 | multiplayer, saves, performance, heat, release | NOT IMPLEMENTED | — |

## F00 — baseline, own layer, fast CI

- [x] `hzd/` tree, `HZD.md`, `AGENTS.md` pointer, `crates/hzd-survival` in the workspace,
      `hzd-check.yml`, docs and the test card.
- [x] Built-in data pack: `hzd/datapack` is embedded into `hzd-survival` at build time and
      written into `<world>/datapacks/hzd` every time a local world opens, so new **and**
      existing worlds get it; the game's pack list enables it unless the owner turned it off.
- [x] Version banner in the log, `HZD <version>` in F3, an empty counters hook for later modules.
- [x] App label is `HZD` (same package id, so updates install in place).
- [x] Fast check green on `hzd/f00` (run `41854339195`); `android` fast-forwarded to the
      module (run `41855903124` for the docs-only follow-up).
- [x] APK workflow green (run `41885982882`, 17 min) after fixing three pre-existing causes:
  Mojang's republished 26.3 metadata, a missing JDK 17 for Gradle, and `horizon-android`
  missing HZD's crate; the banner fix landed in run `41891754748`.
- [x] Console on the phone with the soft keyboard (VERIFIED: the Android IME opened and a typed command ran).
- [x] Built-in pack on a new world and surviving an app update (VERIFIED on the phone; see the report).
- [x] F3 shows `HZD 0.1.0` on the phone, and the log says `HZD layer active (hzd-survival 0.1.0)`.
- [ ] Baseline numbers from the phone (`BASELINE.md`) — the only F00 item left.

The phone ran this build on 2026-11-04 over adb (details and evidence in
`hzd/docs/reports/F00.md`, §4). The working copy this session started from had no `.git`;
the repository metadata was restored from `origin/android` (the tree matched `6eca278`
exactly) and the module was committed, pushed and merged like any other branch
(`hzd/docs/reports/F00.md`, "How to land this").
