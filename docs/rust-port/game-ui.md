# `game` (4/6) — frame loop, shell, menu service, selection

## `frame.rs` — `frame_step` (`cs:0x395B`, `0x3395B`–`0x33A44`)

The galaxy-screen frame body (asm `jmp 0x395B`s until `[0x9CDA] & 2`
exits to the shell). Per call: `vsync` → conditional overlay
(`PANEL_ON && uimode==5` → `Slot(0x1266, 0x7D39)`, `cs:0x66F3`) →
`REDRAW` block (`Slot(0x1260)`/`Slot(0x125C)`, mouse-y clamp
`Mouse(8,0x90,0xBC)`, `select_type`, `Present(5)`) → `PANEL_REQ` →
`info_panel` → `ambient` (selected type's `+0x12` byte picks an
`(ax,cl)`/cadence — kind 7 every 32nd frame `0x26/0x0E`, kinds 3 and 4
every 64th `0x29/0x0D`/`0x0C/0x0C`, kind 6 `0x2A/0x0A` on frame 0 and
`0x21/0x13` on `0x80`; all through `Sfx`, gated `[0x91D3] != 0xFF`) →
`FRAME++` → `tick_step` → `seq_step` → `Native(0xA369)` input →
`menu::service` → `dirty_flags` (`cs:0x3C6D`: snapshot `DIRTY0/1` into
`0x9CBC`/`0x9CBE`, service **at most one** bit per frame — `0x9144&4` →
`0x3CAD`, `&0x8000` → `0x3CDB`, `0x9146&2` → `0x3CF6`, then clear the
bit) → `[0x9CDA]&2` → `exit` (clear `0x91EA`, `Mouse(8,0,0xBC)`, stash
`HOT_SEL` at `0xA81A`) → `Frame::Shell`, else `Frame::Run(tick)`.

## `shell.rs` — the UI shell (`cs:0x2CC5`, `0x32CC5`–`0x32D85`)

`shell_enter`: two identical sound-select blocks (`Chan(0x8A5B,0)`/
`Chan(0x8A3B,3)`/`Sound(0xA)`, `[0x91D3]`-gated) around the
`[0x91EA]`/`[0x91E8]` clears — the `pop ax` between them discards the
return address a menu action abandons. Then `setup`: slots
`0x1260`/`0x125C`, `Native(0x89B3)` CGA border, `Image(0)` backdrop,
`Present(0)`, `Slot(0x1272)`, `[0x928B] = [0x91A6]`, `Refresh`, hotspot
install (`[0x9CC8]=[0xA0AA]`, `[0x9194]=0xA0AC`, `[0x91A8]=0x31`,
`uimode=0`), `PlanetPanel`, `mark_sel`, `tick_day`, `Native(0xA3BC)`,
`Slot(0x1278)`, `Native(0xA36A)`.

`shell_step` — one shell frame (`cs:0x2D5B`–`0x2D84`): `vsync` →
`seq_step` → `Native(0xA369)` → `menu::service` → `tick_step` →
`mark_sel` → `row_click` → `blink_step` → `hover_check` → `ticker_step`
when `[0x91D1]==0` → `FRAME++`. The asm loops forever; exits are
menu-action `jmp`s.

## `menu.rs` — `menu::service` (`cs:0xA10F`, `0x2A10F`–`0x2A18D`)

Polled input then debounced hit-test over the `0x14`-byte hotspot list
`[0x9194]`/`[0x91A8]`. Hotspot record: `+0/+2/+4/+6` = inclusive
`x1,y1,x2,y2`; `+8` = pressed-state image index; `+0xC` = action `cs:`
pointer (`jmp [si+0xC]`); `+0x10..+0x13` = nav links (`0xFF` = stay).

- `keys` — `synth` first: in real-driver mode the latch codes
  `0x7E`/`0x7D`/`0xFE`/`0xFD` → `press` writes `[0x9CDA] = 1/2/0`
  (`0xA18E`). In `K` mode (`0xA1CA`): pending code `0x1C`/`0x01` →
  buttons `1`/`2`, breaks `0x81`/`0x9C` → `0`, arrows → `nav`
  (Tandy extras when `[0x9CC6]==1`), else `snap` — ease the cursor ⅛
  toward the selected hotspot's centre (`cur + ((tgt−cur)>>3 | 1)`,
  `y ≤ 0xBC`), `Native(0xA497)` on move.
- Service body — `DRAG&1` returns; left-button-up clears `ARMED` and
  returns; `ARMED` swallows repeat presses; else `hit_test` scans the
  list: on `x1≤cx≤x2 && y1≤cy≤y2` → `hit`: stash pressed image
  `[0x9188]` (+ `Image` draw when nonzero), `ARMED=1`, selection word to
  `[list−2]`, `dispatch_action(+0xC)`.

