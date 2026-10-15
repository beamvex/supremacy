# Walkthrough — record 0 from disk to pixel, and one frame of `supremacy-run`

This page traces two end-to-end paths through the port at the level of
"which function runs when", so you can watch the whole pipeline fire
before you read the per-module pages. It assumes [concepts.md](concepts.md).

## Path 1: `supremacy ../GAME M ' ' A --idx 0 --out frame.ppm`

What `bin/supremacy.rs` does, step by step, with the asm it mirrors:

1. **Tail rebuild** — `tail_bytes(["M", " ", "A"])` synthesises the
   PSP layout `[6, ' ', 'M', ' ', 'M', ' ', 'A']` — the bytes DOS would
   have left at `es:0x80` for `GAME M M A` ([args.md](args.md)).
2. **`args::parse(&tail)`** → `Config { video: Mcga, keyboard_mouse:
   false, sound: AdLib, timer_variant: 0x1D0 }` — the four cells the
   asm's parser writes at `0x31FF8`.
3. **`Video::stem()`** → `"MCG"`; **`load_set(dir, "MCG",
   heap_segment())`** reads `MCG.GPH` (≤356 12-byte records) and
   `MCG.BIN` (the whole compressed heap), then `gph::fix_up` adds
   `0x3A22` to every `ofs_para` — the loader pair `0x3208E`/`0x320F2`
   ([assets.md](assets.md)).
4. **`Screen::new(Mcga)`** → `buf = vec![0; 0xFA00]` — the `0xA000`
   segment as a flat Vec ([video.md](video.md)).
5. **`Screen::draw_image(&set, 0)`** — record 0 is
   `x=0,y=0,w=0x140,h=0xC8,ofs=0,mode=0` (opaque, full-screen):
   `di_at(0,0) = 0`, `Frame { width: 320, stride: 0x140 }`, `Kind::Opaque`,
   `eat_tail = false`. The LZSS decoder ([lzss.md](lzss.md)) emits
   64 000 bytes into `buf` — row-straight, because stride == width.
   That is the `[0x125A]` slot call `call word [0x125A]` the asm makes.
6. **`Palette` build** — `from_dac(&img[0x15034..0x15034+768])` reads
   the shipped 256×3 six-bit table and scales to 8-bit
   ([palette.md](palette.md)).
7. **`Screen::pixels()`** — for MCGA this is an identity copy of `buf`
   (one byte per pixel); on EGA it would gather four bitplane bits per
   pixel, on CGA/TGA undo the banked packing.
8. **`write_ppm`** — `P6 320 200 255\n` + `pal.rgb(px)` per pixel —
   `frame.ppm` is a viewable image of the title backdrop.

Every step is a direct port: `parse` *is* the PSP read, `load_set` *is*
the `.GPH`/`.BIN` loader, `draw_image` *is* the `[0x125A]` call.

## Path 2: one frame of `supremacy-run`

`bin/run.rs` boots `front::Game::new_title` (the real `0x85B1` attract
path) and `window::run` calls `game.step()` 30×/second. One `step()`:

1. **`pump_input`** — `front::Pump::write` mirrors the window's input
   into `ds:` cells: cursor → `[0x9CD6]`/`[0x9CD8]`, one queued
   scancode → `[0x9CCB]`/`[0x9CCC]` if free, K-flag → `[0x9CCD]`
   ([front.md](front.md) § `Pump`). Also mirrors `[0x91D3]` into the
   audio layer's quiet gate.
2. **Phase dispatch** — `Vm::end` live → `end_step` (ending crawl);
   `Vm::title` live → `title_step` (attract); else `Phase::Shell` →
   `shell_step`, `Phase::Galaxy` → `frame_step` ([game-ui.md](game-ui.md)).
3. **Inside `shell_step`** (uimode 0): `vsync` → `seq_step` (one
   sequencer command if the phase wraps) → `Native(0xA369)` input →
   `menu::service` (hotspot scan over `[0x9194]`) → `tick_step` (one
   sim arm — see below) → `mark_sel` → `row_click` → `blink_step` →
   `hover_check` → `ticker_step` → `FRAME++`
   ([game-tick.md](game-tick.md)).
4. **Inside `tick_step`** — `[0x91CE]` increments and picks one arm:
   on a frame where `bl` is `0x38+5`, say, that's `sim_planet(5)` — the
   fifth planet's economy step ([game-tick.md](game-tick.md) § `sim.rs`).
   Next frame `bl` moves on — the galaxy advances one slice per frame.
5. **Every host touch is a `Call`** — any `Image`/`Glyph`/`Sound`/
   `Present` the routines emit goes to `Host::svc`, which draws into
   `Screen::buf`, programs `Palette`, or routes the audio driver
   ([front.md](front.md) § `svc`).
6. **`window::blit`** — `Screen::pixels()` → `Palette::rgb` per pixel →
   the `minifb` buffer; `Audio::render` pushes `rate/30` PCM samples
   onto the cpal queue. One frame done.

The whole frame cost is bounded the same way the original's was: at
most one sim arm, one sequencer command, one menu scan, and whatever
the armed reprints need — the asm kept 1991 frame times by never doing
more than a slice, and the port inherits that.

## The two moment-to-moment loops side by side

| shell frame (`cs:0x2D5B`) | galaxy frame (`cs:0x395B`) |
|---|---|
| `vsync` | `vsync` |
| `seq_step` | overlay (`PANEL_ON && uimode 5`) |
| `Native(0xA369)` input | `REDRAW` block (conditional) |
| `menu::service` | `PANEL_REQ` → `info_panel` |
| `tick_step` | `ambient` sfx |
| `mark_sel`, `row_click` | `FRAME++` |
| `blink_step`, `hover_check` | `tick_step`, `seq_step` |
| `ticker_step` (if blink idle) | `Native(0xA369)`, `menu::service` |
| `FRAME++` | `dirty_flags` (≤1 bit), exit edge test |

Same skeleton — vsync, seq, input, menu, tick — with each screen's own
jobs around it. The port keeps both as step functions the runner
alternates on `uimode`.

## Where to go next

- The number formats and why every doc cites three address spaces:
  [concepts.md](concepts.md).
- The file formats behind step 3–5 above: [gph.md](gph.md),
  [lzss.md](lzss.md), [video.md](video.md), [palette.md](palette.md).
- The frame machinery behind step 3–4: [game-state.md](game-state.md),
  [game-vm.md](game-vm.md), [game-tick.md](game-tick.md),
  [game-ui.md](game-ui.md).
