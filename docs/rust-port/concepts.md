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

A concrete example. Suppose `ds = 0x1A23` (a real observed value — it
changes per DOS load, which is why nothing can be hardcoded).
Then `mov ax,[0x1258]` reads physical address
`0x1A23 * 16 + 0x1258 = 0x1B488`. The `* 16` is a left shift by 4 — the
segment value counts in units of 16 bytes.

Two consequences you will meet constantly:

- **A "paragraph" is 16 bytes** — the amount one segment unit covers.
  Addresses stored in paragraphs (e.g. `.GPH` `ofs_para`) must be
  multiplied by 16 to become byte offsets. When a doc says "the loader
  adds the heap segment to every `ofs_para`", it is doing
  `segment_value += heap_paragraph`, *not* touching the offset part —
  the offset stays relative to the segment base.
- Many different `seg:ofs` pairs address the same byte.
  `0xA000:0x0000` and `0x9000:0x8000` are the same VGA memory
  (`0xA000*16 + 0 == 0x9000*16 + 0x8000 == 0xA0000`). The port
  mostly ignores segments and works in **flat `ds:`/`cs:` offsets** —
  the `u16` offset part only — because within this program each segment
  register holds one fixed value.

In these docs, `ds:0x7D39` means "the byte at offset `0x7D39` inside the
game's data segment" and `cs:0x773D` "offset `0x773D` inside its code
segment". Square brackets mean "the value stored at": `[0x91B8]` is the
word held at `ds:0x91B8`. `+0xC` after a record means "the field 12
bytes into that record" — the same shorthand the disassembly uses for
`[si+0xC]`.

## 2. The three address spaces in the docs

Every hex number in the docs and code comments is a real address in one
of three spaces (also documented in `decompiled/FUNCTIONS.md`):

| space | meaning | convert to file offset |
|---|---|---|
| file offset | byte offset in `decompiled/GAME.unpacked.exe` | — |
| `cs:` near offset | offset inside the main code segment | `cs + 0x292B0` |
| `ds:`/`dgroup` offset | offset inside the data segment | `ds + 0x106A0` |

Why three spaces? The binary on disk is one flat byte array (file
offsets). When DOS loads it, code and data sit in different segments
with different zero points, so an address inside the data segment
(`ds:`) and the same data's position in the file differ by a constant —
`0x106A0` here, which is dgroup's load segment (`0xFAA`) times 16 plus
the `0xC00` MZ header size. Code works the same way with `0x292B0`.

Worked example: the docs say the state block lives at `ds:0x7D39`. To
find those bytes in the file on disk, compute
`0x7D39 + 0x106A0 = 0x183D9` — which is exactly where
`State::from_image` reads them. Going the other way: an interesting
instruction at file `0x373D9` in `GAME.linear.asm` is `cs:0x73D9`
(`0x373D9 − 0x292B0`), the tick dispatcher these docs call
`cs:0x73D9`.

The port keeps game data addressed by `ds:` offsets exactly as the asm
uses them, so a `[0x91B8]` in a comment and the `0x91B8` in the code are
the *same* number — no mental arithmetic needed when comparing. The
constants `0x106A0` (`DS_BASE`) and the `cs:` bases (`0x292B0` game code,
`0x30000` how the frontend maps it) do the file↔segment translation in
one place.

One subtlety: the frontend uses `0x30000`, not `0x292B0`, for `cs:`
reads. That is not a different address space — it is how `front::Host`
stores the image (game code indexed from a different base), and
`Vm::from_image`'s `jtab::fix` rewrites the shipped op-table pointers
into that space at load. Both spaces name the same bytes; the docs tell
you which each constant belongs to.

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

  Two hardware details that recur: the **PIC** (programmable interrupt
  controller) needs an *EOI* (end-of-interrupt, `out 0x20,0x20`) before
  it will deliver the next IRQ — ISRs "EOI the PIC" as housekeeping.
  And **hooking a vector** means overwriting the interrupt table entry
  (`int 8` → your handler), usually after saving the old pointer so you
  can *chain* to it — the game's timer ISR calls the saved BIOS handler
  every third tick so DOS's clock keeps time.

- **`in`/`out` — I/O ports.** A separate 64 KiB address space talking
  directly to chips. `out 0x388, reg` / `out 0x389, val` writes an OPL2
  register; `in al,0x60` reads a keyboard scancode. In the port every
  `in`/`out` goes through the two-method `audio::Ports` trait
  (`outb`/`inb`), so a test can record the exact byte sequence the asm
  would have emitted.

  Ports are how you program chips whose registers aren't memory-mapped:
  the OPL2 at `0x388/0x389`, the PIT at `0x40..0x43`, the Tandy PSG at
  `0xC0`, the keyboard at `0x60`, the speaker gate at `0x61`, the MPU-401
  at `0x330/0x331`. Many ports are two-part register files — you `out`
  the register number to the address port, then `out`/`in` the value on
  the data port — which is why e.g. an OPL2 write is always a `0x388`
  then a `0x389`.

