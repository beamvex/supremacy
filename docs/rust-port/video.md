# `video` — VRAM and the draw-image path

## What this module does

`video` models one mode's video memory and the **draw-image-by-index**
routine (`ds:[0x125A]`) — the slot every screen painter calls to put a
`.GPH`-indexed picture on screen. Because each of the four adapters lays
out pixels differently (see [concepts.md](concepts.md) §7), the draw
routine exists four times in the asm; here it's one `Screen::draw_image`
that dispatches on `Video`.

The phrase "the `[0x125A]` slot" deserves a note, because the docs use
it constantly: the game's per-mode parameter template ([args.md](args.md)
`template_seg`) contains a table of routine addresses at `ds:0x1258+`.
The asm calls them indirectly — `call word [0x125A]` — so the *same*
call site draws an image in whichever mode is active. In the port the
slot indirection is gone (`Host::svc` matches on `Call::Image`), but the
docs still say "the `[0x125A]` slot" to mean "whichever routine the
original would dispatch to".

## Background — the mode parameter templates

Per-mode constants come from the `0x115B`-byte parameter templates
(`decompiled/FUNCTIONS.md` § "Mode parameter templates"; the video-mode
select `rep movsb`s the chosen one to `ds:0x143`). Byte 0 = the `int 10`
mode number, word 4 = the VRAM segment:

| mode | `int10_mode` | VRAM seg | `vram_len` | `di` for record (x,y) | row advance |
|---|---|---|---|---|---|
| MCG | `0x13` | `0xA000` | `0xFA00` | `y*0x140 + x` | `+0x140 − w` |
| EGA | `0x0D` | `0xA000` | `4×0x1F40` | `y*0x28 + x` (per plane) | `+0x28 − w`, ×4 planes |
| CGA | `0x05` | `0xB800` | `0x4000` | `(y>>1)*0x50 + (y&1)*0x2000 + x` | bank toggle `0x2000`, `+0x50` |
| TGA | `0x09` | `0xB800` | `0x8000` | `(y>>2)*0xA0 + (y&3)*0x2000 + x` | `+0x2000`, wrap `0x8000`, `+0xA0` |

The `di` column is the key fact: it's the byte offset inside VRAM where
row 0 of an image lands for a record at (x,y). MCG is trivially linear
(`y*320 + x`); EGA is linear *per plane* (`y*40 + x`, ×4 planes — each
plane byte covers 8 horizontal pixels, one bit each); CGA/TGA are
**banked** — alternate rows live `0x2000` bytes apart because the
hardware interleaves them.

### What the layouts look like

```
MCG (0xA000, 0xFA00 bytes):   row r → bytes r*320 .. r*320+319
                              di(x,y) = y*0x140 + x   — a bitmap.

EGA (0xA000, 4 × 0x1F40):     plane p byte at y*0x28 + x covers
                              pixels x*8..x*8+8, one bit each;
                              colour index = Σ bit(plane p) << p.
                              di(x,y) = y*0x28 + x within each plane.

CGA (0xB800, 0x4000):         even rows at 0x0000.., odd at 0x2000..
                              di(x,y) = (y>>1)*0x50 + (y&1)*0x2000 + x
                              — "row 5" means bank 1, row-pair 2.

TGA (0xB800, 0x8000):         four banks stepped by (y & 3):
                              di(x,y) = (y>>2)*0xA0 + (y&3)*0x2000 + x
```

The row-advance column is what `lzss::Frame`/`Banked` implement as the
cursor step between rows — the reason "draw a rect" isn't just
`memcpy`: on CGA, finishing row `w` bytes means *rewind* `di` by `w`,
then hop banks.

- `int10.rs` — `int10_mode()` (template byte 0) and
  `programs_palette()` (only MCGA runs the `[0x1258]` DAC slot for real —
  `int 10/AX=1012` at file `0x2EDBB`; other modes land on the `0xF294`
  stub — the template literally points those modes at a `ret`, so the
  palette call is a no-op there).
- `vram.rs` — `vram_segment()`/`vram_len()`.
- `di.rs` — `di_at(x, y)`, the per-mode start-offset formulas above
  (`0x2EBB1`/`0x2FB69`/`0x3064F`/`0x310F9`).

## `Screen` (`screen.rs`)

One mode's VRAM span as a flat `buf: Vec<u8>` — what the decoders write
through `es:di`. MCG/CGA/TGA are single regions; EGA is the four
`0x1F40`-byte bitplanes laid out consecutively (in hardware the draw
routine reprograms the map-mask register between passes, so each plane
stream lands at the same `di` in a different plane — the port's
`buf[plane * 0x1F40 ..]` slices are that plane select).

```rust
let mut scr = supremacy::video::Screen::new(supremacy::args::Video::Mcga);
scr.draw_image(&set, 0);          // the [0x125A] slot
let px = scr.pixels();            // 320×200 palette indices, row-major
```

## `draw_image` (`draw.rs`) — step by step

One call = one record's compressed stream(s) written into `buf` in the
mode's layout. For a record at `(x, y)` with `w` bytes/row, `h` rows:

- `mode_flag` → `Kind::Opaque`/`Xor`; any other value draws nothing
  (`draw_kind`) — matching the asm's two-way `cmp`/`je` (see
  [gph.md](gph.md)).
- `Mcga` — `lzss::Frame` at `di_at(x,y)`, stride `0x140`: a linear
  decode, each emitted byte lands at `di` and `di` hops `0x140 − w` on
  row wrap. The simplest path and the one to trace first.
