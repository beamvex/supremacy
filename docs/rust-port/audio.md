# `audio` — sound device drivers

## What this module does

`audio` ports the sound half of the game: the device-level drivers that
talk to the AdLib OPL2 synthesiser, the Roland MPU-401 MIDI interface,
the PC speaker, and the Tandy 3-voice chip — plus the three complete
`int 0x80` tracker drivers the game picks between based on the `[0x155]`
sound index (1 PC speaker / 2 Tandy / 3 AdLib / 4 Roland; the `S`
command-line letter, see [args.md](args.md)).

The module splits naturally into two layers, and knowing which layer a
file lives in saves a lot of confusion:

- **Device halves** — `opl.rs`, `mpu.rs`, `speaker.rs`: thin models of
  the *chips*, porting the asm routines that program them. These know
  nothing about music.
- **Tracker drivers** — `pc.rs`, `tandy.rs`, `ar.rs` + satellites:
  the three `int 0x80` sound engines that *play* songs by turning
  music streams into device writes through the chip halves.

Every `in`/`out` the drivers issue goes through the `Ports` trait
(`ports.rs`) — `outb(port, val)` / `inb(port) -> u8` — so a test can
record the exact byte sequence the asm would emit and a host can map
them onto a real audio backend ([concepts.md](concepts.md) §3 explains
ports; §9 explains the trait pattern). Drivers poll status bits with
the asm's bounded retry budgets (`0xFFFF` reads).

## How the game routes sound

The game talks to *one* sound interface — `int 0x80` — whose `ah`
register is a command:

| `ah` | command | typical use |
|---|---|---|
| `0` | tick | the driver's per-frame update (paced by the timer) |
| `1` | stop | silence all channels |
| `2` (`al` = id) | load song | install a music record |
| `3` (`al` = id) | load sfx | install a sound-effect record |
| `4` | resume | restart after a pause |
| `5`/`6` | cell peek/poke | the `[0x457]`-style driver cell access |
| `≥0x20` | cell read/write | `[bx+0x23]` region — direct driver state |