Carry flag (`CF`) is DOS's error return: wrappers report failure by
setting it. The port mirrors that with `Option`/`bool` — `None`/`false`
everywhere a wrapper "returned with carry set".

## 4. The PSP command tail

When DOS launches `GAME M M A`, it places a **Program Segment Prefix**
before the program — 256 bytes of per-process state (open-file table,
memory size, interrupt vectors DOS restores on exit) — and copies the
typed tail into it at `es:0x80`: a count byte, a space, then the
characters:

```
es:0x80  06                    ← count: 6 chars after the space
es:0x81  ' '
es:0x82  'M'  ← video letter    (parsed, not scanned)
es:0x83  ' '
es:0x84  'M'  ← keyboard flag   ('K' = emulate mouse, else real)
es:0x85  ' '
es:0x86  'A'  ← sound letter
```

The game's argument parser doesn't scan the string — it reads three
fixed cells: `es:0x82` (video letter), `es:0x84` (keyboard flag),
`es:0x86` (sound letter). That is why `args::parse` takes a byte slice
laid out the same way, and why the binaries' `tail_bytes` helper
rebuilds a synthetic tail from separate CLI flags. It is also why the
letters sit *every other byte* — the parser assumes the DOS-canonical
`"X X X"` spacing and skips the separator cells entirely.

## 5. The `.exe` on disk: MZ + EXEPACK + relocation

A DOS `.exe` starts with an **MZ header** ("MZ" signature, sizes, entry
`cs:ip`, `ss:sp`, and a relocation table). DOS loads the image, then
adds the load segment to every word the table points at — **segment
relocations** — before jumping to the entry point. (The compiler emits
`mov ax, 0xFAA`-style immediate segment references as *placeholder*
values; the relocation table tells DOS which words to patch once the
real load address is known. That is why a disassembly of the file shows
different segment constants than a live DOS session.)

