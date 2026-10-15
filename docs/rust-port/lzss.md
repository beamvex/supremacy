# `lzss` — Okumura LZSS decoder

## What this module does

Every image stream inside a `.BIN` file is compressed with the Okumura
variant of LZSS. `lzss` decodes those streams — either into a flat
`Vec<u8>` for inspection, or straight into a strided VRAM-shaped buffer
the way the original draws them. The module ports the **four
near-identical decoder routines** the game carries (one opaque-store and
one XOR-overlay per memory layout family), unified behind a small
`Emit` trait so one decoder drives every variant.

## Background — how Okumura LZSS works

LZSS is a dictionary compressor: the decoder keeps a window of recently
output bytes and the stream tells it either "emit this literal byte" or
"copy N bytes you already emitted at position P".

The parameters here are `N=4096` (a 4 KB history ring), `F=18`
(max match length), `THRESHOLD=2` (a match encodes `len − 3`, so
`len` ranges `3..=18`):

1. **Ring buffer** — `ring: [u8; 0x1000]`, all zeros at start, write
   cursor `r` starting at `0xFEE` (`N − F`; in the asm it's `SS:0x0000`
   with `bp = 0xFEE`). Every byte the decoder emits is also *stored*
   into the ring at `r`, and `r` wraps with `& 0xFFF`.
2. **Flag bits** — consumed LSB-first from a flag register (`dx` in the
   asm, `flags: u16` here). After 8 shifts the `0x100` sentinel bit has
   shifted out, so the decoder reads a fresh flag byte and sets the high
   byte to `0xFF` (`dh = 0xFF`) — that's how it knows when to reload.
   Bit `1` = literal, bit `0` = match.
3. **Literal** (flag 1) — read one byte, emit it, push into ring.
4. **Match** (flag 0) — read a `u16` token `t`. Decode:

   ```
   position = (t & 0xFF) | ((t & 0xF000) >> 4)   // 12-bit ring index
   length   = ((t >> 8) & 0xF) + 3               // 3..=18 bytes
   ```

   then copy `length` bytes starting at `position` in the ring — each
   copied byte is emitted *and* pushed into the ring at `r`, so a match
   can overlap and repeat patterns.
5. **Terminator** — a match token `t == 0` followed by a `00` byte ends
   the stream (as does truncated input).

## The four asm variants

`GAME.EXE` ships the decoder four times because row stride, store op and
terminator handling differ per video mode:

| file offset | modes | row stride | store op | trailing byte |
|---|---|---|---|---|
| `0x2EBEF` | MCG/TGA/CGA | `0x140` | `stosb` | not consumed |
| `0x2EC8C` | MCG/TGA/CGA | `0x140` | `xor [es:di],al` | not consumed |
| `0x2FC5D` | EGA | `0x28` | `stosb` | consumed (`lodsb` @ `0x2FCF7`) |
| `0x2FCF9` | EGA | `0x28` | `xor [es:di],al` | consumed (`lodsb` @ `0x2FD9A`) |

The stride/`stosb`/`xor` differences live in the *sink* (`Emit`
implementations); the EGA "trailing byte" is the `eat_tail` flag —
the EGA caller expects `si` to land on the next plane stream right
after the `00 00` terminator, so it consumes one extra `0x00`.

## Decoder state (`decoder.rs`, `consts.rs`)

- `ring: [u8; 0x1000]` — the history buffer, zeroed
  (`decoder_new.rs` = the `mov cx,0x7F7` clear loop).
- `r` — write cursor `bp`, starts `0xFEE` (`N − F`); `push` stores the
  emitted byte and wraps `& 0xFFF` (`push.rs`).
- `flags: u16` — the `dx` flag register. `next_flag` shifts right one
  bit; when the `0x100` sentinel has shifted out it reloads `dl` from
  the stream with `dh = 0xFF` (`FLAG_HI`). Bit `1` = literal, `0` =
  match.

## The decode loop (`step.rs`, `run.rs`)

`Decoder::run(src, &mut i, sink, eat_tail)` iterates `step` until it
returns `false` (terminator or truncated input), then — for the EGA
variants only — consumes the trailing zero byte so `i` lands on the next
plane stream.

`step` = one loop iteration (`jmp 0xEC0C`):

- flag 1 → `literal`: `lodsb`, emit byte, push into ring.
- flag 0 → `copy_match`: `lodsw` token `t`. Terminator is `t == 0`
  followed by a `00` byte (or EOF). Otherwise copies
  `((t >> 8) & 0xF) + 3` bytes starting at ring position
  `(t & 0xFF) | ((t & 0xF000) >> 4)`, each byte also pushed back into the
  ring at `r`.

## Output sinks (`emit.rs`, `frame.rs`, `vec_emit.rs`)

The asm stores each output byte with either `stosb` (write) or
`xor [es:di],al` (XOR-overlay) — the `Emit` trait abstracts exactly
that choice plus the destination shape:

```rust
pub trait Emit { fn emit(&mut self, b: u8); }
```

- `Vec<u8>` — flat append sink for inspection/testing.
- `Frame<'a>` (`frame.rs`) — the strided-VRAM sink:
  `{buf, di, width, stride, xor, row_left}`. Each byte stores or XORs at
  `di`, `di += 1`; when `row_left` hits zero it reloads to `width` and
  `di += stride − width` — the `[cs:0x5380]` down-counter + row-wrap the
  asm does at `0x2EC31`/`0x2FC9F`.

`draw` + `Blit` (`draw.rs`) bundle a strided draw call:
`{dst, di, width, stride, kind, eat_tail}` → builds a `Frame` and runs
the decoder, writing the final `di` back. `Kind` (`kind.rs`) is
`Opaque`/`Xor` — the `.GPH` record's `mode_flag` (`dl` at `0x2FB80`).

## Convenience entries

- `decode(src)` — decode from 0 into a `Vec` (matches
  `tools/lzss.py::decode`).
- `decode_at(src, i) -> (Vec<u8>, usize)` — returns bytes plus the index
  just past the `00 00` terminator (packed-mode semantics: trailing zero
  not consumed).
- `decode_planes(src, i, n) -> (Vec<u8>, usize)` — `n` consecutive EGA
  bitplane streams; a fresh `Decoder` per plane (each plane gets its own
  ring in the asm too) with `eat_tail = true` so `si` advances to the
  next stream. Port of the caller loop at `0x2FB85`–`0x2FBE4`
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

## Verification

`tests/lzss_all.rs` decodes **all 1703 records in every shipped asset
set** and compares decoded length + FNV-1a hash against the verified
Python decoder (`decompiled/tools/extract_png.py`); the expected rows
are pinned in `tests/fixtures/manifest.tsv`.
