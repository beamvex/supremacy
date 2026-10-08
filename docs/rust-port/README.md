# Supremacy Rust Port — Implementation Notes

`rust/` is a Rust rewrite of the reverse-engineered assembly routines from
*Supremacy: Your Will Be Done* (Probe Software, 1991), `GAME.EXE`. Every
routine is a port of a specific routine in the unpacked 16-bit image
(`decompiled/GAME.unpacked.exe`); file offsets cited throughout the code
refer to that file and to `decompiled/disasm/GAME.linear.asm`. The full
reverse-engineering report lives in `decompiled/ANALYSIS.md`; the function
map is `decompiled/FUNCTIONS.md`.

The crate is a faithful port, not an emulator: it reproduces the game's
data structures, algorithms and byte-level behaviour (right down to the
LZSS ring cursor and the EXEPACK backward copy), while host-dependent
effects (screen, ports, int calls) are abstracted behind traits so tests
and the `minifb` frontend can drive them.

## Layout

```
rust/
├── Cargo.toml          # crate `supremacy`; optional `minifb` feature
├── src/
│   ├── lib.rs          # module docs + `#![forbid(unsafe_code)]`
│   ├── args/           # GAME <V> <K> <S> command-line parser
│   ├── assets/         # .GPH index + .BIN heap loading
│   ├── audio/          # OPL2 / MPU-401 / PC-speaker device halves
│   ├── exepack/        # Microsoft EXEPACK unpacker
│   ├── front/          # concrete VmHost frontend + minifb window
│   ├── game/           # state block, event VM, tick, UI shell
│   ├── gph/            # 12-byte .GPH index records
│   ├── lzss/           # Okumura LZSS decoder (4 asm variants)
│   ├── palette/        # 256×3 master DAC table
│   ├── platform/       # int 21h files, int-8 timer, int-9 kbd, int-33h
│   ├── video/          # per-mode VRAM + draw-image-by-index
│   └── bin/
│       ├── supremacy.rs   # `supremacy` — renders one record → PPM
│       └── run.rs         # `supremacy-run` — playable minifb shell
└── tests/              # integration tests + fixtures + common rigs
```

## Building and testing

```bash
cd rust
cargo build                       # library + `supremacy` bin
cargo build --features minifb     # adds `supremacy-run`
cargo test                        # integration suite
cargo clippy --all-targets -- -W clippy::pedantic
cargo fmt
```

`#![forbid(unsafe_code)]`, `#![warn(missing_docs)]` and
`#![warn(clippy::pedantic)]` are set in `lib.rs`. Code style: one
item (function/type) per file where practical, methods ≤ 23 lines,
files ≤ 200 lines; `u16::try_from(..).unwrap_or(0)` is the standard
`ds:`-offset narrowing idiom.

## Address spaces

All numbers in doc comments are real addresses in one of three spaces
(see `decompiled/FUNCTIONS.md` § "Address spaces"):

| space | meaning | conversion |
|---|---|---|
| file offset | byte offset in `GAME.unpacked.exe` | — |
| `cs:` near offset | inside the main code segment | file − `0x292B0` |
| `ds:`/`dgroup` offset | inside the data segment | file − `0x10CA0` |

The port keeps game data addressed by `ds:` offsets exactly as the asm
uses them. `Vm::from_image`/`State::from_image` therefore read the image
at `ds_ofs + 0x10CA0`. The frontend `Host` maps `cs:` reads to file
offsets by adding `0x30000` (game code) or `0x292B0` (video-template
segment).

## Module guide

