//! The detail screen's machine-side paints — `cs:0x5057` type-record
//! lookup, the `0x29D9` header the bay select/`mach detail` paths run,
//! the `0x299C` select tail and the `0x55F2` single-bay auto-select
//! (files `0x35057`–`0x35084`, `0x3299C`–`0x32ADC`, `0x355F2`–`0x3563C`).

use super::super::consts::{
    HOT_COUNT, MACH_BASE, MACH_STRIDE, REC_CUR, SEL_MACH, TYPE_BASE, TYPE_STRIDE,
};
use super::super::mach::M_TYPE;
use super::super::rec::{F_LINK_A, F_LINK_B, F_LINK_C};
use super::super::vhost::Call;
use super::super::vops::Ctx;
use super::super::{num, text};
use super::detail::ANIM;
use super::dfield::{self, SWEEP};
use super::snd;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x5057` — `[0x9180]`'s `+0x1E` type index → its `0x9B12` type
/// record, `(t − 1)·0x30` 16-bit wrapped (file `0x35057`–`0x35084`).
pub(super) fn type_rec(c: &mut Ctx) -> u16 {
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let t = u16::from(c.vm.r8(c.st, m + a!(M_TYPE)));
    a!(TYPE_BASE).wrapping_add(t.wrapping_sub(1).wrapping_mul(a!(TYPE_STRIDE)))
}

/// `cs:0x55F2` — exactly one occupied bay auto-selects it (the
/// `jmp 0x2988` into the `0x299C` tail); none or several leave
/// `[0x9180]` alone (file `0x355F2`–`0x3563C`).
pub(super) fn auto_sel(c: &mut Ctx) {
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    let mut it = [a!(F_LINK_A), a!(F_LINK_B), a!(F_LINK_C)]
        .into_iter()
        .map(|f| c.vm.r8(c.st, rec.wrapping_add(f)))
        .filter(|&b| b != 0);
    if let (Some(b), None) = (it.next(), it.next()) {
        sel_mach(c, b);
    }
}

/// `0x299C` — bay index → `[0x9180]`, the `al = 3` select sfx, then
/// the `0x29D9` header (file `0x3299C`–`0x329D8`).
fn sel_mach(c: &mut Ctx, bay: u8) {
    let m =
        a!(MACH_BASE).wrapping_add(u16::from(bay).wrapping_sub(1).wrapping_mul(a!(MACH_STRIDE)));
    c.vm.w16(c.st, a!(SEL_MACH), m);
    snd(c, 3, 0xA);
    mach_head(c, m);
}

/// `cs:0x29D9` — the machine header: list count `0x17`, `ANIM` armed,
/// the name/capacity/stat prints, the column reprints, the `0x68C7`
/// footer, the sweep reset and the meter-strip clears (file
/// `0x329D9`–`0x32ADC`).
pub(super) fn mach_head(c: &mut Ctx, m: u16) {
    c.vm.w16(c.st, a!(HOT_COUNT), 0x17);
    c.vm.w8(c.st, ANIM, 0xFF);
    text::print_str(c, m, 0x12, 8);
    cap(c);
    stat4(c, 4, 0x16, 0x28, num::num_pad);
    dfield::kind(c);
    dfield::pop(c);
    stat4(c, 0x1E, 0x17, 0x18, num::num4);
    stat_c(c);
    desc(c);
    c.host.svc(Call::Slot(0x128C, 0x78));
    cols(c);
}

/// `0x329EF`–`0x32A13` — the `+0xA` capacity row at `(0x14,0x20)`:
/// `0x7325` for `0xFFFE`, else `0x68B6` + `0x6886`.
fn cap(c: &mut Ctx) {
    let t = type_rec(c);
    if c.vm.r16(c.st, t + 0xA) == 0xFFFE {
        text::print_str(c, 0x7325, 0x14, 0x20);
        return;
    }
    let bx = text::print_str(c, 0x68B6, 0x14, 0x20);
    num::num3(c, c.vm.r16(c.st, t + 0xA), bx, 0x20);
}

/// The `0x5057` reload + word print — `0x32A14`–`0x32A22` runs
/// `0x686C` for `+4`, `0x32A29`–`0x32A37` runs `0x6879` for `+0x1E`.
fn stat4(c: &mut Ctx, f: u16, bx: u16, dx: u16, p: fn(&mut Ctx, u16, u16, u16) -> u16) {
    let t = type_rec(c);
    p(c, c.vm.r16(c.st, t + f), bx, dx);
}

/// `0x32A38`–`0x32A4E` — the `+0xC` word `0x6879`-printed at
/// `(0x17,0x30)`, `0xFFFF` shown as `0`.
fn stat_c(c: &mut Ctx) {
    let t = type_rec(c);
    let v = c.vm.r16(c.st, t + 0xC);
    num::num4(c, if v == 0xFFFF { 0 } else { v }, 0x17, 0x30);
}

/// `0x32A4F`–`0x32A80` — the `0x79A4 + 0x16·(type − 1)` blurb at
/// `(0xD,0x9D)` off `[0x9180]`'s `+0x1E`.
fn desc(c: &mut Ctx) {
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    let t = u16::from(c.vm.r8(c.st, m + a!(M_TYPE))).wrapping_sub(1);
    text::print_str(c, 0x79A4u16.wrapping_add(t.wrapping_mul(0x16)), 0xD, 0x9D);
}

/// `0x32AA0`–`0x32ADC` — the four column reprints, `0x293D` total,
/// `0x68C7` footer, `[0x91C4]` reset and the four `0x1262` strip
/// clears at `0x18/0x1C/0x20/0x24`.
fn cols(c: &mut Ctx) {
    dfield::col_food(c);
    dfield::col_min(c);
    dfield::col_fuel(c);
    dfield::col_nrg(c);
    text::print_str(c, 0x68C7, 0xA, 0x52);
    c.vm.w16(c.st, SWEEP, 0);
    for i in 0..4u16 {
        c.host.svc(Call::Rect(0x18 + i * 4, 0xC, 0, 0x92));
    }
}
