# `game` (2/4) — the scripted-event VM (`cs:0x773D`)

## `Vm` (`vm.rs`) — memory model

The interpreter runs over the whole `ds` space, split three ways:

- `low: Box<[u8]>` — `ds:0x0000..0x7D39`: event scripts, the `ds:0x50E3`
  op table, the `ds:0x5183` event slots, message strings, VM flag cells.
- `State` — `ds:0x7D39..0x9CC4` (the saveable block).
- `hi: Box<[u8]>` — `ds:0x9CC4..=0xFFFF`: menu records and scratch.

Plus `modal` (the `cs:0xE713` modal-dialog flag — a code-segment cell in
the asm) and `stage` (the heap segment the `0x844B`/`0x8432` routines
snapshot the save block into/out of, `ds:0x3829` in the asm).

Key addresses: `JT_BASE = 0x50E3` (op jump table — runtime-initialised,
the shipped image holds stale bytes), `TAB_BASE = 0x5183` (4-byte
`{cur, base}` event entries), `HI_OFS = 0x9CC4`, `LOW_LEN = 0x7D39`,
`KEYMAP = 0x7CBD` (`KEYMAP_N = 0x3E` pairs), `KEYMAP_SRC = 0x76BF`.

- `from_image(img)` — loads `ds:0..0x9CC4` from file `0x10CA0+`, seeds
  `stage` from the shipped save block, and copies the image's keymap
  `0x76BF → 0x7CBD` when the bytes are present.
- `r8`/`r16`/`w8`/`w16` — byte/word access across the three regions
  (routed by offset; unaligned-safe LE).
- `stage_save`/`stage_load` — the `0x844B`/`0x8432` block copies.
- `rec_ofs(st, i)` — `[0x9158] + i·0x3A` via `r16`.
- Operand fetchers: `slot(si)` reads a 4-byte `{val, tag}` (tag `0x0FAA`
  shipped, consumed but unused) advancing `si += 4`; `imm(si)` reads a
  bare `u16`, `si += 2`.

## Dispatch (`vops.rs`)

Tick codes `0x38+2N .. 0x56+2N` index the `ds:0x5183` event table; word
0 is the script `si`. Each opcode word indexes `JT_BASE` for a `cs:`
handler address. `Flow`: `Cont` = `jmp 0x7758` (fetch next), `Ret` =
`ret` (suspend). **`si` is never written back** — a suspended event
re-runs from its entry next tick, so scripts are guard+action rules
whose state lives in `ds` cells, not the instruction pointer.

`run(c, idx)` = the `cs:0x773D` entry + `cs:0x7758` fetch loop: `si` from
the event table, loop `{w = r16(si); t = r16(JT_BASE+w); si += 2}` until
a handler returns `Ret`; returns the op count.

`Ctx<'a>` — the five borrow contexts every op takes: `vm`, `st`, `rng`,
`host: &mut dyn VmHost`, `files: &mut dyn DosFiles`. `vsync(c)` = one
`0x2CA9` frame boundary (`Call::Ui(0x2CA9)` + `host.pump_input`).

Scratch registers (inside the save block): `R_SEL 0x81EE` (selected
record), `R_MACH 0x81F2` (found machine), `R_PLANET 0x81F6`, `R_A
0x81F7`, `R_B 0x81F8`, `R_C 0x81FA` (dword).

Helpers: `planet_rec` (`[0x81F6]`·`0x3A` + base), `faction_rec`
(`[0x91B8]`·`0x3A` + base), `uimode`, `kind_of(rec)`, `set_bits` (`or
word`), `unlink_ships(rec)` — zero the 8-byte head of every fleet record
whose `+2` link points at `rec`; `false` on the `[0x914C]` early-out in
UI mode 1.

### Op table (`op`/`op2`/`op3`)

