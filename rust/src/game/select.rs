//! The machine-type select — `cs:0x3A47` (file `0x33A47`–`0x33AE8`),
//! run inside the main loop's redraw block when `[0x91EE]` is set.
//!
//! Resets the draw sequencer, indexes the machine-type table
//! `ds:0x9B12 + [0x91ED]·0x30` into `[0x9154]`, copies the record's
//! sequence pointer (`+0x18`) and period (`+0x1C`) live, draws the
//! type's image plus the panel frame, and prints the two difficulty
//! legend strings.

use super::consts::{
    DIFFICULTY, PANEL_ON, REDRAW, SEQ_DELAY, SEQ_PERIOD, SEQ_PHASE, SEQ_PTR, TYPE_BASE, TYPE_REC,
    TYPE_SEL, TYPE_STRIDE,
};
use super::vhost::Call;
use super::vops::Ctx;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a16 {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `ds:0x9CB6`/`0x9CB8` — the asm's multiply scratch cells.
const MUL_A: u16 = 0x9CB6;
/// Multiply scratch operand (holds the type index).
const MUL_B: u16 = 0x9CB8;
/// The panel-frame image index (`dx = 0x56`, file `0x33AB6`).
const FRAME_IMG: u16 = 0x56;

/// The `cs:0x3A47` body — select type `[0x91ED]` and redraw the panel.
pub fn select_type(c: &mut Ctx) {
    c.vm.w16(c.st, a16!(SEQ_DELAY), 0);
    c.vm.w8(c.st, a16!(SEQ_PHASE), 0);
    c.vm.w8(c.st, a16!(REDRAW), 0);
    let idx = u16::from(c.vm.r8(c.st, a16!(TYPE_SEL)));
    c.vm.w16(c.st, MUL_A, a16!(TYPE_STRIDE));
    c.vm.w16(c.st, MUL_B, idx);
    let si = a16!(TYPE_BASE).wrapping_add(idx.wrapping_mul(a16!(TYPE_STRIDE)));
    c.vm.w16(c.st, a16!(TYPE_REC), si);
    copy_seq(c, si);
    draw_panel(c, si);
    legend(c);
}

/// Record-field copies and the type image (file `0x33A82`–`0x33A96`).
fn copy_seq(c: &mut Ctx, si: u16) {
    let v = c.vm.r16(c.st, si + 6);
    c.vm.w16(c.st, 0x91BE, v);
    let p = c.vm.r16(c.st, si + 0x18);
    c.vm.w16(c.st, a16!(SEQ_PTR), p);
    let per = c.vm.r8(c.st, si + 0x1C);
    c.vm.w8(c.st, a16!(SEQ_PERIOD), per);
    let img = c.vm.r16(c.st, si);
    c.host.svc(Call::Image(img));
}

/// The `[si+0x1D]` flag, the `0x66DE`/`0x89C3` helpers, frame image and
/// the `(0x12,0x98)` caption (file `0x33A9A`–`0x33AC6`).
fn draw_panel(c: &mut Ctx, si: u16) {
    let on = c.vm.r8(c.st, si + 0x1D);
    c.vm.w8(c.st, a16!(PANEL_ON), if on == 0 { 0 } else { 0xFF });
    if on != 0 {
        c.host.svc(Call::Native(0x66DE));
    }
    c.host.svc(Call::Native(0x89C3));
    c.host.svc(Call::Image(FRAME_IMG));
    c.host.svc(Call::Text(0x6C43, 0x12, 0x98));
}

/// The difficulty-indexed legend at `(0,0xB0)` — `0x6C15`/`0x6BBC`/
/// `0x6B38` for difficulties 0/1/2 (file `0x33AC9`–`0x33AE6`).
fn legend(c: &mut Ctx) {
    let diff = c.vm.r8(c.st, a16!(DIFFICULTY));
    let s = match diff {
        0 => 0x6C15,
        1 => 0x6BBC,
        _ => 0x6B38,
    };
    c.host.svc(Call::Text(s, 0, 0xB0));
}
