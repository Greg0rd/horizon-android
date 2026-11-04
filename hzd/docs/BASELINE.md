# HZD baseline

Measured on the owner's phone (OnePlus 10 Pro, Snapdragon 8 Gen 1, Adreno 730, 120 Hz), with
the build the owner installs, default settings, at the point where the module started. Every
later module is compared with this table; a module may not make the same scene more than
~10 % slower without a written reason and a fix plan.

Status: **WAIVED BY THE OWNER (2026-11-04)** — "пропустим замеры, всё хорошо". No numbers
were taken, so the table below stays empty; nothing here may be invented or filled from a PC,
an emulator or a different phone.

What that means from here on:

- The ~10 % rule cannot be checked numerically. A module that would slow the game down is
  caught by the owner playing it, and by the F3 numbers he reports when something *feels*
  wrong (the overlay and the `HZD` line are already on his phone).
- The first real numbers that arrive from the phone — from any module — go into this file and
  become the reference from that point on, marked with the build that produced them.
- What is already on record without a table: the fork's build runs at ~55–60 fps in a forest
  by the F3 overlay's smoothed rate (observed over adb while walking, run `41891754748`,
  2026-11-04, default settings, 2412×1080). That is an observation, not the protocol's
  average/1 %-low pair, and it is written here as such.

## How to measure (if the owner ever wants the table filled)

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