- `Ega` — `ega::draw` (`ega.rs`): four passes over the record's
  consecutive plane streams, each a `Frame` into `buf[plane*0x1F40..]`
  at `di = row*0x28 + col`, `eat_tail = true` (the EGA decoder consumes
  the terminator's trailing byte so `si` lands on the next stream — see
  [lzss.md](lzss.md)). One `.GPH` record therefore contains **four**
  LZSS streams back to back, and all four write the same `di` in their
  own plane — the four bitplanes of one image.
- `Cga`/`Tga` — `banked()` → `Banked` sink (`banked.rs`): the
  interleaved decoder copies `0x30695`/`0x31148`. After `w` emitted
  bytes the cursor backs up (`di −= w`), steps one bank
  (`di += 0x2000`), and on passing `span` (`0x4000`/`0x8000`) drops
  back `span − pitch` (`0x50`/`0xA0`) — walking the interleave the
  same way the asm does. Net effect on CGA: rows alternate between the
  two 8 KB halves exactly like the hardware's interlace.

The `ds:[0x1298]` dirty-region call is bookkeeping, not clipping, so it
is omitted; `ofs_para ≥ 0xA000` can't occur post-fixup and is dropped
the same way MCG drops it (the asm's guard against a record whose
stream pointer would live in VRAM — meaningless on a flat buffer).

## `pixels()` (`pixels.rs`)

Expands the framebuffer to 320×200 palette-index pixels, row-major —
the inverse of each mode's packing (same mapping as
`tools/extract_png.py::expand`). Use it when you want RGB output: feed
each returned byte into `Palette::rgb` (see [palette.md](palette.md)),
exactly as `bin/supremacy.rs`'s `write_ppm` does:

- MCGA — direct byte.
- EGA — gather bit `7−(x&7)` from each of the four planes
  (`p*0x1F40 + y*0x28 + x>>3`) into a 4-bit index. Note `7−(x&7)`:
  the pixels are MSB-first within the byte — bit 7 is the leftmost.
- CGA — `(y>>1)*0x50 + (y&1)*0x2000 + x>>2`, 2 bits per pixel from
  MSB down — so pixel `x` uses bits `(3−(x&3))*2..` of the byte.
- TGA — `(y>>2)*0xA0 + (y&3)*0x2000 + x>>1`, hi nibble for even `x`.

`pixels()` is the debug/escape hatch: the game itself never needs it
(the CRT does the unpacking), but the PPM writer, the minifb blit and
every screenshot test go through it.

## `put_pixel()` (`put.rs`)

The write-side inverse of `pixel_at` — merge one pixel into `buf` in
the mode's layout (GAM-36): direct store on MCG, per-plane bit set/clear
on EGA (the map-mask write the `[0x1268]`/`[0x1262]` routines do via
`call 0x8498`), packed 2-bit/4-bit merges on CGA/TGA. The frontend's
glyph put and popup frame draw through it, so those paths work in
every mode.

On packed modes "set one pixel" is a read-modify-write: on CGA the
byte holds four pixels, so `put_pixel` masks out the two target bits
and ORs the new pair in; on EGA it touches the same bit position in
all four planes, setting or clearing per the colour's bits.

## Pitfalls

- **`di` is not a pixel index on packed modes.** On EGA `di` counts
  *plane bytes* (each covering 8 px); on CGA/TGA it jumps banks every
  row. Reading `buf[di]` expecting "pixel (x,y)" only works on MCG.
- **`w` differs per mode for the same picture.** See
  [gph.md](gph.md) — the record's `w` is bytes/row in the mode's
  packing; a 40-px-wide image is `w=40` on MCG, `w=5` on EGA.
- **EGA = 4 streams.** `draw_image` runs the decoder four times on
  consecutive streams; a record that decodes "short" on plane 0 is
  expected — the planes don't share a ring.
- **`pixels()` is for output, not the game's own path.** The asm never
  expands to a flat framebuffer; the port does it for hosts/tests.

## File map

| file | item |
|---|---|
| `mod.rs` | module docs, the mode table, re-exports |
| `di.rs` | `Video::di_at(x, y)` — the per-mode start offsets |
| `vram.rs` | `vram_segment`/`vram_len` |
| `int10.rs` | `int10_mode`, `programs_palette` |
| `screen.rs` | `Screen { mode, buf }` + `Screen::new` |
| `draw.rs` | `Screen::draw_image` — the `[0x125A]` dispatcher |
| `ega.rs` | `ega::draw` — the four-plane pass |
| `banked.rs` | `Banked` — the CGA/TGA interleave sink |
| `pixels.rs` | `pixels`/`pixel_at` — VRAM → 320×200 indices |
| `put.rs` | `put_pixel` — the per-mode single-pixel write |

## Worked example — where a pixel lands

Record `x=0x40, y=0x60, w=0x20` in each mode (`di_at`):

- **MCG**: `di = 0x60*0x140 + 0x40 = 0x7840` — byte 0x7840 of a 64 000
  buffer.
- **EGA**: `di = 0x60*0x28 + 0x40 = 0xF40` *within each plane* — four
  writes, `buf[0xF40]`, `buf[0x1F40+0xF40]`, `buf[0x3E80+0xF40]`,
  `buf[0x5DC0+0xF40]`.
- **CGA**: `y=0x60` is even → `di = (0x60>>1)*0x50 + 0 + 0x40 =
  0xFA0` — bank 0. `y=0x61` would be `0x50*0x30 + 0x2000 + 0x40`.
- **TGA**: `y&3 = 0` → `di = (0x60>>2)*0xA0 + 0x40 = 0x12C0`.
