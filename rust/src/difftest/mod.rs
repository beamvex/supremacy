//! The GAM-25 DOSBox-X differential-testing harness — runs the original
//! `GAME.EXE` under DOSBox-X and the port side-by-side on the same
//! scripted input, then compares guest state.
//!
//! DOS side: `difftest/probe.asm` (`PROBE.COM`, a TSR loaded before the
//! game) hooks int 21h/int 33h/int 8, discovers the game's `ds` and
//! driver segment at runtime, and executes a binary `SCRIPT.BIN` of
//! frame-scheduled ops — key/mouse injection that mirrors `Pump`, word
//! pokes, and memory/palette dumps written to the mounted drive.
//!
//! Port side: [`drive::run`] replays the same [`script`] ops against
//! [`crate::front::Game`] and writes matching dumps; [`boot`] can start
//! the port from a DOS `ds` dump so runs only need an aligned state,
//! not an aligned RNG seed. [`report`] diffs the two dump sets.
//!
//! Ops/timing: script ticks are the probe's int-8 chain calls
//! (~16.7 Hz); the port maps one tick to one `Game::step`. Transient
//! skew is absorbed by comparing at quiescent checkpoints; `WaitCell`
//! gives exact sync on a `ds` byte when a script needs it.

mod boot;
mod conf;
mod drive;
mod dump;
mod op;
mod report;
mod script;
mod seg;
mod snap;
mod spawn;

pub use boot::{boot_tick, from_dumps, have_boot, port_ds, read_boot};
pub use conf::conf;
pub use drive::run;
pub use dump::{dbg_parse, dump_name, pal_name, stat_name, Dbg, ARMED_NAME, DBG_LEN, DONE_NAME};
pub use op::{Op, Rec};
pub use report::{compare, Diff, Report};
pub use script::{parse, Script};
pub use seg::Seg;
pub use snap::{ds_image, snapshot};
pub use spawn::{find_dosbox, run_dos, DosError};
