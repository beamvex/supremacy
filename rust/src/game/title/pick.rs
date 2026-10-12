//! `0x87D7`–`0x8849` — the scenario picker: the int-33 reset/handler/
//! range calls, the `ds:0x9D36` hotspot list install, the
//! `Image(2)`/`Present(0xB)` show, and the `{0x2CA9, 0xE93C, 0xA10F}`
//! frame loop. Its two hotspot actions are [`scenario_next`]
//! (`0x8926`) and [`scenario_start`] (`0x8984`).

use super::super::consts::{HOT_COUNT, HOT_LIST, HOT_SEL};
use super::super::vend::fx;
use super::super::vhost::Call;
use super::super::vops::{vsync, Ctx};
use super::super::{menu, preset};
use super::Title;

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// `[0x91E5]` — the galaxy-scenario index the chain records write and
/// `scenario_start` dispatches on (`vend::fx::scenario` reads it back
/// for the ending pages).
const SCENARIO: u16 = 0x91E5;
/// `[0x9C9F]` — the scenario-image chain cursor `scenario_next` walks.
const CHAIN_PTR: u16 = 0x9C9F;
/// `ds:0x9C62` — the live `{+0 img, +4 →[0x9209], +8 →[0x91E5]}`
/// chain head (runtime copy of the `ds:0x9662` template — the
/// `live = tpl + 0x600` idiom every hotspot list shares).
const CHAIN_LIVE: u16 = 0x9C62;
/// `ds:0x9662` — the shipped chain template.
const CHAIN_TPL: u16 = 0x9662;
/// `[0x9209]` — the present-variant selector the chain `+4` lands in
/// (`0` → `Present(0xB)`, else `Present(0xA)` + the `0x89B2` stub).
const PICK_VAR: u16 = 0x9209;
/// `ds:0x9D34` — the live picker selection word; the two `0x14`
/// hotspot records follow it (`ds:0x9D36`, tpl `ds:0x9734`).
const PICK_LIVE: u16 = 0x9D34;
/// `ds:0x9734` — the shipped picker list template.
const PICK_TPL: u16 = 0x9734;
/// Picker hotspot count.
const PICK_COUNT: u16 = 2;
/// `[0x91E7]` — the random-galaxy flag the RORN preset (`0x6792`) sets.
const RAND_FLAG: u16 = 0x91E7;
/// `ds:0x9D34` + two `0x14` records — the picker template's byte
/// length.
const PICK_LEN: u16 = 0x2A;

/// The `live = tpl + 0x600` copy every hotspot list shares — the asm
/// builds the live lists in its init stubs; the port runs it on the
/// attract entry instead (same shipped bytes; the mutated sel/`+0x10`
/// fields simply re-initialise).
pub(super) fn live_copy(c: &mut Ctx, tpl: u16, len: u16) {
    for i in 0..len {
        let b = c.vm.r8(c.st, tpl.wrapping_add(i));
        c.vm.w8(c.st, tpl.wrapping_add(0x600).wrapping_add(i), b);
    }
}

/// The two template structs the attract needs — the `ds:0x9662`
/// scenario chain and the `ds:0x9734` picker list — plus the chain
/// cursor + scenario index init `0x85B1` runs.
pub(super) fn live_lists(c: &mut Ctx) {
    let clen = chain_len(c);
    live_copy(c, CHAIN_TPL, clen);
    live_copy(c, PICK_TPL, PICK_LEN);
    c.vm.w16(c.st, CHAIN_PTR, CHAIN_LIVE);
    c.vm.w16(c.st, SCENARIO, 0);
}

/// Byte length of the shipped chain — records `0xC` wide up to and
/// including the `[si] == 0` terminator [`scenario_next`] wraps on.
fn chain_len(c: &mut Ctx) -> u16 {
    let mut si = CHAIN_TPL;
    while c.vm.r16(c.st, si) != 0 {
        si = si.wrapping_add(0xC);
    }
    si.wrapping_add(0xC).wrapping_sub(CHAIN_TPL)
}

