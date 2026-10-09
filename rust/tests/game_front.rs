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

/// A host for `mode` with its matching asset stem and palette model —
/// the `run.rs` palette selection.
fn mode_host(mode: Video) -> Host {
    let i = img();
    let set = load_set(&game_dir(), mode.stem(), heap_segment()).unwrap();
    let pal = match mode {
        Video::Mcga => from_dac(&i[DAC_OFS..DAC_OFS + 768]),
        Video::Cga => supremacy::palette::Palette::cga(),
        Video::Ega | Video::Tga => supremacy::palette::Palette::ega16(),
    };
    Host::new(Screen::new(mode), pal, set, i, false)
}

#[test]
fn image_and_glyph_reach_the_framebuffer() {
    let mut h = host();
    h.svc(Call::Image(0));
    assert!(h.scr.buf.iter().any(|&b| b != 0)); // backdrop painted
    h.svc(Call::Glyph(b'A', 0, 0));
    // Real font: 'A' row 0 = `[3, 1, 3, 0]` baked pixels (ds:0x30F9+0x318).
    assert_eq!(&h.scr.buf[0..4], &[3, 1, 3, 0]);
    assert_eq!(&h.scr.buf[0x140..0x144], &[1, 0, 2, 0]);
}

/// The shipped 'A' record in each mode's packed layout — colours are
/// baked into the font data, so every check reads raw bytes out of
/// `scr.buf` (MCG above).
#[test]
fn glyph_put_lands_in_every_mode() {
    // EGA `0x2FF33`: four plane records `44 AA EE AA AA 00` /
    // `00 55 33 55 77 00` / `CC 22 44 00 44 00` / `00…` merged into the
    // high nibble (even `x4`) — pixel (1,0) = planes 0+2 → colour 5.
    let mut h = mode_host(Video::Ega);
    h.svc(Call::Glyph(b'A', 0, 0));
    assert_eq!(h.scr.buf[0], 0x40);
    assert_eq!(h.scr.buf[0x28], 0xA0);
    assert_eq!(h.scr.buf[0x1F40 + 0x28], 0x50);
    assert_eq!(h.scr.buf[0x3E80], 0xC0);
    assert_eq!(h.scr.pixels()[1], 5);

    // CGA `0x30887`: 2bpp record `DC 4C 7C CC CC 00` — bank 0 rows
    // 0/2/4 at `0`/`0x50`/`0xA0`, bank 1 rows 1/3/5 at `0x2000`/`0x2050`.
    let mut h = mode_host(Video::Cga);
    h.svc(Call::Glyph(b'A', 0, 0));
    assert_eq!(h.scr.buf[0], 0xDC);
    assert_eq!(h.scr.buf[0x2000], 0x4C);
    assert_eq!(h.scr.buf[0x50], 0x7C);
    assert_eq!(h.scr.buf[0x2050], 0xCC);
    assert_eq!(h.scr.buf[0xA0], 0xCC);

    // TGA `0x3133B`: 4bpp record `85 80 50 70 57 70 70 70 70 70 00 00`
    // — `movsw` per row across the four `0x2000` banks.
    let mut h = mode_host(Video::Tga);
    h.svc(Call::Glyph(b'A', 0, 0));
    assert_eq!(&h.scr.buf[0..2], &[0x85, 0x80]);
    assert_eq!(&h.scr.buf[0x2000..0x2002], &[0x50, 0x70]);
    assert_eq!(&h.scr.buf[0x4000..0x4002], &[0x57, 0x70]);
    assert_eq!(&h.scr.buf[0xA0..0xA2], &[0x70, 0x70]);
}

#[test]
fn image_blit_reaches_every_mode() {
    for mode in [Video::Ega, Video::Cga, Video::Tga] {
        let mut h = mode_host(mode);
        h.svc(Call::Image(0));
        assert!(h.scr.buf.iter().any(|&b| b != 0), "{mode:?}");
        assert!(h.scr.pixels().iter().any(|&p| p != 0), "{mode:?}");
    }
}

#[test]
fn rect_outlines_in_every_mode() {
    for mode in [Video::Ega, Video::Cga, Video::Tga] {
        let mut h = mode_host(mode);
        h.svc(Call::Rect(0, 0, 1, 1));
        assert!(h.scr.buf.iter().any(|&b| b != 0), "{mode:?}");
    }
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
    let mut files = supremacy::platform::MemFs::new();
    let mut c = Ctx {
        vm: &mut vm,
        st: &mut st,
        rng: &mut Rng::new(1),
        host: &mut host,
        files: &mut files,
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
        Box::new(supremacy::platform::MemFs::new()),
    );
    assert_eq!(g.phase, Phase::Shell);
    assert_eq!(g.st.byte(UIMODE), 0);
    assert_eq!(g.st.word(HOT_COUNT), 0x31);
    assert!(g.host.scr.buf.iter().any(|&b| b != 0)); // shell backdrop
    let f0 = g.st.byte(game::FRAME);
    g.step();
    assert_ne!(g.st.byte(game::FRAME), f0);

    g.enter_galaxy(); // dispatch 0x3906 — uimode 5 drives the phase
    assert_eq!(g.phase, Phase::Galaxy);
    assert_eq!(g.st.byte(UIMODE), 5);
    g.host.pump.set_buttons(false, true);
    g.step(); // 0x7D → press(2) → frame exit → shell_enter
    assert_eq!(g.phase, Phase::Shell);
    assert_eq!(g.st.byte(UIMODE), 0);
}
