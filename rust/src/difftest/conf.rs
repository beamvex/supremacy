//! The DOSBox-X config the DOS run boots with — mirrors the shipped
//! `dosbox.conf` (`svga_s3`, 16 MiB) but headless-friendly and pointed at
//! the harness: mount the repo as `C:`, install `PROBE.COM`, run the
//! game with the chosen args, `exit` when it quits.

use std::path::Path;

/// Render the `.conf` for a run rooted at `root` (the repo dir that
/// holds `GAME/` and `difftest/`), launching `GAME.EXE <args>`
/// (e.g. `"M K P"` — MCGA, keyboard-emulated mouse, PC speaker).
#[must_use]
pub fn conf(root: &Path, args: &str) -> String {
    format!(
        "\
[sdl]
autolock=false
quit warning=false

[dosbox]
machine=svga_s3
memsize=16

[cpu]
core=normal
cycles=max

[render]
frameskip=0
aspect=false
scaler=none

[mixer]
nosound=true

[midi]
mpu401=none
device=none

[sblaster]
sbtype=none

[speaker]
pcspeaker=false
tandy=off
disney=false

[joystick]
joysticktype=none

[serial]
serial1=disabled
serial2=disabled
serial3=disabled
serial4=disabled

[ipx]
ipx=false

[autoexec]
mount c {root}
c:
cd \\difftest
mkdir out
PROBE.COM
cd \\game
GAME.EXE {args}
exit
",
        root = root.display(),
        args = args,
    )
}
