use super::{Mpu, Ports};

const DATA: u16 = 0x330;
const STAT: u16 = 0x331;
const DSR: u8 = 0x40;
const DRR: u8 = 0x80;

impl<P: Ports> Mpu<P> {
    /// Send one MIDI data byte — file `0x350E`: wait for DSR on the
    /// status port, write `b` to `0x330`.
    pub fn send(&mut self, b: u8) {
        while self.ports.inb(STAT) & DSR != 0 {}
        self.ports.outb(DATA, b);
    }

    /// Command byte + ACK drain — file `0x34F5`: wait for DSR, write `b`
    /// to command port `0x331` under `cli`, then poll DRR and read the
    /// `0x330` response (the `0xFE` ACK; returned).
    pub fn command(&mut self, b: u8) -> u8 {
        while self.ports.inb(STAT) & DSR != 0 {}
        self.ports.outb(STAT, b);
        while self.ports.inb(STAT) & DRR != 0 {}
        self.ports.inb(DATA)
    }

    /// Read one data byte — file `0x351F`: poll DRR on `0x331`, then
    /// `in` from `0x330`.
    pub fn read(&mut self) -> u8 {
        while self.ports.inb(STAT) & DRR != 0 {}
        self.ports.inb(DATA)
    }
}
