//! The game runner — owns the `Vm`/`State`/`Rng`/`Host` bundle and steps
//! the two frame loops the asm alternates between: the `cs:0x2CC5` UI
//! shell (`shell_step`, uimode 0) and the `cs:0x395B` galaxy loop
//! (`frame_step`, exits back to the shell on `[0x9CDA] & 2`).
//!
//! Construction ports the init tail the asm runs before `shell_enter`:
//! `Vm::from_image` (the shipped `ds` image), [`select_galaxy`] +
//! [`new_game`] (record array + resources), then `shell_enter` itself.

use crate::args::Video;
use crate::assets::AssetSet;
use crate::game::{
    frame_step, new_game, select_galaxy, shell_enter, shell_step, Ctx, Frame, Rng, State, Vm,
};
use crate::palette::Palette;
use crate::platform::DosFiles;
use crate::video::Screen;

use super::Host;

/// Which asm loop the runner is in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Phase {
    /// `cs:0x2CC5` — the uimode-0 menu screen.
    Shell,
    /// `cs:0x395B` — the galaxy screen.
    Galaxy,
}

/// The whole playable bundle.
pub struct Game {
    /// `ds:` low/hi regions (from the exe image).
    pub vm: Vm,
    /// The `0x7D39` saveable block.
    pub st: State,
    /// The `0x55B2` LCG.
    pub rng: Rng,
    /// Screen/input host.
    pub host: Host,
    /// The `int 21h` file layer — the save/load routines' `C:` drive.
    pub files: Box<dyn DosFiles>,
    /// Active loop.
    pub phase: Phase,
}

impl Game {
    /// Fresh game — galaxy `preset` (0..3), PRNG `seed`, `shell_enter`ed
    /// over a fresh [`Host`] for `video` mode, saving/loading through
    /// `files`.
    #[must_use]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        img: Vec<u8>,
        set: AssetSet,
        pal: Palette,
        video: Video,
        k_mode: bool,
        preset: usize,
        seed: u32,
        files: Box<dyn DosFiles>,
    ) -> Self {
        let mut g = Self {
            vm: Vm::from_image(&img),
            st: State::new(),
            rng: Rng::new(seed),
            host: Host::new(Screen::new(video), pal, set, img, k_mode),
            files,
            phase: Phase::Shell,
        };
        select_galaxy(&mut g.st, preset);
        // The asm flow is pick → `0x844B` snapshot → `0x8432` restore
        // inside `new_game`; snapshot here so the preset survives it.
        g.vm.stage_save(&g.st);
        new_game(&mut g.vm, &mut g.st, &mut g.rng);
        let mut c = g.ctx();
        shell_enter(&mut c);
        g
    }

    /// `Ctx` over this runner's parts — `&mut` fields, so callers take a
    /// short-lived borrow.
    pub fn ctx(&mut self) -> Ctx<'_> {
        Ctx {
            vm: &mut self.vm,
            st: &mut self.st,
            rng: &mut self.rng,
            host: &mut self.host,
            files: &mut *self.files,
        }
    }

    /// One frame — mirror inputs into `ds:`, then step the active loop.
    /// `Galaxy → Shell` transitions are internal; `Shell → Galaxy` runs
    /// through the `0x305B` new-game menu action (`init::act_newgame`).
    pub fn step(&mut self) {
        self.host.pump.write(&mut self.vm, &mut self.st);
        match self.phase {
            Phase::Shell => {
                let mut c = self.ctx();
                shell_step(&mut c);
            }
            Phase::Galaxy => {
                if let Frame::Shell = frame_step(&mut self.ctx()) {
                    self.phase = Phase::Shell;
                }
            }
        }
    }

    /// Force the galaxy loop — the new-game menu actions that `jmp
    /// 0x395B` aren't decoded yet; this jumps the port ahead for testing.
    pub fn enter_galaxy(&mut self) {
        self.phase = Phase::Galaxy;
    }
}