/// `0x87D7`–`0x8840` — the picker install: mouse reset (`ax=0`), the
/// `0x286B:0x1143` user handler (`ax=0xC`, mask `1`), the `0..0xBC` /
/// `0..0x260` y/x ranges (`ax=8`/`7`), `0xA36A`, the `[0x147]`-segment
/// `0x8983` stub, list + sel + `[0x9209]` init, refresh,
/// `Image(2)`/`Present(0xB)`, `si=0x920B` `0x89B2` stub.
pub(super) fn install(c: &mut Ctx) {
    c.host.svc(Call::Mouse(0, 0, 0));
    c.host.svc(Call::Mouse(0xC, 1, 0x1143));
    c.host.svc(Call::Mouse(8, 0, 0xBC));
    c.host.svc(Call::Mouse(7, 0, 0x260));
    c.host.svc(Call::Mouse(0x14, 0x1F, 0));
    c.host.svc(Call::Native(0x8983));
    c.vm.w16(c.st, a!(HOT_LIST), PICK_LIVE + 2);
    c.vm.w16(c.st, a!(HOT_COUNT), PICK_COUNT);
    c.vm.w16(c.st, a!(HOT_SEL), c.vm.r16(c.st, PICK_LIVE));
    c.vm.w16(c.st, PICK_VAR, 0);
    fx::refresh(c);
    c.host.svc(Call::Image(2));
    c.host.svc_st(Call::Present(0xB), c.vm, c.st);
    c.host.svc(Call::Native(0x89B2));
}

impl Title {
    /// `0x8840` — one picker frame: `0x2CA9` vsync, the `0xE93C` PRNG
    /// churn and `0xA10F` (the menu service — its hotspot dispatches
    /// are the only way out, exactly like the asm).
    pub(super) fn pick(self, c: &mut Ctx) -> Option<Self> {
        vsync(c);
        _ = c.rng.next16();
        menu::service(c);
        (!c.vm.title_done).then_some(self)
    }
}

/// `cs:0x8926` — the "next scenario" hotspot (`+0xC` = `0x8926`, alias
/// `0xF676`): walk the `ds:0x9C62` chain record at `[0x9C9F]` —
/// `bx = [si]` image, `[0x9209] = [si+4]`, `[0x91E5] = [si+8]`,
/// `si += 0xC`, wrapping to the head on `bx == 0` — then refresh,
/// `Image(bx)` and `Present(0xA)` + `si = 0x920B` `0x89B2` when
/// `[0x9209] != 0`, else `Present(0xB)`.
pub fn scenario_next(c: &mut Ctx) {
    let (si, img) = chain_step(c);
    c.vm.w16(c.st, CHAIN_PTR, si);
    fx::refresh(c);
    c.host.svc(Call::Image(img));
    if c.vm.r16(c.st, PICK_VAR) != 0 {
        c.host.svc_st(Call::Present(0xA), c.vm, c.st);
        c.host.svc(Call::Native(0x89B2));
    } else {
        c.host.svc_st(Call::Present(0xB), c.vm, c.st);
    }
}

/// The chain walk — lands `[0x9209]`/`[0x91E5]` from the record, returns
/// the advanced `si` and the record's image (`0` on an empty chain;
/// the `0x100` bound keeps a corrupt link from hanging the frame).
fn chain_step(c: &mut Ctx) -> (u16, u16) {
    let si0 = c.vm.r16(c.st, CHAIN_PTR);
    let (mut si, mut img) = (si0, 0u16);
    for _ in 0..0x100 {
        img = c.vm.r16(c.st, si);
        c.vm.w16(c.st, PICK_VAR, c.vm.r16(c.st, si.wrapping_add(4)));
        c.vm.w16(c.st, SCENARIO, c.vm.r16(c.st, si.wrapping_add(8)));
        si = si.wrapping_add(0xC);
        if img != 0 {
            break;
        }
        si = CHAIN_LIVE;
        if si == si0 {
            break;
        }
    }
    (si, img)
}

/// `cs:0x8984` — the "start game" hotspot (`+0xC` = `0x8984`, alias
/// `0xF6D4`): `[0x9CA1] = 0xF`, refresh, then the `[0x91E5]` dispatch —
/// `0` → `0x67E8` (WOTOK), `1` → `0x67CD` (SMINE), `2` → `0x67B2`
/// (KRART), else `0x6792` (RORN, which also sets `[0x91E7] = 0xFF`).
/// The `0x6801` tail's `pop ax; ret` exits `0x85B1` outright — the
/// port models that as [`Vm::title_done`][super::super::Vm], which
/// [`title_step`][super::title_step] turns into the post tail.
pub fn scenario_start(c: &mut Ctx) {
    c.vm.w8(c.st, fx::SHADE, 0xF);
    fx::refresh(c);
    let s = c.vm.r16(c.st, SCENARIO);
    if s >= 3 {
        c.vm.w8(c.st, RAND_FLAG, 0xFF);
    }
    preset::select_galaxy(c.st, usize::from(3u16.saturating_sub(s.min(3))));
    c.vm.title_done = true;
}
