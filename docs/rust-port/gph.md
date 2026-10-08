# `gph` — `.GPH` graphics index

Every asset set ships a `.GPH` index of fixed 12-byte records:

```c
struct gph_rec { uint16_t x, y, w, h, ofs_para, mode_flag; };
```

`Record` (`record.rs`, `RECORD_LEN = 12` = the `add si,0xC` stride at
`0x320E2`):

| field | asm ref | meaning |
|---|---|---|
| `x`, `y` | `[si+0]`, `[si+2]` | draw position (px / rows) |
| `w` | `[si+4]` | bytes per row for the mode — `ax` on decoder entry |
| `h` | `[si+6]` | row count |
| `ofs_para` | `[si+8]` | stream offset **in paragraphs** inside `.BIN` |
| `mode_flag` | `[si+0xA]` | `0` = opaque blit, `1` = XOR overlay |

- `parse_index(buf)` (`parse.rs`) — six LE `u16`s per record; trailing
  partial record ignored. The game reads a fixed `0x10B0` bytes (≤356
  records) into `ds:0x158`.
- `fix_up(records, heap_seg)` (`fixup.rs`) — the `0x320D8` loop
  (`add [si+0x160],ax`): adds the `.BIN` heap segment to every
  `ofs_para` in place, turning file-relative paragraph offsets into
  absolute segments for the renderer.
- `Record::is_xor` (`record_ofs.rs`) — `mode_flag == 1` (`cmp dl,1` at
  `0x2FBE7`).
