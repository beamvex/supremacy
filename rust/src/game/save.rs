use super::State;
use crate::platform::DosFiles;

/// Save — port of file `0x383F3`: `create` (`0x3C00`, truncating), then
/// `open` write-only (`0x3D01`), `write` the whole state block
/// (`0x4000`, `cx = 0x1F8B` from `ds:0x7D39`), `close` (`0x3E00`).
///
/// The asm leaks the `create` handle and tests only the carry flag after
/// each call; here the leaked handle is closed and a short write counts
/// as failure. `false` mirrors a carry-flag error return.
impl State {
    /// Write the whole state block to savegame `name`.
    pub fn save<F: DosFiles + ?Sized>(&self, fs: &mut F, name: &str) -> bool {
        let Some(created) = fs.create(name) else {
            return false;
        };
        fs.close(created);
        let Some(h) = fs.open_write(name) else {
            return false;
        };
        let n = fs.write(h, &self.block[..]);
        fs.close(h);
        n == self.block.len()
    }
}
