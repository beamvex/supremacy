//! The timed draw sequencer — `cs:0x5C1E` (file `0x35C1E`–`0x35C96`),
//! called once per frame from the main loop.
//!
//! Drives a per-machine-type animation: `[0x91A0]` is a delay countdown
//! that runs out before anything else; `[0x91D6]` is the period and
//! `[0x91D7]` the phase — commands only fire when the phase wraps to 0.
//! `[0x9198]` points into a `cs:`-resident command stream of 4-byte
//! `{op, pad}` records (the pad word is never read):
//!
//! - `0xFFFF` — end: `[0x9198]`/`[0x919A]` cleared (`cs:0x5C8A`).
//! - `0x01F4` — delay: the *next* record's op word becomes `[0x91A0]`
//!   and the stream advances past both records.
//! - `0x0000` — link: `si = cs:[si]` (the next record's op word is a
//!   `cs:` pointer); fetching resumes there without suspending.
//! - anything else — an image index drawn via `call word [0x125A]`.
//!
//! One command (or delay reload) per period boundary; the cursor is
//! committed to `[0x9198]` before suspending.

use super::consts::{SEQ_AUX, SEQ_DELAY, SEQ_PERIOD, SEQ_PHASE, SEQ_PTR};
use super::vhost::Call;
use super::vops::Ctx;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a16 {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// One sequencer step — the `cs:0x5C1E` body. Runs at most one command.
pub fn seq_step(c: &mut Ctx) {
    let delay = c.vm.r16(c.st, a16!(SEQ_DELAY));
    if delay != 0 {
        c.vm.w16(c.st, a16!(SEQ_DELAY), delay - 1);
        return;
    }
    let period = c.vm.r8(c.st, a16!(SEQ_PERIOD));
    if period == 0xFF {
        return;
    }
    if !advance_phase(c, period) {
        return;
    }
    let si = c.vm.r16(c.st, a16!(SEQ_PTR));
    if si == 0 {
        return;
    }
    run_cmd(c, si);
}

/// `inc [0x91D7]` against the period; wraps to 0 and reports whether the
/// command fetch may run this frame (`bl == 0` after the wrap, file
/// `0x35C32`–`0x35C47`).
fn advance_phase(c: &mut Ctx, period: u8) -> bool {
    let mut phase = c.vm.r8(c.st, a16!(SEQ_PHASE)).wrapping_add(1);
    if phase >= period {
        phase = 0;
    }
    c.vm.w8(c.st, a16!(SEQ_PHASE), phase);
    phase == 0
}

/// The `cs:0x5C52` fetch/dispatch loop — one command per call unless a
/// link (`0`) chains the fetch onward.
fn run_cmd(c: &mut Ctx, mut si: u16) {
    loop {
        let op = c.host.cs_word(si);
        si = si.wrapping_add(4);
        match op {
            0xFFFF => return end(c),
            0x01F4 => return delay(c, si),
            0 => si = c.host.cs_word(si),
            img => return draw(c, si, img),
        }
    }
}

/// `0xFFFF` — `cs:0x5C8A`: clear the cursor and the aux pointer.
fn end(c: &mut Ctx) {
    c.vm.w16(c.st, a16!(SEQ_PTR), 0);
    c.vm.w16(c.st, a16!(SEQ_AUX), 0);
}

/// `0x01F4` — the next record's op word is the delay; both records are
/// consumed (file `0x35C63`–`0x35C71`).
fn delay(c: &mut Ctx, si: u16) {
    let d = c.host.cs_word(si);
    c.vm.w16(c.st, a16!(SEQ_DELAY), d);
    c.vm.w16(c.st, a16!(SEQ_PTR), si.wrapping_add(4));
}

/// Image index — commit the cursor, then `call [0x125A]` (file
/// `0x35C7D`–`0x35C88`).
fn draw(c: &mut Ctx, si: u16, img: u16) {
    c.vm.w16(c.st, a16!(SEQ_PTR), si);
    c.host.svc(Call::Image(img));
}
