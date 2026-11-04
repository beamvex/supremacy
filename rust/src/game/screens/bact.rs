//! The `0xA482` build-list dispatch and the bay-side actions —
//! `cs:0x5197`/`0x51A3`/`0x51AF` `bmach_sel 1/2/3` and `cs:0x51F5`
//! `build_xfer` (bay load). `cs:0x52C9` `build_orbit` and `0x563D`
//! `build_deploy` live in [`borb`], `0x56E1` `build_rename` and the
//! `0x7119` machine-grid click in [`bren`], the `0x540C` launch pick
//! in [`bfly`] (files `0x35197`–`0x352C8`).

use super::super::consts::{DIRTY0, MACH_BASE, MACH_STRIDE, SEL_MACH, SEL_REC};
use super::super::mach::{M_FLAGS, M_LINK};
use super::super::rec::{F_LINK_A, F_LINK_B, F_LINK_C, F_OWNER};
use super::super::vhost::Call;
use super::super::vops::{set_bits, Ctx};
use super::{bfield, bfly, borb, bren, machd};

/// `u16` casts for the `ds:`/record constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// The `0xA482` records' `+0xC` words — both windows' forms. Anything
/// else drops to the `Call::Native` sink.
pub(super) fn dispatch(c: &mut Ctx, t: u16) {
    match t {
        0x5197 | 0xBEE7 => bmach_sel(c, a!(F_LINK_A)),
        0x51A3 | 0xBEF3 => bmach_sel(c, a!(F_LINK_B)),
        0x51AF | 0xBEFF => bmach_sel(c, a!(F_LINK_C)),
        0x51F5 | 0xBF45 => build_xfer(c),
        0x52C9 | 0xC019 => borb::build_orbit(c),
        0x540C | 0xC15C => bfly::build_fly(c),
        0x563D | 0xC38D => borb::build_deploy(c),
        0x56E1 | 0xC431 => bren::build_rename(c),
        _ => drop(c.host.svc(Call::Native(t))),
    }
}

/// `cs:0x5197`/`0x51A3`/`0x51AF` — `bmach_sel 1/2/3`: the `[0x917C]`
/// bay byte at `+bay` → `[0x9180]`; empty takes the `0x52DD` `0x69B1`
/// refuse; a flag-`8` unit re-points `[0x917C]` at its `+0x1A` host
/// (file `0x35197`–`0x351F4`).
fn bmach_sel(c: &mut Ctx, bay: u16) {
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    let idx = c.vm.r8(c.st, rec.wrapping_add(bay));
    if idx == 0 {
        return bfield::refuse(c, 0x69B1, 0x19, 0x2000);
    }
    let unit =
        a!(MACH_BASE).wrapping_add(u16::from(idx).wrapping_sub(1).wrapping_mul(a!(MACH_STRIDE)));
    c.vm.w16(c.st, a!(SEL_MACH), unit);
    if c.vm.r8(c.st, unit + a!(M_FLAGS)) & 8 != 0 {
        let host = c.vm.r16(c.st, unit + a!(M_LINK));
        c.vm.w16(c.st, a!(SEL_REC), host);
    }
    set_bits(c, a!(DIRTY0), 0x3000);
}

/// `cs:0x51F5` — `build_xfer`: the guards, then the unit index into
/// the first free `[0x917C]` bay, `+0x1F & 0xFB`, `d0 |= 0x3000`, the
/// `al=0x1E`/`cl=0x15` sfx and the `0x52FE` tail (file
/// `0x351F5`–`0x352C8`).
fn build_xfer(c: &mut Ctx) {
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if m == 0 {
        return bfield::refuse(c, 0x71E5, 0x1F, 0x2000);
    }
    if xfer_guard(c, m) {
        return;
    }
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    let Some(f) = [a!(F_LINK_A), a!(F_LINK_B), a!(F_LINK_C)]
        .into_iter()
        .find(|&f| c.vm.r8(c.st, rec.wrapping_add(f)) == 0)
    else {
        return bfield::refuse(c, 0x7215, 0x16, 0x2000);
    };
    c.vm.w8(c.st, rec.wrapping_add(f), mach_idx(m));
    let fl = c.vm.r8(c.st, m + a!(M_FLAGS)) & 0xFB;
    c.vm.w8(c.st, m + a!(M_FLAGS), fl);
    set_bits(c, a!(DIRTY0), 0x3000);
    super::snd(c, 0x1E, 0x15);
    bfield::prompt(c);
}

/// The `0x3520C`–`0x35251` guards — flag `8` → `0x71F8`, `!&4` →
/// `0x7239`, `0x5085` host `+0x24 == 0` → `0x75D9`.
fn xfer_guard(c: &mut Ctx, m: u16) -> bool {
    let fl = c.vm.r8(c.st, m + a!(M_FLAGS));
    if fl & 8 != 0 {
        bfield::refuse(c, 0x71F8, 0x1A, 0x2000);
        return true;
    }
    if fl & 4 == 0 {
        bfield::refuse(c, 0x7239, 0x1C, 0x2000);
        return true;
    }
    let h = machd::mach_host(c, m);
    if c.vm.r8(c.st, h.wrapping_add(a!(F_OWNER))) == 0 {
        bfield::refuse(c, 0x75D9, 0x11, 0x2000);
        return true;
    }
    false
}

/// `(m − 0x9491)/0x28 + 1` — the 1-based bay byte for `m`.
pub(super) fn mach_idx(m: u16) -> u8 {
    let i = m.wrapping_sub(a!(MACH_BASE)) / a!(MACH_STRIDE);
    u8::try_from(i).unwrap_or(0).wrapping_add(1)
}
