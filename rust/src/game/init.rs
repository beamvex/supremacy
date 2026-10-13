use super::cells::{MENU_COUNT, MENU_LIST, MENU_SEL};
use super::consts::{
    DIFFICULTY, HOT_COUNT, HOT_LIST, HOT_SEL, REC_BASE, REC_STRIDE, SEL_IDX, SND_ON, UIMODE,
};
use super::rec::{
    F_CREDITS, F_DEFENCE, F_DEFLVL, F_ENERGY, F_FOOD, F_FUEL, F_MINERALS, F_NAME, F_OWNER, F_POP,
    F_SERIAL, F_TROOPS, F_WORD0,
};
use super::vhost::Call;
use super::vm::Vm;
use super::vops::{vsync, Ctx};
use super::{selrec, shell, text, Rng, State};

/// `u16` casts for the `ds:` slot constants.
macro_rules! a {
    ($k:expr) => {
        u16::try_from($k).unwrap_or(0)
    };
}

/// Per-difficulty `F_TROOPS` parameter — `[0x91E4]` = 0/1/2 → these
/// words (file `0x330A4`–`0x330CE`).
const DIFF_PARAM: [u16; 3] = [0x1F77, 0x3A2F, 0x5811];
/// Per-difficulty `[0x9164]` initial selected-record index — 6/14/30
/// for 0/1/2.
const SEL_INIT: [u16; 3] = [0x06, 0x0E, 0x1E];
/// `LIFELESS!` — the 9-byte name stamped on uninhabited planets
/// (file `0x3315C`–`0x33170`).
const LIFELESS: [u8; 9] = *b"LIFELESS!";

/// New-game record init — the state writes of `cs:0x9DAB` (file
/// `0x3305B`–`0x331C2`). Opens with `0x8432` (file `0x3309D`) — the
/// staging-heap snapshot that captures the configured save block
/// (menu picks like `[0x91E4]` are in it because the preset runs
/// before this call; the `0x844B` restore direction is the one
/// `0x2F91` uses). Assumes
/// [`select_galaxy`][super::select_galaxy] already anchored the array
/// (`[0x9158]`) and set the counts; the asm's video/sound calls live
/// in [`head`]/[`repaint`] on the `act_newgame` path.
///
/// Record `0` gets the fixed starting block (credits `0x1879A`), the
/// appended faction record `[count]` gets the rolled block
/// (`0xC350 + rand(0x4E20)` credits), planets `1..count−1` are named
/// `LIFELESS!`, every record gets its serial and a cleared `F_WORD0`.
pub fn new_game(vm: &mut Vm, st: &mut State, rng: &mut Rng) {
    vm.stage_save(st);
    let base = st.word(REC_BASE);
    let n = st.planets();
    home_planet(st, base, usize::from(st.byte(DIFFICULTY)));
    faction(st, base, n, rng);
    // `cx = [0x91B8] − 2` with a `dec/jns` tail → n−1 iterations over
    // records 1..=n−1 (planet 0 keeps its name; the faction record at n
    // is untouched).
    for i in 1..n {
        name_record(st, base, i, &LIFELESS);
    }
    for i in 0..=n {
        serial(st, base, i);
    }
    for i in 0..=n {
        st.set_word(
            usize::from(base.wrapping_add(i * rec_stride())) + F_WORD0,
            0,
        );
    }
}

/// `cs:0x305B`/`cs:0x9DAB` — the new-game menu action: [`head`] (the
/// `0x3305B` sound/screen install the `jmp 0x30A0` restart entry
/// skips), then [`newgame_body`] — [`new_game`], the `0x33194` repaint
/// and the `0x331C4` tail ending `jmp 0x2D27` (shell re-entry).
pub fn act_newgame(c: &mut Ctx) {
    head(c);
    newgame_body(c);
}

/// `cs:0x30A0` — the restart re-entry point (file `0x330A0`–`0x331EF`):
/// [`new_game`] (whose `0x8432` stage save is an identity re-save on
/// the restart path), the `0x33194` repaint, then the asm tail
/// `0x331C4`–`0x331E6` — `0xA3BC` input restrict (int-33 `ax=0x14`,
/// mask `0x1E`), `0x83AA` mark, `0x5DD3` select-next, a `0x32`-frame
/// `{0x2CA9, [0x127C]}` wait (the mode's star-grid painter — the near
/// `ds:` slot, not the `cs:` sound export), `0x2FD7`, `[0x1278]`
/// reload, `0xA36A` input reinstall, the `0x6E30` ticker message —
/// before `jmp 0x2D27` re-enters the shell setup tail.
pub fn newgame_body(c: &mut Ctx) {
    new_game(c.vm, c.st, c.rng);
    repaint(c);
    c.host.svc(Call::Mouse(0x14, 0x1E, 0));
    selrec::mark_sel(c);
    selrec::sel_next(c);
    for _ in 0..=0x32 {
        vsync(c);
        c.host.svc(Call::Slot(0x127C, 0));
    }
    c.host.svc(Call::Native(0x2FD7));
    c.host.svc_st(Call::Slot(0x1278, 0), c.vm, c.st);
    c.host.svc(Call::Mouse(0x14, 0x1F, 0));
    text::enqueue(c, 0x6E30);
    shell::re_enter(c);
}

