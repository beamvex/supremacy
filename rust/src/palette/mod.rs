//! MCGA palette — the 256×3 master DAC table at dgroup `0x4994` (file
//! `0x15034` in `GAME.unpacked.exe`), plus the runtime ops the game
//! composes the DAC with: `[0x125E]` present (row table `ds:0x4394` →
//! regs `1..=31`, `dx == 6` = the full master), the `pal_adj` tail
//! (`ds:0x4095` colours `33..` → regs `0x21..=0xFF`), `pal_fade` and
//! the `cs:0x1384` all-black flush — all through the `ds:[0x9CA1]`
//! shade offset. Values are 6-bit VGA DAC components scaled to 8-bit
//! RGB.

mod adj;
mod consts;
mod fade;
mod from_dac;
mod modes;
mod present;
mod rgb;
mod shade;
mod stage;
mod table;

pub use consts::{DAC_LEN, DAC_OFS, ROWS_OFS, WORK_OFS};
pub use from_dac::from_dac;
pub use table::Palette;
