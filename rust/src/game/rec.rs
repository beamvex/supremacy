//! Field offsets inside one `0x3A`-byte planet/faction record, recovered
//! from the new-game init (file `0x330A0`–`0x331C2`), the record accessor
//! `cs:0x5E3A` (`idx·0x3A + [0x9158]`), the planet simulation tick
//! (`cs:0x3D1E`), the defence pass (`cs:0x4434`) and the stat-refresh
//! dispatcher (`cs:0x3C6D`/`0x3533`).

/// Word zeroed for every record at init (file `0x331BA`).
pub const F_WORD0: usize = 0x00;
/// Kind/status word — `0xFFFF` = empty, `0xA` = inhabited (only kind the
/// population sim touches, file `0x33D3E`), `1`/`4`/`7` = other classes
/// dispatched to separate handlers (file `0x33F61`–`0x33F8E`).
pub const F_KIND: usize = 0x0C;
/// Station/ambient type byte — drives the main loop's periodic SFX
/// selector (`3`/`4`/`6`/`7`, file `0x339A9`–`0x339EF`).
pub const F_STYPE: usize = 0x12;
/// Rate byte multiplied by `F_KIND` for display (file `0x33C3B`).
pub const F_RATE: usize = 0x13;
/// 8-byte display name + `0x21` terminator at `+0x16` — init writes
/// `LIFELESS!` (file `0x3315C`–`0x33170`).
pub const F_NAME: usize = 0x0E;
/// Pending-event timer A — nonzero contributes to the alert test
/// (file `0x33F76`).
pub const F_TIMER_A: usize = 0x18;
/// Pending-event timer B (file `0x33F7C`).
pub const F_TIMER_B: usize = 0x1C;
/// Growth score — `coverage/3` (halved while `[0x822B]` set), recomputed
/// each planet tick (file `0x33E26`).
pub const F_GROWTH: usize = 0x1D;
/// Tax slider `0..=100` — raises income (`pop·tax/200`, `/125` for owners
/// `1`/`2`) and lowers the food-coverage target `100 − tax`
/// (file `0x33DB0`/`0x33DD5`).
pub const F_TAX: usize = 0x1E;
/// Food-coverage percent — tracks `100 − tax` by `±1` per tick, forced
/// to `0` while food is exhausted (file `0x33DB5`).
pub const F_COVER: usize = 0x1F;
/// Pending-event timer C (file `0x33F7C`).
pub const F_TIMER_C: usize = 0x20;
/// Decline score — `(100 − coverage)/4 + tax/4`, recomputed each tick
/// (file `0x33E6C`).
pub const F_DECLINE: usize = 0x21;
/// Defence strength — recomputed by the fleet scan `Σ pow + weighted`
/// (file `0x34529`); a word cleared at init for the player record.
pub const F_DEFENCE: usize = 0x22;
/// Owner byte — `2` = the player (file `0x330FB`/`0x3314B`); colonisation
/// rolls `{1,3,4,5}` (file `0x375CD`–`0x375DF`).
pub const F_OWNER: usize = 0x24;
/// Ground-troop garrison — regenerated `+1`/`+3`/`+4` per defence pass
/// by owner, capped `0x3095` (file `0x3456E`–`0x34578`); fights the
/// incoming fleet as `troops·(2+F_29)/100` (file `0x34591`). Difficulty
/// scales the home planet's start (file `0x330A4`).
pub const F_TROOPS: usize = 0x26;
/// Same offset on the faction record holds liquid credits — the
/// tribute (`cs:0x7C62`), pirate raid (`cs:0x7C9F`) and colony-spawn
/// (`cs:0x7EA2`) routines all read/write `rec0+0x26` as money, and the
/// `cs:0x8149` report copies it to the `0x83F4` stat slot.
pub const F_CASH: usize = F_TROOPS;
/// Planet serial index stamped at init (file `0x33185`).
pub const F_SERIAL: usize = 0x28;
/// Byte cleared with the serial (file `0x33188`); also fed into the
/// defence weighting `+1` (file `0x344F3`).
pub const F_29: usize = 0x29;
/// Population — consumed `pop/240` food per tick, grows/declines by
/// `net·pop/400 + 1`, capped `0x7530` (file `0x33D5C`–`0x33EF4`).
pub const F_POP: usize = 0x2A;
/// Food stock — drained by population, refilled by horticultural
/// stations (file `0x33DA7`/`0x374E4`).
pub const F_FOOD: usize = 0x2C;
/// Inbound/hostile strength sum — the raw `Σ ship pow` half of the
/// fleet scan (file `0x3452C`); nonzero contributes to the alert test.
pub const F_ATTACK: usize = 0x2E;
/// Minerals stock — produced by core-mining stations (file `0x37472`).
pub const F_MINERALS: usize = 0x30;
/// Fuel stock — produced by core-mining stations (file `0x37492`).
pub const F_FUEL: usize = 0x32;
/// Energy stock — produced by solar satellites, consumed by
/// horticultural stations (file `0x37517`/`0x374DA`).
pub const F_ENERGY: usize = 0x34;
/// Credits — 32-bit `dword` at `+0x36`/`+0x38` (file `0x3310A`–`0x33117`:
/// `0xC350 + rand(0x4E20)` ≈ 50,000–70,000 for a new faction; tax income
/// accrues at file `0x33DF1`).
pub const F_CREDITS: usize = 0x36;
