use super::pctab::{self, PcEv};
use super::{Pc, Ports};

impl<P: Ports> Pc<P> {
    /// The `0xF3D` stream step — one channel's decode, now over the
    /// compiled [`pctab`] event table (GAM-80). `wait` counts down;
    /// while it holds, the `0xF6F` sustain path applies the
    /// `0x40`/`0x80` pitch slide; on expiry the stream runs until a
    /// `Note`, a `Mute`/`Hold`, or `End` — `End` drops the step frame
    /// (the whole `int 0x80` call is over, `true`, after clearing
    /// `[0x23]`/`[0x24]`/`[0x2A]`).
    pub(super) fn step(&mut self, c: usize) -> bool {
        let mut dl = self.chans[c].flags;
        self.chans[c].wait = self.chans[c].wait.wrapping_sub(1);
        if self.chans[c].wait != 0 {
            self.slide(c, dl);
            return false;
        }
        dl &= 0x30;
        let mut si = self.chans[c].ptr;
        for _ in 0..0x400 {
            let Some((ev, next)) = pctab::at(si) else {
                return false;
            };
            match ev {
                PcEv::Note(al) => {
                    self.note(c, al, dl, next);
                    return false;
                }
                PcEv::Mute => {
                    self.chans[c].fade = 0;
                    self.hold(c, next);
                    return false;
                }
                PcEv::Hold => {
                    self.hold(c, next);
                    return false;
                }
                PcEv::End => {
                    self.end_song();
                    return true;
                }
                PcEv::Wild => return false,
                other => si = self.exec(c, other, next, &mut dl),
            }
        }
        false
    }

    /// `0xF6F` — sustain: `0x40` set slides `+0x15` each tick, `0x80`
    /// picks the direction.
    fn slide(&mut self, c: usize, dl: u8) {
        if dl & 0x40 == 0 {
            return;
        }
        if dl & 0x80 != 0 {
            self.chans[c].note = self.chans[c].note.wrapping_sub(1);
        } else {
            self.chans[c].note = self.chans[c].note.wrapping_add(1);
        }
    }

    /// The `0x80`/`0x8B` hold tail — `wait = reload`, `ptr` commits.
    fn hold(&mut self, c: usize, si: u16) {
        self.chans[c].wait = self.chans[c].reload;
        self.chans[c].ptr = si;
    }

    /// One non-suspending event — returns the next cursor.
    fn exec(&mut self, c: usize, ev: PcEv, next: u16, dl: &mut u8) -> u16 {
        match ev {
            PcEv::Flags(0) => *dl = 0,
            PcEv::Flags(v) => *dl |= v,
            PcEv::Sweep { dt, n } => {
                self.chans[c].sdelta = dt;
                *dl |= 8;
                self.chans[c].scount = n;
            }
            PcEv::Trem { dep, cur } => {
                self.chans[c].tdep = dep;
                self.chans[c].tcur = cur;
                self.chans[c].tlim = cur << 1;
                *dl |= 0x10;
            }
            PcEv::Next => return self.op_next(c),
            PcEv::Trans(v) => self.chans[c].trans = v,
            PcEv::Mux(v) => {
                self.chans[c].mux = v;
                if c == 1 {
                    self.mux_ror = 0;
                }
            }
            PcEv::Gtrans(v) => self.gtrans = v,
            PcEv::List(a) => return self.op_list(c, a),
            PcEv::Tempo(v) => {
                self.tempo = v;
                self.tempo0 = v;
            }
            PcEv::RotMux => {
                self.chans[c].mux = 0x55;
                self.mux_ror = 0xFF;
            }
            PcEv::Reload(v) => self.chans[c].reload = v,
            PcEv::Env(i) => self.chans[c].env = self.d8(0x5EA + u16::from(i)),
            _ => {}
        }
        next
    }

    /// `0xEDC` — advance the stream list: cursor `+0x02 +2`, a `0` word
    /// there loops to `+0x00`; the cursor stores back to `+0x02` and
    /// the new cursor takes the word at it.
    fn op_next(&mut self, c: usize) -> u16 {
        let mut s = self.chans[c].list.wrapping_add(2);
        if self.d16(s) == 0 {
            s = self.chans[c].base;
        }
        self.chans[c].list = s;
        self.d16(s)
    }

    /// `0xED1` — switch to a new list: `base`/`list` take it, the
    /// cursor becomes its first entry.
    fn op_list(&mut self, c: usize, a: u16) -> u16 {
        self.chans[c].base = a;
        self.chans[c].list = a;
        self.d16(a)
    }

    /// `0xF86` + the `0xF97` tail — a note byte: stash the flags copy,
    /// reset the sweep, copy the fade/envelope seeds `+0x08`/`+0x09`
    /// into the live `+0x0A`/`+0x0B`, reload `wait`, store `ptr`.
    fn note(&mut self, c: usize, al: u8, dl: u8, si: u16) {
        let ch = &mut self.chans[c];
        ch.sweep = 0;
        ch.flags = dl;
        ch.note = al;
        ch.fade = ch.mux;
        ch.seq = ch.env;
        ch.wait = ch.reload;
        ch.ptr = si;
    }
}
