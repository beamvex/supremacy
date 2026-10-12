//! The galaxy-screen message tail — the `0x6491` banner/overlay
//! print path (file `0x36491`–`0x364B7`) and the `cs:0x6F2D` wait
//! (file `0x36F2D`–`0x36F52`) it runs before `jmp 0x3AE9`
//! ([`panel::info_panel`]). Shared by `gal_build`'s refuse, cancel
//! and commit reports.

use super::super::cells::OVERLAY_F;
use super::super::consts::PANEL_ON;
use super::super::vhost::Call;
use super::super::vops::{uimode, vsync, Ctx};
use super::super::{panel, seq, text};
use super::bump;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// The `0x68D8` print-row banner `msg` draws first.
const S_BANNER: u16 = 0x68D8;
/// The shared message-row y coordinate.
const MSG_Y: u16 = 0x92;
/// The `0x6F2D` iteration count (`bx = 0x2C`).
const WAIT_N: u16 = 0x2C;

/// `0x6491` — the `0x68D8` banner print then the overlay-gated
/// message at `(0x12, 0x92)` + the wait + panel reprint.
pub(super) fn msg(c: &mut Ctx, s: u16) {
    text::print_str(c, S_BANNER, 0x12, MSG_Y);
    overlay(c, s, 0x12);
}

/// The `0x649F`/`0x64AA` tail — `[0x91EA]`-armed print at
/// `(bx, 0x92)`, the `0x6F2D` wait, `jmp 0x3AE9`. `bx = 0x39` is the
/// name-edit cancel's print column.
pub(super) fn overlay(c: &mut Ctx, s: u16, bx: u16) {
    c.vm.w8(c.st, a!(OVERLAY_F), 0xFF);
    text::print_str(c, s, bx, MSG_Y);
    c.vm.w8(c.st, a!(OVERLAY_F), 0);
    wait(c);
    panel::info_panel(c);
}

/// `cs:0x6F2D` — `0x2C` iterations of the `0x6F39` frame step:
/// `0x2CA9` vsync, the `0x66F3` `[0x91DB]`/`uimode-5` overlay slot,
/// `uimode == 6` → `0xEA2E`, `inc [0x91D4]`, `0xA369` input, `0x5C1E`
/// sequencer (file `0x36F39`–`0x36F52`).
fn wait(c: &mut Ctx) {
    for _ in 0..WAIT_N {
        vsync(c);
        if c.vm.r8(c.st, a!(PANEL_ON)) != 0 && uimode(c) == 5 {
            c.host.svc(Call::Slot(0x1266, 0x7D39));
        }
        if uimode(c) == 6 {
            c.host.svc(Call::Native(0xEA2E));
        }
        bump(c);
        c.host.svc(Call::Native(0xA369));
        seq::seq_step(c);
    }
}