`GAME.EXE` is also **EXEPACK**-compressed (Microsoft's 1990-era packer):
the tail of the load image is a tiny decompressor plus a backwards
command stream. On boot it copies itself high, then runs `std`/`rep
movsb` *backwards* expanding the image in place — writes always land
above the read cursor because compressed data is smaller, so the
expansion never tramples input it hasn't read yet. After expansion it
applies its own relocation table and far-jumps to the real entry point.
`exepack::unpack` reproduces that whole dance in safe Rust;
`decompiled/GAME.unpacked.exe` is its output.

So when you see `GAME.EXE` vs `GAME.unpacked.exe`: the first is what
ships on disk (packed, with stub), the second is memory after the stub
finishes — the image the docs' file offsets index into, and the image
`Vm::from_image`/`State::from_image` carve `ds:` data out of.

## 6. LZSS in one paragraph

LZSS (Okumura flavour) is a dictionary compressor: the decoder keeps a
4 KB **ring buffer** of recently output bytes. A flag bit per token says
"literal" (copy the next byte out) or "match" (a 2-byte token says
*position* + *length* — copy `len` bytes from the ring and emit them,
feeding them back into the ring). `GAME.EXE` ships four near-identical
decoders that differ only in row stride, whether the store is `stosb`
(overwrite) or `xor` (overlay), and whether a trailing byte is consumed.
`lzss/` unifies them behind an `Emit` trait. Full walkthrough in
[lzss.md](lzss.md) — including a byte-by-byte trace of an actual stream.

## 7. VGA-era video modes

The game supports four display adapters; each needs a different memory
layout, which is why every draw routine exists four times:

| adapter | mode | colours | VRAM layout |
|---|---|---|---|
| MCGA/VGA | `0x13` | 256 of 262k (palette) | 320×200 linear, 1 byte/px at `0xA000` |
| EGA | `0x0D` | 16 of 64 | four 0x1F40-bitplanes — each byte covers 8 px, 1 bit per plane |
| CGA | `0x05` | 4 | 2 bits/px; **interlaced**: even rows at `0xB800`, odd at `+0x2000` |
| TGA (Tandy) | `0x09` | 16 | 4 bits/px; **4 banks** of `0x2000` stepping through `y & 3` |

Why the weird layouts? EGA/CGA predate VGA's byte-per-pixel mode. In
EGA mode `0x0D` each VRAM *byte* holds one bit each for 8 horizontal
pixels, and colour comes from four *planes* the hardware sums
bit-parallel — so "pixel 3 is colour 9" means bit 3 of a byte in plane 0
and bit 3 of a byte in plane 3 are both set (9 = 0b1001). CGA squeezes
4 colours into 2 bits/pixel *and* interlaces rows because of how its
CRTC refreshes — even scanlines live in the first 8 KB, odd in the
second. Drawing "row 1" literally means adding `0x2000`. TGA is the
same idea with four banks.

```
CGA VRAM (0xB800)          what you see
offset 0x0000: row 0   →   ┌ row 0
offset 0x0050: row 2   →   │ row 1  (from +0x2000)
        ...                │ row 2  (from +0x0050)
offset 0x2000: row 1   →   │ row 3  (from +0x2050)
        ...                └ ...
```

"Programming the DAC" means writing the VGA palette registers: each of
256 entries is 3×6-bit RGB components (`0..=63`), done via `int 10h
AX=1012` with a 768-byte table. The port scales them to 8-bit RGB with
`v * 255 / 63` (see [palette.md](palette.md)).

The `es:di` cursor every draw routine uses is just an index into VRAM
addressed by `es:di`; in the port `Screen.buf` is that VRAM and `di` a
`usize` — same arithmetic, no segments.

## 8. Numbers the asm way

- **Little-endian everywhere**: a `u16` at `x` is `buf[x] | buf[x+1]<<8`.
  The port's LE reads (`r16`, `word`, `u16::from_le_bytes`) all mean
  this.
- **Wrapping arithmetic**: 16-bit registers overflow silently.
  `word + 1` in the docs means `wrapping_add(1)` in the code — and the
  *original's* overflow behaviour is intentionally preserved (e.g. the
  hover test's 8-bit bound wrap). When the docs say a computation "wraps
  at 8 bits", they mean `as u8`/`wrapping_*`, not a bug.
- **`u16::try_from(x).unwrap_or(0)`** is the codebase's idiom for
  "`ds:` offset that doesn't fit a `usize`/`u16` → 0", mirroring how a
  16-bit register truncates. You'll see it as the `a!(…)` macro in
  `game/` files.
- `mul`/`div` in the asm produce 32-bit results the game truncates to 16
  bits; the port spells that `((a as u32 * b) & 0xFFFF) as u16`-style.
  This matters: the fleet-defence strength formula *depends* on the
  16-bit truncation — computing it in `u32` gives a different (wrong)
  answer.
- **Scancodes vs characters**: a keyboard `int 9` delivers *set-1
  scancodes* — hardware key positions, not ASCII. `0x1C` is Enter,
  `0x01` Esc, `0x48/0x50/0x4B/0x4D` the arrows; a "break" code (key
  released) is the make code `| 0x80`. The `ds:0x7CBD` keymap is what
  translates codes into text characters.

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

The payoff is visible in any `game_*.rs` test: instead of
"pixel (40,12) is colour 9", a test asserts
`calls == [Call::Image(0x12B), Call::Present(0)]` — the same observable
effects the asm produced by `call [0x125A]`, captured without needing a
real screen. `NullHost` (accept everything, return defaults) is how the
VM/tick code is exercised with no frontend at all.

## 10. Reading the provenance notation

Module docs cite the asm they port, e.g. "`cs:0x73D9` (file
`0x373D9`–`0x376C5`)":

- `cs:`/`ds:` offsets address the unpacked image (§2).
- "file `0x…`" is where the same bytes sit in `GAME.unpacked.exe` —
  and also the line offset in `decompiled/disasm/GAME.linear.asm`
  (the disassembly is linear over the file, so `grep` the address or
  jump to the matching offset to read the original instructions).
- `GAM-nn` refers to project tickets — history, not semantics.
- A "slot" like `ds:[0x125A]` is an entry in the mode's dispatch table:
  the asm calls it indirectly (`call [0x125A]`), the port turns it into
  an enum variant + host call. When a doc says "the `[0x1260]` slot",
  read "the per-mode routine the original calls through that cell".
- "`[0x91B8]`" vs "`0x91B8`": brackets = the *value at* the cell;
  bare = the address itself. `st.word(0x91B8)` reads `[0x91B8]`.

A practical lookup loop when a claim surprises you: find the cited file
offset in `GAME.linear.asm`, read the asm, compare with the Rust —
doc comments on the Rust carry the same citation so you can check the
mapping both directions.

## 11. Where the originals live

- `decompiled/ANALYSIS.md` — the full reverse-engineering report.
- `decompiled/FUNCTIONS.md` — the verified function/address map.
- `decompiled/disasm/GAME.linear.asm` — the disassembly every citation
  points into (`ndisasm -b16`, file offsets).
- `decompiled/tools/` — the Python reference implementations the port
  cross-checks against (`extract_png.py`, `lzss.py`).
- `decompiled/fixtures/` — captured dumps from the running original
  (DAC tables, `ds` snapshots) that the tests diff against.

