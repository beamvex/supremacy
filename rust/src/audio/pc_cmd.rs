use super::{Pc, Ports};

/// SFX record base — `ah=3` loads 15 bytes from `cs:0x461 + id·0x0F`
/// (file `0xDBF`).
const SFX_TAB: u16 = 0x461;
/// SFX record stride (file `0xDC1`/`0xDC5`).
const SFX_LEN: u16 = 0x0F;
/// Song table — `ah=2` reads 5-byte `{tempo, list0, list1}` records
/// from `cs:0x6EB + id·5` (file `0xD61`/`0xDA9`).
const SONG_TAB: u16 = 0x6EB;

impl<P: Ports> Pc<P> {
    /// The `int 0x80` entry (file `0xC00`): `ah < 0x20` jumps through
    /// the `cs:0x52` table; `ah ≥ 0x20` is the `[bx+0x23]` cell
    /// peek/poke (`0x20..0x30` read, `0x30..` write — the `0x8A1B`/
    /// `0x8AA7` wrappers' `0x32`/`0x33` hit `pace`/`tempo`). Returns
    /// the `al` the caller sees.
    pub fn dispatch(&mut self, ah: u8, al: u8) -> u8 {
        if ah >= 0x20 {
            return self.cell_io(ah, al);
        }
        match ah {
            0 => self.tick(),
            1 => self.stop(),
            2 => self.song(al),
            3 => self.effect(al),
            4 => self.resume(),
            5 => return self.aux,
            6 => self.aux = al,
            _ => {}
        }
        al
    }

    /// `ah=1` — file `0xFA7`: disarm both engines.
    pub fn stop(&mut self) {
        self.sfx_on = 0;
        self.mus = 0;
    }

    /// `ah=2, al` — file `0xD61`: load song `al` — clear the shared
    /// cells and both channel records, take the tempo from
    /// `cs:0x6EB + al·5`, point each channel at its stream-pointer
    /// list, then arm (`dec`) `[0x23]`/`[0x2A]`.
    pub fn song(&mut self, al: u8) {
        let bx = u16::from(al) * 5;
        self.mus = 0;
        self.loaded = 0;
        self.beat = 0;
        self.gtrans = 0;
        self.mux_ror = 0;
        self.chans = [super::Chan::default(); 2];
        self.tempo = self.d8(bx + SONG_TAB);
        self.tempo0 = self.tempo;
        self.step_acc = 0xFF;
        for c in 0..2u16 {
            let list = self.d16(bx + SONG_TAB + 1 + c * 2);
            let ptr = self.d16(list);
            let ch = &mut self.chans[usize::from(c)];
            ch.base = list;
            ch.list = list;
            ch.ptr = ptr;
            ch.wait = 1;
        }
        self.mus = self.mus.wrapping_sub(1);
        self.loaded = self.loaded.wrapping_sub(1);
    }

    /// `ah=3, al` — file `0xDBF`: copy the 15-byte record at
    /// `cs:0x461 + al·0xF` into the `0x452` sweep block, seed the
    /// countdowns from its tail, then arm (`dec`) `[0x24]`.
    pub fn effect(&mut self, al: u8) {
        let bx = SFX_TAB + u16::from(al) * SFX_LEN;
        self.sfx_on = 0;
        self.sfx.add = self.d16(bx).cast_signed();
        self.sfx.cur = self.d16(bx + 2).cast_signed();
        self.sfx.alt = self.d16(bx + 4).cast_signed();
        self.sfx.da = self.d16(bx + 6).cast_signed();
        self.sfx.db = self.d16(bx + 8).cast_signed();
        self.sfx.sel = self.d8(bx + 0xA);
        self.sfx.apply = self.d8(bx + 0xB);
        self.sfx.end = self.d8(bx + 0xC);
        self.sfx.rcnt = self.d8(bx + 0xD);
        self.sfx.rphase = self.d8(bx + 0xE);
        self.sfx.cnt = self.sfx.rcnt;
        self.sfx.phase = self.sfx.rphase;
        self.sfx_on = self.sfx_on.wrapping_sub(1);
    }

    /// `ah=4` — file `0xFB0`: re-arm music if a song is resident.
    pub fn resume(&mut self) {
        if self.loaded != 0 {
            self.mus = 0xFF;
        }
    }

    /// The `ah ≥ 0x20` cell path — file `0xD4C`/`0xD58`.
    fn cell_io(&mut self, ah: u8, al: u8) -> u8 {
        if ah >= 0x30 {
            *self.cell(ah - 0x30) = al;
            al
        } else {
            *self.cell(ah - 0x20)
        }
    }

    /// `cs:0x23 + i` cell — the byte fields map to their `ds:`-cell
    /// namesakes; anything else lands on a scratch pad.
    fn cell(&mut self, i: u8) -> &mut u8 {
        match i {
            0 => &mut self.mus,
            1 => &mut self.sfx_on,
            2 => &mut self.pace,
            3 => &mut self.tempo,
            4 => &mut self.tempo0,
            5 => &mut self.step_acc,
            6 => &mut self.beat,
            7 => &mut self.loaded,
            8 => &mut self.gtrans,
            9 => &mut self.mux_ror,
            _ => &mut self.scratch[usize::from(i & 7)],
        }
    }
}
