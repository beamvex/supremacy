//! The scripted-event VM — `cs:0x773D` (file `0x3773D`).
//!
//! Memory model: the interpreter runs over the whole `ds` space, which
//! the port splits three ways — [`Vm::low`] is `ds:0x0000..0x7D39`
//! (event scripts, the `ds:0x50E3` op table, the `ds:0x5183` event
//! slots, message strings and VM flag cells), [`State`] covers
//! `ds:0x7D39..0x9CC4` (the saveable block), and [`Vm::hi`] covers the
//! rest (`ds:0x9CC4..=0xFFFF`, menu records and scratch).
//!
//! Dispatch: tick codes `0x38+2N .. 0x56+2N` index the 4-byte event
//! table at `ds:0x5183`; word 0 is the script `si`. Each opcode word is
//! an offset into the op table at `ds:0x50E3` (runtime-initialised —
//! the shipped image holds stale bytes there); the table yields a `cs:`
//! handler address. Handlers either `jmp 0x7758` (continue) or `ret`
//! (suspend). **`si` is never written back** — a suspended event simply
//! re-runs from its entry next tick, so scripts are guard+action rules
//! whose state lives in `ds` flag cells, not in the instruction pointer.

use super::consts::{REC_BASE, REC_STRIDE, STATE_LEN, STATE_OFS};
use super::keymap::KEYMAP_BYTES;
use super::state::State;

/// `ds:0x50E3` — op jump table base (file `0x37760`).
pub const JT_BASE: u16 = 0x50E3;
/// `ds:0x5183` — event table, 4-byte `{cur, base}` entries (file
/// `0x37750`).
pub const TAB_BASE: u16 = 0x5183;
/// `ds:` offset where [`Vm::hi`] starts (`0x9CC4`, just past the save
/// block).
pub const HI_OFS: usize = 0x9CC4;
/// `ds:` limit of [`Vm::low`] — the save-block base.
pub const LOW_LEN: usize = 0x7D39;
/// `ds:0x7CBD` — the 62-entry `{code, output}` keymap the `0xE66E`
/// line editor and the `0x32BA8` yes/no poll read (ends exactly at the
/// save block).
pub const KEYMAP: usize = 0x7CBD;
/// `ds:0x76BF` — the shipped keymap template (file `0x1835F`).
pub const KEYMAP_SRC: usize = 0x76BF;
/// Keymap entry count — 62 `{code, output}` byte pairs.
pub const KEYMAP_N: usize = 0x3E;

/// Flow a handler returns — mirrors `jmp 0x7758` vs `ret`.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Flow {
    /// `jmp 0x7758` — fetch the next op.
    Cont,
    /// `ret` — suspend the event until next dispatch.
    Ret,
}

/// Low/high `ds` memory for the event VM.
pub struct Vm {
    /// `ds:0x0000..0x7D39` — scripts, tables, strings, flag cells.
    pub low: Box<[u8]>,
    /// `ds:0x9CC4..=0xFFFF` — menu records and scratch past the save
    /// block.
    pub hi: Box<[u8]>,
    /// The `cs:0xE713` modal-dialog flag (a code-segment cell in the
    /// asm; kept here since the port has no code segment).
    pub modal: bool,
    /// The heap segment the `0x844B`/`0x8432` routines snapshot the
    /// save block into/out of (allocated at `ds:0x3829` in the asm).
    /// Seeded from the image so `0x8432` during new-game init restores
    /// the pristine shipped state.
    pub stage: Box<[u8; STATE_LEN]>,
}

impl Vm {
    /// Zeroed memory (the image ships real scripts — prefer
    /// [`Vm::from_image`]); the `ds:0x7CBD` keymap is installed from
    /// [`KEYMAP_BYTES`][super::keymap::KEYMAP_BYTES] either way.
    #[must_use]
    pub fn new() -> Self {
        let mut vm = Self {
            low: vec![0; LOW_LEN].into_boxed_slice(),
            hi: vec![0; 0x10000 - HI_OFS].into_boxed_slice(),
            modal: false,
            stage: Box::new([0; STATE_LEN]),
        };
        vm.install_keymap();
        vm
    }

