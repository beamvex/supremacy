//! The outro half of the ending bodies — the image reveal + seq run,
//! the final credits page, the input wait and the `0x1D35` restart
//! (file `0x38BC0`–`0x38C62` / `0x38D18`–`0x38DC1`).

use super::super::super::consts::{BUTTONS, KEY_PEND, SEQ_PTR};
use super::super::super::seq::seq_step;
use super::super::super::vhost::Call;
use super::super::super::vops::{vsync, Ctx};
use super::super::fx;
use super::super::tail;
use super::super::Ending;
use super::{End, Ph};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

impl End {
    /// `Ph::Post` — the `[0x9CA1]`/`[0x9205]`/`[0x91D6]`/`[0x9198]`/
    /// `[0x91A0]`/`[0x91D7]` arm + refresh, then the `Image(0)`/
    /// `Present` reveal + `Seq` — B only on scenario `3` (file
    /// `0x38BC0`–`0x38C10` / `0x38D18`–`0x38D6F`).
    pub(super) fn post(self, c: &mut Ctx) -> Self {
        let (stream, dx) = match self.e {
            Ending::A => (0xA0A, 0xE),
            Ending::B => (0xDBE, 0xF),
        };
        fx::seq_arm(c, stream);
        fx::refresh(c);
        if self.e == Ending::A || fx::scenario(c) == 3 {
            fx::reveal(c, dx, stream);
            return self.at(Ph::Seq);
        }
        self.post_seq(c)
    }

    /// `Ph::Seq` — `cs:0x8863`: one `{0x2CA9, 0x5C1E}` pair per frame
    /// until `[0x9CDA] & 1`, `[0x9CCB]` or `[0x9198] == 0`.
    pub(super) fn seqpoll(self, c: &mut Ctx) -> Self {
        vsync(c);
        seq_step(c);
        let done = c.vm.r16(c.st, a!(BUTTONS)) & 1 != 0
            || fx::pending(c)
            || c.vm.r16(c.st, a!(SEQ_PTR)) == 0;
        if done {
            return self.post_seq(c);
        }
        self
    }

    /// After the seq run — refresh, `Present(9)`, the `0x58F2` credits
    /// page, then clear the input latches and the channel-A stack →
    /// `Ph::Input` (file `0x38C13`–`0x38C38` / `0x38D72`–`0x38D97`).
    fn post_seq(self, c: &mut Ctx) -> Self {
        fx::refresh(c);
        c.host.svc(Call::Present(9));
        fx::text_page(c, 0x58F2);
        c.vm.w16(c.st, a!(BUTTONS), 0);
        c.vm.w8(c.st, a!(KEY_PEND), 0);
        c.host.key_flush();
        self.at(Ph::Input)
    }

    /// `Ph::Input` — the `0x8C3F`/`0x8D9E` poll: one `0x8891` frame
    /// until `[0x9CDA] == 1` or `0x1E2E` pops a key; then `0x2F91`
    /// `stage_save` + the `0x1D35` restart head → `Ph::Title`.
    pub(super) fn input(self, c: &mut Ctx) -> Self {
        fx::stars(c);
        let key = c.host.svc(Call::KeyPop).to_le_bytes()[0];
        if c.vm.r16(c.st, a!(BUTTONS)) != 1 && key == 0 {
            return self;
        }
        c.vm.stage_save(c.st);
        fx::refresh(c);
        c.host.svc(Call::Present(9));
        tail::restart_head(c);
        self.at(Ph::Title)
    }

    /// `Ph::Title` — one `[0x9CDA] == 1`/`[0x9CCB]` poll per frame
    /// while the unported `0x85B1` attract's `Native` stands in; on
    /// input, the `0x1D35` tail reloads the base set and `jmp 0x305B`s.
    pub(super) fn title(self, c: &mut Ctx) -> Option<Self> {
        vsync(c);
        if c.vm.r16(c.st, a!(BUTTONS)) != 1 && !fx::pending(c) {
            return Some(self);
        }
        tail::restart_tail(c);
        None
    }
}
