; probe.asm — the DOS side of the DOSBox-X differential harness (GAM-25).
;
; A TSR loaded before GAME.EXE that drives the original under scripted
; input and dumps guest state for comparison against the Rust port.
;
;   nasm -o probe.com probe.asm
;
; It reads SCRIPT.BIN from the current directory at install time — a
; list of fixed 12-byte records:
;
;   +0  u16 tick    — probe tick count at/after which the op applies
;                     (one tick = one int-8 chain call ≈ 55-60 ms)
;   +2  u8  op      — 1 KEY, 2 MOUSE, 3 DUMP, 4 POKEW, 5 DONE, 6 PAL,
;                     7 STAT, 8 WAITCELL
;   +3  u8  a
;   +4  u16 b
;   +6  u16 c
;   +8  u16 d
;  +10  u16 pad
;
; Ops (applied in order; a KEY whose latch is busy or a DUMP taken while
; DOS is busy simply waits for the next tick):
;   KEY    a = scancode (bit 7 = break) — mirrors `Pump::key`: latches
;          [0x9CCB]=1/[0x9CCC]=a and pushes the code onto the game's
;          int-9 LIFO at drv:0x8B72 (count drv:0x8B7C, drop at 9).
;   MOUSE  a = event mask, b = x px, c = y px, d = buttons — far-calls
;          the handler the game registered through int 33h ax=0x0C/0x14
;          exactly like the real mouse driver (cx is passed doubled —
;          the handler shifts it back). No-op until a handler is seen
;          (always the case in K mode, which never calls int 33h).
;   DUMP   a = file id, b = ofs, c = len, d = seg — writes len raw bytes
;          to C:\DIFFTEST\OUT\Dxx.BIN. Seg sentinel 0xFFFF = game ds,
;          0xFFFE = driver segment (int-8/9 ISR seg — also where the
;          cs:0x55A2 RNG state lives), anything else absolute.
;   POKEW  b = ofs, c = value, d = seg — one word write.
;   DONE   writes C:\DIFFTEST\OUT\DONE.BIN — the host's end marker.
;   PAL    a = file id — dumps the 768-byte VGA DAC to Pxx.BIN.
;   STAT   a = file id — writes the 24-byte debug block to Sxx.BIN.
;   WAITCELL b = ds ofs, c = byte — blocks until ds:[b] == c. Gives the
;          script exact game-state sync points (uimode, counters).
;
; Segment discovery is dynamic so the probe survives different load
; addresses: int 21h/AH=1A with DX=0x28 is the game's entry-stub DTA
; call → game ds; int 21h/AH=25 AL=8/9 installs the game timer/keyboard
; ISRs → the driver segment (LIFO + RNG cells live there).

org 0x100

RECSZ   equ 12
KPEND   equ 0x9CCB          ; ds: key latch flag
KCODE   equ 0x9CCC          ; ds: latched code
LCOUNT  equ 0x8B7C          ; drv: LIFO count (init 0xFFFF, drop at 9)
LBASE   equ 0x8B72          ; drv: LIFO bytes, index = count after inc
OUTDIR  equ fname           ; filename buffer holds the full path

start:
        mov     dx, script_name
        mov     ax, 0x3D00
        int     0x21
        jc      fail
        mov     bx, ax
        mov     dx, script_buf
        mov     cx, SCRIPT_MAX
        mov     ah, 0x3F
        int     0x21
        jc      fail
        mov     [op_total], ax
        mov     ah, 0x3E
        int     0x21
        mov     ah, 0x34              ; InDOS flag → indos_seg:ofs
        int     0x21
        mov     [indos_ofs], bx
        mov     [indos_seg], es
        call    hook
        push    cs
        pop     ds
        mov     dx, msg_ok
        mov     ah, 9
        int     0x21
        mov     dx, end_res
        shr     dx, 4
        inc     dx
        mov     ax, 0x3100            ; keep resident
        int     0x21

