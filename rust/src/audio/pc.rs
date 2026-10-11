use super::Ports;

/// The PC-speaker `int 0x80` driver — the file-`0xC00` module port
/// (`decompiled/FUNCTIONS.md` § "PC speaker driver"). The ISR entry
/// (`sti; push…; ds=cs; call [ah*2+0x52]` for `ah < 0x20`, the
/// `[bx+0x23]` cell peek/poke for `ah ≥ 0x20`) becomes
/// [`Pc::dispatch`].
///
/// State mirrors the module's own `cs:` cells: `[0x23]`/`[0x24]` arm
/// music/sfx, `[0x25]`/`[0x26]` are the two accumulator steps,
/// `[0x29]`/`[0x28]` the accumulators, `[0x2A]` the song-loaded flag,
/// `[0x2B]` the global transpose, `[0x2C]` the channel-mux mode, and
/// `[0x457]` — the `ah=5`/`ah=6` peek/poke — is the [`Sfx::alt`] high
/// byte. Channel records `cs:0x11C`/`0x134` and the sfx block
/// `cs:0x450` are [`Chan`]/[`Sfx`].
pub struct Pc<P: Ports> {
    /// Port bus — the tick writes PIT `0x43`/`0x42` and gates `0x61`.
    pub ports: P,
    /// Module bytes — `cs:` reads map to file `0xC00 + ofs`; reads past
    /// `0xD50` land on the Tandy module exactly like the contiguous DOS
    /// load (the `al=0xFF` sfx lookup at `cs:0x1552` does this), so the
    /// slice covers both modules (`file 0xC00..0x2900`).
    img: Box<[u8]>,
    /// `[0x23]` — nonzero while the two music channels should run.
    pub(super) mus: u8,
    /// `[0x24]` — nonzero while the sfx sweep is armed.
    pub(super) sfx_on: u8,
    /// `[0x25]` — per-tick add to the `[0x29]` music-gate accumulator.
    pub(super) pace: u8,
    /// `[0x26]` — per-tick add to the `[0x28]` note-step accumulator
    /// (the tempo; `0x8E` stream op and `ah=2` load it).
    pub(super) tempo: u8,
    /// `[0x27]` — the second tempo copy `ah=2`/`0x8E` write.
    pub(super) tempo0: u8,
    /// `[0x28]` — note-step accumulator; overflow steps both channels.
    pub(super) step_acc: u8,
    /// `[0x29]` — music-gate accumulator; on overflow the whole music
    /// block is skipped for the tick.
    pub(super) beat: u8,
    /// `[0x2A]` — a song is resident (gates the `ah=4` resume).
    pub(super) loaded: u8,
    /// `[0x2B]` — global transpose added to every rendered note.
    pub(super) gtrans: u8,
    /// `[0x2C]` — nonzero: rotate `[0x13E]` (mux loops); zero: `shr` it
    /// (the pattern runs out and channel 0 takes over).
    pub(super) mux_ror: u8,
    /// `[0x2D]` — unused cell between `mux_ror` and the `[0x2E]`
    /// divisor cache; a real byte of storage for the cell path.
    pub(super) pad: u8,
    /// `[0x2E]` — last-programmed PIT divisor cache.
    pub(super) last: u16,
    /// `[0x30]` — channel-0 rendered-divisor scratch for the mux.
    pub(super) div0: u16,
    /// Pad for `[bx+0x23]` writes at `i ≥ 0x0F` — those would patch the
    /// module's own code/tables, which the immutable image can't model.
    pub(super) scratch: [u8; 8],
    /// The two channel records (`cs:0x11C`/`0x134`, `0x18` bytes each).
    pub(super) chans: [Chan; 2],
    /// The `cs:0x450` sfx sweep block.
    pub(super) sfx: Sfx,
}

