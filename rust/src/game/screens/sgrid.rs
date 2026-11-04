//! The surface screen's frame-loop helpers — the `0x4AFD` entry
//! auto-select of the first `+0x1E == 1` bay machine, the `0x50A9`
//! surface-grid click that loads the hit cell's ship into the
//! selected machine, and the `0x4974` dirty-flag stat reprint /
//! `0x4190` refresh (files `0x34AFD`–`0x34B13`, `0x350A9`–`0x35196`,
//! `0x34974`–`0x349DD`).

use super::super::consts::{
    ARMED, BUTTONS, CUR_X, CUR_Y, DIRTY0, DIRTY1, MACH_BASE, MACH_STRIDE, SEL_MACH, SEL_REC,
};
use super::super::mach::M_TYPE;
use super::super::num;
use super::super::rec::{F_ATTACK, F_DEFENCE, F_LINK_A, F_TROOPS};
use super::super::vops::Ctx;
use super::sact;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:` base of the `0x9AB1` surface table `0x4190` fills.
const SURF: u16 = 0x9AB1;
/// Grid hit-test x floor — columns are `0x28` wide, `<3` admitted.
const GRID_X: u16 = 9;
/// Grid hit-test y floor — rows are `0xA` tall, `<8` admitted.
const GRID_Y: u16 = 0x71;
/// `ds:0x4FDC` — the fourth cargo slot the `0x5179` full check reads.
const CARGO_LAST: u16 = 0x4FDC;
/// The `0x9CBC`/`0x9CBE` dirty-word snapshots `0x4974` takes.
const SNAP0: u16 = 0x9CBC;
const SNAP1: u16 = 0x9CBE;

/// `cs:0x4AFD` — scan the `[0x917C]` `+0x18`/`+0x1C`/`+0x20` bays for
/// the first `+0x1E == 1` machine and take the `0x4296` select tail
/// (`0x42A7`'s `[0x9180]` arm + `0x57BC`/`0x5994`); no match → ret.
pub fn autoselect(c: &mut Ctx) {
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    for i in 0..3u16 {
        let b = c.vm.r8(c.st, rec.wrapping_add(a!(F_LINK_A) + i * 4));
        if b == 0 {
            continue;
        }
        let m =
            a!(MACH_BASE).wrapping_add(u16::from(b).wrapping_sub(1).wrapping_mul(a!(MACH_STRIDE)));
        if c.vm.r8(c.st, m + a!(M_TYPE)) == 1 {
            c.vm.w16(c.st, a!(SEL_MACH), m);
            sact::select(c, m);
            return;
        }
    }
}

/// `cs:0x50A9` — the grid click on the debounced left press: the hit
/// `0x9AB1` cell's ship links to `[0x9180]` and gains the `+0x6`
/// `0x4` aboard bit (`[0x8204] = 0xFF`, `0x57BC`/`0x4190` rebuild).
/// Empty cell → `0x4257`; no machine → `0x69DE`; slot 3 filled →
/// `0x6A17`. Out-of-bounds or a release clears the `[0x91CA]` latch.
pub fn grid(c: &mut Ctx) {
    if c.vm.r16(c.st, a!(BUTTONS)) & 1 == 0 {
        c.vm.w8(c.st, a!(ARMED), 0);
        return;
    }
    if c.vm.r8(c.st, a!(ARMED)) != 0 {
        return;
    }
    c.vm.w8(c.st, a!(ARMED), 0xFF);
    let Some(idx) = cell(c) else {
        c.vm.w8(c.st, a!(ARMED), 0);
        return;
    };
    let si = SURF.wrapping_add(idx.wrapping_mul(4));
    if c.vm.r16(c.st, si) == 0 {
        return sact::refuse(c, 0x69B1, 0xB);
    }
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if m == 0 {
        return sact::refuse(c, 0x69DE, 0xA);
    }
    if c.vm.r16(c.st, CARGO_LAST) != 0 {
        return sact::refuse(c, 0x6A17, 0xA);
    }
    let u = c.vm.r16(c.st, si);
    c.vm.w16(c.st, u + 2, m);
    let flags = c.vm.r8(c.st, u + 6) | 0x4;
    c.vm.w8(c.st, u + 6, flags);
    c.vm.w8(c.st, 0x8204, 0xFF);
    sact::cargo(c);
    sact::planet(c);
}

/// `cs:0x4974` — the per-frame dirty check: `0x9144 & 0x10` → the
/// `[0x917C]` `+0x22`/`+0x26`/`+0x2E` stat reprints at
/// `(0x49,0x4E)`/`(0x49,0x55)`/`(0x47,0x5F)`; else `0x9146 & 8` →
/// `jmp 0x4190` (the surface rebuild).
pub fn dirty(c: &mut Ctx) {
    let (d0, d1) = (c.vm.r16(c.st, a!(DIRTY0)), c.vm.r16(c.st, a!(DIRTY1)));
    c.vm.w16(c.st, SNAP0, d0);
    c.vm.w16(c.st, SNAP1, d1);
    if d0 & 0x10 == 0 {
        if d1 & 8 != 0 {
            c.vm.w16(c.st, a!(DIRTY1), d1 & !8);
            sact::planet(c);
        }
        return;
    }
    c.vm.w16(c.st, a!(DIRTY0), d0 & !0x10);
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    num::num_pad(
        c,
        c.vm.r16(c.st, rec.wrapping_add(a!(F_DEFENCE))),
        0x49,
        0x4E,
    );
    num::num_pad(
        c,
        c.vm.r16(c.st, rec.wrapping_add(a!(F_TROOPS))),
        0x49,
        0x55,
    );
    num::num4(
        c,
        c.vm.r16(c.st, rec.wrapping_add(a!(F_ATTACK))),
        0x47,
        0x5F,
    );
}

/// The `(x−9)/0x28` column `<3` + `(y−0x71)/0xA` row `<8` → the
/// column-major `(col·8 + row) & 0x1F` slot (file `0x350D0`–`0x3514C`).
fn cell(c: &mut Ctx) -> Option<u16> {
    let (x, y) = (c.vm.r16(c.st, a!(CUR_X)), c.vm.r16(c.st, a!(CUR_Y)));
    if y < GRID_Y || x < GRID_X {
        return None;
    }
    let (col, row) = (x.wrapping_sub(GRID_X) / 0x28, y.wrapping_sub(GRID_Y) / 0xA);
    if col >= 3 || row >= 8 {
        return None;
    }
    Some(col.wrapping_mul(8).wrapping_add(row) & 0x1F)
}
