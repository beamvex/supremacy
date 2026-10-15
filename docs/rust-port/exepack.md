# `exepack` — Microsoft EXEPACK unpacker

## What this module does

`GAME.EXE` on disk is not the program that runs — it is
**EXEPACK**-compressed (Microsoft's early-90s executable packer). A tiny
decompression stub sits at the entry point; on boot it expands the real
image in place, applies segment relocations, and jumps to the real code.

`exepack` reproduces that entire process in safe Rust: given the packed
`.exe` bytes it produces the unpacked load image plus the real entry
registers — exactly the `decompiled/GAME.unpacked.exe` every other doc
cites. It also ports the relocation-table parser and can re-emit a valid
`.exe` (`to_exe`), which the test suite round-trips against.

Entry points: `unpack(file) -> Result<Unpacked, Error>`,
`apply_relocs`, `reloc_list`, `Unpacked::to_exe`. The stub ported lives
at `GAME.EXE:0x23A90`.

## Background — the format

An EXEPACK `.exe` stores its load image with the **head uncompressed
and the tail compressed**. At the entry `CS` (MZ field `[0x16]`) sits:

```
block (entry_cs * 16 bytes into the load image):
┌──────────────────────────────────────────────┐
│ 16-byte 'RB' header     (fields below)       │
│ stub code               (the decompressor)   │
│ packed command stream   (grows downwards)    │
│ packed relocation stream                     │
└──────────────────────────────────────────────┘
```

The stub copies its block to the top of a `dest_len`-paragraph buffer,
then decompresses **backwards** (`std`, `DF=1` — the direction flag
makes every string op auto-decrement): writes always land above the
read cursor because compressed data is smaller, so in-place expansion
is safe (see [concepts.md](concepts.md) §5 for the MZ + relocation
background). That last fact is the whole trick: you can expand a buffer
in place only if each write lands at an address *higher* than the
remaining input — backwards-from-the-top guarantees it.

The command stream is likewise stored backwards: the last command byte
sits at the highest address and each command's operands lie *below* its
opcode, so the stub walks `si` downwards decoding
`cmd @ [si]`, `len @ [si-2]`, and the payload below that.

## The 16-byte `'RB'` header

`Header` (`header.rs`) — signature-checked in `Header::parse`
(`header_parse.rs`; `'RB'` at `+0xE`; missing → `Error::MissingPack`):

| off | field | use |
|---|---|---|
| `0x0` | `real_ip` | far-jmp target after unpacking |
| `0x2` | `real_cs` | relocated `+ dest_base` at `0x23B62` |
| `0x6` | `pack_size` | header + stub + fixups byte size |
| `0x8`/`0xA` | `real_sp`/`real_ss` | entry stack (`+ dest_base` on SS) |
| `0xC` | `dest_len` | destination size in paragraphs |

`real_*` are the registers the *real* program starts with — the stub is
a trampoline: unpack, relocate, load `ss:sp`/`cs:ip` from this header,
far-jump. `dest_len` is how big the buffer must be before decompression
begins (the expanded image size rounded up to paragraphs).

`Mz::parse` (`mz_parse.rs`) validates `MZ`/`ZM`, computes the image byte
range as `pages*512 − pad − hdr*16` from `[0x02]`/`[0x04]`/`[0x08]`, and
reads `entry_cs` at `[0x16]`. Errors: `BadMz`.

## `unpack` — step by step (`unpack.rs`)

1. `Mz::parse` → image slice; `block = img[entry_cs*16..]` — find the
   EXEPACK block at the entry CS.
2. `Header::parse(block)` → sizes + entry registers.
3. `dest = vec![0; dest_len*16]`; seed `dest[..img.len()]` with the
   packed image — the head is stored uncompressed, so pre-filling it
   reproduces the in-place algorithm's starting state byte-for-byte.
   (In the asm the image is already in memory under the stub; the port
   models that by copying it into the destination buffer first.)
4. `si = stream_end(img, blk_ofs)` (`scan.rs`) — reproduces the stub's
   `repe scasb` (`al=0xFF`, `cx=0x10`, `DF=1`): scans the ≤16 bytes
   below the EXEPACK block for the last non-`0xFF` byte = the final
   command. The command stream grows *downwards* from there — the packer
   pads the gap with `0xFF`, so the topmost non-`0xFF` byte is the
   first command the stub must execute.
5. `di = dest.len()-1`; `commands::run` walks the stream backwards.

## Command blocks (`read_cmd.rs`, `commands.rs`)

Each command decodes at `si` (backwards): `[si]` = cmd byte,
`[si-2]` = `len`, fill byte at `[si-3]` for `0xB0`. `cmd & 0xFE`:
`0xB0` = `Fill { byte, len }`, `0xB2` = `Copy { src_end, len }`
(`src_end` = highest source address). `cmd & 1` marks the last block
(`Cmd::last` in `cmd.rs`/`cmd_last.rs`, `test al,1` at `0x23B12`).

- `fill` (`fill.rs`) writes `byte` descending `di..di-len+1` — a
  run-length fill for the zero/space expansions.
- `copy` (`copy.rs`) does `buf[di-k] = buf[src_end-k]` for
  `k in 0..len` then `di -= len` — reading from already-expanded or
  still-packed bytes exactly like the asm. Because both cursors go
  *down*, a copy may pull from bytes it just wrote — the same
  self-referential overlapping trick the LZSS ring exploits
  ([lzss.md](lzss.md)).

A small example of the stream's shape (highest address first):

```
[si]   = 0xB3        ← cmd: 0xB2 copy + "last" bit
[si-2] = 0x0400      ← len = 1024
[si-4..]             ← 1024 packed source bytes
... earlier commands below ...
```

Both commands are bounds-checked; `Error::Corrupt(cmd)` = the asm's
corrupt-file print at `0x23B79` (the "Packed file is corrupt" message —
which doubles as the landmark `reloc_ofs` uses to find the relocation
table, since the packed table follows it immediately in the block).

## Relocations

`reloc_list` (`reloc_list.rs`) ports the `0x23B22`–`0x23B44` parser: the
packed stream starts after the `"Packed file is corrupt"` message
(`reloc_ofs.rs` locates it, falling back to GAME's `si=0x125`). It holds
16 batches, one per `0x1000` paragraphs (`dx = 0..=0xF000`): a count word
then `count` `u16` offsets. Each site decodes to linear offset
`dx*16 + ofs` — i.e. batch `dx` holds the offsets whose paragraph is in
`[dx, dx+0x1000)`; multiplying back out gives the flat byte offset in
the image. The `0xFFFF` entry is just uncanonicalised `seg:ofs`
(`0xFFFF:0x0001` = `0xFFFF1` = `0xFFF:0xFFFF` — same byte; the batch
boundary is arbitrary).
Truncated tables yield the entries decoded so far (infallible) — the asm
reads until the table runs out, so the port does the same rather than
erroring.

`apply_relocs` (`apply_relocs.rs`) performs the stub's
`add [es:di],bx` fixups (`0x23B35`) — adds `base` to the `u16` at each
site. On-disk `.exe`s store only the table; the stub does this before
the far jump. (The port exposes the sites as *linear* offsets; the asm
thinks of them as `es:di` pairs over the unpacked image.)

## Repacking

`Unpacked::to_exe` (`to_exe.rs`) emits a valid `.exe`: `mz_write`
(`mz_write.rs`) writes a 28-byte header — `MZ`, page geometry
(`total % 512`, `div_ceil`), reloc count, header paras, `0x74` minalloc,
`0xFFFF` maxalloc, `ss`/`sp`/`0`/`ip`/`cs`, reloc table offset `0x1C` —
followed by the canonical `seg:ofs` (`r>>4`, `r&0xF`) fixup table, zero
padding to the header paragraph boundary, and the unrelocated image.
`sat()` (`sat.rs`) saturates `usize→u16` header fields — header fields
are 16-bit, so an image field that can't fit saturates rather than
wrapping.

Note what `to_exe` produces: an *unpacked* exe — a valid MZ that DOS can
load directly, not a re-compressed EXEPACK file. The round-trip test
cares about the image + relocation table being right, not about
re-running the packer.

`Unpacked` (`unpacked.rs`): `image` (decompressed, pre-fixup), `cs`/`ip`/
`ss`/`sp` (unrelocated), `relocs` (linear sites).

`Error` (`error.rs`/`error_display.rs`): `BadMz`, `MissingPack`,
`Corrupt(u8)`; `Display` mirrors the stub messages. `word.rs` is the
shared bounds-checked LE `u16` read. `consts.rs` holds `SIG`, the
corrupt message, `FALLBACK_TABLE = 0x125`, `SCAN_MAX = 16`.

## Example

```rust
let packed = std::fs::read("GAME.EXE")?;
let up = supremacy::exepack::unpack(&packed)?;
std::fs::write("GAME.unpacked.exe", &up.image)?;   // the image the docs cite
// A DOS-loadable exe, if you want one back:
let exe = up.to_exe();
```

Most users never call this — the regions the port reads are already
embedded as `rust/data/*.bin` (GAM-55) and served by `img::image()` —
but `unpack` is the blobs' provenance (`extract-img` re-slices them
from `GAME.EXE`) and the reference for any future packed file (e.g.
the `SUPCHT.EXE` fixture is a second packed exe from the same era).

## Pitfalls

- **The stream is backwards.** Every offset in the command block is
  relative to a `si` that *decreases*: `[si]` opcode, `[si-2]` length,
  payload below. Forward-reading it looks like garbage.
- **`dest_len` is paragraphs.** Multiply by 16 — the destination buffer
  is `dest_len * 16` bytes, always 16-aligned.
- **Pre-fill matters.** `unpack` must seed the destination with the
  packed image before running commands — the head of the image is
  stored uncompressed and copy commands may reference it.
- **`relocs` are image offsets**, not file offsets — they're only
  meaningful against `Unpacked::image`.
- **`MissingPack` ≠ `BadMz`.** A valid MZ without the `'RB'` tag means
  "not EXEPACKed" — probably already unpacked.

## Verification

`tests/exepack_unpack.rs` unpacks `tests/fixtures/SUPCHT.EXE` (a second
packed exe from the same era) and cross-checks `to_exe` repacking with a
minimal MZ parser — the round trip proves both the decompression and the
header writer.

## File map

| file | item |
|---|---|
| `mod.rs` | module docs, re-exports |
| `mz.rs` / `mz_parse.rs` / `mz_write.rs` | MZ header struct, reader, writer |
| `header.rs` / `header_parse.rs` | the 16-byte `'RB'` EXEPACK header |
| `scan.rs` | `stream_end` — the `repe scasb` `0xFF`-pad scan |
| `read_cmd.rs` | one backwards command decode |
| `cmd.rs` / `cmd_last.rs` | `Cmd::{Fill,Copy}` + `last` bit |
| `commands.rs` | `run` — the decompress loop |
| `fill.rs` / `copy.rs` | the two command executors |
| `unpack.rs` | `unpack(file) -> Unpacked` orchestration |
| `unpacked.rs` | `Unpacked` struct |
| `reloc_list.rs` / `reloc_ofs.rs` | packed relocation-table parse |
| `apply_relocs.rs` | `add base` fixups |
| `to_exe.rs` | re-emit a loadable `.exe` |
| `sat.rs` | saturating `usize→u16` |
| `word.rs` | bounds-checked LE `u16` read |
| `consts.rs` | `SIG`, corrupt message, fallbacks |
| `error.rs` / `error_display.rs` | `Error` enum + messages |

## Reading a packed header by hand

For a real `GAME.EXE`: `Mz::parse` reads `[0x04]` = page count and
`[0x16]` = entry CS. `block = image[entry_cs*16..]` starts with the
16-byte header — `[0x0C]`/`[0x0E]` is `dest_len` in paragraphs, and
`block[0xE..0x10]` is `'RB'`. If you `xxd` the file at
`entry_cs*16 + 0xE` and don't see `52 42`, the file isn't EXEPACKed —
`Header::parse` returns `Error::MissingPack` on exactly that check.
