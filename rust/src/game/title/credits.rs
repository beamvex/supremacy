//! `0x8753`–`0x87D4` — the credits loop: a `menu_text` block arm and
//! the `[0x1290]`/`0x8983`/`[0x1292]` calls — all `ret`s in the shipped
//! binary, so the port runs the continuous scroll they stand in for
//! (`Call::Scroll` + the resume poll per frame, GAM-46) until input
//! (`jmp 0x87d7`).

use super::super::vend::fx;
use super::super::vhost::Call;
use super::super::vops::Ctx;
use super::{Ph, Title};

/// `Call::Text` arg for "keep `cs:0xF672`" — the resume calls.
const TEXT_RESUME: u16 = u16::MAX;
/// The credits stream's first page cursor (`cs:0xF672 = 0x5012`, file
/// `0x38747`).
pub(super) const CREDITS: u16 = 0x5012;

impl Title {
    /// `0x8753`–`0x879B` — the `[0x1296]` arm (`cs:0xF672 = 0x5012` is
    /// set by `enter`), then the `[0x1290]`/`0x8983`/`[0x1292]` calls
    /// in order and into `Ph::Crawl`.
    pub(super) fn page(mut self, c: &mut Ctx) -> Self {
        _ = fx::text_page(c, self.cur);
        self.cur = TEXT_RESUME;
        if Self::input(c) {
            return self.enter_pick(c);
        }
        c.host.svc(Call::Slot(0x1290, 9));
        c.host.svc(Call::Native(0x8983));
        c.host.svc(Call::Slot(0x1292, 0xC));
        self.at(Ph::Crawl)
    }

    /// The per-frame crawl — one `Call::Scroll` step (the snow is
    /// already swept and the marks suppressed), the `0x8891` starframe
    /// for its `0x2CA9` vsync, then the `menu_text` resume poll that
    /// pulls the next block and latches `0x9CCB` at the stream end.
    pub(super) fn crawl(self, c: &mut Ctx) -> Self {
        if Self::input(c) {
            return self.enter_pick(c);
        }
        fx::scroll(c);
        fx::stars(c);
        _ = fx::text_page(c, self.cur);
        self
    }
}
