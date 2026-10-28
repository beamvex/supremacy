//! The detail screen's `cs:0x2759` meter sweep — the `[0x1298]`
//! region bracket, the `[0x91C4]` cap ramp to `0x46`, and the eight
//! `[0x1276]` bar edges over the planet stocks and the docked
//! machine's loads (file `0x32759`–`0x32909`).

use super::super::consts::{REC_CUR, SEL_MACH};
use super::super::oreg;
use super::super::rec::{F_ENERGY, F_FOOD, F_FUEL, F_MINERALS};
use super::super::vhost::Call;
use super::super::vops::Ctx;
use super::detail::ANIM;
use super::dfield::SWEEP;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `cs:0x2759` — the region push, the `[0x91C4]` cap ramp, then a
/// `[0x1276]` edge per planet stock (`/0xFA`, colour `4`) and —
/// `ANIM`-gated — per machine load (`/0x19`, colour `0x10`), closed
/// by `0xA479` (file `0x32759`–`0x32909`).
pub(super) fn meters(c: &mut Ctx) {
    oreg::push(c, 0xB5, 0x13F, 0, 0xA0);
    let n = c.vm.r16(c.st, SWEEP);
    if n != 0x46 {
        c.vm.w16(c.st, SWEEP, n + 1);
    }
    let cap = c.vm.r16(c.st, SWEEP);
    let rec = c.vm.r16(c.st, a!(REC_CUR));
    for (ax, f) in [
        (0x18u16, a!(F_FOOD)),
        (0x1C, a!(F_MINERALS)),
        (0x20, a!(F_FUEL)),
        (0x24, a!(F_ENERGY)),
    ] {
        let h = (c.vm.r16(c.st, rec.wrapping_add(f)) / 0xFA) & 0xFF;
        c.host.svc(Call::Bar(ax, 4, h, cap));
    }
    bars_mach(c, cap);
    oreg::pop(c);
}

/// `0x32838`–`0x32905` — the `ANIM`-gated machine bars (columns
/// `0x19/0x1D/0x21/0x25`, `/0x19`, colour `0x10`).
fn bars_mach(c: &mut Ctx, cap: u16) {
    if c.vm.r8(c.st, ANIM) == 0 {
        return;
    }
    let m = c.vm.r16(c.st, a!(SEL_MACH));
    for (ax, f) in [(0x19u16, 0xAu16), (0x1D, 0xE), (0x21, 0x10), (0x25, 0x12)] {
        let h = (c.vm.r16(c.st, m + f) / 0x19) & 0xFF;
        c.host.svc(Call::Bar(ax, 0x10, h, cap));
    }
}