hook:                                   ; install all three vectors
        mov     ax, 0x3521
        int     0x21
        mov     [old21], bx
        mov     [old21+2], es
        mov     ax, 0x3533
        int     0x21
        mov     [old33], bx
        mov     [old33+2], es
        mov     ax, 0x3508
        int     0x21
        mov     [old8], bx
        mov     [old8+2], es
        push    cs
        pop     ds
        mov     dx, my21
        mov     ax, 0x2521
        int     0x21
        mov     dx, my33
        mov     ax, 0x2533
        int     0x21
        mov     dx, my8
        mov     ax, 0x2508
        int     0x21
        ret

fail:
        mov     dx, msg_fail
        mov     ah, 9
        int     0x21
        mov     ax, 0x4C01
        int     0x21

; ---------------------------------------------------------------- int21

my21:                                   ; register-free capture only
        cmp     ah, 0x1A                ; set DTA — entry stub uses DX=0x28
        jne     .n1a
        cmp     dx, 0x28
        jne     .n1a
        mov     [cs:game_ds], ds
.n1a:   cmp     ah, 0x25                ; set vector — game installs its ISRs
        jne     .n25
        cmp     al, 9
        je      .drv
        cmp     al, 8
        jne     .n25
.drv:   mov     [cs:drv_seg], ds        ; ds:dx = handler → driver segment
.n25:   jmp     far [cs:old21]

; ---------------------------------------------------------------- int33

my33:
        cmp     ax, 0x000C              ; set event handler
        je      .cap
        cmp     ax, 0x0014              ; exchange event handler
        jne     .go
.cap:   mov     [cs:mhand], dx
        mov     [cs:mhand+2], es
.go:    jmp     far [cs:old33]

; ----------------------------------------------------------------- int8

my8:
        push    ax
        push    bx
        push    cx
        push    dx
        push    si
        push    di
        push    bp
        push    ds
        push    es
        cmp     byte [cs:busy], 0
        jne     .out
        mov     byte [cs:busy], 1
        call    sched
        mov     byte [cs:busy], 0
.out:   pop     es
        pop     ds
        pop     bp
        pop     di
        pop     si
        pop     dx
        pop     cx
        pop     bx
        pop     ax
        jmp     far [cs:old8]

; ----------------------------------------------------------- scheduler

sched:
        push    cs
        pop     ds
        mov     ax, [game_ds]
        or      ax, ax
        jz      .ret
        cmp     byte [armed], 0         ; first sight of the game →
        jne     .frame                  ; write the debug block once
        call    dos_free
        jne     .frame                  ; DOS busy — retry next tick
        mov     byte [armed], 1
        call    dbg_fill
        call    name_a                  ; → fname = ARMED.BIN
        push    cs
        pop     es
        mov     dx, dbg_blk
        mov     cx, dbg_len
        call    write_es
.frame:
        inc     word [tick_no]          ; one op-tick per int-8 chain call
.next:  mov     si, [op_ptr]
        cmp     si, [op_total]
        jae     .ret
        mov     ax, [script_buf+si]
        cmp     ax, [tick_no]
        ja      .ret                    ; ops are tick-sorted
        mov     cl, [script_buf+si+2]
        cmp     cl, 1
        je      .key
        cmp     cl, 2
        je      .mouse
        cmp     cl, 3
        je      .dump
        cmp     cl, 4
        je      .pokew
        cmp     cl, 5
        je      .done
        cmp     cl, 6
        je      .pal
        cmp     cl, 7
        je      .stat
        cmp     cl, 8
        je      .waitc
.adv:   add     word [op_ptr], RECSZ
        jmp     .next
.ret:   ret

.waitc:
        mov     ax, [game_ds]
        mov     es, ax
        mov     di, [script_buf+si+4]
        mov     al, [script_buf+si+6]   ; wanted byte
        cmp     [es:di], al
        jne     .ret                    ; not there yet — retry next tick
        jmp     .adv

.key:                                   ; wait for a free latch
        mov     ax, [game_ds]
        mov     es, ax
        cmp     byte [es:KPEND], 0
        jne     .ret                    ; busy — retry next tick
        mov     al, [script_buf+si+3]
        mov     ds, ax
        mov     [KCODE], al
        mov     byte [KPEND], 1
        push    cs
        pop     ds
        mov     dx, [drv_seg]           ; push the int-9 LIFO
        or      dx, dx
        jz      .adv
        mov     es, dx
        cmp     word [es:LCOUNT], 9
        je      .adv
        inc     word [es:LCOUNT]
        mov     di, [es:LCOUNT]
        mov     [es:LBASE+di], al
        jmp     .adv

