//! The text layer — the `cs:0x6709` string printer (file
//! `0x36709`–`0x36791`), now compiled, and the `0x6B06` ticker enqueue
//! (file `0x36B06`–`0x36B49`).
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
use super::texttab;
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
/// Returns `bx` advanced past the emitted chars — the sidebar and
/// machine-list sites chain the next print off it.
///
/// GAM-80: strings containing `0x01..0x04` control bytes are compiled
/// to [`texttab`] draw lists and replayed; everything else is a plain
/// glyph run (`≥ 0x20` → `Call::Glyph`, `< 0x20`/`0xFF` ends).
pub fn print_str(c: &mut Ctx, si: u16, bx: u16, dx: u16) -> u16 {
    if let Some(s) = texttab::at(si) {
        let ok = (0..4).all(|k| c.vm.r8(c.st, si + k) == s.sig[usize::from(k)]);
        if ok {
            return replay(c, bx, dx, s);
        }
    }
    plain(c, si, bx, dx)
}

/// A compiled draw list — each [`Draw`][texttab::Draw] carries its
/// cursor deltas and the `pace` gate; `s.bx` is the net `bx` return.
fn replay(c: &mut Ctx, bx: u16, dx: u16, s: &texttab::Dec) -> u16 {
    for d in s.draws {
        put(
            c,
            d.ch,
            bx.wrapping_add(d.bx.cast_unsigned()),
            dx.wrapping_add(d.dx.cast_unsigned()),
        );
        if d.pace {
            pace(c);
        }
    }
    bx.wrapping_add(s.bx.cast_unsigned())
}

/// A plain run — glyphs `≥ 0x20` at `bx += 1`; ends on `0xFF` or any
/// control byte (dynamic strings never carry `0x01..0x04`).
fn plain(c: &mut Ctx, mut si: u16, mut bx: u16, dx: u16) -> u16 {
    loop {
        let al = c.vm.r8(c.st, si);
        si = si.wrapping_add(1);
        if al == 0xFF || al < 0x20 {
            return bx;
        }
        put(c, al, bx, dx);
        pace(c);
        bx = bx.wrapping_add(1);
    }
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
