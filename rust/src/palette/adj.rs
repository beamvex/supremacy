use super::consts::WORK_OFS;
use super::table::Palette;

impl Palette {
    /// `0x2F1CF` `pal_adj` — the `0x2ED30`/`pal_fade` tail: DAC
    /// `0x21..=0xFF` from `ds:0x4095` colours `33..=255` through the
    /// `ds:[0x9CA1]` shade. The flush covers reg `0x20` too, whose
    /// `cs:0x8` buffer slot (`cs:0x68..0x6A`) is never written — it
    /// keeps its shipped zero, so reg `0x20` comes out black.
    pub fn pal_adj(&mut self, img: &[u8], off: u8) {
        self.colors[0x20] = [0; 3];
        self.stage(
            0x21,
            img.get(WORK_OFS..WORK_OFS + 0x300).unwrap_or(&[]),
            33,
            0xDF,
            off,
            true,
        );
    }
}
