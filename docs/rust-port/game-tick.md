# `game` (3/4) — the tick dispatcher and simulation

## `tick.rs` — `tick_step` (`cs:0x73D9`, file `0x373D9`–`0x376C5`)

One game tick, called once per frame. `[0x91B0]` nonzero → cleared
(sound-service flag, file `0x373D9`–`0x373FD`). Then `[0x91CE]`
(`SEQ`) increments and selects work by `bl`:

| code `bl` | arm | routine |
|---|---|---|
| `< 0x20` | `Machine` | `machine_tick` (record `bl`) — handled before `dispatch` runs |
| `0x20..0x38` | `Ship` | `ship_tick` (`bx = bl − 0x20`, `cs:0x60EE`) |
| `0x38..0x38+N` | `Planet` | `sim_planet` (`b−0x38`) |
| `+N..+2N` | `Defence` | `defence_tick` (`b−0x38−n`) |
| `+2N..0x56+2N` | `Script` | `vops::run` event VM (op count payload) |
| `0xA7` | `HudSel` | selected-object reprint `cs:0x60A8` |
| `0xAA` | `Sprites` | mark-all-sprites `cs:0x3738` |
| `0xA8` | `Jingle` | `vdir::jingle` `cs:0x7E36` |
| `0xA9` | `Tribute` | `vev::tribute` `cs:0x7C62` |
| `0xAB..0xC0` | `MsgOp` | `vdir::msg_op` `cs:0x76DE` |
| `0xC1..0xF7` | `Redraw` | `vdir::redraw` `cs:0x80F0` |
| `0xF8` | `ClearMsg` | `vdir::clear_msg` `cs:0x813D` |
| `0xF9` | `Orders` | `vord::orders` `cs:0x4309` |
| `0xFA` | `Endgame` | `vend::endgame` `cs:0x82E5` |
| `0xFB` | `Pirate` | `vev::pirate` `cs:0x7C9F` |
| `0xFC`/`0xFD` | `HudPop`/`HudCredits` | `cs:0x60D4`/`cs:0x60B6` |
| `0xFE` | `Uprising` | `vup::uprising` `cs:0x7EA2` |
| `0xFF` | `Day` | `day_tick` `cs:0x6A7B` |
| other | `Idle(bl)` | the `cs:0x76C5` `ret` |

`Tick` enum carries each arm's payload (`MachOut`, starvation flag,
`Battle`, `ShipOut`, op count, `Option<Ending>`).

## `machine.rs` — `machine_tick` (`cs:0x742C` region, `0x37423`–`0x375C1`)

Per `M_TYPE==0` skip → `Quiet`. Otherwise, in asm order:

- `countdown` — `M_TIMER−1`, dirty `0x800` (`0x3742C`–`0x3743D`).
- `produce` (skipped while flag `0x4` = mining mode): online (`0x20`)
  type 6 `miner` → host planet minerals `+2` (`+7`/`+5` for
  `G_MINETECH`/owner-5), fuel `+7` (`+0x19`/`+0xF`), dirty
  `0x8000`+`0x3`; online type 7 `farm` → burns 1 energy (empty stock
  clears flag `0x20` instead), food `+0xC` (`+0x19` `G_FARMTECH` /
  `+0x1C` owner-4), dirty `0x4000`. Each add is gated on the current
  stock being under `0x7530` — not a hard cap.
- `solar` — type 3 unconditionally: energy `+6` (`+7` more for owner
  3) while under `0x7530`, dirty `0x2` (`DIRTY1`).
- `mine` (`mine.rs`, `0x3752B`–`0x375A4`) — while flag `0x8` and `M_OPS`
  remain: drains `M_DEPOSIT` by `0x19` (`G_DEEPMINE` set) / `0x32`
  unless the type record's `+0xC` is `0xFFFF` (type table at
  `0x9B12 + (type−1)·0x30`); `M_OPS−1` — mid-flight: `MSG_MINE` = ops
  when flag `0x1`, dirty `0x2000` if selected; on `0`: clear flag `0x8`,
  `M_HOST` = byte at `[M_LINK]+0x28`, flag `0x1` promotes the whole
  byte to `0x11`, dirty `0x3000` if selected.
- `build` — flag `0x1` countdown `M_BUILD` (`MSG_BUILD` = remaining);
  at zero clears `0x1` and `colonise`s the `QUEUED_REC` planet: kind
  `0xA`, owner from `next16 & 7` rejected until `{1,3,4,5}`,
  `pop = rand(0x3E8)`, food `rand(0x4B0)+0x12C`, minerals `0x14`, fuel
  `0x96`, energy `0x23`, credits 0, dirty `0x8` → `MachOut::Colonised`
  (asm also prints `ds:0x831A` and jumps `0x5C97`).

## `sim.rs` — `sim_planet` (`cs:0x3D1E`, `0x33D1E`–`0x33F17`)

Only `F_KIND == 0xA` records simulate. Order:

1. `consume` — food `−= min(food, pop/0xF0)`; shortfall (or empty stock)
   forces `F_COVER = 0`; dirty `0x40`/`0x4000`. Returns the starvation
   warning — true when `food < need` and `TICK.trailing_zeros() ≥ 4`
   (every 16th tick; the asm splices the name into `0x8305` and prints
   `0x82C0`).
