# Concepts primer — what you need to know before reading the port docs

The `rust/` crate re-implements the machine code of *Supremacy: Your Will
Be Done* (Probe Software, 1991) — a 16-bit DOS program. The per-module
docs assume you know some 1980s–90s PC vocabulary. This page explains
every concept those docs use, in the order you are most likely to need
it. Read §1–§3 first; dip into the rest when a doc mentions something
unfamiliar.

Nothing here requires prior reverse-engineering experience — only enough
Rust to recognise `u16`, slices, and traits.

---

## 1. Segmented memory (`seg:offset`)

An 8086 CPU has 16-bit registers but a 20-bit (1 MiB) address bus. It
bridges the gap with **segments**: every memory access is computed as

```
physical address = (segment_register * 16) + offset
```

The segment registers are named `cs` (code), `ds` (data), `es` (extra),
`ss` (stack). Assembly operands like `mov ax,[0x1258]` mean "read a word
at `ds * 16 + 0x1258`".

Two consequences you will meet constantly:

- **A "paragraph" is 16 bytes** — the amount one segment unit covers.
  Addresses stored in paragraphs (e.g. `.GPH` `ofs_para`) must be
  multiplied by 16 to become byte offsets.
- Many different `seg:ofs` pairs address the same byte.
  `0xA000:0x0000` and `0x9000:0x8000` are the same VGA memory. The port
  mostly ignores segments and works in **flat `ds:`/`cs:` offsets** —
  the `u16` offset part only.

In these docs, `ds:0x7D39` means "the byte at offset `0x7D39` inside the
game's data segment" and `cs:0x773D` "offset `0x773D` inside its code
segment". Square brackets mean "the value stored at": `[0x91B8]` is the
word held at `ds:0x91B8`. `+0xC` after a record means "the field 12
bytes into that record".

## 2. The three address spaces in the docs

Every hex number in the docs and code comments is a real address in one
of three spaces (also documented in `decompiled/FUNCTIONS.md`):

| space | meaning | convert to file offset |
|---|---|---|
| file offset | byte offset in `decompiled/GAME.unpacked.exe` | — |
| `cs:` near offset | offset inside the main code segment | `cs + 0x292B0` |
| `ds:`/`dgroup` offset | offset inside the data segment | `ds + 0x106A0` |

Worked example: the docs say the state block lives at `ds:0x7D39`. To
find those bytes in the file on disk, compute
`0x7D39 + 0x106A0 = 0x183D9` — which is exactly where
`State::from_image` reads them.

The port keeps game data addressed by `ds:` offsets exactly as the asm
uses them, so a `[0x91B8]` in a comment and the `0x91B8` in the code are
the *same* number — no mental arithmetic needed when comparing. The
constants `0x106A0` (`DS_BASE`) and the `cs:` bases (`0x292B0` game code,
`0x30000` how the frontend maps it) do the file↔segment translation in
one place.

## 3. Software interrupts and I/O ports

DOS programs call the OS and hardware through two mechanisms:

- **`int n` — software interrupts.** A function call where the number
  selects the service and registers carry arguments:

  | interrupt | service | used here for |
  |---|---|---|
  | `int 21h` | DOS kernel | file open/read/write/close (`AH=3D/3F/40/3E/4E/3C`) |
  | `int 10h` | BIOS video | set mode (`AH=0`), program DAC palette (`AX=1012`) |
  | `int 33h` | mouse driver | position, ranges, event mask (`ax=4/8/0x14`) |
  | `int 8` | hardware timer | fires 18.2 times/sec from PIT channel 0 |
  | `int 9` | keyboard hardware | fires per keypress, delivering a scancode |
  | `int 80h` | the game's own | its private sound-driver API (`ah` = command) |

  `int 8`/`int 9` are *hardware* interrupts — they fire asynchronously;
  the game hooks their vectors to run its own handlers. The port turns
  these into explicit method calls: `Timer::tick()` is "one int-8 pulse
  happened", `Keyboard::irq(scan)` is "one int-9 fired".

- **`in`/`out` — I/O ports.** A separate 64 KiB address space talking
  directly to chips. `out 0x388, reg` / `out 0x389, val` writes an OPL2
  register; `in al,0x60` reads a keyboard scancode. In the port every
  `in`/`out` goes through the two-method `audio::Ports` trait
  (`outb`/`inb`), so a test can record the exact byte sequence the asm
  would have emitted.

Carry flag (`CF`) is DOS's error return: wrappers report failure by
setting it. The port mirrors that with `Option`/`bool` — `None`/`false`
everywhere a wrapper "returned with carry set".

## 4. The PSP command tail

When DOS launches `GAME M M A`, it places a **Program Segment Prefix**
before the program and copies the typed tail into it at `es:0x80`: a
count byte, a space, then the characters. The game's argument parser
doesn't scan the string — it reads three fixed cells: `es:0x82` (video
letter), `es:0x84` (keyboard flag), `es:0x86` (sound letter). That is
why `args::parse` takes a byte slice laid out the same way, and why the
binaries' `tail_bytes` helper rebuilds a synthetic tail from separate
CLI flags.

## 5. The `.exe` on disk: MZ + EXEPACK + relocation

A DOS `.exe` starts with an **MZ header** ("MZ" signature, sizes, entry
`cs:ip`, `ss:sp`, and a relocation table). DOS loads the image, then
adds the load segment to every word the table points at — **segment
relocations** — before jumping to the entry point.

