use crate::audio::{Ar, Pc, Tandy};

use super::Bus;

/// The resident driver the `[0x155]` selector installed — one module
/// per device, all over the same synth [`Bus`]:
///
/// - [`Drv::Pc`]: `[0x156] = 0` — the file-`0xC00` tracker (PIT ch2 +
///   the `0x61` gate).
/// - [`Drv::Tandy`]: `[0x156] = 0xD5` — the file-`0x1950` tracker on
///   the SN76489 (`0xC0` latch port).
/// - [`Drv::Ar`]: `[0x156] = 0x1D0` — the file-`0x2900` C driver
///   (`AdLib` OPL2 or Roland MPU-401 per the `init` `ax` arg).
pub enum Drv {
    /// The PC-speaker module.
    Pc(Pc<Bus>),
    /// The Tandy module.
    Tandy(Tandy<Bus>),
    /// The `AdLib`/Roland module (boxed — its channel/buffer state
    /// dwarfs the tracker records).
    Ar(Box<Ar<Bus>>),
}

impl Drv {
    /// The port bus the driver writes — `render`/`set_rate` reach the
    /// synth through it.
    pub fn bus(&mut self) -> &mut Bus {
        match self {
            Self::Pc(d) => &mut d.ports,
            Self::Tandy(d) => &mut d.ports,
            Self::Ar(d) => &mut d.ports,
        }
    }

    /// The driver's ~50 Hz heartbeat — `tick` on the trackers; the
    /// int-8 `service` entry (jump-table export 2) on the A/R module.
    pub fn tick(&mut self) {
        match self {
            Self::Pc(d) => d.tick(),
            Self::Tandy(d) => d.tick(),
            Self::Ar(d) => d.service(),
        }
    }

    /// `int 0x80` `ah`-dispatch — the trackers' contract. The asm
    /// wrappers gate `[0x155] ≤ 2`, so the A/R arm is unreachable.
    pub fn dispatch(&mut self, ah: u8, al: u8) {
        match self {
            Self::Pc(d) => _ = d.dispatch(ah, al),
            Self::Tandy(d) => _ = d.dispatch(ah, al),
            Self::Ar(_) => {}
        }
    }

    /// `call far [cs:0x1278]` — export 3 `load_song`; a `retf` stub on
    /// the timer drivers (`install_drivers` fills the table with
    /// stubs when `[0x155] ≤ 2`).
    pub fn load_song(&mut self, rec: &[u8]) {
        if let Self::Ar(d) = self {
            d.load_song(rec);
        }
    }

    /// `call far [cs:0x127C]` — export 4 `select_sound` (`cl` = the
    /// sound index into the loaded song's `map`); stub on the
    /// trackers.
    pub fn select(&mut self, cl: u8) {
        if let Self::Ar(d) = self {
            d.select(cl);
        }
    }

    /// `call far [cs:0x1280]` — export 5 `reset` (`en = 0` +
    /// all-notes-off); stub on the trackers.
    pub fn reset(&mut self) {
        if let Self::Ar(d) = self {
            d.reset();
        }
    }
}
