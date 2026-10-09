//! Segment selectors — the probe's encoding for the two segments it
//! captures at runtime plus absolute real-mode segments.

/// Where a `DUMP`/`POKEW`/`WAITCELL` op addresses memory.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Seg {
    /// `0xFFFF` — the game's data segment (dgroup), captured when the
    /// entry stub's `int 21/AH=1A DX=0x28` fires.
    Ds,
    /// `0xFFFE` — the early-resident driver segment (the int-8/9 ISR
    /// `cs:`): holds the `0x8B72` key LIFO and the `0x55A2` RNG state.
    Drv,
    /// Absolute segment, e.g. `0xA000` for VRAM.
    Abs(u16),
}

impl Seg {
    /// The probe's on-the-wire selector word.
    #[must_use]
    pub fn word(self) -> u16 {
        match self {
            Self::Ds => 0xFFFF,
            Self::Drv => 0xFFFE,
            Self::Abs(s) => s,
        }
    }

    /// Decode a selector word back.
    #[must_use]
    pub fn from_word(w: u16) -> Self {
        match w {
            0xFFFF => Self::Ds,
            0xFFFE => Self::Drv,
            s => Self::Abs(s),
        }
    }

    /// Parse `ds`, `drv`, or a hex segment.
    #[must_use]
    pub fn parse(t: &str) -> Option<Self> {
        match t.to_ascii_lowercase().as_str() {
            "ds" => Some(Self::Ds),
            "drv" => Some(Self::Drv),
            s => u16::from_str_radix(s.trim_start_matches("0x"), 16)
                .ok()
                .map(Self::Abs),
        }
    }
}
