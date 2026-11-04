use super::{Ar, Ports};

impl<P: Ports> Ar<P> {
    /// `0x22BC` — the int-8 `service` (also export 2 at `cs:0x006`,
    /// far-callable). Runs `0x1F05` (`fader`), then the cadence:
    /// `[0x19FD] < [0x19F9]` ticks run `0x24AC` (continuous events);
    /// the `div`-th tick fetches the next queue events and runs
    /// `0x23EF` (apply). `pass == 0x40` or `rst` advances `idx`.
    /// (`[0x205A]` is cleared at entry in the asm but never read
    /// elsewhere in the module — a dead cell, unmodelled.)
    pub fn service(&mut self) {
        if self.mode == 0xFF || self.en == 0 {
            return;
        }
        self.fader();
        self.cnt = self.cnt.wrapping_add(1);
        if self.cnt < self.div {
            for c in 0..self.nchans() {
                self.service_c(usize::from(c));
            }
        } else {
            self.cnt = 0;
            self.fetch();
            self.pass = self.pass.wrapping_add(1);
            for c in 0..self.nchans() {
                self.apply_c(usize::from(c));
            }
            if self.pass == 0x40 {
                self.advance();
            }
        }
        if self.rst != 0 {
            self.advance();
        }
    }

    /// `0x1F05` — the `fader`: every other tick `mvol -= frate`, then
    /// `0x1F40` applies the new level (the `jnc` falls into it). On
    /// borrow `mvol = 0` before the apply, then `en = 0`, `fade = 0`.
    fn fader(&mut self) {
        if self.fade == 0 {
            return;
        }
        self.fcnt = self.fcnt.wrapping_sub(1);
        if self.fcnt != 0 {
            return;
        }
        self.fcnt = 2;
        if self.mvol >= self.frate {
            self.mvol -= self.frate;
            self.mvol_apply();
        } else {
            self.mvol = 0;
            self.mvol_apply();
            self.en = 0;
            self.fade = 0;
        }
    }

    /// `0x1F40` — apply `mvol`. Roland: clamp `0x64`, into
    /// `volbuf +0x0A`, `SysEx` it. `AdLib`: `wvol = mvol`, then
    /// `0x2E74` per channel with `bl = wvol`.
    pub(super) fn mvol_apply(&mut self) {
        if self.mode == 1 {
            self.volbuf[0x0A] = self.mvol.min(0x64);
            let b = self.volbuf;
            self.syx(&b);
            return;
        }
        self.wvol = self.mvol;
        for c in 0..self.nchans() {
            self.chan = usize::from(c);
            self.vol_apply(usize::from(c), self.wvol);
        }
    }

    /// `0x23BC` — the queue-`advance`/`reset` block: `cur` back to
    /// bar 0, counters clear, `idx++`, wrapped to `hdr[0xD9]` at
    /// `nmap`.
    fn advance(&mut self) {
        self.cnt = 0;
        self.cur = 0;
        self.rst = 0;
        self.pass = 0;
        self.idx = self.idx.wrapping_add(1);
        if u16::from(self.idx) == self.nmap {
            self.idx = self.hdr[0xD9];
        }
    }

    /// `0x2317` — one queue `fetch`, compiled (GAM-80): `qpages[idx]`
    /// holds the page's bars decoded at `select`, `cur` counts bars.
    /// `dx`'s 2-bit classes became [`ArCell`] records at decode time.
    fn fetch(&mut self) {
        let cells = self
            .qpages
            .get(usize::from(self.idx))
            .and_then(|p| p.get(usize::from(self.cur)))
            .copied()
            .unwrap_or_default();
        self.cur = self.cur.wrapping_add(1);
        let n = usize::from(self.nchans());
        for (ch, cell) in self.chans.iter_mut().zip(cells.iter()).take(n) {
            ch.notes = cell.notes;
            ch.prog = cell.prog;
            ch.kind = cell.kind;
            ch.param = cell.param;
        }
    }

    /// `0x23EF` — the post-`fetch` apply dispatcher (`si` = record,
    /// `bp` = channel + 1): `mode == 0` jumps to the `AdLib` apply at
    /// `0x2A04`; Roland's is the inline `apply_r`.
    fn apply_c(&mut self, c: usize) {
        if self.mode == 0 {
            self.apply_a(c);
        } else {
            self.apply_r(c);
        }
    }

    /// `0x24AC` — the continuous-event dispatcher (same mode split;
    /// `mode == 0` jumps to `0x2ABE`, Roland's is `service_r`).
    fn service_c(&mut self, c: usize) {
        if self.mode == 0 {
            self.service_a(c);
        } else {
            self.service_r(c);
        }
    }

    /// `[0x174B]` — the channel count (records iterated per pass).
    pub(super) fn nchans(&self) -> u8 {
        self.hdr[0xD8]
    }

    /// `tbl[i]` — a word read on the `0x200`-byte sound table.
    pub(super) fn t16(&self, i: u16) -> u16 {
        u16::from(self.tbl[usize::from(i) * 2]) | u16::from(self.tbl[usize::from(i) * 2 + 1]) << 8
    }
}
