use super::table::Palette;

impl Palette {
    /// Standard 16-colour EGA/Tandy palette — what the reference renderer
    /// (`tools/extract_png.py::EGA_PAL`) uses for `EGA*`/`TGA*` sets.
    #[must_use]
    pub fn ega16() -> Self {
        const C: [(u8, u8, u8); 16] = [
            (0, 0, 0),
            (0, 0, 170),
            (0, 170, 0),
            (0, 170, 170),
            (170, 0, 0),
            (170, 0, 170),
            (170, 85, 0),
            (170, 170, 170),
            (85, 85, 85),
            (85, 85, 255),
            (85, 255, 85),
            (85, 255, 255),
            (255, 85, 85),
            (255, 85, 255),
            (255, 255, 85),
            (255, 255, 255),
        ];
        let mut colors = [[0; 3]; 256];
        for (i, c) in C.iter().enumerate() {
            colors[i] = [c.0, c.1, c.2];
        }
        Self { colors }
    }

    /// CGA palette 1 high-intensity — black/cyan/magenta/white, selected
    /// by the game's `int 10/AH=0B BH=1 BL=0` at file `0x31FE3`.
    #[must_use]
    pub fn cga() -> Self {
        let mut colors = [[0; 3]; 256];
        for (i, c) in [(0, 0, 0), (85, 255, 255), (255, 85, 255), (255, 255, 255)]
            .iter()
            .enumerate()
        {
            colors[i] = [c.0, c.1, c.2];
        }
        Self { colors }
    }
}
