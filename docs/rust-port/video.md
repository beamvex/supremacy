# `video` — VRAM and the draw-image path

## What this module does

`video` models one mode's video memory and the **draw-image-by-index**
routine (`ds:[0x125A]`) — the slot every screen painter calls to put a
`.GPH`-indexed picture on screen. Because each of the four adapters lays
out pixels differently (see [concepts.md](concepts.md) §7), the draw
routine exists four times in the asm; here it's one `Screen::draw_image`
that dispatches on `Video`.

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

- `int10.rs` — `int10_mode()` (template byte 0) and
  `programs_palette()` (only MCGA runs the `[0x1258]` DAC slot for real —
  `int 10/AX=1012` at file `0x2EDBB`; other modes land on the `0xF294`
  stub).
- `vram.rs` — `vram_segment()`/`vram_len()`.
- `di.rs` — `di_at(x, y)`, the per-mode start-offset formulas above
  (`0x2EBB1`/`0x2FB69`/`0x3064F`/`0x310F9`).

## `Screen` (`screen.rs`)

One mode's VRAM span as a flat `buf: Vec<u8>` — what the decoders write
through `es:di`. MCG/CGA/TGA are single regions; EGA is the four
`0x1F40`-byte bitplanes laid out consecutively (in hardware the draw
routine reprograms the map-mask register between passes, so each plane
stream lands at the same `di` in a different plane).

```rust
let mut scr = supremacy::video::Screen::new(supremacy::args::Video::Mcga);
scr.draw_image(&set, 0);          // the [0x125A] slot
let px = scr.pixels();            // 320×200 palette indices, row-major
```

`draw_image(set, idx)` (`draw.rs`) — the `ds:[0x125A]` dispatcher, port
of the four per-mode routines:

- `mode_flag` → `Kind::Opaque`/`Xor`; any other value draws nothing
  (`draw_kind`).
- `Mcga` — `lzss::Frame` at `di_at(x,y)`, stride `0x140`.
- `Ega` — `ega::draw` (`ega.rs`): four passes over the record's
  consecutive plane streams, each a `Frame` into `buf[plane*0x1F40..]`
  at `di = row*0x28 + col`, `eat_tail = true` (the EGA decoder consumes
  the terminator's trailing byte so `si` lands on the next stream — see
  [lzss.md](lzss.md)).
- `Cga`/`Tga` — `banked()` → `Banked` sink (`banked.rs`): the
  interleaved decoder copies `0x30695`/`0x31148`. After `w` emitted bytes
  the cursor backs up (`di −= w`), steps one bank (`di += 0x2000`), and
  on passing `span` (`0x4000`/`0x8000`) drops back `span − pitch`
  (`0x50`/`0xA0`) — walking the interleave the same way the asm does.

The `ds:[0x1298]` dirty-region call is bookkeeping, not clipping, so it
is omitted; `ofs_para ≥ 0xA000` can't occur post-fixup and is dropped
the same way MCG drops it.

## `pixels()` (`pixels.rs`)

Expands the framebuffer to 320×200 palette-index pixels, row-major —
the inverse of each mode's packing (same mapping as
`tools/extract_png.py::expand`). Use it when you want RGB output: feed
each returned byte into `Palette::rgb` (see [palette.md](palette.md)),
exactly as `bin/supremacy.rs`'s `write_ppm` does:

- MCGA — direct byte.
- EGA — gather bit `7−(x&7)` from each of the four planes
  (`p*0x1F40 + y*0x28 + x>>3`) into a 4-bit index.
- CGA — `(y>>1)*0x50 + (y&1)*0x2000 + x>>2`, 2 bits per pixel from
  MSB down.
- TGA — `(y>>2)*0xA0 + (y&3)*0x2000 + x>>1`, hi nibble for even `x`.

## `put_pixel()` (`put.rs`)

The write-side inverse of `pixel_at` — merge one pixel into `buf` in
the mode's layout (GAM-36): direct store on MCG, per-plane bit set/clear
on EGA (the map-mask write the `[0x1268]`/`[0x1262]` routines do via
`call 0x8498`), packed 2-bit/4-bit merges on CGA/TGA. The frontend's
glyph put and popup frame draw through it, so those paths work in
every mode.
