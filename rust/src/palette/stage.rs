use super::shade::shade;
use super::table::Palette;

impl Palette {
    /// Stage `len` consecutive DAC registers at `reg` from `tab`'s
    /// colour triples `first..first + len`, each byte through
    /// [`shade`] (`gate` = its zero-component skip).
    pub(super) fn stage(
        &mut self,
        reg: usize,
        tab: &[u8],
        first: usize,
        len: usize,
        off: u8,
        gate: bool,
    ) {
        for (i, c) in tab
            .as_chunks::<3>()
            .0
            .iter()
            .skip(first)
            .take(len)
            .enumerate()
        {
            self.colors[reg + i] = c.map(|v| shade(v, off, gate));
        }
    }
}
