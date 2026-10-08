# `game` (6/6) — dialogs, save/load I/O, line editor

## `dialog.rs` — dialog plumbing (`0x32E21`–`0x33039`)

Both dialogs reuse the shell's frame phases (vsync, input stub, menu
service, oval, sequencer) and swap the hotspot list for their own
`0xB05A`/`0xB098` lists; exits come from menu actions `jmp`ing out or,
for the main dialog, `[0x9CDA] & 2` (right button).

- `pump` (`cs:0x2FE7`) — run the ticker to completion under
  `BLINK_HOLD=0xFF`: `{vsync, blink, ≤3 ticker steps, seq, oval,
  mark_sel, FRAME++}` while `[0x91C2] ≠ 0`; tail clears hold and, mode
  0 only, resets the blink phase + reprints `0x77E3` (the `jmp 0x2D86`
  tail).
- `dlg_enter` (`cs:0x2E21`) — sound, pump, `Slot(0x1274)`, frame rect
  (`Rect(0x14,0x5A,8,0x66)`), install the 3-record `0xB05A`/`0xB05C`
  list (`dlg_list`), `Image(0x12B)`. `dlg_step` (`cs:0x2E60`) = one
  modal iteration (`vsync`, `0xA369`, menu, oval, seq, `KeyPop`);
  `true` on `[0x9CDA]&2`. `dlg_close` (`0x2E7A`) — frame,
  `Slot(0x1272)`, restore the `0xA0AA`/`0xA0AC`/`0x31` list
  (`main_list`).
- `confirm` (`cs:0x2F9B`) — title `0x78B8` + body `0x7C72` prints,
  2-record `0xB098`/`0xB09A` list, modal wait on `[0x91DA]` written by
  the `0x2F01`/`0x2F08`/`0x2F95` actions, then restore the dialog list.
- `snd_block` — the shared `0x8A5B`/`0x8A3B`/`Sound(0xA)` chirp;
  `install`/`dlg_text` (`cs:0x2F24`, print at `(0x2A,0xB1)`)/`frame_rect`
  (`cs:0x2FD7`) helpers.

## `dlg_io.rs` — save/load dialog actions (`0x32E94`–`0x32F9A`)

The filename buffer is `ds:0..0x20`; the input routine sanitises
`0x20` → NUL (terminate) and `0x5B` ('[') → `0x5C` ('\\').

- `load_game` (`cs:0x2E94`) — prompt `0x78E5`, `input_name`,
  `slot_mode(1)` (`[0x9CCA]` + `Slot(0x1278)` pre-load), `st.load(files,
  name)`; carry → `0x7929` error print + `None`. Success →
  `slot_mode(2)`, `PlanetPanel`, `0x7C50` print, then the `[0x91E5]`
  switch → `Loaded::{A,B,C,D}` (`jmp 0x685A`/`0x6847`/`0x6834`/`0x6821`).
- `save_game` (`cs:0x2ED2`) — prompt, input, `st.save`; `0x7C2E` ok /
  `0x7929` error.
- `input_name` (`cs:0x2F2D`) — `NAME_LEN=0`, `NAME_MAX=0x1E`, legend
  `0x78C3`, `Mouse(0x14,0x1E,0)` buttons-only capture, `LINE_MODE=1`
  around `line_input(0, 0x2B, 0x20)`, mask restore `0x1F`,
  `NAME_MAX=9`, then the `ds:0..0x20` sanitiser. `filename()` reads the
  ASCIIZ result.
- `act_yes`/`act_no`/`act_cancel` — `[0x91DA]` = `1`/`2`/`0xFF`;
  `act_stage` (`cs:0x2F91`) — the `0x844B` heap snapshot.

## `linein.rs` — `line_input` (`cs:0xE66E`, `0x2E66E`–`0x2E77C`)

The blocking filename editor: clears `BUTTONS`/`IN_DONE`/`IN_DIRTY`,
`key_flush` (`cs:[0x8B7C] = −1`), then loops — channel-A `KeyPop`, else
channel B (`menu::keys`; right-click answers `0x1C`); the `ds:0x7CBD`
keymap classifies each code: `out 0` exits (`IN_DONE=0xFF`), `1`
backspaces (blank, echo `Slot(0x1268,0x20)`, `NAME_LEN−1`), `2` pads
the field with spaces and exits, else appends while
`NAME_LEN < NAME_MAX` (`Slot(0x1268,ch)` echo). `IN_DIRTY` is the
key-held debounce — a processed key sets it, a non-matching poll clears
it. Idle iterations always `vsync`; when `LINE_MODE==0` they also run
`seq_step`/overlay/`tick_step`/mode-0 `[0x1278]` reload + `FRAME++` —
the filename caller brackets with `[0x9CC5]=1` so dialog waits are
vsync-only (the world pauses during text input).
