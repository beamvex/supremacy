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

## Who these docs are for

You don't need reverse-engineering experience to work on this crate. You
need:

- **Comfortable Rust** — structs, traits, `impl` blocks, `Result`/`Option`,
  slices and `Vec<u8>`. Nothing in the port is exotic (there is *no*
  `unsafe` anywhere — `#![forbid(unsafe_code)]` is set crate-wide).
- **Patience with hex** — every address is written in hex because every
  number is a real offset into the original binary. `0x3A` is just 58;
  the docs keep it hex so you can grep the disassembly for the same
  number.
- **A little 1989-era PC vocabulary** — "segment", "interrupt",
  "I/O port", "palette register". All of it is explained, from scratch,
  in [concepts.md](concepts.md). If you have never seen `seg:offset`
  notation before, read that page first — it is the prerequisite for
  everything else.

## Start here

The docs assume some DOS vocabulary — segments, `ds:`/`cs:` offsets,
interrupts, I/O ports, VGA planes. If any of that is new, read
[concepts.md](concepts.md) first: it explains every notation these pages
use in about ten minutes. Then pick your path:

- **"I just want to run it"** → § Building and testing, § Binaries.
- **"Show me the whole thing working end to end"** →
  [walkthrough.md](walkthrough.md) — two narrated traces: record 0 from
  disk to PPM, and one frame of `supremacy-run`.
- **"I want to understand the graphics pipeline"** →
  [gph](gph.md) → [lzss](lzss.md) → [video](video.md) →
  [palette](palette.md) → [assets](assets.md).
- **"I want to understand the game engine"** →
  [game-state](game-state.md) → [game-vm](game-vm.md) →
  [game-tick](game-tick.md) → [game-ui](game-ui.md) →
  [game-text](game-text.md) → [game-dialogs](game-dialogs.md).
- **"I want to understand the host layer / run the game"** →
  [platform](platform.md) → [args](args.md) → [audio](audio.md) →
  [front](front.md).
- **"I want to test or verify"** → [smoke](smoke.md) →
  [difftest](difftest.md) → § Test suite.
- **"I want to fix a bug"** → find the misbehaving subsystem below
  (§ Module guide), read its page, then grep `rust/src/` for the `cs:`/
  `ds:` addresses cited there — every ported routine carries its asm
  address in a doc comment.

## Prerequisites

- A Rust toolchain (`cargo`; edition 2021 — `#![forbid(unsafe_code)]` is
  set, everything is safe Rust).
- The shipped game data in `../GAME/` (asset sets `MCG.BIN`/`MCG.GPH`,
  `*IN`, `*DTH`, `*WIN` pairs for the other stems) and
  `../decompiled/GAME.unpacked.exe` — the binaries expect these relative
  to the `rust/` directory.
- Optional: `minifb` build deps for `supremacy-run` (a window opens),
  `nasm` + `dosbox-x` only for the differential harness.

On macOS/Linux `cargo` alone covers everything except the two optional
extras. `minifb` needs the usual X11/wayland dev packages on Linux; the
`audio` feature needs `cpal`'s platform deps (ALSA headers on Linux).
Neither is needed for `cargo test` or the `supremacy`/`supremacy-smoke`
binaries.

## Building and testing — step by step

```bash
cd rust

# 1. Build the library and the `supremacy` binary.
cargo build

# 2. Build with the playable window binary too (optional).
cargo build --features minifb

# 3. Run the whole integration suite (needs ../GAME and ../decompiled).
cargo test

# 4. House checks — both are part of the repo's style rules.
cargo clippy --all-targets -- -W clippy::pedantic
cargo fmt
```

Step 3 is worth a caveat: the integration tests read the *real* game
files — `../GAME/MCG.GPH` etc. and `../decompiled/GAME.unpacked.exe`.
If you cloned the repo without the game-data directory, library unit
tests still compile but the integration tests fail at file-open time.
That is expected; the suite is a black-box check against the shipped
assets, not a self-contained fixture set.

`#![forbid(unsafe_code)]`, `#![warn(missing_docs)]` and
`#![warn(clippy::pedantic)]` are set in `lib.rs`. Code style: one
item (function/type) per file where practical, methods ≤ 23 lines,
files ≤ 200 lines; `u16::try_from(..).unwrap_or(0)` is the standard
`ds:`-offset narrowing idiom (see [concepts.md](concepts.md) §8).

A quick smoke test that everything works end-to-end:

```bash
cargo run --bin supremacy -- ../GAME M ' ' A --idx 0 --out frame.ppm
# → writes a 320×200 P6 PPM of asset record 0 drawn in MCGA layout.
cargo run --bin supremacy-smoke -- ../GAME --ticks 300
# → prints e.g. `smoke steps=300 video=… pal=… audio=… state=… rng=…`
```

## The big picture — what happens when it runs

Before diving into modules it helps to hold the whole pipeline in your
head. Running `supremacy GAME M ' ' A --idx 0` performs exactly what
the 1991 `.exe` does at boot, minus the parts this binary doesn't need:

```
GAME.EXE on disk                 rust/ port
───────────────                  ──────────
MZ + EXEPACK image        →      exepack::unpack      (only if you ask;
                                 GAME.unpacked.exe is already shipped)
PSP tail "GAME M M A"     →      args::parse          → Config {video,
                                 keyboard_mouse, sound, timer_variant}
video letter 'M'          →      Video::stem() = "MCG",
                                 Video::template_seg() picks the mode
                                 parameter block
MCG.GPH + MCG.BIN         →      assets::load_set     → AssetSet {
                                 records[], bin[] }   (index + fixup
                                 + compressed heap)
record 0: x,y,w,h,
ofs_para, mode_flag       →      lzss decoder         → pixel bytes
                                 written into Screen::buf at the
                                 mode's VRAM layout (di_at(x,y))
palette table in the exe  →      palette::from_dac    → Palette
                                 (256 × 8-bit RGB)
Screen::buf               →      Screen::pixels()     → 320×200 indices
                                 → Palette::rgb each → PPM on disk
```

The playable binary (`supremacy-run`) adds the other half: a `front::Game`
runner that calls `Game::step` once per window frame. Each step runs the
same sequence the asm's main loop does — mirror input into `ds:` cells,
run one unit of the title/ending machine if parked there, else one shell
or galaxy frame; each frame runs `tick_step` (one slice of simulation),
`seq_step` (the animation sequencer), `menu::service` (hotspot input),
then whatever small reprint/scroll jobs are armed. Anything the asm did
by poking hardware arrives at `front::Host` as a `Call` enum value and
becomes pixels in the `minifb` window or samples in the `cpal` stream.

## Layout