`GAME.EXE` is also **EXEPACK**-compressed (Microsoft's 1990-era packer):
the tail of the load image is a tiny decompressor plus a backwards
command stream. On boot it copies itself high, then runs `std`/`rep
movsb` *backwards* expanding the image in place — writes always land
above the read cursor because compressed data is smaller. After
expansion it applies its own relocation table and far-jumps to the real
entry point. `exepack::unpack` reproduces that whole dance in safe Rust;
`decompiled/GAME.unpacked.exe` is its output.

## 6. LZSS in one paragraph

LZSS (Okumura flavour) is a dictionary compressor: the decoder keeps a
4 KB **ring buffer** of recently output bytes. A flag bit per token says
"literal" (copy the next byte out) or "match" (a 2-byte token says
*position* + *length* — copy `len` bytes from the ring and emit them,
feeding them back into the ring). `GAME.EXE` ships four near-identical
decoders that differ only in row stride, whether the store is `stosb`
(overwrite) or `xor` (overlay), and whether a trailing byte is consumed.
`lzss/` unifies them behind an `Emit` trait. Full walkthrough in
[lzss.md](lzss.md).

## 7. VGA-era video modes

The game supports four display adapters; each needs a different memory
layout, which is why every draw routine exists four times:

| adapter | mode | colours | VRAM layout |
|---|---|---|---|
| MCGA/VGA | `0x13` | 256 of 262k (palette) | 320×200 linear, 1 byte/px at `0xA000` |
| EGA | `0x0D` | 16 of 64 | four 0x1F40-bitplanes — each byte covers 8 px, 1 bit per plane |
| CGA | `0x05` | 4 | 2 bits/px; **interlaced**: even rows at `0xB800`, odd at `+0x2000` |
| TGA (Tandy) | `0x09` | 16 | 4 bits/px; **4 banks** of `0x2000` stepping through `y & 3` |

"Programming the DAC" means writing the VGA palette registers: each of
256 entries is 3×6-bit RGB components (`0..=63`), done via `int 10h
AX=1012` with a 768-byte table. The port scales them to 8-bit RGB with
`v * 255 / 63` (see [palette.md](palette.md)).

The `es:di` cursor every draw routine uses is just an index into VRAM
addressed by `es:di`; in the port `Screen.buf` is that VRAM and `di` a
`usize` — same arithmetic, no segments.

## 8. Numbers the asm way

- **Little-endian everywhere**: a `u16` at `x` is `buf[x] | buf[x+1]<<8`.
- **Wrapping arithmetic**: 16-bit registers overflow silently.
  `word + 1` in the docs means `wrapping_add(1)` in the code — and the
  *original's* overflow behaviour is intentionally preserved (e.g. the
  hover test's 8-bit bound wrap).
- **`u16::try_from(x).unwrap_or(0)`** is the codebase's idiom for
  "`ds:` offset that doesn't fit a `usize`/`u16` → 0", mirroring how a
  16-bit register truncates.
- `mul`/`div` in the asm produce 32-bit results the game truncates to 16
  bits; the port spells that `((a as u32 * b) & 0xFFFF) as u16`-style.

## 9. How the port is organised (the trait pattern)

The port is a **faithful reproduction**, not an emulator: same data
layouts, same algorithms, same byte-level quirks — but anything the asm
did by touching hardware is behind a trait so hosts decide what it
means:

| trait | stands in for | implementors |
|---|---|---|
| `platform::DosFiles` | `int 21h` file calls | `HostFs` (real dir), `MemFs` (test RAM disk) |
| `audio::Ports` | `in`/`out` instructions | `front::audio::Bus` (synth), test recorders |
| `game::VmHost` | calls into engine code the port doesn't own (draw, menu, dialogs, sound) | `front::Host` (real), `NullHost`, test recorders |
| `lzss::Emit` | `stosb`/`xor` byte stores | `Vec<u8>`, `lzss::Frame` (strided VRAM) |

Tests therefore assert *behaviour as ordered call sequences* — a rig
records every `Call` a routine emits instead of mocking pixels. When a
doc says "the routine emits `Call::Image(0)`", it means it invoked the
host the same way the asm invoked the draw slot `ds:[0x125A]`.

## 10. Reading the provenance notation

Module docs cite the asm they port, e.g. "`cs:0x73D9` (file
`0x373D9`–`0x376C5`)":

- `cs:`/`ds:` offsets address the unpacked image (§2).
- "file `0x…`" is where the same bytes sit in `GAME.unpacked.exe`.
- `GAM-nn` refers to project tickets — history, not semantics.
- A "slot" like `ds:[0x125A]` is an entry in the mode's dispatch table:
  the asm calls it indirectly (`call [0x125A]`), the port turns it into
  an enum variant + host call.

## 11. Where the originals live

- `decompiled/ANALYSIS.md` — the full reverse-engineering report.
- `decompiled/FUNCTIONS.md` — the verified function/address map.
- `decompiled/disasm/GAME.linear.asm` — the disassembly every citation
  points into (`ndisasm -b16`, file offsets).
- `decompiled/tools/` — the Python reference implementations the port
  cross-checks against (`extract_png.py`, `lzss.py`).

When a doc's claim seems surprising, the asm comment trail is one lookup
away: file offset → `GAME.linear.asm` line.
