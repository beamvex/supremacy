//! Game logic — the state block, save/load, and the new-game setup.
//!
//! Port of the routines documented in `decompiled/FUNCTIONS.md` § "Save /
//! state block" and § "New-game init":
//!
//! - [`State`]: the `0x1F8B`-byte saveable workspace — exactly the bytes the
//!   save routine dumps (`ds:0x7D39..0x9CC4`; `int 21` create/write at file
//!   `0x383F3`, open/read at `0x383CE`). Field offsets inside it are the
//!   real `ds:` addresses the asm uses.
//! - [`Rng`]: the `cs:0x55B2` PRNG (file `0x2E862`) — a 32-bit LCG
//!   `s = 13·s + 7` returning `hi16((b+1)·(lo^hi))`.
//! - `rec`: the `0x3A`-byte planet/faction record layout — base pointer
//!   `[0x9158]`, planet count / player index `[0x91B8]`, total `[0x91B6]`.
//! - [`new_game`]: `cs:0x9DAB` (file `0x3305B`) — fills the record array,
//!   names planets `LIFELESS!`, rolls the player faction's resources.
//! - [`select_galaxy`]: the size/difficulty preset pickers (file
//!   `0x36792`/`0x367B2`/`0x367CD`/`0x367E8`) that anchor the record array
//!   and set `[0x91B6]`/`[0x91B8]`/`[0x91E4]`.

mod battle;
mod consts;
mod day;
mod defence;
mod fleet;
mod init;
mod load;
mod mach;
mod machine;
mod mine;
mod preset;
mod rec;
mod rng;
mod save;
mod ship;
mod sim;
mod state;
mod tick;
mod vdir;
mod vend;
mod vev;
mod vhost;
mod vhud;
mod vimpl;
mod vm;
mod vmac;
mod vops;
mod vord;
mod vplan;
mod vup;

pub use battle::Battle;
pub use consts::*;
pub use day::day_tick;
pub use defence::defence_tick;
pub use fleet::{ship_tick, ShipOut};
pub use init::new_game;
pub use mach::*;
pub use machine::{machine_tick, MachOut};
pub use preset::{select_galaxy, Preset, PRESETS};
pub use rec::*;
pub use rng::Rng;
pub use ship::*;
pub use sim::sim_planet;
pub use state::State;
pub use tick::{tick_step, Tick};
pub use vend::{endgame, Ending};
pub use vhost::{Call, NullHost, VmHost};
pub use vm::{Vm, JT_BASE, TAB_BASE};
pub use vops::{run, Ctx};
