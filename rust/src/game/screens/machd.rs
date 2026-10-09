//! The machine-record detail entry — `cs:0x243D`/`0x918D` (file
//! `0x3243D`–`0x3247E`): the "examine this machine" head that shares
//! the uimode-2 detail screen through the `0x8E10` variant body.

use super::super::consts::{
    HOT_COUNT, HOT_LIST, HOT_SEL, REC_BASE, REC_STRIDE, SEL_MACH, SEQ_AUX, SEQ_PTR,
};
use super::super::vhost::Call;
use super::super::vops::{kind_of, Ctx};
use super::super::{selrec, shell, text, UIMODE};
use super::detail::{enter_detail_sel, frame_loop, ANIM, S_REFUSE};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x918D` — the machine-record detail entry (file `0x3243D`–
/// `0x3247E`): no selected machine or `+0x1F & 4` falls back to the
/// `0x9176` planet path; `0x5085` resolves the machine's `+0x21` host
/// record to the same kind-`0xA`/`0x7402` guard; colonised runs the
/// `0x29D9` machine header then the `0x8E10` uimode-2 variant body.
pub fn enter_detail_mach(c: &mut Ctx) {
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    if m == 0 || c.vm.r8(c.st, m + 0x1F) & 4 != 0 {
        enter_detail_sel(c);
        return;
    }
    let host = mach_host(c, m);
    if kind_of(c, host) != 0xA {
        text::enqueue(c, S_REFUSE);
        shell::shell_enter(c);
        return;
    }
    draw(c);
    selrec::sync_sel(c);
    head(c);
    body(c);
}

/// `cs:0x5085` — machine record → its `+0x21`-indexed host record,
/// `[0x9158] + i·0x3A` (file `0x35085`–`0x350A8`).
fn mach_host(c: &mut Ctx, m: u16) -> u16 {
    let i = u16::from(c.vm.r8(c.st, m + 0x21));
    c.vm.r16(c.st, a!(REC_BASE))
        .wrapping_add(i.wrapping_mul(a!(REC_STRIDE)))
}

/// The `0x2458` draw head — the `0x2493` slots minus the select
/// clears and sound, `Present(3)` (file `0x32458`–`0x32473`).
fn draw(c: &mut Ctx) {
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Native(0x89C3));
    c.host.svc(Call::Image(0x29));
    c.host.svc(Call::Present(3));
}

/// The `0x29D9` machine header (file `0x329D9`–`0x32ADC`): list
/// count `0x17`, `ANIM` armed, `si` = the machine record — the stat
/// prints stay a host call until that block is ported.
fn head(c: &mut Ctx) {
    c.vm.w16(c.st, a!(HOT_COUNT), 0x17);
    c.vm.w8(c.st, ANIM, 0xFF);
    c.host.svc(Call::Native(0x29D9));
}

/// The `0x8E10` variant body — sequencer cells cleared, the `0x9EDE`
/// list at the `0x29D9` count, `uimode 2`, `0x4C8B`/`0x5994`, then
/// the shared frame loop (file `0x38E10`–`0x38E3C`).
fn body(c: &mut Ctx) {
    c.vm.w16(c.st, a!(SEQ_PTR), 0);
    c.vm.w16(c.st, a!(SEQ_AUX), 0);
    c.vm.w16(c.st, 0x91C4, 0);
    let s = c.vm.r16(c.st, 0x9EDC);
    c.vm.w16(c.st, a!(HOT_SEL), s);
    c.vm.w16(c.st, a!(HOT_LIST), 0x9EDE);
    c.vm.w8(c.st, a!(UIMODE), 2);
    c.host.svc(Call::Native(0x4C8B));
    c.host.svc(Call::Native(0x5994));
    frame_loop(c);
}
