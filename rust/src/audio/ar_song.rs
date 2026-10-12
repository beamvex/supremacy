use super::{Ar, Ports};

impl<P: Ports> Ar<P> {
    /// Export 3 (`cs:0x009`, `0x2081`) — load the `ds:si` record: the
    /// `0x118`-byte header to `cs:0x1673`, the `0x200`-byte sound
    /// table to `cs:0x178C`, the segment to `[0x1C03]`. When live:
    /// `[0x1C81]` = `tbl[maxmap + 1]` (the `0x236A` scan's highest
    /// `map` value + 1 — the queue region just past the last sound),
    /// Roland runs `0x2104` (the per-program `timbre` upload + `C1`
    /// program sweep), and `[0x1FCE]` = `0x122 + 0x100·nprogs`.
    pub fn load_song(&mut self, rec: &[u8]) {
        if let Some(h) = rec.get(..0x118) {
            self.hdr.copy_from_slice(h);
        }
        if let Some(t) = rec.get(0x118..0x318) {
            self.tbl.copy_from_slice(t);
        }
        self.song = rec.to_vec();
        if self.mode == 0xFF {
            return;
        }
        self.qbase = self.t16(u16::from(self.maxmap()) + 1);
        if self.mode == 1 {
            self.prog_init();
        }
        self.qend = 0x122 + 0x100 * u16::from(self.progcount());
    }

    /// `0x236A` — the highest `map` value over `map[0..0x7F]` (`dl`
    /// tracks the running max; `0` for an all-zero `map`).
    fn maxmap(&self) -> u8 {
        (0..0x7Fu8).map(|i| self.map(i)).max().unwrap_or(0)
    }

    /// `0x2104` — for each `progmap` entry with `{prog, vol}` both
    /// nonzero, the `0x214F` `timbre` upload; then `C1 <i>` program
    /// changes for `i` = `0..0x20`.
    fn prog_init(&mut self) {
        for i in 0..0x20u8 {
            let (p, v) = self.progmap(i);
            if p != 0 && v != 0 {
                self.timbre(i);
            }
        }
        for i in 0..0x20u8 {
            self.midi(0xC1);
            self.midi(i);
        }
    }

    /// The `progmap`'s nonzero-program count (the `0x20D9` loop —
    /// `ah` tallies; `nprogs` only counts entries with `lo != 0`).
    fn progcount(&self) -> u8 {
        u8::try_from((0..0x20u8).filter(|&i| self.progmap(i).0 != 0).count()).unwrap_or_default()
    }

    /// `0x214F` — one MT-32 `timbre` upload for program `i`. The record
    /// sits `0x118 + i·0x100` into the `tbl[1]` region: `+0` name (8),
    /// `+0x12` index (4), then four `0x3A`-byte param blocks (`+0x16`
    /// through `+0xFD`), each `SysEx`'d with the `0x228C` address bump.
    /// A name starting with space (`0x20`) skips the upload entirely.
    fn timbre(&mut self, i: u8) {
        let base = usize::from(self.qbase) * 16 + 0x118 + usize::from(i) * 0x100;
        if self.song.get(base) == Some(&0x20) {
            return;
        }
        self.stage[0x5A] = u8::try_from(u16::from(i) * 8 / 0x80).unwrap_or_default();
        self.stage[0x5B] = u8::try_from(u16::from(i) * 8 % 0x80).unwrap_or_default();
        let name = self.rec(base, 8).to_vec();
        self.stage[0x5C..0x5C + name.len()].copy_from_slice(&name);
        if self.stage[0x5C] != 3 {
            self.stage[0x5D] = i;
        }
        let blk = self.stg(0x1C6F, 0x14).to_vec();
        self.syx(&blk);
        self.tidx(base + 0x12, i);
        for p in 0..4 {
            self.tpart(base + 0x16 + p * 0x3A);
        }
    }

    /// `0x21B6` block — seed the `{0x0A, i·2}` `SysEx` address cursor
    /// (`[0x1C83]`), copy the 4-byte index into the `0x1C61` block,
    /// send it, `0x228C` bump by 4.
    fn tidx(&mut self, ofs: usize, i: u8) {
        self.stage[0x66] = 0x0A;
        self.stage[0x67] = i.wrapping_shl(1);
        self.stage[0x4C] = self.stage[0x67];
        self.stage[0x4D] = self.stage[0x66];
        let idx = self.rec(ofs, 4).to_vec();
        self.stage[0x4E..0x4E + idx.len()].copy_from_slice(&idx);
        let b = self.stg(0x1C61, 0x10).to_vec();
        self.syx(&b);
        self.tbump(4);
    }

    /// One `0x3A`-byte param block (`0x21E9`+): the cursor bytes into
    /// the `0x1C1D` block's address field, the body copy, the send,
    /// the `0x3A` bump.
    fn tpart(&mut self, ofs: usize) {
        self.stage[0x08] = self.stage[0x67];
        self.stage[0x09] = self.stage[0x66];
        let body = self.rec(ofs, 0x3A).to_vec();
        self.stage[0x0A..0x0A + body.len()].copy_from_slice(&body);
        let b = self.stg(0x1C1D, 0x4E).to_vec();
        self.syx(&b);
        self.tbump(0x3A);
    }

    /// `0x228C` — the `SysEx` address bump: `[0x1C83] += n`, and on
    /// `> 0x7F` wrap into `[0x1C84]` (`& 0x7F`, `inc`).
    fn tbump(&mut self, n: u8) {
        self.stage[0x66] = self.stage[0x66].wrapping_add(n);
        if self.stage[0x66] > 0x7F {
            self.stage[0x66] &= 0x7F;
            self.stage[0x67] = self.stage[0x67].wrapping_add(1);
        }
    }

    /// `0x22A4` — `nmap` (`[0x1FD2]`) = one past the last nonzero
    /// `map` index (scanning `map[0x7F]` down to `map[1]`; `1` when
    /// all are zero — the loop's `inc cx` tail).
    pub(super) fn map_bound(&mut self) {
        self.nmap = (1..=0x7Fu8)
            .rev()
            .find(|&i| self.map(i) != 0)
            .map_or(1, |i| u16::from(i) + 1);
    }

    /// `0x1FE0` — silence everything. Roland: the `0x200B` `map` block
    /// copied to `anobuf +0x0A`, the all-notes-off `SysEx`, then
    /// `0x7B/0x79/7/0x0A` controller resets on channels 1–9. `AdLib`:
    /// `0x205B` runs `ah = 0..0xB` through `0x2F70` — the melodic
    /// key-writes plus the rhythm mask clears.
    pub(super) fn all_off(&mut self) {
        if self.mode == 0 {
            for ah in 0..0xB {
                self.noff_tail(ah);
            }
            return;
        }
        let n = usize::from(self.nchans()).saturating_sub(1) * 8;
        for i in 0..8 {
            self.anobuf[0x0A + i] = self.d8(0x200B + u16::try_from(n + i).unwrap_or_default());
        }
        let b = self.anobuf;
        self.syx(&b);
        for ch in 1..9u8 {
            for &d in &[0x7B, 0x79, 7, 0x0A] {
                self.midi(0xB0 + ch);
                self.midi(d);
                self.midi(if d == 7 {
                    0x7F
                } else if d == 0xA {
                    0x40
                } else {
                    0
                });
            }
        }
    }
}
