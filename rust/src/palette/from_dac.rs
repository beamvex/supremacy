use super::table::Palette;

/// Build a palette from the raw 6-bit DAC table (`768` bytes). Each
/// component scales `0..=63` to `0..=255`, matching the reference renderer
/// (`v * 255 // 63` in `tools/extract_png.py`).
#[must_use]
pub fn from_dac(raw: &[u8]) -> Palette {
    let mut colors = [[0u8; 3]; 256];
    for (i, c) in raw.as_chunks::<3>().0.iter().take(256).enumerate() {
        colors[i] = [0, 1, 2].map(|k| u8::try_from(u16::from(c[k]) * 255 / 63).unwrap_or_default());
    }
    Palette { colors }
}
