//! Ring-buffer geometry shared by all four decoder variants.

/// Ring size `N` — the `SS:0x0000` buffer masked by `and bp,0xFFF`.
pub(crate) const N: usize = 0x1000;

/// Initial ring write cursor — `mov bp,0xFEE` (`N - F`).
pub(crate) const R_INIT: usize = 0xFEE;

/// Flag-register sentinel bit — `test dx,0x100` after `shr dx,1`.
pub(crate) const SENTINEL: u16 = 0x100;

/// Reload value for the flag register's high byte — `mov dh,0xFF`.
pub(crate) const FLAG_HI: u16 = 0xFF00;
