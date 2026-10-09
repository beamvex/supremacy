//! Video subsystem — mode parameters, the VRAM framebuffer, and the
//! draw-image-by-index path (`ds:[0x125A]`).
//!
//! The per-mode constants come from the `0x115B`-byte parameter templates
//! (see `decompiled/FUNCTIONS.md`): byte 0 = `int 10` mode number, word 4 =
//! VRAM segment. Each mode has its own decoder pair with a distinct
//! framebuffer layout:
//!
//! | mode | int10 | VRAM | `di` for record (x,y) | row advance |
//! |---|---|---|---|---|
//! | MCG | `0x13` | `0xA000` | `y*0x140 + x` | `+0x140 − w` |
//! | EGA | `0x0D` | `0xA000` (4 planes) | `y*0x28 + x` | `+0x28 − w`, ×4 plane streams |
//! | CGA | `0x05` | `0xB800` | `(y>>1)*0x50 + (y&1)*0x2000 + x` | bank toggle `0x2000`, `+0x50` |
//! | TGA | `0x09` | `0xB800` | `(y>>2)*0xA0 + (y&3)*0x2000 + x` | `+0x2000` wrap `0x8000` `+0xA0` |
//!
//! [`Screen`] models the mode's VRAM span as a flat buffer — for EGA the
//! four `0x1F40`-byte planes are laid out consecutively.

mod banked;
mod di;
mod draw;
mod ega;
mod int10;
mod pixels;
mod put;
mod screen;
mod vram;

pub use banked::Banked;
pub use screen::Screen;
