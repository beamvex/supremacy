//! The build grid's mode paints — `0x6FB8`/`0x6FBF` (`[0x91E2]`
//! planet/machine mode) and the `0x6FD2` machine list (file
//! `0x36FB8`–`0x3701C`).

use super::super::consts::{MACH_BASE, MACH_COUNT, MACH_STRIDE, REINF_REC};
use super::super::mach::M_TYPE;
use super::super::vops::Ctx;
use super::super::{oreg, text};
use super::ordlist;

/// `u16` casts for the `ds:`/record constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `[0x91E2]` — the grid-mode flag the `0x6FB8`/`0x6FBF` heads set:
/// `0` lists planets (the `0x701D` click), `0xFF` lists in-use
/// machines (the `0x7119` click).
pub(super) const PICK: u16 = 0x91E2;

/// `0x6FB8` — planet-pick mode: `[0x91E2] = 0` + the `0x6FC6` repaint.
pub(super) fn planet_mode(c: &mut Ctx) {
    c.vm.w8(c.st, PICK, 0);
    mode_paint(c);
}

/// `0x6FBF` — machine-list mode: `[0x91E2] = 0xFF` + the same.
pub(super) fn mach_mode(c: &mut Ctx) {
    c.vm.w8(c.st, PICK, 0xFF);
    mode_paint(c);
}

/// `0x6FC6` — the `0x38CE` frame, then the mode's fill.
fn mode_paint(c: &mut Ctx) {
    ordlist::list_frame(c);
    if c.vm.r8(c.st, PICK) == 0 {
        ordlist::planet_list_paint(c);
    } else {
        mach_paint(c);
    }
}

/// `0x6FD2` — the machine list: the shared region bracket, then each
/// in-use `0x9491` record's `+0` name packed down the columns like
/// `0x6F53`'s planet rows (`si` stashed at `[0x9150]` per print).
pub(super) fn mach_paint(c: &mut Ctx) {
    oreg::push(c, 0x1C, 0xC2, 0x7C, 0xC4);
    let (mut bx, mut dx) = (8u16, 0x83u16);
    for i in 0..a!(MACH_COUNT) {
        let m = a!(MACH_BASE).wrapping_add(i.wrapping_mul(a!(MACH_STRIDE)));
        if c.vm.r8(c.st, m + a!(M_TYPE)) == 0 {
            continue;
        }
        c.vm.w16(c.st, a!(REINF_REC), m);
        let end = text::print_str(c, m, bx, dx);
        (bx, dx) = (end.wrapping_sub(9), dx.wrapping_add(8));
        if dx & 0xFF == 0xC3 {
            (bx, dx) = (bx.wrapping_add(0xA), 0x83);
        }
    }
    oreg::pop(c);
}
