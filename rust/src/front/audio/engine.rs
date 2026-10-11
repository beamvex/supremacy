use crate::args::Sound;
use crate::audio::Pc;

use super::Bus;

/// The driver tick rate — the `install_drivers` int-8 hook (file
/// `0x31E8E`, P/T only) reprograms PIT channel 0 with `0x5CEC`:
/// `PIT_HZ`/0x5CEC ≈ 50 Hz; each pulse runs the ISR tail at file
/// `0x389D7` — `int 0x80` `ah=0` on the loaded driver. (`0x5D37` is
/// the same ~50 Hz on the A/R side — `cs:[0x1C00]` in that module's
/// own timer install, file `0x3552`.)
const TICK_HZ: f64 = 1_193_182.0 / 0x5CEC as f64;
/// Boot-time output rate; [`Audio::set_rate`] retunes to the stream's.
pub const BOOT_RATE: u32 = 44_100;

/// The host sound engine — maps the game's [`Call`][crate::game::Call]
/// sound ops onto the ported `int 0x80` driver over the synth [`Bus`],
/// and renders PCM at the output rate.
///
/// The `int 0x80` wrappers all gate `[0x155] ≤ 2` before interrupting
/// (file `0x38A5B`/`0x38A3B`/`0x38A80`): for `AdLib`/`Roland` the
/// engine is silent and their `[cs:0x127C]`/`[cs:0x1280]` far exports
/// land in [`Audio::dropped`] — that C driver module (`file 0x2900`)
/// isn't ported. Tandy runs the PC engine as an approximation — same
/// `int 0x80` contract, different (unported) module internals.
pub struct Audio {
    /// The file-`0xC00` driver over the synth bus — `drv.ports` is the
    /// [`Bus`] `render` pulls samples from.
    pub drv: Pc<Bus>,
    /// The selected device (`[0x155]`).
    dev: Sound,
    /// Mirrored `[0x91D3]` — the wrappers' gate byte.
    quiet: u8,
    /// int-8 cadence accumulator, in output-sample units.
    acc: f64,
    /// Output rate — the tick accumulator's unit.
    rate: u32,
    /// A/R far-export calls with no ported driver to run them.
    pub dropped: usize,
}

impl Audio {
    /// Engine for `dev` over `img`'s driver modules at `rate` Hz.
    #[must_use]
    pub fn new(dev: Sound, img: &[u8], rate: u32) -> Self {
        Self {
            drv: Pc::new(Bus::new(rate), img),
            dev,
            quiet: 0,
            acc: 0.0,
            rate,
            dropped: 0,
        }
    }

    /// Retune to the stream's negotiated rate.
    pub fn set_rate(&mut self, rate: u32) {
        self.rate = rate;
        self.drv.ports.set_rate(rate);
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
                self.far();
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
    /// already applied the wrapper's `inc cl`; both exports are `retf`
    /// stubs on the timer drivers — they reach only the unported A/R
    /// module).
    pub fn sfx(&mut self, ax: u16, cl: u8) {
        if self.quiet == 0xFF {
            return;
        }
        let al = ax.to_le_bytes()[0]; // `al` = the command's low byte
        self.int80(1, al);
        self.far();
        self.int80(3, al);
        self.far();
        let _ = cl;
    }

    /// `Call::Sound` — the direct `call far [cs:0x127C]` (`cl`
    /// function selector). Same export as `sfx`'s tail.
    pub fn sound(&mut self, cl: u8) {
        self.far();
        let _ = cl;
    }

    /// Fill `out` with the mix — the `ah=0` driver tick interleaved at
    /// ~50 Hz (`TICK_HZ`, the `0x5CEC` PIT rate), then the bus sample
    /// each slot.
    pub fn render(&mut self, out: &mut [f32]) {
        let rate = f64::from(self.rate);
        for s in out.iter_mut() {
            self.acc += TICK_HZ;
            while self.acc >= rate {
                self.acc -= rate;
                if self.timer_dev() {
                    self.drv.tick();
                }
            }
            *s = self.drv.ports.sample();
        }
    }

    /// `[0x155] ≤ 2` — the wrappers' shared gate: the `int 0x80` runs
    /// the timer-driven (PC/Tandy) module, never on the A/R path.
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

    /// `Call::Song`/`Call::SndReset` — the `[cs:0x1278]`/`[cs:0x1280]`
    /// far-export calls the ending/restart music swaps make; same
    /// accounting as `far`.
    pub fn far_export(&mut self) {
        self.far();
    }

    /// The `[cs:0x127C]`/`[cs:0x1280]` far exports — `retf` stubs on
    /// P/T; on A/R they'd land in the unported C driver, so count them.
    fn far(&mut self) {
        if !self.timer_dev() {
            self.dropped += 1;
        }
    }
}
