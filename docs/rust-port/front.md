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
- `Sound`/`Chan`/`Sfx` → `snd` — the `audio` backend below (GAM-31);
  `Song`/`SndReset` (`[cs:0x1278]`/`[cs:0x1280]` far exports) →
  `snd.far_export()`.
- `Call::Load(ofs)` → `front/load.rs` `Loader` — `ds:0x1208`+ cell →
  `{stem}{sfx}.{BIN,GPH}`; the `.GPH` half swaps `set`
  (`Host::set_assets` configures dir + stem; the endings' `*DTH`/`*WIN`
  and the restart tail's `*IN`/base pairs ride it).
- `Call::Text(ofs)` → `front/text.rs` — one `menu_text` page of the
  `ss:0x20000` stream; returns `end<<8 | fb15`; the 16×24 glyph tiles
  are counted `dropped` (menu font not drawn).
- `Call::Stars(cx)` → `front/stars.rs` — the host-side `cs:0x4EC4`
  starfield walk, `cx+1` stars marked in `ink`.
- Menu/Dialog/Refresh/Present/Flash/PlanetPanel/Blit/other
  `Slot`/`Native`/`Ui`/`Draw` → `dropped += 1`
  (GAM-7/GAM-9 stubs — the window's per-frame blit supersedes the
  refresh-list copy and palette flushes).

## `Game`/`Phase` (`runner.rs`)

The playable bundle `{vm, st, rng, host, files, phase}`. `Game::new`
ports the init tail before `shell_enter`: `Vm::from_image` (shipped `ds`
image), `State::new`, `Rng::new(seed)`, `Host::new`, the `[0x143]`
BIOS-mode word the mode select leaves, then `select_galaxy(preset)` →
`new_game` (whose head runs `stage_save`/`0x8432`, so the preset lands
in the staging snapshot) → `shell_enter`.

`Game::new_title` is the asm's boot path instead: `State::from_image`
(the picker's template records ship in the save block) then
`title_enter` — the `0x85B1` attract parked in `Vm::title`; the picker's
start action selects the preset and the shared `0x1D60` tail runs
`act_newgame` (the full `0x305B` — head, repaint and shell re-entry).

`step()` = one frame: `pump_input` mirrors inputs into `ds:` (plus
the `[0x91D3]` gate byte into the audio layer), then — in precedence
order — `end_step` while `Vm::end` is live (the `cs:0x8B17`/`0x8C65`
endings), `title_step` while `Vm::title` is live (boot or post-ending
restart), else `shell_step` (Phase::Shell, `cs:0x2CC5`) or `frame_step`
(Phase::Galaxy, `cs:0x395B`; returns `Frame::Shell` → back to shell on
the `[0x9CDA] & 2` exit edge). `enter_galaxy()` forces the galaxy loop
through the `0x3906` action.

## `window.rs` (feature `minifb`)

`run(g)` — a `320×200` `minifb` window at `Scale::X4`, `set_target_fps(30)`:
per frame `input()` polls cursor/buttons/keys into the pump (scancodes
are set-1: the whole `ds:0x7CBD` keymap — alnum rows, `-`/`=`/`;`/`\`/
`,`/`.`/`/`/space, keypad digits and the edit cluster — plus the
`K`-mode nav arrows `0x48`/`0x50`/`0x4B`/`0x4D`, Enter `0x1C`, Esc
`0x01`; break = `scan | 0x80`), `g.step()`, then
`blit` expands `Screen::pixels()` through `Palette::rgb` into
`0xRRGGBB` `u32`s for `update_with_buffer`. With the `audio`
feature a cpal `audio::Out` stream is opened first (rate negotiated
back into `snd`), and each stepped frame renders `rate/30` mono
samples into its bounded queue.

## `audio` (`audio/`) — the sound backend (GAM-7/31)

`audio::Audio` owns the `[0x155]`-picked driver (`drv::Drv` — `Pc` /
`Tandy` / `Ar` over `audio::Bus`) and maps the game calls onto the asm
wrapper semantics (files `0x38A5B`, `0x38A3B`, `0x38A80`, `0x38577`):
`Sfx` → `int 80` `ah=1`+`ah=3` + the `[cs:0x127C]` export;
`Chan(0x8A5B)` → `ah=1` + `[cs:0x1280]`; `Chan(0x8A3B)` → `ah=3`;
`Chan(0x8A80)` → `ah=2` gated on the mirrored `[0x91D3] == 0`;
`Sound`/`Song`/`SndReset` → the far exports alone. The `int 80` path
gates `[0x155] <= 2`, so the A/R module is only reached through its
`cs:0x126C` far table (`init`/`load_song`/`select`/`reset`).

`Bus` is the concrete `audio::Ports`: `0x43`/`0x42` latch the PIT-2
divisor, `0x61` is the gate, and `sample()` emits the square wave;
`0xC0` feeds the `psg` SN76489 (three tone voices + LFSR noise, mixed
while the driver's `0x61 | 0x60` Tandy-enable write holds); `0x388`/
`0x389` write an `oplon` OPL2 rendered in the same frame; `0x330`/
`0x331` answer as a quiescent MPU-401 (MIDI bytes accepted and
dropped). `Audio::render` runs the driver's `ah=0` tick (the A/R
export-2 `service`) at the ~50 Hz rate `install_drivers` programs
into PIT channel 0 (`0x5CEC`; the `0x5D37` at `cs:[0x1C00]` is the
A/R module's own install) inside the sample stream.
