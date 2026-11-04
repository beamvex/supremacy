# Interpreters — does the game run code in a VM?

**Yes — in fact, several.** `GAME.EXE` is native 8086 code, but it embeds
five bytecode interpreters that drive large parts of its behaviour. The
simulation core (planet economy, battles, ships, rendering, input) is
plain machine code; everything *scripted* — timed events, text layout,
machine animation, music, menu pages — is decoded from byte/word
streams at runtime.

This page catalogues all five; [game-vm.md](game-vm.md) covers the
flagship event VM in depth.

## The five interpreters

| interpreter | asm | bytecode lives in | drives | port |
|---|---|---|---|---|
| Event VM | `cs:0x773D` entry, `cs:0x7758` fetch loop (file `0x3773D`–`0x37C61`) | `ds:0x51FB`+ scripts; `ds:0x50E3` jump table (40 far pointers); `ds:0x5183` event slots | timed events, ambushes, tributes, uprisings, endgame checks, modal dialogs, HUD reprints | `game/{vm,vops,vimpl,vdir,vev,vup,vplan,vmac,vord,vend,vhud,vhost,jtab}.rs` |
| Draw sequencer | `cs:0x5C1E` (file `0x35C1E`–`0x35C96`) | `cs:` stream of 4-byte `{op, pad}` records | per-machine-type animation: image index, delay, link, end | `game/seq.rs` |
| Print interpreter | `cs:0x6709` (file `0x36709`, `print_str`) | `ds:` strings, `0xFF`-terminated, with embedded control bytes `0x01`–`0x04` | ticker/status text: glyph draws, cursor moves, run-length repeats | `game/text.rs` |
| Music stream decoder | PC-speaker module file `0xC00`; per-channel step `cs:0xF3D` (Tandy `0x1C02`, AdLib `ar_*`) | `cs:` byte streams per channel; bytes `≥0x80` index an opcode jump table (`cs:0x32`), `<0x80` index the note table | tracker music + SFX on speaker, Tandy SN76489, AdLib OPL2 | `audio/{pc_step,pc_ops,tandy_step,tandy_ops,ar_*}.rs` |
| Menu text | `ss:` stream at `cs:0xF672`, glyph loop `0x2F640` | byte stream: `0x20` tile clear, `0xF0` newline, `0x00` end, `0xFF` save+suspend, else letter → tile blit | menu pages, the victory credits scroll | `game/vend/seq/scroll.rs`, `game/title/credits.rs` |

## The event VM in one paragraph

Tick codes `0x38+2N .. 0x56+2N` index the `ds:0x5183` event table; word 0
of each 4-byte entry is a script cursor `si`. The fetch loop at
`cs:0x7758` reads a word at `si`, adds `0x50E3`, and `jmp word [bx]`s
through the op table — so each opcode *is* a handler address slot.
Handlers either `jmp 0x7758` (continue) or `ret` (suspend until next
tick). `si` is never written back: a suspended event re-runs from its
first opcode each tick, making scripts guard+action *rules* rather than
resumable coroutines. Persistent state lives in `ds` cells (flags,
delay counters, the `0x81EE`+ scratch registers — all inside the
savegame block). Operands are bare `u16` immediates or 4-byte
`{ptr, tag}` slots. Opcodes cover: guard waits (`byte`/`word == imm`,
flag set/clear, record-kind tests), state mutation (`st_imm`, `add_imm`,
`cp`, `ld_ind`), presentation (`print`, `menu`, `dialog`), world events
(`levy`, `disaster`, `raze`, `depopulate`, `kill_sel`), and a `native`
escape that jumps to a `cs:` routine. Full op table and semantics:
[game-vm.md](game-vm.md); asm analysis:
`decompiled/FUNCTIONS.md` § "Event VM — cs:0x773D".

## What is *not* interpreted

The tick dispatcher `cs:0x73D9` shows the split directly (see
[game-tick.md](game-tick.md) and `decompiled/FUNCTIONS.md` § "Tick
dispatcher"): most codes call ordinary machine-code routines —

- `0x01..0x20` machine production/mining ticks,
- `0x20..0x38` ship ageing ticks (`cs:0x60EE`),
- `0x38+i` planet simulation, `0x38+N+i` planet defence pass,
- `0xFF` day rollover,

plus the frame loop `cs:0x395B`, the UI shell `cs:0x2CC5`, menu/hotspot
hit-testing `cs:0xA10F`, video/palette/LZSS loaders, and file I/O — all
native x86. Only codes `0x38+2N .. 0x56+2N` route into the event VM,
and the sequencer/print/music interpreters run as *services* those
native paths call per frame.

A few event-adjacent tick arms (`0xA9` tribute, `0xFB` pirate raid,
`0xFE` colony spawn, `0xFA` endgame monitor, the `0xAB..0xF8`
message/sprite passes) are compiled native routines that borrow the
VM's `ret`-to-suspend contract — they are reached by tick code, not by
script opcode, so they count as native code sharing the VM's control
convention.

## Evidence

- `decompiled/disasm/GAME.linear.asm` file `0x3773D`: the entry —
  `sub bx,0x38 / sub bx,[0x91b6] ×2 / shl bx,2 / add bx,0x5183` picks
  the event slot; `0x37758`: `mov bx,[si] / add si,2 / add bx,0x50e3 /
  jmp word [bx]` is the fetch-dispatch loop; handlers at
  `0x37768`+ each `jmp 0x7758` or `ret`.
- `decompiled/FUNCTIONS.md`: § "Event VM — cs:0x773D", § "Draw
  sequencer — cs:0x5C1E", `print_str` at file `0x36709`, § "PC speaker
  driver" (opcode jump table `cs:0x32`), the `menu_text` row in the
  routine tables.
- Rust port: `vops::run` (`rust/src/game/vops.rs`) is the
  `cs:0x773D`+`cs:0x7758` pair; `op`/`op2`/`op3` are the jump-table
  dispatch; `tick.rs` routes `Tick::Script` through it. `seq.rs`,
  `text.rs`, `audio/pc_step.rs`, `vend/seq/scroll.rs` port the other
  four interpreters.
- Tests: `rust/tests/{game_vm,game_seq,game_tick}.rs`,
  `rust/tests/audio_{pc,tandy,ar}.rs`.

## Bottom line

The game does not run *all* its code in a VM — the bulk is compiled
8086. But the original authors did ship a real opcode-dispatch
interpreter for scripted events, plus smaller bytecode engines for
text, animation, music and menu pages. In the Rust port those same
interpreters run the shipped bytecode out of the original image; the
port is a reimplementation of the interpreters and the native routines
alike, not an emulator and not a script translator.
