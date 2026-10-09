# Decompilation Report — GAM-1: Supremacy

## Identification

| Field | Value |
|---|---|
| Title | **Supremacy** (full title *Supremacy: Your Will Be Done*; released as *Overlord* in North America) |
| Developer | Probe Software Ltd — `(C) 1991 PROBE SOFTWARE LTD.` (credit string: "FERGUS MCGOVERN", Probe's founder) |
| Publisher | Virgin Mastertronic |
| Year | 1991 |
| Platform | MS-DOS (16-bit real mode) |
| Package origin | freegameempire.com web-play wrapper (`GAME/content.xml` → `ProductConfiguration.xsd`, `PlatformType=Dosbox`) |

## Package layout (as shipped)

```
Supremacy/
├── Play NOW.exe      PE32 Windows binary — a renamed DOSBox build ("DOSBox-lfn"), NOT the game
├── dosbox.conf       DOSBox 0.72 config; [autoexec]: mount C GAME → C: → START.BAT
├── mapper.txt        DOSBox keymap
├── SDL*.dll, msvcr71/msvcp71.dll   DOSBox runtime deps (Win32)
└── GAME/
    ├── GAME.EXE      ← THE GAME: 147,761 bytes, MZ, EXEPACK-packed
    ├── START.BAT     runs GAME.exe
    ├── SUPRE.BAT     runs "GAME M M A" (mode args: sound/mouse options)
    ├── content.xml   freegameempire package descriptor
    ├── {MCG,EGA,CGA,TGA}{,IN,DTH,WIN}.{BIN,GPH}   per-video-mode asset sets (see below)
    └── 26219_game_extra_1.zip   bonus: SUPCHT.EXE savegame cheat (third-party, 2001)
```

## GAME.EXE — executable analysis

### Packing

`GAME.EXE` is a 16-bit MZ executable compressed with **Microsoft EXEPACK**
(giveaway: `!Packed file is corrupt` string at file offset 0x23B8E and the
canonical stub `mov ax,es / add ax,10h / push cs / pop ds / rep movsb /
push ax / retf` at the entry point).

16-byte EXEPACK header at file offset `0x23A80`:

| Field | Value |
|---|---|
| real_ip | 0x8A67 |
| real_cs | 0x286B |
| exepack_size | 0x06B1 (header + 283-byte-style stub + packed relocs) |
| real_sp | 0x8FB4 |
| real_ss | 0x1F6F |
| dest_len | 0x3829 paragraphs |
| signature | `RB` |

### Unpacked image (`decompiled/GAME.unpacked.exe`, 233,104 bytes)

- MZ, 694 relocations, header 0xC00 bytes, minalloc 0x74 / maxalloc 0xFFFF
- Entry `0x286B:0x8A67` (file offset `0x31D17`), stack `0x1F6F:0x8FB4`
- Entry stub: `mov ax,0x0FAA; mov ds,ax; mov dx,0x28; mov ah,0x1A; int 21h`
  (set DTA) → chain of init calls → `call far [0x125E]`. Predominantly
  hand-written assembly (no C runtime signature, no R6xxx error strings).
- Early init code installs a **PIT/keyboard driver**: saves + hooks
  int 8 (timer, es:0x20) and int 9 (keyboard, es:0x24), reprograms the
  8253 via ports 0x43/0x40.
- File I/O is centralised in a handful of `int 21h` wrappers
  (only ~30 `int 21h` sites in the whole binary): `findfirst(0x4E00)` →
  `open(0x3D00)` → `read(0x3F00)` → `close(0x3E00)`.
- Mode dispatch through a **function-pointer table at ds:0x1258–0x1298**
  (33 near offsets into CS). The whole table is copied from a per-mode
  0x115B-byte **parameter template** in dgroup — `rep movsb` at file
  0x32034 copies the selected template to ds:0x143, so dispatch entries
  live inside it. Template bases (dgroup offsets): MCG 0xB0EC,
  EGA 0xC247, CGA 0xD3A2, TGA 0xE4FD (files 0x1B78C/0x1C8E7/0x1DA42/
  0x1EB9D — four consecutive blocks). A separate far table at
  cs:0x126C–0x1282 (6 entries — the AdLib/Roland driver's exports,
  `01D0:0000/…/000F`) is `0x286B:0xFBD4` = `retf`
  stubs when the PC/Tandy sound driver is selected (see FUNCTIONS.md).
  Key MCG entries: `[0x125A]`=draw-image-by-index (0x58DA), the LZSS
  decode/blit routine.

### Command line — `SUPRE.BAT` runs `GAME M M A`

Positional args parsed from the PSP tail at es:0x82/0x84/0x86
(file 0x31FF8–0x3207D):

| Arg | Values | Meaning |
|---|---|---|
| 1 | C / T / E / **M** | video: CGA / Tandy / EGA / **MCGA** (M = default too) |
| 2 | K | keyboard mode: hooks int 33h and emulates the mouse ([0x9CCD]=1); anything else = real mouse driver |
| 3 | P / T / A / R | sound: PC speaker (1) / Tandy 3-voice (2) / **AdLib** (3) / Roland (4) → `[0x155]`; `[0x156]` = timer-ISR variant selector (0, 0xD5, 0x1D0, 0x1D0) — picks which int 8 handler the PIT hook installs |

So `GAME M M A` = MCGA video + mouse + AdLib; bare `GAME` (START.BAT)
defaults to MCGA + mouse + PC speaker.

### Loader / asset access

Filename buffers at `ds:0x1208–0x1257` are `???.???`/`?????.???` templates;
the video-mode select patches the `?`s with `MCG`/`EGA`/`CGA`/`TGA` + `.BIN`/`.GPH`
(which is why no static xrefs to the filename strings exist).

For each asset set the loader opens the `.GPH` and reads **0x10B0 (4272) bytes
→ ds:0x0158** = exactly the 356-record index for the main sets (smaller
sets read their own size via the same code path). The `.BIN` is loaded
whole into a heap segment in 0xFFF0-byte `int 21/3F` chunks (loop around
file offset 0x32135: read → `ds += 0xFFF` → repeat; heap base = paragraph
`(0x9CC4−0x7D39)>>4 +1 +0x3829` ≈ 0x3A22, right after the loaded image).
Afterwards every record's `ofs_para` has the heap segment added in place
(file 0x320D8) → absolute paragraph addresses for the renderer.

### Embedded content

- Full game text in plaintext: menu strings, event messages (comet impacts,
  aracno-locusts, scorpian V4 encounters, treaty warnings, research boosts),
  `PLEASE INSERT 'SUPREMACY' DISK 2` (original was a 2-disk release).
- AdLib/OPL music: dozens of embedded FM instrument patches (Timpani,
  Contrabass, BAGPIPE1, LIFEBASS, …) and song markers `supremsucc` /
  `supremunsuccess`.
- Copy protection: none found beyond the disk-2 prompt.
- `decompiled/strings-unpacked.txt` — full string dump with file offsets.

## Asset formats

### `.GPH` — graphics index (fixed 12-byte records)

```c
struct gph_rec { uint16_t x, y, w, h, ofs_para, mode_flag; };
```

- `x,y` = draw position; `w,h` = image size.
- `ofs_para` = offset **in paragraphs** (÷16 bytes) of the record's
  compressed stream inside `.BIN`. At load time the game adds the heap
  segment to it (loop at file 0x320D8), turning it into an absolute
  segment for the renderer.
- `mode_flag` (old high word of the offset dword): 0 = opaque blit,
  1 = **XOR overlay** draw (`xor [es:di],al` in the decoder).
- `w` is in *bytes-per-row* for the mode: MCG 320 px @8bpp → w=320;
  TGA 4bpp → w=160; CGA 2bpp → w=80; EGA planar → w=40 (per plane).

### `.BIN` — concatenated LZSS streams (NOT a packed archive)

The `.BIN` file is loaded verbatim into a heap segment; it is simply all
of the set's compressed image streams concatenated on 16-byte boundaries.
Each image = 1 stream (MCG/TGA/CGA) or **4 consecutive streams — one per
bitplane** (EGA: the draw routine at 0x2FB36 calls the decoder 4× with
plane mask 1/2/4/8).

**Codec = Okumura LZSS, N=4096, F=18, THRESHOLD=2** (decoder at file
0x2EBEF, XOR variant 0x2EC8C; EGA pair at 0x2FC5D/0x2FCF9; CGA 0x30695/
0x30745 and TGA 0x31148/0x31202 have their own copies — see FUNCTIONS.md):

- 4 KB ring buffer at SS:0x0000–0x0FFF (below the real stack),
  **zero-initialised**, write cursor starts at `r = 0xFEE` (N−F).
- Flag bits consumed **LSB-first**; register refilled when the 0x100
  sentinel shifts out (`shr dx,1` / `test dx,0x100` / reload `dl`, `dh=0xFF`).
- flag=1 → literal byte. flag=0 → 2-byte match token `T` (little-endian):
  `pos = (T & 0xFF) | ((T >> 4) & 0xF00)`, `len = ((T >> 8) & 0xF) + 3`.
- Terminator: match flag + token `00 00` + trailing byte `00`
  (EGA decoder consumes the trailing byte; MCG/TGA/CGA do not).
- Row wrap handled by caller: after `w` bytes the draw pointer advances
  `stride − w` (MCG stride 0x140, EGA stride 0x28). Encoders may omit
  trailing zeros on XOR sprites (e.g. CGA/187 decodes 207 of 210 bytes —
  harmless, pad with 0).

Working decoder + PNG renderer: `tools/lzss.py`, `tools/extract_png.py`,
`tools/extract_blobs.py`. All 1,703 records decode cleanly; rendered
output in `decompiled/png/<SET>/`. MCGA palette = master DAC table at
dgroup 0x4994 (file 0x15034, 256×3 6-bit values; runtime buffer at
CS:0x1384, set via int10/AX=1012). EGA/TGA render with the standard
16-colour palette; CGA with palette 1 (cyan/magenta/white).

| Set | Purpose | Records | .BIN bytes |
|---|---|---|---|
| MCG*  | MCGA/VGA 320×200×256 main game | 356 | 251,760 |
| MCGIN | MCGA intro/title sequence | 136 | 322,064 |
| MCGDTH| MCGA defeat sequence | 10 | 43,936 |
| MCGWIN| MCGA victory sequence | 22 | 64,496 |
| EGA*, EGAIN, EGADTH, EGAWIN | EGA 320×200×16 equivalents | 356/5/10/22 | … |
| CGA*, CGAIN, CGADTH, CGAWIN | CGA 320×200×4 equivalents | 356/5/10/22 | … |
| TGA*, TGAIN, TGADTH, TGAWIN | Tandy 320×200×16 equivalents | 356/5/10/22 | … |

All four mode sets share identical record counts — same content re-encoded
per display depth. 1,703 image records total indexed in
`decompiled/assets/manifest.json` (with exact stream lengths and decoded
sizes); the exact compressed streams are split into
`decompiled/assets/<SET>/NNN_<w>x<h>_x<X>y<Y>[_xor].bin`, and rendered
PNGs live in `decompiled/png/<SET>/` (16 dirs, verified visually —
title/opponent screens, planet UI, win/death sequence frames all render
correctly).

## SUPCHT.EXE (bonus cheat tool)

`SUPREMACY Savegame Editor/Cheat v2.0` (2001, third-party — dates in zip).
Also EXEPACK-packed → `decompiled/SUPCHT.unpacked.exe` (unpacks cleanly).
Edits saved games: Credits / Fuel / Food / Minerals / Energy / Population.
Its strings document the savegame layout — useful for further analysis.

## Deliverables (`decompiled/`)

```
GAME.unpacked.exe      unpacked 16-bit DOS game image (233 KB)
SUPCHT.unpacked.exe    unpacked cheat tool
strings-unpacked.txt   full string dump with offsets
disasm/GAME.linear.asm      linear 16-bit disassembly (ndisasm, 111k insns)
disasm/SUPCHT.linear.asm    linear disassembly of cheat tool
disasm/functions.txt        rizin aaa function list (245 fns recovered)
assets/manifest.json        all 1,703 records w/ exact stream+decoded sizes
assets/<SET>/*.bin          exact per-record compressed stream blobs
png/<SET>/*.png             all images rendered (1,703 across 4 modes)
tools/lzss.py               LZSS decoder (port of file 0x2EBEF routine)
tools/extract_png.py        .BIN/.GPH → PNG renderer (all 4 modes)
tools/extract_blobs.py      exact stream splitter
ANALYSIS.md                 this file
```

`rust/` (GAM-2): Rust port of the documented routines — `exepack`
(unpacker, byte-exact vs `GAME.unpacked.exe` + `SUPCHT.unpacked.exe`),
`lzss` (all four decoder variants + strided/XOR draw), `gph`, `args`,
`assets` (loaders + paragraph fixup), `palette`. `cargo test` verifies
all 1,703 records against the Python reference (FNV-1a fixtures in
`rust/tests/fixtures/`).

## Remaining work

1. ~~Codec~~ — **DONE**: LZSS N=4096/F=18/T=2, decoded + rendered above.
2. ~~Args~~ — **DONE**: `GAME <C/T/E/M> <K> <P/T/A/R>` mapped above.
3. **Full decompile**: load `GAME.unpacked.exe` into Ghidra/IDA
   (`Processor: 8086 real-mode, base = MZ`) — the unpacked image has valid
   relocations, so segment fixups resolve. 245 functions already recovered
   by rizin (`disasm/functions.txt`); remaining work is naming.
4. **Runtime check** (optional): DOSBox-X debugger could dump the heap
   segment to confirm, though the PNG output already proves correctness.
