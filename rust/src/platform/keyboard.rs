/// Scancode queue fed by the int-9 ISR at file `0x31D74`
/// (`286B:8AC4` — the first far pointer in every mode template).
///
/// The ISR reads port `0x60`, ACKs via the `0x61` bit-7 toggle, EOIs the
/// PIC, then:
///
/// - tracks `Ctrl`/`Alt`/`Del` (scancodes `0x1D`/`0x38`/`0x53`, breaks
///   `0x9D`/`0xB8`/`0xD3`) in `[cs:0x8B6F..0x8B71]` — all three held does
///   `jmp 0xFFFF:0` (warm reboot), modelled here as `reboot`;
/// - latches the raw code into `[0x9CCC]` + sets `[0x9CCB]` (the
///   `pending` flag the K-mode mouse path polls);
/// - pushes the code onto a **10-entry LIFO stack** at `cs:0x8B72` with
///   count at `cs:0x8B7C` (`-1` empty, push skipped at 9).
pub struct Keyboard {
    pub(super) buf: [u8; 10],
    pub(super) count: i8,
    /// `[0x9CCC]` — last scancode latched by the ISR.
    pub last: u8,
    /// `[0x9CCB]` — set by the ISR, cleared by the consumer.
    pub pending: bool,
    /// `[cs:0x8B6F..0x8B71]` packed: bit 0 Ctrl, bit 1 Alt, bit 2 Del.
    pub(super) mods: u8,
    /// Set when Ctrl-Alt-Del are all held (`jmp 0xFFFF:0` in asm).
    pub reboot: bool,
}
