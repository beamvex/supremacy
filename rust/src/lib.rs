//! Rust rewrite of the reverse-engineered assembly routines from
//! *Supremacy: Your Will Be Done* (Probe Software, 1991), `GAME.EXE`.
//!
//! Every routine is a port of a specific routine in the unpacked 16-bit
//! image (`decompiled/GAME.unpacked.exe`); file offsets cited in each
//! module refer to that file and to `decompiled/disasm/GAME.linear.asm`.
//! See `decompiled/ANALYSIS.md` for the full reverse-engineering report.
//!
//! - [`exepack`]: Microsoft EXEPACK unpacker (stub at file `0x23A90`).
//! - [`lzss`]: the Okumura LZSS decoder used by all four video modes
//!   (file `0x2EBEF`, `0x2EC8C`, `0x2FC5D`, `0x2FCF9`).
//! - [`gph`]: the 12-byte `.GPH` image index records.
//! - [`args`]: the command-line parser (`GAME <C/T/E/M> <K> <P/T/A/R>`,
//!   file `0x31FF8`–`0x3207D`).
//! - [`assets`]: `.BIN`/`.GPH` asset-set loading and record decoding
//!   (loaders at file `0x3208E`–`0x3215A` onwards).
//! - [`palette`]: the master MCGA DAC table (file `0x15034`).
//! - [`audio`]: the `int 0x80` sound drivers' device halves — OPL2
//!   (`0x388`/`0x389`), MPU-401 (`0x330`/`0x331`), PC speaker PIT ch 2 +
//!   `0x61` gate (files `0x3470`–`0x3F52`, `0x0E9F`).
//! - [`platform`]: the DOS services — `int 21h` file wrappers, the int-8
//!   PIT hook, int-9 scancode queue, int-33h mouse (file `0x353D` onwards).
//! - [`video`]: per-mode parameters, the VRAM [`video::Screen`], and the
//!   draw-image-by-index path (`ds:[0x125A]`; routines at file `0x2EB8A`,
//!   `0x2FB36`, `0x30620`, `0x310CE`).
//! - [`game`]: the `0x1F8B`-byte saveable state block (`ds:0x7D39`),
//!   save/load (`0x383CE`/`0x383F3`), the `cs:0x55B2` LCG, the `0x3A`-byte
//!   record layout, galaxy presets and the `cs:0x9DAB` new-game init.

#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]

pub mod args;
pub mod assets;
pub mod audio;
pub mod exepack;
pub mod game;
pub mod gph;
pub mod lzss;
pub mod palette;
pub mod platform;
pub mod video;