/// One tracker channel record — `cs:0x11C`/`0x134`, `0x18` bytes.
///
/// Field comments carry the asm byte offsets; the meaning is the
/// derived one (FUNCTIONS.md § PC speaker driver).
#[derive(Clone, Copy, Default)]
pub struct Chan {
    /// `+0x00` — base of the stream-pointer list the channel walks.
    pub base: u16,
    /// `+0x02` — list cursor (opcode `0x85` advances +2, wraps to
    /// `base` on a `0` word, and stores it back here).
    pub list: u16,
    /// `+0x04` — stream read pointer.
    pub ptr: u16,
    /// `+0x06` — ticks left before the next stream byte decodes.
    pub wait: u8,
    /// `+0x07` — tick reload written by `0xE0..0xFF` duration bytes.
    pub reload: u8,
    /// `+0x08` — ch0: per-note fade budget; ch1: the `[0x13E]` mux
    /// pattern (opcodes `0x8A`/`0x8F` write it).
    pub mux: u8,
    /// `+0x09` — envelope-sequence base (into `cs:0x6B9`), picked by the
    /// `0xA0..0xDF` stream bytes through the `cs:0x5EA` table.
    pub env: u8,
    /// `+0x0A` — live copy of `+0x08` made per note: ch0's fade counter
    /// (`[0x126]` dec), ch1's rotating mux byte (`[0x13E]`).
    pub fade: u8,
    /// `+0x0B` — live envelope cursor (copy of `+0x09` made per note).
    pub seq: u8,
    /// `+0x0C` — sweep accumulator (added to the divisor when armed).
    pub sweep: u16,
    /// `+0x0E` — sweep tick counter.
    pub scount: u8,
    /// `+0x0F` — tremolo limit ×2.
    pub tlim: u8,
    /// `+0x10` — tremolo depth/step.
    pub tdep: u8,
    /// `+0x11` — tremolo current value.
    pub tcur: u8,
    /// `+0x12` — flags: bit0 per-tick toggle, bit1 half-mute gate, bit3
    /// sweep enable, bit4 tremolo enable, bit5 tremolo direction,
    /// bit6 sustain pitch-slide enable, bit7 slide direction.
    pub flags: u8,
    /// `+0x13` — sweep delta word.
    pub sdelta: u16,
    /// `+0x15` — base note index (the `< 0x80` stream byte).
    pub note: u8,
    /// `+0x16` — channel transpose.
    pub trans: u8,
}

/// The `cs:0x450` sfx block — a 15-byte record loaded by `ah=3` from
/// `cs:0x461 + id·0x0F`, then driven per tick at `0xE41`–`0xE97`.
#[derive(Clone, Copy, Default)]
pub struct Sfx {
    /// `+0x00`/`[0x450]` — beat countdown (reloads from `rcnt` on wrap,
    /// selecting the `alt` divisor).
    pub cnt: u8,
    /// `+0x01`/`[0x451]` — phase countdown (reloads from `rphase`,
    /// applies a `sel`-picked delta to `cur`).
    pub phase: u8,
    /// `+0x02`/`[0x452]` — per-tick divisor add.
    pub add: i16,
    /// `+0x04`/`[0x454]` — running divisor.
    pub cur: i16,
    /// `+0x06`/`[0x456]` — alternate divisor used on beat wrap; the
    /// `ah=5`/`ah=6` `[0x457]` peek/poke aliases its high byte.
    pub alt: i16,
    /// `+0x08`/`[0x458]` — phase delta A (`sel` carry).
    pub da: i16,
    /// `+0x0A`/`[0x45A]` — phase delta B (no carry).
    pub db: i16,
    /// `+0x0C`/`[0x45C]` — delta-select rotate flags.
    pub sel: u8,
    /// `+0x0D`/`[0x45D]` — "sfx wins" rotate: carry keeps the music
    /// divisor, clear replaces it with the sfx divisor.
    pub apply: u8,
    /// `+0x0E`/`[0x45E]` — end countdown; `0` = runs forever, a
    /// counted-down `0` disarms the sweep.
    pub end: u8,
    /// `+0x0F`/`[0x45F]` — `cnt` reload (load copies it into `cnt`).
    pub rcnt: u8,
    /// `+0x10`/`[0x460]` — `phase` reload.
    pub rphase: u8,
}

impl<P: Ports> Pc<P> {
    /// Driver over `ports`; `img` is the unpacked exe (the module bytes
    /// `file 0xC00..0x2900` are copied out — P and T are contiguous in
    /// the load image, so stray reads resolve like the real load).
    #[must_use]
    pub fn new(ports: P, img: &[u8]) -> Self {
        Self {
            ports,
            img: img.get(0xC00..0x2900).unwrap_or(&[]).into(),
            mus: 0,
            sfx_on: 0,
            pace: 0,
            tempo: 0,
            tempo0: 0,
            step_acc: 0,
            beat: 0,
            loaded: 0,
            gtrans: 0,
            mux_ror: 0,
            pad: 0,
            last: 0,
            div0: 0,
            scratch: [0; 8],
            chans: [Chan::default(); 2],
            sfx: Sfx::default(),
        }
    }

    /// Driver `cs:` byte — `img[0xC00 + ofs]`; out-of-module reads
    /// (Tandy spill) return real bytes while the slice covers them,
    /// `0` beyond.
    #[must_use]
    pub fn d8(&self, ofs: u16) -> u8 {
        self.img.get(usize::from(ofs)).copied().unwrap_or(0)
    }

    /// Driver `cs:` word (little-endian).
    #[must_use]
    pub fn d16(&self, ofs: u16) -> u16 {
        u16::from(self.d8(ofs)) | u16::from(self.d8(ofs.wrapping_add(1))) << 8
    }
}
