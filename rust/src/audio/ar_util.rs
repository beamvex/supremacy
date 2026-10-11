use super::{Ar, Mpu, Opl, Ports};

/// The `cs:` staging area's module-space base (`[u8; 0x8E]` copy).
pub(super) const STAGE: u16 = 0x1C1D;

impl<P: Ports> Ar<P> {
    /// `cs:` byte read on the module image.
    pub(super) fn d8(&self, ofs: u16) -> u8 {
        self.img.get(usize::from(ofs)).copied().unwrap_or(0)
    }

    /// `cs:` word read on the module image.
    pub(super) fn d16(&self, ofs: u16) -> u16 {
        u16::from(self.d8(ofs)) | u16::from(self.d8(ofs + 1)) << 8
    }

    /// Staging byte read — `ofs` is module space `0x1C1D..0x1CAB`
    /// (the `[0x1C86]`/`[0x1C98]` rhythm cells live here).
    pub(super) fn st8(&self, ofs: u16) -> u8 {
        self.stage
            .get(usize::from(ofs.wrapping_sub(STAGE)))
            .copied()
            .unwrap_or(0)
    }

    /// Staging byte write — `ofs` is module space `0x1C1D..0x1CAB`.
    pub(super) fn st8s(&mut self, ofs: u16, v: u8) {
        if let Some(b) = self.stage.get_mut(usize::from(ofs.wrapping_sub(STAGE))) {
            *b = v;
        }
    }

    /// Staging block — module space `ofs`, `n` bytes (`SysEx` sends).
    pub(super) fn stg(&self, ofs: u16, n: usize) -> &[u8] {
        let i = usize::from(ofs.wrapping_sub(STAGE));
        self.stage.get(i..i + n).unwrap_or(&[])
    }

    /// Song byte read — `para` is a record-relative paragraph
    /// (`tbl[]` values; `[0x1C03]`'s segment is the record base).
    pub(super) fn s8(&self, para: u16, ofs: u16) -> u8 {
        self.song
            .get(usize::from(para) * 16 + usize::from(ofs))
            .copied()
            .unwrap_or(0)
    }

    /// Song word read — `para`/`ofs` like [`s8`][Self::`s8`].
    pub(super) fn s16(&self, para: u16, ofs: u16) -> u16 {
        u16::from(self.s8(para, ofs)) | u16::from(self.s8(para, ofs + 1)) << 8
    }

    /// Song block read — absolute record offset `ofs`, `n` bytes.
    pub(super) fn rec(&self, ofs: usize, n: usize) -> &[u8] {
        self.song.get(ofs..ofs + n).unwrap_or(&[])
    }

    /// The header's `map` byte for sound `i` (`cs:0x168B + i`).
    pub(super) fn map(&self, i: u8) -> u8 {
        self.hdr[0x18 + usize::from(i)]
    }

    /// Header `progmap` `{prog, vol}` pair for 0-based program `i`
    /// (`cs:0x170B + i·2`).
    pub(super) fn progmap(&self, i: u8) -> (u8, u8) {
        let o = 0x98 + usize::from(i) * 2;
        (self.hdr[o], self.hdr[o + 1])
    }

    /// `0x32FD` — one OPL register write with the status-read delays.
    pub(super) fn opl_reg(&mut self, reg: u8, data: u8) {
        Opl {
            ports: &mut self.ports,
        }
        .write(reg, data);
    }

    /// `0x290E` — one MPU data byte (DSR poll then `0x330`).
    pub(super) fn midi(&mut self, b: u8) {
        Mpu {
            ports: &mut self.ports,
            ack: self.ack,
        }
        .send(b);
    }

    /// `0x32A4` — OPL2 presence `detect` (the timer-1 flag test) via
    /// [`Opl::detect`].
    pub(super) fn opl_detect(&mut self) -> bool {
        Opl {
            ports: &mut self.ports,
        }
        .detect()
    }

    /// `0x28F5` — MPU command byte + the one ack-drain read (the
    /// result is dropped — `rol` polls DRR then reads `0x330`).
    pub(super) fn mputil(&mut self, b: u8) -> u8 {
        let mut m = Mpu {
            ports: &mut self.ports,
            ack: self.ack,
        };
        m.command(b)
    }

    /// `0x28C8` — the counted-pair `SysEx` block send (head, checksummed
    /// body, `0xF7`) — [`Mpu::sysex`] shape: `[n][bytes][n][bytes]`.
    /// `0x28B9`'s settle loop after the send is a no-op on this model.
    pub(super) fn syx(&mut self, block: &[u8]) {
        let h = usize::from(block.first().copied().unwrap_or(0));
        let n = usize::from(block.get(1 + h).copied().unwrap_or(0));
        let head = block.get(1..1 + h).unwrap_or(&[]).to_vec();
        let body = block.get(2 + h..2 + h + n).unwrap_or(&[]).to_vec();
        Mpu {
            ports: &mut self.ports,
            ack: self.ack,
        }
        .sysex(&head, &body);
    }
}
