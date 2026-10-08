use super::{Opl, Ports};

const ADDR: u16 = 0x388;
const DATA: u16 = 0x389;

impl<P: Ports> Opl<P> {
    /// OPL2 register write — file `0x3EFD` (`call 0x3efd` sites take
    /// `ax = reg<<8 | data`).
    ///
    /// Address to `0x388`, 6 status reads (~3.3 µs), data to `0x389`,
    /// then 37 status reads (~23 µs) — the chip's required write delays.
    pub fn write(&mut self, reg: u8, data: u8) {
        self.ports.outb(ADDR, reg);
        for _ in 0..6 {
            self.ports.inb(ADDR);
        }
        self.ports.outb(DATA, data);
        for _ in 0..37 {
            self.ports.inb(ADDR);
        }
    }

    /// Status-port delay loop — file `0x3F38` (`call 0x3f38` ×4 in
    /// `detect` spans ~80 µs for timer 1 to overflow).
    pub fn delay(&mut self) {
        for _ in 0..38 {
            self.ports.inb(ADDR);
        }
    }

    /// Raw status read of `0x388` (the timer flags live in bits 5–7).
    pub fn status(&mut self) -> u8 {
        self.ports.inb(ADDR)
    }
}
