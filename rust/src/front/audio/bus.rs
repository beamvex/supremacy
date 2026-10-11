use crate::platform::PIT_HZ;
use oplon::Opl2;

/// Square-wave amplitude — a modest voice under the OPL.
const SPK_AMP: f32 = 0.25;
/// OPL gain — `render_frame` runs on the hardware ±4084-per-channel
/// exp scale; `/32768` normalises a full 9-voice chord to about ±1.
const OPL_GAIN: f32 = 1.0 / 32768.0;

/// The host's port-mapped sound hardware — the concrete
/// [`Ports`][crate::audio::Ports] sink
/// the drivers write to. PIT channel 2 (`0x43` cmd, `0x42` latch) plus
/// the `0x61` gate synthesise the PC-speaker square wave; `0x388`/
/// `0x389` drive an `oplon` OPL2 (the `AdLib` the `A` mode would use);
/// `0x330`/`0x331` answer as a quiescent MPU-401 so the Roland path
/// sees a live UART (its MIDI bytes are accepted and dropped).
pub struct Bus {
    /// Effective PIT ch2 divisor — `0` encodes 65536 (~18 Hz).
    pub(super) divisor: u16,
    /// Lobyte staged for the lo/hi latch sequence.
    pub(super) lo: u8,
    /// Next `0x42` write is the hibyte (the `0x43` control resets it).
    pub(super) latch_hi: bool,
    /// Port-`0x61` shadow — `inb` returns it.
    pub(super) gate: u8,
    /// `AdLib` address latch (`0x388`).
    pub(super) opl_addr: u8,
    /// The OPL2 chip.
    pub(super) opl: Opl2,
    /// MPU-401 — `0xFF` written to `0x331` arms the reset ACK.
    pub(super) mpu_ack: bool,
    /// Square-wave phase, 0..1 cycles.
    phase: f64,
    /// Output sample rate (speaker step + OPL resample rate).
    rate: u32,
}

impl Bus {
    /// A bus rendering at `rate` Hz; all ports in power-on state.
    #[must_use]
    pub fn new(rate: u32) -> Self {
        Self {
            divisor: 0,
            lo: 0,
            latch_hi: false,
            gate: 0,
            opl_addr: 0,
            opl: Opl2::new(rate),
            mpu_ack: false,
            phase: 0.0,
            rate,
        }
    }

    /// Retarget at a new output rate — rebuilds the OPL resampler
    /// (register state clears, like a fresh detect).
    pub fn set_rate(&mut self, rate: u32) {
        self.rate = rate;
        self.opl = Opl2::new(rate);
    }

    /// The latched PIT ch2 divisor — what the driver last programmed
    /// (`0` encodes 65536). Diagnostics/tests observe the port state
    /// through it.
    #[must_use]
    pub fn divisor(&self) -> u16 {
        self.divisor
    }

    /// One output sample — the speaker's square wave under the OPL's
    /// stereo-mid render, soft-clamped to ±1.
    #[allow(clippy::cast_precision_loss)]
    pub fn sample(&mut self) -> f32 {
        let (l, r) = self.opl.render_frame();
        let opl = (l.saturating_add(r) >> 1) as f32 * OPL_GAIN;
        (self.square() + opl).clamp(-1.0, 1.0)
    }

    /// The `0x42`-programmed channel-2 frequency, `0` when gated off
    /// or past the output Nyquist.
    #[allow(clippy::cast_precision_loss)]
    fn square(&mut self) -> f32 {
        if self.gate & 3 != 3 {
            return 0.0;
        }
        let div = if self.divisor == 0 {
            0x1_0000
        } else {
            u32::from(self.divisor)
        };
        let freq = PIT_HZ as f64 / f64::from(div);
        let step = freq / f64::from(self.rate);
        if step >= 0.5 {
            return 0.0;
        }
        self.phase = (self.phase + step) % 1.0;
        if self.phase < 0.5 {
            SPK_AMP
        } else {
            -SPK_AMP
        }
    }
}
