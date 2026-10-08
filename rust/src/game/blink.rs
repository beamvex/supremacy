//! The blink timer — `cs:0x6B4A` (file `0x36B4A`–`0x36B8E`), called
//! once per shell frame. `[0x91D1]` is the countdown: zero means idle
//! and the routine tail-falls into the `0x6B8F` ticker. While armed,
//! `[0x91F2]` holds the phase machinery off (just `dec` + the
//! `[0x1270]` slot); otherwise `[0x91F3]` counts up to `0x11`, at
//! which point the routine stops decrementing and alternates the
//! `0x77AA`/`0x77E3` strings at `(0x4D,0x67)` on frame phases 0 and 5
//! of 8 — the flashing "press" prompt.

use super::cells::{BLINK_A, BLINK_B, BLINK_HOLD, BLINK_PH, BLINK_T};
use super::consts::FRAME;
use super::ticker::ticker_step;
use super::vhost::Call;
use super::vops::Ctx;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Sub-phase at which the flash takes over (file `0x36B58`).
const FLASH: u8 = 0x11;

/// `cs:0x6B4A` — one timer step (file `0x36B4A`–`0x36B8E`).
pub fn blink_step(c: &mut Ctx) {
    if c.vm.r8(c.st, a!(BLINK_T)) == 0 {
        ticker_step(c);
        return;
    }
    if c.vm.r8(c.st, a!(BLINK_HOLD)) == 0 && c.vm.r8(c.st, a!(BLINK_PH)) == FLASH {
        flash(c);
        return;
    }
    if c.vm.r8(c.st, a!(BLINK_HOLD)) == 0 {
        let ph = c.vm.r8(c.st, a!(BLINK_PH)).wrapping_add(1);
        c.vm.w8(c.st, a!(BLINK_PH), ph);
    }
    let t = c.vm.r8(c.st, a!(BLINK_T)).wrapping_sub(1);
    c.vm.w8(c.st, a!(BLINK_T), t);
    c.host.svc(Call::Slot(0x1270, 0));
}

/// The `0x91F3 == 0x11` arm — print `0x77E3` on phase 5, `0x77AA` on
/// phase 0, nothing otherwise (file `0x36B5F`–`0x36B81`).
fn flash(c: &mut Ctx) {
    let s = match c.vm.r8(c.st, a!(FRAME)) & 7 {
        5 => a!(BLINK_B),
        0 => a!(BLINK_A),
        _ => return,
    };
    c.host.svc(Call::Text(s, 0x4D, 0x67));
}
