use super::tandy_cmd::ENV_TAB;
use super::ttab::{self, Tev};
use super::{Ports, Tandy};

/// `cs:0x762` — the `0xA0..0xBF`-byte → vibrato base table.
const INST_TAB: u16 = 0x762;
/// `cs:0x7E2` — tremolo `{wait, val}` sequence pairs.
const TREM_TAB: u16 = 0x7E2;

/// Decode bound — the asm loops forever on a malformed stream.
const LIM: usize = 0x400;

impl<P: Ports> Tandy<P> {
    /// `0x1C02` — one channel's stream step, now over the compiled
    /// [`ttab`] event table (GAM-80). `wait` counts down; while it
    /// holds, `0x1C39`'s sustain slide runs. On expiry the stream runs
    /// until a `Note`, a `Mute`/`Hold`, or `End` — `End` pops the step
    /// frame and pushes the tick tail: the whole tick is over (`true`)
    /// and later channels are not stepped.
    pub(super) fn step(&mut self, c: usize) -> bool {
        let ch = &mut self.chans[c];
        ch.wait = ch.wait.wrapping_sub(1);
        if ch.wait != 0 {
            self.slide(c);
            return false;
        }
        let mut dl = 0u8;
        let mut si = self.chans[c].ptr;
        for _ in 0..LIM {
            let Some((ev, next)) = ttab::at(si) else {
                return false;
            };
            match ev {
                Tev::Note(al) => {
                    self.note(c, al, dl, next);
                    return false;
                }
                Tev::End => {
                    self.op_end();
                    return true;
                }
                Tev::Mute => {
                    self.op_mute(c);
                    self.tail(c, dl, next);
                    return false;
                }
                Tev::Hold => {
                    self.tail(c, dl, next);
                    return false;
                }
                Tev::Wild => return false,
                other => si = self.exec(c, other, next, &mut dl),
            }
        }
        false
    }

    /// `0x1C39` — the sustain slide: `0x40` walks `+0x16` ±1,
    /// `0x80` picks down.
    fn slide(&mut self, c: usize) {
        let ch = &mut self.chans[c];
        if ch.flags & 0x40 == 0 {
            return;
        }
        if ch.flags & 0x80 != 0 {
            ch.note = ch.note.wrapping_sub(1);
        } else {
            ch.note = ch.note.wrapping_add(1);
        }
    }

    /// One non-suspending event — returns the next cursor.
    fn exec(&mut self, c: usize, ev: Tev, next: u16, dl: &mut u8) -> u16 {
        match ev {
            Tev::Tsel(v) => self.chans[c].tsel = v,
            Tev::Sweep { dt, n } => {
                self.chans[c].sdelta = dt;
                self.chans[c].scount = n;
                *dl |= 8;
            }
            Tev::Flags(v) => *dl |= v,
            Tev::Next => return self.op_next(c),
            Tev::Gtrans(v) => self.gtrans = v,
            Tev::Nadd(w) => self.chans[c].nadd = w,
            Tev::Trans(v) => self.chans[c].trans = v,
            Tev::List(a) => return self.op_list(c, a),
            Tev::Tempo(v) => {
                self.tempo = v;
                self.tempo0 = v;
            }
            Tev::Mvol(v) => self.mvol = v,
            Tev::Reload(v) => self.chans[c].reload = v,
            Tev::Esel(v) => self.chans[c].esel = v,
            Tev::Inst(i) => {
                self.chans[c].inst = self.d8(INST_TAB + 0x40 + u16::from(i));
            }
            _ => {}
        }
        next
    }

    /// `0x1BA2` — `0x85` next stream: `list+2`, `0` word wraps to base.
    fn op_next(&mut self, c: usize) -> u16 {
        let mut s = self.chans[c].list.wrapping_add(2);
        if self.d16(s) == 0 {
            s = self.chans[c].base;
        }
        self.chans[c].list = s;
        self.d16(s)
    }

    /// `0x1BE1` — `0x8B` switch list: `base`/`list` take the operand,
    /// cursor becomes its first entry.
    fn op_list(&mut self, c: usize, a: u16) -> u16 {
        self.chans[c].base = a;
        self.chans[c].list = a;
        self.d16(a)
    }

    /// `0x1C5E` — a note byte `< 0x80`: tremolo pair reload (`+0x13`
    /// → `cs:0x7E2`), tie check on the OLD note's bit7 (set → skip
    /// the vibrato/envelope reset), then the `0x1C95` tail.
    fn note(&mut self, c: usize, al: u8, dl: u8, si: u16) {
        let ch = &mut self.chans[c];
        ch.tcur = ch.tsel;
        let tied = ch.note & 0x80 != 0;
        let tw = self.d8(TREM_TAB + u16::from(self.chans[c].tsel));
        self.chans[c].twait = tw;
        self.chans[c].note = al;
        if !tied {
            self.note_env(c);
        }
        self.tail(c, dl, si);
    }

    /// The untied-note envelope reset (`vcur ← inst`, `ecur ← esel` via
    /// `cs:0x85F`, unless sfx owns the channel).
    fn note_env(&mut self, c: usize) {
        self.chans[c].vcur = self.chans[c].inst;
        if self.chans[c].vlatch < 0xD0 || self.sfx_on == 0 {
            let e = self.d8(ENV_TAB + u16::from(self.chans[c].esel));
            let ch = &mut self.chans[c];
            ch.ecur = e;
            ch.ewait = 1;
            ch.evol = 0;
        }
    }

    /// `0x1C4B` — `0x80` mute: freeze the envelope unless the sfx
    /// engine owns the channel (`latch ≥ 0xD0` while `[0x24]` armed).
    fn op_mute(&mut self, c: usize) {
        if self.chans[c].vlatch < 0xD0 || self.sfx_on == 0 {
            self.chans[c].ewait = 0;
            self.chans[c].evol = 0;
        }
    }

    /// `0x1C95` — the shared decode tail: `run`/`wait`/`flags`/`ptr`.
    pub(super) fn tail(&mut self, c: usize, dl: u8, si: u16) {
        let ch = &mut self.chans[c];
        ch.run = ch.nadd;
        ch.wait = ch.reload;
        ch.flags = dl;
        ch.ptr = si;
    }
}
