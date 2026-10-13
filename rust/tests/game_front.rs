//! The `front` host — call replay into `Screen`/`Palette`, `cs:` reads,
//! and the input pump's `ds:` cell mirroring — plus `Game`'s shell ↔
//! galaxy phase machine against the real `MCG` set and exe image.

mod common;

use common::{game_dir, gsim, read, root};
use supremacy::args::{Sound, Video};
use supremacy::assets::{heap_segment, load_set};
use supremacy::front::{Game, Host, Phase, Pump};
use supremacy::game::{
    self, Call, Ctx, NullHost, Rng, Vm, VmHost, BUTTONS, CUR_X, CUR_Y, HOT_COUNT, HOT_LIST,
    KEY_CODE, KEY_PEND, UIMODE,
};
use supremacy::palette::{from_dac, DAC_OFS, ROWS_OFS, WORK_OFS};
use supremacy::video::Screen;

fn img() -> Vec<u8> {
    read(&root().join("decompiled/GAME.unpacked.exe"))
}

fn host() -> Host {
    let i = img();
    let set = load_set(&game_dir(), "MCG", heap_segment()).unwrap();
    let pal = from_dac(&i[DAC_OFS..DAC_OFS + 768]);
    Host::new(
        Screen::new(Video::Mcga),
        pal,
        set,
        i,
        false,
        Sound::PcSpeaker,
    )
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
    Host::new(Screen::new(mode), pal, set, i, false, Sound::PcSpeaker)
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

/// `Slot(0x1278)` — the `0x2F2C5` `map_draw`: one marker byte per
/// record at its orbit cell; `Slot(0x127C)` fills the minimap region
/// with the `0x2F398` star grid. The EGA/CGA/TGA twins are unported.
#[test]
fn map_draw_marks_records_and_grid_paints_stars() {
    let mut h = host();
    let (mut vm, mut st) = (Vm::new(), gsim::galaxy());
    let base = vm.r16(&st, u16::try_from(game::REC_BASE).unwrap_or(0));
    let n = vm.r16(&st, u16::try_from(game::REC_COUNT).unwrap_or(0));
    for i in 0..=n {
        let rec = base.wrapping_add(i.wrapping_mul(0x3A));
        vm.w16(&mut st, rec + 6, i.wrapping_mul(0x20)); // orbit angle
        vm.w16(&mut st, rec + 8, 0x30); // orbit radius
        vm.w16(&mut st, rec + 0xA, i.wrapping_mul(3)); // row
        vm.w16(&mut st, rec + 0xC, 2); // kind — the no-blink case
        vm.w8(&mut st, rec + 5, 0); // not the selected record
    }
    h.svc_st(Call::Slot(0x1278, 0), &vm, &st);
    let marked = h.scr.buf.iter().filter(|&&b| b != 0).count();
    assert_eq!(marked, usize::from(n) + 1);
    let mut e = mode_host(Video::Ega);
    e.svc_st(Call::Slot(0x1278, 0), &vm, &st);
    assert_eq!(e.dropped, 1);
    h.scr.buf.fill(0);
    h.svc(Call::Slot(0x127C, 0));
    assert!(h.scr.buf[..0x7120].iter().all(|&b| b == 0));
    assert!(h.scr.buf[0x7120..].iter().any(|&b| b != 0));
}

/// `Present(dx)` — the `0x2ED30` composite: regs `1..=31` from row
/// `dx` of `ds:0x4394`, regs `0x21..=0xFF` from `ds:0x4095` colours
/// `33..`, every nonzero component through the `ds:[0x9CA1]` shade.
/// Expected values cross-checked against the DOS-side DAC dump
/// `difftest/out/P05.BIN` (a `Present(0xD)` attract frame).
#[test]
fn present_stages_the_screen_palette() {
    let mut h = host();
    let i = img();
    let (vm, st) = (Vm::new(), gsim::galaxy());
    assert_eq!(h.pal.rgb(1), scaled(&i, DAC_OFS + 3)); // master pre-state
    h.svc_st(Call::Present(0xD), &vm, &st);
    for n in 1..=0x1F_usize {
        assert_eq!(h.pal.rgb(n), scaled(&i, ROWS_OFS + 0xD * 0x60 + n * 3));
    }
    for n in 0x21..=0xFF_usize {
        assert_eq!(h.pal.rgb(n), scaled(&i, WORK_OFS + n * 3));
    }
    // `P05.BIN` spot checks (the dump is raw 6-bit).
    assert_eq!(h.pal.rgb(1).map(|v| v >> 2), [4, 0, 0]);
    assert_eq!(h.pal.rgb(33).map(|v| v >> 2), [60, 60, 60]);
    assert_eq!(h.pal.rgb(255).map(|v| v >> 2), [63, 63, 0]);
}

/// `Present(6)` — the `dx == 6` branch stages the whole `ds:0x4994`
/// master (regs `1..=0xFF`), no `pal_adj`; its `add al,[0x9ca1]` has
/// no zero-gate, so blank components take the shade too.
#[test]
fn present_six_stages_the_master_table() {
    let mut h = host();
    let i = img();
    let (vm, mut st) = (Vm::new(), gsim::galaxy());
    st.set_byte(0x9CA1, 0xF); // scenario shade — clamped at 0x3F
    h.svc_st(Call::Present(6), &vm, &st);
    for n in 1..=0xFF_usize {
        let want = (1..4)
            .map(|k| {
                let v = i[DAC_OFS + n * 3 + k - 1].wrapping_add(0xF).min(0x3F);
                u8::try_from(u16::from(v) * 255 / 63).unwrap()
            })
            .collect::<Vec<_>>();
        assert_eq!(h.pal.rgb(n), [want[0], want[1], want[2]], "reg {n}");
    }
}

fn scaled(i: &[u8], ofs: usize) -> [u8; 3] {
    <[u8; 3]>::try_from(&i[ofs..ofs + 3])
        .unwrap_or_default()
        .map(|v| u8::try_from(u16::from(v) * 255 / 63).unwrap_or(0))
}

/// GAM-46 — `menu_text` must paint the `ss:` stream's 16×24 tiles:
/// the shipped `0x5012` block ("MELBOURNE HOUSE" over three lines at
/// rows 45/71/97) lands in the framebuffer on arm and `0xFF`-suspends
/// with the cursor on the next block, so the credits advance. (The old
/// walk read `file:0x20000` instead of `ss = 0x1F6F` → `0x202F0` and
/// never drew the tiles, so every page returned stream-end instantly.)
#[test]
fn menu_text_paints_the_credits_pages() {
    let mut h = host();
    assert_eq!(h.svc(Call::Text(0x5012)), 0xF1); // fb15 — scroll variant
                                                 // Line 1, third tile: 'M' row 0 of `cs:0x21C4 + 0xC·0x180`.
    assert_eq!(
        &h.scr.buf[0x3868..0x3868 + 0x10],
        &[2, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0]
    );
    assert!(h.scr.buf[0x58C0..].iter().any(|&b| b != 0)); // line 2
    assert!(h.scr.buf[0x7940..].iter().any(|&b| b != 0)); // line 3
    assert_eq!(h.dropped, 0);
}

/// GAM-46 — the crawl is a scroll, not a repaint: `Call::Scroll` shifts
/// the painted block up `arg` rows a call and feeds the queued `0xFF`
/// block's rows in at the bottom, so every card is visible en route and
/// nothing ever repaints over itself.
#[test]
fn scroll_steps_crawl_the_text_up() {
    let mut h = host();
    let tile = [2, 2, 2, 2, 2, 2, 0, 0, 0, 2, 2, 2, 2, 2, 2, 0];
    assert_eq!(h.svc(Call::Text(0x5012)), 0xF1);
    assert_eq!(&h.scr.buf[0x3868..0x3868 + 0x10], &tile); // 'M' row 0
                                                          // Each step — the painted block moves up `2` rows; nothing
                                                          // repaints in place (the old bug).
    h.svc(Call::Scroll(2));
    assert_eq!(&h.scr.buf[0x35E8..0x35E8 + 0x10], &tile);
    h.svc(Call::Scroll(2));
    assert_eq!(&h.scr.buf[0x3368..0x3368 + 0x10], &tile);
    // After 24 more steps the next block ("S.U.P.R.E.M.A.C.Y.") starts
    // feeding in at the bottom of the screen.
    h.svc(Call::Text(u16::MAX)); // pull block 1 into the queue
    for _ in 0..0x30 {
        h.svc(Call::Scroll(2));
    }
    let bottom = &h.scr.buf[0xFA00 - 0x140 * 0x40..];
    assert!(bottom.iter().any(|&b| b != 0), "next block never fed in");
}

/// GAM-46 — the "snow" must clear before the text begins and stay
/// swept: `Call::Text` arm wipes the live star dots and once the crawl
/// is scrolling, `Call::Stars` no longer plots new ones over the text.
#[test]
fn snow_clears_at_arm_and_stays_swept() {
    let mut h = host();
    for _ in 0..0x40 {
        h.svc(Call::Stars(0x63));
    }
    let dots = |h: &Host| {
        h.scr
            .buf
            .iter()
            .fold(0usize, |n, &b| n + usize::from(b == 1))
    };
    assert!(dots(&h) > 0, "starfield never plotted");
    h.svc(Call::Text(0x5012));
    assert_eq!(dots(&h), 0, "snow still on screen at first text");
    h.svc(Call::Scroll(2));
    for _ in 0..0x20 {
        h.svc(Call::Stars(0x63));
        h.svc(Call::Scroll(2));
    }
    assert_eq!(dots(&h), 0, "stars replotted over the crawl");
    assert!(h.scr.buf.iter().any(|&b| b != 0)); // text still scrolling
}

/// GAM-46 — the crawl runs the stream to its `0x00` terminator then
/// reports `0x100` once a full screen of blank tail has fed: every
/// block has scrolled through and the buffer lands clean.
#[test]
fn crawl_drains_the_whole_stream() {
    let mut h = host();
    h.svc(Call::Text(0x5012));
    let mut ax = 0u16;
    for _ in 0..0x2000 {
        h.svc(Call::Scroll(2));
        ax = h.svc(Call::Text(u16::MAX));
        if ax & 0x100 != 0 {
            break;
        }
    }
    assert_eq!(ax & 0x100, 0x100, "stream never ended");
    assert!(h.scr.buf.iter().all(|&b| b == 0));
}

/// Non-MCG `menu_text` twins stay unported — the walk still decodes
/// the stream and counts the tile ops it can't draw.
#[test]
fn menu_text_other_modes_count_undrawn_tiles() {
    let mut h = mode_host(Video::Ega);
    assert_eq!(h.svc(Call::Text(0x5012)), 0xF1);
    assert!(h.dropped > 0);
    assert!(h.scr.buf.iter().all(|&b| b == 0));
}

/// GAM-46 follow-up — the MCG `[0x1294]` star mark (`0x2F59B`) must
/// erase the star's previous pixel (only when it's still the `1`
/// colour) and only plot onto empty cells, so the attract's starfield
/// moves as dots instead of trailing rays over the credits text.
#[test]
fn stars_move_as_dots_and_never_clobber_text() {
    let mut h = host();
    assert_eq!(h.svc(Call::Text(0x5012)), 0xF1); // a real credits page
    let text: Vec<usize> = (0..h.scr.buf.len()).filter(|&i| h.scr.buf[i] > 1).collect();
    let before: Vec<u8> = text.iter().map(|&i| h.scr.buf[i]).collect();
    for _ in 0..0x40 {
        h.svc(Call::Stars(0x63));
    }
    let after: Vec<u8> = text.iter().map(|&i| h.scr.buf[i]).collect();
    assert_eq!(before, after, "stars overwrote text pixels");
    let dots = h
        .scr
        .buf
        .iter()
        .fold(0usize, |n, &b| n + usize::from(b == 1));
    assert!(dots <= 0x63, "stars trailed: {dots} marked pixels");
}

#[test]
fn cs_word_reads_the_code_segment() {
    let mut h = host();
    let i = img();
    // real `cs:0x1384` = file `0x2A634` — the DAC0 triplet.
    let want = u16::from_le_bytes([i[0x2A634], i[0x2A635]]);
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
        (
            vm.r16(&st, u16::try_from(CUR_X).unwrap()),
            vm.r16(&st, u16::try_from(CUR_Y).unwrap())
        ),
        (100, 50)
    );
    assert_eq!(
        (
            vm.r8(&st, u16::try_from(KEY_PEND).unwrap()),
            vm.r8(&st, u16::try_from(KEY_CODE).unwrap())
        ),
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
    assert_eq!(c.vm.r16(c.st, u16::try_from(BUTTONS).unwrap()), 1);
}

/// `pump_input` must run the live vsync hook — the port's stand-in
/// for the int-9/int-33 ISRs and retrace that feed the `ds:` cells
/// mid-loop. Without it, blocking wait-loops see only what was queued
/// at `Game::step` entry and never present their framebuffer.
#[test]
fn pump_input_runs_the_live_poller() {
    let mut h = host();
    h.set_vsync(Some(Box::new(|h| {
        h.pump.move_to(7, 9);
        h.pump.set_buttons(true, false);
    })));
    let (mut vm, mut st) = (Vm::new(), gsim::galaxy());
    h.pump_input(&mut vm, &mut st);
    assert_eq!(
        (
            vm.r16(&st, u16::try_from(CUR_X).unwrap()),
            vm.r16(&st, u16::try_from(CUR_Y).unwrap()),
            vm.r8(&st, u16::try_from(KEY_PEND).unwrap()),
            vm.r8(&st, u16::try_from(KEY_CODE).unwrap())
        ),
        (7, 9, 1, 0x7E)
    );
}

/// `map_click`'s `{vsync, 0xA369, 0xA1CA}` wait-loops poll `[0x9CDA]`
/// mid-step: the `0xA1CA` key service must run (it was a dropped
/// `Call::Native`, which hung the loop) and `pump_input` must deliver
/// fresh edges — this hangs when either half is missing.
#[test]
fn map_click_sees_input_inside_its_wait_loop() {
    let mut g = Game::new(
        img(),
        load_set(&game_dir(), "MCG", heap_segment()).unwrap(),
        from_dac(&img()[DAC_OFS..DAC_OFS + 768]),
        Video::Mcga,
        Sound::PcSpeaker,
        false,
        3,
        0x42,
        Box::new(supremacy::platform::MemFs::new()),
    );
    // The host feed: first release whatever was held (poll-wait 1),
    // then press inside the `[0x111..=0x132] × [0x13..=0x2B]` map
    // region (poll-wait 2) — like a user releasing the dispatching
    // click then clicking the minimap.
    let mut n = 0usize;
    g.host.set_vsync(Some(Box::new(move |h| {
        n += 1;
        if n > 1 {
            h.pump.move_to(0x120, 0x20);
            h.pump.set_buttons(true, false);
        } else {
            h.pump.set_buttons(false, false);
        }
    })));
    let mut c = g.ctx();
    game::dispatch_action(&mut c, 0x23C2); // map_click
    assert_eq!(
        (
            c.vm.r16(c.st, u16::try_from(CUR_X).unwrap()),
            c.vm.r16(c.st, u16::try_from(CUR_Y).unwrap()),
            c.vm.r16(c.st, u16::try_from(BUTTONS).unwrap())
        ),
        (0x120, 0x20, 1)
    );
}

#[test]
fn game_enters_shell_and_galaxy_exits_on_right_click() {
    let mut g = Game::new(
        img(),
        load_set(&game_dir(), "MCG", heap_segment()).unwrap(),
        from_dac(&img()[DAC_OFS..DAC_OFS + 768]),
        Video::Mcga,
        Sound::PcSpeaker,
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

/// GAM-47 — the `0x2E21` save/load dialog action must run its
/// `0x2E60` modal loop. When only the enter head ran, the `0xB05C`
/// list stayed installed with no `[0x9CDA]&2` test anywhere, so a
/// right-click could never reinstall the shell's `0xA0AC` list —
/// the dialog was a dead end off the home screen. The live vsync
/// hook feeds the right-button edge mid-loop.
#[test]
fn save_dialog_right_click_returns_to_the_shell() {
    let mut g = Game::new(
        img(),
        load_set(&game_dir(), "MCG", heap_segment()).unwrap(),
        from_dac(&img()[DAC_OFS..DAC_OFS + 768]),
        Video::Mcga,
        Sound::PcSpeaker,
        false,
        3,
        0x42,
        Box::new(supremacy::platform::MemFs::new()),
    );
    assert_eq!(g.st.word(HOT_LIST), 0xA0AC);
    g.host.set_vsync(Some(Box::new(|h| {
        h.pump.set_buttons(false, true);
    })));
    let mut c = g.ctx();
    game::dispatch_action(&mut c, 0x2E21);
    assert_eq!(c.vm.r16(c.st, u16::try_from(HOT_LIST).unwrap()), 0xA0AC);
    assert_eq!(c.vm.r16(c.st, u16::try_from(HOT_COUNT).unwrap()), 0x31);
    assert_eq!(c.vm.r8(c.st, u16::try_from(UIMODE).unwrap()), 0);
}

/// `Game::new` must boot the shipped state block — the record
/// templates it carries are what `endgame` scans for kind-`7`/`0xA`
/// (`new_game` only patches around them; a zeroed block reads as
/// "every planet gone" and fires the defeat ending ~0xFA ticks in,
/// leaving the game trapped in the outro starfield).
#[test]
fn fresh_galaxy_runs_past_the_endgame_monitor() {
    let mut g = Game::new(
        img(),
        load_set(&game_dir(), "MCG", heap_segment()).unwrap(),
        from_dac(&img()[DAC_OFS..DAC_OFS + 768]),
        Video::Mcga,
        Sound::PcSpeaker,
        false,
        3,
        0x42,
        Box::new(supremacy::platform::MemFs::new()),
    );
    g.enter_galaxy();
    for _ in 0..0x140 {
        g.step();
        assert!(g.vm.end.is_none(), "ending fired on a fresh galaxy");
    }
    assert_eq!(g.phase, Phase::Galaxy);
}

/// The real `*IN` boot — `Game::new_title` on the shipped image uses
/// the `ds:0x9C62`/`ds:0x9D34` structures as they ship (no `tpl + 0x600`
/// copy — that idiom clobbered the live picker list with string data,
/// leaving every hotspot unhittable: GAM-43). A keypress jumps the
/// attract to the picker and a click inside the start rect
/// (`x 0xE2..0x12A`, `y 0xA5..0xB8`) must reach the shell.
#[test]
fn title_boot_picker_start_reaches_the_shell() {
    let mut g = Game::new_title(
        img(),
        load_set(&game_dir(), "MCGIN", heap_segment()).unwrap(),
        from_dac(&img()[DAC_OFS..DAC_OFS + 768]),
        Video::Mcga,
        Sound::PcSpeaker,
        false,
        0x42,
        Box::new(supremacy::platform::MemFs::new()),
    );
    g.host.set_assets(&game_dir(), "MCG");
    assert!(g.vm.title.is_some());

    g.host.pump.key(0x39); // any key → `jmp 0x87D7`
    for _ in 0..0x10 {
        g.step();
        if g.st.word(HOT_LIST) == 0x9D36 {
            break;
        }
    }
    assert_eq!(g.st.word(HOT_LIST), 0x9D36);

    g.host.pump.move_to(0x106, 0xAE);
    g.host.pump.set_buttons(true, false);
    for _ in 0..0x10 {
        g.step();
        if g.vm.title.is_none() {
            break;
        }
        g.host.pump.set_buttons(false, false);
    }
    assert!(g.vm.title.is_none(), "start click never left the picker");
    assert_eq!(g.phase, Phase::Shell);
    assert_eq!(g.st.byte(UIMODE), 0);
    // GAM-44 — the `0x305B` head + `0x33194` repaint must leave the
    // post-pick shell visible: `scenario_start`'s `0x9CA1 = 0xF` +
    // refresh pair fades the picker to black, so without the `0x305B`
    // `Image(0)`/`Present(0)` repaint the chosen-opponent shell stayed
    // on the black palette/cleared buffer.
    assert!(g.host.scr.pixels().iter().any(|&p| p != 0));
    assert!((1..256).any(|n| g.host.pal.rgb(n) != [0, 0, 0]));
}
