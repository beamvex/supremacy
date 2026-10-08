# `game` (1/4) — state block, record layouts, init, save/load

## `State` (`state.rs`, `consts.rs`)

The saveable game-state block: exactly the `0x1F8B` bytes at
`ds:0x7D39..0x9CC4` that the save routine writes whole (`int 21`
create/write at file `0x383F3`) and load reads back (`0x383CE`), and
that the heap-staging copies (`0x844B`/`0x8432`) move to/from the loader
heap segment.

- `block: Box<[u8; STATE_LEN]>` — `STATE_OFS = 0x7D39`,
  `STATE_LEN = 0x1F8B`. Accessors take **real `ds:` offsets** and
  subtract `STATE_OFS` internally: `word`/`set_word`, `dword`/
  `set_dword`, `byte`/`set_byte` (all little-endian).
- `from_image(img)` — the shipped initial state at file
  `STATE_OFS + 0x10CA0`.
- `rec_ofs(i)` — `cs:0x5E3A` accessor: `[0x9158] + i·0x3A`.
- `planets()` — `[0x91B8]` = planet count = index of the appended
  player-faction record.

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

### `rec.rs` — the `0x3A`-byte planet/faction record

Recovered from new-game init (`0x330A0`–`0x331C2`), accessor `cs:0x5E3A`,
planet sim `cs:0x3D1E`, defence `cs:0x4434`, stat refresh
`cs:0x3C6D`/`0x3533`:

| off | field | notes |
|---|---|---|
| `0x00` | `F_WORD0` | zeroed at init |
| `0x0C` | `F_KIND` | `0xFFFF` empty; `0xA` inhabited; `1`/`4`/`7` other classes |
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

- `range(b)` → `hi16((b+1)·r)` (the asm does `inc bx` then `mul`,
  keeping `dx`) — result ∈ `0..=b`.
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
`total−1` → `REC_COUNT` and byte `0x91C6`, `difficulty` → `0x91E4`.

## `init.rs` — `new_game` (`cs:0x9DAB`, file `0x3305B`–`0x331C2`)

Opens with `stage_load` (the `0x8432` staging-heap restore — resets the
save block to the `0x844B` snapshot taken after the menu picks).
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

`act_newgame` (`cs:0x305B`/`0x9DAB` tail, `0x331C4`–`0x331E6`):
`new_game`, then int-33 restrict (`ax=0x14`, mask `0x1E`), `mark_sel`,
`sel_next`, a `0x32`-frame `{vsync, Sound(0)}` wait, `Native(0x2FD7)`,
`Slot(0x1278)` image reload, input reinstall (`ax=0x14`, mask `0x1F`).

## `save.rs` / `load.rs` — savegame I/O

`State::save(fs, name)` — `0x383F3`: `create` (truncate), close the
leaked handle, `open_write`, `write` the whole `0x1F8B` block, `close`;
`false` mirrors the carry-flag error return (short write counts).
`State::load(fs, name)` — `0x383CE`: `open`, `read` the block, `close`;
`false` on error. Both generic over `DosFiles`.

## `keymap.rs` — the `ds:0x7CBD` keymap

`KEYMAP_BYTES`: 124 shipped bytes = 62 `{code, out}` pairs used by the
`0xE66E` line editor and the `0x32BA8` yes/no poll; shipped at
`ds:0x76BF` (file `0x1835F`), copied to `0x7CBD` at init (the runtime
slot overlays warning-string bytes refreshed by `[0x1278]` reloads).
`out` classes: `0` exit, `1` backspace, `2` pad-with-spaces+exit,
otherwise the char to append while `[0x4FCB] < [0x4FCC]`. Table order:
erase keys (`0x0E`/`0x70`/`0x53`), pad keys (`0x1C`/`0x74`), digits,
keypad, `-`/`=`, QWERTY rows, `0x2C..0x35` row, space, `0x5B` exit.
First match wins (e.g. `0x47` is shadowed by keypad-7).
