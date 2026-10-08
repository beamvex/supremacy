//! Selected-record services — the `cs:0x8386`/`0x83AA` flag pair,
//! `cs:0x5E03` record select, and the `cs:0x5D52` list-row click
//! (file `0x38386`–`0x383CD`, `0x35E03`–`0x35E39`, `0x35D52`–`0x35DA0`).
//!
//! `[0x9164]` holds the index of the record the UI is pointed at;
//! `+5` on that record is a "selected/dirty" flag the loop re-arms
//! every frame so the panel knows to redraw it. Clicking the
//! `[0xA9..0xB5)` strip maps the cursor y to a row
//! (`(y − 0x11) >> 3/2/1` by difficulty), bounds-checks against
//! `[0x91C6]`, ignores a same-row repeat, then clears the old flag
//! (`0x8386`) and runs `0x5E03`.

use super::cells::LIST_MAX;
use super::consts::{
    BUTTONS, CUR_X, CUR_Y, DIFFICULTY, REC_BASE, REC_CUR, REC_STRIDE, SEL_IDX, SEL_REC, SEQ_DELAY,
};
use super::status;
use super::vhost::Call;
use super::vops::Ctx;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Left edge of the row-click strip (file `0x35D60`).
const ROW_X0: u16 = 0xA9;
/// Right edge (exclusive) of the strip (file `0x35D66`).
const ROW_X1: u16 = 0xB5;
/// First row's y origin (file `0x35D6B`).
const ROW_Y0: u16 = 0x11;

/// `cs:0x8386` — clear `+5` on record `[0x9164]` (file `0x38386`).
pub fn unmark_sel(c: &mut Ctx) {
    flag(c, 0);
}

/// `cs:0x83AA` — set `+5` on record `[0x9164]` (file `0x383AA`).
pub fn mark_sel(c: &mut Ctx) {
    flag(c, 1);
}

/// Shared body: `bx = [0x9164] * 0x3A + [0x9158]`; `[bx+5] = v`.
fn flag(c: &mut Ctx, v: u8) {
    let i = c.vm.r16(c.st, a!(SEL_IDX));
    let base = c.vm.r16(c.st, a!(REC_BASE));
    let rec = base.wrapping_add(i.wrapping_mul(a!(REC_STRIDE)));
    c.vm.w8(c.st, rec + 5, v);
}

/// `cs:0x5E03` — make `bx` the selected record: index `[0x9164]`,
/// pointer into `[0x9184]`/`[0x917C]`, flag `+5`, panel redraw via
/// `0x5C97` + `0x6AD2`, sequencer delay reset (file `0x35E03`).
pub fn select_rec(c: &mut Ctx, i: u16) {
    c.vm.w16(c.st, a!(SEL_IDX), i);
    let rec =
        c.vm.r16(c.st, a!(REC_BASE))
            .wrapping_add(i.wrapping_mul(a!(REC_STRIDE)));
    c.vm.w16(c.st, a!(REC_CUR), rec);
    c.vm.w16(c.st, a!(SEL_REC), rec);
    mark_sel(c);
    c.host.svc(Call::PlanetPanel);
    status::sel_name(c);
    c.vm.w16(c.st, a!(SEQ_DELAY), 0);
}

/// `cs:0x5D52` — on left-button inside the `[0xA9..0xB5)` strip,
/// select the clicked row (file `0x35D52`–`0x35DA0`).
pub fn row_click(c: &mut Ctx) {
    if c.vm.r16(c.st, a!(BUTTONS)) & 1 == 0 {
        return;
    }
    let Some(row) = clicked_row(c) else { return };
    if row == c.vm.r16(c.st, a!(SEL_IDX)) {
        return;
    }
    unmark_sel(c);
    select_rec(c, row);
}

/// Row computation + bounds (file `0x35D5A`–`0x35D97`): `None` when
/// the cursor is outside the strip or the row is past `[0x91C6]`.
fn clicked_row(c: &mut Ctx) -> Option<u16> {
    let x = c.vm.r16(c.st, a!(CUR_X)) & 0xFF;
    if !(ROW_X0..ROW_X1).contains(&x) {
        return None;
    }
    let y = c.vm.r16(c.st, a!(CUR_Y));
    if y & 0xFF < ROW_Y0 {
        return None;
    }
    let shift = 3u8.saturating_sub(c.vm.r8(c.st, a!(DIFFICULTY)).min(2));
    let row = y.wrapping_sub(ROW_Y0) >> shift;
    // 8-bit compare: `inc bl` wraps and only `al` is tested.
    let lim = (c.vm.r16(c.st, a!(LIST_MAX)) & 0xFF).wrapping_add(1) & 0xFF;
    (row & 0xFF < lim).then_some(row)
}
