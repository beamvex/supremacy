# `game` (1/6) — state block, record layouts, init, save/load

This is page 1 of the `game` module docs — see the
[README](README.md#module-guide) for the full page list.
[concepts.md](concepts.md) explains the `ds:`/`cs:` notation used
throughout.

## What this module covers

Everything about the game's *data*: the `0x1F8B`-byte saveable state
block every routine reads and writes, the layouts of the three record
types inside it (planets/factions, machines, ships), the PRNG, the
galaxy presets, the new-game init, and savegame I/O.

## The big idea: state is a flat memory image, not structs

The single most important thing to understand about `game/` is that the
original has no objects — it has one big `ds:` block where every
variable lives at a fixed offset. `pop` is not a field on a Planet
struct; it is "the `u16` at `record + 0x2A`". The port deliberately keeps
this: `State` is a byte array plus accessors, and the record "types"
below are just *offset constants* — `rec::F_POP` is `0x2A`, not a field.

That design is why a savegame is literally a memory dump: the `0x1F8B`
bytes the save routine writes *are* the game state. It also means every
game variable is inspectable by offset — `st.word(0x91B8)` reads the
same cell the asm's `mov ax,[0x91B8]` did, and a difftest `D01` dump
offsets directly against this page's tables.

## `State` (`state.rs`, `consts.rs`)

The saveable game-state block: exactly the `0x1F8B` bytes at
`ds:0x7D39..0x9CC4` that the save routine writes whole (`int 21`
create/write at file `0x383F3`) and load reads back (`0x383CE`), and
that the heap-staging copies (`0x8432` save/`0x844B` restore) move
to/from the loader heap segment. Because a save file *is* this byte
range, every game variable lives at a fixed `ds:` offset — the "cells"
the docs keep citing.

- `block: Box<[u8; STATE_LEN]>` — `STATE_OFS = 0x7D39`,
  `STATE_LEN = 0x1F8B`. Accessors take **real `ds:` offsets** and
  subtract `STATE_OFS` internally: `word`/`set_word`, `dword`/
  `set_dword`, `byte`/`set_byte` (all little-endian) — so code reads
  `st.word(0x91B8)` exactly where the asm reads `[0x91B8]`.
  (`word(0x91B8)` = bytes `0x91B8..0x91BA` LE; `dword` likewise 4 bytes.)
- `from_image(img)` — the shipped initial state at file
  `STATE_OFS + DS_BASE` (`0x183D9`): the game ships with its "new game"
  template baked into the data segment, so booting `from_image` is
  booting the shipped defaults.
- `rec_ofs(i)` — `cs:0x5E3A` accessor: `[0x9158] + i·0x3A` — the
  `ds:` address of record `i` (multiply index by the `0x3A` = 58-byte
  record stride, add the array base read from the cell). The base is a
  *cell*, not a constant — `select_galaxy` repoints it per preset.
- `planets()` — `[0x91B8]` = planet count = index of the appended
  player-faction record (the faction is stored as an extra record
  right after the last planet — so record `planets()` is the faction,
  and its `+0x26` field is the player's credits).

### `consts.rs` — `ds:` cell map (the real asm addresses)

State/records: `STATE_OFS 0x7D39`, `REC_BASE 0x9158` (array anchor),
`REC_TOTAL 0x91B6`, `REC_COUNT 0x91B8`, `DIFFICULTY 0x91E4`,
`REC_SEL/REC_CUR/REC_ALT` `0x917C`/`0x9184`/`0x9178`, `SEL_IDX 0x9164`
(selected record index — preset-init 6/14/30), `REC_STRIDE 0x3A`,
`SEL_REC 0x917C`, `TICK 0x91BA` (tick-in-day, `1..0x41`), `DAY 0x91BC`,
`SEQ 0x91CE` (tick code counter), `DIRTY0 0x9144` / `DIRTY1 0x9146`
(deferred-refresh flags), `SNAP0/1 0x9CBC`/`0x9CBE`, `UIMODE 0x91CD`.

Arrays: `MACH_BASE 0x9491` (`0x20` × `0x28` machine records),
`FLEET_BASE 0x9991` (`0x17` × `0xC` ship records).

Globals: `G_FAMINE 0x822B` (halves growth), `G_FARMTECH 0x8229`,
`G_MINETECH 0x822C`, `G_DEEPMINE 0x822D` (mine `0x32` vs `0x19`),
`G_KILL_ON/G_KILL_REC/G_KILL_AUX 0x821F/0x8220/0x8224` (delayed
depopulation), `QUEUED_REC 0x9170`, `SEL_MACH 0x9180`,
`SCAN_REC 0x918C`, `MSG_REC 0x9140`, `LINK_REC 0x914C`, `REINF_REC
0x9150`, `MSG_BUILD 0x8314`, `MSG_MINE 0x8318`.

Frame/UI: `FRAME 0x91D4`, `REDRAW 0x91EE`, `PANEL_REQ 0x91EF`,
`TYPE_SEL 0x91ED`, `TYPE_REC 0x9154`, `TYPE_BASE 0x9B12` (stride
`0x30`), `PANEL_ON 0x91DB`, `SEQ_DELAY 0x91A0`, `SEQ_PHASE 0x91D7`,
`SEQ_PERIOD 0x91D6` (`0xFF` disables), `SEQ_PTR 0x9198` (`cs:` stream),
`SEQ_AUX 0x919A`, `SND_ON 0x91D3` (`0xFF` = muted).

Input/menu: `BUTTONS 0x9CDA`, `CUR_X 0x9CD6`, `CUR_Y 0x9CD8`,
`HOT_LIST 0x9194` (`0x14`-byte records), `HOT_COUNT 0x91A8`,
`HOT_SEL 0x9CC8`, `ARMED 0x91CA` (debounce), `HOT_IMG 0x9188`,
`KEY_PEND 0x9CCB`/`KEY_CODE 0x9CCC`, `K_MODE 0x9CCD`, `TANDY 0x9CC6`,
`DRAG 0x91D5`, `SEL_SAVE 0xA81A`.

Read these as a field guide, not a glossary to memorise: when a routine
doc says "the `DIRTY0` flag", grep the constant and read the asm at that
offset to see every writer/reader.

### `cells.rs` — shell/UI `ds:` cells

Ticker: `TICK_X 0x91C0`, `TICK_PTR 0x915C`, `TICK_PUT 0x9160`,
`TICK_LEFT 0x91C2` (`0` disables), `TICK_WAIT 0x91D2`, `TICK_BASE
0x928D` (record stream base / `0xFE` wrap target), `TICK_FLAG 0x91E8`,
`DIGI 0x9CC0` (digit scratch), `LIST_MAX 0x91C6` (row-click bound).
Blink: `BLINK_T 0x91D1`, `BLINK_HOLD 0x91F2`, `BLINK_PH 0x91F3`,
strings `BLINK_A 0x77AA`/`BLINK_B 0x77E3` at `(0x4D,0x67)`.
Shell: `OVERLAY_F 0x91EA`, `SHELL_A6 0x928B`, `SHELL_SRC 0x91A6`,
`MENU_SEL 0xA0AA`, `MENU_LIST 0xA0AC`, `MENU_COUNT 0x31`,
`OVAL_TAB 0xA8` (row half-width table), `ALIEN_NAME 0x71B9`.
Dialogs: `DLG_FLAG 0x91DA`, `LOAD_TO 0x91E5` (post-load screen switch),
`LINE_MODE 0x9CC5`, `SLOT_MODE 0x9CCA`, `NAME_LEN 0x4FCB`,
`NAME_MAX 0x4FCC` (`0x1E` typing / `9` after), `IN_DIRTY 0x4FCA`,
`IN_DONE 0x4FCF`, `DLG_SEL 0xB05A`/`DLG_LIST 0xB05C` (3 records),
`CNF_SEL 0xB098`/`CNF_LIST 0xB09A` (2 records); strings `S_PROMPT
0x78E5`, `S_LEGEND 0x78C3`, `S_ERROR 0x7929`, `S_SAVED 0x7C2E`,
`S_LOADED 0x7C50`, `S_CNF_T 0x78B8`, `S_CNF_B 0x7C72`.

## Record layouts

The galaxy's planets are `0x3A`-byte records in an array anchored at
`[0x9158]`; machines (stations/farms/miners the player builds) are
`0x28`-byte records at `0x9491`; ships are `0xC`-byte records at
`0x9991`. Offsets below are byte offsets inside one record — `+0xC` in
doc shorthand — all little-endian.

What the three arrays are in game terms: a **planet record** is a world
(name, population, stocks, owner, kind) *and* — one extra record past
the planet count — the player's faction (credits, stocks). A **machine
record** is an installed structure on a planet (solar array, core
miner, farm — `M_TYPE`), plus the build/countdown timers. A **ship
record** is a fleet unit (guns, crew, load, the `S_LINK` dock pointer).
Cross-links are by `ds:` offset (`M_LINK`, `S_LINK`, `REINF_REC` …) —
a machine's `M_HOST` is the host planet's serial index `+0x28`, its
`M_LINK` the record pointer.

### `rec.rs` — the `0x3A`-byte planet/faction record

Recovered from new-game init (`0x330A0`–`0x331C2`), accessor `cs:0x5E3A`,
planet sim `cs:0x3D1E`, defence `cs:0x4434`, stat refresh
`cs:0x3C6D`/`0x3533`:

| off | field | notes |
|---|---|---|
| `0x00` | `F_WORD0` | zeroed at init |
| `0x0C` | `F_KIND` | `0xFFFF` empty; `0xA` inhabited; `7` fallen/conquered; `4` unclaimed; `1` colonisation en route |
| `0x12` | `F_STYPE` | station/ambient type → SFX selector |
| `0x13` | `F_RATE` | rate × kind for display |
| `0x0E` | `F_NAME` | 8-byte name + `0x21` terminator (`LIFELESS!` at init) |
| `0x18`/`0x1C`/`0x20` | `F_TIMER_A/B/C` | pending-event timers |
| `0x1D` | `F_GROWTH` | `coverage/3` (halved by famine), per tick |
| `0x1E` | `F_TAX` | slider `0..=100` |
| `0x1F` | `F_COVER` | food-coverage %, tracks `100 − tax` |
| `0x21` | `F_DECLINE` | `(100−coverage)/4 + tax/4` |
| `0x22` | `F_DEFENCE` | fleet-scan weighted strength |
| `0x24` | `F_OWNER` | `2` = player; colonies roll `{1,3,4,5}` |
| `0x26` | `F_TROOPS`/`F_CASH` | garrison on planets; **credits on the faction record** |
| `0x28` | `F_SERIAL` | planet serial index |
| `0x29` | `F_29` | defence weighting `+1` input |
| `0x2A` | `F_POP` | population; `pop/240` food/tick; cap `0x7530` |
| `0x2C` | `F_FOOD` | drained by pop, refilled by farms |
| `0x2E` | `F_ATTACK` | inbound/hostile `Σ ship pow` |
| `0x30`/`0x32`/`0x34` | `F_MINERALS`/`F_FUEL`/`F_ENERGY` | stocks |
| `0x36` | `F_CREDITS` | `u32`; init `0xC350 + rand(0x4E20)` |

### `mach.rs` — the `0x28`-byte machine/station record (`ds:0x9491`, `0x20` entries)

`M_TIMER 0x0C` (build/repair countdown), `M_ACC 0x0A` (produce
accumulator → `0x4B0` harvest), `M_OPS 0x18` (mining ops left),
`M_LINK 0x1A` (linked record), `M_TYPE 0x1E` (`3` solar / `6` core-miner
/ `7` horticultural), `M_FLAGS 0x1F` (`0x1` countdown, `0x4` mining mode,
`0x8` mining running, `0x10` powered, `0x20` online), `M_HOST 0x21`
(host planet index), `M_BUILD 0x22` (colonise countdown),
`M_DEPOSIT 0x26` (mineral deposit).

### `ship.rs` — the `0xC`-byte fleet/ship record (`ds:0x9991`, `0x17` entries)

`S_POW 0x00` (strength), `S_LINK 0x02` (docked-at record, `0` = free),
`S_UNK4 0x04`, `S_FLAGS 0x06` (`0x2` armed, `0x8` reinforce-exempt),
`S_CREW 0x07` (experience), `S_GUNS 0x08`, `S_LOAD 0x0A`.

## `Rng` (`rng.rs`) — `cs:0x55B2` PRNG (file `0x2E862`)

32-bit LCG over the two-word state `[cs:0x55A2]`/`[cs:0x55A4]`:
`s = 13·s + 7` (built from two doublings, a `+7` pair and two
recombination adds), fold `r = lo(s) ^ hi(s)`.

An LCG (linear congruential generator) is the simplest PRNG: multiply
the state by a constant, add a constant, take the low bits. The asm
builds `13·s` out of shifts and adds because a 16-bit `mul` is what it
has — `13·s = (s<<3) + (s<<2) + s`-shape recombination the routine's
two-word adds implement. `Rng::set_state`/`state` expose the raw dword
so `difftest` can transplant the DOS side's seed ([difftest.md](difftest.md)
§ "the RNG problem").

- `range(b)` → `hi16((b+1)·r)` (the asm does `inc bx` then `mul`,
  keeping `dx`) — result ∈ `0..=b`. Not `r % (b+1)`: the asm takes the
  *high* 16 bits of the product — a multiply-shift, which biases
  differently than modulo. Faithful ports keep that.
- `next16()` — second entry point (`bx = 0xFFFF`, file `0x2E93C`), the
  folded word.

## `preset.rs` — galaxy presets

`Preset { base, total, difficulty, banner }`; `PRESETS` holds the four
menu-order pickers (`0x36792`/`0x367B2`/`0x367CD`/`0x367E8`, shared tail
`0x36801`): bases `0x8486`/`0x8486`/`0x8BC6`/`0x8F66`, totals
`0x20`/`0x20`/`0x10`/`0x08`, difficulties `2/2/1/0`, banners
`RORN `/`KRART`/`SMINE`/`WOTOK`.

`select_galaxy(st, idx)` (`0x36801`–`0x3681F`): writes `base` into
`REC_SEL`/`REC_CUR`/`REC_BASE`/`REC_ALT`, `total` → `REC_TOTAL`,
`total−1` → `REC_COUNT` and byte `0x91C6`, `difficulty` → `0x91E4` —
i.e. it points the record array at the preset's `ds:` base and fixes
the counts/difficulty before `new_game` fills it. The presets aren't
data files — they're different `ds:` regions of the shipped image, so
"pick a galaxy" is "repoint the record array".

## `init.rs` — `new_game` (`cs:0x9DAB`, file `0x3305B`–`0x331C2`)

Opens with `stage_save` (the `0x8432` staging-heap snapshot — captures
the post-pick block so `act_stage`'s `0x844B` restore can reset to it).
Assumes `select_galaxy` already ran. Then:

- record 0 (`home_planet`): fixed start — `F_TROOPS =
  DIFF_PARAM[diff]` (`0x1F77`/`0x3A2F`/`0x5811`), `SEL_IDX =
  SEL_INIT[diff]` (6/14/30), credits `0x1879A`, food `0x2AF8`, minerals
  `0x1128`, fuel `0x1C42`, energy `0x1735`, pop `0x22D1`, defence 0,
  owner 2.
- record `[count]` (`faction`): rolled — credits `0xC350 +
  rand(0x4E20)`, stocks `0x9C4 + d·(k+1)` over
  food/minerals/fuel/energy with `d = rand(0x3E8)`, pop `0x5DC +
  rand(0x1F4)`, owner 2.
- records `1..count`: name `LIFELESS!`; all `0..=count` get serial `i`
  and `F_WORD0 = 0`.

`act_newgame` (`cs:0x305B`/`0x9DAB`, file `0x3305B`–`0x331EF`): the
`0x3305B` head — game-song swap (`SndReset`/`Song(0xC3E)`/`Sound(0)`/
`0x89FB`), `[0x91D3]=0`, `Slot(0x1260)`, the shell hotspot install and
`Present(0)` — then `newgame_body` (`cs:0x30A0`, the `0x8427` restart
action's `jmp` target): `new_game`, the `0x33194` repaint
(`0x1260`/`0x125C`/`0x89B3`/`Image(0)`/`Present(0)`, `uimode=0`), then
the `0x331C4` tail — int-33 restrict (`ax=0x14`, mask `0x1E`),
`mark_sel`, `sel_next`, a `0x32`-frame `{vsync, Sound(0)}` wait,
`Native(0x2FD7)`, `Slot(0x1278)` image reload, input reinstall
(`ax=0x14`, mask `0x1F`).

## `save.rs` / `load.rs` — savegame I/O

`State::save(fs, name)` — `0x383F3`: `create` (truncate), close the
leaked handle, `open_write`, `write` the whole `0x1F8B` block, `close`;
`false` mirrors the carry-flag error return (short write counts).
`State::load(fs, name)` — `0x383CE`: `open`, `read` the block, `close`;
`false` on error. Both generic over `DosFiles` — so a `MemFs` test can
save and load without touching disk (see [platform.md](platform.md)).

The "close the leaked handle" detail is real fidelity: the asm opens
with `create`, leaks the handle, reopens with `open_write`, writes,
closes — the port preserves the two-open sequence because on the
original that's literally what the code does. A save file's byte 0 is
`ds:0x7D39`'s byte; there is no header, no version, no checksum.

## `keymap.rs` — the `ds:0x7CBD` keymap

`KEYMAP_BYTES`: 124 shipped bytes = 62 `{code, out}` pairs used by the
`0xE66E` line editor and the `0x32BA8` yes/no poll; shipped at
`ds:0x7CBD` itself (file `0x1835D`), ending exactly at the save block
`0x7D39`. (The earlier `ds:0x76BF` "source" + init-copy story was an
artifact of the stale `+0x10CA0` file mapping — a live `ds` dump shows
the map already at `0x7CBD`.)
`out` classes: `0` exit, `1` backspace, `2` pad-with-spaces+exit,
otherwise the char to append while `[0x4FCB] < [0x4FCC]`. Table order:
Esc (`0x01`→exit), erase keys (`0x0E`/`0x70`/`0x53`), pad keys
(`0x1C`/`0x74`), digits, keypad, `-`/`=`, QWERTY rows, `0x2C..0x35`
row, space, `0x47`/`0x56` `'['` entries.
First match wins (e.g. `0x47` is shadowed by keypad-7).

## Pitfalls

- **Offsets are `ds:`-absolute, not block-relative.** `st.word(0x91B8)`
  is correct; `st.word(0x1479)` (0x91B8 − 0x7D39) silently reads the
  wrong cell.
- **The faction is a record, not a struct.** Player credits live at
  `rec(count) + 0x26` — there is no `player.credits`; `faction_rec`
  computes the record's `ds:` address like any planet's.
- **`rec_ofs` reads `[0x9158]` live.** Repointing `REC_BASE` mid-test
  (a different preset) changes what `rec_ofs(0)` returns.
- **Save = dump, load = dump.** No validation: a truncated `load` can
  leave half-written state, matching the original's carry-flag-or-nothing
  error model.

## File map

| file | item |
|---|---|
| `state.rs` | `State` + `block`/`word`/`dword`/`byte` accessors, `from_image`, `rec_ofs`, `planets` |
| `consts.rs` / `cells.rs` | the `ds:` cell constants above |
| `rec.rs` / `mach.rs` / `ship.rs` | the three record offset tables (`F_*`/`M_*`/`S_*`) |
| `rng.rs` | `Rng` — `next16`, `range`, `set_state` |
| `preset.rs` | `Preset`, `PRESETS`, `select_galaxy` |
| `init.rs` | `new_game`, `act_newgame`, `home_planet` |
| `save.rs` / `load.rs` | `State::save`/`load` over `DosFiles` |
| `keymap.rs` | `KEYMAP_BYTES` — the `ds:0x7CBD` pair table |

## Worked example — reading a planet's population

Planet 3's `F_POP` (`+0x2A`), step by step as the asm does it:

1. `base = st.word(0x9158)` — the record array anchor (`REC_BASE`).
2. `rec = base + 3*0x3A` — `cs:0x5E3A` accessor; `st.word(rec)` lands on
   the record's first word.
3. `pop = st.word(rec + 0x2A)` — the `F_POP` field.

In the port that's `st.word(rec_ofs(&st, 3) + rec::F_POP)`; in the asm
it's `mov di,[0x9158]; ...; mov ax,[di+0x2A]` — same arithmetic.

## How the cells get their names

The `0x9…` constants aren't arbitrary — they're the byte offsets the
asm's `mov`/`cmp`/`test` instructions carry, recovered by reading the
disassembly and tracking which cells each routine touches. When a new
cell surfaces in a routine you're porting, the convention is: add a
`const` to `consts.rs`/`cells.rs` named after what the asm does with it
(`DIRTY0` for the refresh-journal cell), cite the routine's `cs:` range
in its doc comment, and let `st.word(CELL)` mirror the `mov ax,[cell]`.
