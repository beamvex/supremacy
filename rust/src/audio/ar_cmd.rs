use super::{Ar, Ports};

impl<P: Ports> Ar<P> {
    /// Export 0 (`cs:0x000`, file `0x2998`): `al` = 0 → the `0x2D3B`
    /// `AdLib` `init`, nonzero → `0x27CF` Roland; `bh` skips the `0x2952`
    /// timer `install`. Returns `al` = the `detect` result.
    pub fn init(&mut self, ax: u16) -> u8 {
        let ok = if ax.to_le_bytes()[0] == 0 {
            self.adlib_init()
        } else {
            self.roland_init()
        };
        if ok != 0 && ax & 0xFF00 == 0 {
            self.install();
        }
        ok
    }

    /// Export 1 (`cs:0x003`, `0x1DCC`): `al` != 0 re-inits Roland
    /// (`0x1F86`), else `AdLib` (`0x1FB3`) — each only when the module
    /// was live — then `mode = 0xFF` and `0x298C` removes the timer.
    pub fn shutdown(&mut self, al: u8) {
        if self.mode != 0xFF {
            if al != 0 {
                self.roland_init();
            } else {
                self.adlib_init();
            }
        }
        self.mode = 0xFF;
        self.uninstall();
    }

    /// Export 4 (`cs:0x00C`, `0x1DEE`): `select`-sound `cl` — all-off,
    /// `map` bound, default volume, vol-cache clear, an `AdLib` re-`init`,
    /// then `idx = cl`, queue cursor `0x118`, `+0xB`s `0xFF`, `en`.
    pub fn select(&mut self, cl: u8) {
        if self.mode == 0xFF {
            return;
        }
        self.all_off();
        self.map_bound();
        self.mvol = self.dvol;
        self.mvol_apply();
        self.volc = [0; 16];
        if self.mode == 0 {
            self.opl_init();
        }
        self.en = 0;
        self.div = 6;
        self.idx = cl;
        for c in self.chans.iter_mut().take(8) {
            c.last_prog = 0xFF;
        }
        self.fade = 0;
        self.cur = 0x118;
        self.pass = 0;
        self.en = 0xFF;
    }

    /// Export 5 (`cs:0x00F`, `0x1E7C`): `en = 0` + all-off when live.
    pub fn reset(&mut self) {
        if self.mode == 0xFF {
            return;
        }
        self.en = 0;
        self.all_off();
    }

    /// Export 6 (`cs:0x012`, `0x1EB7`): arm the `fader` at rate `cl`
    /// (doubled on `AdLib`) — gated on live + playing + nonzero vol.
    pub fn fade_set(&mut self, cl: u8) {
        if self.mode == 0xFF || self.mvol == 0 || self.en == 0 {
            return;
        }
        self.frate = if self.mode == 1 { cl } else { cl << 1 };
        self.fade = 0xFF;
    }

    /// Export 7 (`cs:0x015`, `0x1EA9`): the fade-running flag —
    /// `[0x2060] == 0xFF` returns in ZF to the far caller.
    #[must_use]
    pub fn fade_query(&self) -> bool {
        self.fade == 0xFF
    }
}
