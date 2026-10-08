//! Audio subsystem — the device drivers behind the `int 0x80` sound
//! interface (`[0x155]` = 1 PC speaker / 2 Tandy / 3 `AdLib` / 4 Roland).
//!
//! `GAME.EXE` carries three separate driver modules selected by the
//! `[0x156]` paragraph offset (`decompiled/FUNCTIONS.md`): image `0x0000`
//! (file `0x0C00`) = PC speaker tracker, `0x0D50` (file `0x1950`) = Tandy,
//! `0x1D00` (file `0x2900`) = AdLib/Roland. Each exports the same `int 0x80`
//! `ah`-dispatch contract; the routines here are the device-level halves:
//!
//! - [`Opl`]: `AdLib` OPL2 on `0x388`/`0x389` — `write` (file `0x3EFD`),
//!   the canonical timer-based `detect` (`0x3EA4`), key-off (`0x3E90`)
//!   and the f-num/block note write (`0x3E5D`).
//! - [`Mpu`]: Roland MPU-401 UART on `0x330`/`0x331` — `reset` (`0x3470`),
//!   `command` (`0x34F5`), `send`/`try_read` (`0x350E`/`0x351F`) and the
//!   checksummed `sysex` block sender (`0x34C8`).
//! - [`Speaker`]: PIT channel 2 + port `0x61` gate — `note` (`0x0E9F`)
//!   with the driver's `[0x2E]` last-divisor cache.
//!
//! All port traffic goes through [`Ports`] so tests can record the exact
//! byte sequences and hosts can map them to real audio backends.

mod mpu;
mod mpu_cmd;
mod mpu_reset;
mod mpu_sysex;
mod opl;
mod opl_detect;
mod opl_key;
mod opl_write;
mod ports;
mod speaker;
mod spk_note;

pub use mpu::Mpu;
pub use opl::Opl;
pub use ports::Ports;
pub use speaker::Speaker;
