# HZD baseline

Measured on the owner's phone (OnePlus 10 Pro, Snapdragon 8 Gen 1, Adreno 730, 120 Hz), with
the build the owner installs, default settings, at the point where the module started. Every
later module is compared with this table; a module may not make the same scene more than
~10 % slower without a written reason and a fix plan.

Status: **NOT_EXECUTED** — nothing here has been measured yet. No number below may be
invented or filled from a PC, an emulator or a different phone.

## How to measure

1. Install the APK of the `android` branch build (`android-dev.yml` artifact), run it once,
   open F3 (three fingers or the debug key).
2. `hzd/docs/testcards/F00.md` lists the four scenes and what to copy from F3 after each.
3. Export the performance recording from the pause menu (the file appears in the game's
   `performance/` folder) and send it together with the APK's commit hash.

## Scenes

| Scene | F3 FPS (avg / 1 % low) | Tick ms (avg / max) | Render distance | LOD distance | Notes |
|---|---|---|---|---|---|
| (a) 2 min walking in a forest | not measured | not measured | not measured | not measured | |
| (b) walking through a village | not measured | not measured | not measured | not measured | |
| (c) 1 min fast flight | not measured | not measured | not measured | not measured | |
| (d) 20 min mixed play | not measured | not measured | not measured | not measured | battery % before/after, phone warm? |

## Performance recordings

| Scene | File | Commit | Date |
|---|---|---|---|
| (a)–(d) | not measured | | |

## Heat and battery

- Battery: not measured.
- Phone temperature after 20 min: not measured.
- The fork runs cool on this phone (the owner's report); the baseline is the number that
  shows a regression, not a target to beat.
