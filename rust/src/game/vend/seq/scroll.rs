//! The scroll half of the ending bodies — the `0x8891` starfield
//! counts and the `menu_text` page/scroll loop (file `0x38B5C`–`0x38BBE`
//! / `0x38CAA`–`0x38D16`).

use super::super::super::vhost::Call;
use super::super::super::vops::Ctx;
use super::super::fx::{self, page_a};
use super::super::Ending;
use super::{End, Ph, TEXT_RESUME};

impl End {
    /// `Ph::Head` — one `0x8891` starframe of the `cx` countdown.
    pub(super) fn head(mut self, c: &mut Ctx) -> Self {
        fx::stars(c);
        if self.n == 0 {
            return self.at(Ph::Page);
        }
        self.n -= 1;
        self
    }

    /// `Ph::Page` — `menu_text` on `cur` (A's scenario pick or B's
    /// running cursor); B exits the credits loop on `[0x9CCB]` →
    /// `Post` (file `0x8CC6`/`0x38C22`).
    pub(super) fn page(mut self, c: &mut Ctx) -> Self {
        let cur = match self.e {
            Ending::A => page_a(c),
            Ending::B => self.cur,
        };
        self.fb = fx::text_page(c, cur);
        self.cur = TEXT_RESUME;
        if self.e == Ending::B && fx::pending(c) {
            return self.post(c);
        }
        self.scroll(c)
    }

    /// `Slot(0x1290,9)` scroll + `0x8983` mark (`dx` = `0xC` vs `9` by
    /// `fb` — the arg rides the unported `Native`), then arm `Hold`
    /// (`0x190`; `0x96` for A's non-scroll page). B's `fb` path
    /// re-tests `[0x9CCB]` (file `0x38CD8`–`0x38CF2`).
    fn scroll(mut self, c: &mut Ctx) -> Self {
        c.host.svc(Call::Slot(0x1290, 9));
        c.host.svc(Call::Native(0x8983));
        if self.e == Ending::B && self.fb && fx::pending(c) {
            return self.post(c);
        }
        self.n = match (self.e, self.fb) {
            (Ending::A, false) => 0x96,
            _ => 0x190,
        };
        self.at(Ph::Hold)
    }

    /// `Ph::Hold` — one `0x8891` frame of the countdown; on expiry A
    /// goes to `Post`, B to `Turn`.
    pub(super) fn hold(mut self, c: &mut Ctx) -> Self {
        fx::stars(c);
        if self.n != 0 {
            self.n -= 1;
            return self;
        }
        match self.e {
            Ending::A => self.post(c),
            Ending::B => self.at(Ph::Turn),
        }
    }

    /// `Ph::Turn` — B's `Slot(0x1292,0xC)` + `[0x9CCB]` test looping
    /// back to `Page` (file `0x38D08`–`0x38D16`).
    pub(super) fn turn(self, c: &mut Ctx) -> Self {
        c.host.svc(Call::Slot(0x1292, 0xC));
        self.at(if fx::pending(c) { Ph::Post } else { Ph::Page })
    }
}