| `cs:` target | handler | effect |
|---|---|---|
| `0x7768`, `0x7AF0` | — | suspend (`ret`) |
| `0x7769`/`0x78C7` | `wait_b` | suspend while `byte[p]` is set / clear |
| `0x7774` | `print` | enqueue ticker string at slot |
| `0x7782`/`0x778C` | `set_flag` | `byte[p] = 0xFF`/`0` |
| `0x7796` | `wait_eq_b` | suspend until `byte[p] == imm8` |
| `0x77A5`/`0x77B4` | `wait_eq_w` | suspend until `word[p] == imm16` (wide slot) |
| `0x77C3`/`0x77E1` | `print_at` | print `planet_rec+0xE` / `[R_SEL]+0xE` name |
| `0x77F0` | `rand_planet` | `R_SEL` = `rand & [0x91B8]` (retry `==`) |
| `0x7811`/`0x7825`/`0x7839` | `ld_ind` | load 1/2/4 bytes `[slot]+imm` → R_A/R_B/R_C |
| `0x7853`/`0x7866`/`0x7879` | `cp` | copy 1/2/4 bytes slot → slot |
| `0x7893`/`0x78A2`/`0x78B1` | `add_imm` | add imm to 1/2/4-byte cell (dword `adc`s) |
| `0x78D5`/`0x78E4`/`0x78F3` | `st_imm` | store imm byte/word/dword at slot |
| `0x7908` | `native` | `jmp word [si]` escape → `Call::Native`, suspends |
| `0x79AA` | `delay` | countdown cell: suspend decrementing while ≠0 |
| `0x79BB` | `menu` | `call 0x2FE7` menu service |
| `0x79C3` | `dialog` | modal dialog block (swaps hotspot list to `0xB032`, restores menu `0x31`) |
| `0x790A` | `kill_sel` | destroy selected planet (vplan) |
| `0x7980`/`0x798E`/`0x799C` | `wait_kind` | suspend unless sel kind is `0xA`/`7`/`4` |
| `0x7A11` | `levy` | pull ≤`0x186` food, dump excess >`0x384` to fuel |
| `0x7A45` | `disaster` | disaster fanfare + halve food/fuel (vdir) |
| `0x7ADD`/`0x7AF1` | `unhide`/`raze` | machine ops (vmac) |
| `0x7BAB`/`0x7C16`/`0x7C30` | `wait_derelict`/`reset_timers`/`wait_ripe` | machine scans (vmac) |
| `0x7BE8` | `print_at` | print `[R_MACH]` name |
| `0x7BF4`/`0x7C05` | `depopulate`/`charge` | sel `pop=0` / `energy=0x74CC` (vplan) |

Unmapped targets → `Flow::Ret`.

## Event routines (tick-direct, all `Ret`-suspending)

- `vdir::disaster` (`cs:0x7A45`) — modal print `0x5D7B`, menu, sound
  seq (`Chan(0x8A5B)`/`Chan(0x8A3B,8)`/`Sound(0x15)`, `[0x91D3]`-gated),
  `Flash`, `Present(0)`, planet panel, `0x5DAE` print; then halves the
  faction's food+fuel, `DIRTY1 | 1`, unlinks its ships → `Ret`.
- `vdir::jingle` (`cs:0x7E36`, tick `0xA8`) — picks a tune index via
  `Native(0x37C7)`; per-difficulty tables `0x4E96`/`0x4E4E` yield the new
  `[0x91A6]` and a note word for `[0x91FF]`; a changed note plays via
  `Chan(0x8A5B,0)`/`Chan(0x8A3B,0xFF)`/`Sound((t&0xFF)−2)` +
  `Native(0x8A80)` (`t==0` or muted → the `0x8A80` tail only); mode 0
  mirrors `[0x91A6]→[0x928B]` + refresh.
- `vdir::msg_op` (`cs:0x76DE`, `0xAB..0xC0`) — message pass: draws one
  pending sprite (retry `rand & 0x1F` < `0x16` with the `0x7562` flag
  set) at the `[0x4FAA]` row for the UI mode; saves/restores `[0x91EA]`
  via `[0x91E9]`; clears on `MSG_ON==0`/mode 0.
- `vdir::redraw` (`cs:0x80F0`, `0xC1..0xF7`) — same save/restore around
  a `cs:0x6709` print-interpreter call alternating sprite sets
  `0x7562`/`0x68BF` on `[0x91CE]` parity.
- `vdir::clear_msg` (`cs:0x813D`, `0xF8`) — clear `0x91E8`, restore
  `0x91EA` from `0x91E9`.
- `vev::tribute` (`cs:0x7C62`, `0xA9`) — while rec0 is conquered (kind
  7), unthreatened and below `0xA240` cash: add the difficulty-scaled
  dole `0xA`/`0x16`/`0x22`/`0x32` (`0x32` needs `[0x91E7]` set).
- `vev::pirate` (`cs:0x7C9F`, `0xFB`) — when rec0 cash ≥ the difficulty
  threshold (`0x2C38`/`0x568C`/`0x7D9D`) and the picked victim is
  inhabited with no troops: raider strength `(rand & mask) + base`
  lands on the victim (`F_TROOPS`), the cash is stolen, victim's name
  copies to `0x8410`, print `0x83F6` + raid sting. Victim pick
  (`cs:0x7D5A`): diff 0 round-robin via `[0x91AA]`; diff 1
  `rand & count`; diff 2 `tick % 8` — `0`/`1` scan machines type 7/6
  for a powered one's host, `2` weak (`pop < 0x1388`), `3` random,
  `4..7` fail (`[0x91CF]` latch).
