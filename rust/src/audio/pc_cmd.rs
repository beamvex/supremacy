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
    /// the `cs:0x52` table — `0` tick, `1` stop, `2`/`al` song,
    /// `3`/`al` sfx, `4` resume, `5`/`6` the `[0x457]` peek/poke (the
    /// sfx `alt` divisor's high byte; `ah ≥ 7` falls off the table
    /// into data, so the port ignores it like the asm's wild near-call
    /// would deserve). `ah ≥ 0x20` is the `[bx+0x23]` cell peek/poke
    /// (`0x20..0x30` read, `0x30..` write — the `0x8A1B`/`0x8AA7`
    /// wrappers' `0x32`/`0x33` hit `pace`/`tempo`). Returns the `al`
    /// the caller sees.
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
            5 => return self.sfx.alt.to_le_bytes()[1],
            6 => self.sfx.alt = self.sfx.alt & 0x00FF | i16::from(al) << 8,
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

    /// The `ah ≥ 0x20` cell path — file `0xD4C`/`0xD58`: `bl` had
    /// `0x30` folded in before the jump, so `0x20..0x30` reads
    /// `cs:0x23 + (ah−0x20)` and `0x30..` writes `cs:0x23 + (ah−0x30)`.
    fn cell_io(&mut self, ah: u8, al: u8) -> u8 {
        if ah >= 0x30 {
            self.cell_set(ah.wrapping_sub(0x30), al);
            al
        } else {
            self.cell_get(ah - 0x20)
        }
    }

    /// `cs:0x23 + i` read — the named cells, the `[0x2D]` pad, the
    /// `[0x2E]`/`[0x30]` word halves, then `cs:` bytes: `i=0x0F` lands
    /// on the `0x32` opcode table (the read path caps at `ah=0x2F`).
    fn cell_get(&self, i: u8) -> u8 {
        match i {
            0 => self.mus,
            1 => self.sfx_on,
            2 => self.pace,
            3 => self.tempo,
            4 => self.tempo0,
            5 => self.step_acc,
            6 => self.beat,
            7 => self.loaded,
            8 => self.gtrans,
            9 => self.mux_ror,
            10 => self.pad,
            11 => self.last.to_le_bytes()[0],
            12 => self.last.to_le_bytes()[1],
            13 => self.div0.to_le_bytes()[0],
            14 => self.div0.to_le_bytes()[1],
            _ => self.d8(0x23 + u16::from(i)),
        }
    }

    /// `cs:0x23 + i` write — matching [`cell_get`][Self::`cell_get`];
    /// `i ≥ 0x0F` would patch the module's code/tables — the port drops
    /// those on the scratch pad instead of self-modifying.
    fn cell_set(&mut self, i: u8, v: u8) {
        match i {
            0 => self.mus = v,
            1 => self.sfx_on = v,
            2 => self.pace = v,
            3 => self.tempo = v,
            4 => self.tempo0 = v,
            5 => self.step_acc = v,
            6 => self.beat = v,
            7 => self.loaded = v,
            8 => self.gtrans = v,
            9 => self.mux_ror = v,
            10 => self.pad = v,
            11 => self.last = self.last & 0xFF00 | u16::from(v),
            12 => self.last = self.last & 0x00FF | u16::from(v) << 8,
            13 => self.div0 = self.div0 & 0xFF00 | u16::from(v),
            14 => self.div0 = self.div0 & 0x00FF | u16::from(v) << 8,
            _ => self.scratch[usize::from(i & 7)] = v,
        }
    }
}
