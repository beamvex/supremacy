# Function / symbol map — GAME.unpacked.exe

Started for GAM-4 (subtask of GAM-3). Extends `ANALYSIS.md` with
function-level findings; every address below was verified by disassembling
`GAME.unpacked.exe` (`ndisasm -b16`, file offsets unless marked `cs:`/`ds:`).

## Address spaces

| space | base | notes |
|---|---|---|
| file offset | 0 | `GAME.unpacked.exe` bytes |
| load image | file − 0xC00 | MZ header is 0xC00 bytes; `disasm/functions.txt` uses this |
| `cs:` near offset | file − 0x292B0 | main code segment CS = 0x286B |
| `ds:`/`dgroup` offset | file − 0x10CA0 | dgroup base = 0xFAA paragraphs |

## Mode parameter templates

The video-mode select copies one 0x115B-byte template to `ds:0x143`
(`rep movsb` at file 0x32034). Each template holds the mode's constants
followed by its dispatch table.

| mode | dgroup | file | byte[0] int10 mode | word[2] | word[4] VRAM seg |
|---|---|---|---|---|---|
| MCG | 0xB0EC | 0x1B78C | 0x13 (320×200×256) | 0x9000 | 0xA000 |
| EGA | 0xC247 | 0x1C8E7 | 0x0D (320×200×16) | 0x8000 | 0xA000 |
| CGA | 0xD3A2 | 0x1DA42 | 0x05 (320×200×4)  | 0x8000 | 0xB800 |
| TGA | 0xE4FD | 0x1EB9D | 0x09 (320×200×16) | 0x8000 | 0xB800 |

- `bytes[6..0x12]` — three far pointers, identical in all four templates:
  `286B:8AC4`, `286B:11C8`, `286B:11C8`.
- `bytes[0x1115..0x1157]` — the 33-entry near dispatch table that lands at
  `ds:0x1258`–`ds:0x1298`. Full per-mode dumps: `disasm/dispatch_tables.txt`.
- Each mode's dispatch targets sit in one contiguous CS region:
  MCG `0x58BF`–`0x6379`, EGA `0x63E2`–`0x736E`, CGA `0x736F`–`0x7DC3`,
  TGA `0x7E1D`–`0x89FA` — the mode code was linked back-to-back.

## Identified dispatch slots

| ds slot | role | MCG | EGA | CGA | TGA |
|---|---|---|---|---|---|
| `[0x1258]` | palette init | cs:0x5B0B | stub 0xF294 | stub 0xF294 | stub 0xF294 |
| `[0x125A]` | draw-image-by-index | cs:0x58DA | cs:0x6886 | cs:0x7370 | cs:0x7E1E |
| `[0x1298]` | dirty-region register | cs:0x6324 | cs:0x7216 | cs:0x7DC3 | cs:0x8921 |
| `[0x128E/90/92]` | (MCG cs:0x6290/E9/EA) | — | stub 0xF723 ×3 | stub 0xF723 ×3 | stub 0xF723 ×3 |

- `[0x1258]` MCG (file 0x2EDBB): `int 10/AX=1012, BX=0, CX=0x100,
  ES:DX=CS:0x1384` — programs all 256 DAC registers from the runtime copy
  of the master palette. Non-MCG stub (file 0x38544):
  `mov byte [0x91CA],0; ret`.
- `[0x1298]` (MCG file 0x2F5D4): `cli; inc byte [0x4C94]`; while the counter
  is 1 and `[0x9CCE]≠0`, grows the dirty rect `[0x9CD6]`/`[0x9CD8]` (±0x10
  slack) to cover the draw's (x1,y1,x2,y2) passed in ax/cx/bx/dx, then
  chains on. Region tracking for screen refresh — not clipping.
- `0xF723` (file 0x389D3): `call 0x2CA9; ret` — shared filler for slots a
  mode doesn't implement.

## draw-image-by-index (ds:[0x125A]) — all four variants

