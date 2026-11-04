//! The timed draw sequencer — `cs:0x5C1E` (file `0x35C1E`–`0x35C96`),
//! called once per frame from the main loop. GAM-80: the `cs:` record
//! stream is compiled into [`seqtab`][super::seqtab] — each cursor
//! maps to a typed [`SeqOp`] instead of fetching record words.
//!
//! Drives a per-machine-type animation: `[0x91A0]` is a delay countdown
//! that runs out before anything else; `[0x91D6]` is the period and
//! `[0x91D7]` the phase — commands only fire when the phase wraps to 0.
//! `[0x9198]` holds the stream cursor. Ops: `End` clears the cursor,
//! `Delay` reloads the countdown, `Go` continues at another cursor,
//! `Img` draws an image index via `call word [0x125A]`.
//!
//! One command (or delay reload) per period boundary; the cursor is
//! committed to `[0x9198]` before suspending.

use super::consts::{SEQ_AUX, SEQ_DELAY, SEQ_PERIOD, SEQ_PHASE, SEQ_PTR};
use super::seqtab::{self, SeqOp};
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
    seq_cmd(c);
}

/// `cs:0x5C49` — the mid-entry `0x5C97` and the `0x60xx` screens
/// tail-call: run one command immediately, skipping the `[0x91A0]`
/// delay and `[0x91D6]`/`[0x91D7]` period gates.
pub fn seq_cmd(c: &mut Ctx) {
    let si = c.vm.r16(c.st, a16!(SEQ_PTR));
    if si == 0 {
        return;
    }
    run_cmd(c, si);
}

/// `inc [0x91D7]` against the period; wraps to 0 and reports whether the
/// command may run this frame (`bl == 0` after the wrap).
fn advance_phase(c: &mut Ctx, period: u8) -> bool {
    let mut phase = c.vm.r8(c.st, a16!(SEQ_PHASE)).wrapping_add(1);
    if phase >= period {
        phase = 0;
    }
    c.vm.w8(c.st, a16!(SEQ_PHASE), phase);
    phase == 0
}

/// The `cs:0x5C52` step — one command per call unless a `Go` chains
/// the walk onward. Off-stream cursors end like the `0xFFFF` record
/// (a stale pointer is a dead stream either way); the `Go` walk stays
/// bounded like the asm's link chain.
fn run_cmd(c: &mut Ctx, mut si: u16) {
    for _ in 0..0x100 {
        let Some((op, next)) = seqtab::at(si) else {
            return end(c);
        };
        match op {
            SeqOp::End => return end(c),
            SeqOp::Delay(d) => return delay(c, next, d),
            SeqOp::Go(t) => si = t,
            SeqOp::Img(img) => return draw(c, next, img),
        }
    }
}

/// `0xFFFF` — `cs:0x5C8A`: clear the cursor and the aux pointer.
fn end(c: &mut Ctx) {
    c.vm.w16(c.st, a16!(SEQ_PTR), 0);
    c.vm.w16(c.st, a16!(SEQ_AUX), 0);
}

/// `0x01F4` — commit the post-record cursor and reload `[0x91A0]`.
fn delay(c: &mut Ctx, si: u16, d: u16) {
    c.vm.w16(c.st, a16!(SEQ_DELAY), d);
    c.vm.w16(c.st, a16!(SEQ_PTR), si);
}

/// Image index — commit the cursor, then `call [0x125A]`.
fn draw(c: &mut Ctx, si: u16, img: u16) {
    c.vm.w16(c.st, a16!(SEQ_PTR), si);
    c.host.svc(Call::Image(img));
}
