//! `cs:0x56E1` `build_rename` and the `cs:0x7119` machine-grid click
//! — the build screen's unit name edit and machine selection (files
//! `0x356E1`–`0x357BB`, `0x37119`–`0x371C3`).

use super::super::cells::{IN_DONE, LINE_MODE, OVERLAY_F};
use super::super::consts::{BUTTONS, CUR_X, CUR_Y, DIRTY0, NAME_BUF, SEL_MACH, SEL_REC};
use super::super::consts::{MACH_BASE, MACH_COUNT, MACH_STRIDE};
use super::super::mach::{M_FLAGS, M_LINK, M_TYPE};
use super::super::vops::{set_bits, Ctx};
use super::super::{dialog, linein, namedit, text};
use super::{bfield, bpaint, dfield, galmsg, machd};

/// `u16` casts for the `ds:`/record constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x56E1` — `build_rename`: `0x9180` guard (the `0x579D` blip
/// pair on empty), then the `0x7513` prompt, the name staged at
/// `0x838E`, the `0xE652` edit bracketed in `[0x9CC5]`; accept
/// repaints `0x6FD2`+`0x5994`, Esc restores + `0x6B24` + `0x6F2D`,
/// the `0x52FE` tail.
pub(super) fn build_rename(c: &mut Ctx) {
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if m == 0 {
        super::snd(c, 0xA, 0xA);
        return dialog::snd_block(c);
    }
    dialog::snd_block(c);
    bfield::prompt(c);
    text::print_str(c, 0x7513, 0x1B, 0x5F);
    namedit::copy9(c, m, a!(NAME_BUF));
    c.vm.w8(c.st, a!(LINE_MODE), 1);
    linein::unnamed_edit(c, m, 0x2B, 0x5F, 8);
    c.vm.w8(c.st, a!(LINE_MODE), 0);
    rename_done(c, m);
    bfield::prompt(c);
}

/// `0x3574E` — the accept/cancel tail of `build_rename`.
fn rename_done(c: &mut Ctx, m: u16) {
    if c.vm.r8(c.st, a!(IN_DONE)) == 0 {
        bpaint::mach_paint(c);
        dfield::bays(c);
        return;
    }
    namedit::copy9(c, a!(NAME_BUF), m);
    c.vm.w8(c.st, a!(OVERLAY_F), 0xFF);
    text::print_str(c, 0x6B24, 0x2B, 0x5F);
    c.vm.w8(c.st, a!(OVERLAY_F), 0);
    galmsg::wait(c);
}

/// `cs:0x7119` — the machine-grid click: the cell map (cols `< 7`,
/// rows `< 8`, `col·8 + row`) then the `0x9491` walk counting in-use
/// `+0x1E` records to the `al`-th — `[0x9180]`, its `0x5085` host into
/// `[0x917C]`, `d0 |= 0x3000`, a flag-`8` unit re-points `[0x917C]` at
/// `[u+0x1A]`.
pub(super) fn mach_click(c: &mut Ctx) {
    if c.vm.r16(c.st, a!(BUTTONS)) & 1 == 0 {
        return;
    }
    let x = c.vm.r16(c.st, a!(CUR_X));
    let y = c.vm.r16(c.st, a!(CUR_Y));
    if y < 0x83 || x < 0x1C {
        return;
    }
    let col = (x.wrapping_sub(0x1C) / 0x28) & 0xFF;
    let row = (y.wrapping_sub(0x83) / 8) & 0xFF;
    if col >= 7 || row >= 8 {
        return;
    }
    pick_mach(c, (col.wrapping_mul(8).wrapping_add(row)) & 0xFF);
}

/// `0x37183` — select the `n`-th in-use machine record.
fn pick_mach(c: &mut Ctx, mut n: u16) {
    for i in 0..a!(MACH_COUNT) {
        let m = a!(MACH_BASE).wrapping_add(i.wrapping_mul(a!(MACH_STRIDE)));
        if c.vm.r8(c.st, m + a!(M_TYPE)) == 0 {
            continue;
        }
        if n == 0 {
            return sel_mach(c, m);
        }
        n = n.wrapping_sub(1);
    }
}

/// `0x3719C` — `[0x9180] = m`, host into `[0x917C]`, `d0 |= 0x3000`,
/// flag `8` → `[0x917C] = [m+0x1A]`.
fn sel_mach(c: &mut Ctx, m: u16) {
    c.vm.w16(c.st, a!(SEL_MACH), m);
    let h = machd::mach_host(c, m);
    c.vm.w16(c.st, a!(SEL_REC), h);
    set_bits(c, a!(DIRTY0), 0x3000);
    if c.vm.r8(c.st, m + a!(M_FLAGS)) & 8 != 0 {
        let d = c.vm.r16(c.st, m + a!(M_LINK));
        c.vm.w16(c.st, a!(SEL_REC), d);
    }
}
