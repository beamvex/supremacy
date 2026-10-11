use super::{Ports, Tandy};

/// `cs:0x4EA` — sfx record table (`0x14` bytes each).
const SFX_TAB: u16 = 0x4EA;
/// `cs:0x93E` — song records `{tempo, list0..3}` (9 bytes each).
const SONG_TAB: u16 = 0x93E;
/// `cs:0x85F` — the `esel → ecur` table the loader seeds ch2 from.
const ENV_TAB: u16 = 0x85F;

impl<P: Ports> Tandy<P> {
    /// The `int 0x80` entry (file `0x1950`): `ah < 0x20` jumps through
    /// the `cs:0x52` table — `0` tick, `1` stop, `2`/`al` song,
    /// `3`/`al` sfx, `4` resume (five entries; `ah ≥ 5` falls off the
    /// table onto the `freq` table like the asm's wild near-call, so the
    /// port ignores it). `ah ≥ 0x20` is the `[bx+0x23]` cell path.
    /// Returns `al`.
    pub fn dispatch(&mut self, ah: u8, al: u8) -> u8 {
        if ah >= 0x20 {
            return self.cell_io(ah, al);
        }
        match ah {
            0 => self.tick(),
            1 => self.stop(),
            2 => self.song(al),
            3 => self.effect(al),
            4 => self.mus = 0xFF,
            _ => {}
        }
        al
    }

    /// `ah=1` — file `0x1FFC`: disarm both engines, then mute all
    /// four attenuators (`0x9F` stepping `0x20` to `0xFF`).
    pub fn stop(&mut self) {
        self.mus = 0;
        self.sfx_on = 0;
        let mut v = 0x9Fu8;
        loop {
            self.ports.outb(0xC0, v);
            let (n, c) = v.overflowing_add(0x20);
            if c {
                break;
            }
            v = n;
        }
    }

    /// `ah=2, al` — file `0x1E79`: stop, clear the records (only the
    /// first 14 words — the `+0x1C`/`+0x1D` latch tails persist),
    /// load `{tempo, list0..3}` from `cs:0x93E + al·9`, arm `[0x23]`.
    pub fn song(&mut self, al: u8) {
        let bx = SONG_TAB + u16::from(al) * 9;
        self.stop();
        let mut lists = [0u16; 4];
        for (i, l) in lists.iter_mut().enumerate() {
            *l = self.d16(bx + 1 + u16::try_from(i).unwrap_or_default() * 2);
        }
        let mut ptrs = [0u16; 4];
        for (i, l) in lists.iter().enumerate() {
            ptrs[i] = self.d16(*l);
        }
        for (i, ch) in self.chans.iter_mut().enumerate() {
            *ch = super::TChan::default();
            ch.vlatch = 0x90 + u8::try_from(i).unwrap_or_default() * 0x20;
            ch.tlatch = 0x80 + u8::try_from(i).unwrap_or_default() * 0x20;
            ch.base = lists[i];
            ch.list = lists[i];
            ch.ptr = ptrs[i];
            ch.wait = 1;
        }
        self.beat = 0;
        self.gtrans = 0;
        self.mvol = 0xFF;
        self.tempo = self.d8(bx);
        self.tempo0 = self.tempo;
        self.step_acc = 0xFF;
        self.mus = self.mus.wrapping_sub(1);
    }

    /// `ah=3, al` — file `0x1EDC`: the `0x14`-byte record at
    /// `cs:0x4EA + al·0x14` seeds the `[0x2C..0x34]` sfx block, forces
    /// ch2's envelope cursor to `cs:0x85F[al·0x14 >> 8]` with
    /// `ewait 1` (the `[0x134]` word write clears `evol` too — the
    /// `imul` left the product's high byte in `bl`), `[0x24] = 0xFF`.
    pub fn effect(&mut self, al: u8) {
        let di = SFX_TAB + u16::from(al) * 0x14;
        self.srec = di;
        self.bcnt = self.d8(di + 0x0C);
        self.pcnt = self.d8(di + 0x0D);
        self.sel = self.d8(di + 0x0E);
        self.vsel = self.d8(di + 0x0F);
        self.cur = self.d16(di + 0x10);
        self.end = self.d8(di + 0x12);
        self.chans[2].ewait = 1;
        self.chans[2].evol = 0;
        self.chans[2].ecur = self.d8(ENV_TAB + u16::from(al) * 0x14 / 0x100);
        self.sfx_on = 0xFF;
    }

    /// `0xEB4` — the cell path: `0x20..0x30` reads `cs:0x23 + i`,
    /// `0x30..` writes it (`bl` had `0x30` folded in before the jump).
    fn cell_io(&mut self, ah: u8, al: u8) -> u8 {
        if ah >= 0x30 {
            self.cell_set(ah.wrapping_sub(0x30), al);
            al
        } else {
            self.cell_get(ah - 0x20)
        }
    }

    /// `cs:0x23 + i` read — the named driver cells.
    fn cell_get(&self, i: u8) -> u8 {
        match i {
            0 => self.mus,
            1 => self.sfx_on,
            2 => self.pace,
            3 => self.mvol,
            4 => self.tempo,
            5 => self.tempo0,
            6 => self.step_acc,
            7 => self.beat,
            8 => self.gtrans,
            _ => self.scratch[usize::from(i & 7)],
        }
    }

    /// `cs:0x23 + i` write — matching [`cell_get`][Self::`cell_get`].
    fn cell_set(&mut self, i: u8, v: u8) {
        match i {
            0 => self.mus = v,
            1 => self.sfx_on = v,
            2 => self.pace = v,
            3 => self.mvol = v,
            4 => self.tempo = v,
            5 => self.tempo0 = v,
            6 => self.step_acc = v,
            7 => self.beat = v,
            8 => self.gtrans = v,
            _ => self.scratch[usize::from(i & 7)] = v,
        }
    }
}
