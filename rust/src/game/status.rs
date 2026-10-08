//! Shell status prints — `cs:0x6A8E` tick/day counter and
//! `cs:0x6AD2` selected-record caption (file `0x36A8E`–`0x36B05`).
//!
//! `0x6A8E` picks a coordinate pair by `uimode` (0 → `(0x35,0xB)`;
//! 3 → `(0x15,0x12)`; 4 → `(0x1D,0x3A)`; other modes print nothing),
//! then emits `[0x91BA]` via the `0x6893` number print, a `0x2F`
//! ('/') separator through `[0x1268]` at `bx+1`, and `[0x91BC]` via
//! `0x68A8` — the `tick/day` readout.
//!
//! `0x6AD2` runs only for `uimode == 0`: at `(0x33,0x4A)` it prints
//! the `0x71B9` fixed string when the selected record's `+0xC` is `1`,
//! else a `0x3E` ('>') marker at `(0x34,0x4A)` plus the record name at
//! `+0xE` — both through the `0x6709` print interpreter.

use super::cells::ALIEN_NAME;
use super::consts::{DAY, REC_CUR, TICK, UIMODE};
use super::num;
use super::text::{self, put};
use super::vhost::Call;
use super::vops::Ctx;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x6A8E` — the `tick/day` counter print (file `0x36A8E`):
/// `[0x91BA]` via `0x6893`, a `0x2F` glyph via `[0x1268]` + `inc bx`,
/// then `[0x91BC]` via `0x68A8`.
pub fn tick_day(c: &mut Ctx) {
    let Some((bx, dx)) = pair(c.vm.r8(c.st, a!(UIMODE))) else {
        return;
    };
    let x = num::num(c, c.vm.r16(c.st, a!(TICK)), bx, dx);
    put(c, 0x2F, x, dx);
    num::num(c, c.vm.r16(c.st, a!(DAY)), x.wrapping_add(1), dx);
}

/// `uimode → (x, y)` for the counter (file `0x36A93`–`0x36AB8`).
fn pair(mode: u8) -> Option<(u16, u16)> {
    match mode {
        0 => Some((0x35, 0xB)),
        3 => Some((0x15, 0x12)),
        4 => Some((0x1D, 0x3A)),
        _ => None,
    }
}

/// `cs:0x6AD2` — selected-record caption, `uimode == 0` only (file
/// `0x36AD2`–`0x36B05`).
pub fn sel_name(c: &mut Ctx) {
    if c.vm.r8(c.st, a!(UIMODE)) != 0 {
        return;
    }
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    if c.vm.r16(c.st, rec + 0xC) == 1 {
        text::print_str(c, a!(ALIEN_NAME), 0x33, 0x4A);
        return;
    }
    c.host.svc(Call::Glyph(0x3E, 0x33, 0x4A));
    text::print_str(c, rec + 0xE, 0x34, 0x4A);
}
