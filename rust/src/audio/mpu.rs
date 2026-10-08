use super::Ports;

/// Roland MPU-401 in UART mode — ports `0x330` (data) / `0x331`
/// (status+command), file `0x3470` onwards in the AdLib/Roland driver.
///
/// Status bits: `0x40` = DSR (set while the write buffer is full —
/// poll until clear before writing), `0x80` = DRR (set while no data is
/// waiting — poll until clear before reading).
pub struct Mpu<P: Ports> {
    /// Port bus.
    pub ports: P,
    /// Last ACK byte read after a reset — the asm stores it at
    /// `[0x1C02]`; `0xFE` marks a live MPU-401.
    pub ack: u8,
}

impl<P: Ports> Mpu<P> {
    /// Wrap a port bus.
    pub fn new(ports: P) -> Self {
        Self { ports, ack: 0 }
    }
}