| module | asm provenance | notes |
|---|---|---|
| [`exepack`](exepack.md) | stub at file `0x23A90` | EXEPACK unpacker + repacker |
| [`lzss`](lzss.md) | `0x2EBEF`, `0x2EC8C`, `0x2FC5D`, `0x2FCF9` | Okumura LZSS, 4 variants |
| [`gph`](gph.md) | `0x320D8` fixup loop | 12-byte image index records |
| [`args`](args.md) | `0x31FF8`–`0x3207D` | PSP-tail argument parser |
| [`assets`](assets.md) | `0x3208E`–`0x3215A` | `.BIN`/`.GPH` loaders |
| [`palette`](palette.md) | file `0x15034` (dgroup `0x4994`) | 256×3 6-bit DAC table |
| [`audio`](audio.md) | `0x3470`–`0x3F52`, `0x0E9F` | OPL2, MPU-401, PIT speaker |
| [`platform`](platform.md) | `0x31D74`, `0x353D`+ | DOS services layer |
| [`video`](video.md) | `0x2EB8A`, `0x2FB36`, `0x30620`, `0x310CE` | VRAM + draw-by-index |
| [`front`](front.md) | GAM-9 frontend | `VmHost` impl, `minifb` window |
| [`game`](game-state.md) | `ds:0x7D39` block etc. | state, VM, tick, UI — see below |

The `game` module is large enough to split across six pages:

- [game-state](game-state.md) — `State`, `ds:` cell maps, record/machine/
  ship field layouts, `Rng`, galaxy presets, `new_game`, save/load.
- [game-vm](game-vm.md) — the `cs:0x773D` scripted-event VM: memory model,
  operand decode, the op dispatch table and every ported handler.
- [game-tick](game-tick.md) — the `cs:0x73D9` tick dispatcher: planet sim,
  machine production/mining, ship ageing, fleet defence, battles, the
  draw sequencer and the status ticker.
- [game-ui](game-ui.md) — the `cs:0x395B` galaxy frame loop, the
  `cs:0x2CC5` shell, the `cs:0xA10F` menu/hotspot service, the `+0xC`
  action table, record selection and the info panel.
- [game-text](game-text.md) — the `cs:0x6709` print interpreter, decimal
  printers, the blink timer and the oval hover test.
- [game-dialogs](game-dialogs.md) — the save/load dialog and confirm
  modal, `dlg_io` save/load actions, and the `cs:0xE66E` line editor.

## Binaries

- `supremacy <GAME-dir> [V] [K] [S] [--set STEM] [--idx N] [--exe PATH]
  [--out frame.ppm]` — parses the PSP-style tail, loads one asset set,
  draws record `idx` into a `Screen` and writes a `320×200` P6 PPM
  through the mode palette (`bin/supremacy.rs`).
- `supremacy-run <GAME-dir> [V] [K] [S] [--set STEM] [--exe PATH]
  [--preset N] [--seed N] [--galaxy]` (feature `minifb`) — boots a
  `front::Game` (fresh galaxy via `select_galaxy` + `new_game`, saves via
  `HostFs` rooted at the game dir) and runs the `minifb` loop
  (`bin/run.rs`).

Both take the DOS letter args positionally; `tail_bytes` synthesises a
PSP command tail (count byte, space, letters at `0x82`/`0x84`/`0x86`) so
`args::parse` sees exactly what the asm sees.

## Test suite

`rust/tests/` — integration tests, all black-box over the public API:

- `lzss_unit.rs` / `lzss_all.rs` — decoder units plus **all 1703 `.GPH`
  records in every asset set**, checked for decoded length + FNV-1a hash
  against the verified Python decoder (`decompiled/tools/extract_png.py`);
  `tests/fixtures/manifest.tsv` carries the expected rows.
- `exepack_unpack.rs` — unpacks `tests/fixtures/SUPCHT.EXE` and
  cross-checks `to_exe` repacking with a minimal MZ parser.
- `args_parse.rs`, `gph_index.rs`, `palette_dac.rs`, `assets_load.rs`,
  `platform_dos.rs`, `audio_dev.rs`, `video_draw.rs` — unit-level
  coverage of each leaf module (port-sequence recording via a mock
  `Ports`, `MemFs` file service, per-mode pixel expansion).
- `game_*.rs` — the `game` module against `tests/common` rigs:
  `gsim.rs` builds a preset galaxy (`select_galaxy(st, 3)` = `WOTOK`,
  8 records) with record/machine/ship offset helpers; `rig.rs` bundles
  `Vm`+`State`+`Rng`+recording `VmHost`+`MemFs` for VM/tick/UI tests.

Test rigs record every `Call` a routine emits, so host-side effects are
asserted as ordered call sequences rather than mocked pixels.
