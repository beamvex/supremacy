//! DOS platform layer — the hardware/OS services the game logic calls.
//!
//! Port of the service surface documented in `decompiled/FUNCTIONS.md`:
//!
//! - [`DosFiles`]: the centralised `int 21h` wrappers — `findfirst(0x4E)`,
//!   `open(0x3D)`, `read(0x3F)`, `close(0x3E)` — the only file I/O the game
//!   performs. [`HostFs`] implements them over `std::fs` rooted at a
//!   directory, standing in for the DOS cwd.
//! - [`Timer`]: the int-8 PIT hook — channel 0 reprogrammed to mode 3
//!   (`out 0x43=0x36`, divisor to `0x40`, file `0x353D`), whose ISR EOIs the
//!   PIC every pulse and chains to the saved BIOS handler every 3rd tick
//!   (file `0x35BF`).
//! - [`Keyboard`]: the int-9 scancode queue the driver feeds.
//! - [`Mouse`]: int-33h state plus the `K`-flag keyboard emulation mode
//!   (`[0x9CCD] = 1`).

mod files;
mod hostfs;
mod kbd_nav;
mod kbd_pop;
mod kbd_push;
mod keyboard;
mod mouse;
mod mouse_feed;
mod mouse_move;
mod pit_freq;
mod timer;
mod timer_new;
mod timer_program;
mod timer_tick;

pub use files::DosFiles;
pub use hostfs::HostFs;
pub use kbd_nav::{nav_dir, NavDir};
pub use keyboard::Keyboard;
pub use mouse::{Mouse, MouseEvent};
pub use timer::Timer;
