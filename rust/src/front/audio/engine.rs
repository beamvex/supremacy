use crate::args::Sound;
use crate::audio::{Ar, Pc, Tandy};

use super::{Bus, Drv};

/// The P/T driver tick rate — the `install_drivers` int-8 hook (file
/// `0x31E8E`) reprograms PIT channel 0 with `0x5CEC`:
/// `PIT_HZ`/0x5CEC ≈ 50 Hz; each pulse runs the ISR tail at file
/// `0x389D7` — `int 0x80` `ah=0` on the loaded driver.
const TICK_HZ: f64 = 1_193_182.0 / 0x5CEC as f64;
/// The A/R module's own int-8 install (`0x2952`, `cs:[0x1C00]`) —
/// `0x5D37` ≈ the same 50 Hz, driving the export-2 `service`.
const AR_TICK_HZ: f64 = 1_193_182.0 / 0x5D37 as f64;
/// Boot-time output rate; [`Audio::set_rate`] retunes to the stream's.
pub const BOOT_RATE: u32 = 44_100;

/// The host sound engine — maps the game's [`Call`][crate::game::Call]
/// sound ops onto the ported `int 0x80` drivers over the synth
/// [`Bus`], and renders PCM at the output rate.
///
/// `[0x155]` picks the resident module (`[0x156]` = `0`/`0xD5`/
/// `0x1D0`): the `int 0x80` `ah`-wrappers (files `0x38A5B`/`0x38A3B`/
/// `0x38A80`) feed the P/T trackers; the `cs:0x126C` far table —
/// `retf`-stubbed for P/T — reaches the A/R driver's jump-table
/// exports (`[cs:0x1278]` load, `[cs:0x127C]` select,
/// `[cs:0x1280]` reset).
pub struct Audio {
    /// The `[0x156]`-selected driver over the synth bus — `drv.bus()`
    /// is the [`Bus`] `render` pulls samples from.
    pub drv: Drv,
    /// The selected device (`[0x155]`).
    dev: Sound,
    /// Mirrored `[0x91D3]` — the wrappers' gate byte.
    quiet: u8,
    /// int-8 cadence accumulator, in output-sample units.
    acc: f64,
    /// Output rate — the tick accumulator's unit.
    rate: u32,
    /// Calls with no `int 0x80`/far-export route to take.
    pub dropped: usize,
}

impl Audio {
    /// Engine for `dev` over `img`'s driver modules at `rate` Hz.
    #[must_use]
    pub fn new(dev: Sound, img: &[u8], rate: u32) -> Self {
        let drv = match dev {
            Sound::PcSpeaker => Drv::Pc(Pc::new(Bus::new(rate), img)),
            Sound::Tandy => Drv::Tandy(Tandy::new(Bus::new(rate), img)),
            Sound::AdLib | Sound::Roland => Drv::Ar(Box::new(Self::ar(dev, img, rate))),
        };
        Self {
            drv,
            dev,
            quiet: 0,
            acc: 0.0,
            rate,
            dropped: 0,
        }
    }

    /// The A/R module after the `0x31FF2` `call far [cs:0x126C]`
    /// (export 0 `init`) with `ax = [0x155] − 3` — `0` `AdLib`, `1`
    /// Roland; `bh = 0` so the driver's int-8 timer installs.
    fn ar(dev: Sound, img: &[u8], rate: u32) -> Ar<Bus> {
        let mut a = Ar::new(Bus::new(rate), img);
        a.init(u16::from(matches!(dev, Sound::Roland)));
        a
    }

    /// Retune to the stream's negotiated rate.
    pub fn set_rate(&mut self, rate: u32) {
        self.rate = rate;
        self.drv.bus().set_rate(rate);
    }

    /// Mirror `[0x91D3]` — the `0x8A80` song helper only fires when the
    /// cell reads `0` (`0xFF` mutes, anything else idles it).
    pub fn set_quiet(&mut self, v: u8) {
        self.quiet = v;
    }

