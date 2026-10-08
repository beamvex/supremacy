/// 8253 PIT channel-0 model behind the int-8 hook (file `0x353D`–`0x35BF`).
///
/// `divisor` is the value the driver loads from `[CS:0x1C00]` and programs
/// with `al=0x36 → out 0x43` + lo/hi to `0x40` (mode 3, square wave; `0`
/// encodes 65536 like the real counter). `phase` is the ISR's `[CS:0x1BFE]`
/// down-counter: every pulse decrements it and every 3rd pulse it reloads
/// to 3 and chains to the saved BIOS int-8 — the game ticks at the
/// reprogrammed rate while DOS keeps 18.2 Hz.
pub struct Timer {
    /// PIT divisor last written to port `0x40` (`0` = 65536 = ~18.2 Hz).
    pub divisor: u16,
    /// `[CS:0x1BFE]` chain phase — ticks until the BIOS handler runs.
    pub phase: u8,
}
