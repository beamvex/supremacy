//! The `cs:0xE66E` line editor (file `0x2E66E`–`0x2E77C`).
//!
//! Blocking input loop used for the save/load filename. Each iteration
//! pops the channel-A int-9 LIFO (`0x1E2E`), falls back to the `0xA348`
//! menu/button path (right-click answers `0x1C`, Enter-equivalent), and
//! scans the 62-entry `{code, out}` byte-pair table at `ds:0x7CBD`
//! (installed from the shipped `ds:0x76BF` template; the runtime table
//! overlays the warning-string bytes, which the per-screen `[0x1278]`
//! reloads refresh). `out` classes: `0` exits (`[0x4FCF] = 0xFF`), `1`
//! backspaces, `2` pads the field with spaces and exits, anything else
//! appends while `[0x4FCB] < [0x4FCC]`. `[0x4FCA]` is a key-held
//! debounce — a processed key sets it, a non-matching poll clears it.
//!
//! Idle iterations run `0x2CA9`, and — only when the line-mode cell
//! `[0x9CC5]` is `0` — `0x5C1E`/`0x66F3`/`0x73D9` plus the
//! `[0x91CD] == 0`-gated `[0x1278]` reload and `inc [0x91D4]`. The
//! filename caller brackets the routine with `[0x9CC5] = 1`, so dialog
//! waits are vsync-only (the world pauses during text input).

use super::cells::{IN_DIRTY, IN_DONE, LINE_MODE, NAME_LEN, NAME_MAX};
use super::consts::{BUTTONS, FRAME, PANEL_ON, UIMODE};
use super::vhost::Call;
use super::vm::KEYMAP;
use super::vops::{vsync, Ctx};
use super::{menu, seq, tick_step};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0xE66E` entry — `si`/`bx`/`bp` are the caller's buffer cursor,
/// x cursor, and field width (`0`/`0x2B`/`0x20` from `cs:0x2F2D`).
/// Clears `[0x9CDA]`/`[0x4FCF]`/`[0x4FCA]` and resets the channel-A
/// stack (`cs:[0x8B7C] = -1`).
pub fn line_input(c: &mut Ctx, si: u16, bx: u16, bp: i16) {
    c.vm.w16(c.st, a!(BUTTONS), 0);
    c.vm.w8(c.st, a!(IN_DONE), 0);
    c.vm.w8(c.st, a!(IN_DIRTY), 0);
    c.host.key_flush();
    edit(c, si, bx, bp);
}

/// The `0xE685` poll/dispatch loop.
fn edit(c: &mut Ctx, si: u16, bx: u16, bp: i16) {
    let (mut si, mut bx, mut bp) = (si, bx, bp);
    loop {
        let al = pop(c);
        if al == 0 {
            idle(c);
            continue;
        }
        let Some(out) = lookup(c, al) else {
            c.vm.w8(c.st, a!(IN_DIRTY), 0);
            idle(c);
            continue;
        };
        if c.vm.r8(c.st, a!(IN_DIRTY)) != 0 {
            idle(c);
            continue;
        }
        match out {
            0 => return exit(c),
            2 => return pad(c, si, bp),
            1 => back(c, &mut si, &mut bx, &mut bp),
            ch => append(c, ch, &mut si, &mut bx, &mut bp),
        }
    }
}

/// Poll channel A (`0x1E2E` LIFO pop), then channel B (`0xA348`).
fn pop(c: &mut Ctx) -> u8 {
    let al = u8::try_from(c.host.svc(Call::KeyPop) & 0xFF).unwrap_or(0);
    if al == 0 {
        chan_b(c)
    } else {
        al
    }
}

/// `cs:0xA348` — run the `0xA1CA` menu input path; when it left the
/// buttons word at 2 (right-click) answer `0x1C` (file `0x2A348`).
fn chan_b(c: &mut Ctx) -> u8 {
    menu::keys(c);
    if c.vm.r16(c.st, a!(BUTTONS)) == 2 {
        0x1C
    } else {
        0
    }
}

/// Scan the `ds:0x7CBD` table — `0xE699`: `cmp [di],al; add di,2;
/// loop`. `None` when no entry matches.
fn lookup(c: &mut Ctx, code: u8) -> Option<u8> {
    for i in 0..u16::try_from(super::vm::KEYMAP_N).unwrap_or(0) {
        let e = a!(KEYMAP).wrapping_add(i.wrapping_mul(2));
        if c.vm.r8(c.st, e) == code {
            return Some(c.vm.r8(c.st, e + 1));
        }
    }
    None
}

