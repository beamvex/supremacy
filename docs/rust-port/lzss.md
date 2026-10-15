# `lzss` — Okumura LZSS decoder

## What this module does

Every image stream inside a `.BIN` file is compressed with the Okumura
variant of LZSS. `lzss` decodes those streams — either into a flat
`Vec<u8>` for inspection, or straight into a strided VRAM-shaped buffer
the way the original draws them. The module ports the **four
near-identical decoder routines** the game carries (one opaque-store and
one XOR-overlay per memory layout family), unified behind a small
`Emit` trait so one decoder drives every variant.

This is the module to understand before [video.md](video.md): a `.GPH`
record points at one of these streams ([gph.md](gph.md)), and drawing an
image is exactly "run this decoder with the record's `w`/`x`/`y` against
the mode's VRAM layout".

## Background — how Okumura LZSS works

LZSS is a dictionary compressor: the decoder keeps a window of recently
output bytes and the stream tells it either "emit this literal byte" or
"copy N bytes you already emitted at position P". The trick is that the
*history is the dictionary* — there is no table to transmit; both sides
build the same window just by emitting bytes.

The parameters here are `N=4096` (a 4 KB history ring), `F=18`
(max match length), `THRESHOLD=2` (a match encodes `len − 3`, so
`len` ranges `3..=18`):

1. **Ring buffer** — `ring: [u8; 0x1000]`, all zeros at start, write
   cursor `r` starting at `0xFEE` (`N − F`; in the asm it's `SS:0x0000`
   with `bp = 0xFEE`). Every byte the decoder emits is also *stored*
   into the ring at `r`, and `r` wraps with `& 0xFFF`.
2. **Flag bits** — consumed LSB-first from a flag register (`dx` in the
   asm, `flags: u16` here). The register is loaded as
   `0xFF00 | flag_byte`; the `0xFF` high byte is a *sentinel* that takes
   exactly 8 shifts to drain — after 8 shifts the `0x100` bit has
   shifted out, so the decoder reads a fresh flag byte and sets the high
   byte to `0xFF` (`dh = 0xFF`) again. `flags` starts at `0`, so the
   very first `next_flag` triggers a load immediately. Bit `1` =
   literal, bit `0` = match.
3. **Literal** (flag 1) — read one byte, emit it, push into ring.
4. **Match** (flag 0) — read a `u16` token `t`. Decode:

   ```
   position = (t & 0xFF) | ((t & 0xF000) >> 4)   // 12-bit ring index
   length   = ((t >> 8) & 0xF) + 3               // 3..=18 bytes
   ```

   then copy `length` bytes starting at `position` in the ring — each
   copied byte is emitted *and* pushed into the ring at `r`, so a match
   can overlap and repeat patterns. The overlap is load-bearing: a match
   whose `position` is just behind `r` repeats the last byte(s) — the
   classic run-encode for solid fills and repeated tiles.
5. **Terminator** — a match token `t == 0` followed by a `00` byte (or
   by end-of-input) ends the stream. Both are checked because the asm
   tests `t == 0` then peeks the next byte — truncated input just stops.

## A worked decode

The seven bytes `03 41 42 EE F1 00 00` decode like this:

```
flags byte = 0x03  →  bits LSB-first: 1 1 0 0 0 0 0 0
ring = all zeros, r = 0xFEE

bit0 = 1 → literal: emit 'A' (0x41);  ring[0xFEE] = 'A', r = 0xFEF
bit1 = 1 → literal: emit 'B' (0x42);  ring[0xFEF] = 'B', r = 0xFF0
bit2 = 0 → match token 0xF1EE:
           position = 0xEE | 0xF00 = 0xFEE
           length   = (0xF1 & 0xF) + 3 = 4
           copy ring[0xFEE],0xFEF,0xFF0,0xFF1 — but 0xFF0/0xFF1 were
           just filled with 'A','B' by this same copy → emits A B A B
bit3 = 0 → match token 0x0000, next byte 0x00 → terminator, stop

output: "ABABAB"
```

Two things the trace demonstrates that the table format hides:

- **The match copy feeds itself.** The match at `0xFEE` runs *through*
  the live write cursor, so bytes 3 and 4 of the copy are bytes 1 and 2
  re-emitted — that's how `AB` became `ABAB`. This is legal and common;
  encoders use it for runs.
- **Flag bytes are only re-read every 8 tokens.** If the stream ends
  mid-octet the leftover flag bits are simply never consumed; the
  decoder's stop is always the terminator token or EOF, never "ran out
  of flags".

## The four asm variants

`GAME.EXE` ships the decoder four times because row stride, store op and
terminator handling differ per video mode:

