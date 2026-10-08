use crate::args::Video;

/// A mode's VRAM span as a flat buffer — what the decoders write through
/// `es:di`.
///
/// MCG/CGA/TGA are one region (`0xFA00`/`0x4000`/`0x8000` bytes). EGA is
/// the four `0x1F40`-byte bitplanes laid out consecutively: in hardware the
/// draw routine reprograms the map-mask register between passes, so each
/// plane stream lands at the same `di` in a different plane.
pub struct Screen {
    /// Mode this buffer is laid out for.
    pub mode: Video,
    /// VRAM contents, [`Video::vram_len`] bytes.
    pub buf: Vec<u8>,
}

impl Screen {
    /// Zeroed VRAM for `mode` (VRAM isn't cleared by `int 10` mode sets on
    /// real hardware, but the game repaints full screens anyway).
    #[must_use]
    pub fn new(mode: Video) -> Self {
        Self {
            mode,
            buf: vec![0; mode.vram_len()],
        }
    }
}