2. `coverage` — `F_COVER` tracks `100 − tax` by `±1`; dirty `0xC0`.
3. Odd `TICK` → `income`: `credits += pop·tax / (owner∈{1,2} ? 125 :
   200)`; dirty `0x4`. Even → `scores` (`F_GROWTH = cover/3`, halved by
   `G_FAMINE`; `F_DECLINE = (100−cover)/4 + tax/4`; dirty `0x40`) and
   `population`.
4. `population` — `net = growth − decline`; growth `net·pop/0x190 + 1`
   (skipped when `pop < 2`), decline same magnitude clamped to `pop`,
   cap `0x7530`, dirty `0x20`; then `delayed_kill` — when `G_KILL_ON`
   and `G_KILL_REC == r`, `pop = 0` and `G_KILL_AUX` clears.

## `defence.rs` + `battle.rs` — `defence_tick` (`cs:0x4434`, `0x34434`–`0x34973`)

`SCAN_REC` = the record; ownerless → `Quiet`. `fleet_scan`: for each of
`0x17` ships with `S_LINK == rec` and flag `0x2`: `F_ATTACK += pow`;
`F_DEFENCE += pow + (((pow·crew/0x48) & 0xFFFF)·(guns+load+F_29+1) &
0xFFFF)` — both multiply stages truncate to 16 bits like `mul`/`div`.
Selected record → dirty `0x8`/`0x10`.

Then: `F_TROOPS == 0` → `Quiet` when defence is also zero, else
`reinforce` (`F_TROOPS=0`; kind `0xA` early-outs → `Reinforced`; else
`REINF_REC` = rec and crews of armed linked ships gain `+7` while under
`0x5D`, dirty `0x8`/`0x18`). Nonzero troops → `regen` (`+4`/`+3`/`+1`
by owner 4/5/other, cap `0x3095`, record 0 skips) then `resolve`:
`def == 0` → `overrun`; `eff = troops·(2+F_29)/100` (min 1); `def >
eff` → `repelled` (`pct = 100·eff/def`; each linked armed ship takes
`pow·pct/100` damage — a `0` roll unlinks the ship; `F_ATTACK` = the
damage sum; troops bleed `def·2/100`, underflow → `reinforce`); else
`destroy_ships` + `overrun` (clear defence/attack; kind-7 records and
gate failures — `0x34649`–`0x34677`: modes 0/5/7 pass, 4 never, others
only for the selected record — return early; `MSG_REC` = rec,
`kill_machines` destroys machines bound to the planet's serial and
their docked ships plus matching `F_TIMER_*` slots, `kind = 7`, dirty
`0x8`/`0x18` → `Overrun`). `destroy_ships` also has the mode-1 abort
when `LINK_REC` points at the first fleet slot.

## `fleet.rs` — `ship_tick` (`cs:0x60EE`, `0x360EE`–`0x361E4`)

`bx = code − 0x20` indexes the fleet array. `S_FLAGS & 2` **set** →
`Skip`. `age`: crew `+1`, `+1` more when diff ≠ 2, `+1` more on diff 0
(`+3`/`+2`/`+1`), clamp `0x64`. UI-mode-1 selected ship
(`[LINK_REC] == si`) → `panel`: stash `si` at `0x9150`, zero crew when
`word[si] == 0`, `SHIP_RATE 0x91D6 = 9 − crew/16` (`0xFF` uncrewed),
`SHIP_ACTIVE 0x91DE` (`0xFF`/0), print crew at `(0x46,0xC0)` and the
`crew/10`-indexed `0x13`-byte status line from `ds:0x7438` at
`(0x3B,0x3F)` → `Panel`, else `Aged`.

## `day.rs` — `day_tick` (`cs:0x6A7B`)

`TICK` runs `1..=0x40`, wraps to `1` and increments `DAY`. (The asm's
readout redraw is the shell's job.)

## `seq.rs` — `seq_step` (`cs:0x5C1E`, `0x35C1E`–`0x35C96`)

Timed draw sequencer, once per frame. `[0x91A0]` delay counts down
first; `[0x91D6]` period `0xFF` disables; `[0x91D7]` phase wraps at the
period and only fires a command on wrap-to-0. `[0x9198]` is a `cs:`
cursor into 4-byte `{op, pad}` records:

- `0xFFFF` — end: clear `0x9198`/`0x919A`.
- `0x01F4` — delay: next record's op word → `[0x91A0]`; skip both.
- `0` — link: `si = cs:[si]` (next record's op word is a `cs:` ptr);
  resume without suspending.
- else — image index: commit `0x9198`, `Call::Image(img)` (the
  `[0x125A]` draw slot).

One command (or delay reload) per period boundary; `cs_word` fetches
come from the host.

## `ticker.rs` — `ticker_step` (`cs:0x6B8F`, `0x36B8F`–`0x36C26`)

Status-line ticker, once per shell frame while `BLINK_T` is idle.
`[0x91C2]` countdown = records left (`0` disables). `[0x91D2]` per-step
delay. `[0x915C]` points at 4-byte `{msg-cursor, unused}` records
(rooted `0x928D`, `0xFE` sentinel wraps). The msg cursor walks the
message one byte per step:

- `0xFC` — next byte → `[0x91D2]`.
- `0xFD` — arm `BLINK_T = 1`, `TICK_X = 0x29` (next line's x).
- `0xFB` — end-of-record step, then print the word at `di` via
  `num::num` at `([0x91C0],0xB8)`.
- `0xFF` — end of record: clear `0x91E8`, wrap cursor on `0xFE`,
  `[0x91C2]−1`, commit.
- else — `Call::Glyph(ch, TICK_X++, 0xB8)`.
