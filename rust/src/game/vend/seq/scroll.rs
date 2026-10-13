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

    /// `Slot(0x1290,9)` scroll + `0x8983` mark — `ret` stubs in the
    /// shipped binary, so B then runs `Ph::Crawl` (the continuous
    /// scroll they stand in for, GAM-46); A keeps its static card and
    /// the `0x8891` hold (`0x190`/`0x96` by `fb`, file
    /// `0x38CD8`–`0x38CF2`).
    fn scroll(mut self, c: &mut Ctx) -> Self {
        c.host.svc(Call::Slot(0x1290, 9));
        c.host.svc(Call::Native(0x8983));
        if self.e == Ending::B {
            if self.fb && fx::pending(c) {
                return self.post(c);
            }
            c.host.svc(Call::Slot(0x1292, 0xC));
            return self.at(Ph::Crawl);
        }
        self.n = if self.fb { 0x190 } else { 0x96 };
        self.at(Ph::Hold)
    }

    /// `Ph::Hold` — one `0x8891` frame of A's countdown; on expiry it
    /// goes to `Post`.
    pub(super) fn hold(mut self, c: &mut Ctx) -> Self {
        fx::stars(c);
        if self.n != 0 {
            self.n -= 1;
            return self;
        }
        self.post(c)
    }

    /// `Ph::Crawl` — B's scroll loop: one `Call::Scroll` step + the
    /// `0x8891` starframe + the `menu_text` resume poll per frame;
    /// `[0x9CCB]` (the stream end or input) exits to `Post` — the
    /// `0x38D08`–`0x38D16` test's crawl form.
    pub(super) fn crawl(self, c: &mut Ctx) -> Self {
        fx::scroll(c);
        fx::stars(c);
        _ = fx::text_page(c, self.cur);
        if fx::pending(c) {
            return self.post(c);
        }
        self
    }
}