| file offset | modes | row stride | store op | trailing byte |
|---|---|---|---|---|
| `0x2EBEF` | MCG/TGA/CGA | `0x140` | `stosb` | not consumed |
| `0x2EC8C` | MCG/TGA/CGA | `0x140` | `xor [es:di],al` | not consumed |
| `0x2FC5D` | EGA | `0x28` | `stosb` | consumed (`lodsb` @ `0x2FCF7`) |
| `0x2FCF9` | EGA | `0x28` | `xor [es:di],al` | consumed (`lodsb` @ `0x2FD9A`) |

Why four routines rather than parameters? 1991-era cycle-counting: the
inner loop is the game's hottest code, so each variant hardcodes its
stride and store op instead of branching. The port gets the same
specialisation for free through generics — the `Emit` sink monomorphises
to the same code a hand-specialised asm loop would produce.

The stride/`stosb`/`xor` differences live in the *sink* (`Emit`
implementations); the EGA "trailing byte" is the `eat_tail` flag —
the EGA caller expects `si` to land on the next plane stream right
after the `00 00` terminator, so it consumes one extra `0x00`. That
matters because EGA images are *four consecutive streams* (one per
bitplane): leave `si` one byte short and plane 1 starts on the
terminator of plane 0.

## Decoder state (`decoder.rs`, `consts.rs`)

- `ring: [u8; 0x1000]` — the history buffer, zeroed
  (`decoder_new.rs` = the `mov cx,0x7F7` clear loop).
- `r` — write cursor `bp`, starts `0xFEE` (`N − F`); `push` stores the
  emitted byte and wraps `& 0xFFF` (`push.rs`).
- `flags: u16` — the `dx` flag register. `next_flag` (`next_flag.rs`)
  shifts right one bit; when the `0x100` sentinel has shifted out it
  reloads `dl` from the stream with `dh = 0xFF` (`FLAG_HI`). Bit `1` =
  literal, `0` = match. `None` on truncated input — the caller treats
  that as the end of stream, same as the asm falling through on a
  missing byte.

## The decode loop (`step.rs`, `run.rs`)

`Decoder::run(src, &mut i, sink, eat_tail)` iterates `step` until it
returns `false` (terminator or truncated input), then — for the EGA
variants only — consumes the trailing zero byte so `i` lands on the next
plane stream.

`step` = one loop iteration (`jmp 0xEC0C`):

- flag 1 → `literal` (`literal.rs`): `lodsb`, emit byte, push into ring.
- flag 0 → `copy_match` (`copy_match.rs`): `lodsw` token `t`.
  Terminator is `t == 0` followed by a `00` byte (or EOF). Otherwise
  copies `((t >> 8) & 0xF) + 3` bytes starting at ring position
  `(t & 0xFF) | ((t & 0xF000) >> 4)`, each byte also pushed back into
  the ring at `r`.

`i` is an in/out cursor — callers keep it across calls because EGA
records are four streams back to back and `eat_tail` positions `i` at
the next one.

## Output sinks (`emit.rs`, `frame.rs`, `vec_emit.rs`)

The asm stores each output byte with either `stosb` (write) or
`xor [es:di],al` (XOR-overlay) — the `Emit` trait abstracts exactly
that choice plus the destination shape:

```rust
pub trait Emit { fn emit(&mut self, b: u8); }
```

- `Vec<u8>` — flat append sink for inspection/testing.
- `Frame<'a>` (`frame.rs`, `frame_new.rs`, `frame_emit.rs`) — the
  strided-VRAM sink:
  `{buf, di, width, stride, xor, row_left}`. Each byte stores or XORs at
  `di`, `di += 1`; when `row_left` hits zero it reloads to `width` and
  `di += stride − width` — the `[cs:0x5380]` down-counter + row-wrap the
  asm does at `0x2EC31`/`0x2FC9F`. In other words `Frame` is *the
  screen-shape aware sink*: the decoder emits a linear byte stream and
  `Frame` distributes it over rows `stride` bytes apart.

```
stream bytes →   A B C D E F G H I   (w=3, stride=5)
buf:             [A B C . .]
                 [D E F . .]
                 [G H I . .]
                 row_left hits 0 → di += stride − width
```

`draw` + `Blit` (`draw.rs`) bundle a strided draw call:
`{dst, di, width, stride, kind, eat_tail}` → builds a `Frame` and runs
the decoder, writing the final `di` back. `Kind` (`kind.rs`) is
`Opaque`/`Xor` — the `.GPH` record's `mode_flag` (`dl` at `0x2FB80`).

## Convenience entries

- `decode(src)` (`decode.rs`, `decoder_default.rs`) — decode from 0
  into a `Vec` (matches `tools/lzss.py::decode`).
- `decode_at(src, i) -> (Vec<u8>, usize)` (`decode_at.rs`) — returns
  bytes plus the index just past the `00 00` terminator (packed-mode
  semantics: trailing zero not consumed).
