use super::Ports;

/// The Tandy `int 0x80` driver — the file-`0x1950` module port
/// (image `0x0D50`). Same `ah`-dispatch contract as the PC-speaker
/// module (ISR `sti; push…; ds=cs; call [ah*2+0x52]` for `ah < 0x20`,
/// the `[bx+0x23]` cell peek/poke for `ah ≥ 0x20`) but a different
/// engine: four 0x1E-byte channel records at `cs:0xEC`/`0x10A`/
/// `0x128`/`0x146` rendered to an SN76489 on port `0xC0` — channels
/// 0–2 as tone voices, channel 3 into the noise register — plus a
/// two-voice sfx sweep driving channels 2/3 while `[0x24]` holds.
pub struct Tandy<P: Ports> {
    /// Port bus — the tick writes PSG latch/data bytes to `0xC0` and
    /// the `0x61` Tandy sound-enable bits.
    pub ports: P,
    /// Module bytes — `cs:` reads `map` to file `0x1950 + ofs`.
    pub(super) img: Box<[u8]>,
    /// `[0x23]` — nonzero while the four music channels should run.
    pub(super) mus: u8,
    /// `[0x24]` — `0xFF` while the sfx sweep is armed.
    pub(super) sfx_on: u8,
    /// `[0x25]` — per-tick add to the `[0x2A]` gate accumulator.
    pub(super) pace: u8,
    /// `[0x26]` — master volume folded into each channel's envelope.
    pub(super) mvol: u8,
    /// `[0x27]` — note-step add (tempo).
    pub(super) tempo: u8,
    /// `[0x28]` — the second tempo copy `ah=2`/`0x8C` write.
    pub(super) tempo0: u8,
    /// `[0x29]` — note-step accumulator; overflow steps the channels.
    pub(super) step_acc: u8,
    /// `[0x2A]` — gate accumulator; overflow skips the music block.
    pub(super) beat: u8,
    /// `[0x2B]` — global transpose (channels 0–2 only).
    pub(super) gtrans: u8,
    /// `[0x2C]` — current sfx record base (`cs:`).
    pub(super) srec: u16,
    /// `[0x2E]` — sfx beat countdown (record `+0x0A` reloads it).
    pub(super) bcnt: u8,
    /// `[0x2F]` — sfx phase countdown (record `+0x0B` reloads it).
    pub(super) pcnt: u8,
    /// `[0x30]` — sfx delta-`select` rotate (`+6` vs `+8` per phase).
    pub(super) sel: u8,
    /// `[0x31]` — sfx channel-`select` rotate (ch2 `0xD0` vs ch3 `0xF0`).
    pub(super) vsel: u8,
    /// `[0x32]` — sfx running divisor.
    pub(super) cur: u16,
    /// `[0x34]` — sfx end countdown.
    pub(super) end: u8,
    /// Pad for `[bx+0x23]` cell traffic outside the named cells.
    pub(super) scratch: [u8; 8],
    /// The four channel records at `cs:0xEC` + `0x1E`·i.
    pub(super) chans: [TChan; 4],
}

/// One Tandy channel record — `0x1E` bytes; the `+0x1C`/`+0x1D` latch
/// pair ships per channel (`0x90/0x80`, `0xB0/0xA0`, `0xD0/0xC0`,
/// `0xF0/0xE0`) and survives `ah=2`'s 14-word record clear.
#[derive(Clone, Copy, Default)]
pub struct TChan {
    /// `+0x00` — flags: bit3 sweep, bit6 slide, bit7 slide-down.
    pub flags: u8,
    /// `+0x02` — stream-pointer list base.
    pub base: u16,
    /// `+0x04` — list cursor.
    pub list: u16,
    /// `+0x06` — stream read pointer.
    pub ptr: u16,
    /// `+0x08` — ticks before the next stream byte decodes.
    pub wait: u8,
    /// `+0x09` — tick reload (`0xE0..` stream bytes set it).
    pub reload: u8,
    /// `+0x0A` — envelope selector (`0xC0..0xDF` bytes).
    pub esel: u8,
    /// `+0x0B` — envelope sequence cursor (into `cs:0x87C`).
    pub ecur: u8,
    /// `+0x0C` — envelope step countdown (`0` freezes it).
    pub ewait: u8,
    /// `+0x0D` — envelope-accumulated volume.
    pub evol: u8,
    /// `+0x0E` — sweep delta word (opcode `0x82` operand).
    pub sdelta: u16,
    /// `+0x10` — running pitch accumulator (divisor offset).
    pub run: u16,
    /// `+0x12` — sweep tick counter.
    pub scount: u8,
    /// `+0x13` — tremolo selector (opcode `0x81` operand).
    pub tsel: u8,
    /// `+0x14` — tremolo sequence cursor (into `cs:0x7E2`).
    pub tcur: u8,
    /// `+0x15` — tremolo step countdown.
    pub twait: u8,
    /// `+0x16` — base note index; `bit7` marks a tied note.
    pub note: u8,
    /// `+0x17` — channel transpose.
    pub trans: u8,
    /// `+0x18` — `run` reload written at each note (opcode `0x89`).
    pub nadd: u16,
    /// `+0x1A` — vibrato base index (`0xA0..0xBF` bytes, `cs:0x762`).
    pub inst: u8,
    /// `+0x1B` — vibrato cursor (into `cs:0x847`).
    pub vcur: u8,
    /// `+0x1C` — PSG volume latch.
    pub vlatch: u8,
    /// `+0x1D` — PSG tone/noise latch.
    pub tlatch: u8,
}

impl<P: Ports> Tandy<P> {
    /// Driver over `ports`; `img` is the unpacked exe — the module
    /// bytes `file 0x1950..0x2900` are copied out.
    #[must_use]
    pub fn new(ports: P, img: &[u8]) -> Self {
        let mut s = Self {
            ports,
            img: img.get(0x1950..0x2900).unwrap_or(&[]).into(),
            mus: 0,
            sfx_on: 0,
            pace: 0,
            mvol: 0,
            tempo: 0,
            tempo0: 0,
            step_acc: 0,
            beat: 0,
            gtrans: 0,
            srec: 0,
            bcnt: 0,
            pcnt: 0,
            sel: 0,
            vsel: 0,
            cur: 0,
            end: 0,
            scratch: [0; 8],
            chans: [TChan::default(); 4],
        };
        for (i, ch) in s.chans.iter_mut().enumerate() {
            ch.vlatch = 0x90 + u8::try_from(i).unwrap_or_default() * 0x20;
            ch.tlatch = 0x80 + u8::try_from(i).unwrap_or_default() * 0x20;
        }
        s
    }

    /// `cs:` byte read on the module image.
    pub(super) fn d8(&self, ofs: u16) -> u8 {
        self.img.get(usize::from(ofs)).copied().unwrap_or(0)
    }

    /// `cs:` word read on the module image.
    pub(super) fn d16(&self, ofs: u16) -> u16 {
        u16::from(self.d8(ofs)) | u16::from(self.d8(ofs + 1)) << 8
    }
}
