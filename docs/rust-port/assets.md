# `assets` — `.BIN`/`.GPH` asset-set loading

Port of the loader pair in `GAME.unpacked.exe`: `.GPH` index read +
in-place paragraph fixup (`0x3208E`–`0x320E9`) and the chunked `.BIN`
heap load (`0x320F2`–`0x3215A`). The `ds:0x1242`/`ds:0x124D` filename
buffers are `???.???` templates the video-mode select patches with
`MCG`/`EGA`/`CGA`/`TGA` + `.BIN`/`.GPH`; here the stem is a parameter.

- `heap_segment()` (`heap.rs`) — the loader's heap-paragraph formula
  `((0x9CC4 − 0x7D39) >> 4) + 1 + 0x3829` = `0x3A22`: the paragraph right
  after the loaded image where `.BIN` data lands.
- `load_index(path, heap_seg)` (`load_index.rs`) — reads `STEM.GPH`,
  truncates to `0x10B0` bytes (`mov cx,0x10B0` at `0x320AC`; ≤356
  records, short reads fine), parses the index and runs the `0x320D8`
  fixup adding `heap_seg` to every `ofs_para`. Asm retries via the
  disk-swap prompt at `0x8AC0`; here `fs::read` errors propagate.
- `load_bin(path)` (`load_bin.rs`) — the asm loops `int 21/3F` with
  `cx=0xFFF0`, bumping `ds` by `0xFFF` paragraphs per chunk until a short
  read; flat host memory makes it a single `fs::read`.
- `load_set(dir, stem, heap_seg)` (`load_set.rs`) — `STEM.GPH` +
  `STEM.BIN` → `AssetSet`, matching how the game loads `MCG`/`EGAIN`/
  `CGAWIN` sets.

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
plane streams.