```
rust/
├── Cargo.toml          # crate `supremacy`; optional `minifb`/`audio` features
├── src/
│   ├── lib.rs          # module docs + `#![forbid(unsafe_code)]`
│   ├── args/           # GAME <V> <K> <S> command-line parser
│   ├── assets/         # .GPH index + .BIN heap loading
│   ├── audio/          # OPL2 / MPU-401 / PC-speaker device halves + drivers
│   ├── exepack/        # Microsoft EXEPACK unpacker
│   ├── front/          # concrete VmHost frontend + minifb window
│   ├── game/           # state block, event VM, tick, UI shell
│   ├── gph/            # 12-byte .GPH index records
│   ├── lzss/           # Okumura LZSS decoder (4 asm variants)
│   ├── palette/        # 256×3 master DAC table + runtime ops
│   ├── platform/       # int 21h files, int-8 timer, int-9 kbd, int-33h
│   ├── smoke/          # headless CI run → FNV-1a digest (GAM-24)
│   ├── video/          # per-mode VRAM + draw-image-by-index
│   └── bin/
│       ├── supremacy.rs   # `supremacy` — renders one record → PPM
│       ├── difftest.rs    # `difftest` — DOSBox-X differential driver
│       ├── run.rs         # `supremacy-run` — playable minifb shell
│       └── smoke.rs       # `supremacy-smoke` — headless digest runner
└── tests/              # integration tests + fixtures + common rigs
```

### Why so many tiny files?

The crate follows the repo's style rule of *one item per file*: a
module like `lzss/` is not one 400-line `lzss.rs`, it is twenty files —
`decoder.rs` (the state struct), `step.rs` (one loop iteration),
`copy_match.rs`, `literal.rs`, `push.rs` (the ring write) and so on.
Every doc page names the file each item lives in (`push.rs` next to
"`push`"), so when a doc says "`push` stores the emitted byte and wraps
`& 0xFFF` (`push.rs`)" you can open exactly that file. Methods are
capped at 23 lines and files at 200 lines, so each file reads top to
bottom in a minute.

### The rest of the repo

```
repo root/
├── GAME/                 # shipped game data — .BIN/.GPH sets, not code
├── decompiled/           # the reverse-engineering artefacts:
│   ├── GAME.unpacked.exe #   the image every doc's file offsets cite
│   ├── ANALYSIS.md       #   the full RE report
│   ├── FUNCTIONS.md      #   verified function/address map
│   ├── disasm/           #   GAME.linear.asm — the disassembly
│   ├── tools/            #   Python reference impls (extract_png.py, lzss.py)
│   └── fixtures/         #   captured dumps used by tests
├── difftest/             # probe.asm (the in-emulator TSR) + run output
└── docs/rust-port/       # these pages
```

## Address spaces

All numbers in doc comments are real addresses in one of three spaces —
see [concepts.md](concepts.md) §2 for the full explanation and a worked
example:

| space | meaning | conversion |
|---|---|---|
| file offset | byte offset in `GAME.unpacked.exe` | — |
| `cs:` near offset | inside the main code segment | file − `0x292B0` |
| `ds:`/`dgroup` offset | inside the data segment | file − `0x106A0` |

The port keeps game data addressed by `ds:` offsets exactly as the asm
uses them. `Vm::from_image`/`State::from_image` therefore read the image
at `ds_ofs + DS_BASE` (`0x106A0` — dgroup `0xFAA` × 16 + the `0xC00`
MZ header; verified against a DOS dump of the running original). The
frontend `Host` maps `cs:` reads to file
offsets by adding `0x30000` (game code) or `0x292B0` (video-template
segment).

The practical upshot: **when a doc comment says `[0x91B8]`, grepping the
code for `0x91B8` finds the port of that exact asm access**, and the
disassembly line at file offset `0x91B8 + 0x106A0 = 0x19858` shows the
original instruction. One number, three coordinate systems, two
constant subtractions.

## Module guide

Each page below pairs the module's public API with the asm it ports.
Difficulty is a rough guide: ○ = gentle, ◐ = needs concepts.md, ● =
deeply asm-flavoured.

| module | asm provenance | notes | depth |
|---|---|---|---|
| [concepts](concepts.md) | — | primer: segments, ints, ports, VGA, notation | ○ |
| [walkthrough](walkthrough.md) | — | end-to-end traces: disk→pixel, one frame | ○ |
| [`exepack`](exepack.md) | stub at file `0x23A90` | EXEPACK unpacker + repacker | ◐ |
| [`lzss`](lzss.md) | `0x2EBEF`, `0x2EC8C`, `0x2FC5D`, `0x2FCF9` | Okumura LZSS, 4 variants | ◐ |
| [`gph`](gph.md) | `0x320D8` fixup loop | 12-byte image index records | ○ |
| [`args`](args.md) | `0x31FF8`–`0x3207D` | PSP-tail argument parser | ○ |
| [`assets`](assets.md) | `0x3208E`–`0x3215A` | `.BIN`/`.GPH` loaders | ○ |
| [`palette`](palette.md) | file `0x15034` (dgroup `0x4994`) | 256×3 6-bit DAC table | ○ |
| [`audio`](audio.md) | `0x3470`–`0x3F52`, `0x0E9F` | OPL2, MPU-401, PIT speaker, drivers | ● |
| [`platform`](platform.md) | `0x31D74`, `0x353D`+ | DOS services layer | ◐ |
| [`video`](video.md) | `0x2EB8A`, `0x2FB36`, `0x30620`, `0x310CE` | VRAM + draw-by-index | ◐ |
| [`front`](front.md) | GAM-9 frontend | `VmHost` impl, `minifb` window | ● |
| [`difftest`](difftest.md) | GAM-25 | DOSBox-X side-by-side harness | ● |
| [`smoke`](smoke.md) | GAM-24 | headless tick run → hash digest | ○ |
| [`game`](game-state.md) | `ds:0x7D39` block etc. | state, VM, tick, UI — see below | ● |

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

All four take the DOS letter args positionally; `tail_bytes` synthesises a
PSP command tail (count byte, space, letters at `0x82`/`0x84`/`0x86`) so
`args::parse` sees exactly what the asm sees (see
[concepts.md](concepts.md) §4 and [args.md](args.md)).

- `supremacy <GAME-dir> [V] [K] [S] [--set STEM] [--idx N] [--exe PATH]
  [--out frame.ppm]` — parses the PSP-style tail, loads one asset set,
  draws record `idx` into a `Screen` and writes a `320×200` P6 PPM
  through the mode palette (`bin/supremacy.rs`). Good first look at the
  pipeline: `args::parse` → `assets::load_set` → `Screen::draw_image` →
  `Palette::rgb` → file.
- `supremacy-run <GAME-dir> [V] [K] [S] [--set STEM] [--exe PATH]
  [--preset N] [--seed N] [--galaxy]` (feature `minifb`) — boots a
  `front::Game` into the `0x85B1` title attract on the `*IN` set
  (`--preset`/`--galaxy` skip it to a fresh galaxy via `select_galaxy` +
  `new_game`; saves via `HostFs` rooted at the game dir) and runs the
  `minifb` loop (`bin/run.rs`). See [front](front.md).
- `supremacy-smoke <GAME-dir> [V] [K] [S] [--ticks N] [--script F.scr]
  [--expect "k=v,…"] [--frame F.ppm]` — the headless CI mode: steps the
  game deterministically with no display or audio device and prints a
  `key=hex` digest of the VRAM, palette, PCM and `ds` state; `--expect`
  gates it (exit 3 on mismatch). See [smoke](smoke.md).
- `difftest <script.scr> [--root DIR] [--port-only] …` — the DOSBox-X
  differential driver (`bin/difftest.rs`); see [difftest](difftest.md).

Typical invocations:

```bash
# Look at the title backdrop (record 0 of the MCGA set):
cargo run --bin supremacy -- ../GAME M ' ' A --idx 0 --out title.ppm

