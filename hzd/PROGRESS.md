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
- [ ] CI green on the `hzd/f00` branch and the APK workflow green after the merge into `android`.
- [ ] Console on the phone with the soft keyboard (code path read, phone check pending).
- [ ] Baseline numbers from the phone (`BASELINE.md`).

Not done in this environment: the repository here arrived without `.git` and without
network access to GitHub from the workspace, so the branch, the push and the CI runs are the
owner's first step (`hzd/docs/reports/F00.md`, "How to land this").
