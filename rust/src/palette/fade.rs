use super::table::Palette;

impl Palette {
    /// `0x2F540` `pal_fade` (`[0x128E]`): the asm stages row `dx`'s 32
    /// colours then decrements the whole block to black over vsync'd
    /// rounds before `pal_adj` — the port lands on the end state (regs
    /// `0..=31` black, `0x21..` refreshed through the shade).
    pub fn pal_fade(&mut self, img: &[u8], off: u8) {
        for c in &mut self.colors[0..=0x1F] {
            *c = [0; 3];
        }
        self.pal_adj(img, off);
    }
}