# Playable window, straight into a fresh galaxy:
cargo run --bin supremacy-run --features minifb,audio -- \
    ../GAME M ' ' A --preset 3 --galaxy

# Deterministic CI check, gated on the pinned golden:
cargo run --bin supremacy-smoke -- ../GAME --ticks 300 \
    --expect "steps=300"
```

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
asserted as ordered call sequences rather than mocked pixels (see
[concepts.md](concepts.md) §9).

## Troubleshooting / FAQ

- **`cargo test` fails opening `../GAME/…` or `GAME.unpacked.exe`** — the
  suite needs the game data next to the repo. Run from `rust/` (not the
  repo root) and check `../GAME`/`../decompiled` exist.
- **`supremacy-run` won't compile** — it needs `--features minifb` (the
  `[[bin]]` is gated with `required-features`). Audio additionally needs
  `audio`. `cargo build --features minifb,audio` builds everything.
- **The window is silent** — audio is a separate feature; rebuild with
  `--features minifb,audio`. `S` arg picks the driver (`A` AdLib, `R`
  Roland, `T` Tandy, anything else PC speaker).
- **Colours look wrong in the PPM** — check the video letter. `supremacy
  GAME E …` draws into EGA bitplanes and maps through the fixed
  16-colour `ega16` palette; only `M` (MCGA) uses the 256-colour DAC.
- **A smoke `--expect` mismatch after changing code** — deliberate
  behaviour change? Regenerate the golden (`tests/smoke.rs` doc header
  has the commands). Otherwise the failing field names the subsystem:
  `video` = framebuffer, `pal` = palette, `audio` = PCM, `state` = `ds`
  cells, `rng` = the LCG stream.
- **"Where is function X in the asm?"** — every ported item's doc comment
  cites its `cs:`/`ds:`/file offset. File offset → `decompiled/disasm/
  GAME.linear.asm` is one search away.

## Reading the code alongside the docs

Every ported item carries its provenance in a doc comment, e.g.

```rust
/// `cs:0x773D` — the event-VM fetch loop …  (file `0x3773D`)
```

Reading one routine, end to end:

1. Find the routine's doc comment (or the page's table entry) → the
   `cs:`/`ds:`/file offset.
2. Open `decompiled/disasm/GAME.linear.asm` at the file offset — the
   `00037…` line labels are file offsets.
3. Read the asm's basic blocks against the Rust — the port keeps the
   same order, so `lodsb` → a slice read, `cmp/je` → a `==`/`if`,
   `call [0x125A]` → a `Call` variant + `svc`.
4. Constants (`0x91B8` etc.) are named in `game/consts.rs`/`cells.rs`
   but the *number* is always the real offset — grep it to find every
   reader/writer.

Two conventions to internalise: `[x]` in prose = the value stored at
`x` (so `[0x91B8]` reads through `State::word`), and a bare `0x…` = the
address itself (the `ds:`/`cs:` label). The `+0xC` shorthand = "field 12
bytes into the record".