.mouse:
        mov     ax, [mhand+2]
        or      ax, ax
        jz      .adv                    ; K mode installs no int33 handler
        mov     di, si
        mov     al, [script_buf+di+3]   ; event mask
        xor     ah, ah
        mov     cx, [script_buf+di+4]   ; x px — handler shifts cx >> 1
        shl     cx, 1
        mov     dx, [script_buf+di+6]   ; y px
        mov     bx, [script_buf+di+8]   ; buttons
        xor     si, si                  ; mickeys
        xor     di, di
        pushf
        call    far [mhand]
        jmp     .adv

.pokew:
        mov     ax, [script_buf+si+8]   ; d = seg selector
        call    seg_of
        or      ax, ax
        jz      .adv
        mov     es, ax
        mov     di, [script_buf+si+4]
        mov     ax, [script_buf+si+6]
        mov     [es:di], ax
        jmp     .adv

.dump:
        call    dos_free
        jne     .ret                    ; retry next tick while DOS busy
        mov     ax, [script_buf+si+8]   ; d = seg selector
        call    seg_of
        or      ax, ax
        jz      .adv
        mov     bp, ax                  ; keep the target segment
        mov     al, [script_buf+si+3]
        call    name_d                  ; → fname
        mov     es, bp
        mov     dx, [script_buf+si+4]   ; b = ofs
        mov     cx, [script_buf+si+6]   ; c = len
        call    write_es
        jmp     .adv

.pal:
        call    dos_free
        jne     .ret
        mov     al, [script_buf+si+3]
        call    name_p                  ; → fname
        mov     dx, 0x3C7               ; DAC index 0
        xor     al, al
        out     dx, al
        mov     di, pal_buf
        mov     cx, 768
.palrd: mov     dx, 0x3C9
        in      al, dx
        mov     [di], al
        inc     di
        loop    .palrd
        mov     dx, pal_buf
        mov     cx, 768
        push    cs
        pop     es
        call    write_es
        jmp     .adv

.done:
        call    dos_free
        jne     .ret
        call    name_e                  ; → fname = DONE.BIN
        push    cs
        pop     es
        mov     dx, done_byte
        mov     cx, 1
        call    write_es
        jmp     .adv

.stat:
        call    dos_free
        jne     .ret
        call    dbg_fill
        mov     al, [script_buf+si+3]
        call    name_s                  ; → fname = Sxx.BIN
        push    cs
        pop     es
        mov     dx, dbg_blk
        mov     cx, dbg_len
        call    write_es
        jmp     .adv

; refresh the debug block (ds=cs)
dbg_fill:
        mov     ax, [game_ds]
        mov     [dbg_blk], ax
        mov     ax, [drv_seg]
        mov     [dbg_blk+2], ax
        mov     ax, [tick_no]
        mov     [dbg_blk+4], ax
        mov     ax, [op_ptr]
        mov     [dbg_blk+6], ax
        mov     ax, [op_total]
        mov     [dbg_blk+8], ax
        mov     ax, [mhand]
        mov     [dbg_blk+10], ax
        mov     ax, [mhand+2]
        mov     [dbg_blk+12], ax
        mov     ax, [indos_seg]
        mov     [dbg_blk+14], ax
        mov     ax, [indos_ofs]
        mov     [dbg_blk+16], ax
        ret

; ------------------------------------------------------------- helpers

; ax = selector (0xFFFF → game ds, 0xFFFE → driver seg, else absolute)
seg_of:
        cmp     ax, 0xFFFF
        jne     .s1
        mov     ax, [game_ds]
        ret
.s1:    cmp     ax, 0xFFFE
        jne     .s2
        mov     ax, [drv_seg]
.s2:    ret

; ZF clear when the DOS InDOS flag is zero (es clobbered)
dos_free:
        mov     es, [indos_seg]
        mov     di, [indos_ofs]
        cmp     byte [es:di], 0
        ret

