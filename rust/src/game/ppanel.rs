//! The selected-planet viewscreen — `cs:0x5C97` (file
//! `0x35C97`–`0x35D51`), the `uimode == 0` planet panel.
//!
//! Resets the draw sequencer (`[0x91A0]`/`[0x91D7]`/`[0x9198]`/`[0x919A]`),
//! frames the viewscreen via `[0x1262]`, captions it via `0x6AD2`, then
//! arms the selected record's animation stream — the fixed `cs:0x6EC`
//! record for kind 1, else `cs:0x698 + [rec+0x24]·0xE` — copying the
//! record's four param words to `[0x9277]`/`[0x927D]`/`[0x927F]`/`[0x9281]`,
//! refreshing (`[0x1264]`), latching its period byte to `[0x91D6]` and
//! running one seq command immediately (`0x5C49`). Kind 7 prints
//! `ds:0x733A` (`CLASSIFIED!`) at `(0x33,0x2A)` instead — the fallen
//! planet's viewscreen. `uimode != 0` tail-jumps to `0x5D51` (`ret`).

use super::consts::{REC_CUR, SEQ_AUX, SEQ_DELAY, SEQ_PERIOD, SEQ_PHASE, SEQ_PTR};
use super::rec::{F_KIND, F_OWNER};
use super::vhost::Call;
use super::vops::{uimode, vsync, Ctx};
use super::{seq, status, text};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a16 {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:0x9CB6`/`0x9CB8` — the asm's multiply scratch cells.
const MUL_A: u16 = 0x9CB6;
/// Multiply scratch operand (holds the owner index).
const MUL_B: u16 = 0x9CB8;
/// `ds:` offset of the `CLASSIFIED!` string kind 7 prints (file `0x179DA`).
const SECRET: u16 = 0x733A;
/// `cs:` base of the per-owner seq-record table (stride `0xE`).
const SEQ_TAB: u16 = 0x698;
/// `cs:` fixed slot the kind-1 record uses.
const SEQ_FIXED: u16 = 0x6EC;
/// `ds:` cells the four table words land in.
const PARAMS: [u16; 4] = [0x9277, 0x927D, 0x927F, 0x9281];

/// `cs:0x5C97` — reset and rearm the viewscreen animation.
pub fn planet_panel(c: &mut Ctx) {
    if uimode(c) != 0 {
        return;
    }
    clear_seq(c);
    c.host.svc(Call::Rect(0x18, 0x13, 0x3, 0x33));
    status::sel_name(c);
    vsync(c);
    let rec = c.vm.r16(c.st, a16!(REC_CUR));
    if c.vm.r16(c.st, rec + a16!(F_KIND)) == 7 {
        text::print_str(c, SECRET, 0x33, 0x2A);
        return;
    }
    arm(c, rec);
}

/// The four-cell sequencer reset (file `0x35CA1`–`0x35CB7`).
fn clear_seq(c: &mut Ctx) {
    c.vm.w16(c.st, a16!(SEQ_DELAY), 0);
    c.vm.w8(c.st, a16!(SEQ_PHASE), 0);
    c.vm.w16(c.st, a16!(SEQ_PTR), 0);
    c.vm.w16(c.st, a16!(SEQ_AUX), 0);
}

/// The `cs:` table slot — `0x6EC` for kind 1, else `0x698 + owner·0xE`
/// (file `0x35CE4`–`0x35D0D`; the mul cells are written for fidelity).
fn seq_slot(c: &mut Ctx, rec: u16) -> u16 {
    if c.vm.r16(c.st, rec + a16!(F_KIND)) == 1 {
        return SEQ_FIXED;
    }
    let owner = u16::from(c.vm.r8(c.st, rec + a16!(F_OWNER)));
    c.vm.w16(c.st, MUL_A, 0xE);
    c.vm.w16(c.st, MUL_B, owner);
    SEQ_TAB.wrapping_add(owner.wrapping_mul(0xE))
}

/// Table load, refresh, period byte and the immediate `0x5C49` command
/// (file `0x35D10`–`0x35D4E`; the `bx = 0xFC0` spin is a hardware delay
/// the port's frame pacing supersedes).
fn arm(c: &mut Ctx, rec: u16) {
    let si = seq_slot(c, rec);
    let ptr = c.host.cs_word(si);
    c.vm.w16(c.st, a16!(SEQ_PTR), ptr);
    for (k, cell) in PARAMS.iter().enumerate() {
        let ofs = si.wrapping_add(4 + u16::try_from(k).unwrap_or(0) * 2);
        let v = c.host.cs_word(ofs);
        c.vm.w16(c.st, *cell, v);
    }
    c.host.svc(Call::Refresh);
    let per = c.host.cs_word(si.wrapping_add(0xC));
    c.vm.w8(
        c.st,
        a16!(SEQ_PERIOD),
        u8::try_from(per & 0xFF).unwrap_or(0),
    );
    seq::seq_cmd(c);
}
