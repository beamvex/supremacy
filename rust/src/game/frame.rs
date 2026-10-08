//! The main loop body — `cs:0x395B` (file `0x3395B`–`0x33A44`), the
//! per-frame pipeline the game runs while the galaxy screen is active.
//! One call of [`frame_step`] is one iteration; the loop head is the
//! caller's job (the asm `jmp 0x395B`s until `[0x9CDA] & 2` exits to
//! the `cs:0x2CC5` UI shell).
//!
//! Order per frame: vsync wait → conditional `uimode-5` overlay →
//! `[0x91EE]` refresh block (present, mouse clamp, type select,
//! dispatch 5) → `[0x91EF]` info panel → ambient sound keyed on the
//! selected type's `+0x12` byte → frame counter → `tick_step` → the
//! `0x5C1E` draw sequencer → input phase → menu service → one deferred
//! dirty-flag redraw → exit edge test.

use super::consts::{
    BUTTONS, DIRTY0, DIRTY1, FRAME, HOT_SEL, PANEL_ON, PANEL_REQ, REDRAW, SEL_SAVE, SNAP0, SNAP1,
    SND_ON, TYPE_REC, UIMODE,
};
use super::vhost::Call;
use super::vops::{vsync, Ctx};
use super::{menu, panel, select, seq, tick_step, Tick};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// What a [`frame_step`] did — the loop's two exits.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Frame {
    /// Iteration completed; payload is the `tick_step` arm that ran.
    Run(Tick),
    /// `[0x9CDA] & 2` edge — the asm `jmp 0x2CC5`s back to the UI shell
    /// after stashing the selection and restoring the mouse range.
    Shell,
}

/// One frame — `cs:0x395B` through the `test [0x9CDA],2` exit check.
pub fn frame_step(c: &mut Ctx) -> Frame {
    vsync(c);
    cond_overlay(c);
    if c.vm.r8(c.st, a!(REDRAW)) != 0 {
        refresh(c);
    }
    if c.vm.r8(c.st, a!(PANEL_REQ)) != 0 {
        panel::info_panel(c);
    }
    ambient(c);
    let f = c.vm.r8(c.st, a!(FRAME)).wrapping_add(1);
    c.vm.w8(c.st, a!(FRAME), f);
    let t = tick_step(c);
    seq::seq_step(c);
    c.host.svc(Call::Native(0xA369));
    menu::service(c);
    dirty_flags(c);
    if c.vm.r16(c.st, a!(BUTTONS)) & 2 != 0 {
        exit(c);
        return Frame::Shell;
    }
    Frame::Run(t)
}

/// `cs:0x66F3` — `di = 0x7D39; call [0x1266]` when `[0x91DB]` is set and
/// `uimode == 5` (file `0x366F3`–`0x36708`).
fn cond_overlay(c: &mut Ctx) {
    if c.vm.r8(c.st, a!(PANEL_ON)) != 0 && c.vm.r8(c.st, a!(UIMODE)) == 5 {
        c.host.svc(Call::Slot(0x1266, 0x7D39));
    }
}

/// The `[0x91EE]` refresh block (file `0x33968`–`0x33997`): two template
/// slots, the `0x90..0xBC` mouse-y clamp, the type select, dispatch 5.
fn refresh(c: &mut Ctx) {
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Mouse(8, 0x90, 0xBC));
    select::select_type(c);
    c.host.svc(Call::Present(5));
}

/// The ambient-sound select (file `0x339A1`–`0x33A04`) — the selected
/// type's `+0x12` byte picks a `(ax, cl)` pair and period; `0x8577`
/// sends `cl + 1` to the driver when `[0x91D3] != 0xFF`.
fn ambient(c: &mut Ctx) {
    let bl = c.vm.r8(c.st, a!(FRAME));
    let t = c.vm.r16(c.st, a!(TYPE_REC));
    let kind = c.vm.r8(c.st, t + 0x12);
    let hit = match kind {
        7 if bl.trailing_zeros() >= 5 => Some((0x26, 0x0E)),
        3 if bl.trailing_zeros() >= 6 => Some((0x29, 0x0D)),
        4 if bl.trailing_zeros() >= 6 => Some((0x0C, 0x0C)),
        6 => ambient6(bl),
        _ => None,
    };
    if let Some((ax, cl)) = hit {
        snd(c, ax, cl);
    }
}

/// Type 6 alternates two commands — `0x2A/0x0A` on frame 0, `0x21/0x13`
/// on frame `0x80` (file `0x339DF`–`0x339F9`).
fn ambient6(bl: u8) -> Option<(u16, u8)> {
    match bl {
        0 => Some((0x2A, 0x0A)),
        0x80 => Some((0x21, 0x13)),
        _ => None,
    }
}

/// The `cs:0x8577` wrapper — skipped when `[0x91D3] == 0xFF` (muted).
fn snd(c: &mut Ctx, ax: u16, cl: u8) {
    if c.vm.r8(c.st, a!(SND_ON)) == 0xFF {
        return;
    }
    c.host.svc(Call::Sfx(ax, cl.wrapping_add(1)));
}

/// `cs:0x3C6D` — snapshot the dirty words, service at most one bit per
/// frame (each arm `ret`s in the asm), clear it in the live word.
fn dirty_flags(c: &mut Ctx) {
    let d0 = c.vm.r16(c.st, a!(DIRTY0));
    let d1 = c.vm.r16(c.st, a!(DIRTY1));
    c.vm.w16(c.st, a!(SNAP0), d0);
    c.vm.w16(c.st, a!(SNAP1), d1);
    if d0 & 4 != 0 {
        serve(c, 0x3CAD, DIRTY0, !4);
    } else if d0 & 0x8000 != 0 {
        serve(c, 0x3CDB, DIRTY0, !0x8000);
    } else if d1 & 2 != 0 {
        serve(c, 0x3CF6, DIRTY1, !2);
    }
}

/// One deferred-redraw arm: host call then `and` the bit off.
fn serve(c: &mut Ctx, f: u16, cell: usize, mask: u16) {
    c.host.svc(Call::Native(f));
    let a = u16::try_from(cell).unwrap_or(0);
    c.vm.w16(c.st, a, c.vm.r16(c.st, a) & mask);
}

/// The exit path (file `0x33A1C`–`0x33A44`): clear `[0x91EA]`, restore
/// the full `0..0xBC` mouse range, stash the selection at `[0xA81A]`.
fn exit(c: &mut Ctx) {
    c.vm.w8(c.st, 0x91EA, 0);
    c.host.svc(Call::Mouse(8, 0, 0xBC));
    let sel = c.vm.r16(c.st, a!(HOT_SEL));
    c.vm.w16(c.st, a!(SEL_SAVE), sel);
}