; al = id → fname = "C:\DIFFTEST\OUT\Dxx.BIN" (xx = hex of al)
name_d:
        push    si
        push    ax
        mov     bx, fname
        mov     cx, name_pfx_len
        mov     si, name_pfx
.nc:    mov     al, [si]
        mov     [bx], al
        inc     si
        inc     bx
        loop    .nc
        mov     byte [bx], 'D'
        inc     bx
        pop     ax
        call    hex2
        mov     si, name_sfx
        jmp     suffix

; al = id → fname = "...\Pxx.BIN"
name_p:
        push    si
        push    ax
        mov     bx, fname
        mov     cx, name_pfx_len
        mov     si, name_pfx
.nc:    mov     al, [si]
        mov     [bx], al
        inc     si
        inc     bx
        loop    .nc
        mov     byte [bx], 'P'
        inc     bx
        pop     ax
        call    hex2
        mov     si, name_sfx
suffix: mov     cx, name_sfx_len
.sc:    mov     al, [si]
        mov     [bx], al
        inc     si
        inc     bx
        loop    .sc
        mov     byte [bx], 0
        pop     si
        ret

; fname = "...\DONE.BIN"
name_e:
        push    si
        mov     bx, fname
        mov     si, name_done
        mov     cx, name_done_len
        jmp     whole

; fname = "...\ARMED.BIN"
name_a:
        push    si
        mov     bx, fname
        mov     si, name_armed
        mov     cx, name_armed_len
whole:  mov     al, [si]
        mov     [bx], al
        inc     si
        inc     bx
        loop    whole
        mov     byte [bx], 0
        pop     si
        ret

; al = id → fname = "...\Sxx.BIN"
name_s:
        push    si
        push    ax
        mov     bx, fname
        mov     cx, name_pfx_len
        mov     si, name_pfx
.nc:    mov     al, [si]
        mov     [bx], al
        inc     si
        inc     bx
        loop    .nc
        mov     byte [bx], 'S'
        inc     bx
        pop     ax
        call    hex2
        mov     si, name_sfx
        jmp     suffix

; al → two hex chars at bx, bx += 2 (clobbers dx)
hex2:
        mov     dx, ax
        mov     al, dl
        shr     al, 4
        call    nib
        mov     al, dl
        and     al, 0x0F
nib:    add     al, '0'
        cmp     al, '9'
        jbe     .st
        add     al, 7
.st:    mov     [bx], al
        inc     bx
        ret

; create fname, write cx bytes from es:dx, close (bx/dx/cx/es kept)
write_es:
        push    dx
        push    cx
        mov     ah, 0x3C
        xor     cx, cx
        mov     dx, fname
        int     0x21
        pop     cx
        pop     dx
        jc      .out
        push    bx
        push    ds
        mov     bx, ax
        push    es
        pop     ds                      ; source segment
        mov     ah, 0x40
        int     0x21
        mov     ah, 0x3E
        int     0x21
        pop     ds
        pop     bx
.out:   ret

; ------------------------------------------------------------- resident data

script_name db  'SCRIPT.BIN', 0
msg_ok      db  'PROBE: resident', 13, 10, '$'
msg_fail    db  'PROBE: SCRIPT.BIN missing', 13, 10, '$'
name_pfx    db  'C:\DIFFTEST\OUT\'
name_pfx_len equ $-name_pfx
name_sfx    db  '.BIN'
name_sfx_len equ $-name_sfx
name_done   db  'C:\DIFFTEST\OUT\DONE.BIN'
name_done_len equ $-name_done
name_armed  db  'C:\DIFFTEST\OUT\ARMED.BIN'
name_armed_len equ $-name_armed
done_byte   db  'K'

op_total    dw  0
op_ptr      dw  0
tick_no     dw  0
game_ds     dw  0
drv_seg     dw  0
busy        db  0
armed       db  0
old21       dd  0
old33       dd  0
old8        dd  0
mhand       dd  0
indos_seg   dw  0
indos_ofs   dw  0

fname       times 32 db 0
dbg_blk     times 24 db 0
dbg_len     equ 24
pal_buf     times 768 db 0
SCRIPT_MAX  equ 6144
script_buf  times SCRIPT_MAX db 0

end_res:
