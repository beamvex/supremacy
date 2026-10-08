# `lzss` — Okumura LZSS decoder

Port of the four near-identical decoder routines in `GAME.unpacked.exe`
(`N=4096`, `F=18`, `THRESHOLD=2`):

| file offset | modes | row stride | store op | trailing byte |
|---|---|---|---|---|
| `0x2EBEF` | MCG/TGA/CGA | `0x140` | `stosb` | not consumed |
| `0x2EC8C` | MCG/TGA/CGA | `0x140` | `xor [es:di],al` | not consumed |
| `0x2FC5D` | EGA | `0x28` | `stosb` | consumed (`lodsb` @ `0x2FCF7`) |
| `0x2FCF9` | EGA | `0x28` | `xor [es:di],al` | consumed (`lodsb` @ `0x2FD9A`) |

## Decoder state (`decoder.rs`, `consts.rs`)

- `ring: [u8; 0x1000]` — the `SS:0x0000` history buffer, zeroed
  (`decoder_new.rs` = the `mov cx,0x7F7` clear loop).
- `r` — write cursor `bp`, starts `0xFEE` (`N − F`); `push` stores the
  emitted byte and wraps `& 0xFFF` (`push.rs`).
- `flags: u16` — the `dx` flag register. `next_flag` shifts right one
  bit; when the `0x100` sentinel has shifted out it reloads
  `dl` from the stream with `dh = 0xFF` (`FLAG_HI`). Bit value `1` =
  literal, `0` = match.

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

`trait Emit { fn emit(&mut self, b: u8) }` abstracts the asm's
`stosb`/`xor [es:di],al` store so one decoder drives flat buffers and
VRAM-shaped destinations.

- `Vec<u8>` — flat append sink.
- `Frame<'a>` (`frame.rs`/`frame_new.rs`/`frame_emit.rs`) — the
  strided-VRAM sink: `{buf, di, width, stride, xor, row_left}`. Each byte
  stores or XORs at `di`, `di += 1`; when `row_left` hits zero it reloads
  to `width` and `di += stride − width` — the `[cs:0x5380]` down-counter
  + row-wrap the asm does at `0x2EC31`/`0x2FC9F`.

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
