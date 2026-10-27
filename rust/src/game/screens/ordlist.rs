//! The orders-screen list region — `cs:0x38CE` frame paint,
//! `cs:0x6F53` planet-list repaint, the `cs:0x701D` click→row
//! hit-test the frame loop polls, and the `cs:0x38C2` status-line
//! hint (files `0x338CE`–`0x33905`, `0x36F53`–`0x36FB7`,
//! `0x3701D`–`0x37118`, `0x338C2`–`0x338CD`).

use super::super::consts::{
    BUTTONS, CUR_X, CUR_Y, LAUNCH_F, REC_BASE, REC_COUNT, REC_STRIDE, REC_TOTAL, SEL_REC, UIMODE,
};
use super::super::rec::{F_KIND, F_NAME, F_OWNER};
use super::super::vhost::Call;
use super::super::vops::{set_bits, Ctx};
use super::super::{dialog, oreg, text};
use super::galmsg;
use super::ordact;

macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// List top edge (file `0x37035`).
const ROW_Y: u16 = 0x83;
/// List left edge (file `0x3703E`).
const ROW_X: u16 = 0x1C;
/// Column pitch — 0x28px per column, 4 columns (file `0x3704E`).
const COL_W: u16 = 0x28;
/// Rows per column — 8px pitch, 8 rows (file `0x37074`).
const COL_N: u16 = 8;

/// `cs:0x38C2` — the `0x75BB` `{27 × ' '}` repeat reprint at
/// `(0xA, 0x61)` — repaints the prompt row between messages.
pub fn hint(c: &mut Ctx) {
    text::print_str(c, 0x75BB, 0xA, 0x61);
}

/// `cs:0x38CE` — the list frame: region push `(0x1C,0xC2,0x7C,0xC4)`,
/// then 8 rows × 4 columns of `[0x126C]` brackets + the `0x68B1`
/// blank-name print, `0xA479` release (file `0x338CE`–`0x33905`).
pub fn list_frame(c: &mut Ctx) {
    oreg::push(c, 0x1C, 0xC2, 0x7C, 0xC4);
    let mut dx = ROW_Y;
    for _ in 0..8u16 {
        let mut bx = 8;
        for _ in 0..4u16 {
            c.host.svc(Call::Slot(0x126C, dx));
            text::print_str(c, 0x68B1, bx, dx);
            bx = bx.wrapping_add(1);
        }
        dx = dx.wrapping_add(8);
    }
    oreg::pop(c);
}

/// `cs:0x6F53` — the planet-list repaint: same region, then per
/// record a kind icon (`[0x126C]` transit/candidate, `[0x126E]`
/// colony, `[0x126A]` other) plus the `+0xE` name for non-`1`/`4`
/// kinds, 8 rows per column (file `0x36F53`–`0x36FB7`).
pub fn planet_list_paint(c: &mut Ctx) {
    oreg::push(c, 0x1C, 0xC2, 0x7C, 0xC4);
    let (mut bx, mut dx) = (8u16, ROW_Y);
    let n = c.vm.r16(c.st, a!(REC_COUNT));
    let mut si = c.vm.r16(c.st, a!(REC_BASE));
    for _ in 0..=n {
        let kind = c.vm.r16(c.st, si.wrapping_add(a!(F_KIND)));
        match kind {
            1 | 4 => drop(c.host.svc(Call::Slot(0x126C, dx))),
            0xA => drop(c.host.svc(Call::Slot(0x126E, dx))),
            _ => drop(c.host.svc(Call::Slot(0x126A, dx))),
        }
        if kind != 1 && kind != 4 {
            bx = text::print_str(c, si.wrapping_add(a!(F_NAME)), bx, dx).wrapping_sub(9);
        }
        dx = dx.wrapping_add(8);
        if dx & 0xFF == 0xC3 {
            dx = ROW_Y;
            bx = bx.wrapping_add(0xA);
        }
        si = si.wrapping_add(a!(REC_STRIDE));
    }
    oreg::pop(c);
}

/// `cs:0x701D` — the in-loop list hit-test: on left-down inside the
/// region map the click to a record (4 cols × 8 rows, column-major),
/// ignore out-of-range and ownerless hits, then per-uimode select.
pub fn list_click(c: &mut Ctx) {
    if c.vm.r16(c.st, a!(BUTTONS)) & 1 == 0 {
        return;
    }
    let Some(si) = clicked(c) else { return };
    match c.vm.r8(c.st, a!(UIMODE)) {
        4 => arm(c, si),
        3 => pick(c, si),
        _ => c.vm.w16(c.st, a!(SEL_REC), si),
    }
}

/// `0x37028`–`0x370A2` — cursor → record pointer; `None` outside the
/// region, past `[0x91B6]`, or on an ownerless (`+0x24 == 0`) record.
fn clicked(c: &mut Ctx) -> Option<u16> {
    let x = c.vm.r16(c.st, a!(CUR_X));
    let y = c.vm.r16(c.st, a!(CUR_Y));
    if y < ROW_Y || x < ROW_X {
        return None;
    }
    let col = (x.wrapping_sub(ROW_X) / COL_W) & 0xFF;
    let row = (y.wrapping_sub(ROW_Y) / 8) & 0xFF;
    if col >= 4 || row >= COL_N {
        return None;
    }
    let idx = row.wrapping_add(col.wrapping_mul(COL_N)) & 0xFF;
    if idx >= c.vm.r16(c.st, a!(REC_TOTAL)) {
        return None;
    }
    let si = idx
        .wrapping_mul(a!(REC_STRIDE))
        .wrapping_add(c.vm.r16(c.st, a!(REC_BASE)));
    (c.vm.r8(c.st, si.wrapping_add(a!(F_OWNER))) != 0).then_some(si)
}

/// The uimode-3 arm — the `0x38C2`/`0x7361`/`0x6F2D` refuse on
/// non-colony kinds (`[0x91E3]` pushes through), then the erase +
/// `0x71C4` mode repaint, the changed-selection chirp, and the
/// `0xD3F6`/`0x17` dirty arm into `[0x917C] = si`.
fn pick(c: &mut Ctx, si: u16) {
    if c.vm.r16(c.st, si.wrapping_add(a!(F_KIND))) != 0xA {
        hint(c);
        text::print_str(c, 0x7361, 0x13, 0x61);
        galmsg::wait(c);
        hint(c);
        if c.vm.r8(c.st, a!(LAUNCH_F)) == 0 {
            return;
        }
    }
    text::print_str(c, 0x68B9, 0x13, 0x2E);
    ordact::mode_redraw(c);
    if si != c.vm.r16(c.st, a!(SEL_REC)) {
        dialog::snd_block(c);
    }
    arm(c, si);
}

/// The `0x37109` tail — dirty arm + `[0x917C] = si`.
fn arm(c: &mut Ctx, si: u16) {
    set_bits(c, a!(super::super::consts::DIRTY1), 0x17);
    set_bits(c, a!(super::super::consts::DIRTY0), 0xD3F6);
    c.vm.w16(c.st, a!(SEL_REC), si);
}
