//! Input pump — the int-9/int-33h halves of the frontend: tracks the
//! host-side cursor/button/key state and mirrors it into the `ds:` cells
//! the ported menu service polls (`[0x9CD6]`/`[0x9CD8]` cursor,
//! `[0x9CCB]`/`[0x9CCC]` latch, `[0x9CCD]` K-mode flag).
//!
//! Button edges are pushed as the same synthetic codes the DOS driver's
//! `0x2A18E` handler emits (`0x7E`/`0x7D` down, `0xFE`/`0xFD` up) so the
//! `cs:0xA18E` `menu::service` synth path latches `[0x9CDA]` itself —
//! exactly the asm's data flow.

use std::collections::VecDeque;

use crate::game::{State, Vm, CUR_X, CUR_Y, KEY_CODE, KEY_PEND, K_MODE};

/// int-33h scancode equivalents (`ds:` latch values).
pub mod code {
    /// Left-button make (`0xA18E` → `[0x9CDA] = 1`).
    pub const LEFT_DOWN: u8 = 0x7E;
    /// Right-button make (`→ 2`).
    pub const RIGHT_DOWN: u8 = 0x7D;
    /// Left-button break (`→ 0`).
    pub const LEFT_UP: u8 = 0xFE;
    /// Right-button break (`→ 0`).
    pub const RIGHT_UP: u8 = 0xFD;
}

/// Cursor/button/key state the window feeds once per frame.
pub struct Pump {
    /// Cursor column in pixels → `[0x9CD6]`.
    pub x: u16,
    /// Cursor row in pixels → `[0x9CD8]`.
    pub y: u16,
    /// int-33h vertical clamp set by `ax = 8` calls (`0x2A2FD` region).
    pub y_range: (u16, u16),
    /// `K`-mode flag → `[0x9CCD]` (keyboard-emulated mouse).
    pub k_mode: bool,
    buttons: u8,
    queue: VecDeque<u8>,
}

impl Pump {
    /// Fresh pump — full `0..0xBC` vertical range like int-33h reset.
    #[must_use]
    pub fn new(k_mode: bool) -> Self {
        Self {
            x: 0xA0,
            y: 0x64,
            y_range: (0, 0xBC),
            k_mode,
            buttons: 0,
            queue: VecDeque::new(),
        }
    }

    /// int-33h `ax = 8` — set the vertical cursor range (`cx`/`dx`).
    pub fn set_range(&mut self, lo: u16, hi: u16) {
        self.y_range = (lo, hi);
        self.move_to(self.x, self.y);
    }

    /// int-33h `ax = 4` / motion — move, clamped to screen and range.
    pub fn move_to(&mut self, x: u16, y: u16) {
        self.x = x.min(319);
        self.y = y.clamp(self.y_range.0, self.y_range.1.min(199));
    }

    /// Edge-detect the two buttons and queue the driver's synthetic
    /// codes for any change.
    pub fn set_buttons(&mut self, left: bool, right: bool) {
        let now = u8::from(left) | u8::from(right) << 1;
        for (bit, down, up) in [
            (0, code::LEFT_DOWN, code::LEFT_UP),
            (1, code::RIGHT_DOWN, code::RIGHT_UP),
        ] {
            match (self.buttons & (1 << bit) != 0, now & (1 << bit) != 0) {
                (false, true) => self.queue.push_back(down),
                (true, false) => self.queue.push_back(up),
                _ => {}
            }
        }
        self.buttons = now;
    }

    /// int-9 — latch a scancode (make or break).
    pub fn key(&mut self, scan: u8) {
        self.queue.push_back(scan);
    }

    /// Mirror state into the `ds:` cells; drops at most one queued code
    /// per frame into the `[0x9CCB]` latch, only when it's free.
    pub fn write(&mut self, vm: &mut Vm, st: &mut State) {
        vm.w16(st, u16::try_from(CUR_X).unwrap_or(0), self.x);
        vm.w16(st, u16::try_from(CUR_Y).unwrap_or(0), self.y);
        vm.w8(
            st,
            u16::try_from(K_MODE).unwrap_or(0),
            u8::from(self.k_mode),
        );
        if vm.r8(st, u16::try_from(KEY_PEND).unwrap_or(0)) == 0 {
            if let Some(c) = self.queue.pop_front() {
                vm.w8(st, u16::try_from(KEY_CODE).unwrap_or(0), c);
                vm.w8(st, u16::try_from(KEY_PEND).unwrap_or(0), 1);
            }
        }
    }
}
