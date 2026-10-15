# `game` (6/6) — dialogs, save/load I/O, line editor

Page 6 of the `game` module docs — the modal dialogs: the save/load
dialog and its actions, the yes/no confirm, and the blocking filename
line editor. These are the routines that *take over the UI* — they swap
the hotspot list for their own and run their own frame loop until
dismissed. See [game-ui.md](game-ui.md) for the shell/menu machinery
they reuse.

"Modal" here means the same thing it did in 1991: while a dialog is up,
the shell's own hotspot list is swapped out and the dialog's takes its
place — so the only click targets that exist are the dialog's. When the
dialog ends, `dlg_close` puts the shell's list back.

## `dialog.rs` — dialog plumbing (`0x32E21`–`0x33039`)

Both dialogs reuse the shell's frame phases (vsync, input stub, menu
service, oval, sequencer) and swap the hotspot list for their own
`0xB05A`/`0xB098` lists; exits come from menu actions `jmp`ing out or,
for the main dialog, `[0x9CDA] & 2` (right button).

- `pump` (`cs:0x2FE7`) — run the ticker to completion under
  `BLINK_HOLD=0xFF`: `{vsync, blink, ≤3 ticker steps, seq, oval,
  mark_sel, FRAME++}` while `[0x91C2] ≠ 0`; tail clears hold and, mode
  0 only, resets the blink phase + reprints `0x77E3` (the `jmp 0x2D86`
  tail) — so a dialog can't open mid-scroll: queued messages finish
  first. The `≤3 ticker steps` bound keeps `pump` O(1) per call — the
  loop drains the queue over several frames rather than blocking.
- `dlg_enter` (`cs:0x2E21`) — sound, pump, `Slot(0x1274)`, frame rect
  (`Rect(0x14,0x5A,8,0x66)`), install the 3-record `0xB05A`/`0xB05C`
  list (`dlg_list`), `Image(0x12B)`. `dlg_step` (`cs:0x2E60`) = one
  modal iteration (`vsync`, `0xA369`, menu, oval, seq, `KeyPop`);
  `true` on `[0x9CDA]&2`. `dlg_close` (`0x2E7A`) — frame,
  `Slot(0x1272)`, restore the `0xA0AA`/`0xA0AC`/`0x31` list
  (`main_list`) — restore is what hands the shell its hotspots back.
- `confirm` (`cs:0x2F9B`) — title `0x78B8` + body `0x7C72` prints,
  2-record `0xB098`/`0xB09A` list, modal wait on `[0x91DA]` written by
  the `0x2F01`/`0x2F08`/`0x2F95` actions, then restore the dialog list —
  the cell is the *result channel*: `1` yes / `2` no / `0xFF` cancel.
  The modal wait is how a "dialog returns a value" works without a
  return stack: the action writes the answer cell, the dialog polls it.
- `snd_block` — the shared `0x8A5B`/`0x8A3B`/`Sound(0xA)` chirp;
  `install`/`dlg_text` (`cs:0x2F24`, print at `(0x2A,0xB1)`)/`frame_rect`
  (`cs:0x2FD7`) helpers.

## `dlg_io.rs` — save/load dialog actions (`0x32E94`–`0x32F9A`)

The filename buffer is `ds:0..0x20`; the input routine sanitises
`0x20` → NUL (terminate) and `0x5B` ('[') → `0x5C` ('\\') — the keymap
can't produce a backslash, so '[' stands in. (DOS filenames don't need
one here — the files are flat names — but the editor keeps the mapping
because its keymap is generic.)

- `load_game` (`cs:0x2E94`) — prompt `0x78E5`, `input_name`,
  `slot_mode(1)` (`[0x9CCA]` + `Slot(0x1278)` pre-load), `st.load(files,
  name)`; carry → `0x7929` error print + `None`. Success →
  `slot_mode(2)`, `ppanel::planet_panel`, `0x7C50` print, then the `[0x91E5]`
  switch → `Loaded::{A,B,C,D}` (`jmp 0x685A`/`0x6847`/`0x6834`/`0x6821`) —
  the saved game itself records which screen to return to. That last
  detail is why loading mid-galaxy returns you mid-galaxy: the save
  block carries `0x91E5`, and `load_game` honours it.
- `save_game` (`cs:0x2ED2`) — prompt, input, `st.save`; `0x7C2E` ok /
  `0x7929` error.
