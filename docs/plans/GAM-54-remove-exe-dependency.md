# GAM-54 — Remove the `GAME.unpacked.exe` dependency

Investigation of how `rust/` uses the unpacked 16-bit image and a plan to
replace the runtime file dependency with embedded data.

## TL;DR

The port treats `decompiled/GAME.unpacked.exe` (233,104 bytes) as a
read-mostly ROM loaded at startup. Because `*.exe`/`*.EXE` are git-ignored,
the file isn't in the repo — a fresh clone builds but can't run
`supremacy`/`supremacy-run`/`supremacy-smoke`, and most of `cargo test`
fails at file-open. The fix: extract the regions the port actually reads
into committed `.bin` blobs, embed them with `include_bytes!`, and keep
`--exe` as an optional override.

## What the exe provides today

`Host::img` (`src/front/host.rs`) is the whole file as a `Vec<u8>`, read —
and in two places written — by raw file offsets:

| file offsets | size | contents | consumers |
|---|---|---|---|
| `0x0000..0x0C00` | 3 KiB | MZ header + reloc table | unused |
| `0x0C00..0x2900` | 7 KiB | PC-speaker + Tandy tracker modules | `audio::Pc::new` (`0xC00..0x2900`), `audio::Tandy::new` (`0x1950..0x2900`) |
| `0x2900..0x4970` | 8 KiB | AdLib/Roland driver module | `audio::Ar::new` |
| `0x4970..0x106A0` | 48 KiB | embedded song records | `front::audio::Audio::song` — `img[seg·16 + 0xC00..]`; segs seen: `0x3D7` (title), `0x5B7`/`0x95A` (endings), `0xC3E` (init) |
| `0x106A0..0x206A0` | 64 KiB | the whole `ds:` image (`DS_BASE = 0x106A0`) | `Vm::from_image` (low + save block + hi — contiguous), `State::from_image`, `font::draw` glyph tables, palette tables (`WORK_OFS 0x14735`, `ROWS_OFS 0x14A34`, `DAC_OFS 0x15034`), `menu_text` `ss:` stream (`STREAM = 0x202F0`) |
| `0x206A0..0x292B0` | 36 KiB | code segments not yet ported | unused today |
| `0x292B0..0x38E90` | 62 KiB | `cs:` space (`TPL_OFS = 0x292B0`): seq command streams (via `ds:[0x9198]`), sin table `cs:0x1284`, DAC stage buffer `cs:0x1384`, RNG seed cells `cs:0x55A2` (**written** — `map::grid`'s `cs_put`, difftest `POKEW`), 16×24 menu font (file `0x2B474`) | `Host::cs_word`, `map::pos`, `map::grid`, `text::mcg::blit`, difftest snap/poke |

Net footprint: ~188 KiB of the 228 KiB image is reachable.

### Where the dependency lives

- `src/bin/supremacy.rs`, `run.rs`, `smoke.rs` — `--exe` flag defaulting
  to `../decompiled/GAME.unpacked.exe`; they exit with `exe: <err>`
  without it.
- `src/bin/difftest.rs` — port side reads the file directly; the DOS side
  boots `GAME/GAME.EXE` under DOSBox-X (see "out of scope" below).
- `src/front/host.rs`, `game/vm.rs`, `game/state.rs`,
  `palette/{consts,present,adj,fade}.rs`, `audio/{ar,pc,tandy}.rs`,
  `front/{font,text,map}/`, `front/audio/engine.rs` — the readers above.
- `rust/tests/` — `game_vm.rs`, `game_state.rs`, `game_front.rs`,
  `smoke.rs`, `difftest.rs`, `palette_dac.rs`, `audio_*.rs` open the file
  (~9 files). `exepack_unpack.rs` needs `GAME/GAME.EXE` and
  `tests/fixtures/SUPCHT.EXE` — both also git-ignored.
- `docs/rust-port/README.md` documents `../decompiled/GAME.unpacked.exe`
  as a prerequisite.

`Play NOW.exe` at the repo root is a red herring: per `decompiled/
ANALYSIS.md` it's a renamed DOSBox Win32 launcher, not the game and not
referenced by the port.

## Recommended plan — Phase 1 (this ticket's follow-up)

Commit the consumed data as blobs; embed; keep `--exe` as an override.

1. **Extract three blobs** into `rust/data/` (`.bin` passes `.gitignore`):
   - `low.bin` — file `0x0C00..0x106A0` (audio modules + songs)
   - `dgroup.bin` — file `0x106A0..0x206A0` (the 64 KiB `ds:` image)
   - `cs.bin` — file `0x292B0..0x38E90` (the `cs:` data space)

   The MZ header and the `0x206A0..0x292B0` hole are never read and stay
   out — the committed bytes are game *data*, not the executable stub.
2. **New module `src/img.rs`**: `pub fn image() -> Vec<u8>` builds a
   `vec![0; 0x38E90]` and copies the three `include_bytes!` slices to
   their file offsets. Every `img[file_ofs]`/`img.get(..)` call site —
   `cs_word`, fonts, palettes, `menu_text`, `Audio::song`, the drivers —
   keeps working **unchanged**; `Host::img` stays a `Vec<u8>` because the
   RNG seed cells are written through it.
3. **Binaries**: `let img = exe.map(std::fs::read).transpose()?
   .unwrap_or_else(img::image)`. `--exe` remains for alternate images and
   difftest debugging; without it the binaries are self-contained and
   run from any cwd.
4. **Tests**: replace `read(&root().join("decompiled/GAME.unpacked.exe"))`
   with `supremacy::img::image()`. `cargo test` then needs only `../GAME`
   (committed). `exepack_unpack.rs`/`difftest.rs` keep their real-exe
   requirement — gate them with an early return when the file is absent,
   matching the documented "black-box check against shipped assets"
   posture.
5. **Regeneration**: a small `extract-img` bin (or
   `decompiled/tools/extract_img.py`) that runs the crate's own
   `exepack::unpack` on `GAME/GAME.EXE` and writes the three blobs —
   byte-exact provenance, no manual hex-surgery. Document it in
   `docs/rust-port/README.md` and drop the `../decompiled` prerequisite.
6. **No behavioural change**: the bytes are identical, so the smoke
   digest (`supremacy-smoke --ticks 300 --expect …`) is the golden check.

## Out of scope / stays as-is

- **`difftest`'s DOS side** needs `GAME.EXE` by definition — it exists to
  compare the port against the original under DOSBox-X. Dev-only; document
  it, don't remove it.
- **`exepack`** — the unpacker is the provenance/regeneration path, not a
  runtime dependency.

## Phase 2 (optional follow-up tickets)

- **Trim `cs.bin`**: most of the `cs:` span is code; only scattered
  tables/streams are read. Needs a coverage pass (the `[0x9198]` seq
  streams are data-driven pointers — instrument `cs_word` during a smoke
  run to collect the live ranges, then pack just those).
- **Typed data tables**: replace raw `img[ofs]` reads with named embedded
  tables (`DS_IMAGE`, `SONGS`, `MENU_FONT`, …) and small accessor methods.
  Cleaner ownership but ~15 call sites of churn and it dilutes the port's
  "every constant is a real file offset" convention — worth doing only if
  the blobs prove awkward.
- **`.gitignore`/`Play NOW.exe`**: the Win32 launcher could be deleted
  outright (repo hygiene, not code).

## Alternatives considered

- **`include_bytes!` the exe directly** — still needs the git-ignored file
  at build time; fails on a fresh clone. Rejected on its own (it's
  equivalent to this plan with one whole-image blob instead of three).
- **`build.rs` unpacks `GAME/GAME.EXE` via `exepack::unpack`** — still a
  git-ignored build input. Useful *only* as the regeneration path in
  step 5.
- **Port the data into Rust tables** — the true end state of a
  decompilation, but a large rewrite of script/string/song data with real
  byte-exactness risk. Not this ticket.

## Risks / notes

- The blobs are derived from `GAME.EXE` — same redistribution status as
  the already-committed `GAME/*.BIN`/`*.GPH` assets. Worth a maintainer
  sign-off, but consistent with existing practice.
- ~1 new module, 3 committed blobs, 3–4 binaries and ~10 test files
  touched; all mechanical, no behaviour change.
- Verification: `cargo test` on a clone without `decompiled/*.exe`;
  unchanged smoke digest; `cargo clippy --all-targets --
  -W clippy::pedantic` + `cargo fmt`; `supremacy-run` boots with no
  `--exe`; `extract-img` output matches the committed blobs.
