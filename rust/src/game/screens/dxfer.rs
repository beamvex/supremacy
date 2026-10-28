//! The detail screen's stock movers — `cs:0x262C` free-capacity probe,
//! the `xfer_in`/`xfer_out` STORES columns (`0x265F`/`0x269C`/`0x26D9`/
//! `0x2719` in, `0x2AF7`/`0x2B20`/`0x2B49`/`0x2B72` out) and the
//! `0x4C9B`/`0x4CD8` people transfers. Each clears the `[0x91CA]`
//! debounce so a held button re-fires on the next `0xA10F` pass (files
//! `0x3262C`–`0x32740`, `0x32AF7`–`0x32B9A`, `0x34C9B`–`0x34D0A`).

use super::super::consts::{ARMED, REC_CUR, SEL_MACH};
use super::super::rec::F_POP;
use super::super::vops::Ctx;
use super::{dfield, dmach};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}
/// `u16` casts for the record/machine field constants.
macro_rules! f {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:` flag `0x262C` sets when the stores are at capacity `[0x91E1]`
/// (file `0x32653`/`0x32659`).
const FULL: u16 = 0x91E1;
/// The `0x7530` planet-stock ceiling every `xfer` honours.
pub(super) const CAP: u16 = 0x7530;

/// `cs:0x262C` — stores capacity into `cx`: type `+4` minus the
/// machine's `+0xA/E/10/12` load, clamped to 5; `[0x91E1] = 0xFF` and
/// the caller `ret`s when nothing fits (file `0x3262C`–`0x3265E`).
fn capacity(c: &mut Ctx) -> u16 {
    let t = dmach::type_rec(c);
    let cx = c.vm.r16(c.st, t + 4);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let load = [0xAu16, 0xE, 0x10, 0x12]
        .iter()
        .fold(0u16, |s, &f| s.wrapping_add(c.vm.r16(c.st, m + f)));
    if cx <= load {
        c.vm.w8(c.st, FULL, 0xFF);
        return 0;
    }
    c.vm.w8(c.st, FULL, 0);
    (cx - load).min(5)
}

/// `cs:0x265F`/`0x269C`/`0x26D9`/`0x2719` — `xfer_in` over the
/// `(planet field, machine field, column)` triple: `min(cx, planet
/// stock)` moves planet `pf` → machine `mf`, then the column tail
/// (files `0x3265F`–`0x32683`, `0x3269C`–`0x326C0`,
/// `0x326D9`–`0x326FD`, `0x32719`–`0x32740`).
pub(super) fn xfer_in(c: &mut Ctx, pf: u16, mf: u16, col: fn(&mut Ctx)) {
    let cx = capacity(c);
    if c.vm.r8(c.st, FULL) != 0 {
        return;
    }
    c.vm.w8(c.st, a!(ARMED), 0);
    let p = c.vm.r16(c.st, a!(REC_CUR));
    let cx = cx.min(c.vm.r16(c.st, p.wrapping_add(pf)));
    let v = c.vm.r16(c.st, p.wrapping_add(pf));
    c.vm.w16(c.st, p.wrapping_add(pf), v - cx);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let v = c.vm.r16(c.st, m + mf);
    c.vm.w16(c.st, m + mf, v.wrapping_add(cx));
    col(c);
}

/// `cs:0x2AF7`/`0x2B20`/`0x2B49`/`0x2B72` — `xfer_out`: `min(5,
/// machine field)` moves `mf` → `pf`, skipped whole while the planet
/// stock sits at `CAP`; always the column tail (file
/// `0x32AF7`–`0x32B9A`).
pub(super) fn xfer_out(c: &mut Ctx, pf: u16, mf: u16, col: fn(&mut Ctx)) {
    c.vm.w8(c.st, a!(ARMED), 0);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let cx = c.vm.r16(c.st, m + mf).min(5);
    let p = c.vm.r16(c.st, a!(REC_CUR));
    if c.vm.r16(c.st, p.wrapping_add(pf)) < CAP {
        let v = c.vm.r16(c.st, m + mf);
        c.vm.w16(c.st, m + mf, v - cx);
        let v = c.vm.r16(c.st, p.wrapping_add(pf));
        c.vm.w16(c.st, p.wrapping_add(pf), v.wrapping_add(cx));
    }
    col(c);
}

/// `cs:0x4C9B` — `xfer_in_people`: `min(5, type +0x1E − crew, planet
/// +0x2A)` into the machine `+0x24`; tail `0x2604` (file
/// `0x34C9B`–`0x34CD7`).
pub(super) fn people_in(c: &mut Ctx) {
    c.vm.w8(c.st, a!(ARMED), 0);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let t = dmach::type_rec(c);
    let cap = c.vm.r16(c.st, t + 0x1E);
    let p = c.vm.r16(c.st, a!(REC_CUR));
    let crew = c.vm.r16(c.st, m + 0x24);
    let cx = if crew.wrapping_add(5) <= cap {
        5
    } else {
        cap.wrapping_sub(crew)
    };
    let pop = c.vm.r16(c.st, p.wrapping_add(f!(F_POP)));
    let cx = cx.min(pop);
    c.vm.w16(c.st, p.wrapping_add(f!(F_POP)), pop - cx);
    c.vm.w16(c.st, m + 0x24, crew.wrapping_add(cx));
    dfield::pop(c);
}

/// `cs:0x4CD8` — `xfer_out_people`: `min(5, +0x24)` off the machine,
/// rolled back when the planet `+0x2A` sits at `CAP`; tail `0x2604`
/// (file `0x34CD8`–`0x34D0A`).
pub(super) fn people_out(c: &mut Ctx) {
    c.vm.w8(c.st, a!(ARMED), 0);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let crew = c.vm.r16(c.st, m + 0x24);
    let cx = crew.min(5);
    c.vm.w16(c.st, m + 0x24, crew - cx);
    let p = c.vm.r16(c.st, a!(REC_CUR));
    let pop = c.vm.r16(c.st, p.wrapping_add(f!(F_POP)));
    if pop < CAP {
        c.vm.w16(c.st, p.wrapping_add(f!(F_POP)), pop.wrapping_add(cx));
    } else {
        c.vm.w16(c.st, m + 0x24, crew);
    }
    dfield::pop(c);
}
