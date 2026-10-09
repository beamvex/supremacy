//! The uimode-2 planet-detail screen — `cs:0x91CF`/`0x9179`/`0x9176`
//! (file `0x32426`–`0x325DF`). Three entry heads over the shared
//! `0x2493` body: all guard on the selected record's `+0xC` kind
//! (`0xA` = colonised) and refuse with the `0x7402` ticker message.

use super::super::consts::{
    BUTTONS, DIRTY0, DIRTY1, FRAME, REC_CUR, SEL_MACH, SEL_REC, SEQ_PTR, SNAP0, SNAP1,
};
use super::super::vhost::Call;
use super::super::vops::{kind_of, vsync, Ctx};
use super::super::{menu, selrec, shell, text, tick_step};
use super::{bump, install, leave, snd};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:` slot of the detail-screen animation flag (`[0x91DD]`).
pub(super) const ANIM: u16 = 0x91DD;
/// The `0x7402` "not colonised" refusal message.
pub(super) const S_REFUSE: u16 = 0x7402;

/// `cs:0x9179` — detail entry: on `kind != 0xA` the refusal enqueues
/// and the `jmp 0x2CC5` re-enters the shell; `kind == 0xA` falls into
/// `0x2493` (file `0x32429`–`0x3243A`).
pub fn enter_detail(c: &mut Ctx) {
    if kind_of(c, c.vm.r16(c.st, a!(REC_CUR))) != 0xA {
        text::enqueue(c, S_REFUSE);
        shell::shell_enter(c);
        return;
    }
    body(c);
}

/// `cs:0x91CF` — the `[0x1274]` window variant: the refusal is a bare
/// `jmp 0x6B06` (enqueue, no shell re-entry) and `kind == 0xA` opens
/// the window first (file `0x3247F`–`0x32492`).
pub fn enter_detail_win(c: &mut Ctx) {
    if kind_of(c, c.vm.r16(c.st, a!(REC_CUR))) != 0xA {
        text::enqueue(c, S_REFUSE);
        return;
    }
    c.host.svc(Call::Slot(0x1274, 0));
    body(c);
}

/// `cs:0x9176` — `0x5EE6` selection resync, then `0x9179` (file
/// `0x32426`–`0x32428`).
pub fn enter_detail_sel(c: &mut Ctx) {
    selrec::sync_sel(c);
    enter_detail(c);
}

/// The `0x2493` shared body — screen draw, the `0x9EDE` 8-record list
/// install, `uimode 2`, then the detail frame loop (file
/// `0x32493`–`0x3251E`).
fn body(c: &mut Ctx) {
    c.vm.w16(c.st, a!(SEL_MACH), 0);
    c.vm.w8(c.st, ANIM, 0);
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Native(0x89C3));
    c.host.svc(Call::Image(0x29));
    c.host.svc(Call::Present(2));
    snd(c, 0xA, 8);
    c.vm.w16(c.st, a!(SEQ_PTR), 0);
    c.vm.w16(c.st, 0x91C4, 0);
    install(c, 0x9EDC, 0x9EDE, 8, 2);
    c.vm.w16(c.st, a!(SEL_REC), c.vm.r16(c.st, a!(REC_CUR)));
    c.host.svc(Call::Native(0x4C8B));
    c.host.svc(Call::Native(0x5994));
    c.host.svc(Call::Slot(0x128C, 0x78));
    for f in [0x55F2, 0x2684, 0x26C1, 0x2741, 0x26FE] {
        c.host.svc(Call::Native(f));
    }
    frame_loop(c);
}

/// One detail frame (file `0x3251F`–`0x325D8`): vsync, the name
/// alternator, the `[0x9144]`/`[0x9146]` dirty dispatch (one bit),
/// `0x2759`, tick, input, menu — exits to the shell on
/// `[0x9CDA] & 2`.
pub(super) fn frame_loop(c: &mut Ctx) {
    loop {
        vsync(c);
        anim(c);
        dirty(c);
        c.host.svc(Call::Native(0x2759));
        let _ = tick_step(c);
        c.host.svc(Call::Native(0xA369));
        menu::service(c);
        bump(c);
        if c.vm.r16(c.st, a!(BUTTONS)) & 2 != 0 {
            break;
        }
    }
    leave(c, 0x9EDC);
}

/// The `0x69BF`/`0x68C7` print alternator at `(0xA,0x52)`, skipped
/// while `[0x91DD]` is set (file `0x32522`–`0x32543`).
fn anim(c: &mut Ctx) {
    if c.vm.r8(c.st, ANIM) != 0 {
        return;
    }
    let s = if c.vm.r8(c.st, a!(FRAME)) & 7 < 4 {
        0x69BF
    } else {
        0x68C7
    };
    text::print_str(c, s, 0xA, 0x52);
}

/// The dirty-word snapshot + dispatch — at most one bit serviced per
/// frame, cleared in the live word (file `0x32545`–`0x325BD`).
fn dirty(c: &mut Ctx) {
    let (d0, d1) = (c.vm.r16(c.st, a!(DIRTY0)), c.vm.r16(c.st, a!(DIRTY1)));
    c.vm.w16(c.st, a!(SNAP0), d0);
    c.vm.w16(c.st, a!(SNAP1), d1);
    if d0 & 0x800 != 0 {
        serve(c, 0x25EB, DIRTY0, !0x800);
    } else if d0 & 0x20 != 0 {
        serve(c, 0x2604, DIRTY0, !0x20);
    } else if d0 & 0x4000 != 0 {
        serve(c, 0x2684, DIRTY0, !0x4000);
    } else if d0 & 0x8000 != 0 {
        serve(c, 0x26C1, DIRTY0, !0x8000);
    } else if d1 & 1 != 0 {
        serve(c, 0x26FE, DIRTY1, !1);
    } else if d1 & 2 != 0 {
        serve(c, 0x2741, DIRTY1, !2);
    }
}

/// One deferred arm: host call then `and` the bit off.
fn serve(c: &mut Ctx, f: u16, cell: usize, mask: u16) {
    c.host.svc(Call::Native(f));
    let a = u16::try_from(cell).unwrap_or(0);
    c.vm.w16(c.st, a, c.vm.r16(c.st, a) & mask);
}
