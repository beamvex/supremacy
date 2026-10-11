//! `AdLib` patch/volume ops — `0x2D04` `patch_fetch`,
//! `0x2DE6` `patch_select`, `0x2E74` `vol_apply`, `0x301B`
//! `slot_stage`, `0x3066` `vol_slot`.

use super::{Ar, Ports};

impl<P: Ports> Ar<P> {
    /// `0x2E74` — write the `0x7F`-then-`wvol`-clamped volume `v` for
    /// record `c`'s voice into `stot` and refresh the operator's TL
    /// register via `vol_slot`. Voice `0..6` `map` through
    /// `cs:0x1D0E`'s mod/car pair (the carrier is `+1`); voice 6
    /// takes `cs:0x1D21`, voices `7..` `cs:0x1D20 + (v-6)·2`.
    pub(super) fn vol_apply(&mut self, c: usize, v: u8) {
        let al = self.voice(c);
        let op = match al {
            0..=5 => self.d8(0x1D0F + u16::from(al) * 2),
            6 => self.d8(0x1D21),
            _ => self.d8(0x1D20 + u16::from(al - 6) * 2),
        };
        let v = v.min(0x7F).min(self.wvol);
        self.stot[usize::from(op)] = v;
        self.vol_slot(op);
    }

    /// `0x3066` — re-emit operator `op`'s `0x40` TL register:
    /// `atten = 0x3F - (slot[8]·stot[op]·2 + 0x7F)/0xFE`, OR'd under
    /// `slot[0]<<6` (the KSL bits).
    pub(super) fn vol_slot(&mut self, op: u8) {
        let s = &self.slot[usize::from(op)];
        let lev = (u16::from(s[8]) * u16::from(self.stot[usize::from(op)]) * 2 + 0x7F) / 0xFE;
        let data = (s[0] << 6) | 0x3Fu8.wrapping_sub(lev.to_le_bytes()[0]);
        self.op_map(op, 0x40, data);
    }

    /// `0x2DE6` — `select` program `p` (0-based) on record `c`'s voice:
    /// `fetch` the 30-byte FM record, copy its `+0x1C` flag into
    /// `op4`, then stage the two 13-byte operator blocks through
    /// `slot_stage` (voice `6` uses the `cs:0x1D20` pair; `7..` the
    /// single slot at `+ (id-6)·2`; melodic uses `cs:0x1D0E/+0xF`).
    pub(super) fn patch_select(&mut self, c: usize, p: u8) {
        self.patch_fetch(p);
        let op4 = self.patch[0x1C];
        self.chans[c].op4 = op4;
        let id = self.voice(c);
        self.st8s(0x79, id);
        self.st8s(0x75, self.patch[0x1A]);
        self.st8s(0x77, self.patch[0x1B]);
        let (m, w) = (self.patch[0x1A], self.patch[0x1B]);
        match id {
            0..=5 => {
                self.stage_op(self.d8(0x1D0E + u16::from(id) * 2), m, 0);
                self.stage_op(self.d8(0x1D0F + u16::from(id) * 2), w, 13);
            }
            6 => {
                self.stage_op(self.d8(0x1D20), m, 0);
                self.stage_op(self.d8(0x1D21), w, 13);
            }
            _ => self.stage_op(self.d8(0x1D20 + u16::from(id - 6) * 2), m, 0),
        }
    }

    /// `0x2D04` — copy the `p·0x40 + qend` FM record out of the
    /// `qbase` region into the `cs:0x1E20` staging patch (30 bytes).
    fn patch_fetch(&mut self, p: u8) {
        let ofs = usize::from(self.qbase) * 16 + usize::from(self.qend) + usize::from(p) * 0x40;
        let r = self.rec(ofs, 30).to_vec();
        self.patch[..r.len()].copy_from_slice(&r);
    }

    /// `0x301B` — stage `src` (a 13-byte operator block) + `wave` into
    /// operator `op`'s `0x1E96` slot record, then refresh every
    /// register that block feeds (`0xBD`, `0x08`, TL, `0xC0`,
    /// `0x60`, `0x80`, `0x20`, `0xE0`).
    pub(super) fn slot_stage(&mut self, op: u8, wave: u8, src: &[u8; 13]) {
        let s = &mut self.slot[usize::from(op)];
        s[..13].copy_from_slice(src);
        s[0xD] = wave & 3;
        self.rhythm_write();
        self.reg8_write();
        self.vol_slot(op);
        self.conn_write(op);
        self.ad_write(op);
        self.sr_write(op);
        self.mul_write(op);
        self.ws_write(op);
    }

    /// Stage from the `patch` staging record's block `ofs`
    /// (`0` = modulator half, `13` = carrier half).
    fn stage_op(&mut self, op: u8, wave: u8, ofs: usize) {
        let mut src = [0u8; 13];
        src.copy_from_slice(&self.patch[ofs..ofs + 13]);
        self.slot_stage(op, wave, &src);
    }
}
