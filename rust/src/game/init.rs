use super::consts::{DIFFICULTY, REC_BASE, REC_STRIDE, SEL_IDX};
use super::rec::{
    F_CREDITS, F_DEFENCE, F_DEFLVL, F_ENERGY, F_FOOD, F_FUEL, F_MINERALS, F_NAME, F_OWNER, F_POP,
    F_SERIAL, F_TROOPS, F_WORD0,
};
use super::vhost::Call;
use super::vm::Vm;
use super::vops::{vsync, Ctx};
use super::{selrec, Rng, State};

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
/// staging-heap restore that resets the save block to the state the
/// `0x844B` snapshot captured (menu picks like `[0x91E4]` survive
/// because the snapshot runs after them). Assumes
/// [`select_galaxy`][super::select_galaxy] already anchored the array
/// (`[0x9158]`) and set the counts; video and sound calls in the asm
/// are the shell's job, not this fn's.
///
/// Record `0` gets the fixed starting block (credits `0x1879A`), the
/// appended faction record `[count]` gets the rolled block
/// (`0xC350 + rand(0x4E20)` credits), planets `1..count−1` are named
/// `LIFELESS!`, every record gets its serial and a cleared `F_WORD0`.
pub fn new_game(vm: &mut Vm, st: &mut State, rng: &mut Rng) {
    vm.stage_load(st);
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

/// `cs:0x305B`/`cs:0x9DAB` — the new-game menu action: [`new_game`]
/// (with its `0x8432` stage restore), then the asm tail at file
/// `0x331C4`–`0x331E6` — `0xA3BC` input restrict (int-33 `ax=0x14`,
/// mask `0x1E`), `0x83AA` mark, `0x5DD3` select-next, a `0x32`-frame
/// `{0x2CA9, [cs:0x127C]}` wait, `0x2FD7`, `[0x1278]` reload, `0xA36A`
/// input reinstall — before `jmp 0x2D27` re-enters the shell loop.
pub fn act_newgame(c: &mut Ctx) {
    new_game(c.vm, c.st, c.rng);
    c.host.svc(Call::Mouse(0x14, 0x1E, 0));
    selrec::mark_sel(c);
    selrec::sel_next(c);
    for _ in 0..=0x32 {
        vsync(c);
        c.host.svc(Call::Sound(0));
    }
    c.host.svc(Call::Native(0x2FD7));
    c.host.svc(Call::Slot(0x1278, 0));
    c.host.svc(Call::Mouse(0x14, 0x1F, 0));
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