- `decode_planes(src, i, n) -> (Vec<u8>, usize)` (`decode_planes.rs`) —
  `n` consecutive EGA bitplane streams; a fresh `Decoder` per plane
  (each plane gets its own ring in the asm too — the ring does *not*
  carry over between planes) with `eat_tail = true` so `si` advances to
  the next stream. Port of the caller loop at `0x2FB85`–`0x2FBE4`
  (`0x2FBEC`–`0x2FC4A` for XOR records); `di` is restored per plane so
  all four land at the same screen offset.

## Examples

Decode a stream straight from an `AssetSet`:

```rust
let set = supremacy::assets::load_set(dir, "MCG", 0x3A22)?;
let bytes = supremacy::lzss::decode(set.stream(0));
```

Draw it into a strided buffer like the game does (MCG layout, record 0's
`x`/`y`/`w`):

```rust
let rec = &set.records[0];
let mut vram = vec![0u8; 0xFA00];
let mut blit = supremacy::lzss::Blit {
    dst: &mut vram,
    di: usize::from(rec.y) * 0x140 + usize::from(rec.x),
    width: usize::from(rec.w),
    stride: 0x140,
    kind: if rec.is_xor() { supremacy::lzss::Kind::Xor }
          else { supremacy::lzss::Kind::Opaque },
    eat_tail: false,
};
let mut i = 0;
supremacy::lzss::draw(set.stream(0), &mut i, &mut blit);
```

In practice `video::Screen::draw_image` does all of this for you —
including the per-mode stride, the four-plane EGA pass, and the
banked CGA/TGA cursor (see [video.md](video.md)).

## Pitfalls

- **The ring is not optional.** Even for "just give me the bytes" you
  must push every emitted byte into the ring — later matches index into
  it, including bytes emitted mid-match. Skipping `push` corrupts every
  subsequent match.
- **`r` starts at `0xFEE`, not 0.** `N − F` = `4096 − 18`; the encoder
  leaves the last 18 positions so a maximum-length match near the end
  wraps cleanly. Starting at 0 shifts every match position.
- **Match `pos` is a ring index, not a distance.** `0xFEE` means "the
  byte at ring slot `0xFEE`", which at stream start is a zero. The
  decoder never computes `r − d`.
- **`eat_tail` only matters for packed streams.** On a single decode to
  `Vec` it changes nothing about output — only where `i` lands. For
  EGA's four-stream records it must be `true`.
- **One `Decoder` per plane.** Reusing the ring across EGA plane
  streams would let plane 1 match against plane 0's bytes — the asm
  re-runs the clear loop for each.

## Verification

`tests/lzss_all.rs` decodes **all 1703 records in every shipped asset
set** and compares decoded length + FNV-1a hash against the verified
Python decoder (`decompiled/tools/extract_png.py`); the expected rows
are pinned in `tests/fixtures/manifest.tsv`. `tests/lzss_unit.rs`
covers the edge cases above (overlap copies, terminator variants,
truncated input) with hand-built streams like the worked example.

## File map

| file | item |
|---|---|
| `decoder.rs` / `decoder_new.rs` / `decoder_default.rs` | `Decoder` state, `new()`, `Default` |
| `consts.rs` | `N`, `R_INIT`, `SENTINEL`, `FLAG_HI` |
| `next_flag.rs` | flag-bit shift + reload (`dx`) |
| `literal.rs` | flag-1 path: emit + push |
| `copy_match.rs` | flag-0 path: token decode + ring copy |
| `token.rs` | LE `u16` token read |
| `push.rs` | `ring[r] = b; r = (r+1) & 0xFFF` |
| `step.rs` | one loop iteration |
| `run.rs` | `Decoder::run` — the loop + `eat_tail` |
| `emit.rs` | `Emit` trait |
| `vec_emit.rs` | `Emit for Vec<u8>` |
| `frame.rs` / `frame_new.rs` / `frame_emit.rs` | `Frame` strided sink |
| `kind.rs` | `Kind::{Opaque,Xor}` |
| `draw.rs` | `Blit` bundle + `draw()` |
| `decode.rs` / `decode_at.rs` / `decode_planes.rs` | convenience entries |

## Hand-verifying a decode

When a stream looks wrong, the fastest check is a three-line driver:

```rust
let s = set.stream(idx);
let (bytes, end) = supremacy::lzss::decode_at(s, 0);
// bytes == what the decoder emitted; end == index just past `00 00`
```

Compare `end` against the *next* record's `ofs_para * 16` — for a dense
packed set they differ only by the 16-byte alignment pad. For EGA,
`decode_planes(s, 0, 4)` hands back all four plane streams and the
index past the last terminator; each plane should be `h * w` bytes.
