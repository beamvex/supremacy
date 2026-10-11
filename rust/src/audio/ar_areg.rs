//! `AdLib` per-operator register writers — `0x31E7`
//! `rhythm_write` (`0xBD`), `0x30C5` `reg8_write` (`0x08`), `0x30DC`
//! `conn_write` (`0xC0`), `0x3120` `ad_write` (`0x60`), `0x3151`
//! `sr_write` (`0x80`), `0x3187` `mul_write` (`0x20`), `0x3214`
//! `ws_write` (`0xE0`). Each re-emits one register for operator
//! `ah` from its `0x1E96` slot record through the `cs:0x1D2A` `map`
//! (`0x30DC` uses `cs:0x1D4E`).

use super::{Ar, Ports};

impl<P: Ports> Ar<P> {
    /// `0x31E7` — `0xBD`: `0x80`/`0x40` from the `[0x1C86]`/`[0x1C88]`
    /// rhythm-enable flags, `0x20` always, low bits = `[0x1C98]` mask.
    pub(super) fn rhythm_write(&mut self) {
        let hi = if self.st8(0x69) != 0 { 0x80 } else { 0 }
            | if self.st8(0x6B) != 0 { 0x40 } else { 0 }
            | 0x20;
        self.opl_reg(0xBD, hi | self.st8(0x7B));
    }

    /// `0x30C5` — `0x08` (CSM/note-sel): `0x40` while `[0x1C8A]`.
    pub(super) fn reg8_write(&mut self) {
        let v = if self.st8(0x6D) != 0 { 0x40 } else { 0 };
        self.opl_reg(0x08, v);
    }

    /// `0x30DC` — `0xC0+map` feedback/connection for `op`. Ops whose
    /// `cs:0x1D3C` kind byte is nonzero skip; else `slot[2]<<1`
    /// (`slot[0xC]` zero sets bit0 — the `& 0xFE` masks it right
    /// back off, faithfully kept).
    pub(super) fn conn_write(&mut self, op: u8) {
        if self.d8(0x1D3C + u16::from(op)) != 0 {
            return;
        }
        let s = &self.slot[usize::from(op)];
        let mut dl = s[2] << 1;
        if s[0xC] == 0 {
            dl |= 1;
        }
        let reg = self.d8(0x1D4E + u16::from(op)).wrapping_add(0xC0);
        self.opl_reg(reg, dl & 0xFE);
    }

    /// `0x3120` — `0x60+map`: `slot[3]<<4 | slot[6]` (attack/decay).
    pub(super) fn ad_write(&mut self, op: u8) {
        let s = &self.slot[usize::from(op)];
        let v = (s[3] << 4) | s[6];
        self.op_map(op, 0x60, v);
    }

    /// `0x3151` — `0x80+map`: `(0xF - slot[4])<<4 | slot[7]`
    /// (sustain/release — the `0xF` complement wraps like the asm's
    /// `sub`/`neg`).
    pub(super) fn sr_write(&mut self, op: u8) {
        let s = &self.slot[usize::from(op)];
        let v = (0xFu8.wrapping_sub(s[4]) << 4) | s[7];
        self.op_map(op, 0x80, v);
    }

    /// `0x3187` — `0x20+map`: flag bits from `slot[9]`/`[0xA]`/`[5]`/
    /// `[0xB]` under `slot[1] & 0xF` (trem/vib/sus/ksr/mult).
    pub(super) fn mul_write(&mut self, op: u8) {
        let s = &self.slot[usize::from(op)];
        let v = u8::from(s[9] != 0) << 7
            | u8::from(s[0xA] != 0) << 6
            | u8::from(s[5] != 0) << 5
            | u8::from(s[0xB] != 0) << 4
            | s[1] & 0xF;
        self.op_map(op, 0x20, v);
    }

    /// `0x3214` — `0xE0+map`: `slot[0xD] & 3` (the wave-`select` byte).
    pub(super) fn ws_write(&mut self, op: u8) {
        let v = self.slot[usize::from(op)][0xD] & 3;
        self.op_map(op, 0xE0, v);
    }

    /// `opl_reg(map[op] + base, v)` — the shared `0x1D2A` lookup tail.
    pub(super) fn op_map(&mut self, op: u8, base: u8, v: u8) {
        let reg = self.d8(0x1D2A + u16::from(op)).wrapping_add(base);
        self.opl_reg(reg, v);
    }
}
