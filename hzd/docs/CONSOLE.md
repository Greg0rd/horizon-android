# HZD console

The console is the test channel for everything HZD builds. On the phone it is the chat
screen; on the PC it is the same chat screen or `horizon-ctl command "…"`.

## Vanilla commands that exist today

Nothing is missing for F00: `/time set`, `/tp`, `/give`, `/summon`, `/weather`, `/gamemode`,
`/difficulty`, `/datapack list`, `/function`, `/seed`, `/save` and the rest of the fork's
tree (`crates/horizon-command`, dispatch in `crates/horizon-client/src/game/commands.rs`).

HZD's own proof that the built-in pack is loaded and enabled:

```
/datapack list                 # lists "file/hzd" among the enabled packs
/function hzd:ping             # the pack's own function, answers in chat
```

## `/hzd` commands

None yet. F00 adds the layer, not gameplay; the first `/hzd` subcommands arrive with the
systems they test (F05 vitals, F07 spawns, F10 events). They are registered in the command
tree (`crates/horizon-command/src/tree_*.rs`) and dispatched with the other game commands,
documented here when they land, and they mark the world `cheated` like vanilla's own
`/gamemode` (off in Hardcore).

## Typing on the phone (the thing F00 has to be sure about)

The path exists in the fork and F00 changes nothing in it:

- touch: the chat screen opens the Android input method, and `ime_edit.rs`
  (`use crate::osk::Output`) feeds the IME's edits into the focused field;
- controllers and keyboards: `osk.rs` (the game's own on-screen keyboard) and
  `game/pad_menu.rs` (`pad_osk_output`) hand the same `Output`s to the field, and
  `touch_ui.rs` draws it;
- `android_input.rs` turns the platform's key events into the same typed input;
- send is Enter, which both the IME's action button and `osk::Output::Enter` reach.

What is **not** proven: that the soft keyboard opens reliably on the owner's OnePlus with the
game's `adjustResize` window mode, and that the suggestion bar does not eat the Enter key.
That is phone work, not code reading — `testcards/F00.md`, step 4.
