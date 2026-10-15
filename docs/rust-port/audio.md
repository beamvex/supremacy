# `audio` — sound device drivers

## What this module does

`audio` ports the sound half of the game: the device-level drivers that
talk to the AdLib OPL2 synthesiser, the Roland MPU-401 MIDI interface,
the PC speaker, and the Tandy 3-voice chip — plus the three complete
`int 0x80` tracker drivers the game picks between based on the `[0x155]`
sound index (1 PC speaker / 2 Tandy / 3 AdLib / 4 Roland; the `S`
command-line letter, see [args.md](args.md)).

Every `in`/`out` the drivers issue goes through the `Ports` trait
(`ports.rs`) — `outb(port, val)` / `inb(port) -> u8` — so a test can
record the exact byte sequence the asm would emit and a host can map
them onto a real audio backend ([concepts.md](concepts.md) §3 explains
ports; §9 explains the trait pattern). Drivers poll status bits with
the asm's bounded retry budgets (`0xFFFF` reads).

## How the game routes sound

The game talks to *one* sound interface — `int 0x80` — whose `ah`
register is a command (`0` tick, `1` stop, `2` load song, `3` load sfx,
`4` resume, `5`/`6` driver-cell peek/poke, `≥0x20` cell read/write).
Which chip actually makes noise depends on `[0x155]`: `≤2` dispatches
into the PC-speaker or Tandy tracker module; `≥3` reaches the
C-compiled AdLib/Roland module through its eight-entry far jump table
(the game's `cs:0x126C` table). The `[0x156]` paragraph offset selects
which int-8 timer hook gets installed — the trackers need a ~50 Hz tick
to advance their streams.

## `Opl<P: Ports>` — AdLib OPL2 (`opl.rs`)

Ports `0x388` (address/status) / `0x389` (data); the AdLib/Roland driver
module at file `0x2900`. The OPL2 is a Yamaha FM synth: you write a
register number to `0x388`, wait, then write the value to `0x389` — the
chip *requires* those delays between address and data writes, which is
why the routines below read the status port a fixed number of times.

- `write(reg, data)` (`opl_write.rs`, file `0x3EFD`) — address to
  `0x388`, **6 status reads** (~3.3 µs), data to `0x389`, then **37
  status reads** (~23 µs) — the chip's required write delays.
- `delay()` — 38 status reads (`0x3F38`; ×4 in `detect` ≈ 80 µs for
  timer 1 overflow). `status()` returns raw `0x388` (timer flags in
  bits 5–7).
- `detect()` (`opl_detect.rs`, file `0x3EA4`) — the canonical OPL2 timer
  test: mask both timers (`reg 4 = 0x60`), clear flags (`reg 4 = 0x80`),
  snapshot `st0`; load timer 1 (`reg 2 = 0xFF`), start it
  (`reg 4 = 0x21`), wait ~4×38 reads, take `st1`; restore. Detected when
  `st0 & 0xE0 == 0` and `st1 & 0xE0 == 0xC0` — a card that isn't there
  returns all-`0xFF` floats on `0x388`, so the flag check is what
  "AdLib present" meant in 1991.
- `key_off(ch)` (`opl_key.rs`, file `0x3E90`) — clears regs `0xA0+ch`
  and `0xB0+ch`, dropping key-on.
- `freq(ch, fnum, block, keybits)` (file `0x3E5D`) — f-num/block/key-on
  write: `0xA0+ch` = f-num lo, `0xB0+ch` =
  `(fnum_hi & 3) | ((block & 7) << 2) | keybits` — the `bh` packing the
  asm builds; `block` is the octave, `fnum` the frequency number.

## `Mpu<P: Ports>` — Roland MPU-401 UART (`mpu.rs`)

Ports `0x330` (data) / `0x331` (status+command), file `0x3470` onwards.
The MPU-401 is the MIDI interface a Roland MT-32 plugs into; in UART
mode it is a dumb serial pipe with two handshake bits:

- `0x40` = DSR — set while the write buffer is full; poll until clear
  before writing.
- `0x80` = DRR — set while no data is available; poll until clear
  before reading.

`ack` mirrors the `[0x1C02]` cell: `0xFE` marks a live MPU-401.

- `reset()` (`mpu_reset.rs`, file `0x3470`) — polls DSR up to `0xFFFF`
  reads, issues `0xFF` (reset) to `0x331` under `cli`, waits DRR, reads
  the reply: only an exact `0xFE` ACK is kept in `ack` (any other byte
  stores `0`); `true` on the `0xFE`.
- `command(b)` (`mpu_cmd.rs`, file `0x34F5`) — wait DSR, write to `0x331`
  under `cli`, poll DRR, read and return the `0x330` response.
- `send(b)` (file `0x350E`) / `read()` (file `0x351F`) — wait for the
  status bit then transfer one byte.
- `sysex(head, body)` (`mpu_sysex.rs`, file `0x34C8`) — sends `head`
  verbatim, `body` while accumulating a running sum, then
  `(-sum) & 0x7F` (Roland checksum) and `0xF7` (EOX). The asm's on-disk
  records are `[n][bytes]` counted pairs — the slices here are the two
  payloads with counts implied.

## `Speaker<P: Ports>` — PC speaker (`speaker.rs`)

PIT channel 2 + port-`0x61` gate; the `note`/`silence` halves of the PC
driver (file `0x0C00` module, frequency program at `0x0E9F`). The PC
speaker can't be programmed directly — you program the PIT's channel 2
to a square wave, then open the speaker gate bits in port `0x61`.

- `note(divisor)` (`spk_note.rs`) — reprograms channel 2 only when the
  divisor differs from the `[0x2E]` last-divisor cache (`last`):
  `out 0x43 = 0xB6` (ch 2, lo/hi, mode 3 square), divisor lo/hi to
  `0x42`; then `in 0x61 | 3` back out (bits 0–1 = timer-2 gate +
  speaker-data enable).
- `silence()` — clears port-`0x61` bits 0–1 and resets `last`. The
  driver reaches silence by keying channels off; this is the modelled
  counterpart of `note`.

## `Pc<P: Ports>` — PC-speaker tracker (`pc.rs`)

The `int 0x80` driver module at file `0x0C00` — the engine the
`[0x155] ≤ 2` wrappers (`0x389D7`–`0x38ABF`) dispatch into. It is a
two-channel tracker: each channel walks a byte stream of opcodes
(tempo, wait, divisor, envelope, jump) and the `voice` render converts
the current stream position into a PIT divisor. Dispatch
(`pc_cmd.rs`, table at `cs:0x52`): `ah=0` tick, `ah=1` stop, `ah=2/al`
load song, `ah=3/al` load SFX (15-byte records at `cs:0x461`), `ah=4`
resume, `ah=5`/`ah=6` the `[0x457]` peek/poke — which aliases the sfx
`alt` divisor's high byte. `ah ≥ 0x20` is the `[bx+0x23]` cell
peek/poke (`0x20..0x30` read, `0x30..` write) covering the driver
cells `[0x23..0x31]` plus the `cs:` bytes beyond.

- `tick` (`pc_tick.rs`, file `0x0DE3`) — `[0x29] += [0x25]` carry
  skips the music block; else `[0x28] += [0x26]` overflow `step`s the
  two channels and each `voice`s its divisor; the `[0x13E]` mux picks
  ch0/ch1, the `0xE41` sfx sweep may override, and the `0xE9F` tail
  programs PIT channel 2 (`Speaker::note`'s `0x43`/`0x42`/`0x61`
  sequence). The `+=`-and-overflow pattern is how the asm paces tempo:
  a fractional accumulator that only steps the engine when it carries.
- `step`/`op` (`pc_step.rs`, `pc_ops.rs`, `cs:0xF3D` + opcode table
  `cs:0x32`) — byte-stream decode. `0x85` advances the `+0x02`
  stream-list cursor (wrap on a `0` word); `0x87` is the end marker —
  its `pop ax` drops the step frame so the `ret` lands on the ISR
  epilogue, ending the whole `int 0x80` call (the tick's remaining work
  is skipped) — an assembly trick the port reproduces by returning
  early.
- `voice` (`pc_voice.rs`, `cs:0xFBD`) — envelope sequence → divisor →
  tremolo (`+0x11` walks −depth to a `0` flip / +depth to the `+0x0F`
  limit — the `sub` half jumps over the `cmp`)/sweep → gate.

## `Tandy<P: Ports>` — Tandy 3-voice tracker (`tandy.rs` + `tandy_*.rs`)

The `int 0x80` driver module at file `0x1950` (image `0x0D50`,
installed at `[0x156] = 0xD5`). Same `ah`-dispatch contract as `Pc`
(`tandy_cmd.rs`, table `cs:0x52`) but four `0x1E`-byte channel records
(`cs:0xEC + 0x1E·i`) rendered to the SN76489 PSG on port `0xC0`
(tone latches `0x80/0xA0/0xC0`, noise `0xE0`, attenuators
`0x90/0xB0/0xD0/0xF0` — the Tandy/PCjr sound chip's 3 tone + 1 noise
voices).

- `tick` (`tandy_tick.rs`, file `0x1B6B`) — `[0x2A] += [0x25]` paces
  the engine: a carry jumps straight to the `0x61 | 0x60` sound-enable
  tail, so overflow ticks render nothing. Otherwise `[0x29] += [0x27]`
  overflow steps all four `TChan` records, then the render pass.
- `step`/`opcode` (`tandy_step.rs`, `tandy_ops.rs`, file `0x1C02` +
  table `cs:0x36`) — the stream decode: `< 0x80` notes (bit7-tied
  notes skip the vibrato/envelope reset), `0x80..0x8D` opcodes (mute,
  tremolo, sweep, slide, list/next-stream, transpose, tempo, master
  volume), `0xA0..` selects. `0x86` pops the step frame — `stop` +
  one render/gate pass ends the tick outright.
- `voice`/`music` (`tandy_voice.rs`, file `0x1DAE`/`0x1DBE`) — env
  (`cs:0x87C` `{add,wait}` pairs seeded via `cs:0x85F`), the
  `stc; adc` master-volume fold, tremolo (`cs:0x7E2`), vibrato
  (`cs:0x847`), sweep, transpose (ch3 skips `[0x2B]`).
- `sfx` (`tandy_sfx.rs`, file `0x1CD8`) — the two-voice sweep on the
  `≥ 0xD0`-latched channels, gated on `[0x24] & 0x81` (the `test`'s
  sign bit picks engine vs a bare `ret` — the transient `1` skips the
  channel silently). It runs once a tick inside ch2's render slot:
  its `ret` pops the render call, so ch3 is skipped while it holds;
  the `end` countdown disarms and falls back to `music`.

## `Ar<P: Ports>` — AdLib/Roland driver (`ar.rs` + `ar_*.rs`)

The C-compiled file-`0x2900` module (image `0x1D00`, installed at
`[0x156] = 0x1D0`). Where `Pc`/`Tandy` are hand-written trackers, `Ar`
is a compiled driver whose exports are the eight 3-byte `jmp`s at
`cs:0`; the game's far table `cs:0x126C`–`0x1282` holds the six
`01D0:xxxx` pointers to the first six (`install_drivers` `0x31EF7`
`retf`-stubs them when `[0x155] ≤ 2`). `[cs:0x1270]`/`[cs:0x1274]`
have no call sites — shutdown is unreferenced and service runs
through the installed int-8 vector (module `0xCBF`, file `0x35BF`:
EOI + every-3rd-tick BIOS chain via `[cs:0x1666]`, then the service
call at the `0x5D37` rate).

| export | `cs:` | file | port |
|---|---|---|---|
| 0 `init` | `0x000` | `0x2998` | `Ar::init(ax)` — `al` 0=`0x2D3B` OPL / 1=`0x27CF` MPU; `bh` skips `0x2952` install |
| 1 `shutdown` | `0x003` | `0x29CC` | `Ar::shutdown(al)` — `al`-picked re-`init` when live, `mode = 0xFF`, `0x298C` uninstall |
| 2 `service` | `0x006` | `0x2EBC` | `Ar::service()` — `0x1F05` fader, then `div`-cadence `0x24AC`/`0x23EF` fetch/apply |
| 3 `load_song` | `0x009` | `0x2C81` | `Ar::load_song(rec)` — `0x118` hdr + `0x200` tbl, `qbase`/`qend`, Roland `0x2104` timbres |
| 4 `select` | `0x00C` | `0x29EE` | `Ar::select(cl)` — all-off, `map` bound, vol reset, `idx`/`cur`/`en` |
| 5 `reset` | `0x00F` | `0x2A7C` | `Ar::reset()` — `en = 0` + `0x1FE0` all-off |
| 6 `fade_set` | `0x012` | `0x2AB7` | `Ar::fade_set(cl)` — arm the fader (`cl` doubled on OPL) |
| 7 `fade_query` | `0x015` | `0x2AA9` | `Ar::fade_query()` — the `[0x2060]` armed flag |

Game-side routing (`front/audio/engine.rs`, `drv.rs`): the
`call far [cs:0x126C]` at `0x31FF2` is `Audio::new`'s `init` with
`ax = [0x155] − 3`; `Call::Song` → `[cs:0x1278]` (4 sites),
`Call::Sound`/`Sfx` → `[cs:0x127C]` (56 sites), `Call::SndReset` +
the `0x8A5B` stop → `[cs:0x1280]` (5 sites). The `int 0x80`
`ah`-wrappers gate on `[0x155] ≤ 2` so the P/T `dispatch` path is
never taken with A/R resident (the vector points at the jump table
itself — export 0).

State mirrors the `cs:` workspace cells (`[0x19AC]` mode,
`[0x1C05]` `en`, `[0x1C02]` MPU ack, `hdr`/`tbl` at `0x1673`/`0x178C`,
the eight `0x20`-byte `AChan` records at `0x1A00`, the `0x1C1D` staging
block and the `0x1BED`/`0x1FF8`/`0x204B` `SysEx` buffers). The queue
interpreter decodes each song's stream region into OPL writes
(`mode == 0`: patch/vol/note/key + rhythm/`4op` staging) or MPU MIDI
(`mode == 1`: note/vol/bend/program/pan + `syx` blocks).

## The host backend (`front::audio`)

The concrete `Ports` the drivers run against — `front::audio::Bus`
synthesises each device they address: the `0x61`-gated PIT-2 square
wave, the SN76489 the `0xC0` bytes feed (gated on the `0x61 | 0x60`
Tandy enable), an `oplon` OPL2 on `0x388`/`0x389`, and a quiescent
MPU-401 on `0x330`/`0x331` (MIDI bytes accepted and dropped — there is
no Roland renderer). The driver's tick is fed at ~50 Hz — the
`install_drivers` int-8 hook (file `0x31E8E`, P/T only) reprograms PIT
channel 0 with divisor `0x5CEC`. The A/R module has its own timer
install (file `0x3552`, `cs:[0x1C00] = 0x5D37`). See [front.md](front.md)
§ `audio`.
