//! Menu-action dispatch — the `jmp [si+0xC]` target table for
//! `menu::service` (file `0x2A16C`).
//!
//! The hotspot record's `+0xC` word is a `cs:` routine the asm jumps
//! to; its `ret` lands back in the menu's caller, so the port invokes
//! the routine inline. The shipped records carry `0x286B`-space
//! offsets while the runtime window uses the same routines `0x6D50`
//! lower — every arm maps both representations. Anything else
//! surfaces as [`Call::Native`] for the frontend.

use super::cells::{BLINK_B, BLINK_PH, S_CNF_T};
use super::text;
use super::vops::Ctx;
use super::{dialog, dlg_io, init, screens};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Dispatch one hotspot action by its `cs:` offset. The second value
/// in each pair is the `+0x6D50` `0x286B`-window alias the shipped
/// records carry.
pub fn dispatch_action(c: &mut Ctx, t: u16) {
    match t {
        0x2E21 | 0x9B71 => dialog::dlg_enter(c),
        0x2E94 | 0x9BE4 => drop(dlg_io::load_game(c)),
        0x2ED2 | 0x9C22 => {
            dlg_io::save_game(c);
        }
        0x2EEE | 0x9C3E => dialog::dlg_list(c),
        0x2F0F | 0x9C5F => cnf_reprint(c),
        0x2F01 | 0x9C51 => dlg_io::act_yes(c),
        0x2F08 | 0x9C58 => dlg_io::act_no(c),
        0x2F0E | 0x9C5E => {}
        0x2F91 | 0x9CE1 => dlg_io::act_stage(c),
        0x2F95 | 0x9CE5 => dlg_io::act_cancel(c),
        0x2F9B | 0x9CEB => dialog::confirm(c),
        0x2D86 | 0x9AD6 => blink_reprint(c),
        0x305B | 0x9DAB => init::act_newgame(c),
        _ => screens::dispatch(c, t),
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