    /// Load `ds:0x0000..0x9CC4` space from the unpacked exe — the low
    /// region ships scripts/strings at file `0x10CA0+`.
    #[must_use]
    pub fn from_image(img: &[u8]) -> Self {
        let mut vm = Self::new();
        let n = LOW_LEN.min(img.len().saturating_sub(0x10CA0));
        vm.low[..n].copy_from_slice(&img[0x10CA0..0x10CA0 + n]);
        let src = HI_OFS + 0x10CA0;
        let n = (0x10000 - HI_OFS).min(img.len().saturating_sub(src));
        vm.hi[..n].copy_from_slice(&img[src..src + n]);
        let lo = STATE_OFS + 0x10CA0;
        vm.stage.copy_from_slice(&img[lo..lo + STATE_LEN]);
        // The asm copies `ds:0x76BF` → `ds:0x7CBD` at init (the site
        // isn't located); use the image's own bytes when they're real.
        let n = KEYMAP_N * 2;
        if img.len() >= KEYMAP_SRC + 0x10CA0 + n {
            vm.low.copy_within(KEYMAP_SRC..KEYMAP_SRC + n, KEYMAP);
        }
        vm
    }

    /// Install the runtime keymap at `ds:0x7CBD` from the embedded
    /// shipped bytes — for `Vm::new` rigs with no image.
    pub fn install_keymap(&mut self) {
        self.low[KEYMAP..KEYMAP + KEYMAP_BYTES.len()].copy_from_slice(&KEYMAP_BYTES);
    }

    /// `0x844B` — snapshot the save block into the staging heap.
    pub fn stage_save(&mut self, st: &State) {
        self.stage.copy_from_slice(&st.block[..]);
    }

    /// `0x8432` — restore the save block from the staging heap.
    pub fn stage_load(&mut self, st: &mut State) {
        st.block.copy_from_slice(&self.stage[..]);
    }

    /// Byte at `ds:` offset `a` across the three regions.
    #[must_use]
    pub fn r8(&self, st: &State, a: u16) -> u8 {
        let a = usize::from(a);
        if a < LOW_LEN {
            self.low[a]
        } else if a < HI_OFS {
            st.byte(a)
        } else {
            self.hi[a - HI_OFS]
        }
    }

    /// Word at `ds:` offset `a` (little-endian, unaligned-ok).
    #[must_use]
    pub fn r16(&self, st: &State, a: u16) -> u16 {
        u16::from(self.r8(st, a)) | u16::from(self.r8(st, a.wrapping_add(1))) << 8
    }

    /// Write byte at `ds:` offset `a`.
    pub fn w8(&mut self, st: &mut State, a: u16, v: u8) {
        let i = usize::from(a);
        if i < LOW_LEN {
            self.low[i] = v;
        } else if i < HI_OFS {
            st.set_byte(i, v);
        } else {
            self.hi[i - HI_OFS] = v;
        }
    }

    /// Write word at `ds:` offset `a`.
    pub fn w16(&mut self, st: &mut State, a: u16, v: u16) {
        self.w8(st, a, u8::try_from(v & 0xFF).unwrap_or(0));
        self.w8(st, a.wrapping_add(1), u8::try_from(v >> 8).unwrap_or(0));
    }

    /// Record base for planet index `i` — `[0x9158] + i·0x3A`.
    #[must_use]
    pub fn rec_ofs(&self, st: &State, i: u16) -> u16 {
        self.r16(st, u16::try_from(REC_BASE).unwrap_or(0))
            .wrapping_add(i.wrapping_mul(u16::try_from(REC_STRIDE).unwrap_or(0)))
    }

    /// Fetch a 4-byte operand slot `{val, tag}` — `di=[si]; si+=4`.
    /// Returns the value word; the tag is consumed but unused.
    pub fn slot(&self, st: &State, si: &mut u16) -> u16 {
        let v = self.r16(st, *si);
        *si = si.wrapping_add(4);
        v
    }

    /// Fetch a bare immediate word — `bx=[si]; si+=2`.
    pub fn imm(&self, st: &State, si: &mut u16) -> u16 {
        let v = self.r16(st, *si);
        *si = si.wrapping_add(2);
        v
    }
}

impl Default for Vm {
    fn default() -> Self {
        Self::new()
    }
}
