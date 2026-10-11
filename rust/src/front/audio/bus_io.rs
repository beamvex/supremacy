use super::Bus;
use crate::audio::Ports;

impl Ports for Bus {
    /// `out dx,al` — the port map the drivers write: PIT control
    /// `0x43` (only channel-2 commands matter — ch0 is the `Timer`
    /// model's), `0x42` divisor latch, `0x61` speaker gate, the OPL
    /// pair `0x388`/`0x389`, and the MPU-401 `0x330`/`0x331`.
    fn outb(&mut self, port: u16, val: u8) {
        match port {
            0x43 => self.cmd(val),
            0x42 => self.latch(val),
            0x61 => self.gate = val,
            0x388 => self.opl_addr = val,
            0x389 => self.opl.write_reg(self.opl_addr, val),
            0x331 => self.mpu_cmd(val),
            // `0x330` UART data is accepted and dropped, like every
            // unmapped port.
            _ => {}
        }
    }

    /// `in al,dx` — `0x61` reads back the gate shadow; `0x331` reports
    /// the MPU status (write-ready; data-ready only while the reset
    /// ACK is pending); `0x330` pops that `0xFE` ACK; `0x388` reports
    /// a quiescent OPL status (no timer flags).
    fn inb(&mut self, port: u16) -> u8 {
        match port {
            0x61 => self.gate,
            0x330 => {
                if self.mpu_ack {
                    self.mpu_ack = false;
                    0xFE
                } else {
                    0
                }
            }
            0x331 => u8::from(!self.mpu_ack) << 7,
            // 0x388 OPL status reads 0 (no timer flags); the rest of
            // the map floats low.
            _ => 0,
        }
    }
}

impl Bus {
    /// `0x43` — mode/command; a channel-2 select resets the lo/hi
    /// latch order (`out 0x43=0xB6` then `0x42` lo,hi).
    fn cmd(&mut self, val: u8) {
        if val >> 6 == 2 {
            self.latch_hi = false;
        }
    }

    /// `0x42` — lobyte/hibyte staging of the channel-2 divisor.
    fn latch(&mut self, val: u8) {
        if self.latch_hi {
            self.divisor = u16::from(self.lo) | u16::from(val) << 8;
            self.latch_hi = false;
        } else {
            self.lo = val;
            self.latch_hi = true;
        }
    }

    /// `0x331` — MPU-401 command port; `0xFF` (reset) arms the `0xFE`
    /// acknowledge the driver's poll reads off `0x330`.
    fn mpu_cmd(&mut self, val: u8) {
        if val == 0xFF {
            self.mpu_ack = true;
        }
    }
}
