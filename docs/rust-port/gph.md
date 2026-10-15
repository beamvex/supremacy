# `gph` — `.GPH` graphics index

## What this module does

Every asset set the game ships comes as a file pair: a `*.BIN` blob of
compressed image streams and a `*.GPH` index describing each image
inside it. `gph` parses that index: it turns the raw `.GPH` bytes into a
vector of `Record`s telling you where each picture goes on screen, how
big it is, where its compressed stream sits in the `.BIN`, and whether
to draw it opaquely or XOR it over the background.

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

`ofs_para` is the format's one DOS-ism: the offset is counted in
**paragraphs** (16-byte units, see [concepts.md](concepts.md) §1), so the
stream for a record with `ofs_para = 0x100` begins at byte `0x1000` of
the `.BIN`. On disk the offsets are relative to the start of the `.BIN`;
at load time the game adds the heap segment where it loaded the blob so
`seg:ofs_para` becomes a far pointer — that addition is the "fixup".

`mode_flag` chooses the store op the decoder uses: `0` overwrites
background bytes (`stosb`), `1` XORs pixels on top (`xor [es:di],al`) —
the game's cheap transparency effect for overlays. Any other value draws
nothing at all.

## The asm being ported

The loader reads exactly `0x10B0` bytes of index (`mov cx,0x10B0` at
file `0x320AC` — room for 356 records; a short read is fine), then the
loop at `0x320D8` walks the records with `add si,0xC` (the 12-byte
stride) and executes `add [si+0x160],ax` — adding the `.BIN` heap
segment to every `ofs_para` in place.

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