    /// `Call::Chan` — the sound-channel helpers (`file 0x38A5B`/
    /// `0x38A3B`/`0x38A80`/`0x389FB`/`0x38A1B`): `0x8A5B` stops music
    /// (`ah=1`) then hits the `[cs:0x1280]` export; `0x8A3B` loads the
    /// `al` sfx record (`ah=3`); `0x8A80` loads song `al` (`ah=2`) but
    /// only while `[0x91D3] == 0`; `0x89FB` is `ah=1`/`al=0`;
    /// `0x8A1B` pokes the `pace` cell (`ah=0x32`).
    pub fn chan(&mut self, target: u16, al: u8) {
        match target {
            0x8A5B => {
                self.int80(1, al);
                self.drv.reset();
            }
            0x8A3B => self.int80(3, al),
            0x8A80 => {
                if self.quiet == 0 {
                    self.int80(2, al);
                }
            }
            0x89FB => self.int80(1, 0),
            0x8A1B => self.int80(0x32, al),
            _ => self.dropped += 1,
        }
    }

    /// `Call::Sfx` — the `cs:0x8577` block (file `0x38577`), gated on
    /// `[0x91D3] != 0xFF`: `0x8A5B` (stop-music `ah=1` + the
    /// `[cs:0x1280]` export), `0x8A3B` (load the `al = ax` sfx record,
    /// `ah=3`), then the `[cs:0x127C]` export with `cl` (the caller
    /// already applied the wrapper's `inc cl`).
    pub fn sfx(&mut self, ax: u16, cl: u8) {
        if self.quiet == 0xFF {
            return;
        }
        let al = ax.to_le_bytes()[0]; // `al` = the command's low byte
        self.int80(1, al);
        self.drv.reset();
        self.int80(3, al);
        self.drv.select(cl);
    }

    /// `Call::Sound` — the direct `call far [cs:0x127C]` (`cl`
    /// function selector). Same export as `sfx`'s tail.
    pub fn sound(&mut self, cl: u8) {
        self.drv.select(cl);
    }

    /// `Call::Song` — `call far [cs:0x1278]` (export 3 `load_song`):
    /// the asm sets `ds = seg`, `si = 0` — the record at file offset
    /// `seg·16 + 0xC00` (`0x03D7` → `0x4970`, the embedded songs).
    pub fn song(&mut self, seg: u16, img: &[u8]) {
        let ofs = usize::from(seg) * 16 + 0xC00;
        self.drv.load_song(img.get(ofs..).unwrap_or(&[]));
    }

    /// `Call::SndReset` — `call far [cs:0x1280]` (export 5 `reset`:
    /// `en = 0` + all-notes-off on A/R; a stub on the trackers).
    pub fn snd_reset(&mut self) {
        self.drv.reset();
    }

    /// Fill `out` with the mix — the driver heartbeat interleaved at
    /// ~50 Hz (`TICK_HZ` on P/T, the A/R module's `0x5D37` rate), then
    /// the bus sample each slot.
    pub fn render(&mut self, out: &mut [f32]) {
        let rate = f64::from(self.rate);
        let hz = if matches!(self.drv, Drv::Ar(_)) {
            AR_TICK_HZ
        } else {
            TICK_HZ
        };
        for s in out.iter_mut() {
            self.acc += hz;
            while self.acc >= rate {
                self.acc -= rate;
                self.drv.tick();
            }
            *s = self.drv.bus().sample();
        }
    }

    /// `[0x155] ≤ 2` — the wrappers' shared gate: the `int 0x80` runs
    /// the timer-driven (PC/Tandy) module, never the A/R path.
    fn timer_dev(&self) -> bool {
        matches!(self.dev, Sound::PcSpeaker | Sound::Tandy)
    }

    /// `int 0x80` — driver `dispatch(ah, al)` when the wrapper's gate
    /// would pass it to the P/T module.
    fn int80(&mut self, ah: u8, al: u8) {
        if self.timer_dev() {
            self.drv.dispatch(ah, al);
        }
    }
}
