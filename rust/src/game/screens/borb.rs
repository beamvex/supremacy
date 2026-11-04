//! `cs:0x52C9` `build_orbit` and `cs:0x563D` `build_deploy` — the
//! build screen's flight-state actions (files `0x352C9`–`0x3540B`,
//! `0x3563D`–`0x356BE`).

use super::super::consts::{DIRTY0, REC_BASE, REC_STRIDE, SEL_MACH, SEL_REC};
use super::super::mach::{M_DEPOSIT, M_FLAGS, M_HOST, M_LINK, M_OPS, M_TYPE};
use super::super::rec::{F_LINK_A, F_LINK_B, F_LINK_C};
use super::super::vops::{set_bits, Ctx};
use super::{bact, bfield, dmach, machd};

/// `u16` casts for the `ds:`/record constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x52C9` — `build_orbit`: the guards, then the matching
/// `[0x917C]` bay byte cleared (`+0x20` on a miss), `+0x1F | 4`,
/// `+0x26 − 0x64`, `d0 |= 0x3000`, the `al=0x21`/`cl=0x14` sfx.
pub(super) fn build_orbit(c: &mut Ctx) {
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if m == 0 {
        return bfield::refuse(c, 0x71E5, 0x1F, 0x2000);
    }
    if orbit_guard(c, m) {
        return;
    }
    let idx = bact::mach_idx(m);
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    let f = [a!(F_LINK_A), a!(F_LINK_B), a!(F_LINK_C)]
        .into_iter()
        .find(|&f| c.vm.r8(c.st, rec.wrapping_add(f)) == idx)
        .unwrap_or(a!(F_LINK_C));
    c.vm.w8(c.st, rec.wrapping_add(f), 0);
    let fl = c.vm.r8(c.st, m + a!(M_FLAGS)) | 4;
    c.vm.w8(c.st, m + a!(M_FLAGS), fl);
    let v = c.vm.r16(c.st, m + a!(M_DEPOSIT)).wrapping_sub(0x64);
    c.vm.w16(c.st, m + a!(M_DEPOSIT), v);
    set_bits(c, a!(DIRTY0), 0x3000);
    super::snd(c, 0x21, 0x14);
    bfield::prompt(c);
}

/// The `0x3530A`–`0x353A3` guards — flag `8` → `0x71F8`, `+0x1E == 4`
/// → `0x7287`, `&4` → `0x75BF`, `&0x10` → `0x73CA`, then the
/// `0x5057`-keyed fuel/crew checks → `0x7300`/`0x72EB`.
fn orbit_guard(c: &mut Ctx, m: u16) -> bool {
    let fl = c.vm.r8(c.st, m + a!(M_FLAGS));
    if fl & 8 != 0 {
        bfield::refuse(c, 0x71F8, 0x1A, 0x2000);
        return true;
    }
    if c.vm.r8(c.st, m + a!(M_TYPE)) == 4 {
        bfield::refuse(c, 0x7287, 0xE, 0x2000);
        return true;
    }
    if fl & 4 != 0 {
        bfield::refuse(c, 0x75BF, 0x1A, 0x2000);
        return true;
    }
    if fl & 0x10 != 0 {
        bfield::refuse(c, 0x73CA, 0x12, 0x2000);
        return true;
    }
    power_guard(c, m)
}

/// `0x3535F`/`0x35381` — the `0x5057`-keyed fuel (`+0xC`/`+0x26` <
/// `0x64` → `0x7300`) and crew (`+0xA`/`+0x14 == 0` → `0x72EB`) checks.
fn power_guard(c: &mut Ctx, m: u16) -> bool {
    let t = dmach::type_rec(c);
    if c.vm.r16(c.st, t + 0xC) != 0xFFFF && c.vm.r16(c.st, m + a!(M_DEPOSIT)) < 0x64 {
        bfield::refuse(c, 0x7300, 0x1B, 0x2000);
        return true;
    }
    let t = dmach::type_rec(c);
    if c.vm.r16(c.st, t + 0xA) != 0xFFFE && c.vm.r16(c.st, m + 0x14) == 0 {
        bfield::refuse(c, 0x72EB, 0x1E, 0x2000);
        return true;
    }
    false
}

/// `cs:0x563D` — `build_deploy`: a flag-`8` unit's `+0x21` index
/// re-seats `+0x18` records past/behind the `+0x1A` destination, the
/// flag clears, `d0 |= 0x3000`, the `al=3` blip and `[0x917C]` becomes
/// the resolved `0x5085` host.
pub(super) fn build_deploy(c: &mut Ctx) {
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if m == 0 || c.vm.r8(c.st, m + a!(M_FLAGS)) & 8 == 0 {
        return;
    }
    let base = c.vm.r16(c.st, a!(REC_BASE));
    let dest = c.vm.r16(c.st, m + a!(M_LINK)).wrapping_sub(base) / a!(REC_STRIDE);
    let eta = c.vm.r16(c.st, m + a!(M_OPS));
    let host = u16::from(c.vm.r8(c.st, m + a!(M_HOST)));
    let dl = if host < dest & 0xFF {
        dest.wrapping_sub(eta)
    } else {
        dest.wrapping_add(eta)
    };
    set_bits(c, a!(DIRTY0), 0x3000);
    let fl = c.vm.r8(c.st, m + a!(M_FLAGS)) & 0xF7;
    c.vm.w8(c.st, m + a!(M_FLAGS), fl);
    c.vm.w8(c.st, m + a!(M_HOST), u8::try_from(dl & 0xFF).unwrap_or(0));
    super::snd(c, 3, 0xA);
    let h = machd::mach_host(c, m);
    c.vm.w16(c.st, a!(SEL_REC), h);
}
