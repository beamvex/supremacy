# `assets` — `.BIN`/`.GPH` asset-set loading

## What this module does

`assets` loads one of the game's graphics sets: it reads the `STEM.GPH`
index (see [gph.md](gph.md)), reads the whole `STEM.BIN` compressed-image
blob, performs the paragraph fixup the original does, and returns an
`AssetSet` that mirrors the post-loader memory state — records plus the
heap they point into. `AssetSet::stream(idx)` then hands each record's
compressed bytes to the LZSS decoder.

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
  right after the loaded image's data area (the `0x9CC4`/`0x7D39` are
  the state block's end and start in `ds:` space).
- The `.GPH` offsets are file-relative paragraphs; the loader adds the
  heap segment to each in place so the renderer can treat
  `seg:ofs_para` as a far pointer ([gph.md](gph.md) explains the
  paragraph unit).
- DOS reads the `.BIN` in `0xFFF0`-byte chunks (`int 21/3F` with
  `cx=0xFFF0`, bumping `ds` by `0xFFF` paragraphs each pass) because a
  single read can't cross a 64 KiB boundary; flat host memory makes that
  one `fs::read`.
- On a failed open the asm re-prompts via the insert-disk handler at
  `0x8AC0`; here `fs` errors simply propagate.

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
stream: `bin[(ofs_para − heap_seg) * 16 ..]` to end of file. Decoders
stop at each stream's own terminator; EGA records hold four consecutive
plane streams (see [lzss.md](lzss.md)).

## The other callers — `front/load.rs`

`front/load.rs` (`Call::Load`, `Host::set_assets`) drives the same
loaders the way the game itself does at runtime: it passes a `ds:0x1208`+
filename cell (`*`, `*IN`, `*DTH`, `*WIN` × `.BIN`/`.GPH`), the loader
resolves it to `{stem}{sfx}.{BIN,GPH}` and swaps the active `AssetSet`
when the `.GPH` half lands — that is how the endings' `*DTH`/`*WIN` sets
and the restart tail's `*IN`/base pairs come in (GAM-33). The `0x8AC0`
insert-disk retry is dropped (`dropped` tally).

## Example

```rust
let set = supremacy::assets::load_set(
    std::path::Path::new("GAME"), "MCG", supremacy::assets::heap_segment(),
)?;
println!("{} records, {} bytes of streams", set.records.len(), set.bin.len());
let lzss_stream = set.stream(0);   // compressed bytes of record 0
```
