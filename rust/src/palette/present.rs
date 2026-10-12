use super::consts::{DAC_OFS, ROWS_OFS};
use super::table::Palette;

/// Row-table stride — 32 colours (the staged loop covers `1..=31`).
const ROW_LEN: usize = 0x60;

impl Palette {
    /// `[0x125E]` present (`0x2ED30`), `dx` = row: stage the row's
    /// colours `1..=31` of the `ds:0x4394` table into DAC `1..=31`
    /// through the `ds:[0x9CA1]` shade, then the `pal_adj` tail. The
    /// `dx == 6` branch stages all 255 colours of the `ds:0x4994`
    /// master instead (no `pal_adj`, no zero-gate). Reg 0's slot in
    /// the `cs:0x8` staging buffer is never written — it keeps its
    /// shipped zero, so the flush blacks reg 0.
    pub fn present(&mut self, img: &[u8], dx: u16, off: u8) {
        self.colors[0] = [0; 3];
        if dx == 6 {
            self.stage(
                1,
                img.get(DAC_OFS..DAC_OFS + 0x300).unwrap_or(&[]),
                1,
                0xFF,
                off,
                false,
            );
            return;
        }
        let at = ROWS_OFS + usize::from(dx) * ROW_LEN;
        self.stage(
            1,
            img.get(at..at + ROW_LEN).unwrap_or(&[]),
            1,
            0x1F,
            off,
            true,
        );
        self.pal_adj(img, off);
    }
}
