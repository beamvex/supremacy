use super::{Mpu, Ports};

impl<P: Ports> Mpu<P> {
    /// Checksummed `SysEx` block send — file `0x34C8`. Sends `head`
    /// verbatim, then `body` keeping a running sum, then
    /// `(-sum) & 0x7F` as the Roland checksum, then `0xF7` (EOX).
    /// The asm's on-disk records are `[n][bytes]` counted pairs — the
    /// slices here are the two payloads; the counts are implied.
    pub fn sysex(&mut self, head: &[u8], body: &[u8]) {
        for &b in head {
            self.send(b);
        }
        let mut sum = 0u8;
        for &b in body {
            sum = sum.wrapping_add(b);
            self.send(b);
        }
        self.send(sum.wrapping_neg() & 0x7F);
        self.send(0xF7);
    }
}
