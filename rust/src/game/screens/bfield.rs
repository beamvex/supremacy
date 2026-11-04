//! The build screen's draw helpers — the `cs:0x4EB6` dirty-word
//! redraw (file `0x34EB6`–`0x35056`), the `0x52DD`/`0x52FE` message
//! tail (file `0x352DD`–`0x35309`) and the `0x56BF` heading erase
//! (`0x356BF`–`0x356E0`). The `0x6FB8`/`0x6FBF`/`0x6FD2` grid paints
//! live in [`bpaint`].

use super::super::consts::{DIRTY0, DIRTY1, FRAME, SEL_MACH, SEL_REC, SNAP0, SNAP1};
use super::super::mach::{M_DEPOSIT, M_FLAGS, M_LINK, M_OPS, M_TYPE};
use super::super::rec::{F_NAME, F_OWNER};
use super::super::vhost::Call;
use super::super::vops::{set_bits, Ctx};
use super::super::{dialog, num, text};
use super::{dfield, dmach, galmsg, machd};

/// `u16` casts for the `ds:`/record constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x4EB6` — the per-frame redraw: the empty-selection prompt
/// blink while `[0x9180] == 0`, else the `0x1000`/`0x2000` dirty
/// blocks off fresh `[0x9CBC]`/`[0x9CBE]` snapshots, then both dirty
/// words cleared.
pub(super) fn redraw(c: &mut Ctx) {
    if c.vm.r16(c.st, a!(SEL_MACH)) == 0 {
        blink(c);
        return;
    }
    snap(c);
    if c.vm.r16(c.st, a!(SNAP0)) & 0x1000 != 0 {
        head(c);
    }
    snap(c);
    if c.vm.r16(c.st, a!(SNAP0)) & 0x2000 != 0 {
        unit(c);
    }
    c.vm.w16(c.st, a!(DIRTY0), 0);
    c.vm.w16(c.st, a!(DIRTY1), 0);
}

/// `0x4EBE` — the `[0x91D4] & 0x3F` blink: `0` reprints the `0x68CF`
/// erase at `(0x2D,0x5F)`, `0x20` the `0x69ED` prompt at `(0x12,0x5F)`.
fn blink(c: &mut Ctx) {
    match c.vm.r8(c.st, a!(FRAME)) & 0x3F {
        0 => drop(text::print_str(c, 0x68CF, 0x2D, 0x5F)),
        0x20 => drop(text::print_str(c, 0x69ED, 0x12, 0x5F)),
        _ => {}
    }
}

/// `0x4EE2` — the `[0x9144]`/`[0x9146]` snapshot into
/// `[0x9CBC]`/`[0x9CBE]` the dirty tests read (re-armed between the
/// blocks — a reprint can dirty the words again).
fn snap(c: &mut Ctx) {
    let d0 = c.vm.r16(c.st, a!(DIRTY0));
    c.vm.w16(c.st, a!(SNAP0), d0);
    let d1 = c.vm.r16(c.st, a!(DIRTY1));
    c.vm.w16(c.st, a!(SNAP1), d1);
}

/// `0x4EF6` — `d0 & 0x1000`: the prompt erase, the `[0x917C]` planet
/// name at `(3,0xE)`, the unit's `+0` name at `(0x1D,0x2B)` and its
/// `0x79A4` type name at `(0x1C,0x48)`, the `0x15C`-based icon put,
/// then `0x5994` bays.
fn head(c: &mut Ctx) {
    prompt(c);
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    text::print_str(c, rec.wrapping_add(a!(F_NAME)), 3, 0xE);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    text::print_str(c, m, 0x1D, 0x2B);
    let t = u16::from(c.vm.r8(c.st, m + a!(M_TYPE))).wrapping_sub(1);
    text::print_str(c, 0x79A4u16.wrapping_add(t.wrapping_mul(0x16)), 0x1C, 0x48);
    let icon = 0x15Cu16.wrapping_add(t);
    c.host.svc(Call::Icon(icon, 0x21, 0x33));
    dfield::bays(c);
}

/// `0x4F80` — `d0 & 0x2000`: the crew/fuel spec pair, then the unit
/// name and flag-word status chain on the `0x5F` row.
fn unit(c: &mut Ctx) {
    crew(c);
    fuel(c);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let bx = text::print_str(c, m, 0xE, 0x5F);
    let fl = c.vm.r8(c.st, m + a!(M_FLAGS));
    if fl & 8 != 0 {
        transit(c, m, bx);
    } else {
        berthed(c, m, fl, bx);
    }
}

