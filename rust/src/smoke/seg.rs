//! Segment selectors — the `.scr` script's way of naming the two
//! segments a script can address symbolically plus absolute
//! real-mode segments.

/// Where a `dump`/`pokew`/`waitcell` op addresses memory.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Seg {
    /// `ds` — the game's data segment (dgroup).
    Ds,
    /// `drv` — the driver/template segment of the exe image: holds the
    /// `0x8B72` key LIFO and the `0x55A2` RNG state.
    Drv,
    /// Absolute segment, e.g. `0xA000` for VRAM.
    Abs(u16),
}

impl Seg {
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
