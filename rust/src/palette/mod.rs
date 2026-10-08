//! Master MCGA palette — the 256×3 DAC table at dgroup `0x4994` (file
//! `0x15034` in `GAME.unpacked.exe`), programmed via `int 10/AX=1012`.
//! Values are 6-bit VGA DAC components scaled to 8-bit RGB.

mod consts;
mod from_dac;
mod modes;
mod rgb;
mod table;

pub use consts::{DAC_LEN, DAC_OFS};
pub use from_dac::from_dac;
pub use table::Palette;
