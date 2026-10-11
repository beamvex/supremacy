//! `AdLib` `init` blocks — `0x2D55` `opl_init`, `0x2D77`
//! `mode_sel`, `0x2DD5` `rhy_set`, `0x2DAC` `wave_init`, `0x2F9D`
//! `slot_init`, `0x300C` `stot_fill`, `0x3290` `key_off_raw`.

use super::{Ar, Ports};

impl<P: Ports> Ar<P> {
    /// `0x2D55` — cold OPL setup: `stot` fill, `mode_sel(1)` (the
    /// rhythm-voice key-off + slot `init`), `rhy_set(0,0,0)`, a raw
    /// key-off on all 9 melodic channels, `wave_init(1)`.
    pub(super) fn opl_init(&mut self) {
        self.stot_fill();
        self.mode_sel(1);
        self.rhy_set(0, 0, 0);
        for dl in 0..9 {
            self.key_off_raw(dl);
        }
        self.wave_init(1);
    }

    /// `0x2D77` — `al == 1` arms rhythm mode: raw key-offs on voices
    /// `6..8`, then `key_write` seeds voice 8 at note `0x18` and
    /// voice 7 at `0x1F` (key bit clear). Either way `[0x1C98]`'s
    /// mask resets, `slot_init` restages the default patches, and
    /// `0xBD` is re-emitted.
    fn mode_sel(&mut self, al: u8) {
        if al == 1 {
            for dl in 6..9 {
                self.key_off_raw(dl);
            }
            self.key_write(8, 0x18, 0);
            self.key_write(7, 0x1F, 0);
        }
        self.st8s(0x7B, 0);
        self.slot_init();
        self.rhythm_write();
    }

    /// `0x2DD5` — latch the three rhythm-enable bytes
    /// (`[0x1C86..0x1C8A]` step 2) then refresh `0xBD` and `0x08`.
    fn rhy_set(&mut self, al: u8, bl: u8, cl: u8) {
        self.st8s(0x69, al);
        self.st8s(0x6B, bl);
        self.st8s(0x6D, cl);
        self.rhythm_write();
        self.reg8_write();
    }

    /// `0x2DAC` — `[0x1C90] = (al<<5)&0x20` (the wave-enable bit),
    /// all `0x12` operators' `0xE0` cleared, then `opl_reg(1, bit)`.
    fn wave_init(&mut self, al: u8) {
        let bit = (al << 5) & 0x20;
        self.st8s(0x73, bit);
        self.st8s(0x74, 0);
        for i in 0..0x12u16 {
            let reg = self.d8(0x1D2A + i).wrapping_add(0xE0);
            self.opl_reg(reg, 0);
        }
        self.opl_reg(1, bit);
    }

    /// `0x2F9D` — restage all 18 operator slots from the shipped
    /// defaults: `cs:0x1C9E`/`0x1CAC` picked by the `cs:0x1D3C`
    /// kind byte for `0..0x12`, then the six explicit rewrites
    /// (`0xC`,`0xF`,`0x10`,`0xE`,`0x11`,`0xD`).
    fn slot_init(&mut self) {
        for dl in 0..0x12u8 {
            let ofs = if self.d8(0x1D3C + u16::from(dl)) == 1 {
                0x1CAC
            } else {
                0x1C9E
            };
            self.stage_img(dl, 0, ofs);
        }
        for (op, ofs) in [
            (0x0C, 0x1CBAu16),
            (0x0F, 0x1CC8),
            (0x10, 0x1CD6),
            (0x0E, 0x1CE4),
            (0x11, 0x1CF2),
            (0x0D, 0x1D00),
        ] {
            self.stage_img(op, 0, ofs);
        }
    }

    /// `0x300C` — `stot` back to `0x7F` (the full-scale default).
    fn stot_fill(&mut self) {
        self.stot = [0x7F; 18];
    }

    /// `0x3290` — raw `A0+dl`/`B0+dl` clear (drops the key bit).
    pub(super) fn key_off_raw(&mut self, dl: u8) {
        self.opl_reg(0xA0 + dl, 0);
        self.opl_reg(0xB0 + dl, 0);
    }

    /// Stage the 13-byte module-image record at `ofs` (wave `w`)
    /// into operator `op` — the `bx = cs:` source form of
    /// `slot_stage`.
    fn stage_img(&mut self, op: u8, w: u8, ofs: u16) {
        let mut src = [0u8; 13];
        for (i, b) in src.iter_mut().enumerate() {
            *b = self.d8(ofs + u16::try_from(i).unwrap_or_default());
        }
        self.slot_stage(op, w, &src);
    }
}
