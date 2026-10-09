# `front` — the GAM-9 frontend

A concrete `VmHost` that replays the game's service calls against a real
framebuffer, palette and asset set, plus the input pump that mirrors
window events into the `ds:` latch cells.

## `font/` — the shipped 4×6 font

Replays each mode's `[0x1268]` put byte-for-byte out of `Host::img`:
the pointer tables and packed records ship in the image's data segment
(`ds:` = file `0x106A0 + ofs`, dgroup `0xFAA`), 60 glyphs covering
codes `0x20..=0x5B` with colours baked in — the puts take no ink
argument, so `Host::ink` only feeds the popup frame.

| file | put | table → records | draw |
|---|---|---|---|
| `mcg.rs` | `0x2EECE` | `ds:0x3009` → `ds:0x30F9` | opaque 4 B × 6 rows, `di += 0x13C` |
| `ega.rs` | `0x2FF33` | `ds:0x3081` → `ds:0x3699` | 6 B × 4 plane blocks (`+0x168`), nibble merge — high for even `x4`, low for odd |
| `cga.rs` | `0x30887` | `ds:0x2F19` → `ds:0x3C51` | opaque 2bpp byte per row, `0x2000`-bank interleave |
| `tga.rs` | `0x3133B` | `ds:0x2F91` → `ds:0x3DB9` | opaque 4bpp `movsw` per row, 4-bank `0x2000` step |

The asm reads its `es:di` row bases from shipped `u16` tables
(`ds:0x28D9`/`0x2A69`/`0x2BF9`/`0x2D89`) whose values equal
`Video::di_at`, so each mode computes rows through that. `mod.rs` owns
the `DS` mapping plus the shared `sub al,0x20` index/read helpers;
codes past `0x5B` draw the same table-bleed garbage as DOS (the index
keeps reading the next mode's bytes out of `img`), and only reads
landing past `img`'s end are skipped.

## `Pump` (`pump.rs`) — input mirror

Tracks host-side cursor/button/key state and writes it into the `ds:`
cells the ported menu service polls (`[0x9CD6]`/`[0x9CD8]` cursor,
`[0x9CCB]`/`[0x9CCC]` latch, `[0x9CCD]` K-mode flag).

- `set_buttons(left, right)` — edge-detects both buttons and queues the
  same synthetic codes the DOS driver's `0x2A18E` handler emits:
  `0x7E`/`0x7D` down, `0xFE`/`0xFD` up (`code` submodule).
- `key(scan)` — latches a make/break scancode into both channels: the
  `[0x9CCB]` queue and the `cs:0x8B72` channel-A LIFO (drop-on-full at
  10 like the asm). `pop_key()` = `0x1E2E` LIFO pop; `flush_keys()` =
  the `cs:[0x8B7C] = −1` reset.
- `set_range`/`move_to` — int-33h `ax=8` vertical clamp and `ax=4`
  position set; cursor clamped to `x≤319`, `y∈range∩≤199`.
- `set_handler(mask)` — int-33h `ax=0x14` event mask; bit 0 gates
  whether motion still mirrors into the `ds:` cursor cells.
- `write(vm, st)` — once per frame: mirrors cursor (if `motion`),
  K-mode flag, and drops at most one queued code into the latch, only
  when `[0x9CCB]` is free.

## `Host` (`host.rs`) — the `VmHost` impl

Owns `Screen`, `Palette`, `Pump`, `AssetSet`, the unpacked exe image and
a `dropped` counter for unmodelled calls. `cs:` reads map through
`CS_OFS = 0x30000` (game code) — `cs_word(ofs)` reads a LE word at
`img[0x30000+ofs]`.

`svc` dispatch:

- `Call::Image(i)` → `Screen::draw_image` (the `[0x125A]` slot) — the
  mode's own decoder: MCG linear, EGA four plane streams, CGA/TGA
  banked row advance (`video::{ega,banked}`).
- `Call::Glyph(ch, bx, dx)` → `font::draw` at `bx*4` px — all modes via
  `Screen::put_pixel` (the mode-layout pixel write in `video::put`).
- `Call::Rect(a,b,c,d)` → `[0x1262]` popup frame: opposing corners in
  4-px-col/row units, normalised, drawn as a 1-px outline in `ink`
  through `put_pixel` (approximation of `0x2EDCE` — the asm fills
  black; same outline in every mode).
- `Call::Mouse(ax,cx,dx)` — int-33h: `4` set position, `8` y range,
  `0x14` event mask (`0x1E` buttons-only vs `0x1F` full).
- `Call::KeyPop` → `pump.pop_key()`, `0` when empty.
- `Call::Slot(0x125C)` → clear `scr.buf` (the `0x2EB6F` `rep stosw`);
  `Call::Slot(0x1260)` → `dac0()`: program DAC reg 0 from the
  `cs:0x1384` template triplet (`TPL_OFS + 0x1384`, 6-bit → 8-bit) —
  MCGA only; the EGA/TGA slots are `int 10/AX=1002` palette-register
  writes and CGA's `AH=5` page select, all no-ops under the fixed
  `Palette::{ega16,cga}` model.
- `pump_input` → `pump.write`; `key_flush` → `pump.flush_keys`.
- Menu/Dialog/Refresh/Present/Flash/PlanetPanel/Blit/Screen/other
  `Slot`/`Native`/`Ui`/`Draw`/`Sound`/`Chan`/`Sfx` → `dropped += 1`
  (GAM-7/GAM-9 stubs — the window's per-frame blit supersedes the
  refresh-list copy and palette flushes).

## `Game`/`Phase` (`runner.rs`)

The playable bundle `{vm, st, rng, host, files, phase}`. `Game::new`
ports the init tail before `shell_enter`: `Vm::from_image` (shipped `ds`
image), `State::new`, `Rng::new(seed)`, `Host::new`, then
`select_galaxy(preset)` → `stage_save` (the `0x844B` snapshot so the
preset survives `new_game`'s `0x8432` restore) → `new_game` →
`shell_enter`.

`step()` = one frame: `pump.write` mirrors inputs into `ds:`, then
`shell_step` (Phase::Shell, `cs:0x2CC5`) or `frame_step`
(Phase::Galaxy, `cs:0x395B`; returns `Frame::Shell` → back to shell on
the `[0x9CDA] & 2` exit edge). `enter_galaxy()` forces the galaxy loop —
the new-game menu actions that `jmp 0x395B` aren't decoded yet.

## `window.rs` (feature `minifb`)

`run(g)` — a `320×200` `minifb` window at `Scale::X4`, `set_target_fps(30)`:
per frame `input()` polls cursor/buttons/keys into the pump (scancodes
are set-1: arrows `0x48`/`0x50`/`0x4B`/`0x4D`, Enter `0x1C`, Esc `0x01`,
Space `0x39`, Backspace `0x0E`; break = `scan | 0x80`), `g.step()`, then
`blit` expands `Screen::pixels()` through `Palette::rgb` into
`0xRRGGBB` `u32`s for `update_with_buffer`.