/// Class 0 — set `[0x4FCF]` and `ret` (file `0x2E6F2`).
fn exit(c: &mut Ctx) {
    c.vm.w8(c.st, a!(IN_DONE), 0xFF);
}

/// Class 1 — `dec si/bx`, blank the char, echo, `dec [0x4FCB]`,
/// `inc bp` (file `0x2E700`–`0x2E72D`).
fn back(c: &mut Ctx, si: &mut u16, bx: &mut u16, bp: &mut i16) {
    if c.vm.r8(c.st, a!(NAME_LEN)) == 0 {
        return;
    }
    *si = si.wrapping_sub(1);
    *bx = bx.wrapping_sub(1);
    c.vm.w8(c.st, a!(IN_DIRTY), 0xFF);
    c.vm.w8(c.st, *si, 0x20);
    c.host.svc(Call::Slot(0x1268, 0x20));
    let len = c.vm.r8(c.st, a!(NAME_LEN));
    c.vm.w8(c.st, a!(NAME_LEN), len.wrapping_sub(1));
    *bp += 1;
}

/// Class ≥3 — append the mapped char while `[0x4FCB] != [0x4FCC]`
/// (file `0x2E730`–`0x2E761`).
fn append(c: &mut Ctx, ch: u8, si: &mut u16, bx: &mut u16, bp: &mut i16) {
    c.vm.w8(c.st, a!(IN_DIRTY), 0xFF);
    if c.vm.r8(c.st, a!(NAME_LEN)) == c.vm.r8(c.st, a!(NAME_MAX)) {
        return;
    }
    c.vm.w8(c.st, *si, ch);
    *si = si.wrapping_add(1);
    c.host.svc(Call::Slot(0x1268, u16::from(ch)));
    *bx = bx.wrapping_add(1);
    let len = c.vm.r8(c.st, a!(NAME_LEN));
    c.vm.w8(c.st, a!(NAME_LEN), len.wrapping_add(1));
    *bp -= 1;
}

/// Class 2 — pad the remaining field with `0x20` then `ret`; the loop
/// exits early on empty or full input (file `0x2E764`–`0x2E77C`).
fn pad(c: &mut Ctx, mut si: u16, mut bp: i16) {
    loop {
        let len = c.vm.r8(c.st, a!(NAME_LEN));
        if len == 0 || len == c.vm.r8(c.st, a!(NAME_MAX)) {
            return;
        }
        c.vm.w8(c.st, si, 0x20);
        si = si.wrapping_add(1);
        bp -= 1;
        if bp < 0 {
            return;
        }
    }
}

/// The `0xE6AE` idle body — `0x2CA9` always; the seq/overlay/tick/
/// reload arm only when `[0x9CC5] == 0`.
fn idle(c: &mut Ctx) {
    vsync(c);
    if c.vm.r8(c.st, a!(LINE_MODE)) != 0 {
        return;
    }
    seq::seq_step(c);
    if c.vm.r8(c.st, a!(PANEL_ON)) != 0 && c.vm.r8(c.st, a!(UIMODE)) == 5 {
        c.host.svc(Call::Slot(0x1266, 0x7D39));
    }
    let _ = tick_step(c);
    if c.vm.r8(c.st, a!(UIMODE)) == 0 {
        c.host.svc(Call::Slot(0x1278, 0));
        let f = c.vm.r8(c.st, a!(FRAME));
        c.vm.w8(c.st, a!(FRAME), f.wrapping_add(1));
    }
}
