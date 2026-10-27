//! The orders sidebar repaint — `cs:0x3438` (file `0x33438`–
//! `0x336BC`). Each call stashes `[0x9144]`/`[0x9146]` into
//! `[0x9CBC]`/`[0x9CBE]` and services the first armed field in the
//! priority chain — one print per frame while the bits drain.

use super::super::consts::{
    DIRTY0, DIRTY1, FRAME, SEL_REC, SNAP0, SNAP1, TICK, TREND_NEW, TREND_OLD, TREND_TK,
};
use super::super::rec::{
    F_COVER, F_CREDITS, F_DECLINE, F_DEFENCE, F_ENERGY, F_FOOD, F_FUEL, F_GROWTH, F_KIND,
    F_MINERALS, F_NAME, F_POP, F_TAX,
};
use super::super::vops::Ctx;
use super::super::{num, text};
use super::{ordact, ordlist};

macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}
macro_rules! f {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x3438` — stash, then the priority chain; each armed bit
/// clears in the live word and prints (file `0x33438`–`0x336BC`).
pub fn sidebar(c: &mut Ctx) {
    let d0 = c.vm.r16(c.st, a!(DIRTY0));
    let d1 = c.vm.r16(c.st, a!(DIRTY1));
    c.vm.w16(c.st, a!(SNAP0), d0);
    c.vm.w16(c.st, a!(SNAP1), d1);
    if d0 & 8 != 0 {
        c.vm.w16(c.st, a!(DIRTY0), d0 & !8);
        ordlist::list_frame(c);
        return ordlist::planet_list_paint(c);
    }
    if c.vm.r16(c.st, a!(SEL_REC)) == 0 {
        return idle(c);
    }
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    if d0 & 0x200 != 0 {
        return name(c, d0, rec);
    }
    if d0 & 2 != 0 {
        return status(c, d0, rec);
    }
    if d0 & 4 != 0 {
        return credits(c, d0, rec);
    }
    if d0 & 0x4000 != 0 {
        return food(c, d0, rec);
    }
    if d0 & 0x8000 != 0 {
        return stock0(c, d0, 0x8000, f!(F_MINERALS), 0x36);
    }
    if d1 & 1 != 0 {
        return stock1(c, d1, 1, f!(F_FUEL), 0x3E);
    }
    if d1 & 2 != 0 {
        return stock1(c, d1, 2, f!(F_ENERGY), 0x46);
    }
    if d0 & 0x20 != 0 {
        return pop(c, d0, rec);
    }
    if d0 & 0x80 != 0 {
        return cover(c, d0, rec);
    }
    if d0 & 0x40 != 0 {
        return growth(c, d0, rec);
    }
    if d0 & 0x100 != 0 {
        return tax(c, d0, rec);
    }
    if d0 & 0x1000 != 0 {
        return hint_block(c, d0);
    }
    if d0 & 0x10 != 0 {
        return defence(c, d0, rec);
    }
    if d1 & 0x10 != 0 {
        c.vm.w16(c.st, a!(DIRTY1), d1 & !0x10);
        ordact::mode_inner(c, 0);
    }
}

/// `0x3457` — no selection: the `[0x91D4]`-phased "SELECT A PLANET!"
/// blink at `(0x10,0x61)` (5 frames text, 3 frames blank).
fn idle(c: &mut Ctx) {
    let s = if c.vm.r8(c.st, a!(FRAME)) & 7 < 5 {
        0x69CD
    } else {
        0x68CF
    };
    text::print_str(c, s, 0x10, 0x61);
}

/// `0x347A` — `0x200`: the `+0xE` name at `(0x13,0xA)`.
fn name(c: &mut Ctx, d0: u16, rec: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !0x200);
    text::print_str(c, rec.wrapping_add(f!(F_NAME)), 0x13, 0xA);
}

/// `0x3498` — `0x2`: the `0x303A` status string at `(0x13,0x1A)`.
fn status(c: &mut Ctx, d0: u16, rec: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !2);
    let s = status_str(c, rec);
    text::print_str(c, s, 0x13, 0x1A);
}

/// `cs:0x303A` — record `+0xC` kind → status string (file
/// `0x3303A`–`0x3305A`).
fn status_str(c: &mut Ctx, rec: u16) -> u16 {
    match c.vm.r16(c.st, rec.wrapping_add(f!(F_KIND))) {
        4 => 0x71AE,
        1 => 0x71B9,
        7 => 0x71A3,
        _ => 0x7198,
    }
}

/// `0x34B5` — `0x4`: the `+0x36/0x38` credits dword at `(0x14,0x22)`
/// via `0x68E0`.
fn credits(c: &mut Ctx, d0: u16, rec: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !4);
    let lo = c.vm.r16(c.st, rec.wrapping_add(f!(F_CREDITS)));
    let hi = c.vm.r16(c.st, rec.wrapping_add(f!(F_CREDITS) + 2));
    num::num32(c, u32::from(hi) << 16 | u32::from(lo), 0x14, 0x22);
}

/// `0x34D8` — `0x4000`: food `0x686C` at `(0x15,0x2E)` + `0x7C09`,
/// then the tick-gated `[0x91F9]`/`[0x91FB]` trend arrow.
fn food(c: &mut Ctx, d0: u16, rec: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !0x4000);
    let v = c.vm.r16(c.st, rec.wrapping_add(f!(F_FOOD)));
    c.vm.w16(c.st, a!(TREND_NEW), v);
    let bx = num::num_pad(c, v, 0x15, 0x2E);
    let bx = text::print_str(c, 0x7C09, bx, 0x2E);
    trend(c, bx);
}

/// `0x34FF` — once per `[0x91BA]` move: `bx − 0xA`, `+`/`−`/blank by
/// the `[0x91F9]`→`[0x91FB]` compare, then `[0x91F9] = [0x91FB]`.
fn trend(c: &mut Ctx, bx: u16) {
    if c.vm.r16(c.st, a!(TREND_TK)) == c.vm.r16(c.st, a!(TICK)) {
        return;
    }
    c.vm.w16(c.st, a!(TREND_TK), c.vm.r16(c.st, a!(TICK)));
    let old = c.vm.r16(c.st, a!(TREND_OLD));
    let cur = c.vm.r16(c.st, a!(TREND_NEW));
    let s = match old.cmp(&cur) {
        std::cmp::Ordering::Equal => 0x68B9,
        std::cmp::Ordering::Less => 0x73C8,
        std::cmp::Ordering::Greater => 0x73C6,
    };
    c.vm.w16(c.st, a!(TREND_OLD), cur);
    text::print_str(c, s, bx.wrapping_sub(0xA), 0x2E);
}

/// `0x3533` — a `DIRTY0` stock word: `0x686C` at `(0x15,dx)` +
/// `0x7C09`.
fn stock0(c: &mut Ctx, d0: u16, bit: u16, f: u16, dx: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !bit);
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    let v = c.vm.r16(c.st, rec.wrapping_add(f));
    let bx = num::num_pad(c, v, 0x15, dx);
    text::print_str(c, 0x7C09, bx, dx);
}

/// `0x3557`/`0x357A` — the `DIRTY1` twins of `stock0`.
fn stock1(c: &mut Ctx, d1: u16, bit: u16, f: u16, dx: u16) {
    c.vm.w16(c.st, a!(DIRTY1), d1 & !bit);
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    let v = c.vm.r16(c.st, rec.wrapping_add(f));
    let bx = num::num_pad(c, v, 0x15, dx);
    text::print_str(c, 0x7C09, bx, dx);
}

/// `0x359D` — `0x20`: population `0x686C` at `(0x34,0xA)`.
fn pop(c: &mut Ctx, d0: u16, rec: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !0x20);
    num::num_pad(c, c.vm.r16(c.st, rec.wrapping_add(f!(F_POP))), 0x34, 0xA);
}

/// `0x35BA` — `0x80`: coverage `0x6886` at `(0x34,0x1A)` + `0x732D`.
fn cover(c: &mut Ctx, d0: u16, rec: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !0x80);
    let v = u16::from(c.vm.r8(c.st, rec.wrapping_add(f!(F_COVER))));
    let bx = num::num3(c, v, 0x34, 0x1A);
    text::print_str(c, 0x732D, bx, 0x1A);
}

/// `0x35E1` — `0x40`: the `+0x1D − +0x21` growth delta (0 at the
/// `0x7530` cap) via `0x6893` at `(0x35,0x12)` + `0x732D`, then the
/// `+`/`−`/blank direction str at `bx − 5`.
fn growth(c: &mut Ctx, d0: u16, rec: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !0x40);
    let (mut v, mut s) = (0u16, 0x68B9u16);
    if c.vm.r16(c.st, rec.wrapping_add(f!(F_POP))) < 0x7530 {
        let grow = c.vm.r8(c.st, rec.wrapping_add(f!(F_GROWTH)));
        let dec = c.vm.r8(c.st, rec.wrapping_add(f!(F_DECLINE)));
        (v, s) = match grow.cmp(&dec) {
            std::cmp::Ordering::Greater => (u16::from(grow - dec), 0x73C8),
            std::cmp::Ordering::Less => (u16::from(dec - grow), 0x73C6),
            std::cmp::Ordering::Equal => (0, 0x68B9),
        };
    }
    let bx = num::num2(c, v, 0x35, 0x12);
    let bx = text::print_str(c, 0x732D, bx, 0x12);
    text::print_str(c, s, bx.wrapping_sub(5), 0x12);
}

/// `0x364E` — `0x100`: tax `0x6886` at `(0x34,0x28)` + `0x732D`.
fn tax(c: &mut Ctx, d0: u16, rec: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !0x100);
    let v = u16::from(c.vm.r8(c.st, rec.wrapping_add(f!(F_TAX))));
    let bx = num::num3(c, v, 0x34, 0x28);
    text::print_str(c, 0x732D, bx, 0x28);
}

/// `0x3675` — `0x1000`: the `0x38C2` hint reprint.
fn hint_block(c: &mut Ctx, d0: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !0x1000);
    ordlist::hint(c);
}

/// `0x3686` — `0x10`: defence `0x692F` (`bp = 0` → the padded-5
/// shape) at `(0x34,0x3E)`.
fn defence(c: &mut Ctx, d0: u16, rec: u16) {
    c.vm.w16(c.st, a!(DIRTY0), d0 & !0x10);
    num::num_pad(
        c,
        c.vm.r16(c.st, rec.wrapping_add(f!(F_DEFENCE))),
        0x34,
        0x3E,
    );
}
