use super::{Pc, Ports};

impl<P: Ports> Pc<P> {
    /// The `cs:0x32` opcode table (`0x80..0x8F` → handlers at
    /// `0xEBC`–`0xFA1`). Returns true when the decode ends — `0x80`/
    /// `0x8B` finish through the `0xF97` tail (`wait=reload`,
    /// `ptr=si`), `0x87` is the song-end marker (file `0xFA1`: its
    /// `pop ax` drops the step frame so the `ret` lands on the ISR
    /// epilogue — the whole `int 0x80` call is over, and it kills
    /// `[0x23]`/`[0x24]`/`[0x2A]`). Bytes `0x90..0x9F` index past the
    /// real table — the asm would jump wild; the port stops.
    pub(super) fn opcode(&mut self, c: usize, op: u8, dl: &mut u8, si: &mut u16) -> bool {
        match op {
            0x80 => self.chans[c].fade = 0,     // 0xF81 — mute → tail
            0x81 => *dl = 0,                    // 0xF28 — flags clear
            0x82 => self.op_sweep(c, dl, si),   // 0xEFC
            0x83 => *dl |= 0xC0,                // 0xF20 — slide down
            0x84 => *dl |= 0x40,                // 0xF23 — slide up
            0x85 => self.op_next(c, si),        // 0xEDC — list advance
            0x86 => self.op_tremolo(c, dl, si), // 0xF09
            0x87 => self.end_song(),            // 0xFA1 — end marker
            0x88 => self.op_trans(c, si),       // 0xEF0
            0x89 => *dl |= 2,                   // 0xF1B — half-mute
            0x8A => self.op_mux(c, si),         // 0xF2C
            0x8B => {}                          // hold — falls to tail
            0x8C => self.op_gtrans(si),         // 0xEF6
            0x8D => self.op_list(c, si),        // 0xED1
            0x8E => self.op_tempo(si),          // 0xEC8
            0x8F => self.op_rotmux(c),          // 0xEBC
            _ => return true,                   // 0x90.. — wild; stop
        }
        match op {
            0x80 | 0x8B | 0x87 => {
                if op != 0x87 {
                    self.chans[c].wait = self.chans[c].reload;
                    self.chans[c].ptr = *si;
                }
                true
            }
            _ => false,
        }
    }

    /// `0xEFC` — sweep: `word` delta `+0x13`, flags `|8`, byte counter
    /// `+0x0E`.
    fn op_sweep(&mut self, c: usize, dl: &mut u8, si: &mut u16) {
        self.chans[c].sdelta = self.d16(*si);
        *si += 2;
        *dl |= 8;
        self.chans[c].scount = self.d8(*si);
        *si += 1;
    }

    /// `0xEDC` — advance the stream list: cursor `+0x02 +2`, a `0` word
    /// there loops to `+0x00`; the cursor stores back to `+0x02` and
    /// `si` takes the word at it.
    fn op_next(&mut self, c: usize, si: &mut u16) {
        let mut s = self.chans[c].list.wrapping_add(2);
        if self.d16(s) == 0 {
            s = self.chans[c].base;
        }
        self.chans[c].list = s;
        *si = self.d16(s);
    }

    /// `0xF09` — tremolo: word low byte `+0x10`, byte `+0x11`, doubled
    /// into `+0x0F`; flags `|0x10`.
    fn op_tremolo(&mut self, c: usize, dl: &mut u8, si: &mut u16) {
        self.chans[c].tdep = self.d8(*si);
        *si += 2;
        self.chans[c].tcur = self.d8(*si);
        *si += 1;
        self.chans[c].tlim = self.chans[c].tcur << 1;
        *dl |= 0x10;
    }

    /// `0xFA1` — the end marker clears `loaded`, `sfx_on`, `mus`.
    fn end_song(&mut self) {
        self.loaded = 0;
        self.sfx_on = 0;
        self.mus = 0;
    }

    /// `0xEF0` — channel transpose `+0x16`.
    fn op_trans(&mut self, c: usize, si: &mut u16) {
        self.chans[c].trans = self.d8(*si);
        *si += 1;
    }

    /// `0xF2C` — fixed mux byte `+0x08`; on ch1 it also kills the
    /// `[0x2C]` rotate so the byte shifts out.
    fn op_mux(&mut self, c: usize, si: &mut u16) {
        self.chans[c].mux = self.d8(*si);
        *si += 1;
        if c == 1 {
            self.mux_ror = 0;
        }
    }

    /// `0xEF6` — the global transpose cell `[0x2B]`.
    fn op_gtrans(&mut self, si: &mut u16) {
        self.gtrans = self.d8(*si);
        *si += 1;
    }

    /// `0xED1` — switch to a new list: `ptr` word is the list address,
    /// `base`/`list` take it, `ptr` becomes its first entry.
    fn op_list(&mut self, c: usize, si: &mut u16) {
        let s = self.d16(*si);
        self.chans[c].base = s;
        self.chans[c].list = s;
        *si = self.d16(s);
    }

    /// `0xEC8` — stream tempo into `[0x26]`/`[0x27]`.
    fn op_tempo(&mut self, si: &mut u16) {
        self.tempo = self.d8(*si);
        self.tempo0 = self.tempo;
        *si += 1;
    }

    /// `0xEBC` — the `0x55` alternating mux + `[0x2C]` rotate mode.
    fn op_rotmux(&mut self, c: usize) {
        self.chans[c].mux = 0x55;
        self.mux_ror = 0xFF;
    }
}
