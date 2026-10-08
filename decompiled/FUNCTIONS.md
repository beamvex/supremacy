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
- Field map (TBD tags until SUPCHT offsets are cracked): `+0x00` word0,
  `+0x0E` name, `+0x22`, `+0x24` owner, `+0x26` diff param, `+0x28`
  serial, `+0x29`, `+0x2A..+0x34` five stock words, `+0x36` credits dword.
- `file 0x36709` — `print_str`: the `0xFF/0xFD`-terminated string
  interpreter used by the message region.
- `file 0x366DE` — `init_planet_names`: 99 × 12-byte records at
  `ds:0x7D39`+… (planet-name table, distinct from the `0x3A` records).

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
