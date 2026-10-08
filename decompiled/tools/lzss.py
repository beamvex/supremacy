import struct, sys

def decode(buf):
    """LZSS N=4096 F=18 THRESH=2, ring zero-init, r=0xFEE, flags LSB-first,
    token LE {pos&0xFF, (pos>>4)&0xF0 | (len-3)}, end: flag0 + 00 00 [00]."""
    ring = bytearray(4096)
    r = 0xFEE
    out = bytearray()
    i = 0
    flags = 0
    flagbits = 0  # count of consumed bits sentinel: emulate dx>>1 with 0x100 test
    while True:
        # shift flag register right; when bit8 sentinel shifted out, reload
        flags >>= 1
        if not (flags & 0x100):
            flags = buf[i] | 0xFF00
            i += 1
        if flags & 1:
            b = buf[i]; i += 1
            out.append(b)
            ring[r] = b; r = (r + 1) & 0xFFF
        else:
            t = buf[i] | (buf[i+1] << 8); i += 2
            if t == 0 and buf[i] == 0:
                break
            pos = (t & 0xFF) | ((t & 0xF000) >> 4)
            ln = ((t >> 8) & 0xF) + 3
            for _ in range(ln):
                b = ring[pos]
                out.append(b)
                ring[r] = b; r = (r + 1) & 0xFFF
                pos = (pos + 1) & 0xFFF
    return bytes(out)

if __name__ == '__main__':
    binfile, gphfile = sys.argv[1], sys.argv[2]
    data = open(binfile, 'rb').read()
    g = open(gphfile, 'rb').read()
    n = int(sys.argv[3]) if len(sys.argv) > 3 else 0
    x,y,w,h,ofs,ex = struct.unpack('<4H2H', g[n*12:n*12+12])
    print(f'rec {n}: x={x} y={y} w={w} h={h} ofs=0x{ofs:x} ex={ex}', file=sys.stderr)
    stream = data[ofs*16:]
    dec = decode(stream)
    print(f'decoded {len(dec)} bytes (expected {w*h})', file=sys.stderr)
    sys.stdout.buffer.write(dec)
