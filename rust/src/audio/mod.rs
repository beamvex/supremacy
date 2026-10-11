//! Audio subsystem — the device drivers behind the `int 0x80` sound
//! interface (`[0x155]` = 1 PC speaker / 2 Tandy / 3 `AdLib` / 4 Roland).
//!
//! `GAME.EXE` carries three separate driver modules selected by the
//! `[0x156]` paragraph offset (`decompiled/FUNCTIONS.md`): image `0x0000`
//! (file `0x0C00`) = PC speaker tracker, `0x0D50` (file `0x1950`) = Tandy,
//! `0x1D00` (file `0x2900`) = `AdLib`/Roland. Each exports the same `int 0x80`
//! `ah`-dispatch contract (the A/R module's is its `jmp`-table far
//! exports at `cs:0x126C`); the routines here cover both layers:
//!
//! - [`Opl`]: `AdLib` OPL2 on `0x388`/`0x389` — `write` (file `0x3EFD`),
//!   the canonical timer-based `detect` (`0x3EA4`), key-off (`0x3E90`)
//!   and the f-num/block note write (`0x3E5D`).
//! - [`Mpu`]: Roland MPU-401 UART on `0x330`/`0x331` — `reset` (`0x3470`),
//!   `command` (`0x34F5`), `send`/`try_read` (`0x350E`/`0x351F`) and the
//!   checksummed `sysex` block sender (`0x34C8`).
//! - [`Speaker`]: PIT channel 2 + port `0x61` gate — `note` (`0x0E9F`)
//!   with the driver's `[0x2E]` last-divisor cache.
//! - [`Pc`]: the whole file-`0xC00` PC-speaker driver — the `int 0x80`
//!   `ah`-dispatch ([`Pc::dispatch`]), the `int 8`-rate tick engine
//!   (`0xDE3`), the two tracker channels' stream decode (`0xF3D`),
//!   opcode table (`cs:0x32`), voice render (`0xFBD`) and the
//!   `cs:0x450` sfx sweep — feeding PIT divisors to its [`Ports`] bus.
//! - [`Tandy`]: the file-`0x1950` driver — same `ah`-dispatch
//!   ([`Tandy::dispatch`]), four `0x1E`-byte channel records decoded
//!   by the same stream engine but rendered to an SN76489 on `0xC0`
//!   (tone latches `0x80/0xA0/0xC0`, noise `0xE0`, attenuators
//!   `0x90/0xB0/0xD0/0xF0`), the `cs:0x4EA` sfx sweep on channels
//!   2/3, and the `0x61 | 0x60` Tandy sound-enable write.
//! - [`Ar`]: the C-compiled file-`0x2900` `AdLib`/Roland driver — the
//!   eight-entry `jmp` table at `cs:0` (the game's far table
//!   `cs:0x126C` calls six of them): [`Ar::init`] (export 0),
//!   [`Ar::shutdown`] (1), [`Ar::service`] (2 — also the int-8 ISR),
//!   [`Ar::load_song`] (3), [`Ar::select`] (4), [`Ar::reset`] (5),
//!   [`Ar::fade_set`]/[`Ar::fade_query`] (6/7). The queue interpreter
//!   feeds OPL registers (`mode == 0`) or MPU-401 MIDI (`mode == 1`).
//!
//! All port traffic goes through [`Ports`] so tests can record the exact
//! byte sequences and hosts can `map` them to real audio backends.

mod ar;
mod ar_ach;
mod ar_ainit;
mod ar_aops;
mod ar_areg;
mod ar_av;
mod ar_chan;
mod ar_cmd;
mod ar_init;
mod ar_rch;
mod ar_rsv;
mod ar_song;
mod ar_tick;
mod ar_util;
mod mpu;
mod mpu_cmd;
mod mpu_reset;
mod mpu_sysex;
mod opl;
mod opl_detect;
mod opl_key;
mod opl_write;
mod pc;
mod pc_cmd;
mod pc_ops;
mod pc_step;
mod pc_tick;
mod pc_voice;
mod ports;
mod speaker;
mod spk_note;
mod tandy;
mod tandy_cmd;
mod tandy_ops;
mod tandy_sfx;
mod tandy_step;
mod tandy_tick;
mod tandy_voice;

pub use ar::Ar;
pub use mpu::Mpu;
pub use opl::Opl;
pub use pc::{Chan, Pc, Sfx};
pub use ports::Ports;
pub use speaker::Speaker;
pub use tandy::{TChan, Tandy};