- `vup::uprising` (`cs:0x7EA2`, `0xFE`) — arms on the first kind-4
  (destroyed) record once `day ≥ 0x7DB`/`0x7DD`; counts down `[0x91C7]`
  (`[rec+0x1A]/0xFA + 0xB` ticks), then spawns a kind-7 colony: owner
  rerolled (`rand & 7` until `1..=5`), `rand(0x4B0)` cash stolen from
  rec0, rolled stocks, name via `name_gen`.
- `vup::name_gen` (`cs:0x7FA5`) — 9 spaces, `rand&3 + 1` letters
  alternating tables `0x794B` (mask `0xF`) / `0x795B` (mask `7`), then a
  3-char digraph from `0x7963` (4-byte records) whose first byte differs
  from the name's last.
- `vplan::kill_sel` (`0x790A`) — unlink ships, `kind=4`, `owner=0`,
  dirty `0x8`, panel refresh in mode 0; suspends on the UI-mode aborts.
- `vplan::levy` (`0x7A11`) — modal + print `0x5D33`; food `−min(0x186)`,
  excess above `0x384` → fuel.
- `vplan::depopulate` (`0x7BF4`) — sel `pop = 0`, dirty `0x20`.
  `vplan::charge` (`0x7C05`) — sel `energy = 0x74CC`, dirty `0x2`.
- `vmac::unhide` (`0x7ADD`) — clear flag `0x20` on all `0x20` machines.
- `vmac::raze` (`0x7AF1`) — raze every machine hosted on the selected
  planet (`M_HOST` byte == `sel+0x28` serial); skips non-hosted machines
  and flag-`0x4`-without-`0x10` miners, suspends (`Ret`) when `sel ==
  SEL_REC`, when `uimode ∉ {0,5}` and the machine is `SEL_MACH`, or on
  the mode-1 unlink abort; else unlink ships, clear type, clear
  matching post-fleet tail timer bytes (`tail+0x18/0x1C/0x20`).
- `vmac::wait_derelict` (`0x7BAB`) — suspend until a type-7/`0x30`-flags
  machine exists off the current planet; stores it at `R_MACH`, clears
  type.
- `vmac::reset_timers` (`0x7C16`) — zero `M_TIMER` on all machines,
  dirty `0x800`.
- `vmac::wait_ripe` (`0x7C30`) — suspend until a type-5/`0x4`-flag
  machine's `M_ACC ≥ 0x4B0`; harvest → `R_MACH`, clear `M_ACC`, dirty
  `0x4000`.
- `vord::orders` (`0xF9`, `cs:0x4309`) — orders popup, gated on
  `[0x91F4]` set, `uimode 7` and `[0x9198]==0`: frame rect, headers
  (`0x4F50`/`0x4F6E`/`0x4F8D`/`0x4FA6`), pick a `0x1E`-byte entry from
  `ds:0x4EBA` by the selected record's `+0x2E`/`+0x26`/kind (cycling
  selector `0x91AC` wraps `5→2`), load entry into the message cells
  (`0x9198`/`0x9255..0x926C`/`0x91D6`), refresh, `Native(0x5C49)`.
- `vend::endgame` (`0xFA`, `cs:0x82E5`) — faction kind 7 or no kind-7
  planet → `Ending::A` (flag `0x91F1`, code `2`, `Screen(0x8B17)`);
  rec0 kind `0xA` or no kind-`0xA` planet → `Ending::B` (`0x91F0`, `3`,
  `Screen(0x8C65)`); else `None`.
- `vhud` — `0xA7` sel-ship reprint `(0x2C,0xB)`, `0xFD` faction credits
  dword `(0x45,0x32)`, `0xFC` faction pop `(0x49,0xB)` (all mode-1
  only); `0xAA` set `|0x80` on every `0x7562` sprite slot up to the
  `0xFF` terminator.

## `vhost.rs` — host services

`Call` enum records one host-side effect in issue order: `Menu`,
`Dialog`, `Refresh`, `Draw(u16,u16)`, `Sound(u8)`, `Chan(u16,u8)`,
`Flash`, `Present(u16)`, `PlanetPanel`, `Blit(u16,u16)`,
`Glyph(u8,u16,u16)`, `Native(u16)`, `Ui(u16)`, `Rect(u16,u16,u16,u16)`,
`Screen(u16)`, `Slot(u16,u16)`, `Image(u16)`, `Sfx(u16,u8)` (the
`0x8577` wrapper — `ax` command, `cl+1` driver function, `[0x91D3]`
gate applied before emitting), `Mouse(u16,u16,u16)` (int 33h),
`KeyPop` (channel-A LIFO pop → `ax`).

`trait VmHost { svc(call) -> u16; cs_word(ofs) -> u16; pump_input(vm, st);
key_flush(); }` — `cs_word` serves the draw sequencer's `cs:` reads;
`pump_input` re-mirrors input at every `0x2CA9` boundary so modal loops
see input; `key_flush` resets channel A. `NullHost` drops everything.
