# `audio` — sound device drivers

The device-level halves of the three driver modules behind the game's
`int 0x80` sound interface (`[0x155]` = 1 PC speaker / 2 Tandy / 3 AdLib
/ 4 Roland; selected by the `[0x156]` paragraph offset). All port
traffic goes through the `Ports` trait (`ports.rs`) — `outb(port, val)`
/ `inb(port) -> u8` — so tests record exact byte sequences and hosts map
them to real audio backends. Drivers poll status bits with the asm's
bounded retry budgets (`0xFFFF` reads).

## `Opl<P: Ports>` — AdLib OPL2 (`opl.rs`)

Ports `0x388` (address/status) / `0x389` (data); the AdLib/Roland driver
module at file `0x2900`.

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
  `st0 & 0xE0 == 0` and `st1 & 0xE0 == 0xC0`.
- `key_off(ch)` (`opl_key.rs`, file `0x3E90`) — clears regs `0xA0+ch`
  and `0xB0+ch`, dropping key-on.
- `freq(ch, fnum, block, keybits)` (file `0x3E5D`) — f-num/block/key-on
  write: `0xA0+ch` = f-num lo, `0xB0+ch` =
  `(fnum_hi & 3) | ((block & 7) << 2) | keybits` — the `bh` packing the
  asm builds.

## `Mpu<P: Ports>` — Roland MPU-401 UART (`mpu.rs`)

Ports `0x330` (data) / `0x331` (status+command), file `0x3470` onwards.
Status bits: `0x40` = DSR (set while write buffer full — poll until
clear before writing), `0x80` = DRR (set while no data — poll until
clear before reading). `ack` mirrors the `[0x1C02]` cell: `0xFE` marks a
live MPU-401.

- `reset()` (`mpu_reset.rs`, file `0x3470`) — polls DSR up to `0xFFFF`
  reads, issues `0xFF` (reset) to `0x331` under `cli`, waits DRR, reads
  the `0xFE` ACK into `ack`; `true` when the ACK arrives.
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
driver (file `0x0C00` module, frequency program at `0x0E9F`).

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
`[0x155] ≤ 2` wrappers (`0x389D7`–`0x38ABF`) dispatch into. Dispatch
(`pc_cmd.rs`, table at `cs:0x52`): `ah=0` tick, `ah=1` stop, `ah=2/al`
load song, `ah=3/al` load SFX (15-byte records at `cs:0x461`), `ah=4`
resume.

- `tick` (`pc_tick.rs`, file `0x0DE3`) — SFX sweep then the two music
  channels; each active channel `step`s its stream and `voice`s the
  result to PIT channel 2 (`Speaker::note`'s `0x43`/`0x42`/`0x61`
  sequence).
- `step`/`op` (`pc_step.rs`, `pc_ops.rs`, `cs:0xF3D` + opcode table
  `cs:0x32`) — byte-stream decode: tempo, wait, divisor, envelope and
  jump opcodes.
- `voice` (`pc_voice.rs`, `cs:0xFBD`) — envelope sequence → divisor →
  tremolo/sweep → gate.

The host backend (`front::audio`, see `front.md`) supplies the
concrete `Ports`; the driver's tick is fed at ~50 Hz — the
`install_drivers` int-8 hook (file `0x31E8E`, P/T only) reprograms
PIT channel 0 with divisor `0x5CEC`. The A/R module has its own
timer install (file `0x3552`, `cs:[0x1C00] = 0x5D37`).