/// `0x4F80` — `0x5057` `+0xA` gate: `0x7325` "REMOTE " alone for
/// `0xFFFE`, else `0x68B7` + the `+0x14` crew `0x6879` field.
fn crew(c: &mut Ctx) {
    let t = dmach::type_rec(c);
    if c.vm.r16(c.st, t + 0xA) == 0xFFFE {
        text::print_str(c, 0x7325, 0x2A, 0x2B);
        return;
    }
    let bx = text::print_str(c, 0x68B7, 0x2A, 0x2B);
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    num::num4(c, c.vm.r16(c.st, m + 0x14), bx, 0x2B);
}

/// `0x4FA7` — `0x5057` `+0xC` gate: `0x731C` "NUCLEAR " for `0xFFFF`,
/// else the `+0x26` `0x686C` field + `0x7C09` " T.".
fn fuel(c: &mut Ctx) {
    let t = dmach::type_rec(c);
    if c.vm.r16(c.st, t + 0xC) == 0xFFFF {
        text::print_str(c, 0x731C, 0x29, 0x3A);
        return;
    }
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let bx = num::num_pad(c, c.vm.r16(c.st, m + a!(M_DEPOSIT)), 0x29, 0x3A);
    text::print_str(c, 0x7C09, bx, 0x3A);
}

/// `0x34FE6` — the flag-`8` chain: `" IS NOW IN TRANSIT TO "`,
/// `[u+0x1A]+0xE`, `" : E.D.A. "`, the `+0x18` `0x6999` count, `0x68B9`.
fn transit(c: &mut Ctx, m: u16, bx: u16) {
    let bx = text::print_str(c, 0x6A85, bx, 0x5F);
    let d = c.vm.r16(c.st, m + a!(M_LINK));
    let bx = text::print_str(c, d.wrapping_add(a!(F_NAME)), bx, 0x5F);
    let bx = text::print_str(c, 0x6AB3, bx, 0x5F);
    let bx = num::num(c, c.vm.r16(c.st, m + a!(M_OPS)), bx, 0x5F);
    text::print_str(c, 0x68B9, bx, 0x5F);
}

/// `0x3501F` — the berthed chain: `&4` → `0x6ABE` orbit, `&0x10` →
/// `0x6A9C` surface, else `0x6A6C` bay, then the `0x5085` host's name
/// or `0x752E` "A DEAD STAR!" on an ownerless record.
fn berthed(c: &mut Ctx, m: u16, fl: u8, bx: u16) {
    let s = if fl & 4 != 0 {
        0x6ABE
    } else if fl & 0x10 != 0 {
        0x6A9C
    } else {
        0x6A6C
    };
    let bx = text::print_str(c, s, bx, 0x5F);
    let h = machd::mach_host(c, m);
    let s = if c.vm.r8(c.st, h.wrapping_add(a!(F_OWNER))) == 0 {
        0x752E
    } else {
        h.wrapping_add(a!(F_NAME))
    };
    text::print_str(c, s, bx, 0x5F);
}

/// `0x52FE` — the `0x68DC` erase print at `(0xB,0x5F)` the message
/// tails bracket with.
pub(super) fn prompt(c: &mut Ctx) {
    text::print_str(c, 0x68DC, 0xB, 0x5F);
}

/// `0x52DD` — `mask` into `[0x9144]`, then the message: the erase,
/// `si` at `(bx,0x5F)`, the `0x8594` blip, the `0x6F2D` wait and the
/// closing erase.
pub(super) fn refuse(c: &mut Ctx, s: u16, bx: u16, mask: u16) {
    set_bits(c, a!(DIRTY0), mask);
    prompt(c);
    text::print_str(c, s, bx, 0x5F);
    dialog::snd_block(c);
    galmsg::wait(c);
    prompt(c);
}

/// `0x56BF` — the five-row heading erase: `0x68B1` at `(0x34,0x8F)`
/// stepping `dx + 0xA`.
pub(super) fn erase_head(c: &mut Ctx) {
    let mut dx = 0x8Fu16;
    for _ in 0..5 {
        text::print_str(c, 0x68B1, 0x34, dx);
        dx = dx.wrapping_add(0xA);
    }
}
