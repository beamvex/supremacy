//! The uimode-7 surface screen — `cs:0x4036`/`0xAD86` (file
//! `0x34036`–`0x34165`): the windowed planet-surface entry whose
//! record guard admits colonised records (`+0xC == 0xA`), anything
//! with machines aboard (`+0x18`/`+0x1C`/`+0x20`) or a pending
//! `+0x2E`; kinds `4`/`1` answer `0x6AD6`, a bare kind `7` refuses
//! via the shared `0x2434` path.

use super::super::consts::{BUTTONS, DIRTY0, DIRTY1, REC_CUR, SEL_MACH, SEL_REC, SEQ_PTR};
use super::super::vhost::Call;
use super::super::vops::{kind_of, vsync, Ctx};
use super::super::{menu, seq, shell, text, tick_step};
use super::detail::S_REFUSE;
use super::{bump, install, leave, snd};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// The `0x6AD6` "no colony here" ticker message (file `0x3406B`).
const S_NOCOLONY: u16 = 0x6AD6;

/// `cs:0xAD86` — the `[0x1274]`-windowed entry and its `+0xC`/cargo
/// guard (file `0x34036`–`0x34070`).
pub fn enter_surface(c: &mut Ctx) {
    c.host.svc(Call::Slot(0x1274, 0));
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    let kind = kind_of(c, rec);
    if !admitted(c, rec, kind) {
        return;
    }
    body(c);
}

/// The `0x403E`–`0x4068` guard — `0x4071` admits, `0x406B` enqueues
/// `0x6AD6` and returns, `0x2434` refuses with the shell re-entry.
fn admitted(c: &mut Ctx, rec: u16, kind: u16) -> bool {
    if kind == 0xA {
        return true;
    }
    if kind == 4 || kind == 1 {
        text::enqueue(c, S_NOCOLONY);
        return false;
    }
    let aboard = c.vm.r8(c.st, rec + 0x18) | c.vm.r8(c.st, rec + 0x1C) | c.vm.r8(c.st, rec + 0x20);
    if aboard != 0 || c.vm.r16(c.st, rec + 0x2E) != 0 {
        return true;
    }
    if kind != 7 {
        text::enqueue(c, S_NOCOLONY);
        return false;
    }
    text::enqueue(c, S_REFUSE);
    shell::shell_enter(c);
    false
}

/// `0x4071` — the open: chirp + `0x8A80` channel `5`,
/// `[0x917C] = [0x9184]`, the draw head, the clears, the `0x4166`
/// idle banner, the `0xABA4` `0x2A`-record `uimode-7` install, the
/// table build + header prints, then the frame loop.
fn body(c: &mut Ctx) {
    snd(c, 0, 1);
    c.host.svc(Call::Chan(0x8A80, 5));
    let cur = c.vm.r16(c.st, a!(REC_CUR));
    c.vm.w16(c.st, a!(SEL_REC), cur);
    draw(c);
    clears(c);
    banner(c);
    install(c, 0xABA2, 0xABA4, 0x2A, 7);
    table(c);
    frame_loop(c);
}

/// The `0x4092` draw head — `[0x1260]`/`[0x125C]` pair, `0x89B3`,
/// `Image(0x57)`, `Present(7)` (file `0x34096`–`0x340B0`).
fn draw(c: &mut Ctx) {
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Native(0x89B3));
    c.host.svc(Call::Image(0x57));
    c.host.svc(Call::Present(7));
}

/// `0x40B1` — the `SEL_MACH`/dirty/`SEQ` clears (file `0x340B1`–
/// `0x340C8`).
fn clears(c: &mut Ctx) {
    c.vm.w16(c.st, a!(SEL_MACH), 0);
    c.vm.w16(c.st, a!(DIRTY0), 0);
    c.vm.w16(c.st, a!(DIRTY1), 0);
    c.vm.w16(c.st, a!(SEQ_PTR), 0);
}

/// `cs:0x4166` — while `SEQ` is idle, arm the `[0x91F4]` orders flag
/// and draw the `(0x1C,6)-(3,0x41)` popup frame with `0x4F98` at
/// `(0x39,0x39)` (file `0x34166`–`0x3418F`).
fn banner(c: &mut Ctx) {
    if c.vm.r16(c.st, a!(SEQ_PTR)) != 0 {
        return;
    }
    c.vm.w8(c.st, 0x91F4, 0xFF);
    c.host.svc(Call::Rect(0x1C, 6, 3, 0x41));
    text::print_str(c, 0x4F98, 0x39, 0x39);
}

/// `0x40E3` — the `0x49DE`/`0x5994` pair, the `0x4190` ship-table
/// build, the header prints, then the stat-bar priming (file
/// `0x340E3`–`0x3411C`).
fn table(c: &mut Ctx) {
    c.host.svc(Call::Native(0x49DE));
    c.host.svc(Call::Native(0x5994));
    c.host.svc(Call::Native(0x4190));
    header(c);
    c.vm.w16(c.st, 0x9C9D, 0xFFFF);
    c.vm.w16(c.st, 0x9C9B, 0xFFFF);
}

/// `0x40EC` — the record `+0xE` name at `(3,0xE)`, `0x4B71`, `0x736C`
/// at `(0x34,0x4E)`, `0x442E`, `0x4AFD`.
fn header(c: &mut Ctx) {
    let rec = c.vm.r16(c.st, a!(SEL_REC));
    text::print_str(c, rec + 0xE, 3, 0xE);
    c.host.svc(Call::Native(0x4B71));
    text::print_str(c, 0x736C, 0x34, 0x4E);
    c.host.svc(Call::Native(0x442E));
    c.host.svc(Call::Native(0x4AFD));
}

/// The `0x411D` frame loop — `{vsync, seq, 0x4974, 0x50A9, 0x5E83,
/// input, menu, tick, inc FRAME}` until `[0x9CDA] & 2`, then the
/// chirp, the `[0xABA2]` stash and `jmp 0x2CC5` (file
/// `0x3411D`–`0x34165`).
fn frame_loop(c: &mut Ctx) {
    loop {
        vsync(c);
        seq::seq_step(c);
        for f in [0x4974, 0x50A9, 0x5E83, 0xA369] {
            c.host.svc(Call::Native(f));
        }
        menu::service(c);
        let _ = tick_step(c);
        bump(c);
        if c.vm.r16(c.st, a!(BUTTONS)) & 2 != 0 {
            break;
        }
    }
    snd(c, 0, 1);
    leave(c, 0xABA2);
}