Which chip actually makes noise depends on `[0x155]`: `≤2` dispatches
into the PC-speaker or Tandy tracker module; `≥3` reaches the
C-compiled AdLib/Roland module through its eight-entry far jump table
(the game's `cs:0x126C` table). The `[0x156]` paragraph offset selects
which int-8 timer hook gets installed — the trackers need a ~50 Hz tick
to advance their streams.

So a `GAME … A` (AdLib) boot and a `GAME … T` (Tandy) boot run *the
same* `int 0x80` calls — the call sites are identical; what differs is
which driver module the vector lands in and therefore which chip's
ports see traffic. The port models this by picking the `drv::Drv`
variant from `[0x155]` at `Audio::new` time (see [front.md](front.md)).

## `Opl<P: Ports>` — AdLib OPL2 (`opl.rs`)

Ports `0x388` (address/status) / `0x389` (data); the AdLib/Roland driver
module at file `0x2900`. The OPL2 is a Yamaha FM synth: you write a
register number to `0x388`, wait, then write the value to `0x389` — the
chip *requires* those delays between address and data writes, which is
why the routines below read the status port a fixed number of times.
(The reads aren't checking anything — they're the cheapest way to burn
the microseconds the chip needs; the asm `in al,0x38…` loops are delay
loops, and the port keeps the same counts.)

The OPL2's register space is worth a sentence: `0xA0+ch`/`0xB0+ch`
hold a channel's frequency (10-bit "f-num") and octave ("block") plus
the key-on bit — `freq`/`key_off` below program exactly those. Other
register banks (`0x20`+ = tremolo/vibrato per operator, `0x40`+ =
volume, `0x60`+/`0x80`+ = envelope attack/decay/sustain/release,
`0xC0`+ = feedback/algorithm) are what the song streams write
indirectly through the driver.

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
  "AdLib present" meant in 1991. The cleverness: the timer is part of
  the *chip's* logic, so a timer overflow proves silicon, not just a
  responding bus.
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

`ack` mirrors the `[0x1C02]` cell: `0xFE` marks a live MPU-401. (UART
mode is the lazy mode: the MPU-401 also has an intelligent mode that
sequences MIDI itself, but the game just shoves bytes through the dumb
pipe and does all timing in its own driver.)

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
  payloads with counts implied. (SysEx = MIDI system-exclusive message:
  `F0 … F7` frames with a manufacturer checksum — Roland timbre
  uploads go through this.)

## `Speaker<P: Ports>` — PC speaker (`speaker.rs`)

PIT channel 2 + port-`0x61` gate; the `note`/`silence` halves of the PC
driver (file `0x0C00` module, frequency program at `0x0E9F`). The PC
speaker can't be programmed directly — you program the PIT's channel 2
to a square wave, then open the speaker gate bits in port `0x61`.
(The PIT channel 2 output is wired *through* the gate to the speaker
cone: `0x61` bit 1 is the data enable, bit 0 gates timer 2's output —
both must be set for a tone.)

- `note(divisor)` (`spk_note.rs`) — reprograms channel 2 only when the
  divisor differs from the `[0x2E]` last-divisor cache (`last`):
  `out 0x43 = 0xB6` (ch 2, lo/hi, mode 3 square), divisor lo/hi to
  `0x42`; then `in 0x61 | 3` back out (bits 0–1 = timer-2 gate +
  speaker-data enable). The `last`-divisor cache is load-bearing —
  rewriting the PIT mid-note produces an audible click, so the driver
  skips same-divisor programs.
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
cells `[0x23..0x31]` plus the `cs:` bytes beyond — a generic
read/write-anything escape the game uses for driver introspection.

- `tick` (`pc_tick.rs`, file `0x0DE3`) — `[0x29] += [0x25]` carry
  skips the music block; else `[0x28] += [0x26]` overflow `step`s the
  two channels and each `voice`s its divisor; the `[0x13E]` mux picks
  ch0/ch1, the `0xE41` sfx sweep may override, and the `0xE9F` tail
  programs PIT channel 2 (`Speaker::note`'s `0x43`/`0x42`/`0x61`
  sequence). The `+=`-and-overflow pattern is how the asm paces tempo:
  a fractional accumulator that only steps the engine when it carries —
  cheap fixed-point timing, so tempo is a ratio of two cells rather
  than a division.
- `step`/`op` (`pc_step.rs`, `pc_ops.rs`, `cs:0xF3D` + opcode table
  `cs:0x32`) — byte-stream decode. `0x85` advances the `+0x02`
  stream-list cursor (wrap on a `0` word); `0x87` is the end marker —
  its `pop ax` drops the step frame so the `ret` lands on the ISR
  epilogue, ending the whole `int 0x80` call (the tick's remaining work
  is skipped) — an assembly trick the port reproduces by returning
  early.
- `voice` (`pc_voice.rs`, `cs:0xFBD`) — envelope sequence → divisor →
  tremolo (`+0x11` walks −depth to a `0` flip / +depth to the `+0x0F`
  limit — the `sub` half jumps over the `cmp`)/sweep → gate. This is
  the "make a beep sound like music" layer: envelope shapes volume over
  a note's life, tremolo wobbles it, sweep slides the pitch.

## `Tandy<P: Ports>` — Tandy 3-voice tracker (`tandy.rs` + `tandy_*.rs`)

The `int 0x80` driver module at file `0x1950` (image `0x0D50`,
installed at `[0x156] = 0xD5`). Same `ah`-dispatch contract as `Pc`
(`tandy_cmd.rs`, table `cs:0x52`) but four `0x1E`-byte channel records
(`cs:0xEC + 0x1E·i`) rendered to the SN76489 PSG on port `0xC0`
(tone latches `0x80/0xA0/0xC0`, noise `0xE0`, attenuators
`0x90/0xB0/0xD0/0xF0` — the Tandy/PCjr sound chip's 3 tone + 1 noise
voices).

The SN76489 is a much simpler device than the OPL2: one port (`0xC0`),
one byte per register write, and each write selects a channel+register
in its high nibble. `0x80|0xA0|0xC0` latch tone periods on channels
0–2, `0xE0` configures the noise generator, and `0x90|0xB0|0xD0|0xF0`
set per-voice attenuation (volume) — the tracker's job is walking the
streams and latching these at the right ticks.

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

That last detail matters: `Pc`/`Tandy` are *ticked* by the game's
`int 0x80 ah=0`; `Ar` installs its own interrupt handler and services
itself asynchronously. So the AdLib driver never appears in a tick
call chain — it just hears port writes when the game calls the far
exports.

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
(`mode == 1`: note/vol/bend/program/pan + `syx` blocks). The `ar_*`
satellite files are this interpreter's arms — `ar_ach`/`ar_rch` the
channel ops, `ar_aops`/`ar_reg` the OPL register writes, `ar_song` the
song-record parser, `ar_tick` the queue walk, `ar_av`/`ar_rsv` the
volume/reserve helpers.

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

So the signal path in the running port is: game code emits
`Call::Sfx`/`Call::Song` → `front::audio::engine` maps them onto the
`ah` commands the asm wrappers would send → `Drv::{Pc,Tandy,Ar}`
advances its tracker → the tracker issues `outb`/`inb` through `Bus` →
`Bus`'s synth models produce PCM → `front::audio::Out` (cpal) plays it.

## Pitfalls

- **`Ports` is the only hardware boundary.** A driver that "writes to
  port 0x388" is calling `ports.outb(0x388, v)` — nothing here touches
  a real device. Tests record the port sequence; `Bus` synthesises it.
- **The delays are semantics.** `write`'s 6+37 status reads aren't
  busy-wait flavour — the OPL2's timing contract; tests assert them.
- **`[0x155]` picks the engine, not the chip.** `Pc` and `Tandy` are
  trackers; `Ar` is a compiled module with a far-call table — they
  differ in *shape*, not just output device.
- **Tie cells to saves.** Driver scratch (`[0x457]`, the `≥0x20`
  peek/poke region) is `cs:`/`ds:` state — VM scripts and savegames
  can legitimately read it.

## File map

| file | item |
|---|---|
| `ports.rs` | `Ports` trait — `outb`/`inb` |
| `opl.rs`, `opl_write.rs`, `opl_detect.rs`, `opl_key.rs` | OPL2 device half |
| `mpu.rs`, `mpu_reset.rs`, `mpu_cmd.rs`, `mpu_sysex.rs` | MPU-401 UART half |
| `speaker.rs`, `spk_note.rs` | PC-speaker gate + PIT ch-2 program |
| `pc.rs`, `pc_cmd.rs`, `pc_tick.rs`, `pc_step.rs`, `pc_ops.rs`, `pc_voice.rs` | PC-speaker tracker |
| `tandy.rs`, `tandy_cmd.rs`, `tandy_tick.rs`, `tandy_step.rs`, `tandy_ops.rs`, `tandy_voice.rs`, `tandy_sfx.rs` | Tandy 3-voice tracker |
| `ar.rs`, `ar_init.rs`, `ar_cmd.rs`, `ar_tick.rs`, `ar_song.rs`, `ar_ach.rs`, `ar_rch.rs`, `ar_aops.rs`, `ar_reg.rs`, `ar_av.rs`, `ar_rsv.rs`, `ar_util.rs`, `ar_chan.rs`, `ar_ainit.rs` | the C-compiled AdLib/Roland module + its queue interpreter |
| `mod.rs` | module docs, re-exports |

## Worked example — an OPL2 note write

`freq(2, 0x205, 4, 0x20)` on `Bus` produces this port sequence:
`out(0x388, 0xA2)` → 6 `in(0x388)` → `out(0x389, 0x05)` → 37
`in(0x388)` → `out(0x388, 0xB2)` → 6 `in` → `out(0x389, (0x02 & 3) |
(4<<2) | 0x20 = 0x32)` → 37 `in`. A test asserting "channel 2 keyed
on at block 4" checks exactly that byte list — the port-traffic
recording is the spec.
