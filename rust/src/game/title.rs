//! `cs:0x85B1` — the title attract and scenario picker (file
//! `0x385B1`–`0x389B2`), run once at boot (entry stub `call 0x85B1`)
//! and again inside the `0x1D35` restart tail after either ending.
//!
//! The asm blocks through the whole thing: the opening title slide +
//! `0x8863` seq-run, two extra MCGA slides (`[0x143] == 0x13` only),
//! a `0x7D`-frame retrace wait, the `0x63`-frame starfield warm-up and
//! the repeating `menu_text` credits loop — every boundary polled for
//! `[0x9CCB]`/`[0x9CDA]`, any input jumping to the `0x87D7` picker
//! install: mouse setup, the `ds:0x9D36` two-hotspot list (cycle +
//! start), `Image(2)`/`Present(0xB)` and a `{0x2CA9, 0xE93C, 0xA10F}`
//! frame loop that only exits through the hotspot actions —
//! [`scenario_next`] (`0x8926`, `ret`s back into the loop) and
//! [`scenario_start`] (`0x8984`, `jmp`s to a galaxy preset whose
//! `0x6801` `pop ax; ret` tail returns to the `0x85B1` caller).
//!
//! Since input only reaches the port between frames the body suspends
//! in [`Vm::title`][super::Vm] after [`enter`] and [`title_step`]
//! advances it one unit per frame — one `0x8891` starframe, one
//! `menu_text` page, one `0x8863` seq poll or one picker frame at a
//! time. `scenario_start` flips [`Vm::title_done`][super::Vm] where
//! the asm `ret`s out of `0x85B1`; `title_step` then runs the post
//! tail — the `0x1D60` block shared with the ending restart
//! (`[0x9C9A]=1`, the base asset pair, `jmp 0x305B`).
//!
//! The `0x884B` `{0x2CA9, [0x9209]?}` spin in the middle of the picker
//! loop is unreachable in the asm (only its own `jmp` targets it) and
//! isn't modelled.

mod credits;
mod pick;
mod show;

pub use pick::{scenario_next, scenario_start};

use super::consts::{BUTTONS, KEY_PEND};
use super::vend::fx;
use super::vend::tail;
use super::vhost::Call;
use super::vops::Ctx;
use credits::CREDITS;

/// A running title attract — the phase, the generic `cx`/`dx`
/// countdown (seq wait, starfield counts), the `cs:0xFB15` scroll flag
/// `menu_text` reports and the credits page cursor.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Title {
    ph: Ph,
    n: u16,
    fb: bool,
    cur: u16,
}

/// The attract phase — one arm per `title_step`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    /// A slide's `0x8863` seq-run — `k` indexes the slide table.
    Seq(u8),
    /// The post-slides `dx = 0x7D` vsync countdown (`0x86F5`).
    Wait,
    /// The `cx = 0x63` `0x8891` starfield warm-up (`0x872B`).
    Warm,
    /// One `menu_text` credits page + the `[0x1290]`/`0x8983` scroll.
    Page,
    /// The post-page `0x8891` countdown (`0x190`/`0x96` frames).
    Hold,
    /// The `Slot(0x1292,0xC)` page-advance + loop back to `Page`.
    Turn,
    /// The `0x8840` picker loop — `{0x2CA9, 0xE93C, 0xA10F}` per frame.
    Pick,
}

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `0x85B1`–`0x8629` — the attract entry: scenario chain + index init,
/// the `0xA3BC` buttons-only hook, input-latch clears and the
/// channel-A flush (`cs:[0x8B7C] = 0xFFFF`), the title-music idiom
/// (`al = 0` `0x8A80` + `[cs:0x1280]` reset + `[cs:0x1278]` at
/// `ds:0x3D7` + `[cs:0x127C]` `cl = 0`), shade/star clears,
/// `Image(0)`/`Present(0xD)` and the `0x9F6` seq arm. The
/// `tpl + 0x600` live-list/chain copies the asm builds in its init
/// stubs run here too. Parks the frame-paced body in `Vm::title`.
pub fn enter(c: &mut Ctx) {
    pick::live_lists(c);
    c.host.svc(Call::Mouse(0x14, 0x1E, 0));
    c.vm.w16(c.st, a!(BUTTONS), 0);
    c.vm.w8(c.st, a!(KEY_PEND), 0);
    c.host.key_flush();
    music(c);
    c.vm.w8(c.st, fx::SHADE, 0);
    c.vm.w8(c.st, fx::STAR_AUX, 0);
    fx::refresh(c);
    c.host.svc(Call::Image(0));
    c.host.svc_st(Call::Present(0xD), c.vm, c.st);
    fx::seq_arm(c, 0x9F6);
    c.vm.title_done = false;
    c.vm.title = Some(Title {
        ph: Ph::Seq(0),
        n: 0,
        fb: false,
        cur: CREDITS,
    });
}

/// `0x85D2`–`0x85F3` — `al = 0` on the `0x8A80` channel, the driver
/// reset, `load_song` at `ds:0x3D7` and `select_sound cl = 0`.
fn music(c: &mut Ctx) {
    c.host.svc(Call::Chan(0x8A80, 0));
    c.host.svc(Call::SndReset);
    c.host.svc(Call::Song(0x3D7));
    c.host.svc(Call::Sound(0));
}

/// One frame of the running attract — `Vm::title` is `None` when done.
/// On completion (`scenario_start`'s hand-off) the `0x1D60` post tail
/// runs inline — the `0x85B1` `ret` site both the entry stub and the
/// `0x1D35` restart tail share.
pub fn title_step(c: &mut Ctx) {
    let Some(t) = c.vm.title.take() else {
        return;
    };
    c.vm.title = t.step(c);
    if c.vm.title_done {
        c.vm.title_done = false;
        c.vm.title = None;
        tail::restart_tail(c);
    }
}

impl Title {
    /// One frame of the body — `Some` while still running.
    fn step(self, c: &mut Ctx) -> Option<Self> {
        match self.ph {
            Ph::Seq(k) => Some(self.seqpoll(c, k)),
            Ph::Wait => Some(self.wait(c)),
            Ph::Warm => Some(self.warm(c)),
            Ph::Page => Some(self.page(c)),
            Ph::Hold => Some(self.hold(c)),
            Ph::Turn => Some(self.turn(c)),
            Ph::Pick => self.pick(c),
        }
    }

    /// `ph = p`.
    fn at(mut self, ph: Ph) -> Self {
        self.ph = ph;
        self
    }

    /// The `[0x9CCB]`/`[0x9CDA]` exit test every block runs — any
    /// input jumps to `0x87D7`, the picker install.
    fn input(c: &mut Ctx) -> bool {
        c.vm.r16(c.st, a!(BUTTONS)) & 1 != 0 || fx::pending(c)
    }

    /// Jump to the picker (`jmp 0x87d7`) — runs the `0x87D7` install
    /// inline and arms `Ph::Pick`.
    fn enter_pick(self, c: &mut Ctx) -> Self {
        pick::install(c);
        self.at(Ph::Pick)
    }

    /// One `0x8891` starfield frame of a `cx` countdown; arms `done`
    /// when `n` runs out (`dec/jns` — `n + 1` frames total).
    fn countdown(mut self, c: &mut Ctx, done: Ph) -> Self {
        if Self::input(c) {
            return self.enter_pick(c);
        }
        fx::stars(c);
        if self.n == 0 {
            return self.at(done);
        }
        self.n -= 1;
        self
    }
}
