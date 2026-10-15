# `assets` — `.BIN`/`.GPH` asset-set loading

## What this module does

`assets` loads one of the game's graphics sets: it reads the `STEM.GPH`
index (see [gph.md](gph.md)), reads the whole `STEM.BIN` compressed-image
blob, performs the paragraph fixup the original does, and returns an
`AssetSet` that mirrors the post-loader memory state — records plus the
heap they point into. `AssetSet::stream(idx)` then hands each record's
compressed bytes to the LZSS decoder.

The unit to hold in your head: a **set** is a stem plus a suffix —
`MCG` + `""`/`IN`/`DTH`/`WIN` gives `MCG.BIN`/`MCG.GPH`, `MCGIN.BIN`/
`MCGIN.GPH`, and so on. The game keeps *one* set resident at a time and
swaps it at scene boundaries (title vs endings vs gameplay).

## Background — how the original loads a set

Port of the loader pair in `GAME.unpacked.exe`: the `.GPH` index read +
in-place paragraph fixup (`0x3208E`–`0x320E9`) and the chunked `.BIN`
heap load (`0x320F2`–`0x3215A`).

- The filename buffers at `ds:0x1242`/`ds:0x124D` are `???.???`
  templates the video-mode select patches with `MCG`/`EGA`/`CGA`/`TGA` +
  `.BIN`/`.GPH` — that is why each mode ships its own asset set. Here
  the stem is just a parameter.
- The `.BIN` lands on a heap computed as
  `((0x9CC4 − 0x7D39) >> 4) + 1 + 0x3829` = `0x3A22` — the paragraph
  right after the loaded image's data area. Read the formula as: "the
  save block spans `0x7D39..0x9CC4` (`0x1F8B` bytes); that is
  `0x1F9` paragraphs, round up one, and add it to the image's data
  start `0x3829`" — i.e. *the heap is the memory immediately past the
  state block*, which is where the game can afford to drop hundreds of
  KB of compressed art.
- The `.GPH` offsets are file-relative paragraphs; the loader adds the
  heap segment to each in place so the renderer can treat
  `seg:ofs_para` as a far pointer ([gph.md](gph.md) explains the
  paragraph unit and why the fixup happens once, in place).
- DOS reads the `.BIN` in `0xFFF0`-byte chunks (`int 21/3F` with
  `cx=0xFFF0`, bumping `ds` by `0xFFF` paragraphs each pass) because a
  single read can't cross a 64 KiB boundary; flat host memory makes that
  one `fs::read`. (This is a recurring shape in the port: a loop that
  exists only because of a 64 KiB segment boundary collapses to one
  call.)
- On a failed open the asm re-prompts via the insert-disk handler at
  `0x8AC0`; here `fs` errors simply propagate.

## Step by step — `load_set`

