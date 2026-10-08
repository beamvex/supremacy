//! Menu-action dispatch — the `jmp [si+0xC]` target table for
//! `menu::service` (file `0x2A16C`).
//!
//! The hotspot record's `+0xC` word is a `cs:` routine the asm jumps
//! to; its `ret` lands back in the menu's caller, so the port invokes
//! the routine inline. Targets decoded so far map to the ported
//! routines; anything else surfaces as [`Call::Native`] for the
//! frontend.

use super::cells::{BLINK_B, BLINK_PH, S_CNF_T};
use super::text;
use super::vhost::Call;
use super::vops::Ctx;
use super::{dialog, dlg_io};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Dispatch one hotspot action by its `cs:` offset.
pub fn dispatch_action(c: &mut Ctx, t: u16) {
    match t {
        0x2E21 => dialog::dlg_enter(c),
        0x2E94 => drop(dlg_io::load_game(c)),
        0x2ED2 => {
            dlg_io::save_game(c);
        }
        0x2EEE => dialog::dlg_list(c),
        0x2F0F => cnf_reprint(c),
        0x2F01 => dlg_io::act_yes(c),
        0x2F08 => dlg_io::act_no(c),
        0x2F0E => {}
        0x2F91 => dlg_io::act_stage(c),
        0x2F95 => dlg_io::act_cancel(c),
        0x2F9B => dialog::confirm(c),
        0x2D86 => blink_reprint(c),
        _ => drop(c.host.svc(Call::Native(t))),
    }
}

/// `cs:0x2F0F` — reinstall the dialog list then fall into `0x2F21`
/// (title reprint) (file `0x32F0F`–`0x32F2C`).
fn cnf_reprint(c: &mut Ctx) {
    dialog::dlg_list(c);
    dialog::dlg_text(c, a!(S_CNF_T));
}

/// `cs:0x2D86` — blink-phase reset + `0x77E3` print at `(0x4D,0x67)`
/// (file `0x32D86`–`0x32D96`).
fn blink_reprint(c: &mut Ctx) {
    c.vm.w8(c.st, a!(BLINK_PH), 0);
    text::print_str(c, a!(BLINK_B), 0x4D, 0x67);
}