## `actions.rs` — `dispatch_action` (the `+0xC` table, `0x2A16C`)

| `cs:` | action |
|---|---|
| `0x2E21` | `dialog::dlg_enter` (save/load dialog) |
| `0x2E94` | `dlg_io::load_game` |
| `0x2ED2` | `dlg_io::save_game` |
| `0x2EEE` | `dialog::dlg_list` |
| `0x2F0F` | confirm reprint (list + title) |
| `0x2F01`/`0x2F08`/`0x2F95` | `act_yes`/`act_no`/`act_cancel` → `[0x91DA]` = `1`/`2`/`0xFF` |
| `0x2F0E` | no-op |
| `0x2F91` | `act_stage` (`0x844B` heap snapshot) |
| `0x2F9B` | `dialog::confirm` |
| `0x2D86` | blink-phase reset + `0x77E3` reprint |
| `0x305B`/`0x9DAB` | `init::act_newgame` |
| other | `Call::Native(t)` for the frontend |

## `selrec.rs` — record selection

- `mark_sel`/`unmark_sel` (`cs:0x83AA`/`0x8386`) — set/clear `+5` on
  record `[0x9164]` (re-armed every frame so the panel redraws).
- `select_rec(i)` (`cs:0x5E03`) — `SEL_IDX = i`, `REC_CUR`/`SEL_REC` =
  `base + i·0x3A`, mark, `PlanetPanel`, `sel_name`, `SEQ_DELAY = 0`.
- `sel_next` (`cs:0x5DD3`) — sound chirp, unmark, `[0x9164]+1` wrapping
  to 0 at `[0x91C6]` (8-bit compare), fall into `select_rec`.
- `row_click` (`cs:0x5D52`) — left-button inside the `[0xA9..0xB5)`
  strip maps `y` to a row (`(y−0x11) >> 3/2/1` by difficulty),
  bounds-checks `< [0x91C6]+1` (8-bit wrap), ignores same-row repeats,
  then unmark + `select_rec`.

## `select.rs` — `select_type` (`cs:0x3A47`)

Runs in the frame's `REDRAW` block: resets the sequencer
(`SEQ_DELAY`/`SEQ_PHASE`/`REDRAW`), indexes `ds:0x9B12 + [0x91ED]·0x30`
into `TYPE_REC` (asm multiply scratch `0x9CB6`/`0x9CB8` reproduced),
copies the type record's `+6` → `[0x91BE]`, sequence ptr `+0x18` →
`SEQ_PTR`, period `+0x1C` → `SEQ_PERIOD`, draws `Image(word[si])`, sets
`PANEL_ON` from `+0x1D` (+`Native(0x66DE)` when set), `Native(0x89C3)`,
frame `Image(0x56)`, caption `0x6C43` at `(0x12,0x98)`, difficulty
legend `0x6C15`/`0x6BBC`/`0x6B38` at `(0,0xB0)`.

## `panel.rs` — `info_panel` (`cs:0x3AE9`)

Clears `PANEL_REQ` and reprints the selected type record `[0x9154]`'s
stats; coordinates and which rows print depend on `DIFFICULTY`:
name `+0x14`/`+0x2C` strings, `Native` `0x3CAD`/`0x3CDB`/`0x3CF6`,
then rows — `+6` (y `0xB0`/`0xB7`/`0xB3` by diff 2/0/1), `+0x10`
(mode 2 only) and `+0xE` (mode ≠ 0), `+0xA` capacity (mode 2;
`0xFFFE` → `ds:0x7325` "none"), `+4` cost, `+0xC` stock (`0xFFFF` →
`ds:0x731C`), owned count via `count_type` (`cs:0x6632` scan of the
machine array), diff-2 `+0x13·+0xC` total cost (`0xFFFF` →
`ds:0x7509`), `+0x1E` (mode ≠ 0).

## `status.rs` — shell status prints

`tick_day` (`cs:0x6A8E`) — `[0x91BA]` at a uimode-keyed `(x,y)`
(0→`(0x35,0xB)`, 3→`(0x15,0x12)`, 4→`(0x1D,0x3A)`), a `0x2F` ('/')
glyph, then `[0x91BC]` — the `tick/day` readout. `sel_name`
(`cs:0x6AD2`, mode 0) — `+0xC == 1` prints the `0x71B9` "alien" string
at `(0x33,0x4A)`, else a `0x3E` ('>') marker at `(0x33,0x4A)` plus the
record name at `(0x34,0x4A)`.
