//! The `0x540C` commit tail (`0x35511`–`0x355F1`) — the destination's
//! TO print, the distance/fuel readouts, the `0x72BA`/`0x7300`
//! refuses and the flag-`8` transit commit.

use super::super::consts::{DIRTY0, G_DEEPMINE, LAUNCH_F, SEL_REC};
use super::super::mach::{M_DEPOSIT, M_FLAGS, M_HOST, M_LINK, M_OPS};
use super::super::rec::F_NAME;
use super::super::vops::{set_bits, Ctx};
use super::super::{num, text};
use super::{bfield, bfly, bpaint, dmach};

/// `u16` casts for the `ds:`/record constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `[0x91B4]` — the transit-distance cell.
const DIST: u16 = 0x91B4;
/// `[0x8209]` — the launch-commit flag the `ev05` event waits on.
const LAUNCHED: u16 = 0x8209;

/// `0x35511` — the release wait, the TO name + `0x52FE`, the
/// distance/fuel maths and the commit; `d == 0` or short fuel take
/// the `0x52DD` refuses.
pub(super) fn commit(c: &mut Ctx, m: u16) {
    bfly::release(c);
    let dest = c.vm.r16(c.st, a!(SEL_REC));
    text::print_str(c, dest.wrapping_add(a!(F_NAME)), 0x34, 0xA3);
    bfield::prompt(c);
    let d = dist(c, m, dest);
    if d == 0 {
        return same_planet(c);
    }
    let cost = report(c, d);
    if short_fuel(c, m, cost) {
        return;
    }
    commit_mach(c, m, dest, d);
}

/// `0x35532` — `([dest+0x28] − [u+0x21])` as a signed byte, its abs
/// stored at `[0x91B4]`; `0` is "already there".
fn dist(c: &mut Ctx, m: u16, dest: u16) -> u16 {
    let d = i8::from_ne_bytes([c
        .vm
        .r8(c.st, dest.wrapping_add(0x28))
        .wrapping_sub(c.vm.r8(c.st, m + a!(M_HOST)))]);
    let v = u16::from(d.unsigned_abs());
    c.vm.w16(c.st, DIST, v);
    v
}

/// `0x35565`–`0x355A1` — the `DAYS` readout at `(0x35,0xAD)` and the
/// `×0x19` (×2 while `[0x822D]` is clear) fuel cost `T.` at
/// `(0x36,0xB7)`; returns the cost.
fn report(c: &mut Ctx, d: u16) -> u16 {
    let bx = num::num3(c, d, 0x35, 0xAD);
    text::print_str(c, 0x7C0D, bx, 0xAD);
    let mut cost = d.wrapping_mul(0x19);
    if c.vm.r8(c.st, a!(G_DEEPMINE)) == 0 {
        cost = cost.wrapping_add(cost);
    }
    let bx = num::num4(c, cost, 0x36, 0xB7);
    text::print_str(c, 0x7C09, bx, 0xB7);
    cost
}

/// `0x3559E` — the `0x5057` `+0xC` fuel gate: non-nuclear with
/// `cost > [u+0x26]` restores `[0x917C]`, refuses `0x7300` at `0x3000`
/// and repaints the machine grid (the `jmp 0x6FBF` exit).
fn short_fuel(c: &mut Ctx, m: u16, cost: u16) -> bool {
    let t = dmach::type_rec(c);
    if c.vm.r16(c.st, t + 0xC) == 0xFFFF || cost <= c.vm.r16(c.st, m + a!(M_DEPOSIT)) {
        return false;
    }
    let rec = c.vm.r16(c.st, bfly::SAVE_REC);
    c.vm.w16(c.st, a!(SEL_REC), rec);
    bfield::refuse(c, 0x7300, 0x1A, 0x3000);
    bpaint::mach_mode(c);
    true
}

/// `0x35547` — `d == 0`: `[0x91E3]` down, the `0x72BA` refuse, the
/// `0x6FBF` repaint, the `0x56BF` erase (the `jmp` tail).
fn same_planet(c: &mut Ctx) {
    c.vm.w8(c.st, a!(LAUNCH_F), 0);
    bfield::refuse(c, 0x72BA, 0x1C, 0x2000);
    bpaint::mach_mode(c);
    bfield::erase_head(c);
}

/// `0x355CB` — `[0x8209]`/`[0x91E3]` flags, `+0x18 = dist`,
/// `+0x1F | 8`, `+0x1A = dest`, `d0 |= 0x3000`, `jmp 0x6FBF`.
fn commit_mach(c: &mut Ctx, m: u16, dest: u16, d: u16) {
    c.vm.w8(c.st, LAUNCHED, 0xFF);
    c.vm.w8(c.st, a!(LAUNCH_F), 0);
    c.vm.w16(c.st, m + a!(M_OPS), d);
    let fl = c.vm.r8(c.st, m + a!(M_FLAGS)) | 8;
    c.vm.w8(c.st, m + a!(M_FLAGS), fl);
    c.vm.w16(c.st, m + a!(M_LINK), dest);
    set_bits(c, a!(DIRTY0), 0x3000);
    bpaint::mach_mode(c);
}