Common contract: `dx` = image index → `si = ds:0x158 + dx*12` (the fixed-up
`.GPH` record); rect (x1,x2 in ax/bx; y1,y2 in cx/dx) → `call [0x1298]`;
`ax` = `w`, `dl` = `mode_flag`; `ds = ofs_para`, `si = 0` (record's stream);
`dl==0` → opaque decoder, `dl==1` → XOR decoder, anything else skipped;
then `call 0xA479` (post-draw, TBD) and restore.

| mode | routine (file) | di computation | decoder pair (file) | row advance |
|---|---|---|---|---|
| MCG | 0x2EB8A | `y*0x140 + x` | 0x2EBEF / 0x2EC8C | `di += 0x140 − w` |
| EGA | 0x2FB36 | `y*0x28 + x` per plane | 0x2FC5D / 0x2FCF9 | `di += 0x28 − w` |
| CGA | 0x30620 | `(y>>1)*0x50 + (y&1)*0x2000 + x` | 0x30695 / 0x30745 | `di ^= 0x2000`, if `<0x2000` then `+0x50` |
| TGA | 0x310CE | `(y>>2)*0xA0 + (y&3)*0x2000 + x` | 0x31148 / 0x31202 | `di += 0x2000`, if `≥0x8000` then `−0x8000 +0xA0` |

Mode-specific details:

- MCG: skips the draw when `ofs_para ≥ 0xA000` (i.e. record not fixed up).
- EGA: rect passed to `[0x1298]` in **pixels** (`x*8`, `w*8`). For each of
  plane masks 1/2/4/8: `cli`, `[cs:0x538D]=mask`, `[cs:0x5388]=plane`,
  `call 0x8498` (programs the map-mask register), `sti`, decode one plane
  stream with `di` preserved across calls; afterwards `call 0x8468`
  (restores EGA regs). EGA decoders consume the terminator's trailing byte.
- CGA: rect in pixels (`x*4`); its **own** decoder copies implement the
  two-bank interleave (`B800`/`BA00`): each emitted row toggles bank
  `0x2000`; returning to bank 0 advances `0x50` bytes.
- TGA: rect in pixels (`x*2`); four Tandy banks `n*0x2000` for `y&3`, pitch
  `0xA0` per bank-cycle. **Correction to ANALYSIS.md**: the `0x2EBEF`/
  `0x2EC8C` decoders are MCG-only — every mode has its own decoder pair
  (identical LZSS core, different emit/row-advance). Terminator handling:
  only the EGA pair consumes the trailing `00`.

## Sound driver modules — `[0x156]` is a paragraph offset

`[0x156]` (arg byte: P `0` / T `0xD5` / A `0x1D0` / R `0x1D0`) is the
paragraph offset into the load image of the `int 0x80` sound-driver
module — the install sets `int 0x80` = `(loadbase + [0x156]):0`
(file `0x31EDD`):

| sound | `[0x156]` | image | file | entry |
|---|---|---|---|---|
| P | `0x000` | `0x0000` | `0xC00` | `sti; push…; ds=cs; call [ah*2+0x52]` for `ah<0x20`, `ah≥0x20` → `0xD4C` |
| T | `0x0D5` | `0x0D50` | `0x1950` | same pusha-style ISR prologue |
| A | `0x1D0` | `0x1D00` | `0x2900` | jump table of 3-byte `jmp`s (C-compiled driver — `bp` frames, `call 0x7970`/`0xB590` runtime) |
| R | `0x1D0` | `0x1D00` | `0x2900` | same module, selected later by the ax arg |

- **Far table correction** (errata to ANALYSIS.md): the 6-entry far table
  is at `cs:0x126C`–`0x1282` (file `0x2A51C`), not `ds:` — callers use an
  explicit `cs:` override (`call far [cs:0x1280]`). In the image it holds
  `01D0:0000/0003/0006/0009/000C/000F` — exports of the AdLib/Roland
  driver's jump table. The init (file `0x31EF7`) overwrites all six with
  `286B:FBD4` = `retf` stubs **only when sound ≤ 2** (P/T); for A/R the
  driver exports stay live. `call far [cs:0x126C]` at `0x31FF2` is invoked
  with `ax = [0x155] − 3` (AdLib=0 / Roland=1).
- **int-8 install** (`install_drivers`, file `0x31E8E`, runs only when
  `[0x155] ≤ 2`): BIOS int-8 IVT copied → int `0x81` slot (`es:[0x204]`)
  and `cs:[0..3]`; old int `0x80` → `cs:[4..6]`; int 8 set to
  `286B:F727` (file `0x389D7`, the ISR tail); PIT programmed `0x43=0x36`,
  divisor `0x5CEC` (~50.2 Hz) via `0x40`; far-table stub-fill above.
  `[0x156]` indexes which `int80`-calling ISR variant the handler uses.
- **int-80 game-side wrappers** (file `0x389D7`–`0x38ABF`, all gated on
  `[0x155] ≤ 2`): `0x389D7` int-8 tail → `int 0x80 ah=0` + EOI + `int 0x81`
  (chained BIOS tick); `0x389FB` `ah=1`/`2`; `0x38A1B` `ah=0x32`/`0x33`;
  `0x38A3B` `ah=3`/`4` with `al` = sfx id; `0x38A5B` `ah=1`/`2` +
  `call far [cs:0x1280]`; `0x38A80` `ah=2`/`0` gated on `[0x91D3]==0`;
  `0x38AA7` `ah=0x32`/`0x33` `al=0`.

## Device drivers inside the A/R module (file `0x2900` segment)

- `0x3EFD` `opl_write(ax)`: `ah`=reg, `al`=data — `0x388`←reg, 6 status
  reads, `0x389`←data, 37 status reads.
- `0x3F38` `opl_delay`: 38 status reads (`call`ed ×4 in detect).
- `0x3EA4` `opl_detect` → `cl`: canonical OPL2 timer test — `r4=0x60`,
  `r4=0x80`, `st0=in 0x388`; `r2=0xFF`, `r4=0x21`; delay×4; `st1=in 0x388`
  (read twice); restore `r4=0x60`/`0x80`. Present ⇔ `st0&0xE0==0` and
  `st1&0xE0==0xC0`.
- `0x3E90` `opl_key_off(dl=ch)`: `0xA0+ch`=0, `0xB0+ch`=0.
- `0x3E5D` note-on register half: caches `[si+0x16]=bx&0x3FF`,
  `[si+0x15]=bh>>4`; writes `0xA0+ch`=bl (f-num low), `0xB0+ch`=
  `(bh&3) | ((bh>>1)&0x1C) | cl` (f-num hi + block + key bits).
- `0x3470` `mpu_reset` → `al`: DSR poll (`0x331` bit 6, `0xFFFF` budget),
  `out 0x331=0xFF` (reset) under `cli`, DRR poll (bit 7), `in 0x330` →
  `0xFE` ACK stored at `[0x1C02]`; returns 0 on timeout/no-ACK.
- `0x34F5` `mpu_cmd(al)`: DSR wait, `out 0x331`, DRR wait, drain `0x330`.
- `0x350E` `mpu_send(al)`: DSR wait, `out 0x330`.
- `0x351F` `mpu_read` → `al`: DRR wait, `in 0x330`.
- `0x34C8` `mpu_sysex(si)`: `[si]`-counted head block, then `[si]`-counted
  body summed into `dl`, sends `(-dl)&0x7F` checksum + `0xF7`.
- `0x33D0` Roland init: `mpu_reset` → `mpu_cmd(0x3F)` (UART mode) → setup
  block loop (`0x3431`) + `mpu_sysex(0x1C07)`; on success `[0x19AC]=1`,
  `[0x2062]=0x4B`, returns `al=1`.

## PC speaker driver (file `0xC00` module)

- `0x0E9F` `spk_note(cx=divisor)`: skips PIT program when `cx == [0x2E]`;
  `out 0x43=0xB6` (ch 2, lo/hi, mode 3), `out 0x42` lo/hi, `[0x2E]=cx`,
  then `in 0x61 | 3` → `out 0x61` (gate + data enable).
- The tracker above it (file `0x0E80`+): beat counter `[0x450]`, divisor
  pair `[0x454]`/`[0x456]`; channel records at `di` — stream ptr `+4`,
  ticks-left `+6`, reload `+7`, volume idx `+9` (via `[bx+0x6B9]`),
  arpeggio idx `+0xB`, flags `+0x12`, transpose `+0x15`/`+0x16`; opcode
  jump table at `cs:0x32` for stream bytes `≥0x80`, note table
  `[bx+0x5EA]`.

## int-9 keyboard ISR (file `0x31D74` = `286B:8AC4`)

The first far pointer in every mode template. `in 0x60` → `ah`; keyboard
ACK via `0x61` bit-7 toggle (`|0x80`,`out`,`&0x7F`,`out`); EOI `0x20`;
then `0x31DB5` tracks Ctrl `0x1D`/`0x9D`, Alt `0x38`/`0xB8`, Del
`0x53`/`0xD3` into `cs:0x8B6F`–`0x8B71` — all three held ⇒ `jmp
0xFFFF:0x0000` (warm reboot). The code is latched to `[0x9CCC]` with
`[0x9CCB]=1` (consumed by the K-mode mouse path) and pushed onto a
**10-entry LIFO stack** at `cs:0x8B72` with count `cs:0x8B7C` (init
`0xFFFF`, push skipped at 9). Pop = `0x31E2E`, push helper = `0x31E4A`.
Vector swap helpers: `0x31E64` int-9 ↔ `ds:[0x149]`; `0x38E3C` int-`0x24`
(critical error) ↔ `ds:[0x151]`.

## Mouse / menu input (file `0x2A0DF`+)

- `0x2A0DF`: when `[0x9CCD]==1` (`K` arg), swaps the int-`0x33` IVT entry
  (`es:0xCC`/`0xCE`) against `ds:[0x14D]`/`[0x14F]` — hook/unhook.
- `0x2A18E` (real driver): synthetic codes `0x7E`/`0x7D` → left/right
  button down, `0xFE`/`0xFD` → release; writes `[0x9CDA]` +
  `cx,dx`/`bx`/`bp` result regs.
- `0x2A1CD` (`K` mode): `Enter 0x1C` → left down, `Esc 0x01` → right down,
  breaks `0x9C`/`0x81` → release; arrow/nav scancodes below.
- `0x2A20E`+ nav table: `0x48/0x50/0x4B/0x4D` = up/down/left/right →
  selects hotspot-record field `+0x10`–`+0x13`; Tandy (`[0x9CC6]==1`) adds
  `0x29`/`0x4A`/`0x2B`/`0x4E`. `0x2A2AF` loads `[bx+si]` → new
  `[0x9CC8]` selection (`0xFF` = stay).
- `0x2A10F` menu service: polls keys, on `[0x9CDA]&1` walks the hotspot
  list `ds:[0x9194]` (`[0x91A8]` entries × `0x14` bytes: `+0/+2/+4/+6` =
  x1,y1,x2,y2; `+8` = pressed-state image idx drawn via `[0x125A]` when
  nonzero; `+0xC` = action jump; `+0x10..13` nav links), hit-tests
  `[0x9CD6]`/`[0x9CD8]`, then `jmp [si+0xC]`.
- `0x2A2D2`: `K` mode snaps the cursor to the hit item's centre,
  `y ≤ 0xBC`.

## Timer / interrupt driver (early-resident segment)

A small separate code segment (file ≈ 0x3400–0x3600; its CS base maps near
file 0x2900) holds the hardware driver installed during init:

- file 0x353D — `prog_pit`: `al=0x36 → out 0x43`; `ax` → `out 0x40` lo/hi
  (PIT channel 0, mode 3 square wave, divisor in `ax`).
- file 0x354C — `cli; call prog_pit; sti` wrapper.
- file 0x3552 — `timer_install`: `[CS:0x1C85]=0xFF` active flag;
  `ax=[CS:0x1C00]` divisor → `prog_pit`; `int 21/AH=35 AL=8` saves the BIOS
  int-8 vector to `[CS:0x1666]`; `int 21/AH=25 AL=8` points int 8 at
  `CS:0xCBF`.
- file 0x358C — `timer_uninstall`: if active, `prog_pit(0)` (back to
  18.2 Hz) and restores the saved int-8 vector.
- file 0x35BF — int-8 ISR: `out 0x20,0x20` (EOI), `dec byte [CS:0x1BFE]`;
  every 3rd tick resets the counter to 3 and `pushf` + `call far` the saved
  BIOS handler — the game tick runs at the reprogrammed PIT rate while DOS
  keeps its 18.2 Hz timebase. The `[0x156]` arg byte (0 / 0xD5 / 0x1D0 /
  0x1D0 per sound mode) selects among ISR variants.
- int 9 is saved/hooked the same way (ANALYSIS §entry stub; keyboard queue
  feeds the game).
- file 0x3520 — MPU-401 status poll (`dx=0x331`, `in al,dx`, `test 0x80`):
  part of the Roland path.
- file 0x389D7 — ISR tail used by the int-8 variants: `ds=0xFAA`, compares
  `[0x155]` (sound index) to pick a handler, `int 0x80` / `int 0x81` are
  soft-interrupt slots for sound service + chained-vector calls, ends
  `out 0x20,0x20; iret`. `[0x155]=0..4` = PC/Tandy/AdLib/Roland.

## Save / load — the state block

The savegame is a **raw memory dump**, not a structured format: the same
`0x1F8B`-byte region `ds:0x7D39..0x9CC4` is written and read whole.

- file `0x383CE` — `load_state`: `int 21/AH=3D00` open read-only →
  `AH=3F00` `cx=0x1F8B` `dx=0x7D39` read into the state block → `AH=3E00`
  close. Carry-flag error → `false`.
- file `0x383F3` — `save_state`: `int 21/AH=3C00` create/truncate (handle
  leaked), `AH=3D01` open write-only → `AH=4000` `cx=0x1F8B` `dx=0x7D39`
  write → `AH=3E00` close.
- file `0x38432` / `0x3844B` — staging copies `ds:0x7D39` ↔ an allocated
  loader-heap segment (the save/load UI round-trips through it).
- Callers live in the save/load dialog flow `cs:0x9BF0–0x9DED` (file
  `0x32EA0` region): "PLEASE ENTER YOUR FILENAME?" (ds:0x7F85),
  "PLEASE WAIT - LOADING/SAVING ...." (0x7EA8/0x7EC5), "GAME WAS
  SUCCESSFULLY SAVED!" (0x82CE), "GAME HAS LOADED SUCCESSFULLY!" (0x82F0)
  — reached via a 12-entry near-pointer table at file `0x4708`.
- The block is a mixed workspace: `0xFD`-separated message strings first
  ("  \xFDFORMAT COMPLETE.\xFD...", planet names like STARNAM29), then
  the `0x3A`-stride record array (anchored by the galaxy preset), scalars
  and menu state. Initial contents ship in the image at file
  `0x189D9`–`0x1A963` (file ofs = `ds:` + `0x10CA0`).

## PRNG — `cs:0x55B2` (file `0x2E862`–`0x2E93B`)

`rand(b)` in `bx` → result in `ax`:

- 32-bit LCG over `[0x55A2]:[0x55A4]` — scramble is sequenced on 16-bit
  halves: `p = s + 7`; `a = s·4`; `m = a`; `a = a·2 + p + m` →
  `s' = 13·s + 7` (mod 2³²).
- Fold: `r = lo16(s') ^ hi16(s')` (both `[0x55A6]` and `[0x55A8]` end up
  equal to `r` via the `xchg`+xor pair).
- `dx:ax = (b+1) · r`; returns `dx` = `hi16((b+1)·r)` — i.e. a `0..=b`
  value scaled through the 16-bit product's high word.

Rust port: `game::Rng`; the test model (`tests/game_state.rs`) replays
the half-word `add`/`adc`/`xchg` sequence independently and matches.

## Records / new-game init — `cs:0x9DAB` (file `0x3305B`–`0x331C2`)

- Record array: base `[0x9158]`, stride `0x3A` (58) bytes — accessor
  `cs:0x5E3A` (file `0x35E3A`): `ax = idx·0x3A + [0x9158]`.
  `[0x91B6]` = total records, `[0x91B8]` = planet count = index of the
  appended player-faction record, planet-count byte `[0x91C6]`,
  difficulty `[0x91E4]`.
- Galaxy preset picker (file `0x36792`–`0x3681F`), four branches; three
  confirmed — `(base, total, diff)` = `(0x8486,0x20,2)`, `(0x8BC6,0x10,1)`,
  `(0x8F66,0x08,0)`; each stores a 2-word banner (little-endian bytes read
  `RORN `/`KRART`/`SMINE`/`WOTOK` — display encoding unverified).
- `cs:0x9DAB` new-game init: rec 0 fixed block (difficulty-scaled word
  `+0x26` = `0x1F77/0x3A2F/0x5811`, `[0x9164]` pacing 6/14/30, credits
  dword `+0x36` = `0x1879A`, stocks `+0x2A..+0x34`, owner `+0x24` = 2);
  rec `[0x91B8]` rolled block (credits `0xC350 + rand(0x4E20)`, stocks
  `0x9C4 + k·rand(0x3E8)` arithmetic series, `0x5DC + rand(0x1F4)`);
  recs `1..n−1` named `LIFELESS!` (`+0x0E`, `0x21`-terminated); all
  `0..=n` records get serial `+0x28` = index, `+0x29` = 0, `+0x00` = 0.
- Field map (TBD tags until SUPCHT offsets are cracked): `+0x00`
  reserved word (zeroed at init, no runtime accesses found), `+0x0E`
  name, `+0x22`, `+0x24` owner, `+0x26` diff param, `+0x28`
  serial, `+0x29`, `+0x2A..+0x34` five stock words, `+0x36` credits
  dword.
- `file 0x36709` — `print_str`: the `0xFF`-terminated string
  interpreter used by the message region. Operands: `≥0x20` draws the
  glyph via `[0x1268]` at `(bx·4px, dx)` then `bx += 1`; `0x01`/`0x02`
  add the next byte (signed) to `dx`/`bx`; `0x03` reads a `{count,char}`
  word and repeats (`bx += 1` each); `0x04` is the vertical variant
  (`dx += 6`); other control bytes end the print. When `[0x91EA] ≠ 0`
  each direct char also runs the pace block (`0x5C1E` seq, `0x73D9`
  tick, `0x2CA9` vsync, `0x66F3` overlay once).
- `file 0x366DE` — `init_planet_names`: 99 × 12-byte records at
  `ds:0x7D39`+… (planet-name table, distinct from the `0x3A` records).

## Record fields — decoded via the stat display + FORMAT REPORT

The stat-report strings (`FOOD`, `MINERALS`, `FUELS`, `ENERGY`,
`CIVILIANS` — `ds:0x653A` region, file `0x171DA`+) pin the stock words;
the sim/display routines pin the rest:

- `+0x00` reserved word (zeroed at init), `+0x0C` kind (`0xA` =
  colonised planet, `7` = fallen, `4` = unclaimed candidate, `1` =
  colonisation en route — file `0x37F7A`/`0x37F8B`), `+0x0E` name
  (9 bytes; `+0x12`/`+0x13` are inside it — the "station type"/"rate"
  reads at file `0x339A9`/`0x33C3B` go through the machine-type record
  `[0x9154]`, stride `0x30`, not this record),
- `+0x18`/`+0x1C`/`+0x20` machine-link slots A/B/C — bytes holding a
  1-based index into the machine array at `ds:0x9491` (stride `0x28`),
  `0` = empty; nonzero resolves `(v−1)·0x28 + 0x9491` (file
  `0x3297E`/`0x351B3`), feeds the orders alert `or` test (file
  `0x33F76`), and is cleared when the matching machine is razed (file
  `0x346F7`, VM op `0x7AF1` does the same on the fleet-tail copy),
- `+0x1A` distance/travel word — `/0xFA` → transit days at colony
  launch (file `0x3724A`) and `/0xFA + 0xB` → colonise countdown
  `[0x91C7]` (file `0x37F90`),
- `+0x1D` growth score (`cover/3`), `+0x1E` tax slider,
  `+0x1F` food-coverage gauge, `+0x21` decline score
  (`(100−cover)/4 + tax/4`), `+0x22` defence strength,
- `+0x24` owner, `+0x26` troops (regen `+1/+3/+4` for owner −/5/4, cap
  `0x3095`; liquid cash on the faction record), `+0x28` serial,
  `+0x29` defence-system upgrade level `0..=3` — panel buttons step it
  with icon `0x58`/`0x59`/`0x5A` redraws + SFX (file `0x34A0C`/
  `0x34A4D`); weights `guns+load+F_29+1` (file `0x344F3`) and
  `troops·(2+F_29)/100` (file `0x34597`),
- `+0x2A` population (cap `0x7530`), `+0x2C` food, `+0x2E` attack sum
  (Σ raw ship power), `+0x30` minerals, `+0x32` fuel,
  `+0x34` energy, `+0x36/+0x38` credits dword — pinned by the spy-menu
  charge routine `cs:0x82BE` (file `0x382BE`) which `sub`/`sbb`s the
  menu price (`1000`/`1520`/`2200`/`4720`, file `0x38238`) from the
  dword, and by machine builds deducting the type-table `+0x06` cost
  (file `0x365CD`).

## Planet simulation — `cs:0x3D1E` (file `0x33D1E`–`0x33F17`)

Per-tick update for kind-`0xA` records, reached from the dispatcher
(`0x38 + planet_index`):

- Consumption: `need = pop/0xF0`; `food = max(0, food − need)`; shortfall
  zeroes the coverage gauge and warns every 16th tick (`tick & 0xF == 0`).
- Coverage `+0x1F` tracks `100 − tax` by ±1; growth score `= cover/3`,
  decline `= (100−cover)/4 + tax/4`.
- Population: `net = growth − decline`; `±(|net|·pop/400 + 1)` (decline
  clamped to `pop`), growth skipped under 2, cap `0x7530`, wrapping add.
- Income (odd ticks): `pop·tax / (owner ∈ {1,2} ? 125 : 200)` added to
  the credits dword.
- Delayed depopulation: `[0x8220]`/`[0x821F]` armed + record match →
  `pop = 0`.
- Dirty flags `[0x9144]`/`[0x9146]` mark fields for redraw (`0x20` pop,
  `0x40` warn, `0xC0` cover, `0x4000` food, …).

## Machine/station array — `ds:0x9491`, stride `0x28`, 32 entries

Fields: `M_TYPE` kind byte (`0` = free, `3` solar, `6` core-miner, `7`
horticultural, `5`+ colony/other), `M_FLAGS` (`0x1` building, `0x4`
offline-mask, `0x8` mining, `0x10` destroyed-keep, `0x20` online),
`M_HOST` host-planet index, `M_LINK` linked record, `M_OPS` ops left,
`M_TIMER` countdown, `M_BUILD` colonise countdown, `M_DEPOSIT` mineral
deposit. Machine-type table at `ds:0x9B12` stride `0x30` (`+0xC` =
`0xFFFF` = no drain) — fields from the spawn copy (file `0x36590`/
`0x38085`) and the purchase deduction (file `0x365BE`–`0x365ED`):
`+0x06` credits cost (vs faction `+0x36`/`+0x38`), `+0x08` build seed
→ `M_TIMER`, `+0x0A` → machine `+0x14`, `+0x0C` deposit seed → scaled
into `M_DEPOSIT`, `+0x0E` energy cost (vs faction `+0x34`, difficulty
`≠0`), `+0x10` minerals cost (vs faction `+0x30`, difficulty `≥2`),
`+0x12` type id → `M_TYPE` (also the SFX selector input at file
`0x339A9`), `+0x13` aux/rate byte → machine `+0x20` (the `0x33C3B`
rate multiply).

Tick — `cs:0x742C` (file `0x37423`–`0x37616`), dispatched on codes
`1..0x20`:

- `M_TIMER` decrements, dirty `0x800`.
- Miner (online): minerals `+2 (+7 tech, +5 owner 5)`, fuel
  `+7 (+0x19 tech, +0xF owner 5)`; farm: burns 1 energy (else goes
  offline), food `+0xC (+0x19 tech, +0x1C owner 4)`; solar: energy
  `+6 (+7 owner 3)` — all under the `0x7530` stock ceiling.
- Mining flag `0x8`: drains `M_DEPOSIT` by `0x19`/`0x32` (deep-mine
  tech), `M_OPS−1`; on completion relinks `M_HOST` from the linked
  record's `+0x28` and clears flag `0x8` (or re-arms `0x11`).
- `M_BUILD` hitting 0 colonises `QUEUED_REC`: kind `0xA`, owner rolled
  from `{1,3,4,5}` (`rand & 7` rejected otherwise), pop `rand(0x3E8)`,
  food `0x12C + rand(0x4B0)`, stocks `0x14/0x96/0x23`, credits 0.

## Fleet / defence — `cs:0x4434` (file `0x34434`–`0x34973`)

Ship records: `ds:0x9991`, stride `0xC`, 23 entries — `S_POW` power,
`S_LINK` docked record offset, `S_FLAGS` (`0x2` = armed, `0x8` mask),
`S_GUNS`, `S_LOAD`, `S_CREW`.

Per-planet pass (dispatcher codes `0x38 + total + planet_index`):

- Scan: `F_ATTACK` = Σ `pow`, `F_DEFENCE` = Σ `pow + (pow·crew/0x48 &
  0xFFFF)·(guns + load + F_29 + 1) & 0xFFFF` — the asm's `mul`/`div`
  keep `ax`, so both stages truncate to 16 bits.
- Troops 0 + defence → reinforcement: crews of armed linked ships `+7`
  (cap `0x5D`); troops 0 + no defence → quiet.
- Regen `+4/+3/+1` (owner 4/5/other, cap `0x3095`), then
  `eff = troops·(2 + F_29)/100` min 1 vs `F_DEFENCE`:
  - `def > eff` → repelled: each linked armed ship's `pow` scaled by
    `100·eff/def`% (0 unlink+dirty `0x8`), `F_ATTACK` = Σ result, troops
    `− def·2/100` (underflow → reinforcement).
  - `def ≤ eff` → overrun: linked armed ships zeroed (8 bytes), bound
    machines + their ships destroyed, matching planet link slots
    (`+0x18`/`+0x1C`/`+0x20` holding the machine index) cleared,
    kind `7`, message `MSG_REC`, dirty `0x8`/`0x18`.

## Tick dispatcher — `cs:0x73D9` (file `0x373D9`–`0x376C5`)

`[0x91CE]` is a per-tick sequencer incremented each call from the main
loop (file `0x339F0` region). The byte selects the work:

- `1..0x20` → machine `code` tick.
- `0x20..0x38` → ship `code − 0x20` tick (`cs:0x60EE`, below).
- `0x38 + i` (`i < total`) → planet `i` simulation.
- `0x38 + total + i` → planet `i` defence pass.
- `0x38 + 2·total + i` → event slot `i` (the `cs:0x773D` VM below).
- `0xA7` selected-object reprint (`0x60A8`), `0xA8` jingle selector
  (`0x7E36`), `0xA9` tribute (`0x7C62`), `0xAA` mark-all-sprites
  (`0x3738`), `0xAB..0xC0` message pass (`0x76DE`), `0xC1..0xF7` sprite
  redraw (`0x80F0`), `0xF8` message clear (`0x813D`), `0xF9` orders
  popup (`0x4309`), `0xFA` endgame monitor (`0x82E5`), `0xFB` pirate
  raid (`0x7C9F`), `0xFC`/`0xFD` faction pop/credits reprint
  (`0x60D4`/`0x60B6`), `0xFE` colony spawn (`0x7EA2`), `0xFF` → day
  rollover. Codes below `0x20` after the machine arm are idle (`ret`).

### Ship tick — `cs:0x60EE` (file `0x360EE`–`0x361E4`)

- `bx = code − 0x20` indexes the `ds:0x9991` fleet array (stride
  `0xC`; 24 codes, one more than the `0x17` the defence scan uses).
- `+6` flag bit `0x2` → skip. `+7` crew/experience byte: `+1` plus
  `+1` for `diff != 2` plus `+1` for `diff == 0` (so `3`/`2`/`1`),
  clamped `0x64` — the same byte the defence pass weights as
  `pow·crew/0x48` and reinforcement grows `+7`.
- UI mode 1 and `si == [0x914C]` (selected): stash `si → [0x9150]`,
  crew `= 0` when `word[si] == 0`, `[0x91D6] = 9 − crew/16` (`0xFF`
  when crew `0`), `[0x91DE] = 0xFF` when crew nonzero; print crew at
  `(0x46,0xC0)` (`0x6886` number print) and the `crew/10`-indexed
  `0x13`-byte status line from `ds:0x7438` at `(0x3B,0x3F)` (`0x6709`).

### HUD reprints — UI mode 1 only

- `0xA7` (`0x60A8`): `si = [0x914C]`; print `word[si]` at `(0x2C,0xB)`.
- `0xFD` (`0x60B6`): `si =` faction rec (`0x5E3A`); print credits
  dword `[si+0x36]` at `(0x45,0x32)` via `0x68E0`.
- `0xFC` (`0x60D4`): faction `[si+0x2A]` (population) at `(0x49,0xB)`
  via `0x686C` (space-padded `u16` print, `cmp ax,0x2710` region).
- `0xAA` (`0x3738`): `or byte,0x80` over the `0xFF`-terminated sprite
  pending-flag bytes at `ds:0x7562` — marks all sprites dirty for the
  `0xC1..0xF7` redraw pass (the shipped image's string bytes there are
  stale workspace, not the live table).

### Orders popup — `0xF9` → `cs:0x4309` (file `0x34309`–`0x3442D`)

Gate: `[0x91F4] != 0 && [0x91CD] == 7 && [0x9198] == 0`. Then: box via
`[0x1262]` (`0x1C/6/3/0x41`), header print `ds:0x4F50` at `(0x38,0x1E)`,
pick `bx` from selected record `[0x917C]`: `+0x2E != 0` → `1` (if
`+0x26 == 0`) or `[0x91AC]+1` wrapped `5 → 2`; else `+0x26 != 0` or
kind `7` → `0`, kind `0xA` → `1`, other kinds → fallback print
`ds:0x4F6E` and out. Prints `0x4F8D`/`0x4FA6`, `call 0x2CA9` on the
`bx`-th `0x1E`-byte entry at `ds:0x4EBA`, clears `[0x91A0]`/`[0x91D7]`,
`lodsw`-copies word → `[0x9198]` + 12 words → `[0x9255..0x926C]` +
byte → `[0x91D6]`, refresh `[0x1264]`, `0x7D0` spin, `call 0x5C49`,
`[0x91F4] = 0`. The `ds:0x4F50`-region strings live in runtime-built
menu workspace — the shipped bytes there are stale script slots.

### Endgame monitor — `0xFA` → `cs:0x82E5` (file `0x382E5`–`0x38338`)

Order of tests: faction record (`0x5E3A`) kind `7` → **A**; record `0`
kind `0xA` → **B**; else the `cs:0x37C7` planet scan
(`bp = #kind-0xA`, `ax = #kind-7` over `[0x91B8]` records): `ax == 0`
→ **A**, `bp == 0` → **B**, else `ret` (`cs:0x8308` — the `74 01`
skips it, so contested galaxies stay quiet).

- **A** (`0x8322`): `[0x91F1] = 0xFF`, `[0x9C9A] = 2`, calls
  `0xA3BC`/`0x21BF`/`0x215B`/`0xA36A`, `jmp 0x8B17`.
- **B** (`0x8309`): `[0x91F0] = 0xFF`, `[0x9C9A] = 3`, calls
  `0xA3BC`/`0x20F2`/`0x208E`/`0xA36A`, `jmp 0x8C65`.
- `0x8B17`/`0x8C65` are full-screen sequences: `0xA3BC` + sound `al`,
  `mov ds,0x95A` + `call far [cs:0x1278]` image load, palette/present
  slots. Which ending is victory vs defeat is unverified — the
  conditions only say A ⇔ (faction kind `7` ∨ no kind-`7` planets),
  B ⇔ (rec0 kind `0xA` ∨ no kind-`0xA` planets).

## Event VM — `cs:0x773D` (file `0x3773D`–`0x37C61`)

Tick codes `0x38+2N .. 0x56+2N` index a 4-byte event table at
`ds:0x5183`; word 0 is the script `si`. Fetch loop `cs:0x7758`:
`bx = [si]; jmp word [bx + 0x50E3]` — the `ds:0x50E3` op table maps
script words to the inline handlers `cs:0x7768..0x7C61` (the table is
runtime-initialised; the shipped image holds unrelated script bytes
there — its words look like `{ptr, 0x0FAA}` slots). Handlers either
`jmp 0x7758` (continue) or `ret` (suspend the event). **`si` is never
written back** — a suspended event re-runs from its table entry next
tick, so scripts are guard+action rules; persistence lives in `ds`
flag cells (`0x82xx`), not the instruction pointer.

Operand encodings: bare `u16` immediate (`si += 2`), or a 4-byte slot
`{ptr16, tag}` (`si += 4`; the shipped tag is `0x0FAA`, consumed but
semantically unused).

| handler | op |
|---|---|
| `0x7768`/`0x7AF0` | bare suspend (`ret`) |
| `0x7769`/`0x78C7` | wait `byte[p] != 0` / `== 0` |
| `0x7774` | print `0xFF`-terminated string at slot (`call 0x6B06`) |
| `0x7782`/`0x778C` | `byte[p] = 0xFF` / `0` |
| `0x7796` | wait `byte[p] == imm8` |
| `0x77A5`/`0x77B4` | wait `word[p] == imm16` (wide = slot operand) |
| `0x77C3`/`0x77E1`/`0x7BE8` | print name of current/selected/found record (`+0xE`) |
| `0x77F0` | `[0x81EE]` = random planet record |
| `0x7811`/`0x7825`/`0x7839` | indirect load 1/2/4 bytes → `0x81F7`/`0x81F8`/`0x81FA` |
| `0x7853`/`0x7866`/`0x7879` | copy 1/2/4 bytes slot→slot |
| `0x7893`/`0x78A2`/`0x78B1` | `+imm` on byte/word/dword at slot (`adc` carry) |
| `0x78D5`/`0x78E4`/`0x78F3` | store byte/word/dword immediate at slot |
| `0x7908` | `jmp word [si]` — escape to a `cs:` routine; suspends |
| `0x790A` | destroy selected planet (`0x81EE`): unlink ships, kind 4 |
| `0x7980`/`0x798E`/`0x799C` | wait selected record kind `0xA`/`7`/`4` |
| `0x79AA` | countdown cell: suspend decrementing while nonzero |
| `0x79BB`/`0x79C3` | menu service / modal dialog block (`cs:0xE713` flag) |
| `0x7A11` | levy: pull food to `0x186`, dump excess-`0x384` into `+0x32` |
| `0x7A45` | disaster fanfare: modal print, `0x924D` flash loop, halve food+fuel |
| `0x7ADD` | clear flag `0x20` on all `0x20` machines |
| `0x7AF1` | raze machines on selected planet + matching tail machine-link slots |
| `0x7BAB` | wait: find derelict (type 7, flags `0x30`) off current planet |
| `0x7BF4`/`0x7C05` | depopulate selected (`+0x2A=0`) / charge (`+0x34=0x74CC`) |
| `0x7C16` | zero all machine build timers `+0x0C` |
| `0x7C30` | wait: harvest ripe farm (type 5, flag `0x4`, `+0x0A ≥ 0x4B0`) |

The tick-direct events (`0x7C62` tribute, `0x7C9F` pirate + `0x7D5A`
victim picker, `0x7EA2` colony spawn + `0x7FA5` name generator, and
the `0x76DE`/`0x80F0`/`0x813D`/`0x7E36` UI/sound passes) share the
same `ret` contract — they suspend the slot after firing.

Rust port: `game::{Vm, Ctx, run, VmHost, Call, NullHost}` across
`rust/src/game/{vm,vhost,vops,vimpl,vplan,vmac,vdir,vev,vup}.rs`;
tests in `rust/tests/game_vm.rs`.

## Day rollover — file `0x36A7B` region

`[0x91BA]` tick counter `+1`, wraps `0x41 → 1` while `[0x91BC]` day
`+1`; then day-level UI/resource work. Sound-service flag `[0x91B0]`
cleared per step.

Rust port: `game::{tick_step, Tick, sim_planet, machine_tick,
defence_tick, day_tick, ship_tick, endgame, Battle, MachOut, ShipOut,
Ending}` in `rust/src/game/` (HUD/popup arms `vhud`/`vord` are
module-private, reached through `tick_step`); tests in
`rust/tests/{game_tick,game_planet,game_battle,game_vm,game_hud}.rs`.

## Main loop — `cs:0x395B` (file `0x3395B`–`0x33A44`)

The galaxy-screen frame loop (entered via `jmp 0x395B`, exits
`jmp 0x2CC5` to the UI shell on `[0x9CDA] & 2`). One iteration:

1. `0x2CA9` — vsync wait (polls `[es:0x63]+6`, waits for the `0x3DA`
   bit-3 edge).
2. `0x66F3` — `[0x91DB] && uimode==5` → `di=0x7D39; call [0x1266]`.
3. `[0x91EE]` redraw block: `call [0x1260]`, `call [0x125C]`,
   `int 33/ax=8 cx=0x90 dx=0xBC` (clamp mouse-y), `0x3A47` type select,
   `call [0x125E]` `dx=5`.
4. `[0x91EF]` → `0x3AE9` info panel (below).
5. Ambient-sound select on the frame counter `[0x91D4]` and the
   selected type's `+0x12` byte — type 7: every 32 frames `0x26/0x0E`;
   3: every 64 `0x29/0x0D`; 4: every 64 `0x0C/0x0C`; 6: `0x2A/0x0A` on
   frame 0 and `0x21/0x13` on frame `0x80`. All via `cs:0x8577` — the
   `call far [cs:0x127C]` wrapper (`cl+1` to the driver) gated on
   `[0x91D3] != 0xFF`.
6. `[0x91D4] += 1`; `call 0x73D9` (tick_step); `call 0x5C1E` (sequencer);
   `call 0xA369` + `call 0xA10F` — **low-segment** (`cs:0 ↔ file
   0x20000`) input + menu services; `call 0x3C6D` dirty flags.
7. `[0x9CDA] & 2` → exit: `[0x91EA]=0`, `int33 ax=8 cx=0 dx=0xBC`,
   `[0xA81A]=[0x9CC8]`, `jmp 0x2CC5`.

### Draw sequencer — `cs:0x5C1E` (file `0x35C1E`–`0x35C96`)

Per-machine-type animation. `[0x91A0]` delay countdown runs out first;
`[0x91D6]` is the period (`0xFF` disables) and `[0x91D7]` the phase —
commands fire only when the phase wraps to 0. `[0x9198]` is a `cs:`
cursor into a stream of 4-byte `{op, pad}` records:

- `0xFFFF` — end (`0x5C8A`): clears `[0x9198]`/`[0x919A]`.
- `0x01F4` — delay: the *next* record's op word → `[0x91A0]`; both
  records consumed.
- `0x0000` — link: `si = cs:[si]` (next record's op = new stream `cs:`
  ptr); fetch continues without suspending.
- else — image index → `call [0x125A]` (draw). One command per period.

`0x5C97` is the reset/setup entry (clears all four cells; uimode≠0
tail-jumps to `0x5D51`).

### Type select + info panel — `cs:0x3A47`/`cs:0x3AE9`

`0x3A47` (file `0x33A47`): resets the sequencer, indexes the
machine-type table `ds:0x9B12 + [0x91ED]·0x30` into `[0x9154]`, copies
`+6`→`[0x91BE]`, `+0x18`→`[0x9198]` (seq list), `+0x1C`→`[0x91D6]`
(period), draws `[si]` via `[0x125A]`, sets `[0x91DB]` from `+0x1D`
(calling `0x66DE` when set), `0x89C3`, frame image `0x56`, caption
`ds:0x6C43` at `(0x12,0x98)`, difficulty legend `0x6C15/0x6BBC/0x6B38`
at `(0,0xB0)`.

`0x3AE9` (file `0x33AE9`): the full stat reprint — `+0x14`/`+0x2C`
name lines, the three `0x3CAD`/`0x3CDB`/`0x3CF6` deferred services,
then `+6`, `+0x10` (mode 2), `+0xE` (mode≠0), `+0xA` (`0xFFFE` →
`ds:0x7325` "none"), `+4`, `+0xC` (`0xFFFF` → `ds:0x731C`), owned-count
via `0x6632` (scans `ds:0x9491` counting `M_TYPE` matches), `+0xC·+0x13`
total (mode 2, `0xFFFF` → `ds:0x7509`), `+0x1E` (mode≠0). Row y-coords
vary by `[0x91E4]`.

### Menu service — `cs:0xA10F` (file `0x2A10F`–`0x2A2D1`)

Per-frame: `0xA1CA` key path (synthetic `0x7E/0x7D/0xFE/0xFD` codes when
`K` off; `K` mode: Enter `0x1C`/Esc `0x01` press, `0x81`/`0x9C` release,
else arrow nav over `+0x10..0x13` links with Tandy `0x29/0x4A/0x2B/0x4E`
alternates; no pending key → `0xA2D2` cursor-snap easing ⅛ toward the
hotspot centre, `y ≤ 0xBC`). Then `[0x91D5]&1` suppresses; debounce via
`[0x91CA]`; hit-test `x1≤cx≤x2, y1≤dy≤y2` over `[0x91A8]` records at
`[0x9194]`; on hit: `[0x9188]=+8`, draw `+8` if nonzero, arm, stash
`[0x9CC8]` at `[list−2]`, `jmp [si+0xC]`.

`0x2A369` (the loop's `call 0xA369` target in the low segment) is a
`ret` stub — the real install routine starts at `0xA36A` (`cli`,
`[0x9CCE]=1`, `int33 ax=4` set cursor pos, `call [0x1288]`/`[0x128A]`
cursor off/on, `int33 ax=0xC cx=0x1F es:dx=286B:0x1143` event-handler
install, `sti`).

`0x3C6D` — dirty-flag service: snapshots `[0x9144]`/`[0x9146]` to
`[0x9CBC]`/`[0x9CBE]`, services at most one bit per frame — `0x9144` bit
2 → `0x3CAD` (faction credits reprint), bit 15 → `0x3CDB`, `0x9146` bit
1 → `0x3CF6` — and clears the bit in the live word.

Rust port: `game::{frame_step, Frame, seq_step, select_type,
menu_service}` plus private `panel`; new `Call::{Slot,Image,Sfx,Mouse}`
and `VmHost::cs_word`; tests in
`rust/tests/{game_loop,game_seq,game_menu}.rs`.

## Menu-action targets — the `+0xC` record words

Hotspot records store their action as a **far** `0x286B:` pointer at
`+0xC`/`+0xE` (the shipped templates live inside the save-block image,
file `0x1A3E2`–`0x1B784`; the live lists are runtime-built copies).
Two code windows reference the same routines: record value `V` ⇄
game-window `cs` `V − 0x6D50` ⇄ file `V + 0x292B0`. Screen entries
share the `pop ax` idiom (the `jmp [si+0xC]` abandons the caller's
return address) and end `jmp 0x2CC5`/`0x2D27` back into the shell; the
`N+3` variant runs `call word [0x1274]` first. Named so far:

- Screen entries: `0x3906`/`0x3909` galaxy (`uimode 5` — hands the
  runner `Phase::Galaxy`), `0x2426`/`0x2429`/`0x247F` detail (`uimode
  2`; `0x2426` resyncs via `0x5EE6` first), `0x243D` machine-detail
  variant (host via `0x5085`, header `0x29D9`, body `0x8E10`),
  `0x31F2`/`0x31F5` orders (u3), `0x4BA7`/`0x4BAA` build (u4),
  `0x5F12`/`0x5F15` fleet (u1), `0x5867`/`0x587A` status (u6),
  `0x4036` surface (u7 — kind/`+0x18|1C|20`/`+0x2E` guard,
  `0x6AD6` message / `0x2434` refuse).
- Standalone: `0x23C2` minimap click, `0x2D97` status report,
  `0x305B` new game (tail `0x331C4` → `jmp 0x2D27`), `0x8427`/
  `0x8430` restart (`0x2F91` stage + `jmp 0x30A0`), `0x854A` sound
  toggle, `0x8544`/`0x8539` grid stubs (`[0x91CA]` clear / key-synth
  `0x1C`), `0x80D2..0x80EA` mode-cell setters `1..5` → `[0x91DC]`,
  `0xF294`/`0xF289` record forms of the two stubs.
- Selection nav: `0x5DA1`/`0x5DD3` prev/next (`[0x9164] ± 1` wrapping
  `[0x91C6]`, `0x8386`/`0x5E03`), `0x5E5A` faction select
  (`0x5E3A` rec → `[0x917C]` → `0x5EE6` → `jmp 0x5C97`),
  `0x5EE6` `sync_sel` helper (`[0x917C]` → `[0x9184]`/`[0x9164]`).
- Dialog list (already dispatched): `0x9B71` enter, `0x9BE4` load,
  `0x9C22` save, `0x9C51`/`0x9C58` yes/no, `0x9CE5` cancel,
  `0x9CEB` confirm, `0xF177` restart.
- Still `Call::Native`: the per-screen gameplay actions — the
  detail-list resource transfers (`0x265F`/`0x269C`/`0x26D9`/`0x2719`
  planet→machine via `0x262C` capacity check; `0x2AF7`/`0x2B20`/
  `0x2B49`/`0x2B72` machine→planet, cap `0x7530`; `0x297E`/`0x2ADD`/
  `0x2AEA` machine-cell selects; `0x2B9B` confirm-and-scuttle;
  `0x4DBB`/`0x4E0A` transfer-all), the fleet-launch flow `0x71E3`,
  and the build/orders/fleet inner actions (`0x36BD`–`0x380E` and
  `0x3F18`–`0x71E3` families in the record table). These are game
  logic, tracked under GAM-8.

Rust port: `game::dispatch_action` splits at
`game::screens::dispatch` — `screens/{galaxy,detail,machd,orders,
build,fleet,status,surface,misc}.rs` per family, `selrec::sync_sel`/
`sel_prev`/`sel_next`/`sel_faction`, `shell::re_enter` for the
`0x2D27` tail. Tests in `rust/tests/game_actions.rs`.

## Text layer + frontend — `rust/src/front/`

- `cs:0x68D6` — the `[0x1268]` char put (`al` glyph, `(bx,dx)` cursor;
  `bx` in 4-pixel columns, `dx` in pixel rows). All four variants index
  `al − 0x20` through a 60-entry `u16` pointer table into packed glyph
  records — the tables **ship in the image's data segment** (`ds:` =
  file `0x106A0 + ofs`; dgroup `0xFAA`), not staged by `0x1278`:
  `ds:0x2F19`/`0x2F91`/`0x3009`/`0x3081` are the CGA/TGA/MCG/EGA
  pointer tables (back-to-back, 60×`u16` each — the `0x3009` "120-entry
  table" is really the MCG+EGA pair), then the records: `ds:0x30F9`
  MCG 24-byte opaque 8bpp cells (4 B/row, `di += 0x13C`), `ds:0x3699`
  EGA four `0x168`-byte plane blocks (glyph `i` plane `p` at
  `p*0x168 + i*6`, 6 rows nibble-merged — high nibble for even `bx`,
  low for odd), `ds:0x3C51` CGA 6-byte 2bpp records (`movsb` + the
  `0x2000`-bank toggle), `ds:0x3DB9` TGA 12-byte 4bpp records
  (`movsw` + `+0x2000`/wrap-`0x8000` advance). Colours are baked in —
  the puts take no ink argument. Row start tables `ds:0x28D9`/`0x2A69`/
  `0x2BF9`/`0x2D89` (CGA/TGA/MCG/EGA, 200×`u16`) hold each mode's
  `di(y)` — `y*0x140`, `y*0x28`, `(y>>1)*0x50+(y&1)*0x2000`,
  `(y>>2)*0xA0+(y&3)*0x2000`. Codes `≥ 0x5C` run off the table into
  the next mode's pointers (garbage glyphs — the game only prints
  `0x20..0x5B`).
- `cs:0x686C` — `u16` decimal print space-padded to 5 cells (pads the
  four leading decades, units forced).
- `cs:0x698F` — `u16` unpadded print (skips leading decades; `0` still
  prints `'0'`); sole call site is the `0x6B8F` ticker.
- `cs:0x68E0` — `bp:ax` `u32` print padded to 8 cells (pads decades
  ≥ `0x10000`; `1000..1` forced).
- `cs:0x6B06` — **not** a print: the ticker enqueue. Sound select
  (`0x8A5B`/`0x8A3B`/`0x127C`, muted on `[0x91D3] == 0xFF`), `[0x91E8]
  = 0xFF`, appends `{si,0}` at the `[0x9160]` write cursor (`0xFE`
  wraps to `0x928D`), `[0x91C2] += 1` (cap `0x80`).
- `[0x125C]` `0x2EB6F` — the `rep stosw` region fill (clear screen,
  `ax` colour).
- `[0x1260]` `0x2EDA8` — `int 10/AX=1012` DAC reg 0 from the template
  `cs:0x1384` triplet (template segment maps `cs:`→file `+0x292B0`,
  distinct from the game-code `+0x30000`).

Rust port: `game::{text, num}` (`print_str`, `enqueue`, `put`,
`num_pad`, `num`, `num32`) — all `Call::Text/Print/Num` sites rewired
to emit real `Call::Glyph`s; `front::{Host, Pump, Game, font}` is the
concrete `VmHost` (image draws, glyph puts via the shipped `ds:` font
tables sliced out of the exe image, `0x125C` clear, `0x1260` DAC,
int-33 calls) and `front::window`
(feature `minifb`) presents the buffer + pumps input. `supremacy-run`
binary drives it; tests in `rust/tests/game_front.rs`.

## Still unnamed (next passes)

- `call 0xA479` after every draw (file 0x33729) — screen present / region
  flush?
- word[2] of the template (0x9000 MCG / 0x8000 others) — likely a VRAM or
  aperture parameter.
- The remaining ~29 dispatch slots per mode (`disasm/dispatch_tables.txt`)
  — region boundaries give each a candidate family; naming needs call-site
  xrefs.
- `fcn.*` map from `disasm/functions.txt` (image-offset space; entry0 =
  file 0x31D17) — bulk naming still ahead.
