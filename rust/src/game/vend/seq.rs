//! The `cs:0x8B17`/`cs:0x8C65` ending bodies (file `0x38B17`–`0x38DC3`)
//! as a per-frame state machine.
//!
//! The asm blocks inside the sequence: starfield waits are
//! retrace-counted `0x8891` loops, `menu_text` pages and the
//! `[0x9CCB]`/`[0x9CDA]` input latches are polled between frames, and
//! the shared tail `jmp 0x1D35`s into the restart. Since input only
//! reaches the port between frames, the body suspends in `Vm::end`
//! after [`head`] and [`end_step`] advances it one unit per frame —
//! one `0x8891` starframe, one `menu_text` page, one `0x8863` seq poll
//! or one input poll at a time.

mod outro;
mod scroll;

use super::super::vhost::Call;
use super::super::vops::{vsync, Ctx};
use super::fx;
use super::Ending;

/// `[0x91E5]` — the galaxy-scenario index the endings pick pages on
/// (`0` WOTOK … `3` RORN; the same slot `cells::LOAD_TO` reuses).
pub(super) const SCENARIO: u16 = 0x91E5;
/// `Call::Text` arg for "keep `cs:0xF672`" — the resume calls.
const TEXT_RESUME: u16 = u16::MAX;

/// A running ending sequence — which ending, the phase, the `cx`
/// starfield countdown and the `menu_text` state (`cur` = the
/// `cs:0xF672` cursor to pass the next [`Call::Text`], `fb` = the last
/// page's `cs:0xFB15` scroll-variant byte).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct End {
    e: Ending,
    ph: Ph,
    n: u16,
    fb: bool,
    cur: u16,
}

/// The sequence phase — one arm per `end_step`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ph {
    /// The shared `cx = 0x63` starfield warm-up (`0x8B5C`/`0x8CAA`).
    Head,
    /// One `menu_text` block arm + the `[0x1290]`/`0x8983` scroll
    /// calls — A then runs a static card `Hold`; B the `Crawl`.
    Page,
    /// A's post-page `0x8891` countdown (`0x190`/`0x96` frames).
    Hold,
    /// B's continuous text crawl — `Call::Scroll` + the `menu_text`
    /// resume poll per frame until `[0x9CCB]` (GAM-46; the
    /// `[0x1292]` page-advance's scroll stand-in).
    Crawl,
    /// The `0x8863` `{0x2CA9, 0x5C1E}` `seq_run` wait.
    Seq,
    /// The `{0x8891, [0x9CDA]==1?, 0x1E2E}` input wait (`0x8C3F`).
    Input,
    /// The `0x85B1` title attract's exit poll inside `0x1D35`.
    Title,
}

impl End {
    /// Fresh machine parked on `Ph::Head`; B's first `menu_text`
    /// cursor is `0x5953` (`0x5E52` on the RORN scenario — file
    /// `0x38CB5`–`0x38CC2`).
    pub(super) fn new(e: Ending, scenario: u16) -> Self {
        let cur = match (e, scenario) {
            (Ending::B, 3) => 0x5E52,
            (Ending::B, _) => 0x5953,
            _ => TEXT_RESUME,
        };
        Self {
            e,
            ph: Ph::Head,
            n: 0x63,
            fb: false,
            cur,
        }
    }

    /// One frame of the body — `Some` while still running.
    fn step(self, c: &mut Ctx) -> Option<Self> {
        match self.ph {
            Ph::Head => Some(self.head(c)),
            Ph::Page => Some(self.page(c)),
            Ph::Hold => Some(self.hold(c)),
            Ph::Crawl => Some(self.crawl(c)),
            Ph::Seq => Some(self.seqpoll(c)),
            Ph::Input => Some(self.input(c)),
            Ph::Title => self.title(c),
        }
    }

    /// `ph = p`.
    fn at(mut self, ph: Ph) -> Self {
        self.ph = ph;
        self
    }
}

/// One frame of the running ending — `Vm::end` is `None` when done.
pub fn end_step(c: &mut Ctx) {
    let Some(e) = c.vm.end.take() else { return };
    c.vm.end = e.step(c);
}

/// The shared sequence head — `0xA3BC` hook, `Chan(0x8A80, al)`, the
/// music-swap idiom (`[cs:0x1280]` reset, `load_song` at `ds:seg`,
/// `[cs:0x127C]` cl=0), `Slot(0x128E,8)` fade, refresh, `0x2CA9`,
/// `Present(9)`, `Slot(0x1292,9)` (file `0x38B17`–`0x38B58`).
pub(super) fn head(c: &mut Ctx, e: Ending) {
    let (al, seg) = match e {
        Ending::A => (2, 0x95A),
        Ending::B => (1, 0x5B7),
    };
    c.host.svc(Call::Mouse(0x14, 0x1E, 0));
    c.host.svc(Call::Chan(0x8A80, al));
    c.host.svc(Call::SndReset);
    c.host.svc(Call::Song(seg));
    c.host.svc(Call::Sound(0));
    c.host.svc_st(Call::Slot(0x128E, 8), c.vm, c.st);
    fx::refresh(c);
    vsync(c);
    c.host.svc_st(Call::Present(9), c.vm, c.st);
    c.host.svc(Call::Slot(0x1292, 9));
}