When a doc's claim seems surprising, the asm comment trail is one lookup
away: file offset → `GAME.linear.asm` line.

## 12. Glossary

Quick reference for terms the docs use without re-explaining:

| term | meaning |
|---|---|
| **paragraph** | 16 bytes; the unit segment registers count in (§1) |
| **dgroup** | the program's data-segment group; `ds:` addresses index it |
| **cell** | a fixed `ds:` byte/word used as a global variable (`[0x91B8]` etc.) |
| **slot** | an entry in a per-mode dispatch table (`ds:[0x125A]`) — an indirect call target |
| **far pointer** | `seg:ofs` pair, 4 bytes; `far call`/`retf` cross segments |
| **ISR** | interrupt service routine — the handler a vector points at |
| **PIC / EOI** | interrupt controller / end-of-interrupt write to port `0x20` |
| **PIT** | programmable interval timer (ports `0x40..0x43`); ch 0 = int-8 tick, ch 2 = speaker |
| **scancode** | hardware key code from port `0x60`; bit 7 = release ("break") |
| **DAC** | VGA palette registers — 256 × 3 × 6-bit (§7) |
| **PSP** | Program Segment Prefix; the command tail lives at `es:0x80` (§4) |
| **provenance** | which asm routine a Rust item ports; cited as file/`cs:`/`ds:` offsets (§10) |
| **`rep movsb`/`stosb`/`lodsb`/`scasb`** | block-copy / store / load / scan string instructions — `rep` repeats `cx` times |
| **`std`/`cld`** | set/clear direction flag — `std` makes string ops walk *backwards* (EXEPACK relies on it, §5) |
| **TSR** | terminate-and-stay-resident — a program that hooks interrupt vectors and stays loaded |

## 13. A two-minute asm cheat sheet

The docs quote 8086 instructions constantly; this is the whole
vocabulary you need:

| instruction | reads as |
|---|---|
| `mov ax,[0x1258]` | `ax = word at ds:0x1258` |
| `mov [0x91B8],ax` | `word at ds:0x91B8 = ax` |
| `lodsb`/`lodsw` | read byte/word at `ds:si`, `si += 1`/`2` |
| `stosb`/`stosw` | write byte/word at `es:di`, `di += 1`/`2` |
| `scasb` | compare `al` to `es:[di]`, `di += 1` |
| `rep movsb` | copy `cx` bytes `ds:si` → `es:di` |
| `call word [0x125A]` / `jmp [si+0xC]` | indirect call/jump through a cell — the "slot" pattern |
| `call`/`ret` vs `call far`/`retf` | near (same segment, 16-bit) vs far (`seg:ofs`) calls |
| `cmp a,b` + `je`/`jne`/`jb`/`ja`/`jge` | compare + conditional jump on the flags `cmp` sets |
| `test al,1` + `jz`/`jnz` | bit test (a & b, flags only) + jump on zero |
| `and`/`or`/`xor`/`shr`/`shl` | bitwise ops; `shr dx,1` = `dx >>= 1` |
| `mul bx` / `div bx` | `dx:ax = ax*bx` / `ax = dx:ax/bx, dx = rem` — 32-bit through `dx:ax` |
| `push`/`pop` | stack — `pop ax` discarding a return address is a real trick the port models as early-return |
| `int 0x21` | software interrupt — the "syscall" (§3) |
| `in al,0x60` / `out 0x43,al` | port read/write — the "MMIO" (§3) |
| `cli`/`sti` | disable/enable interrupts around a critical section |
| `std`/`cld` | direction flag — `std` makes `lodsb`/`stosb` go *backwards* |
| `nop` | do nothing — often a patched-out call left as filler |

Flags live in `cf`/`zf`/`sf` — `stc` sets carry, `adc` adds *with*
carry (chained 16→32-bit adds), and the carry flag is DOS's error
return (§3). When a doc says "the `test`'s sign bit", it means
`test x, 0x80`-style — the sign flag reflects the top bit.

## 14. Following a citation, step by step

A doc says "`machine_tick` — `cs:0x742C` region, `0x37423`–`0x375C1`".
To see the original:

1. Open `decompiled/disasm/GAME.linear.asm`.
2. Find file offset `0x37423` — the disassembly is linear over
   `GAME.unpacked.exe`, so the line labelled `00037423` (or near it) is
   the routine's first instruction.
3. Read forward to `0x375C1` — that's the whole routine.
4. The port file is named next to the citation (`machine.rs`/
   `mine.rs`); its doc comments cite the same offsets, so you can map
   asm basic blocks to Rust functions one-to-one.

If the doc cites `cs:` instead (e.g. `cs:0x773D`), the label space is
`file − 0x30000` for names, `file − 0x292B0` for offsets stored in the
image — [front.md](front.md) § "The two `cs:` spaces" has the full
story.
