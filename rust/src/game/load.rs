use super::State;
use crate::platform::DosFiles;

/// Load — port of file `0x383CE`: `open` read-only (`0x3D00`), `read`
/// the whole state block (`0x3F00`, `cx = 0x1F8B` into `ds:0x7D39`),
/// `close` (`0x3E00`). `false` mirrors the carry-flag error returns.
impl State {
    /// Replace the block with the contents of savegame `name`.
    pub fn load<F: DosFiles + ?Sized>(&mut self, fs: &mut F, name: &str) -> bool {
        let Some(h) = fs.open(name) else { return false };
        let n = fs.read(h, &mut self.block[..]);
        fs.close(h);
        n == self.block.len()
    }
}
