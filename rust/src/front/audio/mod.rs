//! Host-side audio — the real output end of the game's sound calls
//! (GAM-31). The VM emits `Call::Sound`/`Call::Chan`/`Call::Sfx`;
//! [`Audio`] maps them onto the ported `int 0x80` contract the asm
//! wrappers implement (`decompiled/FUNCTIONS.md`, files `0x38A5B`,
//! `0x38A3B`, `0x38A80`, `0x38577`), and the driver's port writes land
//! on [`Bus`] — the [`Ports`][crate::audio::Ports] impl that turns
//! them into samples: PIT ch2 + the `0x61` gate become the speaker
//! square wave, `0x388`/`0x389` write an `oplon` OPL2, `0x330`/`0x331`
//! answer as a quiescent MPU-401.
//!
//! `window` (feature `audio`) owns the cpal `Out`: the game thread
//! renders a frame's worth of samples per step and the stream
//! callback drains them.

mod bus;
mod bus_io;
mod drv;
mod engine;
#[cfg(feature = "audio")]
mod out;

pub use bus::Bus;
pub use drv::Drv;
pub use engine::{Audio, BOOT_RATE};
#[cfg(feature = "audio")]
pub use out::Out;
