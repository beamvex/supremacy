# `gph` — `.GPH` graphics index

## What this module does

Every asset set the game ships comes as a file pair: a `*.BIN` blob of
compressed image streams and a `*.GPH` index describing each image
inside it. `gph` parses that index: it turns the raw `.GPH` bytes into a
vector of `Record`s telling you where each picture goes on screen, how
big it is, where its compressed stream sits in the `.BIN`, and whether
to draw it opaquely or XOR it over the background.

Think of `.BIN` as a tape of LZSS streams back to back and `.GPH` as the
track list — the decoder never looks inside the `.BIN` without a record
telling it where to start and how much screen to cover.

It also ports the small **fixup loop** the original runs at load time,
which relocates stream offsets so the decoder can address them against
the segment where the `.BIN` was loaded.

## Background — the file format

A `.GPH` file is a flat array of fixed 12-byte records — a C struct
written verbatim to disk (all fields little-endian `u16`):

```c
struct gph_rec {
    uint16_t x, y;       // draw position (pixels / rows)
    uint16_t w, h;       // image size; w is bytes-per-row in this mode
    uint16_t ofs_para;   // stream offset inside .BIN — in paragraphs
    uint16_t mode_flag;  // 0 = opaque blit, 1 = XOR overlay
};
```

A worked example — one record's bytes as they appear in `MCG.GPH`:

```
bytes on disk:        00 00   00 00   40 01   C8 00   00 00   00 00
field:                x=0     y=0     w=0x140 h=0xC8  ofs=0   mode=0
                                      (320B/row)(200 rows)      opaque
```

That record says "draw a full 320×200 opaque screen whose stream starts
at byte 0 of `MCG.BIN`" — `w` is *bytes per row* in the mode's layout,
so `0x140` = 320 on MCGA (1 byte/px) but the same visual width would be
`0x28` = 40 on EGA (8 px per byte per plane) or `0x50` on CGA (4 px per
byte, 80 bytes/row).

`ofs_para` is the format's one DOS-ism: the offset is counted in
**paragraphs** (16-byte units, see [concepts.md](concepts.md) §1), so the
stream for a record with `ofs_para = 0x100` begins at byte `0x1000` of
the `.BIN`. Streams are therefore always 16-byte aligned in the blob —
the compiler/packer padded each one. On disk the offsets are relative
to the start of the `.BIN`; at load time the game adds the heap segment
where it loaded the blob so `seg:ofs_para` becomes a far pointer — that
addition is the "fixup". Concretely: with heap segment `0x3A22`, a
record whose disk `ofs_para` is `0x100` becomes `0x3B22`, meaning "the
stream at `0x3B22:0000`" — which in the loaded image is
`(0x3B22 − 0x3A22) * 16 = 0x1000` bytes into the heap, exactly the same
`.BIN` byte.

`mode_flag` chooses the store op the decoder uses: `0` overwrites
background bytes (`stosb`), `1` XORs pixels on top (`xor [es:di],al`) —
the game's cheap transparency effect for overlays. Any other value draws
nothing at all: the dispatch is a `cmp`/`je` pair, not a table, so
values ≥ 2 fall off the end and the record is skipped. (The port's
`draw_kind` reproduces this — don't "fix" it into an error.)

## The asm being ported

The loader reads exactly `0x10B0` bytes of index (`mov cx,0x10B0` at
file `0x320AC` — room for 356 records; a short read is fine), then the
loop at `0x320D8` walks the records with `add si,0xC` (the 12-byte
stride) and executes `add [si+0x160],ax` — adding the `.BIN` heap
segment to every `ofs_para` in place. The `+0x160` is the record's
`ofs_para` field as seen through the loader's `si` base — same field
`+8` within a record, offset by where `si` points.

The important invariant: **fixup happens once, at load, in place**.
After it runs, `ofs_para` no longer describes a file position — it is a
live segment value, meaningless without knowing the heap segment it was
relocated against. That is why `AssetSet` keeps `heap_seg` around:
`AssetSet::stream` subtracts it back to recover the file-relative
paragraph before multiplying by 16.

## API tour

`Record` (`record.rs`, `RECORD_LEN = 12` = the `add si,0xC` stride at
`0x320E2`):

| field | asm ref | meaning |
|---|---|---|
| `x`, `y` | `[si+0]`, `[si+2]` | draw position (px / rows) |
| `w` | `[si+4]` | bytes per row for the mode — `ax` on decoder entry |
| `h` | `[si+6]` | row count |
| `ofs_para` | `[si+8]` | stream offset **in paragraphs** inside `.BIN` |
| `mode_flag` | `[si+0xA]` | `0` = opaque blit, `1` = XOR overlay |

Functions:

- `parse_index(buf) -> Vec<Record>` (`parse.rs`) — reads six LE `u16`s
  per record; a trailing partial record is ignored, matching the asm
  which never checks. The game reads a fixed `0x10B0` bytes (≤356
  records) into `ds:0x158`.
- `fix_up(records, heap_seg)` (`fixup.rs`) — the `0x320D8` loop: adds
  `heap_seg` to every `ofs_para` in place, turning file-relative
  paragraph offsets into absolute segment bases for the renderer.
- `Record::is_xor()` (`record_ofs.rs`) — `mode_flag == 1` (`cmp dl,1` at
  `0x2FBE7`).

## Example

```rust
let raw = std::fs::read("GAME/MCG.GPH")?;
let mut recs = supremacy::gph::parse_index(&raw[..0x10B0.min(raw.len())]);
supremacy::gph::fix_up(&mut recs, 0x3A22);           // heap paragraph
assert_eq!(recs[0].mode_flag, 0);                  // opaque blit
let stream_byte_ofs = usize::from(recs[0].ofs_para.wrapping_sub(0x3A22)) * 16;
```

You normally don't call `parse_index`/`fix_up` directly — `assets::load_set`
does both and returns the records inside an `AssetSet` (see
[assets.md](assets.md)); `AssetSet::stream` then undoes the fixup to slice
each record's bytes out of the `.BIN` blob.

## Pitfalls

- **Paragraphs, not bytes.** Forgetting the `* 16` puts you 15/16 short
  of the stream — the single most common mistake when hand-rolling this
  path. `AssetSet::stream` already does it.
- **`ofs_para` is relocated after `fix_up`.** Code that treats a fixed
  record's `ofs_para` as a file offset is off by `heap_seg * 16`; code
  that treats an *unfixed* one as a segment base is off the other way.
  Know which side of `fix_up` your records are on.
- **`w` is mode-dependent.** It is bytes-per-row in that mode's VRAM
  layout (MCG 320, EGA 40/plane, CGA 80, TGA 160) — not pixels. The
  same image in different sets has different `w`/`h` values.
- **`mode_flag ≥ 2` draws nothing** by design — the asm has no third
  store op.

## File map

| file | item |
|---|---|
| `mod.rs` | module docs, re-exports |
| `record.rs` | `Record` struct + `RECORD_LEN = 12` |
| `record_ofs.rs` | field offsets + `Record::is_xor` |
| `parse.rs` | `parse_index` — LE u16 sextets → `Vec<Record>` |
| `fixup.rs` | `fix_up` — the `add [si+0x160],ax` loop |

## Where the records come from

`MCG.GPH`'s 356 records cover the title backdrop, panel frames, planet
sprites, UI chrome — anything the game draws by index. Records aren't
numbered semantically; a doc that says "record `0x12B`" means "the
352nd (12-byte) entry's stream", and which image that is comes from
the asset toolchain, not the format. `extract_png.py` in
`decompiled/tools/` is the reference decoder for eyeballing one.
