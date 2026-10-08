//! The text layer — `cs:0x6709` string-print interpreter (file
//! `0x36709`–`0x36791`) and the `0x6B06` ticker enqueue (file
//! `0x36B06`–`0x36B49`).
//!
//! Interpreter operand bytes: `0xFF` ends the string; `≥ 0x20` draws a
//! glyph via `[0x1268]` (`Call::Glyph`) at `(bx·4px, dx)` then `bx += 1`;
//! `0x01`/`0x02` add the next byte (sign-extended) to `dx`/`bx`; `0x03`
//! reads a `{count, char}` word and repeats the char `count` times
//! (`bx += 1` each); `0x04` is the vertical variant (`dx += 6`). Other
//! control bytes end the print like `0xFF`.
//!
//! When `[0x91EA]` is nonzero each direct char also runs the typewriter
//! pace block — `0x5C1E`/`0x73D9`/`0x2CA9`/`0x66F3` once (file
//! `0x3672A`–`0x36744`).

use super::cells::{OVERLAY_F, TICK_LEFT, TICK_PUT};
use super::consts::{PANEL_ON, SND_ON, UIMODE};
use super::vhost::Call;
use super::vops::{vsync, Ctx};
use super::{seq, tick_step};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:` base of the ticker record stream the `0xFE` sentinel wraps to
/// (file `0x36B39`).
const TICK_BASE: u16 = 0x928D;

/// `cs:0x6709` — print the string at `ds:si`, cursor `(bx, dx)`.
pub fn print_str(c: &mut Ctx, mut si: u16, mut bx: u16, mut dx: u16) {
    loop {
        let al = c.vm.r8(c.st, si);
        si = si.wrapping_add(1);
        if al == 0xFF || (al < 0x20 && !esc(c, al, &mut si, &mut bx, &mut dx)) {
            return;
        }
        if al >= 0x20 {
            put(c, al, bx, dx);
            pace(c);
            bx = bx.wrapping_add(1);
        }
    }
}

/// The `0x01`/`0x02`/`0x03`/`0x04` escapes — returns `false` for the
/// control bytes that terminate the print (file `0x36712`–`0x36791`).
fn esc(c: &mut Ctx, al: u8, si: &mut u16, bx: &mut u16, dx: &mut u16) -> bool {
    match al {
        0x01 | 0x02 => {
            let n = i8::from_ne_bytes([c.vm.r8(c.st, *si)]);
            *si = si.wrapping_add(1);
            let cell = if al == 0x01 { dx } else { bx };
            *cell = cell.wrapping_add(u16::from_ne_bytes(i16::from(n).to_ne_bytes()));
            true
        }
        0x03 | 0x04 => repeat(c, c.vm.r16(c.st, *si), bx, dx, al == 0x04, si),
        _ => false,
    }
}

/// `0x03`/`0x04` — `{count, char}` word; draw `count` times (`≥ 1`),
/// `bx += 1` across or `dx += 6` down (file `0x3675B`/`0x36775`).
fn repeat(c: &mut Ctx, w: u16, bx: &mut u16, dx: &mut u16, vert: bool, si: &mut u16) -> bool {
    *si = si.wrapping_add(2);
    let n = u8::try_from(w & 0xFF).unwrap_or(0).max(1);
    let ch = u8::try_from(w >> 8).unwrap_or(0);
    for _ in 0..n {
        put(c, ch, *bx, *dx);
        if vert {
            *dx = dx.wrapping_add(6);
        } else {
            *bx = bx.wrapping_add(1);
        }
    }
    true
}

/// `0x68D6` — `[0x1268]` glyph put at `(bx, dx)`; shared by `0x6709`
/// and the number printers.
pub fn put(c: &mut Ctx, ch: u8, bx: u16, dx: u16) {
    c.host.svc(Call::Glyph(ch, bx, dx));
}

/// The `[0x91EA]`-gated typewriter pace — `0x5C1E` seq, `0x73D9` tick,
/// `0x2CA9` vsync, `0x66F3` overlay (file `0x3672A`–`0x36744`).
fn pace(c: &mut Ctx) {
    if c.vm.r8(c.st, a!(OVERLAY_F)) == 0 {
        return;
    }
    seq::seq_step(c);
    let _ = tick_step(c);
    vsync(c);
    if c.vm.r8(c.st, a!(PANEL_ON)) != 0 && c.vm.r8(c.st, a!(UIMODE)) == 5 {
        c.host.svc(Call::Slot(0x1266, 0x7D39));
    }
}

/// `cs:0x6B06` — enqueue the `ds:si` message into the status ticker:
/// the `0x8A5B`/`0x8A3B`/`0x127C` sound select (suppressed when
/// `[0x91D3] == 0xFF`), `[0x91E8] = 0xFF`, then append `{si, 0}` at the
/// `[0x9160]` write cursor with a `0xFE` wrap to `0x928D` and
/// `[0x91C2] += 1` (skipped at `0x80`).
pub fn enqueue(c: &mut Ctx, si: u16) {
    if c.vm.r8(c.st, a!(SND_ON)) != 0xFF {
        c.host.svc(Call::Chan(0x8A5B, 0));
        c.host.svc(Call::Chan(0x8A3B, 0x1F));
        c.host.svc(Call::Sound(0xC));
    }
    c.vm.w8(c.st, a!(super::cells::TICK_FLAG), 0xFF);
    if c.vm.r16(c.st, a!(TICK_LEFT)) == 0x80 {
        return;
    }
    let mut di = c.vm.r16(c.st, a!(TICK_PUT));
    if c.vm.r16(c.st, di) == 0xFE {
        di = TICK_BASE;
    }
    c.vm.w16(c.st, di, si);
    c.vm.w16(c.st, a!(TICK_PUT), di.wrapping_add(4));
    let n = c.vm.r16(c.st, a!(TICK_LEFT)).wrapping_add(1);
    c.vm.w16(c.st, a!(TICK_LEFT), n);
}
