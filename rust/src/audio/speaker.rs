use super::Ports;

/// PC speaker via PIT channel 2 and the port-`0x61` gate — the
/// `note`/`silence` halves of the PC driver (file `0x0C00` module),
/// frequency program at `0x0E9F`.
///
/// `last` mirrors the driver's `[0x2E]` cache: the PIT is only
/// reprogrammed when the divisor changes.
pub struct Speaker<P: Ports> {
    /// Port bus.
    pub ports: P,
    /// `[0x2E]` — last divisor written to channel 2.
    pub last: u16,
}

impl<P: Ports> Speaker<P> {
    /// Wrap a port bus; `[0x2E]` starts cleared by the driver's init.
    pub fn new(ports: P) -> Self {
        Self { ports, last: 0 }
    }
}
