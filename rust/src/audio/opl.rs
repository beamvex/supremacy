use super::Ports;

/// `AdLib` OPL2 register file behind ports `0x388` (address/status) and
/// `0x389` (data) — the AdLib/Roland driver module at file `0x2900`.
///
/// `P` is the host port bus; methods reproduce the driver's exact
/// write/detect sequences including the status-port read delays
/// (6 reads after address, 37 after data, 38 in the wait helper).
pub struct Opl<P: Ports> {
    /// Port bus the `in`/`out`s land on.
    pub ports: P,
}

impl<P: Ports> Opl<P> {
    /// Wrap a port bus.
    pub fn new(ports: P) -> Self {
        Self { ports }
    }
}
