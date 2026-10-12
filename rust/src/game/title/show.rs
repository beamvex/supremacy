//! `0x8629`–`0x8746` — the slide seq-runs, the retrace wait and the
//! starfield warm-up that carry the attract to the credits loop.

use super::super::consts::{SEQ_PERIOD, SEQ_PTR};
use super::super::seq::seq_step;
use super::super::vend::fx;
use super::super::vhost::Call;
use super::super::vops::{vsync, Ctx};
use super::{Ph, Title};

/// `[0x143]` — the video-mode BIOS number the mode select's template
/// copy leaves (the port's [`crate::args::Video::bios_mode`]); `0x13`
/// gates the MCGA-only slides.
const MODE_WORD: u16 = 0x143;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

impl Title {
    /// `0x8863` — one `0x5C1E` seq step per frame; the loop ends when
    /// the stream hits its `0xFFFF` terminator (`[0x9198] = 0`) and —
    /// like every attract block — early on `[0x9CDA] & 1`/`[0x9CCB]`
    /// (`jnz`/`jz 0x87d7`, the picker install).
    pub(super) fn seqpoll(self, c: &mut Ctx, k: u8) -> Self {
        vsync(c);
        seq_step(c);
        if Self::input(c) {
            return self.enter_pick(c);
        }
        if c.vm.r16(c.st, a!(SEQ_PTR)) != 0 {
            return self;
        }
        self.slide(c, k)
    }

    /// The `0x8640`/`0x887E` advance chain — after slide 0 the
    /// `[0x143] == 0x13` test runs (MCGA-only slides; other modes jump
    /// `0x8637` straight to the wait), then slides 1→2→3 run
    /// unconditionally, and slide 3 falls into the `0x86F5` wait.
    fn slide(self, c: &mut Ctx, k: u8) -> Self {
        if k == 3 || (k == 0 && c.vm.r16(c.st, MODE_WORD) != 0x13) {
            return Self {
                n: 0x7D,
                ..self.at(Ph::Wait)
            };
        }
        self.slide_arm(c, k + 1)
    }

    /// The per-slide present+arm — slide 1 (`0x864B`): refresh,
    /// `Image(5)`, `Present(8)`, `0xB22`; slide 2 (`0x8674`):
    /// `Slot(0x128E,8)`, `Image(0x3B)`, `Present(8)`, `0xC8A`; slide 3
    /// (`0x86A6`): refresh, `Image(0)`, `Present(0xD)`, `0x9F6`. Only
    /// `[0x91D6]`/`[0x9198]` are re-armed — `[0x91A0]`/`[0x91D7]`
    /// carry over exactly like the asm.
    fn slide_arm(self, c: &mut Ctx, k: u8) -> Self {
        let (img, ptr) = match k {
            1 => (5, 0xB22),
            2 => (0x3B, 0xC8A),
            _ => (0, 0x9F6),
        };
        match k {
            2 => _ = c.host.svc(Call::Slot(0x128E, 8)),
            _ => fx::refresh(c),
        }
        c.host.svc(Call::Image(img));
        c.host.svc(Call::Present(if k == 3 { 0xD } else { 8 }));
        c.vm.w16(c.st, a!(SEQ_PERIOD), 8);
        c.vm.w16(c.st, a!(SEQ_PTR), ptr);
        self.at(Ph::Seq(k))
    }

    /// `0x86F5` — the `dx = 0x7D` vsync countdown before the credits
    /// head (`0x7E` frames, `jns` counting `0x7D..=0`).
    pub(super) fn wait(mut self, c: &mut Ctx) -> Self {
        vsync(c);
        if Self::input(c) {
            return self.enter_pick(c);
        }
        if self.n == 0 {
            return self.credits_head(c);
        }
        self.n -= 1;
        self
    }

    /// `0x8701`–`0x871B` — refresh, vsync, `Present(9)`,
    /// `Slot(0x1292,9)` and arm the `cx = 0x63` warm-up.
    fn credits_head(self, c: &mut Ctx) -> Self {
        fx::refresh(c);
        vsync(c);
        c.host.svc(Call::Present(9));
        c.host.svc(Call::Slot(0x1292, 9));
        Self {
            n: 0x63,
            ..self.at(Ph::Warm)
        }
    }

    /// `0x872B` — the `cx = 0x63` `0x8891` warm-up loop; exits to the
    /// `0x8747` credits cursor arm (`Ph::Page`).
    pub(super) fn warm(self, c: &mut Ctx) -> Self {
        self.countdown(c, Ph::Page)
    }
}