- `input_name` (`cs:0x2F2D`) — `NAME_LEN=0`, `NAME_MAX=0x1E`, legend
  `0x78C3`, `Mouse(0x14,0x1E,0)` buttons-only capture, `LINE_MODE=1`
  around `line_input(0, 0x2B, 0x20)`, mask restore `0x1F`,
  `NAME_MAX=9`, then the `ds:0..0x20` sanitiser. `filename()` reads the
  ASCIIZ result. `LINE_MODE=1` is the "don't tick the world" flag —
  see `linein.rs` below.
- `act_yes`/`act_no`/`act_cancel` — `[0x91DA]` = `1`/`2`/`0xFF`;
  `act_stage` (`cs:0x2F91`) — the `0x844B` heap restore (loads the
  checkpoint `stage_save` snapshotted at `new_game` time — "restart
  scenario").

## `linein.rs` — `line_input` (`cs:0xE66E`, `0x2E66E`–`0x2E77C`)

The blocking filename editor: clears `BUTTONS`/`IN_DONE`/`IN_DIRTY`,
`key_flush` (`cs:[0x8B7C] = −1`), then loops — channel-A `KeyPop`, else
channel B (`menu::keys`; right-click answers `0x1C`); the `ds:0x7CBD`
keymap classifies each code: `out 0` exits (`IN_DONE=0xFF`), `1`
backspaces (blank, echo `Slot(0x1268,0x20)`, `NAME_LEN−1`), `2` pads
the field with spaces and exits, else appends while
`NAME_LEN < NAME_MAX` (`Slot(0x1268,ch)` echo). `IN_DIRTY` is the
key-held debounce — a processed key sets it, a non-matching poll clears
it, so a held key doesn't auto-repeat: each make produces one
character.

Idle iterations always `vsync`; when `LINE_MODE==0` they also run
`seq_step`/overlay/`tick_step`/mode-0 `[0x1278]` reload + `FRAME++` —
the filename caller brackets with `[0x9CC5]=1` so dialog waits are
vsync-only (**the world pauses during text input** — no ticks run).
This is why a name prompt feels instantaneous in the original: the
whole galaxy stops simulating while you type.

## `namedit.rs` — the serial-numbered editors

Two wrappers around `line_input` that prefill the field from a shipped
template: `cs:0xE77D` (the `0x9136` name template — prints it, copies
its 9 bytes into the edit target, *bumps the template's two-digit
serial before the edit*, then runs the editor bracketed in
`LINE_MODE=1`); `cs:0xE7CD` — the same over the `[0x9154]` type record's
`+0x22` name, bumping the serial only on accept. `cs:0xE82D` is the
shared serial-bump helper. The pre-bump is the quirk: the asm
increments "UNIT 07" → "UNIT 08" in the template *before* editing, so
the *next* launch offers the next number whether or not this one was
accepted.

## Pitfalls

- **Dialogs run *inside* the frame loop, not instead of it.** A test
  that calls `dlg_step` expecting the shell to be paused will find the
  ticker, seq and oval still run — only the hotspot list changed.
- **`[0x91DA]` is the answer channel.** There is no return value — poll
  the cell after the modal wait, don't expect `confirm` to hand you
  one.
- **Filename sanitisation is bidirectional.** `[` → `\` on the way in,
  and `0x20` NUL-terminates — a filename typed with a space ends there.
- **`LINE_MODE` gates world updates.** `line_input` only runs
  `tick_step`/`seq_step` when `[0x9CC5] == 0`; the save prompt sets it,
  so the galaxy won't advance while the user types.

## File map

| file | item |
|---|---|
| `dialog.rs` | `pump`/`dlg_enter`/`dlg_step`/`dlg_close`/`confirm` + `snd_block`/`install`/`dlg_text`/`frame_rect` |
| `dlg_io.rs` | `load_game`/`save_game`/`input_name`/`act_yes`/`act_no`/`act_cancel`/`act_stage` |
| `linein.rs` | `line_input` — the `cs:0xE66E` blocking editor |
| `namedit.rs` | `prompt_edit`/`type_edit`/`serial_bump` — the `0xE77D`/`0xE7CD` wrappers |

## Worked example — saving a game

1. `dlg_enter` swaps in the 3-record `0xB05A` list.
2. Hotspot → `save_game` (`0x2ED2`): prints prompt `0x78E5`, runs
   `input_name` — `LINE_MODE=1` brackets `line_input` so no ticks run.
3. User types "TEST"; each key goes through `line_input`'s keymap
   classify → append while `NAME_LEN < NAME_MAX`.
4. Accept → `st.save(files, "TEST")` → the `0x1F8B` block write.
5. `0x7C2E` ("saved") prints; `dlg_close` restores the shell list.
