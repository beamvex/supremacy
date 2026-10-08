//! Planet ops for the event VM — `cs:0x790A`, `0x7980`/`0x798E`/
//! `0x799C`, `0x7A11`, `0x7BF4`, `0x7C05`.

use super::consts::{DIRTY0, DIRTY1, REC_SEL};
use super::rec::{F_ENERGY, F_FOOD, F_FUEL, F_KIND, F_OWNER, F_POP};
use super::text;
use super::vhost::Call;
use super::vm::Flow;
use super::vops::{kind_of, set_bits, uimode, unlink_ships, Ctx, R_SEL};

/// `cs:0x790A` — destroy the selected planet (`[0x81EE]`): unlink its
/// ships, `kind = 4`, `owner = 0`, dirty `0x8`, panel refresh in UI
/// mode 0. Suspends on the `[0x914C]`/`[0x917C]` abort paths.
pub fn kill_sel(c: &mut Ctx) -> Flow {
    let sel = c.vm.r16(c.st, R_SEL);
    if !unlink_ships(c, sel) {
        return Flow::Ret;
    }
    let m = uimode(c);
    if m != 0 && m != 5 && sel == c.vm.r16(c.st, u16::try_from(REC_SEL).unwrap_or(0)) {
        return Flow::Ret;
    }
    c.vm.w16(c.st, sel + u16::try_from(F_KIND).unwrap_or(0), 4);
    c.vm.w8(c.st, sel + u16::try_from(F_OWNER).unwrap_or(0), 0);
    set_bits(c, u16::try_from(DIRTY0).unwrap_or(0), 0x8);
    if m == 0 {
        c.host.svc(Call::PlanetPanel);
    }
    Flow::Cont
}

/// `cs:0x7980`/`0x798E`/`0x799C` — suspend unless the selected record's
/// kind is `k`.
pub fn wait_kind(c: &mut Ctx, k: u16) -> Flow {
    let sel = c.vm.r16(c.st, R_SEL);
    if kind_of(c, sel) == k {
        Flow::Cont
    } else {
        Flow::Ret
    }
}

/// `cs:0x7A11` — the levy: prints `0x5D33`, then on the faction record
/// pulls up to `0x186` food and dumps the excess above `0x384` into
/// `+0x32`. Sets the `cs:0xE713` modal flag first; suspends after.
pub fn levy(c: &mut Ctx) -> Flow {
    c.vm.modal = true;
    text::enqueue(c, 0x5D33);
    let rec = super::vops::faction_rec(c);
    let food = c.vm.r16(c.st, rec + u16::try_from(F_FOOD).unwrap_or(0));
    let left = food - food.min(0x186);
    c.vm.w16(c.st, rec + u16::try_from(F_FOOD).unwrap_or(0), left);
    c.vm.w16(
        c.st,
        rec + u16::try_from(F_FUEL).unwrap_or(0),
        left.saturating_sub(0x384),
    );
    Flow::Ret
}

/// `cs:0x7BF4` — depopulate the selected record (`+0x2A = 0`, dirty
/// `0x20`).
pub fn depopulate(c: &mut Ctx) -> Flow {
    let sel = c.vm.r16(c.st, R_SEL);
    c.vm.w16(c.st, sel + u16::try_from(F_POP).unwrap_or(0), 0);
    set_bits(c, u16::try_from(DIRTY0).unwrap_or(0), 0x20);
    Flow::Cont
}

/// `cs:0x7C05` — charge the selected record (`+0x34 = 0x74CC`, dirty
/// `0x2` in `[0x9146]`).
pub fn charge(c: &mut Ctx) -> Flow {
    let sel = c.vm.r16(c.st, R_SEL);
    c.vm.w16(c.st, sel + u16::try_from(F_ENERGY).unwrap_or(0), 0x74CC);
    set_bits(c, u16::try_from(DIRTY1).unwrap_or(0), 2);
    Flow::Cont
}
