use super::table::Palette;

impl Palette {
    /// 8-bit RGB for palette index `i`.
    #[must_use]
    pub fn rgb(&self, i: usize) -> [u8; 3] {
        self.colors[i]
    }
}
