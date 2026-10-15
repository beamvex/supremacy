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

- a 16-byte `'RB'` header,
- the stub code,
- the packed command stream (backwards),
- the packed relocation stream.

The stub copies its block to the top of a `dest_len`-paragraph buffer,
then decompresses **backwards** (`std`, `DF=1`): writes always land
above the read cursor because compressed data is smaller, so in-place
expansion is safe (see [concepts.md](concepts.md) §5 for the MZ +
relocation background).

## The 16-byte `'RB'` header

`Header` (`header.rs`) — signature-checked in `Header::parse` (`'RB'` at
`+0xE`; missing → `Error::MissingPack`):

| off | field | use |
|---|---|---|
| `0x0` | `real_ip` | far-jmp target after unpacking |
| `0x2` | `real_cs` | relocated `+ dest_base` at `0x23B62` |
| `0x6` | `pack_size` | header + stub + fixups byte size |
| `0x8`/`0xA` | `real_sp`/`real_ss` | entry stack (`+ dest_base` on SS) |
| `0xC` | `dest_len` | destination size in paragraphs |

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
4. `si = stream_end(img, blk_ofs)` — `scan.rs` reproduces the stub's
   `repe scasb` (`al=0xFF`, `cx=0x10`, `DF=1`): scans the ≤16 bytes below
   the EXEPACK block for the last non-`0xFF` byte = the final command.
   The command stream grows *downwards* from there.
5. `di = dest.len()-1`; `commands::run` walks the stream backwards.

## Command blocks (`read_cmd.rs`, `commands.rs`)

Each command decodes at `si` (backwards): `[si]` = cmd byte,
`[si-2]` = `len`, fill byte at `[si-3]` for `0xB0`. `cmd & 0xFE`:
`0xB0` = `Fill { byte, len }`, `0xB2` = `Copy { src_end, len }`
(`src_end` = highest source address). `cmd & 1` marks the last block
(`Cmd::last`, `test al,1` at `0x23B12`).

- `fill` writes `byte` descending `di..di-len+1`.
- `copy` does `buf[di-k] = buf[src_end-k]` for `k in 0..len` then
  `di -= len` — reading from already-expanded or still-packed bytes
  exactly like the asm.

Both are bounds-checked; `Error::Corrupt(cmd)` = the asm's
corrupt-file print at `0x23B79`.

## Relocations

`reloc_list` (`reloc_list.rs`) ports the `0x23B22`–`0x23B44` parser: the
packed stream starts after the `"Packed file is corrupt"` message
(`reloc_ofs.rs` locates it, falling back to GAME's `si=0x125`). It holds
16 batches, one per `0x1000` paragraphs (`dx = 0..=0xF000`): a count word
then `count` `u16` offsets. Each site decodes to linear offset
`dx*16 + ofs`; the `0xFFFF` entry is just uncanonicalised `seg:ofs`
(`0xFFFF:0x0001` = `0xFFFF1` = `0xFFF:0xFFFF` — same byte).
Truncated tables yield the entries decoded so far (infallible).

`apply_relocs(image, relocs, base)` performs the stub's
`add [es:di],bx` fixups (`0x23B35`) — adds `base` to the `u16` at each
site. On-disk `.exe`s store only the table; the stub does this before
the far jump.

## Repacking

`Unpacked::to_exe` (`to_exe.rs`) emits a valid `.exe`: `mz_write`
(`mz_write.rs`) writes a 28-byte header — `MZ`, page geometry
(`total % 512`, `div_ceil`), reloc count, header paras, `0x74` minalloc,
`0xFFFF` maxalloc, `ss`/`sp`/`0`/`ip`/`cs`, reloc table offset `0x1C` —
followed by the canonical `seg:ofs` (`r>>4`, `r&0xF`) fixup table, zero
padding to the header paragraph boundary, and the unrelocated image.
`sat()` saturates `usize→u16` header fields.

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

## Verification

`tests/exepack_unpack.rs` unpacks `tests/fixtures/SUPCHT.EXE` (a second
packed exe from the same era) and cross-checks `to_exe` repacking with a
minimal MZ parser — the round trip proves both the decompression and the
header writer.