/// `0x3305B`–`0x3309C` — the `0x305B` entry head: the game-song swap
/// (`SndReset`, `Song(0x0C3E)`, `Sound(0)`, `0x89FB`), `[0x91D3] = 0`
/// (sound on), the `0x1260` DAC flush, the shell hotspot install
/// (`[0x9CC8] = [0xA0AA]`, `[0x9194] = 0xA0AC`, `[0x91A8] = 0x31`)
/// and the `dx = 0` `0x125E` present.
fn head(c: &mut Ctx) {
    c.host.svc(Call::SndReset);
    c.host.svc(Call::Song(0xC3E));
    c.host.svc(Call::Sound(0));
    c.host.svc(Call::Chan(0x89FB, 0));
    c.vm.w8(c.st, a!(SND_ON), 0);
    c.host.svc(Call::Slot(0x1260, 0));
    let sel = c.vm.r16(c.st, a!(MENU_SEL));
    c.vm.w16(c.st, a!(HOT_SEL), sel);
    c.vm.w16(c.st, a!(HOT_LIST), a!(MENU_LIST));
    c.vm.w16(c.st, a!(HOT_COUNT), MENU_COUNT);
    c.host.svc_st(Call::Present(0), c.vm, c.st);
}

/// `0x33194`–`0x331B2` — the mid-init repaint both entries share:
/// `0x1260`/`0x125C`, the `0x89B3` border, `Image(0)`, `Present(0)`
/// and `uimode = 0` (file `0x33194`–`0x331B0`).
fn repaint(c: &mut Ctx) {
    c.host.svc(Call::Slot(0x1260, 0));
    c.host.svc(Call::Slot(0x125C, 0));
    c.host.svc(Call::Native(0x89B3));
    c.host.svc(Call::Image(0));
    c.host.svc_st(Call::Present(0), c.vm, c.st);
    c.vm.w8(c.st, a!(UIMODE), 0);
}

/// `i·0x3A` as `u16` — the stride the asm multiplies by.
fn rec_stride() -> u16 {
    u16::try_from(REC_STRIDE).unwrap_or(0)
}

/// Record `0` — fixed player start (file `0x330A0`–`0x330FB`).
fn home_planet(st: &mut State, base: u16, diff: usize) {
    let r = usize::from(base);
    st.set_word(r + F_TROOPS, DIFF_PARAM[diff.min(2)]);
    st.set_word(SEL_IDX, SEL_INIT[diff.min(2)]);
    st.set_dword(r + F_CREDITS, 0x1_879A);
    st.set_word(r + F_FOOD, 0x2AF8);
    st.set_word(r + F_MINERALS, 0x1128);
    st.set_word(r + F_FUEL, 0x1C42);
    st.set_word(r + F_ENERGY, 0x1735);
    st.set_word(r + F_POP, 0x22D1);
    st.set_word(r + F_DEFENCE, 0);
    st.set_byte(r + F_OWNER, 2);
}

/// Record `[count]` — the rolled faction block (file `0x330FF`–`0x3314B`).
fn faction(st: &mut State, base: u16, n: u16, rng: &mut Rng) {
    let r = usize::from(base.wrapping_add(n.wrapping_mul(rec_stride())));
    st.set_dword(r + F_CREDITS, 0xC350 + u32::from(rng.range(0x4E20)));
    let d = rng.range(0x3E8);
    for (k, f) in [F_FOOD, F_MINERALS, F_FUEL, F_ENERGY].iter().enumerate() {
        let v = 0x9C4 + d.wrapping_mul(u16::try_from(k + 1).unwrap_or(0));
        st.set_word(r + f, v);
    }
    st.set_word(r + F_POP, 0x5DC + rng.range(0x1F4));
    st.set_word(r + F_DEFENCE, 0);
    st.set_byte(r + F_OWNER, 2);
}

/// `name` → `+0x0E..+0x16` of record `i` (file `0x3314F`–`0x33178`).
fn name_record(st: &mut State, base: u16, i: u16, name: &[u8; 9]) {
    let r = usize::from(base.wrapping_add(i.wrapping_mul(rec_stride())));
    for (k, &b) in name.iter().enumerate() {
        st.set_byte(r + F_NAME + k, b);
    }
}

/// Serial stamp — `+0x28 = i`, `+0x29 = 0` (file `0x3317A`–`0x33192`).
fn serial(st: &mut State, base: u16, i: u16) {
    let r = usize::from(base.wrapping_add(i.wrapping_mul(rec_stride())));
    st.set_byte(r + F_SERIAL, u8::try_from(i).unwrap_or(0));
    st.set_byte(r + F_DEFLVL, 0);
}
