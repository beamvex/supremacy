//! The `front` host — call replay into `Screen`/`Palette`, `cs:` reads,
//! and the input pump's `ds:` cell mirroring — plus `Game`'s shell ↔
//! galaxy phase machine against the real `MCG` set and exe image.

mod common;

use common::{game_dir, gsim, read, root};
use supremacy::args::Video;
use supremacy::assets::{heap_segment, load_set};
use supremacy::front::{Game, Host, Phase, Pump};
use supremacy::game::{
    self, Call, Ctx, NullHost, Rng, Vm, VmHost, BUTTONS, CUR_X, CUR_Y, HOT_COUNT, KEY_CODE,
    KEY_PEND, UIMODE,
};
use supremacy::palette::{from_dac, DAC_OFS};
use supremacy::video::Screen;

fn img() -> Vec<u8> {
    read(&root().join("decompiled/GAME.unpacked.exe"))
}

fn host() -> Host {
    let i = img();
    let set = load_set(&game_dir(), "MCG", heap_segment()).unwrap();
    let pal = from_dac(&i[DAC_OFS..DAC_OFS + 768]);
    Host::new(Screen::new(Video::Mcga), pal, set, i, false)
}

#[test]
fn image_and_glyph_reach_the_framebuffer() {
    let mut h = host();
    h.svc(Call::Image(0));
    assert!(h.scr.buf.iter().any(|&b| b != 0)); // backdrop painted
    h.svc(Call::Glyph(b'A', 0, 0));
    // 'A' row 0 = 0x6 → columns 1..=2 lit at ink 0x0F.
    assert_eq!((h.scr.buf[0], h.scr.buf[1], h.scr.buf[2]), (0, 0x0F, 0x0F));
}

#[test]
fn slots_clear_and_program_dac() {
    let mut h = host();
    h.svc(Call::Glyph(b'#', 0, 0));
    h.svc(Call::Slot(0x125C, 0));
    assert!(h.scr.buf.iter().all(|&b| b == 0));
    h.svc(Call::Slot(0x1260, 0));
    // template `cs:0x1384` = file 0x2A634, ships zeroed → black.
    assert_eq!(h.pal.rgb(0), [0, 0, 0]);
}

#[test]
fn cs_word_reads_the_code_segment() {
    let mut h = host();
    let i = img();
    let want = u16::from_le_bytes([i[0x31384], i[0x31385]]);
    assert_eq!(h.cs_word(0x1384), want);
}

#[test]
fn pump_latches_synthetic_codes() {
    let mut vm = Vm::new();
    let mut st = gsim::galaxy();
    let mut p = Pump::new(false);
    p.move_to(100, 50);
    p.set_buttons(true, false);
    p.write(&mut vm, &mut st);
    assert_eq!(
        (vm.r16(&st, CUR_X as u16), vm.r16(&st, CUR_Y as u16)),
        (100, 50)
    );
    assert_eq!(
        (vm.r8(&st, KEY_PEND as u16), vm.r8(&st, KEY_CODE as u16)),
        (1, 0x7E)
    );
    let mut host = NullHost;
    let mut c = Ctx {
        vm: &mut vm,
        st: &mut st,
        rng: &mut Rng::new(1),
        host: &mut host,
    };
    game::menu_service(&mut c); // synth → press(1)
    assert_eq!(c.vm.r16(c.st, BUTTONS as u16), 1);
}

#[test]
fn game_enters_shell_and_galaxy_exits_on_right_click() {
    let mut g = Game::new(
        img(),
        load_set(&game_dir(), "MCG", heap_segment()).unwrap(),
        from_dac(&img()[DAC_OFS..DAC_OFS + 768]),
        Video::Mcga,
        false,
        3,
        0x42,
    );
    assert_eq!(g.phase, Phase::Shell);
    assert_eq!(g.st.byte(UIMODE), 0);
    assert_eq!(g.st.word(HOT_COUNT), 0x31);
    assert!(g.host.scr.buf.iter().any(|&b| b != 0)); // shell backdrop
    let f0 = g.st.byte(game::FRAME);
    g.step();
    assert_ne!(g.st.byte(game::FRAME), f0);

    g.enter_galaxy();
    g.host.pump.set_buttons(false, true);
    g.step(); // 0x7D → press(2) → frame exit
    assert_eq!(g.phase, Phase::Shell);
}
