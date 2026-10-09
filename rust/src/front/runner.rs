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
    dispatch_action, frame_step, new_game, select_galaxy, shell_enter, shell_step, Ctx, Frame, Rng,
    State, Vm, UIMODE,
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
    /// Transitions are action-driven: a shell menu action sets
    /// `uimode = 5` (`cs:0x3906`) to hand the runner the galaxy loop;
    /// the galaxy exit edge `jmp 0x2CC5`s — [`shell_enter`] — back.
    pub fn step(&mut self) {
        self.host.pump.write(&mut self.vm, &mut self.st);
        match self.phase {
            Phase::Shell => {
                let mut c = self.ctx();
                shell_step(&mut c);
            }
            Phase::Galaxy => {
                if let Frame::Shell = frame_step(&mut self.ctx()) {
                    let mut c = self.ctx();
                    shell_enter(&mut c);
                }
            }
        }
        self.sync_phase();
    }

    /// Track the phase off `[0x91CD]` — the menu actions set `uimode`
    /// before `jmp`ing into their loop, so `5` means the galaxy loop
    /// is live and anything else is (or has returned to) the shell.
    fn sync_phase(&mut self) {
        let uimode = self.st.byte(UIMODE);
        self.phase = if uimode == 5 {
            Phase::Galaxy
        } else {
            Phase::Shell
        };
    }

    /// Enter the galaxy loop through the ported `cs:0x3906` menu
    /// action — the record-driven path the shell's hotspot list takes.
    pub fn enter_galaxy(&mut self) {
        let mut c = self.ctx();
        dispatch_action(&mut c, 0x3906);
        self.sync_phase();
    }
}
