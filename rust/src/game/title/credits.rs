//! `0x8753`–`0x87D4` — the repeating credits-page loop: a `menu_text`
//! page, the `[0x1290]` scroll, the `0x8891` hold countdown and the
//! `[0x1292]` advance, looping until input (`jmp 0x87d7`).

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
    /// `0x8753`–`0x879B` — one page: `call word [0x1296]` with
    /// `es = [0x147]` (the `0x5012` cursor is armed by `enter` /
    /// `turn`), then `[0x1290]` and the `cs:0xFB15`-keyed `0x8983`
    /// scroll stub — `dx = 0xC` when the flag is set, plain otherwise;
    /// both are `ret`s in the port, only `Native` records them.
    pub(super) fn page(mut self, c: &mut Ctx) -> Self {
        self.fb = fx::text_page(c, self.cur);
        self.cur = TEXT_RESUME;
        if Self::input(c) {
            return self.enter_pick(c);
        }
        c.host.svc(Call::Slot(0x1290, 9));
        if Self::input(c) {
            return self.enter_pick(c);
        }
        c.host.svc(Call::Native(0x8983));
        if Self::input(c) {
            return self.enter_pick(c);
        }
        self.at_hold()
    }

    /// Arm the `0x879B` hold — `cx = 0x190` while `cs:0xFB15` is set,
    /// else `0x96`.
    fn at_hold(self) -> Self {
        Self {
            n: if self.fb { 0x190 } else { 0x96 },
            ..self.at(Ph::Hold)
        }
    }

    /// `0x87A9` — the hold countdown: input check, `0x8891`, `dec/jns`.
    /// Ends on `Ph::Turn` (`0x87BF`).
    pub(super) fn hold(self, c: &mut Ctx) -> Self {
        self.countdown(c, Ph::Turn)
    }

    /// `0x87BF`–`0x87D4` — `dx = 0xC` on `[0x1292]` and back to
    /// `0x8753` (`Ph::Page`).
    pub(super) fn turn(self, c: &mut Ctx) -> Self {
        c.host.svc(Call::Slot(0x1292, 0xC));
        if Self::input(c) {
            return self.enter_pick(c);
        }
        self.at(Ph::Page)
    }
}
