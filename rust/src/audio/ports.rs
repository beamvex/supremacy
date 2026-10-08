/// Host-side port I/O — stands in for the `in`/`out` instructions the
/// drivers issue against `0x388`/`0x389` (OPL2), `0x330`/`0x331` (MPU-401)
/// and `0x40`–`0x43`/`0x61` (PIT + speaker gate).
///
/// `inb` returns whatever the device reports; the drivers poll status
/// bits with bounded retry budgets exactly like the asm (0xFFFF polls).
pub trait Ports {
    /// `out dx,al`.
    fn outb(&mut self, port: u16, val: u8);
    /// `in al,dx`.
    fn inb(&mut self, port: u16) -> u8;
}