1. `load_index(dir/STEM.GPH, heap_seg)` reads up to `0x10B0` bytes of
   index (the asm's fixed read — a short file is fine), parses each
   12-byte record, and runs `gph::fix_up` so every `ofs_para` gains
   `heap_seg`.
2. `load_bin(dir/STEM.BIN)` reads the entire blob — all the set's
   LZSS streams concatenated, 16-byte aligned.
3. `AssetSet { records, bin, heap_seg }` is the post-load memory state:
   the index already relocated, the heap as a flat `Vec`.

The asymmetry is deliberate: the index knows where it was relocated
*to*, so `stream(idx)` can undo it (`(ofs_para − heap_seg) * 16`) to
slice the `Vec`.

## API tour

- `heap_segment() -> u16` (`heap.rs`) — the `0x3A22` paragraph above.
- `load_index(path, heap_seg) -> io::Result<Vec<Record>>` (`load_index.rs`)
  — reads `STEM.GPH`, truncates to `0x10B0` bytes (≤356 records, short
  reads fine), parses the index and runs the `0x320D8` fixup adding
  `heap_seg` to every `ofs_para`.
- `load_bin(path) -> io::Result<Vec<u8>>` (`load_bin.rs`) — the whole
  `.BIN` payload.
- `load_set(dir, stem, heap_seg) -> io::Result<AssetSet>` (`load_set.rs`)
  — `STEM.GPH` + `STEM.BIN` → `AssetSet`, matching how the game loads
  the `MCG`/`EGAIN`/`CGAWIN` sets.

`AssetSet` (`set.rs`) mirrors the post-loader memory state:

- `records: Vec<Record>` — the `ds:0x158` index, `ofs_para` already
  relocated.
- `bin: Vec<u8>` — the whole `.BIN` payload: all the set's compressed
  streams concatenated on 16-byte boundaries.
- `heap_seg: u16` — the load segment, kept so stream slicing can undo
  the fixup.

`AssetSet::stream(idx)` (`stream.rs`) — record `idx`'s compressed
stream: `bin[(ofs_para − heap_seg) * 16 ..]` to end of file. The slice
runs to EOF, not to the next record — the LZSS decoder stops at the
stream's own `00 00` terminator, so an over-long tail is harmless and
the asm relies on the same thing. EGA records hold four consecutive
plane streams (see [lzss.md](lzss.md)).

## The other callers — `front/load.rs`

`front/load.rs` (`Call::Load`, `Host::set_assets`) drives the same
loaders the way the game itself does at runtime: it passes a `ds:0x1208`+
filename cell and the loader resolves it to `{stem}{sfx}.{BIN,GPH}`:

| cell | resolves to | when the game calls it |
|---|---|---|
| `0x1208` | `{stem}.BIN` | base gameplay set |
| `0x1210` | `{stem}.GPH` | (index half of the same) |
| `0x1218`/`0x1222` | `{stem}IN.BIN`/`.GPH` | intro/title set |
| `0x122C`/`0x1237` | `{stem}DTH.BIN`/`.GPH` | defeat ending |
| `0x1242`/`0x124D` | `{stem}WIN.BIN`/`.GPH` | victory ending |

The asm loads `.BIN` then `.GPH` at every call site, so the loader
stashes the `.BIN` payload and swaps the active `AssetSet` when the
`.GPH` half lands — that is how the endings' `*DTH`/`*WIN` sets and the
restart tail's `*IN`/base pairs come in (GAM-33). The `0x8AC0`
insert-disk retry is dropped (`dropped` tally) — a missing file in the
port is a counted skip, not a modal "insert disk" prompt.

## Example

```rust
let set = supremacy::assets::load_set(
    std::path::Path::new("GAME"), "MCG", supremacy::assets::heap_segment(),
)?;
println!("{} records, {} bytes of streams", set.records.len(), set.bin.len());
let lzss_stream = set.stream(0);   // compressed bytes of record 0
```

## Pitfalls

- **`heap_seg` must match.** `stream()` only returns the right bytes if
  the `heap_seg` used at `fix_up` equals the one stored on the set —
  always use `heap_segment()` (or the value you passed `load_index`)
  rather than a hardcoded `0x3A22`.
- **The stem is three letters.** `"MCG"`, `"EGA"`, `"CGA"`, `"TGA"` —
  suffixes (`"MCGIN"`) are formed by the caller, not by `load_set`.
- **`stream(idx)` runs to EOF.** Slice it further only if you know the
  record's decoded length — the terminator, not the next record's
  offset, bounds a stream.
- **Errors are `io::Result`, not the `0x8AC0` prompt.** On disk the
  original retries forever; the port surfaces the failure to the caller.

## File map

| file | item |
|---|---|
| `mod.rs` | module docs, re-exports |
| `heap.rs` | `heap_segment()` — the `0x3A22` formula |
| `load_index.rs` | `.GPH` read + parse + fixup |
| `load_bin.rs` | `.BIN` whole-file read |
| `load_set.rs` | `load_set` — the pair → `AssetSet` |
| `set.rs` | `AssetSet` struct |
| `stream.rs` | `AssetSet::stream` — fixup undo + slice |

## Worked example — computing where record 9's stream lives

With `heap_seg = 0x3A22` and record 9's `ofs_para = 0x100` on disk:

- After `fix_up`: `ofs_para = 0x100 + 0x3A22 = 0x3B22`.
- `AssetSet::stream(9)` undoes it: `(0x3B22 − 0x3A22) * 16 = 0x1000` —
  byte `0x1000` of `bin` is the stream's first byte (a flags byte for
  the LZSS decoder — see [lzss.md](lzss.md)).
- The record's `x`/`y`/`w`/`h`/`mode_flag` are untouched by fixup —
  only `ofs_para` is relocated.
