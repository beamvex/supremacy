use super::{Mpu, Ports};

const DATA: u16 = 0x330;
const STAT: u16 = 0x331;
const DSR: u8 = 0x40;
const DRR: u8 = 0x80;

impl<P: Ports> Mpu<P> {
    /// MPU-401 reset + detect — file `0x3470`. Polls DSR for up to
    /// `0xFFFF` reads, issues `0xFF` (reset) to the command port under
    /// `cli`, then waits for DRR and reads the data port expecting the
    /// `0xFE` ACK — the asm `cmp`s the byte and stores `0xFE` or `0`
    /// to `[0x1C02]` (`self.ack`). Returns `true` only for `0xFE` —
    /// any other reply (or a timeout's `0`) fails the detect.
    pub fn reset(&mut self) -> bool {
        for _ in 0..0xFFFF {
            if self.ports.inb(STAT) & DSR == 0 {
                self.ports.outb(STAT, 0xFF);
                self.ack = if self.read_ack() == 0xFE { 0xFE } else { 0 };
                return self.ack != 0;
            }
        }
        false
    }

    /// Poll DRR for up to `0xFFFF` reads, then read the data port —
    /// the second wait loop + `in 0x330` of `0x3470` (`0` on timeout).
    fn read_ack(&mut self) -> u8 {
        for _ in 0..0xFFFF {
            if self.ports.inb(STAT) & DRR == 0 {
                return self.ports.inb(DATA);
            }
        }
        0
    }
}
