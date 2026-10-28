//! The detail screen's field prints — the `body` initial calls
//! (`cs:0x4C8B` planet name, `0x5994` bay names) and the `dirty()`
//! reprint tails (`0x25EB`/`0x2604`/`0x2684`/`0x26C1`/`0x26FE`/
//! `0x2741`) (files `0x325EB`–`0x3262B`, `0x32684`–`0x32758`,
//! `0x3290A`–`0x3297D`, `0x34C8B`–`0x34C9A`, `0x35994`–`0x359EA`).

use super::super::consts::{MACH_BASE, MACH_STRIDE, REC_CUR, SEL_MACH, SEL_REC};
use super::super::rec::{
    F_ENERGY, F_FOOD, F_FUEL, F_LINK_A, F_LINK_B, F_LINK_C, F_MINERALS, F_NAME, F_POP,
};
use super::super::vops::Ctx;
use super::super::{num, text};
use super::detail::ANIM;
use super::dmach;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:` cell of the meter sweep counter `[0x91C4]` — shared with
/// `body`'s clear and [`dmach::mach_head`]'s reset.
pub(super) const SWEEP: u16 = 0x91C4;

/// `cs:0x4C8B` — the selected record's `+0xE` name at `(0x3,0xE)`
/// (file `0x34C8B`–`0x34C9A`).
pub(super) fn name(c: &mut Ctx) {
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    text::print_str(c, rec.wrapping_add(a!(F_NAME)), 3, 0xE);
}

/// `cs:0x5994` — the three bay names at `(5,0x29)`/`(5,0x34)`/
/// `(5,0x3F)`: an empty bay prints `0x6B2E`, a docked one its machine
/// record's name (file `0x35994`–`0x359EA`).
pub(super) fn bays(c: &mut Ctx) {
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    for (f, row) in [
        (a!(F_LINK_A), 0x29u16),
        (a!(F_LINK_B), 0x34),
        (a!(F_LINK_C), 0x3F),
    ] {
        let b = c.vm.r8(c.st, rec.wrapping_add(f));
        let s = if b == 0 { 0x6B2E } else { mach_at(b) };
        text::print_str(c, s, 5, row);
    }
}

/// `0x9491 + (b − 1)·0x28` — a 1-based bay index to its machine record.
fn mach_at(b: u8) -> u16 {
    a!(MACH_BASE).wrapping_add(u16::from(b).wrapping_sub(1).wrapping_mul(a!(MACH_STRIDE)))
}

/// `cs:0x25EB` — `d0 & 0x800`: the docked machine's `+0xC` word padded
/// at `(0x16,0x40)`; no machine → nothing (file `0x325EB`–`0x32603`).
pub(super) fn kind(c: &mut Ctx) {
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if m == 0 {
        return;
    }
    num::num_pad(c, c.vm.r16(c.st, m + 0xC), 0x16, 0x40);
}

/// `cs:0x2604` — `d0 & 0x20`: `ANIM`-gated — population padded at
/// `(0x16,0x10)` and the machine `+0x24` crew `0x6879`-printed at
/// `(0x11,0x7A)` (file `0x32604`–`0x3262B`).
pub(super) fn pop(c: &mut Ctx) {
    if c.vm.r8(c.st, ANIM) == 0 {
        return;
    }
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    num::num_pad(c, c.vm.r16(c.st, rec.wrapping_add(a!(F_POP))), 0x16, 0x10);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    num::num4(c, c.vm.r16(c.st, m + 0x24), 0x11, 0x7A);
}

/// `cs:0x2961` — one STORES pair: planet `[si]` at `(bx,0xB4)`, then
/// when `ANIM` the machine `[di]` reprints on the `0xBE` row (file
/// `0x32961`–`0x3297D`).
fn pair(c: &mut Ctx, si: u16, di: u16, bx: u16) {
    num::num_pad(c, c.vm.r16(c.st, si), bx, 0xB4);
    if c.vm.r8(c.st, ANIM) == 0 {
        return;
    }
    num::num_pad(c, c.vm.r16(c.st, di), bx, 0xBE);
}

/// `cs:0x293D` — the machine `+0xA/E/10/12` load total at
/// `(0x16,0x38)`, `ANIM`-gated (file `0x3293D`–`0x32960`).
fn total(c: &mut Ctx) {
    if c.vm.r8(c.st, ANIM) == 0 {
        return;
    }
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let v = [0xAu16, 0xE, 0x10, 0x12]
        .iter()
        .fold(0u16, |s, &f| s.wrapping_add(c.vm.r16(c.st, m + f)));
    num::num_pad(c, v, 0x16, 0x38);
}

/// `cs:0x290A` — the aux line at `(0x19,0x7A)`: a `0xFFFF` type `+0xC`
/// prints `0x7606` alone, else `0x68B8` + the machine `+0x26` `0x6879`
/// + `0x68B9` (file `0x3290A`–`0x3293C`).
fn fuel(c: &mut Ctx) {
    if c.vm.r8(c.st, ANIM) == 0 {
        return;
    }
    let t = dmach::type_rec(c);
    if c.vm.r16(c.st, t + 0xC) == 0xFFFF {
        text::print_str(c, 0x7606, 0x19, 0x7A);
        return;
    }
    let bx = text::print_str(c, 0x68B8, 0x19, 0x7A);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let bx = num::num4(c, c.vm.r16(c.st, m + 0x26), bx, 0x7A);
    text::print_str(c, 0x68B9, bx, 0x7A);
}

/// `cs:0x2684` — `d0 & 0x4000`: food column `0x30`, planet `+0x2C`
/// against machine `+0xA` (file `0x32684`–`0x3269B`).
pub(super) fn col_food(c: &mut Ctx) {
    let (si, di) = (pfield(c, F_FOOD), mfield(c, 0xA));
    pair(c, si, di, 0x30);
    total(c);
}

/// `cs:0x26C1` — `d0 & 0x8000`: minerals column `0x38`, planet `+0x30`
/// against machine `+0xE` (file `0x326C1`–`0x326D8`).
pub(super) fn col_min(c: &mut Ctx) {
    let (si, di) = (pfield(c, F_MINERALS), mfield(c, 0xE));
    pair(c, si, di, 0x38);
    total(c);
}

/// `cs:0x26FE` — `d1 & 1`: fuel column `0x40`, planet `+0x32` against
/// machine `+0x10`, plus the `0x290A` aux line (file
/// `0x326FE`–`0x32718`).
pub(super) fn col_fuel(c: &mut Ctx) {
    let (si, di) = (pfield(c, F_FUEL), mfield(c, 0x10));
    pair(c, si, di, 0x40);
    fuel(c);
    total(c);
}

/// `cs:0x2741` — `d1 & 2`: energy column `0x48`, planet `+0x34`
/// against machine `+0x12` (file `0x32741`–`0x32758`).
pub(super) fn col_nrg(c: &mut Ctx) {
    let (si, di) = (pfield(c, F_ENERGY), mfield(c, 0x12));
    pair(c, si, di, 0x48);
    total(c);
}

/// `[0x9184] + f` / `[0x9180] + f` — the `si`/`di` pair pointers.
fn pfield(c: &Ctx, f: usize) -> u16 {
    c.vm.r16(c.st, a!(REC_CUR)).wrapping_add(a!(f))
}

/// The machine half of [`pfield`].
fn mfield(c: &Ctx, f: u16) -> u16 {
    c.vm.r16(c.st, a!(SEL_MACH)).wrapping_add(f)
}
