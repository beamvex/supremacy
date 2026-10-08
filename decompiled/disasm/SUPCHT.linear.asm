00000000  4D                dec bp
00000001  5A                pop dx
00000002  D001              rol byte [bx+di],1
00000004  660027            o32 add [bx],ah
00000007  02A00081          add ah,[bx+si-0x7f00]
0000000B  00FF              add bh,bh
0000000D  FF1D              call word far [di]
0000000F  0C00              or al,0x0
00000011  08AEFBC8          or [bp-0x3705],ch
00000015  00E5              add ch,ah
00000017  07                pop es
00000018  1C00              sbb al,0x0
0000001A  0000              add [bx+si],al
0000001C  56                push si
0000001D  BA0000            mov dx,0x0
00000020  37                aaa
00000021  0000              add [bx+si],al
00000023  003C              add [si],bh
00000025  0000              add [bx+si],al
00000027  004100            add [bx+di+0x0],al
0000002A  0000              add [bx+si],al
0000002C  46                inc si
0000002D  0000              add [bx+si],al
0000002F  004B00            add [bp+di+0x0],cl
00000032  0000              add [bx+si],al
00000034  50                push ax
00000035  0000              add [bx+si],al
00000037  005B00            add [bp+di+0x0],bl
0000003A  0000              add [bx+si],al
0000003C  7100              jno 0x3e
0000003E  0000              add [bx+si],al
00000040  7A00              jpe 0x42
00000042  0000              add [bx+si],al
00000044  8D00              lea ax,[bx+si]
00000046  0000              add [bx+si],al
00000048  96                xchg ax,si
00000049  0000              add [bx+si],al
0000004B  00AF0000          add [bx+0x0],ch
0000004F  00C5              add ch,al
00000051  0000              add [bx+si],al
00000053  00CE              add dh,cl
00000055  0000              add [bx+si],al
00000057  00E4              add ah,ah
00000059  0000              add [bx+si],al
0000005B  00FD              add ch,bh
0000005D  0000              add [bx+si],al
0000005F  00060100          add [0x1],al
00000063  001F              add [bx],bl
00000065  0100              add [bx+si],ax
00000067  0035              add [di],dh
00000069  0100              add [bx+si],ax
0000006B  003E0100          add [0x1],bh
0000006F  004701            add [bx+0x1],al
00000072  0000              add [bx+si],al
00000074  4D                dec bp
00000075  0100              add [bx+si],ax
00000077  005601            add [bp+0x1],dl
0000007A  0000              add [bx+si],al
0000007C  5F                pop di
0000007D  0100              add [bx+si],ax
0000007F  006501            add [di+0x1],ah
00000082  0000              add [bx+si],al
00000084  6E                outsb
00000085  0100              add [bx+si],ax
00000087  00870100          add [bx+0x1],al
0000008B  00900100          add [bx+si+0x1],dl
0000008F  00A90100          add [bx+di+0x1],ch
00000093  00B20100          add [bp+si+0x1],dh
00000097  00C8              add al,cl
00000099  0100              add [bx+si],ax
0000009B  00E1              add cl,ah
0000009D  0100              add [bx+si],ax
0000009F  00EA              add dl,ch
000000A1  0100              add [bx+si],ax
000000A3  0003              add [bp+di],al
000000A5  0200              add al,[bx+si]
000000A7  000C              add [si],cl
000000A9  0200              add al,[bx+si]
000000AB  0025              add [di],ah
000000AD  0200              add al,[bx+si]
000000AF  002E0200          add [0x2],ch
000000B3  004702            add [bx+0x2],al
000000B6  0000              add [bx+si],al
000000B8  50                push ax
000000B9  0200              add al,[bx+si]
000000BB  006602            add [bp+0x2],ah
000000BE  0000              add [bx+si],al
000000C0  7F02              jg 0xc4
000000C2  0000              add [bx+si],al
000000C4  8802              mov [bp+si],al
000000C6  0000              add [bx+si],al
000000C8  9E                sahf
000000C9  0200              add al,[bx+si]
000000CB  00B70200          add [bx+0x2],dh
000000CF  00C0              add al,al
000000D1  0200              add al,[bx+si]
000000D3  00D9              add cl,bl
000000D5  0200              add al,[bx+si]
000000D7  00E2              add dl,ah
000000D9  0200              add al,[bx+si]
000000DB  00FB              add bl,bh
000000DD  0200              add al,[bx+si]
000000DF  0004              add [si],al
000000E1  0300              add ax,[bx+si]
000000E3  0011              add [bx+di],dl
000000E5  0300              add ax,[bx+si]
000000E7  001B              add [bp+di],bl
000000E9  0300              add ax,[bx+si]
000000EB  0024              add [si],ah
000000ED  0300              add ax,[bx+si]
000000EF  002E0300          add [0x3],ch
000000F3  0037              add [bx],dh
000000F5  0300              add ax,[bx+si]
000000F7  004103            add [bx+di+0x3],al
000000FA  0000              add [bx+si],al
000000FC  4B                dec bx
000000FD  0300              add ax,[bx+si]
000000FF  005803            add [bx+si+0x3],bl
00000102  0000              add [bx+si],al
00000104  650300            add ax,[gs:bx+si]
00000107  007703            add [bx+0x3],dh
0000010A  0000              add [bx+si],al
0000010C  8A03              mov al,[bp+di]
0000010E  0000              add [bx+si],al
00000110  9C                pushf
00000111  0300              add ax,[bx+si]
00000113  00AF0300          add [bx+0x3],ch
00000117  00C1              add cl,al
00000119  0300              add ax,[bx+si]
0000011B  00D3              add bl,dl
0000011D  0300              add ax,[bx+si]
0000011F  00D8              add al,bl
00000121  0300              add ax,[bx+si]
00000123  00E6              add dh,ah
00000125  0300              add ax,[bx+si]
00000127  00F3              add bl,dh
00000129  0300              add ax,[bx+si]
0000012B  0000              add [bx+si],al
0000012D  0400              add al,0x0
0000012F  0012              add [bp+si],dl
00000131  0400              add al,0x0
00000133  0022              add [bp+si],ah
00000135  0400              add al,0x0
00000137  0034              add [si],dh
00000139  0400              add al,0x0
0000013B  004404            add [si+0x4],al
0000013E  0000              add [bx+si],al
00000140  56                push si
00000141  0400              add al,0x0
00000143  005E04            add [bp+0x4],bl
00000146  0000              add [bx+si],al
00000148  670400            a32 add al,0x0
0000014B  006C04            add [si+0x4],ch
0000014E  0000              add [bx+si],al
00000150  7A04              jpe 0x156
00000152  0000              add [bx+si],al
00000154  90                nop
00000155  0400              add al,0x0
00000157  00990400          add [bx+di+0x4],bl
0000015B  00B20400          add [bp+si+0x4],dh
0000015F  00BB0400          add [bp+di+0x4],bh
00000163  00D1              add cl,dl
00000165  0400              add al,0x0
00000167  00EA              add dl,ch
00000169  0400              add al,0x0
0000016B  00F3              add bl,dh
0000016D  0400              add al,0x0
0000016F  00FC              add ah,bh
00000171  0400              add al,0x0
00000173  0005              add [di],al
00000175  050000            add ax,0x0
00000178  1B05              sbb ax,[di]
0000017A  0000              add [bx+si],al
0000017C  3405              xor al,0x5
0000017E  0000              add [bx+si],al
00000180  46                inc si
00000181  050000            add ax,0x0
00000184  59                pop cx
00000185  050000            add ax,0x0
00000188  5E                pop si
00000189  050000            add ax,0x0
0000018C  6305              arpl [di],ax
0000018E  0000              add [bx+si],al
00000190  7105              jno 0x197
00000192  0000              add [bx+si],al
00000194  8705              xchg ax,[di]
00000196  0000              add [bx+si],al
00000198  90                nop
00000199  050000            add ax,0x0
0000019C  A90500            test ax,0x5
0000019F  00B20500          add [bp+si+0x5],dh
000001A3  00C8              add al,cl
000001A5  050000            add ax,0x0
000001A8  E105              loope 0x1af
000001AA  0000              add [bx+si],al
000001AC  EA050000F3        jmp word 0xf300:word 0x5
000001B1  050000            add ax,0x0
000001B4  09060000          or [0x0],ax
000001B8  22060000          and al,[0x0]
000001BC  2B060000          sub ax,[0x0]
000001C0  41                inc cx
000001C1  06                push es
000001C2  0000              add [bx+si],al
000001C4  5A                pop dx
000001C5  06                push es
000001C6  0000              add [bx+si],al
000001C8  63060000          arpl [0x0],ax
000001CC  7C06              jl 0x1d4
000001CE  0000              add [bx+si],al
000001D0  85060000          test [0x0],ax
000001D4  92                xchg ax,dx
000001D5  06                push es
000001D6  0000              add [bx+si],al
000001D8  A4                movsb
000001D9  06                push es
000001DA  0000              add [bx+si],al
000001DC  BA0600            mov dx,0x6
000001DF  00D3              add bl,dl
000001E1  06                push es
000001E2  0000              add [bx+si],al
000001E4  DC060000          fadd qword [0x0]
000001E8  F206              repne push es
000001EA  0000              add [bx+si],al
000001EC  0B07              or ax,[bx]
000001EE  0000              add [bx+si],al
000001F0  1407              adc al,0x7
000001F2  0000              add [bx+si],al
000001F4  2D0700            sub ax,0x7
000001F7  00360700          add [0x7],dh
000001FB  004F07            add [bx+0x7],cl
000001FE  0000              add [bx+si],al
00000200  58                pop ax
00000201  07                pop es
00000202  0000              add [bx+si],al
00000204  7107              jno 0x20d
00000206  0000              add [bx+si],al
00000208  7A07              jpe 0x211
0000020A  0000              add [bx+si],al
0000020C  93                xchg ax,bx
0000020D  07                pop es
0000020E  0000              add [bx+si],al
00000210  9C                pushf
00000211  07                pop es
00000212  0000              add [bx+si],al
00000214  B507              mov ch,0x7
00000216  0000              add [bx+si],al
00000218  BE0700            mov si,0x7
0000021B  00D7              add bh,dl
0000021D  07                pop es
0000021E  0000              add [bx+si],al
00000220  E007              loopne 0x229
00000222  0000              add [bx+si],al
00000224  F9                stc
00000225  07                pop es
00000226  0000              add [bx+si],al
00000228  0208              add cl,[bx+si]
0000022A  0000              add [bx+si],al
0000022C  1B08              sbb cx,[bx+si]
0000022E  0000              add [bx+si],al
00000230  2408              and al,0x8
00000232  0000              add [bx+si],al
00000234  3D0800            cmp ax,0x8
00000237  004608            add [bp+0x8],al
0000023A  0000              add [bx+si],al
0000023C  5F                pop di
0000023D  0800              or [bx+si],al
0000023F  006808            add [bx+si+0x8],ch
00000242  0000              add [bx+si],al
00000244  81080000          or word [bx+si],0x0
00000248  8A08              mov cl,[bx+si]
0000024A  0000              add [bx+si],al
0000024C  A00800            mov al,[0x8]
0000024F  00B90800          add [bx+di+0x8],bh
00000253  00C2              add dl,al
00000255  0800              or [bx+si],al
00000257  00DB              add bl,bl
00000259  0800              or [bx+si],al
0000025B  00E4              add ah,ah
0000025D  0800              or [bx+si],al
0000025F  00FD              add ch,bh
00000261  0800              or [bx+si],al
00000263  00060900          add [0x9],al
00000267  001F              add [bx],bl
00000269  0900              or [bx+si],ax
0000026B  0028              add [bx+si],ch
0000026D  0900              or [bx+si],ax
0000026F  004109            add [bx+di+0x9],al
00000272  0000              add [bx+si],al
00000274  4A                dec dx
00000275  0900              or [bx+si],ax
00000277  006309            add [bp+di+0x9],ah
0000027A  0000              add [bx+si],al
0000027C  6C                insb
0000027D  0900              or [bx+si],ax
0000027F  00820900          add [bp+si+0x9],al
00000283  009B0900          add [bp+di+0x9],bl
00000287  00A80900          add [bx+si+0x9],ch
0000028B  00C1              add cl,al
0000028D  0900              or [bx+si],ax
0000028F  00CA              add dl,cl
00000291  0900              or [bx+si],ax
00000293  00E3              add bl,ah
00000295  0900              or [bx+si],ax
00000297  00EC              add ah,ch
00000299  0900              or [bx+si],ax
0000029B  0005              add [di],al
0000029D  0A00              or al,[bx+si]
0000029F  000E0A00          add [0xa],cl
000002A3  0027              add [bx],ah
000002A5  0A00              or al,[bx+si]
000002A7  0030              add [bx+si],dh
000002A9  0A00              or al,[bx+si]
000002AB  00490A            add [bx+di+0xa],cl
000002AE  0000              add [bx+si],al
000002B0  52                push dx
000002B1  0A00              or al,[bx+si]
000002B3  00680A            add [bx+si+0xa],ch
000002B6  0000              add [bx+si],al
000002B8  810A0000          or word [bp+si],0x0
000002BC  8A0A              mov cl,[bp+si]
000002BE  0000              add [bx+si],al
000002C0  A30A00            mov [0xa],ax
000002C3  00AC0A00          add [si+0xa],ch
000002C7  00C2              add dl,al
000002C9  0A00              or al,[bx+si]
000002CB  00DB              add bl,bl
000002CD  0A00              or al,[bx+si]
000002CF  00E4              add ah,ah
000002D1  0A00              or al,[bx+si]
000002D3  00F1              add cl,dh
000002D5  0A00              or al,[bx+si]
000002D7  00FB              add bl,bh
000002D9  0A00              or al,[bx+si]
000002DB  0004              add [si],al
000002DD  0B00              or ax,[bx+si]
000002DF  000E0B00          add [0xb],cl
000002E3  001B              add [bp+di],bl
000002E5  0B00              or ax,[bx+si]
000002E7  0028              add [bx+si],ch
000002E9  0B00              or ax,[bx+si]
000002EB  00460B            add [bp+0xb],al
000002EE  0000              add [bx+si],al
000002F0  58                pop ax
000002F1  0B00              or ax,[bx+si]
000002F3  006A0B            add [bp+si+0xb],ch
000002F6  0000              add [bx+si],al
000002F8  6F                outsw
000002F9  0B00              or ax,[bx+si]
000002FB  00880B00          add [bx+si+0xb],cl
000002FF  00910B00          add [bx+di+0xb],dl
00000303  00970B00          add [bx+0xb],dl
00000307  00AA0B00          add [bp+si+0xb],ch
0000030B  00C8              add al,cl
0000030D  0B00              or ax,[bx+si]
0000030F  00DA              add dl,bl
00000311  0B00              or ax,[bx+si]
00000313  00EC              add ah,ch
00000315  0B00              or ax,[bx+si]
00000317  00F1              add cl,dh
00000319  0B00              or ax,[bx+si]
0000031B  000A              add [bp+si],cl
0000031D  0C00              or al,0x0
0000031F  0013              add [bp+di],dl
00000321  0C00              or al,0x0
00000323  0019              add [bx+di],bl
00000325  0C00              or al,0x0
00000327  002C              add [si],ch
00000329  0C00              or al,0x0
0000032B  004A0C            add [bp+si+0xc],cl
0000032E  0000              add [bx+si],al
00000330  5C                pop sp
00000331  0C00              or al,0x0
00000333  006E0C            add [bp+0xc],ch
00000336  0000              add [bx+si],al
00000338  730C              jnc 0x346
0000033A  0000              add [bx+si],al
0000033C  8C0C              mov word [si],cs
0000033E  0000              add [bx+si],al
00000340  95                xchg ax,bp
00000341  0C00              or al,0x0
00000343  009B0C00          add [bp+di+0xc],bl
00000347  00AE0C00          add [bp+0xc],ch
0000034B  00CC              add ah,cl
0000034D  0C00              or al,0x0
0000034F  00DE              add dh,bl
00000351  0C00              or al,0x0
00000353  00F0              add al,dh
00000355  0C00              or al,0x0
00000357  00F5              add ch,dh
00000359  0C00              or al,0x0
0000035B  000E0D00          add [0xd],cl
0000035F  0017              add [bx],dl
00000361  0D0000            or ax,0x0
00000364  1D0D00            sbb ax,0xd
00000367  0030              add [bx+si],dh
00000369  0D0000            or ax,0x0
0000036C  4E                dec si
0000036D  0D0000            or ax,0x0
00000370  60                pusha
00000371  0D0000            or ax,0x0
00000374  720D              jc 0x383
00000376  0000              add [bx+si],al
00000378  770D              ja 0x387
0000037A  0000              add [bx+si],al
0000037C  90                nop
0000037D  0D0000            or ax,0x0
00000380  99                cwd
00000381  0D0000            or ax,0x0
00000384  9F                lahf
00000385  0D0000            or ax,0x0
00000388  B20D              mov dl,0xd
0000038A  0000              add [bx+si],al
0000038C  C40D              les cx,word [di]
0000038E  0000              add [bx+si],al
00000390  D7                xlatb
00000391  0D0000            or ax,0x0
00000394  F5                cmc
00000395  0D0000            or ax,0x0
00000398  07                pop es
00000399  0E                push cs
0000039A  0000              add [bx+si],al
0000039C  190E0000          sbb [0x0],cx
000003A0  1E                push ds
000003A1  0E                push cs
000003A2  0000              add [bx+si],al
000003A4  37                aaa
000003A5  0E                push cs
000003A6  0000              add [bx+si],al
000003A8  40                inc ax
000003A9  0E                push cs
000003AA  0000              add [bx+si],al
000003AC  46                inc si
000003AD  0E                push cs
000003AE  0000              add [bx+si],al
000003B0  59                pop cx
000003B1  0E                push cs
000003B2  0000              add [bx+si],al
000003B4  9B0E              wait push cs
000003B6  0000              add [bx+si],al
000003B8  A5                movsw
000003B9  0E                push cs
000003BA  0000              add [bx+si],al
000003BC  B50E              mov ch,0xe
000003BE  0000              add [bx+si],al
000003C0  BF0E00            mov di,0xe
000003C3  00CE              add dh,cl
000003C5  0E                push cs
000003C6  0000              add [bx+si],al
000003C8  D7                xlatb
000003C9  0E                push cs
000003CA  0000              add [bx+si],al
000003CC  DC0E0000          fmul qword [0x0]
000003D0  EA0E0000FF        jmp word 0xff00:word 0xe
000003D5  0E                push cs
000003D6  0000              add [bx+si],al
000003D8  0C0F              or al,0xf
000003DA  0000              add [bx+si],al
000003DC  190F              sbb [bx],cx
000003DE  0000              add [bx+si],al
000003E0  380F              cmp [bx],cl
000003E2  0000              add [bx+si],al
000003E4  52                push dx
000003E5  0F0000            sldt word [bx+si]
000003E8  6C                insb
000003E9  0F0000            sldt word [bx+si]
000003EC  860F              xchg cl,[bx]
000003EE  0000              add [bx+si],al
000003F0  A00F00            mov al,[0xf]
000003F3  00BA0F00          add [bp+si+0xf],bh
000003F7  00CA              add dl,cl
000003F9  0F0000            sldt word [bx+si]
000003FC  E90F00            jmp 0x40e
000003FF  0003              add [bp+di],al
00000401  1000              adc [bx+si],al
00000403  001D              add [di],bl
00000405  1000              adc [bx+si],al
00000407  0037              add [bx],dh
00000409  1000              adc [bx+si],al
0000040B  005110            add [bx+di+0x10],dl
0000040E  0000              add [bx+si],al
00000410  6B1000            imul dx,[bx+si],0x0
00000413  007B10            add [bp+di+0x10],bh
00000416  0000              add [bx+si],al
00000418  9A100000B4        call word 0xb400:word 0x10
0000041D  1000              adc [bx+si],al
0000041F  00CE              add dh,cl
00000421  1000              adc [bx+si],al
00000423  00E8              add al,ch
00000425  1000              adc [bx+si],al
00000427  0002              add [bp+si],al
00000429  1100              adc [bx+si],ax
0000042B  001C              add [si],bl
0000042D  1100              adc [bx+si],ax
0000042F  0029              add [bx+di],ch
00000431  1100              adc [bx+si],ax
00000433  00361100          add [0x11],dh
00000437  004011            add [bx+si+0x11],al
0000043A  0000              add [bx+si],al
0000043C  49                dec cx
0000043D  1100              adc [bx+si],ax
0000043F  004E11            add [bp+0x11],cl
00000442  0000              add [bx+si],al
00000444  5C                pop sp
00000445  1100              adc [bx+si],ax
00000447  007111            add [bx+di+0x11],dh
0000044A  0000              add [bx+si],al
0000044C  7E11              jng 0x45f
0000044E  0000              add [bx+si],al
00000450  8B11              mov dx,[bx+di]
00000452  0000              add [bx+si],al
00000454  AA                stosb
00000455  1100              adc [bx+si],ax
00000457  00C4              add ah,al
00000459  1100              adc [bx+si],ax
0000045B  00DE              add dh,bl
0000045D  1100              adc [bx+si],ax
0000045F  00F8              add al,bh
00000461  1100              adc [bx+si],ax
00000463  0012              add [bp+si],dl
00000465  1200              adc al,[bx+si]
00000467  002C              add [si],ch
00000469  1200              adc al,[bx+si]
0000046B  003C              add [si],bh
0000046D  1200              adc al,[bx+si]
0000046F  005B12            add [bp+di+0x12],bl
00000472  0000              add [bx+si],al
00000474  7512              jnz 0x488
00000476  0000              add [bx+si],al
00000478  8F                db 0x8f
00000479  1200              adc al,[bx+si]
0000047B  00A91200          add [bx+di+0x12],ch
0000047F  00C3              add bl,al
00000481  1200              adc al,[bx+si]
00000483  00DD              add ch,bl
00000485  1200              adc al,[bx+si]
00000487  00ED              add ch,ch
00000489  1200              adc al,[bx+si]
0000048B  000C              add [si],cl
0000048D  1300              adc ax,[bx+si]
0000048F  00261300          add [0x13],ah
00000493  004013            add [bx+si+0x13],al
00000496  0000              add [bx+si],al
00000498  5A                pop dx
00000499  1300              adc ax,[bx+si]
0000049B  007413            add [si+0x13],dh
0000049E  0000              add [bx+si],al
000004A0  8E13              mov ss,word [bp+di]
000004A2  0000              add [bx+si],al
000004A4  9B1300            wait adc ax,[bx+si]
000004A7  00A41300          add [si+0x13],ah
000004AB  00A91300          add [bx+di+0x13],ch
000004AF  00B71300          add [bx+0x13],dh
000004B3  00DD              add ch,bl
000004B5  1300              adc ax,[bx+si]
000004B7  00EA              add dl,ch
000004B9  1300              adc ax,[bx+si]
000004BB  0009              add [bx+di],cl
000004BD  1400              adc al,0x0
000004BF  0028              add [bx+si],ch
000004C1  1400              adc al,0x0
000004C3  004F14            add [bx+0x14],cl
000004C6  0000              add [bx+si],al
000004C8  69140000          imul dx,[si],0x0
000004CC  831400            adc word [si],0x0
000004CF  009D1400          add [di+0x14],bl
000004D3  00B71400          add [bx+0x14],dh
000004D7  00D1              add cl,dl
000004D9  1400              adc al,0x0
000004DB  00EB              add bl,ch
000004DD  1400              adc al,0x0
000004DF  0005              add [di],al
000004E1  150000            adc ax,0x0
000004E4  1F                pop ds
000004E5  150000            adc ax,0x0
000004E8  3915              cmp [di],dx
000004EA  0000              add [bx+si],al
000004EC  53                push bx
000004ED  150000            adc ax,0x0
000004F0  6D                insw
000004F1  150000            adc ax,0x0
000004F4  8715              xchg dx,[di]
000004F6  0000              add [bx+si],al
000004F8  A11500            mov ax,[0x15]
000004FB  00BB1500          add [bp+di+0x15],bh
000004FF  00D5              add ch,dl
00000501  150000            adc ax,0x0
00000504  EF                out dx,ax
00000505  150000            adc ax,0x0
00000508  09160000          or [0x0],dx
0000050C  23160000          and dx,[0x0]
00000510  3D1600            cmp ax,0x16
00000513  005716            add [bx+0x16],dl
00000516  0000              add [bx+si],al
00000518  7116              jno 0x530
0000051A  0000              add [bx+si],al
0000051C  8B160000          mov dx,[0x0]
00000520  A5                movsw
00000521  16                push ss
00000522  0000              add [bx+si],al
00000524  BF1600            mov di,0x16
00000527  00D9              add cl,bl
00000529  16                push ss
0000052A  0000              add [bx+si],al
0000052C  F316              rep push ss
0000052E  0000              add [bx+si],al
00000530  0D1700            or ax,0x17
00000533  0027              add [bx],ah
00000535  17                pop ss
00000536  0000              add [bx+si],al
00000538  41                inc cx
00000539  17                pop ss
0000053A  0000              add [bx+si],al
0000053C  5B                pop bx
0000053D  17                pop ss
0000053E  0000              add [bx+si],al
00000540  7517              jnz 0x559
00000542  0000              add [bx+si],al
00000544  8F                db 0x8f
00000545  17                pop ss
00000546  0000              add [bx+si],al
00000548  A91700            test ax,0x17
0000054B  00C3              add bl,al
0000054D  17                pop ss
0000054E  0000              add [bx+si],al
00000550  DD17              fst qword [bx]
00000552  0000              add [bx+si],al
00000554  F717              not word [bx]
00000556  0000              add [bx+si],al
00000558  1118              adc [bx+si],bx
0000055A  0000              add [bx+si],al
0000055C  2B18              sub bx,[bx+si]
0000055E  0000              add [bx+si],al
00000560  45                inc bp
00000561  1800              sbb [bx+si],al
00000563  005F18            add [bx+0x18],bl
00000566  0000              add [bx+si],al
00000568  7918              jns 0x582
0000056A  0000              add [bx+si],al
0000056C  93                xchg ax,bx
0000056D  1800              sbb [bx+si],al
0000056F  00AD1800          add [di+0x18],ch
00000573  00C7              add bh,al
00000575  1800              sbb [bx+si],al
00000577  00E1              add cl,ah
00000579  1800              sbb [bx+si],al
0000057B  00FB              add bl,bh
0000057D  1800              sbb [bx+si],al
0000057F  0015              add [di],dl
00000581  1900              sbb [bx+si],ax
00000583  002F              add [bx],ch
00000585  1900              sbb [bx+si],ax
00000587  004919            add [bx+di+0x19],cl
0000058A  0000              add [bx+si],al
0000058C  6319              arpl [bx+di],bx
0000058E  0000              add [bx+si],al
00000590  7D19              jnl 0x5ab
00000592  0000              add [bx+si],al
00000594  97                xchg ax,di
00000595  1900              sbb [bx+si],ax
00000597  00B11900          add [bx+di+0x19],dh
0000059B  00CB              add bl,cl
0000059D  1900              sbb [bx+si],ax
0000059F  00E5              add ch,ah
000005A1  1900              sbb [bx+si],ax
000005A3  00FF              add bh,bh
000005A5  1900              sbb [bx+si],ax
000005A7  0019              add [bx+di],bl
000005A9  1A00              sbb al,[bx+si]
000005AB  0033              add [bp+di],dh
000005AD  1A00              sbb al,[bx+si]
000005AF  004D1A            add [di+0x1a],cl
000005B2  0000              add [bx+si],al
000005B4  671A00            sbb al,[eax]
000005B7  00811A00          add [bx+di+0x1a],al
000005BB  009B1A00          add [bp+di+0x1a],bl
000005BF  00B51A00          add [di+0x1a],dh
000005C3  00CF              add bh,cl
000005C5  1A00              sbb al,[bx+si]
000005C7  00E9              add cl,ch
000005C9  1A00              sbb al,[bx+si]
000005CB  0003              add [bp+di],al
000005CD  1B00              sbb ax,[bx+si]
000005CF  001D              add [di],bl
000005D1  1B00              sbb ax,[bx+si]
000005D3  0037              add [bx],dh
000005D5  1B00              sbb ax,[bx+si]
000005D7  00511B            add [bx+di+0x1b],dl
000005DA  0000              add [bx+si],al
000005DC  6B1B00            imul bx,[bp+di],0x0
000005DF  00851B00          add [di+0x1b],al
000005E3  009F1B00          add [bx+0x1b],bl
000005E7  00B91B00          add [bx+di+0x1b],bh
000005EB  00D3              add bl,dl
000005ED  1B00              sbb ax,[bx+si]
000005EF  00ED              add ch,ch
000005F1  1B00              sbb ax,[bx+si]
000005F3  0007              add [bx],al
000005F5  1C00              sbb al,0x0
000005F7  0021              add [bx+di],ah
000005F9  1C00              sbb al,0x0
000005FB  003B              add [bp+di],bh
000005FD  1C00              sbb al,0x0
000005FF  00551C            add [di+0x1c],dl
00000602  0000              add [bx+si],al
00000604  6F                outsw
00000605  1C00              sbb al,0x0
00000607  00891C00          add [bx+di+0x1c],cl
0000060B  00A31C00          add [bp+di+0x1c],ah
0000060F  00BD1C00          add [di+0x1c],bh
00000613  00D7              add bh,dl
00000615  1C00              sbb al,0x0
00000617  00F1              add cl,dh
00000619  1C00              sbb al,0x0
0000061B  000B              add [bp+di],cl
0000061D  1D0000            sbb ax,0x0
00000620  251D00            and ax,0x1d
00000623  003F              add [bx],bh
00000625  1D0000            sbb ax,0x0
00000628  59                pop cx
00000629  1D0000            sbb ax,0x0
0000062C  731D              jnc 0x64b
0000062E  0000              add [bx+si],al
00000630  8D1D              lea bx,[di]
00000632  0000              add [bx+si],al
00000634  A7                cmpsw
00000635  1D0000            sbb ax,0x0
00000638  C11D00            rcr word [di],byte 0x0
0000063B  00DB              add bl,bl
0000063D  1D0000            sbb ax,0x0
00000640  F5                cmc
00000641  1D0000            sbb ax,0x0
00000644  0F1E00            nop 0x1e,ax,[bx+si]
00000647  0029              add [bx+di],ch
00000649  1E                push ds
0000064A  0000              add [bx+si],al
0000064C  43                inc bx
0000064D  1E                push ds
0000064E  0000              add [bx+si],al
00000650  5D                pop bp
00000651  1E                push ds
00000652  0000              add [bx+si],al
00000654  771E              ja 0x674
00000656  0000              add [bx+si],al
00000658  91                xchg ax,cx
00000659  1E                push ds
0000065A  0000              add [bx+si],al
0000065C  AB                stosw
0000065D  1E                push ds
0000065E  0000              add [bx+si],al
00000660  C51E0000          lds bx,word [0x0]
00000664  DF1E0000          fistp word [0x0]
00000668  F9                stc
00000669  1E                push ds
0000066A  0000              add [bx+si],al
0000066C  131F              adc bx,[bx]
0000066E  0000              add [bx+si],al
00000670  2D1F00            sub ax,0x1f
00000673  00471F            add [bx+0x1f],al
00000676  0000              add [bx+si],al
00000678  61                popa
00000679  1F                pop ds
0000067A  0000              add [bx+si],al
0000067C  7B1F              jpo 0x69d
0000067E  0000              add [bx+si],al
00000680  95                xchg ax,bp
00000681  1F                pop ds
00000682  0000              add [bx+si],al
00000684  AF                scasw
00000685  1F                pop ds
00000686  0000              add [bx+si],al
00000688  C9                leave
00000689  1F                pop ds
0000068A  0000              add [bx+si],al
0000068C  E31F              jcxz 0x6ad
0000068E  0000              add [bx+si],al
00000690  FD                std
00000691  1F                pop ds
00000692  0000              add [bx+si],al
00000694  17                pop ss
00000695  2000              and [bx+si],al
00000697  0031              add [bx+di],dh
00000699  2000              and [bx+si],al
0000069B  004B20            add [bp+di+0x20],cl
0000069E  0000              add [bx+si],al
000006A0  652000            and [gs:bx+si],al
000006A3  007F20            add [bx+0x20],bh
000006A6  0000              add [bx+si],al
000006A8  99                cwd
000006A9  2000              and [bx+si],al
000006AB  00B32000          add [bp+di+0x20],dh
000006AF  00CD              add ch,cl
000006B1  2000              and [bx+si],al
000006B3  00E7              add bh,ah
000006B5  2000              and [bx+si],al
000006B7  0001              add [bx+di],al
000006B9  2100              and [bx+si],ax
000006BB  001B              add [bp+di],bl
000006BD  2100              and [bx+si],ax
000006BF  0035              add [di],dh
000006C1  2100              and [bx+si],ax
000006C3  004F21            add [bx+0x21],cl
000006C6  0000              add [bx+si],al
000006C8  69210000          imul sp,[bx+di],0x0
000006CC  832100            and word [bx+di],0x0
000006CF  009D2100          add [di+0x21],bl
000006D3  00B72100          add [bx+0x21],dh
000006D7  00D1              add cl,dl
000006D9  2100              and [bx+si],ax
000006DB  00EB              add bl,ch
000006DD  2100              and [bx+si],ax
000006DF  0005              add [di],al
000006E1  2200              and al,[bx+si]
000006E3  001F              add [bx],bl
000006E5  2200              and al,[bx+si]
000006E7  0039              add [bx+di],bh
000006E9  2200              and al,[bx+si]
000006EB  005322            add [bp+di+0x22],dl
000006EE  0000              add [bx+si],al
000006F0  6D                insw
000006F1  2200              and al,[bx+si]
000006F3  00872200          add [bx+0x22],al
000006F7  00A12200          add [bx+di+0x22],ah
000006FB  00BB2200          add [bp+di+0x22],bh
000006FF  00D5              add ch,dl
00000701  2200              and al,[bx+si]
00000703  00EF              add bh,ch
00000705  2200              and al,[bx+si]
00000707  0009              add [bx+di],cl
00000709  2300              and ax,[bx+si]
0000070B  0023              add [bp+di],ah
0000070D  2300              and ax,[bx+si]
0000070F  003D              add [di],bh
00000711  2300              and ax,[bx+si]
00000713  005723            add [bx+0x23],dl
00000716  0000              add [bx+si],al
00000718  7123              jno 0x73d
0000071A  0000              add [bx+si],al
0000071C  8B23              mov sp,[bp+di]
0000071E  0000              add [bx+si],al
00000720  A5                movsw
00000721  2300              and ax,[bx+si]
00000723  00BF2300          add [bx+0x23],bh
00000727  00D9              add cl,bl
00000729  2300              and ax,[bx+si]
0000072B  00F3              add bl,dh
0000072D  2300              and ax,[bx+si]
0000072F  000D              add [di],cl
00000731  2400              and al,0x0
00000733  0027              add [bx],ah
00000735  2400              and al,0x0
00000737  004124            add [bx+di+0x24],al
0000073A  0000              add [bx+si],al
0000073C  5B                pop bx
0000073D  2400              and al,0x0
0000073F  007524            add [di+0x24],dh
00000742  0000              add [bx+si],al
00000744  8F                db 0x8f
00000745  2400              and al,0x0
00000747  00A92400          add [bx+di+0x24],ch
0000074B  00C3              add bl,al
0000074D  2400              and al,0x0
0000074F  00DD              add ch,bl
00000751  2400              and al,0x0
00000753  00F7              add bh,dh
00000755  2400              and al,0x0
00000757  0011              add [bx+di],dl
00000759  250000            and ax,0x0
0000075C  2B25              sub sp,[di]
0000075E  0000              add [bx+si],al
00000760  45                inc bp
00000761  250000            and ax,0x0
00000764  5F                pop di
00000765  250000            and ax,0x0
00000768  69250000          imul sp,[di],0x0
0000076C  7225              jc 0x793
0000076E  0000              add [bx+si],al
00000770  7725              ja 0x797
00000772  0000              add [bx+si],al
00000774  7F25              jg 0x79b
00000776  0000              add [bx+si],al
00000778  2027              and [bx],ah
0000077A  0000              add [bx+si],al
0000077C  4D                dec bp
0000077D  2E0000            add [cs:bx+si],al
00000780  88C0              mov al,al
00000782  0000              add [bx+si],al
00000784  92                xchg ax,dx
00000785  3E0000            add [ds:bx+si],al
00000788  8CC0              mov ax,es
0000078A  0000              add [bx+si],al
0000078C  90                nop
0000078D  C00000            rol byte [bx+si],byte 0x0
00000790  237F00            and di,[bx+0x0]
00000793  00A47F00          add [si+0x7f],ah
00000797  00A97F00          add [bx+di+0x7f],ch
0000079B  00B47F00          add [si+0x7f],dh
0000079F  00BA7F00          add [bp+si+0x7f],bh
000007A3  00C9              add cl,cl
000007A5  7F00              jg 0x7a7
000007A7  00E9              add cl,ch
000007A9  800000            add byte [bx+si],0x0
000007AC  FD                std
000007AD  800000            add byte [bx+si],0x0
000007B0  DAC0              fcmovb st0
000007B2  0000              add [bx+si],al
000007B4  70C1              jo 0x777
000007B6  0000              add [bx+si],al
000007B8  94                xchg ax,sp
000007B9  C00000            rol byte [bx+si],byte 0x0
000007BC  91                xchg ax,cx
000007BD  7D00              jnl 0x7bf
000007BF  00997D00          add [bx+di+0x7d],bl
000007C3  00A77D00          add [bx+0x7d],ah
000007C7  00C0              add al,al
000007C9  7D00              jnl 0x7cb
000007CB  00D0              add al,dl
000007CD  4D                dec bp
000007CE  0000              add [bx+si],al
000007D0  D54D              aad byte 0x4d
000007D2  0000              add [bx+si],al
000007D4  1A4E00            sbb cl,[bp+0x0]
000007D7  0038              add [bx+si],bh
000007D9  4E                dec si
000007DA  0000              add [bx+si],al
000007DC  C54E00            lds cx,word [bp+0x0]
000007DF  0027              add [bx],ah
000007E1  50                push ax
000007E2  0000              add [bx+si],al
000007E4  315000            xor [bx+si+0x0],dx
000007E7  005950            add [bx+di+0x50],bl
000007EA  0000              add [bx+si],al
000007EC  688100            push word 0x81
000007EF  0098C000          add [bx+si+0xc0],bl
000007F3  002C              add [si],ch
000007F5  51                push cx
000007F6  0000              add [bx+si],al
000007F8  DB5700            fist dword [bx+0x0]
000007FB  0004              add [si],al
000007FD  58                pop ax
000007FE  0000              add [bx+si],al
00000800  2D5800            sub ax,0x58
00000803  00C6              add dh,al
00000805  C00000            rol byte [bx+si],byte 0x0
00000808  99                cwd
00000809  5B                pop bx
0000080A  0000              add [bx+si],al
0000080C  B55B              mov ch,0x5b
0000080E  0000              add [bx+si],al
00000810  C25B00            ret word 0x5b
00000813  00D7              add bh,dl
00000815  5B                pop bx
00000816  0000              add [bx+si],al
00000818  98                cbw
00000819  8800              mov [bx+si],al
0000081B  0000              add [bx+si],al
0000081D  8A00              mov al,[bx+si]
0000081F  00B88D00          add [bx+si+0x8d],bh
00000823  002F              add [bx],ch
00000825  8F00              pop word [bx+si]
00000827  0011              add [bx+di],dl
00000829  94                xchg ax,sp
0000082A  0000              add [bx+si],al
0000082C  53                push bx
0000082D  94                xchg ax,sp
0000082E  0000              add [bx+si],al
00000830  9A81000022        call word 0x2200:word 0x81
00000835  650000            add [gs:bx+si],al
00000838  AA                stosb
00000839  81000065          add word [bx+si],0x6500
0000083D  650000            add [gs:bx+si],al
00000840  9C                pushf
00000841  C00000            rol byte [bx+si],byte 0x0
00000844  3A6F00            cmp ch,[bx+0x0]
00000847  00446F            add [si+0x6f],al
0000084A  0000              add [bx+si],al
0000084C  49                dec cx
0000084D  6F                outsw
0000084E  0000              add [bx+si],al
00000850  A0C000            mov al,[0xc0]
00000853  006D6F            add [di+0x6f],ch
00000856  0000              add [bx+si],al
00000858  D36F00            shr word [bx+0x0],cl
0000085B  0021              add [bx+di],ah
0000085D  7000              jo 0x85f
0000085F  000C              add [si],cl
00000861  7200              jc 0x863
00000863  000A              add [bp+si],cl
00000865  7400              jz 0x867
00000867  0011              add [bx+di],dl
00000869  7400              jz 0x86b
0000086B  001F              add [bx],bl
0000086D  7400              jz 0x86f
0000086F  007774            add [bx+0x74],dh
00000872  0000              add [bx+si],al
00000874  867400            xchg dh,[si+0x0]
00000877  008D7400          add [di+0x74],cl
0000087B  00CE              add dh,cl
0000087D  7400              jz 0x87f
0000087F  00D7              add bh,dl
00000881  7400              jz 0x883
00000883  00F9              add cl,bh
00000885  7500              jnz 0x887
00000887  00A4C000          add [si+0xc0],ah
0000088B  00E0              add al,ah
0000088D  7700              ja 0x88f
0000088F  00A8C000          add [bx+si+0xc0],ch
00000893  00607B            add [bx+si+0x7b],ah
00000896  0000              add [bx+si],al
00000898  A4                movsb
00000899  7B00              jpo 0x89b
0000089B  00F0              add al,dh
0000089D  7C00              jl 0x89f
0000089F  00F6              add dh,dh
000008A1  7C00              jl 0x8a3
000008A3  00BAC000          add [bp+si+0xc0],bh
000008A7  00BEC000          add [bp+0xc0],bh
000008AB  00C2              add dl,al
000008AD  C00000            rol byte [bx+si],byte 0x0
000008B0  99                cwd
000008B1  82                db 0x82
000008B2  0000              add [bx+si],al
000008B4  A7                cmpsw
000008B5  82                db 0x82
000008B6  0000              add [bx+si],al
000008B8  0000              add [bx+si],al
000008BA  0000              add [bx+si],al
000008BC  0000              add [bx+si],al
000008BE  0000              add [bx+si],al
000008C0  0000              add [bx+si],al
000008C2  0000              add [bx+si],al
000008C4  0000              add [bx+si],al
000008C6  0000              add [bx+si],al
000008C8  0000              add [bx+si],al
000008CA  0000              add [bx+si],al
000008CC  0000              add [bx+si],al
000008CE  0000              add [bx+si],al
000008D0  0000              add [bx+si],al
000008D2  0000              add [bx+si],al
000008D4  0000              add [bx+si],al
000008D6  0000              add [bx+si],al
000008D8  0000              add [bx+si],al
000008DA  0000              add [bx+si],al
000008DC  0000              add [bx+si],al
000008DE  0000              add [bx+si],al
000008E0  0000              add [bx+si],al
000008E2  0000              add [bx+si],al
000008E4  0000              add [bx+si],al
000008E6  0000              add [bx+si],al
000008E8  0000              add [bx+si],al
000008EA  0000              add [bx+si],al
000008EC  0000              add [bx+si],al
000008EE  0000              add [bx+si],al
000008F0  0000              add [bx+si],al
000008F2  0000              add [bx+si],al
000008F4  0000              add [bx+si],al
000008F6  0000              add [bx+si],al
000008F8  0000              add [bx+si],al
000008FA  0000              add [bx+si],al
000008FC  0000              add [bx+si],al
000008FE  0000              add [bx+si],al
00000900  0000              add [bx+si],al
00000902  0000              add [bx+si],al
00000904  0000              add [bx+si],al
00000906  0000              add [bx+si],al
00000908  0000              add [bx+si],al
0000090A  0000              add [bx+si],al
0000090C  0000              add [bx+si],al
0000090E  0000              add [bx+si],al
00000910  0000              add [bx+si],al
00000912  0000              add [bx+si],al
00000914  0000              add [bx+si],al
00000916  0000              add [bx+si],al
00000918  0000              add [bx+si],al
0000091A  0000              add [bx+si],al
0000091C  0000              add [bx+si],al
0000091E  0000              add [bx+si],al
00000920  0000              add [bx+si],al
00000922  0000              add [bx+si],al
00000924  0000              add [bx+si],al
00000926  0000              add [bx+si],al
00000928  0000              add [bx+si],al
0000092A  0000              add [bx+si],al
0000092C  0000              add [bx+si],al
0000092E  0000              add [bx+si],al
00000930  0000              add [bx+si],al
00000932  0000              add [bx+si],al
00000934  0000              add [bx+si],al
00000936  0000              add [bx+si],al
00000938  0000              add [bx+si],al
0000093A  0000              add [bx+si],al
0000093C  0000              add [bx+si],al
0000093E  0000              add [bx+si],al
00000940  0000              add [bx+si],al
00000942  0000              add [bx+si],al
00000944  0000              add [bx+si],al
00000946  0000              add [bx+si],al
00000948  0000              add [bx+si],al
0000094A  0000              add [bx+si],al
0000094C  0000              add [bx+si],al
0000094E  0000              add [bx+si],al
00000950  0000              add [bx+si],al
00000952  0000              add [bx+si],al
00000954  0000              add [bx+si],al
00000956  0000              add [bx+si],al
00000958  0000              add [bx+si],al
0000095A  0000              add [bx+si],al
0000095C  0000              add [bx+si],al
0000095E  0000              add [bx+si],al
00000960  0000              add [bx+si],al
00000962  0000              add [bx+si],al
00000964  0000              add [bx+si],al
00000966  0000              add [bx+si],al
00000968  0000              add [bx+si],al
0000096A  0000              add [bx+si],al
0000096C  0000              add [bx+si],al
0000096E  0000              add [bx+si],al
00000970  0000              add [bx+si],al
00000972  0000              add [bx+si],al
00000974  0000              add [bx+si],al
00000976  0000              add [bx+si],al
00000978  0000              add [bx+si],al
0000097A  0000              add [bx+si],al
0000097C  0000              add [bx+si],al
0000097E  0000              add [bx+si],al
00000980  0000              add [bx+si],al
00000982  0000              add [bx+si],al
00000984  0000              add [bx+si],al
00000986  0000              add [bx+si],al
00000988  0000              add [bx+si],al
0000098A  0000              add [bx+si],al
0000098C  0000              add [bx+si],al
0000098E  0000              add [bx+si],al
00000990  0000              add [bx+si],al
00000992  0000              add [bx+si],al
00000994  0000              add [bx+si],al
00000996  0000              add [bx+si],al
00000998  0000              add [bx+si],al
0000099A  0000              add [bx+si],al
0000099C  0000              add [bx+si],al
0000099E  0000              add [bx+si],al
000009A0  0000              add [bx+si],al
000009A2  0000              add [bx+si],al
000009A4  0000              add [bx+si],al
000009A6  0000              add [bx+si],al
000009A8  0000              add [bx+si],al
000009AA  0000              add [bx+si],al
000009AC  0000              add [bx+si],al
000009AE  0000              add [bx+si],al
000009B0  0000              add [bx+si],al
000009B2  0000              add [bx+si],al
000009B4  0000              add [bx+si],al
000009B6  0000              add [bx+si],al
000009B8  0000              add [bx+si],al
000009BA  0000              add [bx+si],al
000009BC  0000              add [bx+si],al
000009BE  0000              add [bx+si],al
000009C0  0000              add [bx+si],al
000009C2  0000              add [bx+si],al
000009C4  0000              add [bx+si],al
000009C6  0000              add [bx+si],al
000009C8  0000              add [bx+si],al
000009CA  0000              add [bx+si],al
000009CC  0000              add [bx+si],al
000009CE  0000              add [bx+si],al
000009D0  0000              add [bx+si],al
000009D2  0000              add [bx+si],al
000009D4  0000              add [bx+si],al
000009D6  0000              add [bx+si],al
000009D8  0000              add [bx+si],al
000009DA  0000              add [bx+si],al
000009DC  0000              add [bx+si],al
000009DE  0000              add [bx+si],al
000009E0  0000              add [bx+si],al
000009E2  0000              add [bx+si],al
000009E4  0000              add [bx+si],al
000009E6  0000              add [bx+si],al
000009E8  0000              add [bx+si],al
000009EA  0000              add [bx+si],al
000009EC  0000              add [bx+si],al
000009EE  0000              add [bx+si],al
000009F0  0000              add [bx+si],al
000009F2  0000              add [bx+si],al
000009F4  0000              add [bx+si],al
000009F6  0000              add [bx+si],al
000009F8  0000              add [bx+si],al
000009FA  0000              add [bx+si],al
000009FC  0000              add [bx+si],al
000009FE  0000              add [bx+si],al
00000A00  626C53            bound bp,[si+0x53]
00000A03  55                push bp
00000A04  50                push ax
00000A05  43                inc bx
00000A06  48                dec ax
00000A07  54                push sp
00000A08  3120              xor [bx+si],sp
00000A0A  0000              add [bx+si],al
00000A0C  E205              loop 0xa13
00000A0E  4E                dec si
00000A0F  005A00            add [bp+si+0x0],bl
00000A12  0000              add [bx+si],al
00000A14  0000              add [bx+si],al
00000A16  0000              add [bx+si],al
00000A18  3000              xor [bx+si],al
00000A1A  1E                push ds
00000A1B  0000              add [bx+si],al
00000A1D  0000              add [bx+si],al
00000A1F  006000            add [bx+si+0x0],ah
00000A22  0000              add [bx+si],al
00000A24  0000              add [bx+si],al
00000A26  0000              add [bx+si],al
00000A28  0000              add [bx+si],al
00000A2A  0000              add [bx+si],al
00000A2C  0002              add [bp+si],al
00000A2E  8010B8            adc byte [bx+si],0xb8
00000A31  FFFF              udw
00000A33  50                push ax
00000A34  9A89505802        call word 0x258:word 0x5089
00000A39  9A55000000        call word 0x0:word 0x55
00000A3E  9A74040000        call word 0x0:word 0x474
00000A43  9A56110000        call word 0x0:word 0x1156
00000A48  9A6B050000        call word 0x0:word 0x56b
00000A4D  9A23365802        call word 0x258:word 0x3623
00000A52  E98803            jmp 0xddd
00000A55  B90400            mov cx,0x4
00000A58  9ABB4F5802        call word 0x258:word 0x4fbb
00000A5D  B80100            mov ax,0x1
00000A60  50                push ax
00000A61  B80F00            mov ax,0xf
00000A64  50                push ax
00000A65  B80100            mov ax,0x1
00000A68  50                push ax
00000A69  50                push ax
00000A6A  B80400            mov ax,0x4
00000A6D  50                push ax
00000A6E  9AEA4F5802        call word 0x258:word 0x4fea
00000A73  B8FFFF            mov ax,0xffff
00000A76  50                push ax
00000A77  9A89505802        call word 0x258:word 0x5089
00000A7C  B80100            mov ax,0x1
00000A7F  50                push ax
00000A80  50                push ax
00000A81  50                push ax
00000A82  B81700            mov ax,0x17
00000A85  50                push ax
00000A86  B80400            mov ax,0x4
00000A89  50                push ax
00000A8A  9A16505802        call word 0x258:word 0x5016
00000A8F  B86000            mov ax,0x60
00000A92  50                push ax
00000A93  9A1D415802        call word 0x258:word 0x411d
00000A98  B80100            mov ax,0x1
00000A9B  50                push ax
00000A9C  B80300            mov ax,0x3
00000A9F  50                push ax
00000AA0  B80100            mov ax,0x1
00000AA3  50                push ax
00000AA4  B80A00            mov ax,0xa
00000AA7  50                push ax
00000AA8  B80400            mov ax,0x4
00000AAB  50                push ax
00000AAC  9A16505802        call word 0x258:word 0x5016
00000AB1  B80100            mov ax,0x1
00000AB4  50                push ax
00000AB5  B80E00            mov ax,0xe
00000AB8  50                push ax
00000AB9  B80100            mov ax,0x1
00000ABC  50                push ax
00000ABD  50                push ax
00000ABE  B80400            mov ax,0x4
00000AC1  50                push ax
00000AC2  9AEA4F5802        call word 0x258:word 0x4fea
00000AC7  B88800            mov ax,0x88
00000ACA  50                push ax
00000ACB  9A22415802        call word 0x258:word 0x4122
00000AD0  B80100            mov ax,0x1
00000AD3  50                push ax
00000AD4  B80700            mov ax,0x7
00000AD7  50                push ax
00000AD8  B80100            mov ax,0x1
00000ADB  50                push ax
00000ADC  50                push ax
00000ADD  B80400            mov ax,0x4
00000AE0  50                push ax
00000AE1  9AEA4F5802        call word 0x258:word 0x4fea
00000AE6  B80100            mov ax,0x1
00000AE9  50                push ax
00000AEA  B80400            mov ax,0x4
00000AED  50                push ax
00000AEE  B80100            mov ax,0x1
00000AF1  50                push ax
00000AF2  B81000            mov ax,0x10
00000AF5  50                push ax
00000AF6  B80400            mov ax,0x4
00000AF9  50                push ax
00000AFA  9A16505802        call word 0x258:word 0x5016
00000AFF  B8CC00            mov ax,0xcc
00000B02  50                push ax
00000B03  9A22415802        call word 0x258:word 0x4122
00000B08  B80100            mov ax,0x1
00000B0B  50                push ax
00000B0C  B80600            mov ax,0x6
00000B0F  50                push ax
00000B10  B80100            mov ax,0x1
00000B13  50                push ax
00000B14  B80300            mov ax,0x3
00000B17  50                push ax
00000B18  B80400            mov ax,0x4
00000B1B  50                push ax
00000B1C  9A16505802        call word 0x258:word 0x5016
00000B21  B80100            mov ax,0x1
00000B24  50                push ax
00000B25  B80B00            mov ax,0xb
00000B28  50                push ax
00000B29  B80100            mov ax,0x1
00000B2C  50                push ax
00000B2D  50                push ax
00000B2E  B80400            mov ax,0x4
00000B31  50                push ax
00000B32  9AEA4F5802        call word 0x258:word 0x4fea
00000B37  B80201            mov ax,0x102
00000B3A  50                push ax
00000B3B  9A1D415802        call word 0x258:word 0x411d
00000B40  B82200            mov ax,0x22
00000B43  50                push ax
00000B44  9ABF4B5802        call word 0x258:word 0x4bbf
00000B49  50                push ax
00000B4A  9A1D415802        call word 0x258:word 0x411d
00000B4F  B82001            mov ax,0x120
00000B52  50                push ax
00000B53  9A1D415802        call word 0x258:word 0x411d
00000B58  B82200            mov ax,0x22
00000B5B  50                push ax
00000B5C  9ABF4B5802        call word 0x258:word 0x4bbf
00000B61  50                push ax
00000B62  9A1D415802        call word 0x258:word 0x411d
00000B67  B82A01            mov ax,0x12a
00000B6A  50                push ax
00000B6B  9A22415802        call word 0x258:word 0x4122
00000B70  B80100            mov ax,0x1
00000B73  50                push ax
00000B74  B80700            mov ax,0x7
00000B77  50                push ax
00000B78  B80100            mov ax,0x1
00000B7B  50                push ax
00000B7C  B80300            mov ax,0x3
00000B7F  50                push ax
00000B80  B80400            mov ax,0x4
00000B83  50                push ax
00000B84  9A16505802        call word 0x258:word 0x5016
00000B89  B85601            mov ax,0x156
00000B8C  50                push ax
00000B8D  9A22415802        call word 0x258:word 0x4122
00000B92  B80100            mov ax,0x1
00000B95  50                push ax
00000B96  B80800            mov ax,0x8
00000B99  50                push ax
00000B9A  B80100            mov ax,0x1
00000B9D  50                push ax
00000B9E  B80300            mov ax,0x3
00000BA1  50                push ax
00000BA2  B80400            mov ax,0x4
00000BA5  50                push ax
00000BA6  9A16505802        call word 0x258:word 0x5016
00000BAB  B8A401            mov ax,0x1a4
00000BAE  50                push ax
00000BAF  9A22415802        call word 0x258:word 0x4122
00000BB4  B80100            mov ax,0x1
00000BB7  50                push ax
00000BB8  B80700            mov ax,0x7
00000BBB  50                push ax
00000BBC  B80100            mov ax,0x1
00000BBF  50                push ax
00000BC0  50                push ax
00000BC1  B80400            mov ax,0x4
00000BC4  50                push ax
00000BC5  9AEA4F5802        call word 0x258:word 0x4fea
00000BCA  B80100            mov ax,0x1
00000BCD  50                push ax
00000BCE  B80A00            mov ax,0xa
00000BD1  50                push ax
00000BD2  B80100            mov ax,0x1
00000BD5  50                push ax
00000BD6  B80500            mov ax,0x5
00000BD9  50                push ax
00000BDA  B80400            mov ax,0x4
00000BDD  50                push ax
00000BDE  9A16505802        call word 0x258:word 0x5016
00000BE3  B8C001            mov ax,0x1c0
00000BE6  50                push ax
00000BE7  9A22415802        call word 0x258:word 0x4122
00000BEC  B80100            mov ax,0x1
00000BEF  50                push ax
00000BF0  B80B00            mov ax,0xb
00000BF3  50                push ax
00000BF4  B80100            mov ax,0x1
00000BF7  50                push ax
00000BF8  B80500            mov ax,0x5
00000BFB  50                push ax
00000BFC  B80400            mov ax,0x4
00000BFF  50                push ax
00000C00  9A16505802        call word 0x258:word 0x5016
00000C05  B8EC01            mov ax,0x1ec
00000C08  50                push ax
00000C09  9A22415802        call word 0x258:word 0x4122
00000C0E  B80100            mov ax,0x1
00000C11  50                push ax
00000C12  B80C00            mov ax,0xc
00000C15  50                push ax
00000C16  B80100            mov ax,0x1
00000C19  50                push ax
00000C1A  B80500            mov ax,0x5
00000C1D  50                push ax
00000C1E  B80400            mov ax,0x4
00000C21  50                push ax
00000C22  9A16505802        call word 0x258:word 0x5016
00000C27  B81802            mov ax,0x218
00000C2A  50                push ax
00000C2B  9A22415802        call word 0x258:word 0x4122
00000C30  B80100            mov ax,0x1
00000C33  50                push ax
00000C34  B80D00            mov ax,0xd
00000C37  50                push ax
00000C38  B80100            mov ax,0x1
00000C3B  50                push ax
00000C3C  B80500            mov ax,0x5
00000C3F  50                push ax
00000C40  B80400            mov ax,0x4
00000C43  50                push ax
00000C44  9A16505802        call word 0x258:word 0x5016
00000C49  B84A02            mov ax,0x24a
00000C4C  50                push ax
00000C4D  9A22415802        call word 0x258:word 0x4122
00000C52  B80100            mov ax,0x1
00000C55  50                push ax
00000C56  B80B00            mov ax,0xb
00000C59  50                push ax
00000C5A  B80100            mov ax,0x1
00000C5D  50                push ax
00000C5E  50                push ax
00000C5F  B80400            mov ax,0x4
00000C62  50                push ax
00000C63  9AEA4F5802        call word 0x258:word 0x4fea
00000C68  B80100            mov ax,0x1
00000C6B  50                push ax
00000C6C  B80F00            mov ax,0xf
00000C6F  50                push ax
00000C70  B80100            mov ax,0x1
00000C73  50                push ax
00000C74  B80500            mov ax,0x5
00000C77  50                push ax
00000C78  B80400            mov ax,0x4
00000C7B  50                push ax
00000C7C  9A16505802        call word 0x258:word 0x5016
00000C81  B87802            mov ax,0x278
00000C84  50                push ax
00000C85  9A22415802        call word 0x258:word 0x4122
00000C8A  B80100            mov ax,0x1
00000C8D  50                push ax
00000C8E  B80E00            mov ax,0xe
00000C91  50                push ax
00000C92  B80100            mov ax,0x1
00000C95  50                push ax
00000C96  50                push ax
00000C97  B80400            mov ax,0x4
00000C9A  50                push ax
00000C9B  9AEA4F5802        call word 0x258:word 0x4fea
00000CA0  B80100            mov ax,0x1
00000CA3  50                push ax
00000CA4  B81100            mov ax,0x11
00000CA7  50                push ax
00000CA8  B80100            mov ax,0x1
00000CAB  50                push ax
00000CAC  B80500            mov ax,0x5
00000CAF  50                push ax
00000CB0  B80400            mov ax,0x4
00000CB3  50                push ax
00000CB4  9A16505802        call word 0x258:word 0x5016
00000CB9  B8B002            mov ax,0x2b0
00000CBC  50                push ax
00000CBD  9A22415802        call word 0x258:word 0x4122
00000CC2  B80100            mov ax,0x1
00000CC5  50                push ax
00000CC6  B81200            mov ax,0x12
00000CC9  50                push ax
00000CCA  B80100            mov ax,0x1
00000CCD  50                push ax
00000CCE  B80500            mov ax,0x5
00000CD1  50                push ax
00000CD2  B80400            mov ax,0x4
00000CD5  50                push ax
00000CD6  9A16505802        call word 0x258:word 0x5016
00000CDB  B8C402            mov ax,0x2c4
00000CDE  50                push ax
00000CDF  9A22415802        call word 0x258:word 0x4122
00000CE4  B80100            mov ax,0x1
00000CE7  50                push ax
00000CE8  B81300            mov ax,0x13
00000CEB  50                push ax
00000CEC  B80100            mov ax,0x1
00000CEF  50                push ax
00000CF0  B80500            mov ax,0x5
00000CF3  50                push ax
00000CF4  B80400            mov ax,0x4
00000CF7  50                push ax
00000CF8  9A16505802        call word 0x258:word 0x5016
00000CFD  B8D802            mov ax,0x2d8
00000D00  50                push ax
00000D01  9A22415802        call word 0x258:word 0x4122
00000D06  B80100            mov ax,0x1
00000D09  50                push ax
00000D0A  B8FF7F            mov ax,0x7fff
00000D0D  50                push ax
00000D0E  9AD6515802        call word 0x258:word 0x51d6
00000D13  50                push ax
00000D14  B83000            mov ax,0x30
00000D17  50                push ax
00000D18  9A0A4B5802        call word 0x258:word 0x4b0a
00000D1D  B83000            mov ax,0x30
00000D20  50                push ax
00000D21  9A834D5802        call word 0x258:word 0x4d83
00000D26  50                push ax
00000D27  B83000            mov ax,0x30
00000D2A  50                push ax
00000D2B  9A0A4B5802        call word 0x258:word 0x4b0a
00000D30  B81B00            mov ax,0x1b
00000D33  50                push ax
00000D34  9ABF4B5802        call word 0x258:word 0x4bbf
00000D39  50                push ax
00000D3A  B83000            mov ax,0x30
00000D3D  50                push ax
00000D3E  9A804B5802        call word 0x258:word 0x4b80
00000D43  7403              jz 0xd48
00000D45  E90500            jmp 0xd4d
00000D48  9A23365802        call word 0x258:word 0x3623
00000D4D  B83000            mov ax,0x30
00000D50  50                push ax
00000D51  8D46F2            lea ax,[bp-0xe]
00000D54  50                push ax
00000D55  9A0A4B5802        call word 0x258:word 0x4b0a
00000D5A  8D46F2            lea ax,[bp-0xe]
00000D5D  50                push ax
00000D5E  B8F602            mov ax,0x2f6
00000D61  50                push ax
00000D62  9A804B5802        call word 0x258:word 0x4b80
00000D67  7403              jz 0xd6c
00000D69  E91300            jmp 0xd7f
00000D6C  B8FC02            mov ax,0x2fc
00000D6F  50                push ax
00000D70  B83400            mov ax,0x34
00000D73  50                push ax
00000D74  9A0A4B5802        call word 0x258:word 0x4b0a
00000D79  E85000            call 0xdcc
00000D7C  E94A00            jmp 0xdc9
00000D7F  8D46F2            lea ax,[bp-0xe]
00000D82  50                push ax
00000D83  B80A03            mov ax,0x30a
00000D86  50                push ax
00000D87  9A804B5802        call word 0x258:word 0x4b80
00000D8C  7403              jz 0xd91
00000D8E  E91300            jmp 0xda4
00000D91  B81003            mov ax,0x310
00000D94  50                push ax
00000D95  B83400            mov ax,0x34
00000D98  50                push ax
00000D99  9A0A4B5802        call word 0x258:word 0x4b0a
00000D9E  E82B00            call 0xdcc
00000DA1  E92500            jmp 0xdc9
00000DA4  8D46F2            lea ax,[bp-0xe]
00000DA7  50                push ax
00000DA8  B81E03            mov ax,0x31e
00000DAB  50                push ax
00000DAC  9A804B5802        call word 0x258:word 0x4b80
00000DB1  7403              jz 0xdb6
00000DB3  E91300            jmp 0xdc9
00000DB6  B82403            mov ax,0x324
00000DB9  50                push ax
00000DBA  B83400            mov ax,0x34
00000DBD  50                push ax
00000DBE  9A0A4B5802        call word 0x258:word 0x4b0a
00000DC3  E80600            call 0xdcc
00000DC6  E90000            jmp 0xdc9
00000DC9  E83AFF            call 0xd06
00000DCC  8D46F2            lea ax,[bp-0xe]
00000DCF  50                push ax
00000DD0  9A324F5802        call word 0x258:word 0x4f32
00000DD5  9A904F5802        call word 0x258:word 0x4f90
00000DDA  CA0000            retf word 0x0
00000DDD  E99100            jmp 0xe71
00000DE0  B90400            mov cx,0x4
00000DE3  9ABB4F5802        call word 0x258:word 0x4fbb
00000DE8  B83000            mov ax,0x30
00000DEB  50                push ax
00000DEC  8D46F2            lea ax,[bp-0xe]
00000DEF  50                push ax
00000DF0  9A0A4B5802        call word 0x258:word 0x4b0a
00000DF5  8D46F2            lea ax,[bp-0xe]
00000DF8  50                push ax
00000DF9  B8F602            mov ax,0x2f6
00000DFC  50                push ax
00000DFD  9A804B5802        call word 0x258:word 0x4b80
00000E02  7403              jz 0xe07
00000E04  E91000            jmp 0xe17
00000E07  B8FC02            mov ax,0x2fc
00000E0A  50                push ax
00000E0B  B83400            mov ax,0x34
00000E0E  50                push ax
00000E0F  9A0A4B5802        call word 0x258:word 0x4b0a
00000E14  E94400            jmp 0xe5b
00000E17  8D46F2            lea ax,[bp-0xe]
00000E1A  50                push ax
00000E1B  B80A03            mov ax,0x30a
00000E1E  50                push ax
00000E1F  9A804B5802        call word 0x258:word 0x4b80
00000E24  7403              jz 0xe29
00000E26  E91000            jmp 0xe39
00000E29  B81003            mov ax,0x310
00000E2C  50                push ax
00000E2D  B83400            mov ax,0x34
00000E30  50                push ax
00000E31  9A0A4B5802        call word 0x258:word 0x4b0a
00000E36  E92200            jmp 0xe5b
00000E39  8D46F2            lea ax,[bp-0xe]
00000E3C  50                push ax
00000E3D  B81E03            mov ax,0x31e
00000E40  50                push ax
00000E41  9A804B5802        call word 0x258:word 0x4b80
00000E46  7403              jz 0xe4b
00000E48  E91000            jmp 0xe5b
00000E4B  B83C03            mov ax,0x33c
00000E4E  50                push ax
00000E4F  B83400            mov ax,0x34
00000E52  50                push ax
00000E53  9A0A4B5802        call word 0x258:word 0x4b0a
00000E58  E90000            jmp 0xe5b
00000E5B  9A55000000        call word 0x0:word 0x55
00000E60  8D46F2            lea ax,[bp-0xe]
00000E63  50                push ax
00000E64  9A324F5802        call word 0x258:word 0x4f32
00000E69  9A904F5802        call word 0x258:word 0x4f90
00000E6E  CA0000            retf word 0x0
00000E71  E9F400            jmp 0xf68
00000E74  B90000            mov cx,0x0
00000E77  9ABB4F5802        call word 0x258:word 0x4fbb
00000E7C  B80100            mov ax,0x1
00000E7F  50                push ax
00000E80  B80F00            mov ax,0xf
00000E83  50                push ax
00000E84  B80100            mov ax,0x1
00000E87  50                push ax
00000E88  50                push ax
00000E89  B80400            mov ax,0x4
00000E8C  50                push ax
00000E8D  9AEA4F5802        call word 0x258:word 0x4fea
00000E92  B8FFFF            mov ax,0xffff
00000E95  50                push ax
00000E96  9A89505802        call word 0x258:word 0x5089
00000E9B  B80100            mov ax,0x1
00000E9E  50                push ax
00000E9F  B80200            mov ax,0x2
00000EA2  50                push ax
00000EA3  B80100            mov ax,0x1
00000EA6  50                push ax
00000EA7  B81900            mov ax,0x19
00000EAA  50                push ax
00000EAB  B80400            mov ax,0x4
00000EAE  50                push ax
00000EAF  9A16505802        call word 0x258:word 0x5016
00000EB4  B85403            mov ax,0x354
00000EB7  50                push ax
00000EB8  9A22415802        call word 0x258:word 0x4122
00000EBD  B80100            mov ax,0x1
00000EC0  50                push ax
00000EC1  B80E00            mov ax,0xe
00000EC4  50                push ax
00000EC5  B80100            mov ax,0x1
00000EC8  50                push ax
00000EC9  50                push ax
00000ECA  B80400            mov ax,0x4
00000ECD  50                push ax
00000ECE  9AEA4F5802        call word 0x258:word 0x4fea
00000ED3  B80100            mov ax,0x1
00000ED6  50                push ax
00000ED7  B80500            mov ax,0x5
00000EDA  50                push ax
00000EDB  B80100            mov ax,0x1
00000EDE  50                push ax
00000EDF  B81600            mov ax,0x16
00000EE2  50                push ax
00000EE3  B80400            mov ax,0x4
00000EE6  50                push ax
00000EE7  9A16505802        call word 0x258:word 0x5016
00000EEC  B87803            mov ax,0x378
00000EEF  50                push ax
00000EF0  9A1D415802        call word 0x258:word 0x411d
00000EF5  B83400            mov ax,0x34
00000EF8  50                push ax
00000EF9  9A1D415802        call word 0x258:word 0x411d
00000EFE  B88E03            mov ax,0x38e
00000F01  50                push ax
00000F02  9A22415802        call word 0x258:word 0x4122
00000F07  B80100            mov ax,0x1
00000F0A  50                push ax
00000F0B  B80B00            mov ax,0xb
00000F0E  50                push ax
00000F0F  B80100            mov ax,0x1
00000F12  50                push ax
00000F13  50                push ax
00000F14  B80400            mov ax,0x4
00000F17  50                push ax
00000F18  9AEA4F5802        call word 0x258:word 0x4fea
00000F1D  B80100            mov ax,0x1
00000F20  50                push ax
00000F21  B80900            mov ax,0x9
00000F24  50                push ax
00000F25  B80100            mov ax,0x1
00000F28  50                push ax
00000F29  B80600            mov ax,0x6
00000F2C  50                push ax
00000F2D  B80400            mov ax,0x4
00000F30  50                push ax
00000F31  9A16505802        call word 0x258:word 0x5016
00000F36  B89C03            mov ax,0x39c
00000F39  50                push ax
00000F3A  E80000            call 0xf3d
00000F3D  58                pop ax
00000F3E  050D00            add ax,0xd
00000F41  0E                push cs
00000F42  50                push ax
00000F43  9A64485802        call word 0x258:word 0x4864
00000F48  EB04              jmp 0xf4e
00000F4A  0200              add al,[bx+si]
00000F4C  0103              add [bp+di],ax
00000F4E  B83800            mov ax,0x38
00000F51  1E                push ds
00000F52  50                push ax
00000F53  33C0              xor ax,ax
00000F55  50                push ax
00000F56  9A1A4A5802        call word 0x258:word 0x4a1a
00000F5B  9A56425802        call word 0x258:word 0x4256
00000F60  9A904F5802        call word 0x258:word 0x4f90
00000F65  CA0000            retf word 0x0
00000F68  E97609            jmp 0x18e1
00000F6B  B90800            mov cx,0x8
00000F6E  9ABB4F5802        call word 0x258:word 0x4fbb
00000F73  B80100            mov ax,0x1
00000F76  50                push ax
00000F77  B80E00            mov ax,0xe
00000F7A  50                push ax
00000F7B  B80100            mov ax,0x1
00000F7E  50                push ax
00000F7F  50                push ax
00000F80  B80400            mov ax,0x4
00000F83  50                push ax
00000F84  9AEA4F5802        call word 0x258:word 0x4fea
00000F89  B8FFFF            mov ax,0xffff
00000F8C  50                push ax
00000F8D  9A89505802        call word 0x258:word 0x5089
00000F92  B80100            mov ax,0x1
00000F95  50                push ax
00000F96  B80200            mov ax,0x2
00000F99  50                push ax
00000F9A  B80100            mov ax,0x1
00000F9D  50                push ax
00000F9E  B81900            mov ax,0x19
00000FA1  50                push ax
00000FA2  B80400            mov ax,0x4
00000FA5  50                push ax
00000FA6  9A16505802        call word 0x258:word 0x5016
00000FAB  B85403            mov ax,0x354
00000FAE  50                push ax
00000FAF  9A22415802        call word 0x258:word 0x4122
00000FB4  B80100            mov ax,0x1
00000FB7  50                push ax
00000FB8  B80700            mov ax,0x7
00000FBB  50                push ax
00000FBC  B80100            mov ax,0x1
00000FBF  50                push ax
00000FC0  50                push ax
00000FC1  B80400            mov ax,0x4
00000FC4  50                push ax
00000FC5  9AEA4F5802        call word 0x258:word 0x4fea
00000FCA  B80100            mov ax,0x1
00000FCD  50                push ax
00000FCE  B80300            mov ax,0x3
00000FD1  50                push ax
00000FD2  B80100            mov ax,0x1
00000FD5  50                push ax
00000FD6  B81900            mov ax,0x19
00000FD9  50                push ax
00000FDA  B80400            mov ax,0x4
00000FDD  50                push ax
00000FDE  9A16505802        call word 0x258:word 0x5016
00000FE3  B8C003            mov ax,0x3c0
00000FE6  50                push ax
00000FE7  9A1D415802        call word 0x258:word 0x411d
00000FEC  B83800            mov ax,0x38
00000FEF  50                push ax
00000FF0  9A22415802        call word 0x258:word 0x4122
00000FF5  B80100            mov ax,0x1
00000FF8  50                push ax
00000FF9  B80E00            mov ax,0xe
00000FFC  50                push ax
00000FFD  B80100            mov ax,0x1
00001000  50                push ax
00001001  50                push ax
00001002  B80400            mov ax,0x4
00001005  50                push ax
00001006  9AEA4F5802        call word 0x258:word 0x4fea
0000100B  B80100            mov ax,0x1
0000100E  50                push ax
0000100F  B80500            mov ax,0x5
00001012  50                push ax
00001013  B80100            mov ax,0x1
00001016  50                push ax
00001017  B82200            mov ax,0x22
0000101A  50                push ax
0000101B  B80400            mov ax,0x4
0000101E  50                push ax
0000101F  9A16505802        call word 0x258:word 0x5016
00001024  B8DE03            mov ax,0x3de
00001027  50                push ax
00001028  9A22415802        call word 0x258:word 0x4122
0000102D  B80100            mov ax,0x1
00001030  50                push ax
00001031  B80B00            mov ax,0xb
00001034  50                push ax
00001035  B80100            mov ax,0x1
00001038  50                push ax
00001039  50                push ax
0000103A  B80400            mov ax,0x4
0000103D  50                push ax
0000103E  9AEA4F5802        call word 0x258:word 0x4fea
00001043  B80100            mov ax,0x1
00001046  50                push ax
00001047  B80500            mov ax,0x5
0000104A  50                push ax
0000104B  B80100            mov ax,0x1
0000104E  50                push ax
0000104F  B82300            mov ax,0x23
00001052  50                push ax
00001053  B80400            mov ax,0x4
00001056  50                push ax
00001057  9A16505802        call word 0x258:word 0x5016
0000105C  B8E403            mov ax,0x3e4
0000105F  50                push ax
00001060  9A22415802        call word 0x258:word 0x4122
00001065  B80100            mov ax,0x1
00001068  50                push ax
00001069  B80500            mov ax,0x5
0000106C  50                push ax
0000106D  B80100            mov ax,0x1
00001070  50                push ax
00001071  B82100            mov ax,0x21
00001074  50                push ax
00001075  B80400            mov ax,0x4
00001078  50                push ax
00001079  9A16505802        call word 0x258:word 0x5016
0000107E  B8FA03            mov ax,0x3fa
00001081  50                push ax
00001082  9A22415802        call word 0x258:word 0x4122
00001087  B84A00            mov ax,0x4a
0000108A  50                push ax
0000108B  B80004            mov ax,0x400
0000108E  50                push ax
0000108F  9A804B5802        call word 0x258:word 0x4b80
00001094  7403              jz 0x1099
00001096  E90D00            jmp 0x10a6
00001099  B80404            mov ax,0x404
0000109C  50                push ax
0000109D  B84A00            mov ax,0x4a
000010A0  50                push ax
000010A1  9A0A4B5802        call word 0x258:word 0x4b0a
000010A6  B80100            mov ax,0x1
000010A9  50                push ax
000010AA  B80F00            mov ax,0xf
000010AD  50                push ax
000010AE  B80100            mov ax,0x1
000010B1  50                push ax
000010B2  50                push ax
000010B3  B80400            mov ax,0x4
000010B6  50                push ax
000010B7  9AEA4F5802        call word 0x258:word 0x4fea
000010BC  B80100            mov ax,0x1
000010BF  50                push ax
000010C0  B80500            mov ax,0x5
000010C3  50                push ax
000010C4  B80100            mov ax,0x1
000010C7  50                push ax
000010C8  B83600            mov ax,0x36
000010CB  50                push ax
000010CC  B80400            mov ax,0x4
000010CF  50                push ax
000010D0  9A16505802        call word 0x258:word 0x5016
000010D5  B84A00            mov ax,0x4a
000010D8  50                push ax
000010D9  9A22415802        call word 0x258:word 0x4122
000010DE  B80100            mov ax,0x1
000010E1  50                push ax
000010E2  B80B00            mov ax,0xb
000010E5  50                push ax
000010E6  B80100            mov ax,0x1
000010E9  50                push ax
000010EA  50                push ax
000010EB  B80400            mov ax,0x4
000010EE  50                push ax
000010EF  9AEA4F5802        call word 0x258:word 0x4fea
000010F4  B80100            mov ax,0x1
000010F7  50                push ax
000010F8  B80500            mov ax,0x5
000010FB  50                push ax
000010FC  B80100            mov ax,0x1
000010FF  50                push ax
00001100  B80A00            mov ax,0xa
00001103  50                push ax
00001104  B80400            mov ax,0x4
00001107  50                push ax
00001108  9A16505802        call word 0x258:word 0x5016
0000110D  B80A04            mov ax,0x40a
00001110  50                push ax
00001111  9A22415802        call word 0x258:word 0x4122
00001116  B80100            mov ax,0x1
00001119  50                push ax
0000111A  B80500            mov ax,0x5
0000111D  50                push ax
0000111E  B80100            mov ax,0x1
00001121  50                push ax
00001122  B80800            mov ax,0x8
00001125  50                push ax
00001126  B80400            mov ax,0x4
00001129  50                push ax
0000112A  9A16505802        call word 0x258:word 0x5016
0000112F  B8FA03            mov ax,0x3fa
00001132  50                push ax
00001133  9A22415802        call word 0x258:word 0x4122
00001138  B80100            mov ax,0x1
0000113B  50                push ax
0000113C  B80600            mov ax,0x6
0000113F  50                push ax
00001140  B80100            mov ax,0x1
00001143  50                push ax
00001144  B80A00            mov ax,0xa
00001147  50                push ax
00001148  B80400            mov ax,0x4
0000114B  50                push ax
0000114C  9A16505802        call word 0x258:word 0x5016
00001151  B81604            mov ax,0x416
00001154  50                push ax
00001155  9A22415802        call word 0x258:word 0x4122
0000115A  B80100            mov ax,0x1
0000115D  50                push ax
0000115E  B80600            mov ax,0x6
00001161  50                push ax
00001162  B80100            mov ax,0x1
00001165  50                push ax
00001166  B80800            mov ax,0x8
00001169  50                push ax
0000116A  B80400            mov ax,0x4
0000116D  50                push ax
0000116E  9A16505802        call word 0x258:word 0x5016
00001173  B8FA03            mov ax,0x3fa
00001176  50                push ax
00001177  9A22415802        call word 0x258:word 0x4122
0000117C  B80100            mov ax,0x1
0000117F  50                push ax
00001180  B80700            mov ax,0x7
00001183  50                push ax
00001184  B80100            mov ax,0x1
00001187  50                push ax
00001188  B80A00            mov ax,0xa
0000118B  50                push ax
0000118C  B80400            mov ax,0x4
0000118F  50                push ax
00001190  9A16505802        call word 0x258:word 0x5016
00001195  B82404            mov ax,0x424
00001198  50                push ax
00001199  9A22415802        call word 0x258:word 0x4122
0000119E  B80100            mov ax,0x1
000011A1  50                push ax
000011A2  B80700            mov ax,0x7
000011A5  50                push ax
000011A6  B80100            mov ax,0x1
000011A9  50                push ax
000011AA  B80800            mov ax,0x8
000011AD  50                push ax
000011AE  B80400            mov ax,0x4
000011B1  50                push ax
000011B2  9A16505802        call word 0x258:word 0x5016
000011B7  B8FA03            mov ax,0x3fa
000011BA  50                push ax
000011BB  9A22415802        call word 0x258:word 0x4122
000011C0  B80100            mov ax,0x1
000011C3  50                push ax
000011C4  B80800            mov ax,0x8
000011C7  50                push ax
000011C8  B80100            mov ax,0x1
000011CB  50                push ax
000011CC  B80A00            mov ax,0xa
000011CF  50                push ax
000011D0  B80400            mov ax,0x4
000011D3  50                push ax
000011D4  9A16505802        call word 0x258:word 0x5016
000011D9  B82E04            mov ax,0x42e
000011DC  50                push ax
000011DD  9A22415802        call word 0x258:word 0x4122
000011E2  B80100            mov ax,0x1
000011E5  50                push ax
000011E6  B80800            mov ax,0x8
000011E9  50                push ax
000011EA  B80100            mov ax,0x1
000011ED  50                push ax
000011EE  B80800            mov ax,0x8
000011F1  50                push ax
000011F2  B80400            mov ax,0x4
000011F5  50                push ax
000011F6  9A16505802        call word 0x258:word 0x5016
000011FB  B8FA03            mov ax,0x3fa
000011FE  50                push ax
000011FF  9A22415802        call word 0x258:word 0x4122
00001204  B80100            mov ax,0x1
00001207  50                push ax
00001208  B80900            mov ax,0x9
0000120B  50                push ax
0000120C  B80100            mov ax,0x1
0000120F  50                push ax
00001210  B80A00            mov ax,0xa
00001213  50                push ax
00001214  B80400            mov ax,0x4
00001217  50                push ax
00001218  9A16505802        call word 0x258:word 0x5016
0000121D  B83A04            mov ax,0x43a
00001220  50                push ax
00001221  9A22415802        call word 0x258:word 0x4122
00001226  B80100            mov ax,0x1
00001229  50                push ax
0000122A  B80900            mov ax,0x9
0000122D  50                push ax
0000122E  B80100            mov ax,0x1
00001231  50                push ax
00001232  B80800            mov ax,0x8
00001235  50                push ax
00001236  B80400            mov ax,0x4
00001239  50                push ax
0000123A  9A16505802        call word 0x258:word 0x5016
0000123F  B8FA03            mov ax,0x3fa
00001242  50                push ax
00001243  9A22415802        call word 0x258:word 0x4122
00001248  B80100            mov ax,0x1
0000124B  50                push ax
0000124C  B80A00            mov ax,0xa
0000124F  50                push ax
00001250  B80100            mov ax,0x1
00001253  50                push ax
00001254  B80A00            mov ax,0xa
00001257  50                push ax
00001258  B80400            mov ax,0x4
0000125B  50                push ax
0000125C  9A16505802        call word 0x258:word 0x5016
00001261  B84A04            mov ax,0x44a
00001264  50                push ax
00001265  9A22415802        call word 0x258:word 0x4122
0000126A  B80100            mov ax,0x1
0000126D  50                push ax
0000126E  B80A00            mov ax,0xa
00001271  50                push ax
00001272  B80100            mov ax,0x1
00001275  50                push ax
00001276  B80800            mov ax,0x8
00001279  50                push ax
0000127A  B80400            mov ax,0x4
0000127D  50                push ax
0000127E  9A16505802        call word 0x258:word 0x5016
00001283  B8FA03            mov ax,0x3fa
00001286  50                push ax
00001287  9A22415802        call word 0x258:word 0x4122
0000128C  B80100            mov ax,0x1
0000128F  50                push ax
00001290  B80E00            mov ax,0xe
00001293  50                push ax
00001294  B80100            mov ax,0x1
00001297  50                push ax
00001298  50                push ax
00001299  B80400            mov ax,0x4
0000129C  50                push ax
0000129D  9AEA4F5802        call word 0x258:word 0x4fea
000012A2  B80100            mov ax,0x1
000012A5  50                push ax
000012A6  B80500            mov ax,0x5
000012A9  50                push ax
000012AA  B80100            mov ax,0x1
000012AD  50                push ax
000012AE  B80900            mov ax,0x9
000012B1  50                push ax
000012B2  B80400            mov ax,0x4
000012B5  50                push ax
000012B6  9A16505802        call word 0x258:word 0x5016
000012BB  B8F602            mov ax,0x2f6
000012BE  50                push ax
000012BF  9A22415802        call word 0x258:word 0x4122
000012C4  B80100            mov ax,0x1
000012C7  50                push ax
000012C8  B80600            mov ax,0x6
000012CB  50                push ax
000012CC  B80100            mov ax,0x1
000012CF  50                push ax
000012D0  B80900            mov ax,0x9
000012D3  50                push ax
000012D4  B80400            mov ax,0x4
000012D7  50                push ax
000012D8  9A16505802        call word 0x258:word 0x5016
000012DD  B80A03            mov ax,0x30a
000012E0  50                push ax
000012E1  9A22415802        call word 0x258:word 0x4122
000012E6  B80100            mov ax,0x1
000012E9  50                push ax
000012EA  B80700            mov ax,0x7
000012ED  50                push ax
000012EE  B80100            mov ax,0x1
000012F1  50                push ax
000012F2  B80900            mov ax,0x9
000012F5  50                push ax
000012F6  B80400            mov ax,0x4
000012F9  50                push ax
000012FA  9A16505802        call word 0x258:word 0x5016
000012FF  B81E03            mov ax,0x31e
00001302  50                push ax
00001303  9A22415802        call word 0x258:word 0x4122
00001308  B80100            mov ax,0x1
0000130B  50                push ax
0000130C  B80800            mov ax,0x8
0000130F  50                push ax
00001310  B80100            mov ax,0x1
00001313  50                push ax
00001314  B80900            mov ax,0x9
00001317  50                push ax
00001318  B80400            mov ax,0x4
0000131B  50                push ax
0000131C  9A16505802        call word 0x258:word 0x5016
00001321  B85404            mov ax,0x454
00001324  50                push ax
00001325  9A22415802        call word 0x258:word 0x4122
0000132A  B80100            mov ax,0x1
0000132D  50                push ax
0000132E  B80900            mov ax,0x9
00001331  50                push ax
00001332  B80100            mov ax,0x1
00001335  50                push ax
00001336  B80900            mov ax,0x9
00001339  50                push ax
0000133A  B80400            mov ax,0x4
0000133D  50                push ax
0000133E  9A16505802        call word 0x258:word 0x5016
00001343  B85A04            mov ax,0x45a
00001346  50                push ax
00001347  9A22415802        call word 0x258:word 0x4122
0000134C  B80100            mov ax,0x1
0000134F  50                push ax
00001350  B80A00            mov ax,0xa
00001353  50                push ax
00001354  B80100            mov ax,0x1
00001357  50                push ax
00001358  B80900            mov ax,0x9
0000135B  50                push ax
0000135C  B80400            mov ax,0x4
0000135F  50                push ax
00001360  9A16505802        call word 0x258:word 0x5016
00001365  B86004            mov ax,0x460
00001368  50                push ax
00001369  9A22415802        call word 0x258:word 0x4122
0000136E  B80100            mov ax,0x1
00001371  50                push ax
00001372  B80700            mov ax,0x7
00001375  50                push ax
00001376  B80100            mov ax,0x1
00001379  50                push ax
0000137A  50                push ax
0000137B  B80400            mov ax,0x4
0000137E  50                push ax
0000137F  9AEA4F5802        call word 0x258:word 0x4fea
00001384  B80100            mov ax,0x1
00001387  50                push ax
00001388  B80500            mov ax,0x5
0000138B  50                push ax
0000138C  B80100            mov ax,0x1
0000138F  50                push ax
00001390  B81600            mov ax,0x16
00001393  50                push ax
00001394  B80400            mov ax,0x4
00001397  50                push ax
00001398  9A16505802        call word 0x258:word 0x5016
0000139D  FF363E00          push word [0x3e]
000013A1  FF363C00          push word [0x3c]
000013A5  9A13415802        call word 0x258:word 0x4113
000013AA  B80100            mov ax,0x1
000013AD  50                push ax
000013AE  B80600            mov ax,0x6
000013B1  50                push ax
000013B2  B80100            mov ax,0x1
000013B5  50                push ax
000013B6  B81600            mov ax,0x16
000013B9  50                push ax
000013BA  B80400            mov ax,0x4
000013BD  50                push ax
000013BE  9A16505802        call word 0x258:word 0x5016
000013C3  FF364000          push word [0x40]
000013C7  9A04415802        call word 0x258:word 0x4104
000013CC  B80100            mov ax,0x1
000013CF  50                push ax
000013D0  B80700            mov ax,0x7
000013D3  50                push ax
000013D4  B80100            mov ax,0x1
000013D7  50                push ax
000013D8  B81600            mov ax,0x16
000013DB  50                push ax
000013DC  B80400            mov ax,0x4
000013DF  50                push ax
000013E0  9A16505802        call word 0x258:word 0x5016
000013E5  FF364200          push word [0x42]
000013E9  9A04415802        call word 0x258:word 0x4104
000013EE  B80100            mov ax,0x1
000013F1  50                push ax
000013F2  B80800            mov ax,0x8
000013F5  50                push ax
000013F6  B80100            mov ax,0x1
000013F9  50                push ax
000013FA  B81600            mov ax,0x16
000013FD  50                push ax
000013FE  B80400            mov ax,0x4
00001401  50                push ax
00001402  9A16505802        call word 0x258:word 0x5016
00001407  FF364400          push word [0x44]
0000140B  9A04415802        call word 0x258:word 0x4104
00001410  B80100            mov ax,0x1
00001413  50                push ax
00001414  B80900            mov ax,0x9
00001417  50                push ax
00001418  B80100            mov ax,0x1
0000141B  50                push ax
0000141C  B81600            mov ax,0x16
0000141F  50                push ax
00001420  B80400            mov ax,0x4
00001423  50                push ax
00001424  9A16505802        call word 0x258:word 0x5016
00001429  FF364600          push word [0x46]
0000142D  9A04415802        call word 0x258:word 0x4104
00001432  B80100            mov ax,0x1
00001435  50                push ax
00001436  B80A00            mov ax,0xa
00001439  50                push ax
0000143A  B80100            mov ax,0x1
0000143D  50                push ax
0000143E  B81600            mov ax,0x16
00001441  50                push ax
00001442  B80400            mov ax,0x4
00001445  50                push ax
00001446  9A16505802        call word 0x258:word 0x5016
0000144B  FF364800          push word [0x48]
0000144F  9A04415802        call word 0x258:word 0x4104
00001454  B80100            mov ax,0x1
00001457  50                push ax
00001458  B80E00            mov ax,0xe
0000145B  50                push ax
0000145C  B80100            mov ax,0x1
0000145F  50                push ax
00001460  50                push ax
00001461  B80400            mov ax,0x4
00001464  50                push ax
00001465  9AEA4F5802        call word 0x258:word 0x4fea
0000146A  B80100            mov ax,0x1
0000146D  50                push ax
0000146E  B80D00            mov ax,0xd
00001471  50                push ax
00001472  B80100            mov ax,0x1
00001475  50                push ax
00001476  B80700            mov ax,0x7
00001479  50                push ax
0000147A  B80400            mov ax,0x4
0000147D  50                push ax
0000147E  9A16505802        call word 0x258:word 0x5016
00001483  B86604            mov ax,0x466
00001486  50                push ax
00001487  9A22415802        call word 0x258:word 0x4122
0000148C  B80100            mov ax,0x1
0000148F  50                push ax
00001490  B80E00            mov ax,0xe
00001493  50                push ax
00001494  B80100            mov ax,0x1
00001497  50                push ax
00001498  B80700            mov ax,0x7
0000149B  50                push ax
0000149C  B80400            mov ax,0x4
0000149F  50                push ax
000014A0  9A16505802        call word 0x258:word 0x5016
000014A5  B8A804            mov ax,0x4a8
000014A8  50                push ax
000014A9  9A22415802        call word 0x258:word 0x4122
000014AE  B80100            mov ax,0x1
000014B1  50                push ax
000014B2  B80700            mov ax,0x7
000014B5  50                push ax
000014B6  B80100            mov ax,0x1
000014B9  50                push ax
000014BA  50                push ax
000014BB  B80400            mov ax,0x4
000014BE  50                push ax
000014BF  9AEA4F5802        call word 0x258:word 0x4fea
000014C4  B80100            mov ax,0x1
000014C7  50                push ax
000014C8  B81100            mov ax,0x11
000014CB  50                push ax
000014CC  B80100            mov ax,0x1
000014CF  50                push ax
000014D0  B80500            mov ax,0x5
000014D3  50                push ax
000014D4  B80400            mov ax,0x4
000014D7  50                push ax
000014D8  9A16505802        call word 0x258:word 0x5016
000014DD  B8EE04            mov ax,0x4ee
000014E0  50                push ax
000014E1  9A22415802        call word 0x258:word 0x4122
000014E6  B80100            mov ax,0x1
000014E9  50                push ax
000014EA  B8FF7F            mov ax,0x7fff
000014ED  50                push ax
000014EE  9AD6515802        call word 0x258:word 0x51d6
000014F3  50                push ax
000014F4  8D46EE            lea ax,[bp-0x12]
000014F7  50                push ax
000014F8  9A0A4B5802        call word 0x258:word 0x4b0a
000014FD  8D46EE            lea ax,[bp-0x12]
00001500  50                push ax
00001501  9A834D5802        call word 0x258:word 0x4d83
00001506  50                push ax
00001507  8D46EE            lea ax,[bp-0x12]
0000150A  50                push ax
0000150B  9A0A4B5802        call word 0x258:word 0x4b0a
00001510  8D46EE            lea ax,[bp-0x12]
00001513  50                push ax
00001514  8D46F2            lea ax,[bp-0xe]
00001517  50                push ax
00001518  9A0A4B5802        call word 0x258:word 0x4b0a
0000151D  8D46F2            lea ax,[bp-0xe]
00001520  50                push ax
00001521  B8F602            mov ax,0x2f6
00001524  50                push ax
00001525  9A804B5802        call word 0x258:word 0x4b80
0000152A  7403              jz 0x152f
0000152C  E97000            jmp 0x159f
0000152F  B80100            mov ax,0x1
00001532  50                push ax
00001533  B81400            mov ax,0x14
00001536  50                push ax
00001537  B80100            mov ax,0x1
0000153A  50                push ax
0000153B  B80500            mov ax,0x5
0000153E  50                push ax
0000153F  B80400            mov ax,0x4
00001542  50                push ax
00001543  9A16505802        call word 0x258:word 0x5016
00001548  B83805            mov ax,0x538
0000154B  50                push ax
0000154C  E80000            call 0x154f
0000154F  58                pop ax
00001550  050D00            add ax,0xd
00001553  0E                push cs
00001554  50                push ax
00001555  9A64485802        call word 0x258:word 0x4864
0000155A  EB04              jmp 0x1560
0000155C  0200              add al,[bx+si]
0000155E  0114              add [si],dx
00001560  BB3C00            mov bx,0x3c
00001563  1E                push ds
00001564  07                pop es
00001565  06                push es
00001566  53                push bx
00001567  9A114A5802        call word 0x258:word 0x4a11
0000156C  9A56425802        call word 0x258:word 0x4256
00001571  B80100            mov ax,0x1
00001574  50                push ax
00001575  B81400            mov ax,0x14
00001578  50                push ax
00001579  B80100            mov ax,0x1
0000157C  50                push ax
0000157D  B80500            mov ax,0x5
00001580  50                push ax
00001581  B80400            mov ax,0x4
00001584  50                push ax
00001585  9A16505802        call word 0x258:word 0x5016
0000158A  B81E00            mov ax,0x1e
0000158D  50                push ax
0000158E  9A0C4D5802        call word 0x258:word 0x4d0c
00001593  50                push ax
00001594  9A22415802        call word 0x258:word 0x4122
00001599  E8D7F9            call 0xf73
0000159C  E92803            jmp 0x18c7
0000159F  8D46F2            lea ax,[bp-0xe]
000015A2  50                push ax
000015A3  B80A03            mov ax,0x30a
000015A6  50                push ax
000015A7  9A804B5802        call word 0x258:word 0x4b80
000015AC  7403              jz 0x15b1
000015AE  E97000            jmp 0x1621
000015B1  B80100            mov ax,0x1
000015B4  50                push ax
000015B5  B81400            mov ax,0x14
000015B8  50                push ax
000015B9  B80100            mov ax,0x1
000015BC  50                push ax
000015BD  B80500            mov ax,0x5
000015C0  50                push ax
000015C1  B80400            mov ax,0x4
000015C4  50                push ax
000015C5  9A16505802        call word 0x258:word 0x5016
000015CA  B85005            mov ax,0x550
000015CD  50                push ax
000015CE  E80000            call 0x15d1
000015D1  58                pop ax
000015D2  050D00            add ax,0xd
000015D5  0E                push cs
000015D6  50                push ax
000015D7  9A64485802        call word 0x258:word 0x4864
000015DC  EB04              jmp 0x15e2
000015DE  0200              add al,[bx+si]
000015E0  0102              add [bp+si],ax
000015E2  BB4000            mov bx,0x40
000015E5  1E                push ds
000015E6  07                pop es
000015E7  06                push es
000015E8  53                push bx
000015E9  9A0E4A5802        call word 0x258:word 0x4a0e
000015EE  9A56425802        call word 0x258:word 0x4256
000015F3  B80100            mov ax,0x1
000015F6  50                push ax
000015F7  B81400            mov ax,0x14
000015FA  50                push ax
000015FB  B80100            mov ax,0x1
000015FE  50                push ax
000015FF  B80500            mov ax,0x5
00001602  50                push ax
00001603  B80400            mov ax,0x4
00001606  50                push ax
00001607  9A16505802        call word 0x258:word 0x5016
0000160C  B81E00            mov ax,0x1e
0000160F  50                push ax
00001610  9A0C4D5802        call word 0x258:word 0x4d0c
00001615  50                push ax
00001616  9A22415802        call word 0x258:word 0x4122
0000161B  E855F9            call 0xf73
0000161E  E9A602            jmp 0x18c7
00001621  8D46F2            lea ax,[bp-0xe]
00001624  50                push ax
00001625  B81E03            mov ax,0x31e
00001628  50                push ax
00001629  9A804B5802        call word 0x258:word 0x4b80
0000162E  7403              jz 0x1633
00001630  E97000            jmp 0x16a3
00001633  B80100            mov ax,0x1
00001636  50                push ax
00001637  B81400            mov ax,0x14
0000163A  50                push ax
0000163B  B80100            mov ax,0x1
0000163E  50                push ax
0000163F  B80500            mov ax,0x5
00001642  50                push ax
00001643  B80400            mov ax,0x4
00001646  50                push ax
00001647  9A16505802        call word 0x258:word 0x5016
0000164C  B86805            mov ax,0x568
0000164F  50                push ax
00001650  E80000            call 0x1653
00001653  58                pop ax
00001654  050D00            add ax,0xd
00001657  0E                push cs
00001658  50                push ax
00001659  9A64485802        call word 0x258:word 0x4864
0000165E  EB04              jmp 0x1664
00001660  0200              add al,[bx+si]
00001662  0102              add [bp+si],ax
00001664  BB4200            mov bx,0x42
00001667  1E                push ds
00001668  07                pop es
00001669  06                push es
0000166A  53                push bx
0000166B  9A0E4A5802        call word 0x258:word 0x4a0e
00001670  9A56425802        call word 0x258:word 0x4256
00001675  B80100            mov ax,0x1
00001678  50                push ax
00001679  B81400            mov ax,0x14
0000167C  50                push ax
0000167D  B80100            mov ax,0x1
00001680  50                push ax
00001681  B80500            mov ax,0x5
00001684  50                push ax
00001685  B80400            mov ax,0x4
00001688  50                push ax
00001689  9A16505802        call word 0x258:word 0x5016
0000168E  B81E00            mov ax,0x1e
00001691  50                push ax
00001692  9A0C4D5802        call word 0x258:word 0x4d0c
00001697  50                push ax
00001698  9A22415802        call word 0x258:word 0x4122
0000169D  E8D3F8            call 0xf73
000016A0  E92402            jmp 0x18c7
000016A3  8D46F2            lea ax,[bp-0xe]
000016A6  50                push ax
000016A7  B85404            mov ax,0x454
000016AA  50                push ax
000016AB  9A804B5802        call word 0x258:word 0x4b80
000016B0  7403              jz 0x16b5
000016B2  E97000            jmp 0x1725
000016B5  B80100            mov ax,0x1
000016B8  50                push ax
000016B9  B81400            mov ax,0x14
000016BC  50                push ax
000016BD  B80100            mov ax,0x1
000016C0  50                push ax
000016C1  B80500            mov ax,0x5
000016C4  50                push ax
000016C5  B80400            mov ax,0x4
000016C8  50                push ax
000016C9  9A16505802        call word 0x258:word 0x5016
000016CE  B87C05            mov ax,0x57c
000016D1  50                push ax
000016D2  E80000            call 0x16d5
000016D5  58                pop ax
000016D6  050D00            add ax,0xd
000016D9  0E                push cs
000016DA  50                push ax
000016DB  9A64485802        call word 0x258:word 0x4864
000016E0  EB04              jmp 0x16e6
000016E2  0200              add al,[bx+si]
000016E4  0102              add [bp+si],ax
000016E6  BB4400            mov bx,0x44
000016E9  1E                push ds
000016EA  07                pop es
000016EB  06                push es
000016EC  53                push bx
000016ED  9A0E4A5802        call word 0x258:word 0x4a0e
000016F2  9A56425802        call word 0x258:word 0x4256
000016F7  B80100            mov ax,0x1
000016FA  50                push ax
000016FB  B81400            mov ax,0x14
000016FE  50                push ax
000016FF  B80100            mov ax,0x1
00001702  50                push ax
00001703  B80500            mov ax,0x5
00001706  50                push ax
00001707  B80400            mov ax,0x4
0000170A  50                push ax
0000170B  9A16505802        call word 0x258:word 0x5016
00001710  B81E00            mov ax,0x1e
00001713  50                push ax
00001714  9A0C4D5802        call word 0x258:word 0x4d0c
00001719  50                push ax
0000171A  9A22415802        call word 0x258:word 0x4122
0000171F  E851F8            call 0xf73
00001722  E9A201            jmp 0x18c7
00001725  8D46F2            lea ax,[bp-0xe]
00001728  50                push ax
00001729  B85A04            mov ax,0x45a
0000172C  50                push ax
0000172D  9A804B5802        call word 0x258:word 0x4b80
00001732  7403              jz 0x1737
00001734  E97000            jmp 0x17a7
00001737  B80100            mov ax,0x1
0000173A  50                push ax
0000173B  B81400            mov ax,0x14
0000173E  50                push ax
0000173F  B80100            mov ax,0x1
00001742  50                push ax
00001743  B80500            mov ax,0x5
00001746  50                push ax
00001747  B80400            mov ax,0x4
0000174A  50                push ax
0000174B  9A16505802        call word 0x258:word 0x5016
00001750  B89205            mov ax,0x592
00001753  50                push ax
00001754  E80000            call 0x1757
00001757  58                pop ax
00001758  050D00            add ax,0xd
0000175B  0E                push cs
0000175C  50                push ax
0000175D  9A64485802        call word 0x258:word 0x4864
00001762  EB04              jmp 0x1768
00001764  0200              add al,[bx+si]
00001766  0102              add [bp+si],ax
00001768  BB4600            mov bx,0x46
0000176B  1E                push ds
0000176C  07                pop es
0000176D  06                push es
0000176E  53                push bx
0000176F  9A0E4A5802        call word 0x258:word 0x4a0e
00001774  9A56425802        call word 0x258:word 0x4256
00001779  B80100            mov ax,0x1
0000177C  50                push ax
0000177D  B81400            mov ax,0x14
00001780  50                push ax
00001781  B80100            mov ax,0x1
00001784  50                push ax
00001785  B80500            mov ax,0x5
00001788  50                push ax
00001789  B80400            mov ax,0x4
0000178C  50                push ax
0000178D  9A16505802        call word 0x258:word 0x5016
00001792  B81E00            mov ax,0x1e
00001795  50                push ax
00001796  9A0C4D5802        call word 0x258:word 0x4d0c
0000179B  50                push ax
0000179C  9A22415802        call word 0x258:word 0x4122
000017A1  E8CFF7            call 0xf73
000017A4  E92001            jmp 0x18c7
000017A7  8D46F2            lea ax,[bp-0xe]
000017AA  50                push ax
000017AB  B8DE03            mov ax,0x3de
000017AE  50                push ax
000017AF  9A804B5802        call word 0x258:word 0x4b80
000017B4  7403              jz 0x17b9
000017B6  E91300            jmp 0x17cc
000017B9  B8A805            mov ax,0x5a8
000017BC  50                push ax
000017BD  B84A00            mov ax,0x4a
000017C0  50                push ax
000017C1  9A0A4B5802        call word 0x258:word 0x4b0a
000017C6  E8AAF7            call 0xf73
000017C9  E9FB00            jmp 0x18c7
000017CC  8D46F2            lea ax,[bp-0xe]
000017CF  50                push ax
000017D0  B86004            mov ax,0x460
000017D3  50                push ax
000017D4  9A804B5802        call word 0x258:word 0x4b80
000017D9  7403              jz 0x17de
000017DB  E97000            jmp 0x184e
000017DE  B80100            mov ax,0x1
000017E1  50                push ax
000017E2  B81400            mov ax,0x14
000017E5  50                push ax
000017E6  B80100            mov ax,0x1
000017E9  50                push ax
000017EA  B80500            mov ax,0x5
000017ED  50                push ax
000017EE  B80400            mov ax,0x4
000017F1  50                push ax
000017F2  9A16505802        call word 0x258:word 0x5016
000017F7  B8B005            mov ax,0x5b0
000017FA  50                push ax
000017FB  E80000            call 0x17fe
000017FE  58                pop ax
000017FF  050D00            add ax,0xd
00001802  0E                push cs
00001803  50                push ax
00001804  9A64485802        call word 0x258:word 0x4864
00001809  EB04              jmp 0x180f
0000180B  0200              add al,[bx+si]
0000180D  0102              add [bp+si],ax
0000180F  BB4800            mov bx,0x48
00001812  1E                push ds
00001813  07                pop es
00001814  06                push es
00001815  53                push bx
00001816  9A0E4A5802        call word 0x258:word 0x4a0e
0000181B  9A56425802        call word 0x258:word 0x4256
00001820  B80100            mov ax,0x1
00001823  50                push ax
00001824  B81400            mov ax,0x14
00001827  50                push ax
00001828  B80100            mov ax,0x1
0000182B  50                push ax
0000182C  B80500            mov ax,0x5
0000182F  50                push ax
00001830  B80400            mov ax,0x4
00001833  50                push ax
00001834  9A16505802        call word 0x258:word 0x5016
00001839  B81E00            mov ax,0x1e
0000183C  50                push ax
0000183D  9A0C4D5802        call word 0x258:word 0x4d0c
00001842  50                push ax
00001843  9A22415802        call word 0x258:word 0x4122
00001848  E828F7            call 0xf73
0000184B  E97900            jmp 0x18c7
0000184E  8D46F2            lea ax,[bp-0xe]
00001851  50                push ax
00001852  B8C405            mov ax,0x5c4
00001855  50                push ax
00001856  9A804B5802        call word 0x258:word 0x4b80
0000185B  7403              jz 0x1860
0000185D  E93000            jmp 0x1890
00001860  C7063C00C09E      mov word [0x3c],0x9ec0
00001866  C7063E00E605      mov word [0x3e],0x5e6
0000186C  C70640003075      mov word [0x40],0x7530
00001872  C70642003075      mov word [0x42],0x7530
00001878  C70644003075      mov word [0x44],0x7530
0000187E  C70646003075      mov word [0x46],0x7530
00001884  C70648003075      mov word [0x48],0x7530
0000188A  E8E6F6            call 0xf73
0000188D  E93700            jmp 0x18c7
00001890  8D46F2            lea ax,[bp-0xe]
00001893  50                push ax
00001894  B8CA05            mov ax,0x5ca
00001897  50                push ax
00001898  9A804B5802        call word 0x258:word 0x4b80
0000189D  7403              jz 0x18a2
0000189F  E90800            jmp 0x18aa
000018A2  9A23365802        call word 0x258:word 0x3623
000018A7  E91D00            jmp 0x18c7
000018AA  8D46F2            lea ax,[bp-0xe]
000018AD  50                push ax
000018AE  B8D005            mov ax,0x5d0
000018B1  50                push ax
000018B2  9A804B5802        call word 0x258:word 0x4b80
000018B7  7403              jz 0x18bc
000018B9  E90800            jmp 0x18c4
000018BC  9AE40E0000        call word 0x0:word 0xee4
000018C1  E90300            jmp 0x18c7
000018C4  E81FFC            call 0x14e6
000018C7  8D46F2            lea ax,[bp-0xe]
000018CA  50                push ax
000018CB  9A324F5802        call word 0x258:word 0x4f32
000018D0  8D46EE            lea ax,[bp-0x12]
000018D3  50                push ax
000018D4  9A324F5802        call word 0x258:word 0x4f32
000018D9  9A904F5802        call word 0x258:word 0x4f90
000018DE  CA0000            retf word 0x0
000018E1  E96F02            jmp 0x1b53
000018E4  B90400            mov cx,0x4
000018E7  9ABB4F5802        call word 0x258:word 0x4fbb
000018EC  B83800            mov ax,0x38
000018EF  50                push ax
000018F0  B80100            mov ax,0x1
000018F3  50                push ax
000018F4  B8FFFF            mov ax,0xffff
000018F7  50                push ax
000018F8  B82000            mov ax,0x20
000018FB  50                push ax
000018FC  9AF8155802        call word 0x258:word 0x15f8
00001901  B83000            mov ax,0x30
00001904  50                push ax
00001905  8D46F2            lea ax,[bp-0xe]
00001908  50                push ax
00001909  9A0A4B5802        call word 0x258:word 0x4b0a
0000190E  8D46F2            lea ax,[bp-0xe]
00001911  50                push ax
00001912  B8F602            mov ax,0x2f6
00001915  50                push ax
00001916  9A804B5802        call word 0x258:word 0x4b80
0000191B  7403              jz 0x1920
0000191D  E99F00            jmp 0x19bf
00001920  B80100            mov ax,0x1
00001923  50                push ax
00001924  B8FA13            mov ax,0x13fa
00001927  99                cwd
00001928  52                push dx
00001929  50                push ax
0000192A  BB3C00            mov bx,0x3c
0000192D  1E                push ds
0000192E  07                pop es
0000192F  06                push es
00001930  53                push bx
00001931  B80400            mov ax,0x4
00001934  50                push ax
00001935  9AC1005802        call word 0x258:word 0xc1
0000193A  B80100            mov ax,0x1
0000193D  50                push ax
0000193E  B8F413            mov ax,0x13f4
00001941  99                cwd
00001942  52                push dx
00001943  50                push ax
00001944  BB4000            mov bx,0x40
00001947  1E                push ds
00001948  07                pop es
00001949  06                push es
0000194A  53                push bx
0000194B  B80200            mov ax,0x2
0000194E  50                push ax
0000194F  9AC1005802        call word 0x258:word 0xc1
00001954  B80100            mov ax,0x1
00001957  50                push ax
00001958  B8F613            mov ax,0x13f6
0000195B  99                cwd
0000195C  52                push dx
0000195D  50                push ax
0000195E  BB4200            mov bx,0x42
00001961  1E                push ds
00001962  07                pop es
00001963  06                push es
00001964  53                push bx
00001965  B80200            mov ax,0x2
00001968  50                push ax
00001969  9AC1005802        call word 0x258:word 0xc1
0000196E  B80100            mov ax,0x1
00001971  50                push ax
00001972  B8F813            mov ax,0x13f8
00001975  99                cwd
00001976  52                push dx
00001977  50                push ax
00001978  BB4400            mov bx,0x44
0000197B  1E                push ds
0000197C  07                pop es
0000197D  06                push es
0000197E  53                push bx
0000197F  B80200            mov ax,0x2
00001982  50                push ax
00001983  9AC1005802        call word 0x258:word 0xc1
00001988  B80100            mov ax,0x1
0000198B  50                push ax
0000198C  B8EE13            mov ax,0x13ee
0000198F  99                cwd
00001990  52                push dx
00001991  50                push ax
00001992  BB4600            mov bx,0x46
00001995  1E                push ds
00001996  07                pop es
00001997  06                push es
00001998  53                push bx
00001999  B80200            mov ax,0x2
0000199C  50                push ax
0000199D  9AC1005802        call word 0x258:word 0xc1
000019A2  B80100            mov ax,0x1
000019A5  50                push ax
000019A6  B8F013            mov ax,0x13f0
000019A9  99                cwd
000019AA  52                push dx
000019AB  50                push ax
000019AC  BB4800            mov bx,0x48
000019AF  1E                push ds
000019B0  07                pop es
000019B1  06                push es
000019B2  53                push bx
000019B3  B80200            mov ax,0x2
000019B6  50                push ax
000019B7  9AC1005802        call word 0x258:word 0xc1
000019BC  E96201            jmp 0x1b21
000019BF  8D46F2            lea ax,[bp-0xe]
000019C2  50                push ax
000019C3  B80A03            mov ax,0x30a
000019C6  50                push ax
000019C7  9A804B5802        call word 0x258:word 0x4b80
000019CC  7403              jz 0x19d1
000019CE  E99F00            jmp 0x1a70
000019D1  B80100            mov ax,0x1
000019D4  50                push ax
000019D5  B82A12            mov ax,0x122a
000019D8  99                cwd
000019D9  52                push dx
000019DA  50                push ax
000019DB  BB3C00            mov bx,0x3c
000019DE  1E                push ds
000019DF  07                pop es
000019E0  06                push es
000019E1  53                push bx
000019E2  B80400            mov ax,0x4
000019E5  50                push ax
000019E6  9AC1005802        call word 0x258:word 0xc1
000019EB  B80100            mov ax,0x1
000019EE  50                push ax
000019EF  B82412            mov ax,0x1224
000019F2  99                cwd
000019F3  52                push dx
000019F4  50                push ax
000019F5  BB4000            mov bx,0x40
000019F8  1E                push ds
000019F9  07                pop es
000019FA  06                push es
000019FB  53                push bx
000019FC  B80200            mov ax,0x2
000019FF  50                push ax
00001A00  9AC1005802        call word 0x258:word 0xc1
00001A05  B80100            mov ax,0x1
00001A08  50                push ax
00001A09  B82612            mov ax,0x1226
00001A0C  99                cwd
00001A0D  52                push dx
00001A0E  50                push ax
00001A0F  BB4200            mov bx,0x42
00001A12  1E                push ds
00001A13  07                pop es
00001A14  06                push es
00001A15  53                push bx
00001A16  B80200            mov ax,0x2
00001A19  50                push ax
00001A1A  9AC1005802        call word 0x258:word 0xc1
00001A1F  B80100            mov ax,0x1
00001A22  50                push ax
00001A23  B82812            mov ax,0x1228
00001A26  99                cwd
00001A27  52                push dx
00001A28  50                push ax
00001A29  BB4400            mov bx,0x44
00001A2C  1E                push ds
00001A2D  07                pop es
00001A2E  06                push es
00001A2F  53                push bx
00001A30  B80200            mov ax,0x2
00001A33  50                push ax
00001A34  9AC1005802        call word 0x258:word 0xc1
00001A39  B80100            mov ax,0x1
00001A3C  50                push ax
00001A3D  B81E12            mov ax,0x121e
00001A40  99                cwd
00001A41  52                push dx
00001A42  50                push ax
00001A43  BB4600            mov bx,0x46
00001A46  1E                push ds
00001A47  07                pop es
00001A48  06                push es
00001A49  53                push bx
00001A4A  B80200            mov ax,0x2
00001A4D  50                push ax
00001A4E  9AC1005802        call word 0x258:word 0xc1
00001A53  B80100            mov ax,0x1
00001A56  50                push ax
00001A57  B82012            mov ax,0x1220
00001A5A  99                cwd
00001A5B  52                push dx
00001A5C  50                push ax
00001A5D  BB4800            mov bx,0x48
00001A60  1E                push ds
00001A61  07                pop es
00001A62  06                push es
00001A63  53                push bx
00001A64  B80200            mov ax,0x2
00001A67  50                push ax
00001A68  9AC1005802        call word 0x258:word 0xc1
00001A6D  E9B100            jmp 0x1b21
00001A70  8D46F2            lea ax,[bp-0xe]
00001A73  50                push ax
00001A74  B81E03            mov ax,0x31e
00001A77  50                push ax
00001A78  9A804B5802        call word 0x258:word 0x4b80
00001A7D  7403              jz 0x1a82
00001A7F  E99F00            jmp 0x1b21
00001A82  B80100            mov ax,0x1
00001A85  50                push ax
00001A86  B88A0E            mov ax,0xe8a
00001A89  99                cwd
00001A8A  52                push dx
00001A8B  50                push ax
00001A8C  BB3C00            mov bx,0x3c
00001A8F  1E                push ds
00001A90  07                pop es
00001A91  06                push es
00001A92  53                push bx
00001A93  B80400            mov ax,0x4
00001A96  50                push ax
00001A97  9AC1005802        call word 0x258:word 0xc1
00001A9C  B80100            mov ax,0x1
00001A9F  50                push ax
00001AA0  B8840E            mov ax,0xe84
00001AA3  99                cwd
00001AA4  52                push dx
00001AA5  50                push ax
00001AA6  BB4000            mov bx,0x40
00001AA9  1E                push ds
00001AAA  07                pop es
00001AAB  06                push es
00001AAC  53                push bx
00001AAD  B80200            mov ax,0x2
00001AB0  50                push ax
00001AB1  9AC1005802        call word 0x258:word 0xc1
00001AB6  B80100            mov ax,0x1
00001AB9  50                push ax
00001ABA  B8860E            mov ax,0xe86
00001ABD  99                cwd
00001ABE  52                push dx
00001ABF  50                push ax
00001AC0  BB4200            mov bx,0x42
00001AC3  1E                push ds
00001AC4  07                pop es
00001AC5  06                push es
00001AC6  53                push bx
00001AC7  B80200            mov ax,0x2
00001ACA  50                push ax
00001ACB  9AC1005802        call word 0x258:word 0xc1
00001AD0  B80100            mov ax,0x1
00001AD3  50                push ax
00001AD4  B8880E            mov ax,0xe88
00001AD7  99                cwd
00001AD8  52                push dx
00001AD9  50                push ax
00001ADA  BB4400            mov bx,0x44
00001ADD  1E                push ds
00001ADE  07                pop es
00001ADF  06                push es
00001AE0  53                push bx
00001AE1  B80200            mov ax,0x2
00001AE4  50                push ax
00001AE5  9AC1005802        call word 0x258:word 0xc1
00001AEA  B80100            mov ax,0x1
00001AED  50                push ax
00001AEE  B87E0E            mov ax,0xe7e
00001AF1  99                cwd
00001AF2  52                push dx
00001AF3  50                push ax
00001AF4  BB4600            mov bx,0x46
00001AF7  1E                push ds
00001AF8  07                pop es
00001AF9  06                push es
00001AFA  53                push bx
00001AFB  B80200            mov ax,0x2
00001AFE  50                push ax
00001AFF  9AC1005802        call word 0x258:word 0xc1
00001B04  B80100            mov ax,0x1
00001B07  50                push ax
00001B08  B8800E            mov ax,0xe80
00001B0B  99                cwd
00001B0C  52                push dx
00001B0D  50                push ax
00001B0E  BB4800            mov bx,0x48
00001B11  1E                push ds
00001B12  07                pop es
00001B13  06                push es
00001B14  53                push bx
00001B15  B80200            mov ax,0x2
00001B18  50                push ax
00001B19  9AC1005802        call word 0x258:word 0xc1
00001B1E  E90000            jmp 0x1b21
00001B21  B80100            mov ax,0x1
00001B24  50                push ax
00001B25  50                push ax
00001B26  9A3F175802        call word 0x258:word 0x173f
00001B2B  B84A00            mov ax,0x4a
00001B2E  50                push ax
00001B2F  B8A805            mov ax,0x5a8
00001B32  50                push ax
00001B33  9A804B5802        call word 0x258:word 0x4b80
00001B38  7403              jz 0x1b3d
00001B3A  E90500            jmp 0x1b42
00001B3D  9AB1130000        call word 0x0:word 0x13b1
00001B42  8D46F2            lea ax,[bp-0xe]
00001B45  50                push ax
00001B46  9A324F5802        call word 0x258:word 0x4f32
00001B4B  9A904F5802        call word 0x258:word 0x4f90
00001B50  CA0000            retf word 0x0
00001B53  E95802            jmp 0x1dae
00001B56  B90400            mov cx,0x4
00001B59  9ABB4F5802        call word 0x258:word 0x4fbb
00001B5E  B83800            mov ax,0x38
00001B61  50                push ax
00001B62  B80100            mov ax,0x1
00001B65  50                push ax
00001B66  B8FFFF            mov ax,0xffff
00001B69  50                push ax
00001B6A  B82000            mov ax,0x20
00001B6D  50                push ax
00001B6E  9AF8155802        call word 0x258:word 0x15f8
00001B73  B83000            mov ax,0x30
00001B76  50                push ax
00001B77  8D46F2            lea ax,[bp-0xe]
00001B7A  50                push ax
00001B7B  9A0A4B5802        call word 0x258:word 0x4b0a
00001B80  8D46F2            lea ax,[bp-0xe]
00001B83  50                push ax
00001B84  B8F602            mov ax,0x2f6
00001B87  50                push ax
00001B88  9A804B5802        call word 0x258:word 0x4b80
00001B8D  7403              jz 0x1b92
00001B8F  E99F00            jmp 0x1c31
00001B92  B80100            mov ax,0x1
00001B95  50                push ax
00001B96  B8FA13            mov ax,0x13fa
00001B99  99                cwd
00001B9A  52                push dx
00001B9B  50                push ax
00001B9C  BB3C00            mov bx,0x3c
00001B9F  1E                push ds
00001BA0  07                pop es
00001BA1  06                push es
00001BA2  53                push bx
00001BA3  B80400            mov ax,0x4
00001BA6  50                push ax
00001BA7  9A96005802        call word 0x258:word 0x96
00001BAC  B80100            mov ax,0x1
00001BAF  50                push ax
00001BB0  B8F413            mov ax,0x13f4
00001BB3  99                cwd
00001BB4  52                push dx
00001BB5  50                push ax
00001BB6  BB4000            mov bx,0x40
00001BB9  1E                push ds
00001BBA  07                pop es
00001BBB  06                push es
00001BBC  53                push bx
00001BBD  B80200            mov ax,0x2
00001BC0  50                push ax
00001BC1  9A96005802        call word 0x258:word 0x96
00001BC6  B80100            mov ax,0x1
00001BC9  50                push ax
00001BCA  B8F613            mov ax,0x13f6
00001BCD  99                cwd
00001BCE  52                push dx
00001BCF  50                push ax
00001BD0  BB4200            mov bx,0x42
00001BD3  1E                push ds
00001BD4  07                pop es
00001BD5  06                push es
00001BD6  53                push bx
00001BD7  B80200            mov ax,0x2
00001BDA  50                push ax
00001BDB  9A96005802        call word 0x258:word 0x96
00001BE0  B80100            mov ax,0x1
00001BE3  50                push ax
00001BE4  B8F813            mov ax,0x13f8
00001BE7  99                cwd
00001BE8  52                push dx
00001BE9  50                push ax
00001BEA  BB4400            mov bx,0x44
00001BED  1E                push ds
00001BEE  07                pop es
00001BEF  06                push es
00001BF0  53                push bx
00001BF1  B80200            mov ax,0x2
00001BF4  50                push ax
00001BF5  9A96005802        call word 0x258:word 0x96
00001BFA  B80100            mov ax,0x1
00001BFD  50                push ax
00001BFE  B8EE13            mov ax,0x13ee
00001C01  99                cwd
00001C02  52                push dx
00001C03  50                push ax
00001C04  BB4600            mov bx,0x46
00001C07  1E                push ds
00001C08  07                pop es
00001C09  06                push es
00001C0A  53                push bx
00001C0B  B80200            mov ax,0x2
00001C0E  50                push ax
00001C0F  9A96005802        call word 0x258:word 0x96
00001C14  B80100            mov ax,0x1
00001C17  50                push ax
00001C18  B8F013            mov ax,0x13f0
00001C1B  99                cwd
00001C1C  52                push dx
00001C1D  50                push ax
00001C1E  BB4800            mov bx,0x48
00001C21  1E                push ds
00001C22  07                pop es
00001C23  06                push es
00001C24  53                push bx
00001C25  B80200            mov ax,0x2
00001C28  50                push ax
00001C29  9A96005802        call word 0x258:word 0x96
00001C2E  E96201            jmp 0x1d93
00001C31  8D46F2            lea ax,[bp-0xe]
00001C34  50                push ax
00001C35  B80A03            mov ax,0x30a
00001C38  50                push ax
00001C39  9A804B5802        call word 0x258:word 0x4b80
00001C3E  7403              jz 0x1c43
00001C40  E99F00            jmp 0x1ce2
00001C43  B80100            mov ax,0x1
00001C46  50                push ax
00001C47  B82A12            mov ax,0x122a
00001C4A  99                cwd
00001C4B  52                push dx
00001C4C  50                push ax
00001C4D  BB3C00            mov bx,0x3c
00001C50  1E                push ds
00001C51  07                pop es
00001C52  06                push es
00001C53  53                push bx
00001C54  B80400            mov ax,0x4
00001C57  50                push ax
00001C58  9A96005802        call word 0x258:word 0x96
00001C5D  B80100            mov ax,0x1
00001C60  50                push ax
00001C61  B82412            mov ax,0x1224
00001C64  99                cwd
00001C65  52                push dx
00001C66  50                push ax
00001C67  BB4000            mov bx,0x40
00001C6A  1E                push ds
00001C6B  07                pop es
00001C6C  06                push es
00001C6D  53                push bx
00001C6E  B80200            mov ax,0x2
00001C71  50                push ax
00001C72  9A96005802        call word 0x258:word 0x96
00001C77  B80100            mov ax,0x1
00001C7A  50                push ax
00001C7B  B82612            mov ax,0x1226
00001C7E  99                cwd
00001C7F  52                push dx
00001C80  50                push ax
00001C81  BB4200            mov bx,0x42
00001C84  1E                push ds
00001C85  07                pop es
00001C86  06                push es
00001C87  53                push bx
00001C88  B80200            mov ax,0x2
00001C8B  50                push ax
00001C8C  9A96005802        call word 0x258:word 0x96
00001C91  B80100            mov ax,0x1
00001C94  50                push ax
00001C95  B82812            mov ax,0x1228
00001C98  99                cwd
00001C99  52                push dx
00001C9A  50                push ax
00001C9B  BB4400            mov bx,0x44
00001C9E  1E                push ds
00001C9F  07                pop es
00001CA0  06                push es
00001CA1  53                push bx
00001CA2  B80200            mov ax,0x2
00001CA5  50                push ax
00001CA6  9A96005802        call word 0x258:word 0x96
00001CAB  B80100            mov ax,0x1
00001CAE  50                push ax
00001CAF  B81E12            mov ax,0x121e
00001CB2  99                cwd
00001CB3  52                push dx
00001CB4  50                push ax
00001CB5  BB4600            mov bx,0x46
00001CB8  1E                push ds
00001CB9  07                pop es
00001CBA  06                push es
00001CBB  53                push bx
00001CBC  B80200            mov ax,0x2
00001CBF  50                push ax
00001CC0  9A96005802        call word 0x258:word 0x96
00001CC5  B80100            mov ax,0x1
00001CC8  50                push ax
00001CC9  B82012            mov ax,0x1220
00001CCC  99                cwd
00001CCD  52                push dx
00001CCE  50                push ax
00001CCF  BB4800            mov bx,0x48
00001CD2  1E                push ds
00001CD3  07                pop es
00001CD4  06                push es
00001CD5  53                push bx
00001CD6  B80200            mov ax,0x2
00001CD9  50                push ax
00001CDA  9A96005802        call word 0x258:word 0x96
00001CDF  E9B100            jmp 0x1d93
00001CE2  8D46F2            lea ax,[bp-0xe]
00001CE5  50                push ax
00001CE6  B81E03            mov ax,0x31e
00001CE9  50                push ax
00001CEA  9A804B5802        call word 0x258:word 0x4b80
00001CEF  7403              jz 0x1cf4
00001CF1  E99F00            jmp 0x1d93
00001CF4  B80100            mov ax,0x1
00001CF7  50                push ax
00001CF8  B88A0E            mov ax,0xe8a
00001CFB  99                cwd
00001CFC  52                push dx
00001CFD  50                push ax
00001CFE  BB3C00            mov bx,0x3c
00001D01  1E                push ds
00001D02  07                pop es
00001D03  06                push es
00001D04  53                push bx
00001D05  B80400            mov ax,0x4
00001D08  50                push ax
00001D09  9A96005802        call word 0x258:word 0x96
00001D0E  B80100            mov ax,0x1
00001D11  50                push ax
00001D12  B8840E            mov ax,0xe84
00001D15  99                cwd
00001D16  52                push dx
00001D17  50                push ax
00001D18  BB4000            mov bx,0x40
00001D1B  1E                push ds
00001D1C  07                pop es
00001D1D  06                push es
00001D1E  53                push bx
00001D1F  B80200            mov ax,0x2
00001D22  50                push ax
00001D23  9A96005802        call word 0x258:word 0x96
00001D28  B80100            mov ax,0x1
00001D2B  50                push ax
00001D2C  B8860E            mov ax,0xe86
00001D2F  99                cwd
00001D30  52                push dx
00001D31  50                push ax
00001D32  BB4200            mov bx,0x42
00001D35  1E                push ds
00001D36  07                pop es
00001D37  06                push es
00001D38  53                push bx
00001D39  B80200            mov ax,0x2
00001D3C  50                push ax
00001D3D  9A96005802        call word 0x258:word 0x96
00001D42  B80100            mov ax,0x1
00001D45  50                push ax
00001D46  B8880E            mov ax,0xe88
00001D49  99                cwd
00001D4A  52                push dx
00001D4B  50                push ax
00001D4C  BB4400            mov bx,0x44
00001D4F  1E                push ds
00001D50  07                pop es
00001D51  06                push es
00001D52  53                push bx
00001D53  B80200            mov ax,0x2
00001D56  50                push ax
00001D57  9A96005802        call word 0x258:word 0x96
00001D5C  B80100            mov ax,0x1
00001D5F  50                push ax
00001D60  B87E0E            mov ax,0xe7e
00001D63  99                cwd
00001D64  52                push dx
00001D65  50                push ax
00001D66  BB4600            mov bx,0x46
00001D69  1E                push ds
00001D6A  07                pop es
00001D6B  06                push es
00001D6C  53                push bx
00001D6D  B80200            mov ax,0x2
00001D70  50                push ax
00001D71  9A96005802        call word 0x258:word 0x96
00001D76  B80100            mov ax,0x1
00001D79  50                push ax
00001D7A  B8800E            mov ax,0xe80
00001D7D  99                cwd
00001D7E  52                push dx
00001D7F  50                push ax
00001D80  BB4800            mov bx,0x48
00001D83  1E                push ds
00001D84  07                pop es
00001D85  06                push es
00001D86  53                push bx
00001D87  B80200            mov ax,0x2
00001D8A  50                push ax
00001D8B  9A96005802        call word 0x258:word 0x96
00001D90  E90000            jmp 0x1d93
00001D93  B80100            mov ax,0x1
00001D96  50                push ax
00001D97  50                push ax
00001D98  9A3F175802        call word 0x258:word 0x173f
00001D9D  8D46F2            lea ax,[bp-0xe]
00001DA0  50                push ax
00001DA1  9A324F5802        call word 0x258:word 0x4f32
00001DA6  9A904F5802        call word 0x258:word 0x4f90
00001DAB  CA0000            retf word 0x0
00001DAE  E9CB11            jmp 0x2f7c
00001DB1  B91200            mov cx,0x12
00001DB4  9ABB4F5802        call word 0x258:word 0x4fbb
00001DB9  C746F0C800        mov word [bp-0x10],0xc8
00001DBE  C746EE6400        mov word [bp-0x12],0x64
00001DC3  C746EC0300        mov word [bp-0x14],0x3
00001DC8  C746EA0200        mov word [bp-0x16],0x2
00001DCD  C746E80200        mov word [bp-0x18],0x2
00001DD2  B83000            mov ax,0x30
00001DD5  50                push ax
00001DD6  8D46F2            lea ax,[bp-0xe]
00001DD9  50                push ax
00001DDA  9A0A4B5802        call word 0x258:word 0x4b0a
00001DDF  8D46F2            lea ax,[bp-0xe]
00001DE2  50                push ax
00001DE3  B8F602            mov ax,0x2f6
00001DE6  50                push ax
00001DE7  9A804B5802        call word 0x258:word 0x4b80
00001DEC  7403              jz 0x1df1
00001DEE  E90D00            jmp 0x1dfe
00001DF1  C746E6FC00        mov word [bp-0x1a],0xfc
00001DF6  C746E49000        mov word [bp-0x1c],0x90
00001DFB  E93E00            jmp 0x1e3c
00001DFE  8D46F2            lea ax,[bp-0xe]
00001E01  50                push ax
00001E02  B80A03            mov ax,0x30a
00001E05  50                push ax
00001E06  9A804B5802        call word 0x258:word 0x4b80
00001E0B  7403              jz 0x1e10
00001E0D  E90D00            jmp 0x1e1d
00001E10  C746E62C00        mov word [bp-0x1a],0x2c
00001E15  C746E48F00        mov word [bp-0x1c],0x8f
00001E1A  E91F00            jmp 0x1e3c
00001E1D  8D46F2            lea ax,[bp-0xe]
00001E20  50                push ax
00001E21  B81E03            mov ax,0x31e
00001E24  50                push ax
00001E25  9A804B5802        call word 0x258:word 0x4b80
00001E2A  7403              jz 0x1e2f
00001E2C  E90D00            jmp 0x1e3c
00001E2F  C746E68C00        mov word [bp-0x1a],0x8c
00001E34  C746E48B00        mov word [bp-0x1c],0x8b
00001E39  E90000            jmp 0x1e3c
00001E3C  B83800            mov ax,0x38
00001E3F  50                push ax
00001E40  B80100            mov ax,0x1
00001E43  50                push ax
00001E44  B8FFFF            mov ax,0xffff
00001E47  50                push ax
00001E48  B82000            mov ax,0x20
00001E4B  50                push ax
00001E4C  9AF8155802        call word 0x258:word 0x15f8
00001E51  B80100            mov ax,0x1
00001E54  50                push ax
00001E55  B8591C            mov ax,0x1c59
00001E58  99                cwd
00001E59  52                push dx
00001E5A  50                push ax
00001E5B  8D5EF0            lea bx,[bp-0x10]
00001E5E  1E                push ds
00001E5F  07                pop es
00001E60  06                push es
00001E61  53                push bx
00001E62  B80200            mov ax,0x2
00001E65  50                push ax
00001E66  9AC1005802        call word 0x258:word 0xc1
00001E6B  B80100            mov ax,0x1
00001E6E  50                push ax
00001E6F  B85B1C            mov ax,0x1c5b
00001E72  99                cwd
00001E73  52                push dx
00001E74  50                push ax
00001E75  8D5EE6            lea bx,[bp-0x1a]
00001E78  1E                push ds
00001E79  07                pop es
00001E7A  06                push es
00001E7B  53                push bx
00001E7C  B80200            mov ax,0x2
00001E7F  50                push ax
00001E80  9AC1005802        call word 0x258:word 0xc1
00001E85  B80100            mov ax,0x1
00001E88  50                push ax
00001E89  B85C1C            mov ax,0x1c5c
00001E8C  99                cwd
00001E8D  52                push dx
00001E8E  50                push ax
00001E8F  8D5EE4            lea bx,[bp-0x1c]
00001E92  1E                push ds
00001E93  07                pop es
00001E94  06                push es
00001E95  53                push bx
00001E96  B80200            mov ax,0x2
00001E99  50                push ax
00001E9A  9AC1005802        call word 0x258:word 0xc1
00001E9F  B80100            mov ax,0x1
00001EA2  50                push ax
00001EA3  B85F1C            mov ax,0x1c5f
00001EA6  99                cwd
00001EA7  52                push dx
00001EA8  50                push ax
00001EA9  8D5EE8            lea bx,[bp-0x18]
00001EAC  1E                push ds
00001EAD  07                pop es
00001EAE  06                push es
00001EAF  53                push bx
00001EB0  B80200            mov ax,0x2
00001EB3  50                push ax
00001EB4  9AC1005802        call word 0x258:word 0xc1
00001EB9  B80100            mov ax,0x1
00001EBC  50                push ax
00001EBD  B8601C            mov ax,0x1c60
00001EC0  99                cwd
00001EC1  52                push dx
00001EC2  50                push ax
00001EC3  8D5EEE            lea bx,[bp-0x12]
00001EC6  1E                push ds
00001EC7  07                pop es
00001EC8  06                push es
00001EC9  53                push bx
00001ECA  B80200            mov ax,0x2
00001ECD  50                push ax
00001ECE  9AC1005802        call word 0x258:word 0xc1
00001ED3  B80100            mov ax,0x1
00001ED6  50                push ax
00001ED7  B8611C            mov ax,0x1c61
00001EDA  99                cwd
00001EDB  52                push dx
00001EDC  50                push ax
00001EDD  8D5EEC            lea bx,[bp-0x14]
00001EE0  1E                push ds
00001EE1  07                pop es
00001EE2  06                push es
00001EE3  53                push bx
00001EE4  B80200            mov ax,0x2
00001EE7  50                push ax
00001EE8  9AC1005802        call word 0x258:word 0xc1
00001EED  B80100            mov ax,0x1
00001EF0  50                push ax
00001EF1  B8631C            mov ax,0x1c63
00001EF4  99                cwd
00001EF5  52                push dx
00001EF6  50                push ax
00001EF7  8D5EEA            lea bx,[bp-0x16]
00001EFA  1E                push ds
00001EFB  07                pop es
00001EFC  06                push es
00001EFD  53                push bx
00001EFE  B80200            mov ax,0x2
00001F01  50                push ax
00001F02  9AC1005802        call word 0x258:word 0xc1
00001F07  B80100            mov ax,0x1
00001F0A  50                push ax
00001F0B  B8651C            mov ax,0x1c65
00001F0E  99                cwd
00001F0F  52                push dx
00001F10  50                push ax
00001F11  8D5EF0            lea bx,[bp-0x10]
00001F14  1E                push ds
00001F15  07                pop es
00001F16  06                push es
00001F17  53                push bx
00001F18  B80200            mov ax,0x2
00001F1B  50                push ax
00001F1C  9AC1005802        call word 0x258:word 0xc1
00001F21  B80100            mov ax,0x1
00001F24  50                push ax
00001F25  B8671C            mov ax,0x1c67
00001F28  99                cwd
00001F29  52                push dx
00001F2A  50                push ax
00001F2B  8D5EE6            lea bx,[bp-0x1a]
00001F2E  1E                push ds
00001F2F  07                pop es
00001F30  06                push es
00001F31  53                push bx
00001F32  B80200            mov ax,0x2
00001F35  50                push ax
00001F36  9AC1005802        call word 0x258:word 0xc1
00001F3B  B80100            mov ax,0x1
00001F3E  50                push ax
00001F3F  B8681C            mov ax,0x1c68
00001F42  99                cwd
00001F43  52                push dx
00001F44  50                push ax
00001F45  8D5EE4            lea bx,[bp-0x1c]
00001F48  1E                push ds
00001F49  07                pop es
00001F4A  06                push es
00001F4B  53                push bx
00001F4C  B80200            mov ax,0x2
00001F4F  50                push ax
00001F50  9AC1005802        call word 0x258:word 0xc1
00001F55  B80100            mov ax,0x1
00001F58  50                push ax
00001F59  B86B1C            mov ax,0x1c6b
00001F5C  99                cwd
00001F5D  52                push dx
00001F5E  50                push ax
00001F5F  8D5EE8            lea bx,[bp-0x18]
00001F62  1E                push ds
00001F63  07                pop es
00001F64  06                push es
00001F65  53                push bx
00001F66  B80200            mov ax,0x2
00001F69  50                push ax
00001F6A  9AC1005802        call word 0x258:word 0xc1
00001F6F  B80100            mov ax,0x1
00001F72  50                push ax
00001F73  B86C1C            mov ax,0x1c6c
00001F76  99                cwd
00001F77  52                push dx
00001F78  50                push ax
00001F79  8D5EEE            lea bx,[bp-0x12]
00001F7C  1E                push ds
00001F7D  07                pop es
00001F7E  06                push es
00001F7F  53                push bx
00001F80  B80200            mov ax,0x2
00001F83  50                push ax
00001F84  9AC1005802        call word 0x258:word 0xc1
00001F89  B80100            mov ax,0x1
00001F8C  50                push ax
00001F8D  B86D1C            mov ax,0x1c6d
00001F90  99                cwd
00001F91  52                push dx
00001F92  50                push ax
00001F93  8D5EEC            lea bx,[bp-0x14]
00001F96  1E                push ds
00001F97  07                pop es
00001F98  06                push es
00001F99  53                push bx
00001F9A  B80200            mov ax,0x2
00001F9D  50                push ax
00001F9E  9AC1005802        call word 0x258:word 0xc1
00001FA3  B80100            mov ax,0x1
00001FA6  50                push ax
00001FA7  B86F1C            mov ax,0x1c6f
00001FAA  99                cwd
00001FAB  52                push dx
00001FAC  50                push ax
00001FAD  8D5EEA            lea bx,[bp-0x16]
00001FB0  1E                push ds
00001FB1  07                pop es
00001FB2  06                push es
00001FB3  53                push bx
00001FB4  B80200            mov ax,0x2
00001FB7  50                push ax
00001FB8  9AC1005802        call word 0x258:word 0xc1
00001FBD  B80100            mov ax,0x1
00001FC0  50                push ax
00001FC1  B8711C            mov ax,0x1c71
00001FC4  99                cwd
00001FC5  52                push dx
00001FC6  50                push ax
00001FC7  8D5EF0            lea bx,[bp-0x10]
00001FCA  1E                push ds
00001FCB  07                pop es
00001FCC  06                push es
00001FCD  53                push bx
00001FCE  B80200            mov ax,0x2
00001FD1  50                push ax
00001FD2  9AC1005802        call word 0x258:word 0xc1
00001FD7  B80100            mov ax,0x1
00001FDA  50                push ax
00001FDB  B8731C            mov ax,0x1c73
00001FDE  99                cwd
00001FDF  52                push dx
00001FE0  50                push ax
00001FE1  8D5EE6            lea bx,[bp-0x1a]
00001FE4  1E                push ds
00001FE5  07                pop es
00001FE6  06                push es
00001FE7  53                push bx
00001FE8  B80200            mov ax,0x2
00001FEB  50                push ax
00001FEC  9AC1005802        call word 0x258:word 0xc1
00001FF1  B80100            mov ax,0x1
00001FF4  50                push ax
00001FF5  B8741C            mov ax,0x1c74
00001FF8  99                cwd
00001FF9  52                push dx
00001FFA  50                push ax
00001FFB  8D5EE4            lea bx,[bp-0x1c]
00001FFE  1E                push ds
00001FFF  07                pop es
00002000  06                push es
00002001  53                push bx
00002002  B80200            mov ax,0x2
00002005  50                push ax
00002006  9AC1005802        call word 0x258:word 0xc1
0000200B  B80100            mov ax,0x1
0000200E  50                push ax
0000200F  B8771C            mov ax,0x1c77
00002012  99                cwd
00002013  52                push dx
00002014  50                push ax
00002015  8D5EE8            lea bx,[bp-0x18]
00002018  1E                push ds
00002019  07                pop es
0000201A  06                push es
0000201B  53                push bx
0000201C  B80200            mov ax,0x2
0000201F  50                push ax
00002020  9AC1005802        call word 0x258:word 0xc1
00002025  B80100            mov ax,0x1
00002028  50                push ax
00002029  B8781C            mov ax,0x1c78
0000202C  99                cwd
0000202D  52                push dx
0000202E  50                push ax
0000202F  8D5EEE            lea bx,[bp-0x12]
00002032  1E                push ds
00002033  07                pop es
00002034  06                push es
00002035  53                push bx
00002036  B80200            mov ax,0x2
00002039  50                push ax
0000203A  9AC1005802        call word 0x258:word 0xc1
0000203F  B80100            mov ax,0x1
00002042  50                push ax
00002043  B8791C            mov ax,0x1c79
00002046  99                cwd
00002047  52                push dx
00002048  50                push ax
00002049  8D5EEC            lea bx,[bp-0x14]
0000204C  1E                push ds
0000204D  07                pop es
0000204E  06                push es
0000204F  53                push bx
00002050  B80200            mov ax,0x2
00002053  50                push ax
00002054  9AC1005802        call word 0x258:word 0xc1
00002059  B80100            mov ax,0x1
0000205C  50                push ax
0000205D  B87B1C            mov ax,0x1c7b
00002060  99                cwd
00002061  52                push dx
00002062  50                push ax
00002063  8D5EEA            lea bx,[bp-0x16]
00002066  1E                push ds
00002067  07                pop es
00002068  06                push es
00002069  53                push bx
0000206A  B80200            mov ax,0x2
0000206D  50                push ax
0000206E  9AC1005802        call word 0x258:word 0xc1
00002073  B80100            mov ax,0x1
00002076  50                push ax
00002077  B87D1C            mov ax,0x1c7d
0000207A  99                cwd
0000207B  52                push dx
0000207C  50                push ax
0000207D  8D5EF0            lea bx,[bp-0x10]
00002080  1E                push ds
00002081  07                pop es
00002082  06                push es
00002083  53                push bx
00002084  B80200            mov ax,0x2
00002087  50                push ax
00002088  9AC1005802        call word 0x258:word 0xc1
0000208D  B80100            mov ax,0x1
00002090  50                push ax
00002091  B87F1C            mov ax,0x1c7f
00002094  99                cwd
00002095  52                push dx
00002096  50                push ax
00002097  8D5EE6            lea bx,[bp-0x1a]
0000209A  1E                push ds
0000209B  07                pop es
0000209C  06                push es
0000209D  53                push bx
0000209E  B80200            mov ax,0x2
000020A1  50                push ax
000020A2  9AC1005802        call word 0x258:word 0xc1
000020A7  B80100            mov ax,0x1
000020AA  50                push ax
000020AB  B8801C            mov ax,0x1c80
000020AE  99                cwd
000020AF  52                push dx
000020B0  50                push ax
000020B1  8D5EE4            lea bx,[bp-0x1c]
000020B4  1E                push ds
000020B5  07                pop es
000020B6  06                push es
000020B7  53                push bx
000020B8  B80200            mov ax,0x2
000020BB  50                push ax
000020BC  9AC1005802        call word 0x258:word 0xc1
000020C1  B80100            mov ax,0x1
000020C4  50                push ax
000020C5  B8831C            mov ax,0x1c83
000020C8  99                cwd
000020C9  52                push dx
000020CA  50                push ax
000020CB  8D5EE8            lea bx,[bp-0x18]
000020CE  1E                push ds
000020CF  07                pop es
000020D0  06                push es
000020D1  53                push bx
000020D2  B80200            mov ax,0x2
000020D5  50                push ax
000020D6  9AC1005802        call word 0x258:word 0xc1
000020DB  B80100            mov ax,0x1
000020DE  50                push ax
000020DF  B8841C            mov ax,0x1c84
000020E2  99                cwd
000020E3  52                push dx
000020E4  50                push ax
000020E5  8D5EEE            lea bx,[bp-0x12]
000020E8  1E                push ds
000020E9  07                pop es
000020EA  06                push es
000020EB  53                push bx
000020EC  B80200            mov ax,0x2
000020EF  50                push ax
000020F0  9AC1005802        call word 0x258:word 0xc1
000020F5  B80100            mov ax,0x1
000020F8  50                push ax
000020F9  B8851C            mov ax,0x1c85
000020FC  99                cwd
000020FD  52                push dx
000020FE  50                push ax
000020FF  8D5EEC            lea bx,[bp-0x14]
00002102  1E                push ds
00002103  07                pop es
00002104  06                push es
00002105  53                push bx
00002106  B80200            mov ax,0x2
00002109  50                push ax
0000210A  9AC1005802        call word 0x258:word 0xc1
0000210F  B80100            mov ax,0x1
00002112  50                push ax
00002113  B8871C            mov ax,0x1c87
00002116  99                cwd
00002117  52                push dx
00002118  50                push ax
00002119  8D5EEA            lea bx,[bp-0x16]
0000211C  1E                push ds
0000211D  07                pop es
0000211E  06                push es
0000211F  53                push bx
00002120  B80200            mov ax,0x2
00002123  50                push ax
00002124  9AC1005802        call word 0x258:word 0xc1
00002129  B80100            mov ax,0x1
0000212C  50                push ax
0000212D  B8891C            mov ax,0x1c89
00002130  99                cwd
00002131  52                push dx
00002132  50                push ax
00002133  8D5EF0            lea bx,[bp-0x10]
00002136  1E                push ds
00002137  07                pop es
00002138  06                push es
00002139  53                push bx
0000213A  B80200            mov ax,0x2
0000213D  50                push ax
0000213E  9AC1005802        call word 0x258:word 0xc1
00002143  B80100            mov ax,0x1
00002146  50                push ax
00002147  B88B1C            mov ax,0x1c8b
0000214A  99                cwd
0000214B  52                push dx
0000214C  50                push ax
0000214D  8D5EE6            lea bx,[bp-0x1a]
00002150  1E                push ds
00002151  07                pop es
00002152  06                push es
00002153  53                push bx
00002154  B80200            mov ax,0x2
00002157  50                push ax
00002158  9AC1005802        call word 0x258:word 0xc1
0000215D  B80100            mov ax,0x1
00002160  50                push ax
00002161  B88C1C            mov ax,0x1c8c
00002164  99                cwd
00002165  52                push dx
00002166  50                push ax
00002167  8D5EE4            lea bx,[bp-0x1c]
0000216A  1E                push ds
0000216B  07                pop es
0000216C  06                push es
0000216D  53                push bx
0000216E  B80200            mov ax,0x2
00002171  50                push ax
00002172  9AC1005802        call word 0x258:word 0xc1
00002177  B80100            mov ax,0x1
0000217A  50                push ax
0000217B  B88F1C            mov ax,0x1c8f
0000217E  99                cwd
0000217F  52                push dx
00002180  50                push ax
00002181  8D5EE8            lea bx,[bp-0x18]
00002184  1E                push ds
00002185  07                pop es
00002186  06                push es
00002187  53                push bx
00002188  B80200            mov ax,0x2
0000218B  50                push ax
0000218C  9AC1005802        call word 0x258:word 0xc1
00002191  B80100            mov ax,0x1
00002194  50                push ax
00002195  B8901C            mov ax,0x1c90
00002198  99                cwd
00002199  52                push dx
0000219A  50                push ax
0000219B  8D5EEE            lea bx,[bp-0x12]
0000219E  1E                push ds
0000219F  07                pop es
000021A0  06                push es
000021A1  53                push bx
000021A2  B80200            mov ax,0x2
000021A5  50                push ax
000021A6  9AC1005802        call word 0x258:word 0xc1
000021AB  B80100            mov ax,0x1
000021AE  50                push ax
000021AF  B8911C            mov ax,0x1c91
000021B2  99                cwd
000021B3  52                push dx
000021B4  50                push ax
000021B5  8D5EEC            lea bx,[bp-0x14]
000021B8  1E                push ds
000021B9  07                pop es
000021BA  06                push es
000021BB  53                push bx
000021BC  B80200            mov ax,0x2
000021BF  50                push ax
000021C0  9AC1005802        call word 0x258:word 0xc1
000021C5  B80100            mov ax,0x1
000021C8  50                push ax
000021C9  B8931C            mov ax,0x1c93
000021CC  99                cwd
000021CD  52                push dx
000021CE  50                push ax
000021CF  8D5EEA            lea bx,[bp-0x16]
000021D2  1E                push ds
000021D3  07                pop es
000021D4  06                push es
000021D5  53                push bx
000021D6  B80200            mov ax,0x2
000021D9  50                push ax
000021DA  9AC1005802        call word 0x258:word 0xc1
000021DF  B80100            mov ax,0x1
000021E2  50                push ax
000021E3  B8951C            mov ax,0x1c95
000021E6  99                cwd
000021E7  52                push dx
000021E8  50                push ax
000021E9  8D5EF0            lea bx,[bp-0x10]
000021EC  1E                push ds
000021ED  07                pop es
000021EE  06                push es
000021EF  53                push bx
000021F0  B80200            mov ax,0x2
000021F3  50                push ax
000021F4  9AC1005802        call word 0x258:word 0xc1
000021F9  B80100            mov ax,0x1
000021FC  50                push ax
000021FD  B8971C            mov ax,0x1c97
00002200  99                cwd
00002201  52                push dx
00002202  50                push ax
00002203  8D5EE6            lea bx,[bp-0x1a]
00002206  1E                push ds
00002207  07                pop es
00002208  06                push es
00002209  53                push bx
0000220A  B80200            mov ax,0x2
0000220D  50                push ax
0000220E  9AC1005802        call word 0x258:word 0xc1
00002213  B80100            mov ax,0x1
00002216  50                push ax
00002217  B8981C            mov ax,0x1c98
0000221A  99                cwd
0000221B  52                push dx
0000221C  50                push ax
0000221D  8D5EE4            lea bx,[bp-0x1c]
00002220  1E                push ds
00002221  07                pop es
00002222  06                push es
00002223  53                push bx
00002224  B80200            mov ax,0x2
00002227  50                push ax
00002228  9AC1005802        call word 0x258:word 0xc1
0000222D  B80100            mov ax,0x1
00002230  50                push ax
00002231  B89B1C            mov ax,0x1c9b
00002234  99                cwd
00002235  52                push dx
00002236  50                push ax
00002237  8D5EE8            lea bx,[bp-0x18]
0000223A  1E                push ds
0000223B  07                pop es
0000223C  06                push es
0000223D  53                push bx
0000223E  B80200            mov ax,0x2
00002241  50                push ax
00002242  9AC1005802        call word 0x258:word 0xc1
00002247  B80100            mov ax,0x1
0000224A  50                push ax
0000224B  B89C1C            mov ax,0x1c9c
0000224E  99                cwd
0000224F  52                push dx
00002250  50                push ax
00002251  8D5EEE            lea bx,[bp-0x12]
00002254  1E                push ds
00002255  07                pop es
00002256  06                push es
00002257  53                push bx
00002258  B80200            mov ax,0x2
0000225B  50                push ax
0000225C  9AC1005802        call word 0x258:word 0xc1
00002261  B80100            mov ax,0x1
00002264  50                push ax
00002265  B89D1C            mov ax,0x1c9d
00002268  99                cwd
00002269  52                push dx
0000226A  50                push ax
0000226B  8D5EEC            lea bx,[bp-0x14]
0000226E  1E                push ds
0000226F  07                pop es
00002270  06                push es
00002271  53                push bx
00002272  B80200            mov ax,0x2
00002275  50                push ax
00002276  9AC1005802        call word 0x258:word 0xc1
0000227B  B80100            mov ax,0x1
0000227E  50                push ax
0000227F  B89F1C            mov ax,0x1c9f
00002282  99                cwd
00002283  52                push dx
00002284  50                push ax
00002285  8D5EEA            lea bx,[bp-0x16]
00002288  1E                push ds
00002289  07                pop es
0000228A  06                push es
0000228B  53                push bx
0000228C  B80200            mov ax,0x2
0000228F  50                push ax
00002290  9AC1005802        call word 0x258:word 0xc1
00002295  B80100            mov ax,0x1
00002298  50                push ax
00002299  B8A11C            mov ax,0x1ca1
0000229C  99                cwd
0000229D  52                push dx
0000229E  50                push ax
0000229F  8D5EF0            lea bx,[bp-0x10]
000022A2  1E                push ds
000022A3  07                pop es
000022A4  06                push es
000022A5  53                push bx
000022A6  B80200            mov ax,0x2
000022A9  50                push ax
000022AA  9AC1005802        call word 0x258:word 0xc1
000022AF  B80100            mov ax,0x1
000022B2  50                push ax
000022B3  B8A31C            mov ax,0x1ca3
000022B6  99                cwd
000022B7  52                push dx
000022B8  50                push ax
000022B9  8D5EE6            lea bx,[bp-0x1a]
000022BC  1E                push ds
000022BD  07                pop es
000022BE  06                push es
000022BF  53                push bx
000022C0  B80200            mov ax,0x2
000022C3  50                push ax
000022C4  9AC1005802        call word 0x258:word 0xc1
000022C9  B80100            mov ax,0x1
000022CC  50                push ax
000022CD  B8A41C            mov ax,0x1ca4
000022D0  99                cwd
000022D1  52                push dx
000022D2  50                push ax
000022D3  8D5EE4            lea bx,[bp-0x1c]
000022D6  1E                push ds
000022D7  07                pop es
000022D8  06                push es
000022D9  53                push bx
000022DA  B80200            mov ax,0x2
000022DD  50                push ax
000022DE  9AC1005802        call word 0x258:word 0xc1
000022E3  B80100            mov ax,0x1
000022E6  50                push ax
000022E7  B8A71C            mov ax,0x1ca7
000022EA  99                cwd
000022EB  52                push dx
000022EC  50                push ax
000022ED  8D5EE8            lea bx,[bp-0x18]
000022F0  1E                push ds
000022F1  07                pop es
000022F2  06                push es
000022F3  53                push bx
000022F4  B80200            mov ax,0x2
000022F7  50                push ax
000022F8  9AC1005802        call word 0x258:word 0xc1
000022FD  B80100            mov ax,0x1
00002300  50                push ax
00002301  B8A81C            mov ax,0x1ca8
00002304  99                cwd
00002305  52                push dx
00002306  50                push ax
00002307  8D5EEE            lea bx,[bp-0x12]
0000230A  1E                push ds
0000230B  07                pop es
0000230C  06                push es
0000230D  53                push bx
0000230E  B80200            mov ax,0x2
00002311  50                push ax
00002312  9AC1005802        call word 0x258:word 0xc1
00002317  B80100            mov ax,0x1
0000231A  50                push ax
0000231B  B8A91C            mov ax,0x1ca9
0000231E  99                cwd
0000231F  52                push dx
00002320  50                push ax
00002321  8D5EEC            lea bx,[bp-0x14]
00002324  1E                push ds
00002325  07                pop es
00002326  06                push es
00002327  53                push bx
00002328  B80200            mov ax,0x2
0000232B  50                push ax
0000232C  9AC1005802        call word 0x258:word 0xc1
00002331  B80100            mov ax,0x1
00002334  50                push ax
00002335  B8AB1C            mov ax,0x1cab
00002338  99                cwd
00002339  52                push dx
0000233A  50                push ax
0000233B  8D5EEA            lea bx,[bp-0x16]
0000233E  1E                push ds
0000233F  07                pop es
00002340  06                push es
00002341  53                push bx
00002342  B80200            mov ax,0x2
00002345  50                push ax
00002346  9AC1005802        call word 0x258:word 0xc1
0000234B  B80100            mov ax,0x1
0000234E  50                push ax
0000234F  B8AD1C            mov ax,0x1cad
00002352  99                cwd
00002353  52                push dx
00002354  50                push ax
00002355  8D5EF0            lea bx,[bp-0x10]
00002358  1E                push ds
00002359  07                pop es
0000235A  06                push es
0000235B  53                push bx
0000235C  B80200            mov ax,0x2
0000235F  50                push ax
00002360  9AC1005802        call word 0x258:word 0xc1
00002365  B80100            mov ax,0x1
00002368  50                push ax
00002369  B8AF1C            mov ax,0x1caf
0000236C  99                cwd
0000236D  52                push dx
0000236E  50                push ax
0000236F  8D5EE6            lea bx,[bp-0x1a]
00002372  1E                push ds
00002373  07                pop es
00002374  06                push es
00002375  53                push bx
00002376  B80200            mov ax,0x2
00002379  50                push ax
0000237A  9AC1005802        call word 0x258:word 0xc1
0000237F  B80100            mov ax,0x1
00002382  50                push ax
00002383  B8B01C            mov ax,0x1cb0
00002386  99                cwd
00002387  52                push dx
00002388  50                push ax
00002389  8D5EE4            lea bx,[bp-0x1c]
0000238C  1E                push ds
0000238D  07                pop es
0000238E  06                push es
0000238F  53                push bx
00002390  B80200            mov ax,0x2
00002393  50                push ax
00002394  9AC1005802        call word 0x258:word 0xc1
00002399  B80100            mov ax,0x1
0000239C  50                push ax
0000239D  B8B31C            mov ax,0x1cb3
000023A0  99                cwd
000023A1  52                push dx
000023A2  50                push ax
000023A3  8D5EE8            lea bx,[bp-0x18]
000023A6  1E                push ds
000023A7  07                pop es
000023A8  06                push es
000023A9  53                push bx
000023AA  B80200            mov ax,0x2
000023AD  50                push ax
000023AE  9AC1005802        call word 0x258:word 0xc1
000023B3  B80100            mov ax,0x1
000023B6  50                push ax
000023B7  B8B41C            mov ax,0x1cb4
000023BA  99                cwd
000023BB  52                push dx
000023BC  50                push ax
000023BD  8D5EEE            lea bx,[bp-0x12]
000023C0  1E                push ds
000023C1  07                pop es
000023C2  06                push es
000023C3  53                push bx
000023C4  B80200            mov ax,0x2
000023C7  50                push ax
000023C8  9AC1005802        call word 0x258:word 0xc1
000023CD  B80100            mov ax,0x1
000023D0  50                push ax
000023D1  B8B51C            mov ax,0x1cb5
000023D4  99                cwd
000023D5  52                push dx
000023D6  50                push ax
000023D7  8D5EEC            lea bx,[bp-0x14]
000023DA  1E                push ds
000023DB  07                pop es
000023DC  06                push es
000023DD  53                push bx
000023DE  B80200            mov ax,0x2
000023E1  50                push ax
000023E2  9AC1005802        call word 0x258:word 0xc1
000023E7  B80100            mov ax,0x1
000023EA  50                push ax
000023EB  B8B71C            mov ax,0x1cb7
000023EE  99                cwd
000023EF  52                push dx
000023F0  50                push ax
000023F1  8D5EEA            lea bx,[bp-0x16]
000023F4  1E                push ds
000023F5  07                pop es
000023F6  06                push es
000023F7  53                push bx
000023F8  B80200            mov ax,0x2
000023FB  50                push ax
000023FC  9AC1005802        call word 0x258:word 0xc1
00002401  B80100            mov ax,0x1
00002404  50                push ax
00002405  B8B91C            mov ax,0x1cb9
00002408  99                cwd
00002409  52                push dx
0000240A  50                push ax
0000240B  8D5EF0            lea bx,[bp-0x10]
0000240E  1E                push ds
0000240F  07                pop es
00002410  06                push es
00002411  53                push bx
00002412  B80200            mov ax,0x2
00002415  50                push ax
00002416  9AC1005802        call word 0x258:word 0xc1
0000241B  B80100            mov ax,0x1
0000241E  50                push ax
0000241F  B8BB1C            mov ax,0x1cbb
00002422  99                cwd
00002423  52                push dx
00002424  50                push ax
00002425  8D5EE6            lea bx,[bp-0x1a]
00002428  1E                push ds
00002429  07                pop es
0000242A  06                push es
0000242B  53                push bx
0000242C  B80200            mov ax,0x2
0000242F  50                push ax
00002430  9AC1005802        call word 0x258:word 0xc1
00002435  B80100            mov ax,0x1
00002438  50                push ax
00002439  B8BC1C            mov ax,0x1cbc
0000243C  99                cwd
0000243D  52                push dx
0000243E  50                push ax
0000243F  8D5EE4            lea bx,[bp-0x1c]
00002442  1E                push ds
00002443  07                pop es
00002444  06                push es
00002445  53                push bx
00002446  B80200            mov ax,0x2
00002449  50                push ax
0000244A  9AC1005802        call word 0x258:word 0xc1
0000244F  B80100            mov ax,0x1
00002452  50                push ax
00002453  B8BF1C            mov ax,0x1cbf
00002456  99                cwd
00002457  52                push dx
00002458  50                push ax
00002459  8D5EE8            lea bx,[bp-0x18]
0000245C  1E                push ds
0000245D  07                pop es
0000245E  06                push es
0000245F  53                push bx
00002460  B80200            mov ax,0x2
00002463  50                push ax
00002464  9AC1005802        call word 0x258:word 0xc1
00002469  B80100            mov ax,0x1
0000246C  50                push ax
0000246D  B8C01C            mov ax,0x1cc0
00002470  99                cwd
00002471  52                push dx
00002472  50                push ax
00002473  8D5EEE            lea bx,[bp-0x12]
00002476  1E                push ds
00002477  07                pop es
00002478  06                push es
00002479  53                push bx
0000247A  B80200            mov ax,0x2
0000247D  50                push ax
0000247E  9AC1005802        call word 0x258:word 0xc1
00002483  B80100            mov ax,0x1
00002486  50                push ax
00002487  B8C11C            mov ax,0x1cc1
0000248A  99                cwd
0000248B  52                push dx
0000248C  50                push ax
0000248D  8D5EEC            lea bx,[bp-0x14]
00002490  1E                push ds
00002491  07                pop es
00002492  06                push es
00002493  53                push bx
00002494  B80200            mov ax,0x2
00002497  50                push ax
00002498  9AC1005802        call word 0x258:word 0xc1
0000249D  B80100            mov ax,0x1
000024A0  50                push ax
000024A1  B8C31C            mov ax,0x1cc3
000024A4  99                cwd
000024A5  52                push dx
000024A6  50                push ax
000024A7  8D5EEA            lea bx,[bp-0x16]
000024AA  1E                push ds
000024AB  07                pop es
000024AC  06                push es
000024AD  53                push bx
000024AE  B80200            mov ax,0x2
000024B1  50                push ax
000024B2  9AC1005802        call word 0x258:word 0xc1
000024B7  B80100            mov ax,0x1
000024BA  50                push ax
000024BB  B8C51C            mov ax,0x1cc5
000024BE  99                cwd
000024BF  52                push dx
000024C0  50                push ax
000024C1  8D5EF0            lea bx,[bp-0x10]
000024C4  1E                push ds
000024C5  07                pop es
000024C6  06                push es
000024C7  53                push bx
000024C8  B80200            mov ax,0x2
000024CB  50                push ax
000024CC  9AC1005802        call word 0x258:word 0xc1
000024D1  B80100            mov ax,0x1
000024D4  50                push ax
000024D5  B8C71C            mov ax,0x1cc7
000024D8  99                cwd
000024D9  52                push dx
000024DA  50                push ax
000024DB  8D5EE6            lea bx,[bp-0x1a]
000024DE  1E                push ds
000024DF  07                pop es
000024E0  06                push es
000024E1  53                push bx
000024E2  B80200            mov ax,0x2
000024E5  50                push ax
000024E6  9AC1005802        call word 0x258:word 0xc1
000024EB  B80100            mov ax,0x1
000024EE  50                push ax
000024EF  B8C81C            mov ax,0x1cc8
000024F2  99                cwd
000024F3  52                push dx
000024F4  50                push ax
000024F5  8D5EE4            lea bx,[bp-0x1c]
000024F8  1E                push ds
000024F9  07                pop es
000024FA  06                push es
000024FB  53                push bx
000024FC  B80200            mov ax,0x2
000024FF  50                push ax
00002500  9AC1005802        call word 0x258:word 0xc1
00002505  B80100            mov ax,0x1
00002508  50                push ax
00002509  B8CB1C            mov ax,0x1ccb
0000250C  99                cwd
0000250D  52                push dx
0000250E  50                push ax
0000250F  8D5EE8            lea bx,[bp-0x18]
00002512  1E                push ds
00002513  07                pop es
00002514  06                push es
00002515  53                push bx
00002516  B80200            mov ax,0x2
00002519  50                push ax
0000251A  9AC1005802        call word 0x258:word 0xc1
0000251F  B80100            mov ax,0x1
00002522  50                push ax
00002523  B8CC1C            mov ax,0x1ccc
00002526  99                cwd
00002527  52                push dx
00002528  50                push ax
00002529  8D5EEE            lea bx,[bp-0x12]
0000252C  1E                push ds
0000252D  07                pop es
0000252E  06                push es
0000252F  53                push bx
00002530  B80200            mov ax,0x2
00002533  50                push ax
00002534  9AC1005802        call word 0x258:word 0xc1
00002539  B80100            mov ax,0x1
0000253C  50                push ax
0000253D  B8CD1C            mov ax,0x1ccd
00002540  99                cwd
00002541  52                push dx
00002542  50                push ax
00002543  8D5EEC            lea bx,[bp-0x14]
00002546  1E                push ds
00002547  07                pop es
00002548  06                push es
00002549  53                push bx
0000254A  B80200            mov ax,0x2
0000254D  50                push ax
0000254E  9AC1005802        call word 0x258:word 0xc1
00002553  B80100            mov ax,0x1
00002556  50                push ax
00002557  B8CF1C            mov ax,0x1ccf
0000255A  99                cwd
0000255B  52                push dx
0000255C  50                push ax
0000255D  8D5EEA            lea bx,[bp-0x16]
00002560  1E                push ds
00002561  07                pop es
00002562  06                push es
00002563  53                push bx
00002564  B80200            mov ax,0x2
00002567  50                push ax
00002568  9AC1005802        call word 0x258:word 0xc1
0000256D  B80100            mov ax,0x1
00002570  50                push ax
00002571  B8D11C            mov ax,0x1cd1
00002574  99                cwd
00002575  52                push dx
00002576  50                push ax
00002577  8D5EF0            lea bx,[bp-0x10]
0000257A  1E                push ds
0000257B  07                pop es
0000257C  06                push es
0000257D  53                push bx
0000257E  B80200            mov ax,0x2
00002581  50                push ax
00002582  9AC1005802        call word 0x258:word 0xc1
00002587  B80100            mov ax,0x1
0000258A  50                push ax
0000258B  B8D31C            mov ax,0x1cd3
0000258E  99                cwd
0000258F  52                push dx
00002590  50                push ax
00002591  8D5EE6            lea bx,[bp-0x1a]
00002594  1E                push ds
00002595  07                pop es
00002596  06                push es
00002597  53                push bx
00002598  B80200            mov ax,0x2
0000259B  50                push ax
0000259C  9AC1005802        call word 0x258:word 0xc1
000025A1  B80100            mov ax,0x1
000025A4  50                push ax
000025A5  B8D41C            mov ax,0x1cd4
000025A8  99                cwd
000025A9  52                push dx
000025AA  50                push ax
000025AB  8D5EE4            lea bx,[bp-0x1c]
000025AE  1E                push ds
000025AF  07                pop es
000025B0  06                push es
000025B1  53                push bx
000025B2  B80200            mov ax,0x2
000025B5  50                push ax
000025B6  9AC1005802        call word 0x258:word 0xc1
000025BB  B80100            mov ax,0x1
000025BE  50                push ax
000025BF  B8D71C            mov ax,0x1cd7
000025C2  99                cwd
000025C3  52                push dx
000025C4  50                push ax
000025C5  8D5EE8            lea bx,[bp-0x18]
000025C8  1E                push ds
000025C9  07                pop es
000025CA  06                push es
000025CB  53                push bx
000025CC  B80200            mov ax,0x2
000025CF  50                push ax
000025D0  9AC1005802        call word 0x258:word 0xc1
000025D5  B80100            mov ax,0x1
000025D8  50                push ax
000025D9  B8D81C            mov ax,0x1cd8
000025DC  99                cwd
000025DD  52                push dx
000025DE  50                push ax
000025DF  8D5EEE            lea bx,[bp-0x12]
000025E2  1E                push ds
000025E3  07                pop es
000025E4  06                push es
000025E5  53                push bx
000025E6  B80200            mov ax,0x2
000025E9  50                push ax
000025EA  9AC1005802        call word 0x258:word 0xc1
000025EF  B80100            mov ax,0x1
000025F2  50                push ax
000025F3  B8D91C            mov ax,0x1cd9
000025F6  99                cwd
000025F7  52                push dx
000025F8  50                push ax
000025F9  8D5EEC            lea bx,[bp-0x14]
000025FC  1E                push ds
000025FD  07                pop es
000025FE  06                push es
000025FF  53                push bx
00002600  B80200            mov ax,0x2
00002603  50                push ax
00002604  9AC1005802        call word 0x258:word 0xc1
00002609  B80100            mov ax,0x1
0000260C  50                push ax
0000260D  B8DB1C            mov ax,0x1cdb
00002610  99                cwd
00002611  52                push dx
00002612  50                push ax
00002613  8D5EEA            lea bx,[bp-0x16]
00002616  1E                push ds
00002617  07                pop es
00002618  06                push es
00002619  53                push bx
0000261A  B80200            mov ax,0x2
0000261D  50                push ax
0000261E  9AC1005802        call word 0x258:word 0xc1
00002623  B80100            mov ax,0x1
00002626  50                push ax
00002627  B8DD1C            mov ax,0x1cdd
0000262A  99                cwd
0000262B  52                push dx
0000262C  50                push ax
0000262D  8D5EF0            lea bx,[bp-0x10]
00002630  1E                push ds
00002631  07                pop es
00002632  06                push es
00002633  53                push bx
00002634  B80200            mov ax,0x2
00002637  50                push ax
00002638  9AC1005802        call word 0x258:word 0xc1
0000263D  B80100            mov ax,0x1
00002640  50                push ax
00002641  B8DF1C            mov ax,0x1cdf
00002644  99                cwd
00002645  52                push dx
00002646  50                push ax
00002647  8D5EE6            lea bx,[bp-0x1a]
0000264A  1E                push ds
0000264B  07                pop es
0000264C  06                push es
0000264D  53                push bx
0000264E  B80200            mov ax,0x2
00002651  50                push ax
00002652  9AC1005802        call word 0x258:word 0xc1
00002657  B80100            mov ax,0x1
0000265A  50                push ax
0000265B  B8E01C            mov ax,0x1ce0
0000265E  99                cwd
0000265F  52                push dx
00002660  50                push ax
00002661  8D5EE4            lea bx,[bp-0x1c]
00002664  1E                push ds
00002665  07                pop es
00002666  06                push es
00002667  53                push bx
00002668  B80200            mov ax,0x2
0000266B  50                push ax
0000266C  9AC1005802        call word 0x258:word 0xc1
00002671  B80100            mov ax,0x1
00002674  50                push ax
00002675  B8E31C            mov ax,0x1ce3
00002678  99                cwd
00002679  52                push dx
0000267A  50                push ax
0000267B  8D5EE8            lea bx,[bp-0x18]
0000267E  1E                push ds
0000267F  07                pop es
00002680  06                push es
00002681  53                push bx
00002682  B80200            mov ax,0x2
00002685  50                push ax
00002686  9AC1005802        call word 0x258:word 0xc1
0000268B  B80100            mov ax,0x1
0000268E  50                push ax
0000268F  B8E41C            mov ax,0x1ce4
00002692  99                cwd
00002693  52                push dx
00002694  50                push ax
00002695  8D5EEE            lea bx,[bp-0x12]
00002698  1E                push ds
00002699  07                pop es
0000269A  06                push es
0000269B  53                push bx
0000269C  B80200            mov ax,0x2
0000269F  50                push ax
000026A0  9AC1005802        call word 0x258:word 0xc1
000026A5  B80100            mov ax,0x1
000026A8  50                push ax
000026A9  B8E51C            mov ax,0x1ce5
000026AC  99                cwd
000026AD  52                push dx
000026AE  50                push ax
000026AF  8D5EEC            lea bx,[bp-0x14]
000026B2  1E                push ds
000026B3  07                pop es
000026B4  06                push es
000026B5  53                push bx
000026B6  B80200            mov ax,0x2
000026B9  50                push ax
000026BA  9AC1005802        call word 0x258:word 0xc1
000026BF  B80100            mov ax,0x1
000026C2  50                push ax
000026C3  B8E71C            mov ax,0x1ce7
000026C6  99                cwd
000026C7  52                push dx
000026C8  50                push ax
000026C9  8D5EEA            lea bx,[bp-0x16]
000026CC  1E                push ds
000026CD  07                pop es
000026CE  06                push es
000026CF  53                push bx
000026D0  B80200            mov ax,0x2
000026D3  50                push ax
000026D4  9AC1005802        call word 0x258:word 0xc1
000026D9  B80100            mov ax,0x1
000026DC  50                push ax
000026DD  B8E91C            mov ax,0x1ce9
000026E0  99                cwd
000026E1  52                push dx
000026E2  50                push ax
000026E3  8D5EF0            lea bx,[bp-0x10]
000026E6  1E                push ds
000026E7  07                pop es
000026E8  06                push es
000026E9  53                push bx
000026EA  B80200            mov ax,0x2
000026ED  50                push ax
000026EE  9AC1005802        call word 0x258:word 0xc1
000026F3  B80100            mov ax,0x1
000026F6  50                push ax
000026F7  B8EB1C            mov ax,0x1ceb
000026FA  99                cwd
000026FB  52                push dx
000026FC  50                push ax
000026FD  8D5EE6            lea bx,[bp-0x1a]
00002700  1E                push ds
00002701  07                pop es
00002702  06                push es
00002703  53                push bx
00002704  B80200            mov ax,0x2
00002707  50                push ax
00002708  9AC1005802        call word 0x258:word 0xc1
0000270D  B80100            mov ax,0x1
00002710  50                push ax
00002711  B8EC1C            mov ax,0x1cec
00002714  99                cwd
00002715  52                push dx
00002716  50                push ax
00002717  8D5EE4            lea bx,[bp-0x1c]
0000271A  1E                push ds
0000271B  07                pop es
0000271C  06                push es
0000271D  53                push bx
0000271E  B80200            mov ax,0x2
00002721  50                push ax
00002722  9AC1005802        call word 0x258:word 0xc1
00002727  B80100            mov ax,0x1
0000272A  50                push ax
0000272B  B8EF1C            mov ax,0x1cef
0000272E  99                cwd
0000272F  52                push dx
00002730  50                push ax
00002731  8D5EE8            lea bx,[bp-0x18]
00002734  1E                push ds
00002735  07                pop es
00002736  06                push es
00002737  53                push bx
00002738  B80200            mov ax,0x2
0000273B  50                push ax
0000273C  9AC1005802        call word 0x258:word 0xc1
00002741  B80100            mov ax,0x1
00002744  50                push ax
00002745  B8F01C            mov ax,0x1cf0
00002748  99                cwd
00002749  52                push dx
0000274A  50                push ax
0000274B  8D5EEE            lea bx,[bp-0x12]
0000274E  1E                push ds
0000274F  07                pop es
00002750  06                push es
00002751  53                push bx
00002752  B80200            mov ax,0x2
00002755  50                push ax
00002756  9AC1005802        call word 0x258:word 0xc1
0000275B  B80100            mov ax,0x1
0000275E  50                push ax
0000275F  B8F11C            mov ax,0x1cf1
00002762  99                cwd
00002763  52                push dx
00002764  50                push ax
00002765  8D5EEC            lea bx,[bp-0x14]
00002768  1E                push ds
00002769  07                pop es
0000276A  06                push es
0000276B  53                push bx
0000276C  B80200            mov ax,0x2
0000276F  50                push ax
00002770  9AC1005802        call word 0x258:word 0xc1
00002775  B80100            mov ax,0x1
00002778  50                push ax
00002779  B8F31C            mov ax,0x1cf3
0000277C  99                cwd
0000277D  52                push dx
0000277E  50                push ax
0000277F  8D5EEA            lea bx,[bp-0x16]
00002782  1E                push ds
00002783  07                pop es
00002784  06                push es
00002785  53                push bx
00002786  B80200            mov ax,0x2
00002789  50                push ax
0000278A  9AC1005802        call word 0x258:word 0xc1
0000278F  B80100            mov ax,0x1
00002792  50                push ax
00002793  B8F51C            mov ax,0x1cf5
00002796  99                cwd
00002797  52                push dx
00002798  50                push ax
00002799  8D5EF0            lea bx,[bp-0x10]
0000279C  1E                push ds
0000279D  07                pop es
0000279E  06                push es
0000279F  53                push bx
000027A0  B80200            mov ax,0x2
000027A3  50                push ax
000027A4  9AC1005802        call word 0x258:word 0xc1
000027A9  B80100            mov ax,0x1
000027AC  50                push ax
000027AD  B8F71C            mov ax,0x1cf7
000027B0  99                cwd
000027B1  52                push dx
000027B2  50                push ax
000027B3  8D5EE6            lea bx,[bp-0x1a]
000027B6  1E                push ds
000027B7  07                pop es
000027B8  06                push es
000027B9  53                push bx
000027BA  B80200            mov ax,0x2
000027BD  50                push ax
000027BE  9AC1005802        call word 0x258:word 0xc1
000027C3  B80100            mov ax,0x1
000027C6  50                push ax
000027C7  B8F81C            mov ax,0x1cf8
000027CA  99                cwd
000027CB  52                push dx
000027CC  50                push ax
000027CD  8D5EE4            lea bx,[bp-0x1c]
000027D0  1E                push ds
000027D1  07                pop es
000027D2  06                push es
000027D3  53                push bx
000027D4  B80200            mov ax,0x2
000027D7  50                push ax
000027D8  9AC1005802        call word 0x258:word 0xc1
000027DD  B80100            mov ax,0x1
000027E0  50                push ax
000027E1  B8FB1C            mov ax,0x1cfb
000027E4  99                cwd
000027E5  52                push dx
000027E6  50                push ax
000027E7  8D5EE8            lea bx,[bp-0x18]
000027EA  1E                push ds
000027EB  07                pop es
000027EC  06                push es
000027ED  53                push bx
000027EE  B80200            mov ax,0x2
000027F1  50                push ax
000027F2  9AC1005802        call word 0x258:word 0xc1
000027F7  B80100            mov ax,0x1
000027FA  50                push ax
000027FB  B8FC1C            mov ax,0x1cfc
000027FE  99                cwd
000027FF  52                push dx
00002800  50                push ax
00002801  8D5EEE            lea bx,[bp-0x12]
00002804  1E                push ds
00002805  07                pop es
00002806  06                push es
00002807  53                push bx
00002808  B80200            mov ax,0x2
0000280B  50                push ax
0000280C  9AC1005802        call word 0x258:word 0xc1
00002811  B80100            mov ax,0x1
00002814  50                push ax
00002815  B8FD1C            mov ax,0x1cfd
00002818  99                cwd
00002819  52                push dx
0000281A  50                push ax
0000281B  8D5EEC            lea bx,[bp-0x14]
0000281E  1E                push ds
0000281F  07                pop es
00002820  06                push es
00002821  53                push bx
00002822  B80200            mov ax,0x2
00002825  50                push ax
00002826  9AC1005802        call word 0x258:word 0xc1
0000282B  B80100            mov ax,0x1
0000282E  50                push ax
0000282F  B8FF1C            mov ax,0x1cff
00002832  99                cwd
00002833  52                push dx
00002834  50                push ax
00002835  8D5EEA            lea bx,[bp-0x16]
00002838  1E                push ds
00002839  07                pop es
0000283A  06                push es
0000283B  53                push bx
0000283C  B80200            mov ax,0x2
0000283F  50                push ax
00002840  9AC1005802        call word 0x258:word 0xc1
00002845  B80100            mov ax,0x1
00002848  50                push ax
00002849  B8011D            mov ax,0x1d01
0000284C  99                cwd
0000284D  52                push dx
0000284E  50                push ax
0000284F  8D5EF0            lea bx,[bp-0x10]
00002852  1E                push ds
00002853  07                pop es
00002854  06                push es
00002855  53                push bx
00002856  B80200            mov ax,0x2
00002859  50                push ax
0000285A  9AC1005802        call word 0x258:word 0xc1
0000285F  B80100            mov ax,0x1
00002862  50                push ax
00002863  B8031D            mov ax,0x1d03
00002866  99                cwd
00002867  52                push dx
00002868  50                push ax
00002869  8D5EE6            lea bx,[bp-0x1a]
0000286C  1E                push ds
0000286D  07                pop es
0000286E  06                push es
0000286F  53                push bx
00002870  B80200            mov ax,0x2
00002873  50                push ax
00002874  9AC1005802        call word 0x258:word 0xc1
00002879  B80100            mov ax,0x1
0000287C  50                push ax
0000287D  B8041D            mov ax,0x1d04
00002880  99                cwd
00002881  52                push dx
00002882  50                push ax
00002883  8D5EE4            lea bx,[bp-0x1c]
00002886  1E                push ds
00002887  07                pop es
00002888  06                push es
00002889  53                push bx
0000288A  B80200            mov ax,0x2
0000288D  50                push ax
0000288E  9AC1005802        call word 0x258:word 0xc1
00002893  B80100            mov ax,0x1
00002896  50                push ax
00002897  B8071D            mov ax,0x1d07
0000289A  99                cwd
0000289B  52                push dx
0000289C  50                push ax
0000289D  8D5EE8            lea bx,[bp-0x18]
000028A0  1E                push ds
000028A1  07                pop es
000028A2  06                push es
000028A3  53                push bx
000028A4  B80200            mov ax,0x2
000028A7  50                push ax
000028A8  9AC1005802        call word 0x258:word 0xc1
000028AD  B80100            mov ax,0x1
000028B0  50                push ax
000028B1  B8081D            mov ax,0x1d08
000028B4  99                cwd
000028B5  52                push dx
000028B6  50                push ax
000028B7  8D5EEE            lea bx,[bp-0x12]
000028BA  1E                push ds
000028BB  07                pop es
000028BC  06                push es
000028BD  53                push bx
000028BE  B80200            mov ax,0x2
000028C1  50                push ax
000028C2  9AC1005802        call word 0x258:word 0xc1
000028C7  B80100            mov ax,0x1
000028CA  50                push ax
000028CB  B8091D            mov ax,0x1d09
000028CE  99                cwd
000028CF  52                push dx
000028D0  50                push ax
000028D1  8D5EEC            lea bx,[bp-0x14]
000028D4  1E                push ds
000028D5  07                pop es
000028D6  06                push es
000028D7  53                push bx
000028D8  B80200            mov ax,0x2
000028DB  50                push ax
000028DC  9AC1005802        call word 0x258:word 0xc1
000028E1  B80100            mov ax,0x1
000028E4  50                push ax
000028E5  B80B1D            mov ax,0x1d0b
000028E8  99                cwd
000028E9  52                push dx
000028EA  50                push ax
000028EB  8D5EEA            lea bx,[bp-0x16]
000028EE  1E                push ds
000028EF  07                pop es
000028F0  06                push es
000028F1  53                push bx
000028F2  B80200            mov ax,0x2
000028F5  50                push ax
000028F6  9AC1005802        call word 0x258:word 0xc1
000028FB  B80100            mov ax,0x1
000028FE  50                push ax
000028FF  B80D1D            mov ax,0x1d0d
00002902  99                cwd
00002903  52                push dx
00002904  50                push ax
00002905  8D5EF0            lea bx,[bp-0x10]
00002908  1E                push ds
00002909  07                pop es
0000290A  06                push es
0000290B  53                push bx
0000290C  B80200            mov ax,0x2
0000290F  50                push ax
00002910  9AC1005802        call word 0x258:word 0xc1
00002915  B80100            mov ax,0x1
00002918  50                push ax
00002919  B80F1D            mov ax,0x1d0f
0000291C  99                cwd
0000291D  52                push dx
0000291E  50                push ax
0000291F  8D5EE6            lea bx,[bp-0x1a]
00002922  1E                push ds
00002923  07                pop es
00002924  06                push es
00002925  53                push bx
00002926  B80200            mov ax,0x2
00002929  50                push ax
0000292A  9AC1005802        call word 0x258:word 0xc1
0000292F  B80100            mov ax,0x1
00002932  50                push ax
00002933  B8101D            mov ax,0x1d10
00002936  99                cwd
00002937  52                push dx
00002938  50                push ax
00002939  8D5EE4            lea bx,[bp-0x1c]
0000293C  1E                push ds
0000293D  07                pop es
0000293E  06                push es
0000293F  53                push bx
00002940  B80200            mov ax,0x2
00002943  50                push ax
00002944  9AC1005802        call word 0x258:word 0xc1
00002949  B80100            mov ax,0x1
0000294C  50                push ax
0000294D  B8131D            mov ax,0x1d13
00002950  99                cwd
00002951  52                push dx
00002952  50                push ax
00002953  8D5EE8            lea bx,[bp-0x18]
00002956  1E                push ds
00002957  07                pop es
00002958  06                push es
00002959  53                push bx
0000295A  B80200            mov ax,0x2
0000295D  50                push ax
0000295E  9AC1005802        call word 0x258:word 0xc1
00002963  B80100            mov ax,0x1
00002966  50                push ax
00002967  B8141D            mov ax,0x1d14
0000296A  99                cwd
0000296B  52                push dx
0000296C  50                push ax
0000296D  8D5EEE            lea bx,[bp-0x12]
00002970  1E                push ds
00002971  07                pop es
00002972  06                push es
00002973  53                push bx
00002974  B80200            mov ax,0x2
00002977  50                push ax
00002978  9AC1005802        call word 0x258:word 0xc1
0000297D  B80100            mov ax,0x1
00002980  50                push ax
00002981  B8151D            mov ax,0x1d15
00002984  99                cwd
00002985  52                push dx
00002986  50                push ax
00002987  8D5EEC            lea bx,[bp-0x14]
0000298A  1E                push ds
0000298B  07                pop es
0000298C  06                push es
0000298D  53                push bx
0000298E  B80200            mov ax,0x2
00002991  50                push ax
00002992  9AC1005802        call word 0x258:word 0xc1
00002997  B80100            mov ax,0x1
0000299A  50                push ax
0000299B  B8171D            mov ax,0x1d17
0000299E  99                cwd
0000299F  52                push dx
000029A0  50                push ax
000029A1  8D5EEA            lea bx,[bp-0x16]
000029A4  1E                push ds
000029A5  07                pop es
000029A6  06                push es
000029A7  53                push bx
000029A8  B80200            mov ax,0x2
000029AB  50                push ax
000029AC  9AC1005802        call word 0x258:word 0xc1
000029B1  B80100            mov ax,0x1
000029B4  50                push ax
000029B5  B8191D            mov ax,0x1d19
000029B8  99                cwd
000029B9  52                push dx
000029BA  50                push ax
000029BB  8D5EF0            lea bx,[bp-0x10]
000029BE  1E                push ds
000029BF  07                pop es
000029C0  06                push es
000029C1  53                push bx
000029C2  B80200            mov ax,0x2
000029C5  50                push ax
000029C6  9AC1005802        call word 0x258:word 0xc1
000029CB  B80100            mov ax,0x1
000029CE  50                push ax
000029CF  B81B1D            mov ax,0x1d1b
000029D2  99                cwd
000029D3  52                push dx
000029D4  50                push ax
000029D5  8D5EE6            lea bx,[bp-0x1a]
000029D8  1E                push ds
000029D9  07                pop es
000029DA  06                push es
000029DB  53                push bx
000029DC  B80200            mov ax,0x2
000029DF  50                push ax
000029E0  9AC1005802        call word 0x258:word 0xc1
000029E5  B80100            mov ax,0x1
000029E8  50                push ax
000029E9  B81C1D            mov ax,0x1d1c
000029EC  99                cwd
000029ED  52                push dx
000029EE  50                push ax
000029EF  8D5EE4            lea bx,[bp-0x1c]
000029F2  1E                push ds
000029F3  07                pop es
000029F4  06                push es
000029F5  53                push bx
000029F6  B80200            mov ax,0x2
000029F9  50                push ax
000029FA  9AC1005802        call word 0x258:word 0xc1
000029FF  B80100            mov ax,0x1
00002A02  50                push ax
00002A03  B81F1D            mov ax,0x1d1f
00002A06  99                cwd
00002A07  52                push dx
00002A08  50                push ax
00002A09  8D5EE8            lea bx,[bp-0x18]
00002A0C  1E                push ds
00002A0D  07                pop es
00002A0E  06                push es
00002A0F  53                push bx
00002A10  B80200            mov ax,0x2
00002A13  50                push ax
00002A14  9AC1005802        call word 0x258:word 0xc1
00002A19  B80100            mov ax,0x1
00002A1C  50                push ax
00002A1D  B8201D            mov ax,0x1d20
00002A20  99                cwd
00002A21  52                push dx
00002A22  50                push ax
00002A23  8D5EEE            lea bx,[bp-0x12]
00002A26  1E                push ds
00002A27  07                pop es
00002A28  06                push es
00002A29  53                push bx
00002A2A  B80200            mov ax,0x2
00002A2D  50                push ax
00002A2E  9AC1005802        call word 0x258:word 0xc1
00002A33  B80100            mov ax,0x1
00002A36  50                push ax
00002A37  B8211D            mov ax,0x1d21
00002A3A  99                cwd
00002A3B  52                push dx
00002A3C  50                push ax
00002A3D  8D5EEC            lea bx,[bp-0x14]
00002A40  1E                push ds
00002A41  07                pop es
00002A42  06                push es
00002A43  53                push bx
00002A44  B80200            mov ax,0x2
00002A47  50                push ax
00002A48  9AC1005802        call word 0x258:word 0xc1
00002A4D  B80100            mov ax,0x1
00002A50  50                push ax
00002A51  B8231D            mov ax,0x1d23
00002A54  99                cwd
00002A55  52                push dx
00002A56  50                push ax
00002A57  8D5EEA            lea bx,[bp-0x16]
00002A5A  1E                push ds
00002A5B  07                pop es
00002A5C  06                push es
00002A5D  53                push bx
00002A5E  B80200            mov ax,0x2
00002A61  50                push ax
00002A62  9AC1005802        call word 0x258:word 0xc1
00002A67  B80100            mov ax,0x1
00002A6A  50                push ax
00002A6B  B8251D            mov ax,0x1d25
00002A6E  99                cwd
00002A6F  52                push dx
00002A70  50                push ax
00002A71  8D5EF0            lea bx,[bp-0x10]
00002A74  1E                push ds
00002A75  07                pop es
00002A76  06                push es
00002A77  53                push bx
00002A78  B80200            mov ax,0x2
00002A7B  50                push ax
00002A7C  9AC1005802        call word 0x258:word 0xc1
00002A81  B80100            mov ax,0x1
00002A84  50                push ax
00002A85  B8271D            mov ax,0x1d27
00002A88  99                cwd
00002A89  52                push dx
00002A8A  50                push ax
00002A8B  8D5EE6            lea bx,[bp-0x1a]
00002A8E  1E                push ds
00002A8F  07                pop es
00002A90  06                push es
00002A91  53                push bx
00002A92  B80200            mov ax,0x2
00002A95  50                push ax
00002A96  9AC1005802        call word 0x258:word 0xc1
00002A9B  B80100            mov ax,0x1
00002A9E  50                push ax
00002A9F  B8281D            mov ax,0x1d28
00002AA2  99                cwd
00002AA3  52                push dx
00002AA4  50                push ax
00002AA5  8D5EE4            lea bx,[bp-0x1c]
00002AA8  1E                push ds
00002AA9  07                pop es
00002AAA  06                push es
00002AAB  53                push bx
00002AAC  B80200            mov ax,0x2
00002AAF  50                push ax
00002AB0  9AC1005802        call word 0x258:word 0xc1
00002AB5  B80100            mov ax,0x1
00002AB8  50                push ax
00002AB9  B82B1D            mov ax,0x1d2b
00002ABC  99                cwd
00002ABD  52                push dx
00002ABE  50                push ax
00002ABF  8D5EE8            lea bx,[bp-0x18]
00002AC2  1E                push ds
00002AC3  07                pop es
00002AC4  06                push es
00002AC5  53                push bx
00002AC6  B80200            mov ax,0x2
00002AC9  50                push ax
00002ACA  9AC1005802        call word 0x258:word 0xc1
00002ACF  B80100            mov ax,0x1
00002AD2  50                push ax
00002AD3  B82C1D            mov ax,0x1d2c
00002AD6  99                cwd
00002AD7  52                push dx
00002AD8  50                push ax
00002AD9  8D5EEE            lea bx,[bp-0x12]
00002ADC  1E                push ds
00002ADD  07                pop es
00002ADE  06                push es
00002ADF  53                push bx
00002AE0  B80200            mov ax,0x2
00002AE3  50                push ax
00002AE4  9AC1005802        call word 0x258:word 0xc1
00002AE9  B80100            mov ax,0x1
00002AEC  50                push ax
00002AED  B82D1D            mov ax,0x1d2d
00002AF0  99                cwd
00002AF1  52                push dx
00002AF2  50                push ax
00002AF3  8D5EEC            lea bx,[bp-0x14]
00002AF6  1E                push ds
00002AF7  07                pop es
00002AF8  06                push es
00002AF9  53                push bx
00002AFA  B80200            mov ax,0x2
00002AFD  50                push ax
00002AFE  9AC1005802        call word 0x258:word 0xc1
00002B03  B80100            mov ax,0x1
00002B06  50                push ax
00002B07  B82F1D            mov ax,0x1d2f
00002B0A  99                cwd
00002B0B  52                push dx
00002B0C  50                push ax
00002B0D  8D5EEA            lea bx,[bp-0x16]
00002B10  1E                push ds
00002B11  07                pop es
00002B12  06                push es
00002B13  53                push bx
00002B14  B80200            mov ax,0x2
00002B17  50                push ax
00002B18  9AC1005802        call word 0x258:word 0xc1
00002B1D  B80100            mov ax,0x1
00002B20  50                push ax
00002B21  B8311D            mov ax,0x1d31
00002B24  99                cwd
00002B25  52                push dx
00002B26  50                push ax
00002B27  8D5EF0            lea bx,[bp-0x10]
00002B2A  1E                push ds
00002B2B  07                pop es
00002B2C  06                push es
00002B2D  53                push bx
00002B2E  B80200            mov ax,0x2
00002B31  50                push ax
00002B32  9AC1005802        call word 0x258:word 0xc1
00002B37  B80100            mov ax,0x1
00002B3A  50                push ax
00002B3B  B8331D            mov ax,0x1d33
00002B3E  99                cwd
00002B3F  52                push dx
00002B40  50                push ax
00002B41  8D5EE6            lea bx,[bp-0x1a]
00002B44  1E                push ds
00002B45  07                pop es
00002B46  06                push es
00002B47  53                push bx
00002B48  B80200            mov ax,0x2
00002B4B  50                push ax
00002B4C  9AC1005802        call word 0x258:word 0xc1
00002B51  B80100            mov ax,0x1
00002B54  50                push ax
00002B55  B8341D            mov ax,0x1d34
00002B58  99                cwd
00002B59  52                push dx
00002B5A  50                push ax
00002B5B  8D5EE4            lea bx,[bp-0x1c]
00002B5E  1E                push ds
00002B5F  07                pop es
00002B60  06                push es
00002B61  53                push bx
00002B62  B80200            mov ax,0x2
00002B65  50                push ax
00002B66  9AC1005802        call word 0x258:word 0xc1
00002B6B  B80100            mov ax,0x1
00002B6E  50                push ax
00002B6F  B8371D            mov ax,0x1d37
00002B72  99                cwd
00002B73  52                push dx
00002B74  50                push ax
00002B75  8D5EE8            lea bx,[bp-0x18]
00002B78  1E                push ds
00002B79  07                pop es
00002B7A  06                push es
00002B7B  53                push bx
00002B7C  B80200            mov ax,0x2
00002B7F  50                push ax
00002B80  9AC1005802        call word 0x258:word 0xc1
00002B85  B80100            mov ax,0x1
00002B88  50                push ax
00002B89  B8381D            mov ax,0x1d38
00002B8C  99                cwd
00002B8D  52                push dx
00002B8E  50                push ax
00002B8F  8D5EEE            lea bx,[bp-0x12]
00002B92  1E                push ds
00002B93  07                pop es
00002B94  06                push es
00002B95  53                push bx
00002B96  B80200            mov ax,0x2
00002B99  50                push ax
00002B9A  9AC1005802        call word 0x258:word 0xc1
00002B9F  B80100            mov ax,0x1
00002BA2  50                push ax
00002BA3  B8391D            mov ax,0x1d39
00002BA6  99                cwd
00002BA7  52                push dx
00002BA8  50                push ax
00002BA9  8D5EEC            lea bx,[bp-0x14]
00002BAC  1E                push ds
00002BAD  07                pop es
00002BAE  06                push es
00002BAF  53                push bx
00002BB0  B80200            mov ax,0x2
00002BB3  50                push ax
00002BB4  9AC1005802        call word 0x258:word 0xc1
00002BB9  B80100            mov ax,0x1
00002BBC  50                push ax
00002BBD  B83B1D            mov ax,0x1d3b
00002BC0  99                cwd
00002BC1  52                push dx
00002BC2  50                push ax
00002BC3  8D5EEA            lea bx,[bp-0x16]
00002BC6  1E                push ds
00002BC7  07                pop es
00002BC8  06                push es
00002BC9  53                push bx
00002BCA  B80200            mov ax,0x2
00002BCD  50                push ax
00002BCE  9AC1005802        call word 0x258:word 0xc1
00002BD3  B80100            mov ax,0x1
00002BD6  50                push ax
00002BD7  B83D1D            mov ax,0x1d3d
00002BDA  99                cwd
00002BDB  52                push dx
00002BDC  50                push ax
00002BDD  8D5EF0            lea bx,[bp-0x10]
00002BE0  1E                push ds
00002BE1  07                pop es
00002BE2  06                push es
00002BE3  53                push bx
00002BE4  B80200            mov ax,0x2
00002BE7  50                push ax
00002BE8  9AC1005802        call word 0x258:word 0xc1
00002BED  B80100            mov ax,0x1
00002BF0  50                push ax
00002BF1  B83F1D            mov ax,0x1d3f
00002BF4  99                cwd
00002BF5  52                push dx
00002BF6  50                push ax
00002BF7  8D5EE6            lea bx,[bp-0x1a]
00002BFA  1E                push ds
00002BFB  07                pop es
00002BFC  06                push es
00002BFD  53                push bx
00002BFE  B80200            mov ax,0x2
00002C01  50                push ax
00002C02  9AC1005802        call word 0x258:word 0xc1
00002C07  B80100            mov ax,0x1
00002C0A  50                push ax
00002C0B  B8401D            mov ax,0x1d40
00002C0E  99                cwd
00002C0F  52                push dx
00002C10  50                push ax
00002C11  8D5EE4            lea bx,[bp-0x1c]
00002C14  1E                push ds
00002C15  07                pop es
00002C16  06                push es
00002C17  53                push bx
00002C18  B80200            mov ax,0x2
00002C1B  50                push ax
00002C1C  9AC1005802        call word 0x258:word 0xc1
00002C21  B80100            mov ax,0x1
00002C24  50                push ax
00002C25  B8431D            mov ax,0x1d43
00002C28  99                cwd
00002C29  52                push dx
00002C2A  50                push ax
00002C2B  8D5EE8            lea bx,[bp-0x18]
00002C2E  1E                push ds
00002C2F  07                pop es
00002C30  06                push es
00002C31  53                push bx
00002C32  B80200            mov ax,0x2
00002C35  50                push ax
00002C36  9AC1005802        call word 0x258:word 0xc1
00002C3B  B80100            mov ax,0x1
00002C3E  50                push ax
00002C3F  B8441D            mov ax,0x1d44
00002C42  99                cwd
00002C43  52                push dx
00002C44  50                push ax
00002C45  8D5EEE            lea bx,[bp-0x12]
00002C48  1E                push ds
00002C49  07                pop es
00002C4A  06                push es
00002C4B  53                push bx
00002C4C  B80200            mov ax,0x2
00002C4F  50                push ax
00002C50  9AC1005802        call word 0x258:word 0xc1
00002C55  B80100            mov ax,0x1
00002C58  50                push ax
00002C59  B8451D            mov ax,0x1d45
00002C5C  99                cwd
00002C5D  52                push dx
00002C5E  50                push ax
00002C5F  8D5EEC            lea bx,[bp-0x14]
00002C62  1E                push ds
00002C63  07                pop es
00002C64  06                push es
00002C65  53                push bx
00002C66  B80200            mov ax,0x2
00002C69  50                push ax
00002C6A  9AC1005802        call word 0x258:word 0xc1
00002C6F  B80100            mov ax,0x1
00002C72  50                push ax
00002C73  B8471D            mov ax,0x1d47
00002C76  99                cwd
00002C77  52                push dx
00002C78  50                push ax
00002C79  8D5EEA            lea bx,[bp-0x16]
00002C7C  1E                push ds
00002C7D  07                pop es
00002C7E  06                push es
00002C7F  53                push bx
00002C80  B80200            mov ax,0x2
00002C83  50                push ax
00002C84  9AC1005802        call word 0x258:word 0xc1
00002C89  B80100            mov ax,0x1
00002C8C  50                push ax
00002C8D  B8491D            mov ax,0x1d49
00002C90  99                cwd
00002C91  52                push dx
00002C92  50                push ax
00002C93  8D5EF0            lea bx,[bp-0x10]
00002C96  1E                push ds
00002C97  07                pop es
00002C98  06                push es
00002C99  53                push bx
00002C9A  B80200            mov ax,0x2
00002C9D  50                push ax
00002C9E  9AC1005802        call word 0x258:word 0xc1
00002CA3  B80100            mov ax,0x1
00002CA6  50                push ax
00002CA7  B84B1D            mov ax,0x1d4b
00002CAA  99                cwd
00002CAB  52                push dx
00002CAC  50                push ax
00002CAD  8D5EE6            lea bx,[bp-0x1a]
00002CB0  1E                push ds
00002CB1  07                pop es
00002CB2  06                push es
00002CB3  53                push bx
00002CB4  B80200            mov ax,0x2
00002CB7  50                push ax
00002CB8  9AC1005802        call word 0x258:word 0xc1
00002CBD  B80100            mov ax,0x1
00002CC0  50                push ax
00002CC1  B84C1D            mov ax,0x1d4c
00002CC4  99                cwd
00002CC5  52                push dx
00002CC6  50                push ax
00002CC7  8D5EE4            lea bx,[bp-0x1c]
00002CCA  1E                push ds
00002CCB  07                pop es
00002CCC  06                push es
00002CCD  53                push bx
00002CCE  B80200            mov ax,0x2
00002CD1  50                push ax
00002CD2  9AC1005802        call word 0x258:word 0xc1
00002CD7  B80100            mov ax,0x1
00002CDA  50                push ax
00002CDB  B84F1D            mov ax,0x1d4f
00002CDE  99                cwd
00002CDF  52                push dx
00002CE0  50                push ax
00002CE1  8D5EE8            lea bx,[bp-0x18]
00002CE4  1E                push ds
00002CE5  07                pop es
00002CE6  06                push es
00002CE7  53                push bx
00002CE8  B80200            mov ax,0x2
00002CEB  50                push ax
00002CEC  9AC1005802        call word 0x258:word 0xc1
00002CF1  B80100            mov ax,0x1
00002CF4  50                push ax
00002CF5  B8501D            mov ax,0x1d50
00002CF8  99                cwd
00002CF9  52                push dx
00002CFA  50                push ax
00002CFB  8D5EEE            lea bx,[bp-0x12]
00002CFE  1E                push ds
00002CFF  07                pop es
00002D00  06                push es
00002D01  53                push bx
00002D02  B80200            mov ax,0x2
00002D05  50                push ax
00002D06  9AC1005802        call word 0x258:word 0xc1
00002D0B  B80100            mov ax,0x1
00002D0E  50                push ax
00002D0F  B8511D            mov ax,0x1d51
00002D12  99                cwd
00002D13  52                push dx
00002D14  50                push ax
00002D15  8D5EEC            lea bx,[bp-0x14]
00002D18  1E                push ds
00002D19  07                pop es
00002D1A  06                push es
00002D1B  53                push bx
00002D1C  B80200            mov ax,0x2
00002D1F  50                push ax
00002D20  9AC1005802        call word 0x258:word 0xc1
00002D25  B80100            mov ax,0x1
00002D28  50                push ax
00002D29  B8531D            mov ax,0x1d53
00002D2C  99                cwd
00002D2D  52                push dx
00002D2E  50                push ax
00002D2F  8D5EEA            lea bx,[bp-0x16]
00002D32  1E                push ds
00002D33  07                pop es
00002D34  06                push es
00002D35  53                push bx
00002D36  B80200            mov ax,0x2
00002D39  50                push ax
00002D3A  9AC1005802        call word 0x258:word 0xc1
00002D3F  B80100            mov ax,0x1
00002D42  50                push ax
00002D43  B8551D            mov ax,0x1d55
00002D46  99                cwd
00002D47  52                push dx
00002D48  50                push ax
00002D49  8D5EF0            lea bx,[bp-0x10]
00002D4C  1E                push ds
00002D4D  07                pop es
00002D4E  06                push es
00002D4F  53                push bx
00002D50  B80200            mov ax,0x2
00002D53  50                push ax
00002D54  9AC1005802        call word 0x258:word 0xc1
00002D59  B80100            mov ax,0x1
00002D5C  50                push ax
00002D5D  B8571D            mov ax,0x1d57
00002D60  99                cwd
00002D61  52                push dx
00002D62  50                push ax
00002D63  8D5EE6            lea bx,[bp-0x1a]
00002D66  1E                push ds
00002D67  07                pop es
00002D68  06                push es
00002D69  53                push bx
00002D6A  B80200            mov ax,0x2
00002D6D  50                push ax
00002D6E  9AC1005802        call word 0x258:word 0xc1
00002D73  B80100            mov ax,0x1
00002D76  50                push ax
00002D77  B8581D            mov ax,0x1d58
00002D7A  99                cwd
00002D7B  52                push dx
00002D7C  50                push ax
00002D7D  8D5EE4            lea bx,[bp-0x1c]
00002D80  1E                push ds
00002D81  07                pop es
00002D82  06                push es
00002D83  53                push bx
00002D84  B80200            mov ax,0x2
00002D87  50                push ax
00002D88  9AC1005802        call word 0x258:word 0xc1
00002D8D  B80100            mov ax,0x1
00002D90  50                push ax
00002D91  B85B1D            mov ax,0x1d5b
00002D94  99                cwd
00002D95  52                push dx
00002D96  50                push ax
00002D97  8D5EE8            lea bx,[bp-0x18]
00002D9A  1E                push ds
00002D9B  07                pop es
00002D9C  06                push es
00002D9D  53                push bx
00002D9E  B80200            mov ax,0x2
00002DA1  50                push ax
00002DA2  9AC1005802        call word 0x258:word 0xc1
00002DA7  B80100            mov ax,0x1
00002DAA  50                push ax
00002DAB  B85C1D            mov ax,0x1d5c
00002DAE  99                cwd
00002DAF  52                push dx
00002DB0  50                push ax
00002DB1  8D5EEE            lea bx,[bp-0x12]
00002DB4  1E                push ds
00002DB5  07                pop es
00002DB6  06                push es
00002DB7  53                push bx
00002DB8  B80200            mov ax,0x2
00002DBB  50                push ax
00002DBC  9AC1005802        call word 0x258:word 0xc1
00002DC1  B80100            mov ax,0x1
00002DC4  50                push ax
00002DC5  B85D1D            mov ax,0x1d5d
00002DC8  99                cwd
00002DC9  52                push dx
00002DCA  50                push ax
00002DCB  8D5EEC            lea bx,[bp-0x14]
00002DCE  1E                push ds
00002DCF  07                pop es
00002DD0  06                push es
00002DD1  53                push bx
00002DD2  B80200            mov ax,0x2
00002DD5  50                push ax
00002DD6  9AC1005802        call word 0x258:word 0xc1
00002DDB  B80100            mov ax,0x1
00002DDE  50                push ax
00002DDF  B85F1D            mov ax,0x1d5f
00002DE2  99                cwd
00002DE3  52                push dx
00002DE4  50                push ax
00002DE5  8D5EEA            lea bx,[bp-0x16]
00002DE8  1E                push ds
00002DE9  07                pop es
00002DEA  06                push es
00002DEB  53                push bx
00002DEC  B80200            mov ax,0x2
00002DEF  50                push ax
00002DF0  9AC1005802        call word 0x258:word 0xc1
00002DF5  B80100            mov ax,0x1
00002DF8  50                push ax
00002DF9  B8611D            mov ax,0x1d61
00002DFC  99                cwd
00002DFD  52                push dx
00002DFE  50                push ax
00002DFF  8D5EF0            lea bx,[bp-0x10]
00002E02  1E                push ds
00002E03  07                pop es
00002E04  06                push es
00002E05  53                push bx
00002E06  B80200            mov ax,0x2
00002E09  50                push ax
00002E0A  9AC1005802        call word 0x258:word 0xc1
00002E0F  B80100            mov ax,0x1
00002E12  50                push ax
00002E13  B8631D            mov ax,0x1d63
00002E16  99                cwd
00002E17  52                push dx
00002E18  50                push ax
00002E19  8D5EE6            lea bx,[bp-0x1a]
00002E1C  1E                push ds
00002E1D  07                pop es
00002E1E  06                push es
00002E1F  53                push bx
00002E20  B80200            mov ax,0x2
00002E23  50                push ax
00002E24  9AC1005802        call word 0x258:word 0xc1
00002E29  B80100            mov ax,0x1
00002E2C  50                push ax
00002E2D  B8641D            mov ax,0x1d64
00002E30  99                cwd
00002E31  52                push dx
00002E32  50                push ax
00002E33  8D5EE4            lea bx,[bp-0x1c]
00002E36  1E                push ds
00002E37  07                pop es
00002E38  06                push es
00002E39  53                push bx
00002E3A  B80200            mov ax,0x2
00002E3D  50                push ax
00002E3E  9AC1005802        call word 0x258:word 0xc1
00002E43  B80100            mov ax,0x1
00002E46  50                push ax
00002E47  B8671D            mov ax,0x1d67
00002E4A  99                cwd
00002E4B  52                push dx
00002E4C  50                push ax
00002E4D  8D5EE8            lea bx,[bp-0x18]
00002E50  1E                push ds
00002E51  07                pop es
00002E52  06                push es
00002E53  53                push bx
00002E54  B80200            mov ax,0x2
00002E57  50                push ax
00002E58  9AC1005802        call word 0x258:word 0xc1
00002E5D  B80100            mov ax,0x1
00002E60  50                push ax
00002E61  B8681D            mov ax,0x1d68
00002E64  99                cwd
00002E65  52                push dx
00002E66  50                push ax
00002E67  8D5EEE            lea bx,[bp-0x12]
00002E6A  1E                push ds
00002E6B  07                pop es
00002E6C  06                push es
00002E6D  53                push bx
00002E6E  B80200            mov ax,0x2
00002E71  50                push ax
00002E72  9AC1005802        call word 0x258:word 0xc1
00002E77  B80100            mov ax,0x1
00002E7A  50                push ax
00002E7B  B8691D            mov ax,0x1d69
00002E7E  99                cwd
00002E7F  52                push dx
00002E80  50                push ax
00002E81  8D5EEC            lea bx,[bp-0x14]
00002E84  1E                push ds
00002E85  07                pop es
00002E86  06                push es
00002E87  53                push bx
00002E88  B80200            mov ax,0x2
00002E8B  50                push ax
00002E8C  9AC1005802        call word 0x258:word 0xc1
00002E91  B80100            mov ax,0x1
00002E94  50                push ax
00002E95  B86B1D            mov ax,0x1d6b
00002E98  99                cwd
00002E99  52                push dx
00002E9A  50                push ax
00002E9B  8D5EEA            lea bx,[bp-0x16]
00002E9E  1E                push ds
00002E9F  07                pop es
00002EA0  06                push es
00002EA1  53                push bx
00002EA2  B80200            mov ax,0x2
00002EA5  50                push ax
00002EA6  9AC1005802        call word 0x258:word 0xc1
00002EAB  B80100            mov ax,0x1
00002EAE  50                push ax
00002EAF  B86D1D            mov ax,0x1d6d
00002EB2  99                cwd
00002EB3  52                push dx
00002EB4  50                push ax
00002EB5  8D5EF0            lea bx,[bp-0x10]
00002EB8  1E                push ds
00002EB9  07                pop es
00002EBA  06                push es
00002EBB  53                push bx
00002EBC  B80200            mov ax,0x2
00002EBF  50                push ax
00002EC0  9AC1005802        call word 0x258:word 0xc1
00002EC5  B80100            mov ax,0x1
00002EC8  50                push ax
00002EC9  B86F1D            mov ax,0x1d6f
00002ECC  99                cwd
00002ECD  52                push dx
00002ECE  50                push ax
00002ECF  8D5EE6            lea bx,[bp-0x1a]
00002ED2  1E                push ds
00002ED3  07                pop es
00002ED4  06                push es
00002ED5  53                push bx
00002ED6  B80200            mov ax,0x2
00002ED9  50                push ax
00002EDA  9AC1005802        call word 0x258:word 0xc1
00002EDF  B80100            mov ax,0x1
00002EE2  50                push ax
00002EE3  B8701D            mov ax,0x1d70
00002EE6  99                cwd
00002EE7  52                push dx
00002EE8  50                push ax
00002EE9  8D5EE4            lea bx,[bp-0x1c]
00002EEC  1E                push ds
00002EED  07                pop es
00002EEE  06                push es
00002EEF  53                push bx
00002EF0  B80200            mov ax,0x2
00002EF3  50                push ax
00002EF4  9AC1005802        call word 0x258:word 0xc1
00002EF9  B80100            mov ax,0x1
00002EFC  50                push ax
00002EFD  B8731D            mov ax,0x1d73
00002F00  99                cwd
00002F01  52                push dx
00002F02  50                push ax
00002F03  8D5EE8            lea bx,[bp-0x18]
00002F06  1E                push ds
00002F07  07                pop es
00002F08  06                push es
00002F09  53                push bx
00002F0A  B80200            mov ax,0x2
00002F0D  50                push ax
00002F0E  9AC1005802        call word 0x258:word 0xc1
00002F13  B80100            mov ax,0x1
00002F16  50                push ax
00002F17  B8741D            mov ax,0x1d74
00002F1A  99                cwd
00002F1B  52                push dx
00002F1C  50                push ax
00002F1D  8D5EEE            lea bx,[bp-0x12]
00002F20  1E                push ds
00002F21  07                pop es
00002F22  06                push es
00002F23  53                push bx
00002F24  B80200            mov ax,0x2
00002F27  50                push ax
00002F28  9AC1005802        call word 0x258:word 0xc1
00002F2D  B80100            mov ax,0x1
00002F30  50                push ax
00002F31  B8751D            mov ax,0x1d75
00002F34  99                cwd
00002F35  52                push dx
00002F36  50                push ax
00002F37  8D5EEC            lea bx,[bp-0x14]
00002F3A  1E                push ds
00002F3B  07                pop es
00002F3C  06                push es
00002F3D  53                push bx
00002F3E  B80200            mov ax,0x2
00002F41  50                push ax
00002F42  9AC1005802        call word 0x258:word 0xc1
00002F47  B80100            mov ax,0x1
00002F4A  50                push ax
00002F4B  B8771D            mov ax,0x1d77
00002F4E  99                cwd
00002F4F  52                push dx
00002F50  50                push ax
00002F51  8D5EEA            lea bx,[bp-0x16]
00002F54  1E                push ds
00002F55  07                pop es
00002F56  06                push es
00002F57  53                push bx
00002F58  B80200            mov ax,0x2
00002F5B  50                push ax
00002F5C  9AC1005802        call word 0x258:word 0xc1
00002F61  B80100            mov ax,0x1
00002F64  50                push ax
00002F65  50                push ax
00002F66  9A3F175802        call word 0x258:word 0x173f
00002F6B  8D46F2            lea ax,[bp-0xe]
00002F6E  50                push ax
00002F6F  9A324F5802        call word 0x258:word 0x4f32
00002F74  9A904F5802        call word 0x258:word 0x4f90
00002F79  CA0000            retf word 0x0
00002F7C  9A0C365802        call word 0x258:word 0x360c
00002F81  00558B            add [di-0x75],dl
00002F84  EC                in al,dx
00002F85  B402              mov ah,0x2
00002F87  8B5E06            mov bx,[bp+0x6]
00002F8A  E83001            call 0x30bd
00002F8D  5D                pop bp
00002F8E  CA0200            retf word 0x2
00002F91  55                push bp
00002F92  8BEC              mov bp,sp
00002F94  B404              mov ah,0x4
00002F96  EBEF              jmp 0x2f87
00002F98  55                push bp
00002F99  8BEC              mov bp,sp
00002F9B  8B5E08            mov bx,[bp+0x8]
00002F9E  8B5606            mov dx,[bp+0x6]
00002FA1  0AF6              or dh,dh
00002FA3  750D              jnz 0x2fb2
00002FA5  0AD2              or dl,dl
00002FA7  7409              jz 0x2fb2
00002FA9  B408              mov ah,0x8
00002FAB  E80F01            call 0x30bd
00002FAE  5D                pop bp
00002FAF  CA0400            retf word 0x4
00002FB2  E94429            jmp 0x58f9
00002FB5  55                push bp
00002FB6  8BEC              mov bp,sp
00002FB8  32C0              xor al,al
00002FBA  8B5E06            mov bx,[bp+0x6]
00002FBD  E80E01            call 0x30ce
00002FC0  5D                pop bp
00002FC1  CA0200            retf word 0x2
00002FC4  55                push bp
00002FC5  8BEC              mov bp,sp
00002FC7  B001              mov al,0x1
00002FC9  EBEF              jmp 0x2fba
00002FCB  55                push bp
00002FCC  8BEC              mov bp,sp
00002FCE  B002              mov al,0x2
00002FD0  8B4E08            mov cx,[bp+0x8]
00002FD3  0BC9              or cx,cx
00002FD5  7813              js 0x2fea
00002FD7  8B5606            mov dx,[bp+0x6]
00002FDA  8BD9              mov bx,cx
00002FDC  0BDA              or bx,dx
00002FDE  740A              jz 0x2fea
00002FE0  8B5E0A            mov bx,[bp+0xa]
00002FE3  E8E800            call 0x30ce
00002FE6  5D                pop bp
00002FE7  CA0600            retf word 0x6
00002FEA  E95D29            jmp 0x594a
00002FED  55                push bp
00002FEE  8BEC              mov bp,sp
00002FF0  B003              mov al,0x3
00002FF2  EBDC              jmp 0x2fd0
00002FF4  55                push bp
00002FF5  8BEC              mov bp,sp
00002FF7  B004              mov al,0x4
00002FF9  8B5E0C            mov bx,[bp+0xc]
00002FFC  56                push si
00002FFD  57                push di
00002FFE  06                push es
00002FFF  8B7606            mov si,[bp+0x6]
00003002  C47E08            les di,word [bp+0x8]
00003005  E8C600            call 0x30ce
00003008  07                pop es
00003009  5F                pop di
0000300A  5E                pop si
0000300B  5D                pop bp
0000300C  CA0800            retf word 0x8
0000300F  55                push bp
00003010  8BEC              mov bp,sp
00003012  B005              mov al,0x5
00003014  EBE3              jmp 0x2ff9
00003016  55                push bp
00003017  8BEC              mov bp,sp
00003019  B006              mov al,0x6
0000301B  8B4E0E            mov cx,[bp+0xe]
0000301E  0BC9              or cx,cx
00003020  78C8              js 0x2fea
00003022  8B560C            mov dx,[bp+0xc]
00003025  8BD9              mov bx,cx
00003027  0BDA              or bx,dx
00003029  74BF              jz 0x2fea
0000302B  8B5E10            mov bx,[bp+0x10]
0000302E  56                push si
0000302F  57                push di
00003030  06                push es
00003031  8B7606            mov si,[bp+0x6]
00003034  C47E08            les di,word [bp+0x8]
00003037  E89400            call 0x30ce
0000303A  07                pop es
0000303B  5F                pop di
0000303C  5E                pop si
0000303D  5D                pop bp
0000303E  CA0C00            retf word 0xc
00003041  55                push bp
00003042  8BEC              mov bp,sp
00003044  B007              mov al,0x7
00003046  EBD3              jmp 0x301b
00003048  55                push bp
00003049  8BEC              mov bp,sp
0000304B  56                push si
0000304C  33C0              xor ax,ax
0000304E  33F6              xor si,si
00003050  40                inc ax
00003051  E83A1F            call 0x4f8e
00003054  7409              jz 0x305f
00003056  E8A321            call 0x51fc
00003059  3AC3              cmp al,bl
0000305B  75F4              jnz 0x3051
0000305D  EBEF              jmp 0x304e
0000305F  5E                pop si
00003060  5D                pop bp
00003061  CB                retf
00003062  55                push bp
00003063  8BEC              mov bp,sp
00003065  8B5E06            mov bx,[bp+0x6]
00003068  0BDB              or bx,bx
0000306A  7407              jz 0x3073
0000306C  B400              mov ah,0x0
0000306E  E84C00            call 0x30bd
00003071  EB0F              jmp 0x3082
00003073  F606240B01        test byte [0xb24],0x1
00003078  7440              jz 0x30ba
0000307A  B80644            mov ax,0x4406
0000307D  CD21              int byte 0x21
0000307F  98                cbw
00003080  F7D0              not ax
00003082  5D                pop bp
00003083  CA0200            retf word 0x2
00003086  55                push bp
00003087  8BEC              mov bp,sp
00003089  56                push si
0000308A  8B5E08            mov bx,[bp+0x8]
0000308D  E87521            call 0x5205
00003090  7503              jnz 0x3095
00003092  E99728            jmp 0x592c
00003095  8B5E06            mov bx,[bp+0x6]
00003098  4B                dec bx
00003099  81FB0200          cmp bx,0x2
0000309D  7203              jc 0x30a2
0000309F  E95728            jmp 0x58f9
000030A2  03DB              add bx,bx
000030A4  33C0              xor ax,ax
000030A6  99                cwd
000030A7  2E03B7FB01        add si,[cs:bx+0x1fb]
000030AC  2EFFA7FF01        jmp word near [cs:bx+0x1ff]
000030B1  AD                lodsw
000030B2  EB01              jmp 0x30b5
000030B4  AC                lodsb
000030B5  5E                pop si
000030B6  5D                pop bp
000030B7  CA0400            retf word 0x4
000030BA  E96F28            jmp 0x592c
000030BD  56                push si
000030BE  E84421            call 0x5205
000030C1  74F7              jz 0x30ba
000030C3  E83C15            call 0x4602
000030C6  5E                pop si
000030C7  C3                ret
000030C8  56                push si
000030C9  57                push di
000030CA  06                push es
000030CB  E98700            jmp 0x3155
000030CE  56                push si
000030CF  57                push di
000030D0  06                push es
000030D1  56                push si
000030D2  E83021            call 0x5205
000030D5  5B                pop bx
000030D6  74E2              jz 0x30ba
000030D8  F60424            test byte [si],0x24
000030DB  7503              jnz 0x30e0
000030DD  E9A300            jmp 0x3183
000030E0  A804              test al,0x4
000030E2  7512              jnz 0x30f6
000030E4  F60420            test byte [si],0x20
000030E7  7403              jz 0x30ec
000030E9  E99A00            jmp 0x3186
000030EC  1E                push ds
000030ED  07                pop es
000030EE  8D7C13            lea di,[si+0x13]
000030F1  8B5C06            mov bx,[si+0x6]
000030F4  EB62              jmp 0x3158
000030F6  0BDB              or bx,bx
000030F8  7556              jnz 0x3150
000030FA  F60420            test byte [si],0x20
000030FD  7544              jnz 0x3143
000030FF  50                push ax
00003100  51                push cx
00003101  52                push dx
00003102  BB0200            mov bx,0x2
00003105  0C08              or al,0x8
00003107  A801              test al,0x1
00003109  7529              jnz 0x3134
0000310B  53                push bx
0000310C  57                push di
0000310D  50                push ax
0000310E  8BFC              mov di,sp
00003110  E8B5FF            call 0x30c8
00003113  58                pop ax
00003114  5F                pop di
00003115  5B                pop bx
00003116  03D8              add bx,ax
00003118  E84E00            call 0x3169
0000311B  50                push ax
0000311C  57                push di
0000311D  9A324F5802        call word 0x258:word 0x4f32
00003122  59                pop cx
00003123  E319              jcxz 0x313e
00003125  8BD9              mov bx,cx
00003127  E89217            call 0x48bc
0000312A  890D              mov [di],cx
0000312C  895D02            mov [di+0x2],bx
0000312F  897FFE            mov [bx-0x2],di
00003132  EB0A              jmp 0x313e
00003134  53                push bx
00003135  031D              add bx,[di]
00003137  E82F00            call 0x3169
0000313A  5B                pop bx
0000313B  E88AFF            call 0x30c8
0000313E  5A                pop dx
0000313F  59                pop cx
00003140  58                pop ax
00003141  0C10              or al,0x10
00003143  268B1D            mov bx,[es:di]
00003146  268B7D02          mov di,[es:di+0x2]
0000314A  1E                push ds
0000314B  07                pop es
0000314C  0BDB              or bx,bx
0000314E  7415              jz 0x3165
00003150  F60420            test byte [si],0x20
00003153  7503              jnz 0x3158
00003155  E81100            call 0x3169
00003158  893E3806          mov [0x638],di
0000315C  8C063A06          mov word [0x63a],es
00003160  B40A              mov ah,0xa
00003162  E89D14            call 0x4602
00003165  07                pop es
00003166  5F                pop di
00003167  5E                pop si
00003168  C3                ret
00003169  3B5C06            cmp bx,[si+0x6]
0000316C  7707              ja 0x3175
0000316E  F6440508          test byte [si+0x5],0x8
00003172  7504              jnz 0x3178
00003174  C3                ret
00003175  E9C927            jmp 0x5941
00003178  E9BD27            jmp 0x5938
0000317B  0000              add [bx+si],al
0000317D  0100              add [bx+si],ax
0000317F  3401              xor al,0x1
00003181  3101              xor [bx+di],ax
00003183  E9AC27            jmp 0x5932
00003186  E99A27            jmp 0x5923
00003189  005256            add [bp+si+0x56],dl
0000318C  80FC02            cmp ah,0x2
0000318F  7750              ja 0x31e1
00003191  8ADC              mov bl,ah
00003193  32FF              xor bh,bh
00003195  D1E3              shl bx,1
00003197  8BF3              mov si,bx
00003199  80C431            add ah,0x31
0000319C  88263309          mov [0x933],ah
000031A0  BA3009            mov dx,0x930
000031A3  B8013D            mov ax,0x3d01
000031A6  CD21              int byte 0x21
000031A8  7237              jc 0x31e1
000031AA  8BD8              mov bx,ax
000031AC  B80044            mov ax,0x4400
000031AF  CD21              int byte 0x21
000031B1  F6C280            test dl,0x80
000031B4  7427              jz 0x31dd
000031B6  B80144            mov ax,0x4401
000031B9  80CA20            or dl,0x20
000031BC  32F6              xor dh,dh
000031BE  CD21              int byte 0x21
000031C0  E8C32A            call 0x5c86
000031C3  720A              jc 0x31cf
000031C5  B80A44            mov ax,0x440a
000031C8  CD21              int byte 0x21
000031CA  F6C680            test dh,0x80
000031CD  7518              jnz 0x31e7
000031CF  1E                push ds
000031D0  B84000            mov ax,0x40
000031D3  8ED8              mov ds,ax
000031D5  F74408FFFF        test word [si+0x8],0xffff
000031DA  1F                pop ds
000031DB  750A              jnz 0x31e7
000031DD  B43E              mov ah,0x3e
000031DF  CD21              int byte 0x21
000031E1  B401              mov ah,0x1
000031E3  33DB              xor bx,bx
000031E5  EB02              jmp 0x31e9
000031E7  32E4              xor ah,ah
000031E9  5E                pop si
000031EA  5A                pop dx
000031EB  C3                ret
000031EC  51                push cx
000031ED  52                push dx
000031EE  50                push ax
000031EF  8BD4              mov dx,sp
000031F1  B90100            mov cx,0x1
000031F4  B440              mov ah,0x40
000031F6  CD21              int byte 0x21
000031F8  5A                pop dx
000031F9  7204              jc 0x31ff
000031FB  32E4              xor ah,ah
000031FD  EB02              jmp 0x3201
000031FF  B401              mov ah,0x1
00003201  5A                pop dx
00003202  59                pop cx
00003203  C3                ret
00003204  50                push ax
00003205  B43E              mov ah,0x3e
00003207  CD21              int byte 0x21
00003209  58                pop ax
0000320A  C3                ret
0000320B  005657            add [bp+0x57],dl
0000320E  B4FF              mov ah,0xff
00003210  8A17              mov dl,[bx]
00003212  80FA01            cmp dl,0x1
00003215  7603              jna 0x321a
00003217  E9F300            jmp 0x330d
0000321A  BE4C09            mov si,0x94c
0000321D  7403              jz 0x3222
0000321F  BE3609            mov si,0x936
00003222  8814              mov [si],dl
00003224  32F6              xor dh,dh
00003226  8BFA              mov di,dx
00003228  D1E7              shl di,1
0000322A  E8E300            call 0x3310
0000322D  0BC0              or ax,ax
0000322F  7403              jz 0x3234
00003231  E9D900            jmp 0x330d
00003234  8B5716            mov dx,[bx+0x16]
00003237  8B4F1A            mov cx,[bx+0x1a]
0000323A  E83F01            call 0x337c
0000323D  8A4F1C            mov cl,[bx+0x1c]
00003240  C6440100          mov byte [si+0x1],0x0
00003244  C6440202          mov byte [si+0x2],0x2
00003248  204C02            and [si+0x2],cl
0000324B  C6441104          mov byte [si+0x11],0x4
0000324F  8A4F1C            mov cl,[bx+0x1c]
00003252  204C11            and [si+0x11],cl
00003255  8B4F14            mov cx,[bx+0x14]
00003258  894C12            mov [si+0x12],cx
0000325B  8B4F18            mov cx,[bx+0x18]
0000325E  894C14            mov [si+0x14],cx
00003261  33C9              xor cx,cx
00003263  394F08            cmp [bx+0x8],cx
00003266  7403              jz 0x326b
00003268  8B4F06            mov cx,[bx+0x6]
0000326B  894C07            mov [si+0x7],cx
0000326E  33C9              xor cx,cx
00003270  394F0C            cmp [bx+0xc],cx
00003273  7403              jz 0x3278
00003275  8B4F06            mov cx,[bx+0x6]
00003278  894C05            mov [si+0x5],cx
0000327B  C744030000        mov word [si+0x3],0x0
00003280  C6440900          mov byte [si+0x9],0x0
00003284  C6440A00          mov byte [si+0xa],0x0
00003288  C6440B00          mov byte [si+0xb],0x0
0000328C  C6440C00          mov byte [si+0xc],0x0
00003290  8A7701            mov dh,[bx+0x1]
00003293  8A5702            mov dl,[bx+0x2]
00003296  0AD2              or dl,dl
00003298  7409              jz 0x32a3
0000329A  80FE08            cmp dh,0x8
0000329D  7509              jnz 0x32a8
0000329F  B4FF              mov ah,0xff
000032A1  EB51              jmp 0x32f4
000032A3  80FE04            cmp dh,0x4
000032A6  74F7              jz 0x329f
000032A8  80EE05            sub dh,0x5
000032AB  88740E            mov [si+0xe],dh
000032AE  8AF2              mov dh,dl
000032B0  80FA02            cmp dl,0x2
000032B3  720C              jc 0x32c1
000032B5  B601              mov dh,0x1
000032B7  FECA              dec dl
000032B9  FEC6              inc dh
000032BB  FEC6              inc dh
000032BD  FECA              dec dl
000032BF  75F8              jnz 0x32b9
000032C1  88740D            mov [si+0xd],dh
000032C4  8A4703            mov al,[bx+0x3]
000032C7  FEC8              dec al
000032C9  7804              js 0x32cf
000032CB  804C0E04          or byte [si+0xe],0x4
000032CF  8B4F04            mov cx,[bx+0x4]
000032D2  E8E000            call 0x33b5
000032D5  B4FF              mov ah,0xff
000032D7  E31B              jcxz 0x32f4
000032D9  E82301            call 0x33ff
000032DC  0AE4              or ah,ah
000032DE  7514              jnz 0x32f4
000032E0  8B4F08            mov cx,[bx+0x8]
000032E3  894C07            mov [si+0x7],cx
000032E6  8B4F0C            mov cx,[bx+0xc]
000032E9  894C05            mov [si+0x5],cx
000032EC  8B4F0A            mov cx,[bx+0xa]
000032EF  894C03            mov [si+0x3],cx
000032F2  EB19              jmp 0x330d
000032F4  50                push ax
000032F5  53                push bx
000032F6  51                push cx
000032F7  52                push dx
000032F8  33C9              xor cx,cx
000032FA  880E2E06          mov [0x62e],cl
000032FE  E81A03            call 0x361b
00003301  5A                pop dx
00003302  59                pop cx
00003303  5B                pop bx
00003304  58                pop ax
00003305  80FCFC            cmp ah,0xfc
00003308  7503              jnz 0x330d
0000330A  E8BF28            call 0x5bcc
0000330D  5F                pop di
0000330E  5E                pop si
0000330F  C3                ret
00003310  1E                push ds
00003311  33D2              xor dx,dx
00003313  8EDA              mov ds,dx
00003315  87950004          xchg dx,[di+0x400]
00003319  1F                pop ds
0000331A  8995BE07          mov [di+0x7be],dx
0000331E  0BD2              or dx,dx
00003320  7503              jnz 0x3325
00003322  B4FE              mov ah,0xfe
00003324  C3                ret
00003325  53                push bx
00003326  56                push si
00003327  BE1C07            mov si,0x71c
0000332A  BBEE05            mov bx,0x5ee
0000332D  0BFF              or di,di
0000332F  7406              jz 0x3337
00003331  BE0B07            mov si,0x70b
00003334  BBF005            mov bx,0x5f0
00003337  B90CEF            mov cx,0xef0c
0000333A  81BDBE07F803      cmp word [di+0x7be],0x3f8
00003340  7403              jz 0x3345
00003342  B90BF7            mov cx,0xf70b
00003345  FA                cli
00003346  83C204            add dx,0x4
00003349  B001              mov al,0x1
0000334B  EE                out dx,al
0000334C  EB00              jmp 0x334e
0000334E  83EA03            sub dx,0x3
00003351  48                dec ax
00003352  EE                out dx,al
00003353  8BD6              mov dx,si
00003355  8BF3              mov si,bx
00003357  8AC1              mov al,cl
00003359  B435              mov ah,0x35
0000335B  CD21              int byte 0x21
0000335D  891C              mov [si],bx
0000335F  8C4402            mov word [si+0x2],es
00003362  1E                push ds
00003363  07                pop es
00003364  0E                push cs
00003365  1F                pop ds
00003366  B425              mov ah,0x25
00003368  CD21              int byte 0x21
0000336A  06                push es
0000336B  1F                pop ds
0000336C  8AE5              mov ah,ch
0000336E  E421              in al,byte 0x21
00003370  22C4              and al,ah
00003372  EB00              jmp 0x3374
00003374  E621              out byte 0x21,al
00003376  33C0              xor ax,ax
00003378  FB                sti
00003379  5E                pop si
0000337A  5B                pop bx
0000337B  C3                ret
0000337C  56                push si
0000337D  53                push bx
0000337E  50                push ax
0000337F  BB0E06            mov bx,0x60e
00003382  803C00            cmp byte [si],0x0
00003385  7403              jz 0x338a
00003387  BB1C06            mov bx,0x61c
0000338A  33C0              xor ax,ax
0000338C  894704            mov [bx+0x4],ax
0000338F  E82804            call 0x37ba
00003392  894F02            mov [bx+0x2],cx
00003395  894F06            mov [bx+0x6],cx
00003398  BBF205            mov bx,0x5f2
0000339B  803C00            cmp byte [si],0x0
0000339E  7403              jz 0x33a3
000033A0  BB0006            mov bx,0x600
000033A3  33C0              xor ax,ax
000033A5  894704            mov [bx+0x4],ax
000033A8  E80F04            call 0x37ba
000033AB  895702            mov [bx+0x2],dx
000033AE  895706            mov [bx+0x6],dx
000033B1  58                pop ax
000033B2  5B                pop bx
000033B3  5E                pop si
000033B4  C3                ret
000033B5  53                push bx
000033B6  52                push dx
000033B7  BB4D04            mov bx,0x44d
000033BA  8BD1              mov dx,cx
000033BC  43                inc bx
000033BD  43                inc bx
000033BE  2E8B0F            mov cx,[cs:bx]
000033C1  43                inc bx
000033C2  43                inc bx
000033C3  E304              jcxz 0x33c9
000033C5  3BD1              cmp dx,cx
000033C7  75F3              jnz 0x33bc
000033C9  2E8B0F            mov cx,[cs:bx]
000033CC  5A                pop dx
000033CD  5B                pop bx
000033CE  C3                ret
000033CF  4B                dec bx
000033D0  0000              add [bx+si],al
000033D2  06                push es
000033D3  6E                outsb
000033D4  0017              add [bx],dl
000033D6  0496              add al,0x96
000033D8  0000              add [bx+si],al
000033DA  032C              add bp,[si]
000033DC  01800158          add [bx+si+0x5801],ax
000033E0  02C0              add al,al
000033E2  00B00460          add [bx+si+0x6004],dh
000033E6  0008              add [bx+si],cl
000033E8  07                pop es
000033E9  40                inc ax
000033EA  006009            add [bx+si+0x9],ah
000033ED  3000              xor [bx+si],al
000033EF  C01218            rcl byte [bp+si],byte 0x18
000033F2  0080250C          add [bx+si+0xc25],al
000033F6  0000              add [bx+si],al
000033F8  4B                dec bx
000033F9  06                push es
000033FA  0000              add [bx+si],al
000033FC  0000              add [bx+si],al
000033FE  008B95BE          add [bp+di-0x416b],cl
00003402  07                pop es
00003403  83C203            add dx,0x3
00003406  B080              mov al,0x80
00003408  EE                out dx,al
00003409  83EA02            sub dx,0x2
0000340C  8AC5              mov al,ch
0000340E  EB00              jmp 0x3410
00003410  EE                out dx,al
00003411  4A                dec dx
00003412  8AC1              mov al,cl
00003414  EB00              jmp 0x3416
00003416  EE                out dx,al
00003417  8A6C0D            mov ch,[si+0xd]
0000341A  B103              mov cl,0x3
0000341C  D2E5              shl ch,cl
0000341E  8A440E            mov al,[si+0xe]
00003421  0AC5              or al,ch
00003423  83C203            add dx,0x3
00003426  EB00              jmp 0x3428
00003428  EE                out dx,al
00003429  83EA03            sub dx,0x3
0000342C  33C9              xor cx,cx
0000342E  EB00              jmp 0x3430
00003430  EC                in al,dx
00003431  E2FB              loop 0x342e
00003433  83C205            add dx,0x5
00003436  EB00              jmp 0x3438
00003438  EC                in al,dx
00003439  42                inc dx
0000343A  EB00              jmp 0x343c
0000343C  EC                in al,dx
0000343D  EB00              jmp 0x343f
0000343F  EC                in al,dx
00003440  88440C            mov [si+0xc],al
00003443  4A                dec dx
00003444  4A                dec dx
00003445  8A4402            mov al,[si+0x2]
00003448  3402              xor al,0x2
0000344A  0C09              or al,0x9
0000344C  EB00              jmp 0x344e
0000344E  EE                out dx,al
0000344F  83EA03            sub dx,0x3
00003452  B00B              mov al,0xb
00003454  EB00              jmp 0x3456
00003456  EE                out dx,al
00003457  42                inc dx
00003458  EB00              jmp 0x345a
0000345A  EC                in al,dx
0000345B  E80300            call 0x3461
0000345E  8AE6              mov ah,dh
00003460  C3                ret
00003461  53                push bx
00003462  51                push cx
00003463  57                push di
00003464  33DB              xor bx,bx
00003466  8BCB              mov cx,bx
00003468  33FF              xor di,di
0000346A  803E2E0600        cmp byte [0x62e],0x0
0000346F  7578              jnz 0x34e9
00003471  E85827            call 0x5bcc
00003474  32E4              xor ah,ah
00003476  8A440C            mov al,[si+0xc]
00003479  A810              test al,0x10
0000347B  750F              jnz 0x348c
0000347D  837C0300          cmp word [si+0x3],0x0
00003481  7409              jz 0x348c
00003483  B603              mov dh,0x3
00003485  3B7C03            cmp di,[si+0x3]
00003488  736F              jnc 0x34f9
0000348A  FEC4              inc ah
0000348C  A820              test al,0x20
0000348E  750F              jnz 0x349f
00003490  837C0500          cmp word [si+0x5],0x0
00003494  7409              jz 0x349f
00003496  B604              mov dh,0x4
00003498  3B7C05            cmp di,[si+0x5]
0000349B  735C              jnc 0x34f9
0000349D  FEC4              inc ah
0000349F  A880              test al,0x80
000034A1  750F              jnz 0x34b2
000034A3  837C0700          cmp word [si+0x7],0x0
000034A7  7409              jz 0x34b2
000034A9  B605              mov dh,0x5
000034AB  3B7C07            cmp di,[si+0x7]
000034AE  7349              jnc 0x34f9
000034B0  FEC4              inc ah
000034B2  0AE4              or ah,ah
000034B4  7441              jz 0x34f7
000034B6  8BF9              mov di,cx
000034B8  32E4              xor ah,ah
000034BA  CD1A              int byte 0x1a
000034BC  87F9              xchg di,cx
000034BE  8BC1              mov ax,cx
000034C0  0BC3              or ax,bx
000034C2  7506              jnz 0x34ca
000034C4  8BCF              mov cx,di
000034C6  8BDA              mov bx,dx
000034C8  EB9E              jmp 0x3468
000034CA  2BD3              sub dx,bx
000034CC  1BF9              sbb di,cx
000034CE  7307              jnc 0x34d7
000034D0  81C2B000          add dx,0xb0
000034D4  83D718            adc di,0x18
000034D7  0BFF              or di,di
000034D9  BFFFFF            mov di,0xffff
000034DC  758C              jnz 0x346a
000034DE  B83700            mov ax,0x37
000034E1  F7E2              mul dx
000034E3  7285              jc 0x346a
000034E5  8BF8              mov di,ax
000034E7  EB81              jmp 0x346a
000034E9  FF168E0A          call word near [0xa8e]
000034ED  F606900A08        test byte [0xa90],0x8
000034F2  7480              jz 0x3474
000034F4  B6FC              mov dh,0xfc
000034F6  3D32F6            cmp ax,0xf632
000034F9  5F                pop di
000034FA  59                pop cx
000034FB  5B                pop bx
000034FC  C3                ret
000034FD  56                push si
000034FE  53                push bx
000034FF  BE3609            mov si,0x936
00003502  0AE4              or ah,ah
00003504  7403              jz 0x3509
00003506  BE4C09            mov si,0x94c
00003509  E83100            call 0x353d
0000350C  80FC00            cmp ah,0x0
0000350F  7527              jnz 0x3538
00003511  BBF205            mov bx,0x5f2
00003514  803C00            cmp byte [si],0x0
00003517  7403              jz 0x351c
00003519  BB0006            mov bx,0x600
0000351C  837F0800          cmp word [bx+0x8],0x0
00003520  750A              jnz 0x352c
00003522  807C0F00          cmp byte [si+0xf],0x0
00003526  7410              jz 0x3538
00003528  B01A              mov al,0x1a
0000352A  EB08              jmp 0x3534
0000352C  06                push es
0000352D  8E4412            mov es,word [si+0x12]
00003530  E8AF02            call 0x37e2
00003533  07                pop es
00003534  0BE4              or sp,sp
00003536  EB02              jmp 0x353a
00003538  32C0              xor al,al
0000353A  5B                pop bx
0000353B  5E                pop si
0000353C  C3                ret
0000353D  807C0A00          cmp byte [si+0xa],0x0
00003541  741B              jz 0x355e
00003543  8A440A            mov al,[si+0xa]
00003546  B406              mov ah,0x6
00003548  A802              test al,0x2
0000354A  751C              jnz 0x3568
0000354C  B402              mov ah,0x2
0000354E  A804              test al,0x4
00003550  7516              jnz 0x3568
00003552  B407              mov ah,0x7
00003554  A808              test al,0x8
00003556  7510              jnz 0x3568
00003558  B4FF              mov ah,0xff
0000355A  A810              test al,0x10
0000355C  750A              jnz 0x3568
0000355E  B401              mov ah,0x1
00003560  807C0900          cmp byte [si+0x9],0x0
00003564  7502              jnz 0x3568
00003566  32E4              xor ah,ah
00003568  C6440900          mov byte [si+0x9],0x0
0000356C  C6440A00          mov byte [si+0xa],0x0
00003570  C3                ret
00003571  57                push di
00003572  56                push si
00003573  52                push dx
00003574  51                push cx
00003575  53                push bx
00003576  BE3609            mov si,0x936
00003579  BB0E06            mov bx,0x60e
0000357C  0AE4              or ah,ah
0000357E  7406              jz 0x3586
00003580  BE4C09            mov si,0x94c
00003583  BB1C06            mov bx,0x61c
00003586  50                push ax
00003587  E8D7FE            call 0x3461
0000358A  58                pop ax
0000358B  8AE6              mov ah,dh
0000358D  80FC00            cmp ah,0x0
00003590  752E              jnz 0x35c0
00003592  8B4F06            mov cx,[bx+0x6]
00003595  E83426            call 0x5bcc
00003598  3B4F08            cmp cx,[bx+0x8]
0000359B  74E9              jz 0x3586
0000359D  FA                cli
0000359E  807C0100          cmp byte [si+0x1],0x0
000035A2  7513              jnz 0x35b7
000035A4  C64401FF          mov byte [si+0x1],0xff
000035A8  33D2              xor dx,dx
000035AA  8A14              mov dl,[si]
000035AC  8BFA              mov di,dx
000035AE  D1E7              shl di,1
000035B0  8B95BE07          mov dx,[di+0x7be]
000035B4  EE                out dx,al
000035B5  EB08              jmp 0x35bf
000035B7  06                push es
000035B8  8E4414            mov es,word [si+0x14]
000035BB  E80C02            call 0x37ca
000035BE  07                pop es
000035BF  FB                sti
000035C0  5B                pop bx
000035C1  59                pop cx
000035C2  5A                pop dx
000035C3  5E                pop si
000035C4  5F                pop di
000035C5  C3                ret
000035C6  53                push bx
000035C7  56                push si
000035C8  BBF205            mov bx,0x5f2
000035CB  BE0E06            mov si,0x60e
000035CE  0AE4              or ah,ah
000035D0  7406              jz 0x35d8
000035D2  BB0006            mov bx,0x600
000035D5  BE1C06            mov si,0x61c
000035D8  8B5708            mov dx,[bx+0x8]
000035DB  8B4C06            mov cx,[si+0x6]
000035DE  2B4C08            sub cx,[si+0x8]
000035E1  32E4              xor ah,ah
000035E3  5E                pop si
000035E4  5B                pop bx
000035E5  C3                ret
000035E6  56                push si
000035E7  53                push bx
000035E8  51                push cx
000035E9  52                push dx
000035EA  BE3609            mov si,0x936
000035ED  BB0E06            mov bx,0x60e
000035F0  0AE4              or ah,ah
000035F2  7406              jz 0x35fa
000035F4  BE4C09            mov si,0x94c
000035F7  BB1C06            mov bx,0x61c
000035FA  837F0800          cmp word [bx+0x8],0x0
000035FE  740D              jz 0x360d
00003600  E85EFE            call 0x3461
00003603  0AF6              or dh,dh
00003605  7506              jnz 0x360d
00003607  837F0800          cmp word [bx+0x8],0x0
0000360B  75F3              jnz 0x3600
0000360D  51                push cx
0000360E  33C9              xor cx,cx
00003610  E2FE              loop 0x3610
00003612  59                pop cx
00003613  E80500            call 0x361b
00003616  5A                pop dx
00003617  59                pop cx
00003618  5B                pop bx
00003619  5E                pop si
0000361A  C3                ret
0000361B  57                push di
0000361C  33D2              xor dx,dx
0000361E  8A14              mov dl,[si]
00003620  8BFA              mov di,dx
00003622  D1E7              shl di,1
00003624  8B95BE07          mov dx,[di+0x7be]
00003628  B001              mov al,0x1
0000362A  E301              jcxz 0x362d
0000362C  90                nop
0000362D  83C204            add dx,0x4
00003630  EE                out dx,al
00003631  C747080000        mov word [bx+0x8],0x0
00003636  83EA03            sub dx,0x3
00003639  EB00              jmp 0x363b
0000363B  B80010            mov ax,0x1000
0000363E  EE                out dx,al
0000363F  4A                dec dx
00003640  80FE03            cmp dh,0x3
00003643  7402              jz 0x3647
00003645  B408              mov ah,0x8
00003647  FA                cli
00003648  8426B507          test [0x7b5],ah
0000364C  7408              jz 0x3656
0000364E  E421              in al,byte 0x21
00003650  0AC4              or al,ah
00003652  EB00              jmp 0x3654
00003654  E621              out byte 0x21,al
00003656  BBEE05            mov bx,0x5ee
00003659  0BFF              or di,di
0000365B  7403              jz 0x3660
0000365D  BBF005            mov bx,0x5f0
00003660  B00C              mov al,0xc
00003662  80FE03            cmp dh,0x3
00003665  7402              jz 0x3669
00003667  B00B              mov al,0xb
00003669  1E                push ds
0000366A  C517              lds dx,word [bx]
0000366C  B425              mov ah,0x25
0000366E  CD21              int byte 0x21
00003670  FB                sti
00003671  1F                pop ds
00003672  33C0              xor ax,ax
00003674  8907              mov [bx],ax
00003676  894702            mov [bx+0x2],ax
00003679  33DB              xor bx,bx
0000367B  879DBE07          xchg bx,[di+0x7be]
0000367F  1E                push ds
00003680  33D2              xor dx,dx
00003682  8EDA              mov ds,dx
00003684  899D0004          mov [di+0x400],bx
00003688  1F                pop ds
00003689  5F                pop di
0000368A  C3                ret
0000368B  1E                push ds
0000368C  57                push di
0000368D  52                push dx
0000368E  2E8E1E7036        mov ds,word [cs:0x3670]
00003693  8B16C007          mov dx,[0x7c0]
00003697  BF4C09            mov di,0x94c
0000369A  EB0F              jmp 0x36ab
0000369C  1E                push ds
0000369D  57                push di
0000369E  52                push dx
0000369F  2E8E1E7036        mov ds,word [cs:0x3670]
000036A4  8B16BE07          mov dx,[0x7be]
000036A8  BF3609            mov di,0x936
000036AB  50                push ax
000036AC  53                push bx
000036AD  56                push si
000036AE  42                inc dx
000036AF  42                inc dx
000036B0  EB00              jmp 0x36b2
000036B2  EC                in al,dx
000036B3  A801              test al,0x1
000036B5  750E              jnz 0x36c5
000036B7  32E4              xor ah,ah
000036B9  8BF0              mov si,ax
000036BB  52                push dx
000036BC  2EFF945407        call word near [cs:si+0x754]
000036C1  FA                cli
000036C2  5A                pop dx
000036C3  EBEB              jmp 0x36b0
000036C5  5E                pop si
000036C6  5B                pop bx
000036C7  BA2000            mov dx,0x20
000036CA  B020              mov al,0x20
000036CC  EB00              jmp 0x36ce
000036CE  EE                out dx,al
000036CF  58                pop ax
000036D0  5A                pop dx
000036D1  5F                pop di
000036D2  1F                pop ds
000036D3  CF                iret
000036D4  F607CB            test byte [bx],0xcb
000036D7  07                pop es
000036D8  5E                pop si
000036D9  07                pop es
000036DA  5C                pop sp
000036DB  07                pop es
000036DC  FB                sti
000036DD  C3                ret
000036DE  32E4              xor ah,ah
000036E0  83C203            add dx,0x3
000036E3  EB00              jmp 0x36e5
000036E5  EC                in al,dx
000036E6  241E              and al,0x1e
000036E8  7415              jz 0x36ff
000036EA  807D1100          cmp byte [di+0x11],0x0
000036EE  7406              jz 0x36f6
000036F0  807D0D00          cmp byte [di+0xd],0x0
000036F4  7504              jnz 0x36fa
000036F6  3C04              cmp al,0x4
000036F8  7405              jz 0x36ff
000036FA  88450A            mov [di+0xa],al
000036FD  B480              mov ah,0x80
000036FF  83EA05            sub dx,0x5
00003702  BBF205            mov bx,0x5f2
00003705  803D00            cmp byte [di],0x0
00003708  7403              jz 0x370d
0000370A  BB0006            mov bx,0x600
0000370D  8B7708            mov si,[bx+0x8]
00003710  3B7706            cmp si,[bx+0x6]
00003713  7426              jz 0x373b
00003715  807D0F00          cmp byte [di+0xf],0x0
00003719  7520              jnz 0x373b
0000371B  EB00              jmp 0x371d
0000371D  EC                in al,dx
0000371E  FB                sti
0000371F  807D1000          cmp byte [di+0x10],0x0
00003723  740A              jz 0x372f
00003725  3C1A              cmp al,0x1a
00003727  7506              jnz 0x372f
00003729  C6450FFF          mov byte [di+0xf],0xff
0000372D  EB14              jmp 0x3743
0000372F  0AC4              or al,ah
00003731  06                push es
00003732  8E4512            mov es,word [di+0x12]
00003735  E89200            call 0x37ca
00003738  07                pop es
00003739  EB08              jmp 0x3743
0000373B  EB00              jmp 0x373d
0000373D  EC                in al,dx
0000373E  FB                sti
0000373F  C6450901          mov byte [di+0x9],0x1
00003743  8BC7              mov ax,di
00003745  D0E8              shr al,1
00003747  FF267E0A          jmp word near [0xa7e]
0000374B  FB                sti
0000374C  807D0B00          cmp byte [di+0xb],0x0
00003750  751E              jnz 0x3770
00003752  8BF3              mov si,bx
00003754  BB0E06            mov bx,0x60e
00003757  803D00            cmp byte [di],0x0
0000375A  7403              jz 0x375f
0000375C  BB1C06            mov bx,0x61c
0000375F  837F0800          cmp word [bx+0x8],0x0
00003763  740C              jz 0x3771
00003765  4A                dec dx
00003766  4A                dec dx
00003767  06                push es
00003768  8E4514            mov es,word [di+0x14]
0000376B  E87400            call 0x37e2
0000376E  07                pop es
0000376F  EE                out dx,al
00003770  C3                ret
00003771  C6450100          mov byte [di+0x1],0x0
00003775  C3                ret
00003776  FB                sti
00003777  83C204            add dx,0x4
0000377A  EB00              jmp 0x377c
0000377C  EC                in al,dx
0000377D  88450C            mov [di+0xc],al
00003780  837D0700          cmp word [di+0x7],0x0
00003784  7404              jz 0x378a
00003786  A880              test al,0x80
00003788  7414              jz 0x379e
0000378A  837D0500          cmp word [di+0x5],0x0
0000378E  7404              jz 0x3794
00003790  A820              test al,0x20
00003792  740A              jz 0x379e
00003794  837D0300          cmp word [di+0x3],0x0
00003798  7408              jz 0x37a2
0000379A  A810              test al,0x10
0000379C  7504              jnz 0x37a2
0000379E  88550B            mov [di+0xb],dl
000037A1  C3                ret
000037A2  807D0B00          cmp byte [di+0xb],0x0
000037A6  74F9              jz 0x37a1
000037A8  C6450B00          mov byte [di+0xb],0x0
000037AC  4A                dec dx
000037AD  EB00              jmp 0x37af
000037AF  EC                in al,dx
000037B0  2420              and al,0x20
000037B2  74F9              jz 0x37ad
000037B4  83EA03            sub dx,0x3
000037B7  EB92              jmp 0x374b
000037B9  0089470A          add [bx+di+0xa47],cl
000037BD  89470C            mov [bx+0xc],ax
000037C0  C747080000        mov word [bx+0x8],0x0
000037C5  C7070000          mov word [bx],0x0
000037C9  C3                ret
000037CA  56                push si
000037CB  8B770C            mov si,[bx+0xc]
000037CE  268804            mov [es:si],al
000037D1  46                inc si
000037D2  3B7702            cmp si,[bx+0x2]
000037D5  7503              jnz 0x37da
000037D7  8B7704            mov si,[bx+0x4]
000037DA  89770C            mov [bx+0xc],si
000037DD  FF4708            inc word [bx+0x8]
000037E0  5E                pop si
000037E1  C3                ret
000037E2  56                push si
000037E3  8B770A            mov si,[bx+0xa]
000037E6  268A04            mov al,[es:si]
000037E9  46                inc si
000037EA  3B7702            cmp si,[bx+0x2]
000037ED  7503              jnz 0x37f2
000037EF  8B7704            mov si,[bx+0x4]
000037F2  89770A            mov [bx+0xa],si
000037F5  FF4F08            dec word [bx+0x8]
000037F8  5E                pop si
000037F9  C3                ret
000037FA  50                push ax
000037FB  52                push dx
000037FC  56                push si
000037FD  050F00            add ax,0xf
00003800  80D200            adc dl,0x0
00003803  B90400            mov cx,0x4
00003806  D1EA              shr dx,1
00003808  D1D8              rcr ax,1
0000380A  E2FA              loop 0x3806
0000380C  0BD2              or dx,dx
0000380E  750C              jnz 0x381c
00003810  8BF0              mov si,ax
00003812  E82100            call 0x3836
00003815  7307              jnc 0x381e
00003817  E81500            call 0x382f
0000381A  7302              jnc 0x381e
0000381C  33C0              xor ax,ax
0000381E  91                xchg ax,cx
0000381F  5E                pop si
00003820  5A                pop dx
00003821  58                pop ax
00003822  C3                ret
00003823  50                push ax
00003824  06                push es
00003825  8EC0              mov es,ax
00003827  B449              mov ah,0x49
00003829  E80E00            call 0x383a
0000382C  07                pop es
0000382D  58                pop ax
0000382E  C3                ret
0000382F  8BC6              mov ax,si
00003831  40                inc ax
00003832  FF166209          call word near [0x962]
00003836  8BDE              mov bx,si
00003838  B448              mov ah,0x48
0000383A  CD21              int byte 0x21
0000383C  7306              jnc 0x3844
0000383E  3D0800            cmp ax,0x8
00003841  7502              jnz 0x3845
00003843  F9                stc
00003844  C3                ret
00003845  3D0700            cmp ax,0x7
00003848  7505              jnz 0x384f
0000384A  EA1B2A5802        jmp word 0x258:word 0x2a1b
0000384F  E95021            jmp 0x59a2
00003852  C10B1A            ror word [bp+di],byte 0x1a
00003855  0C30              or al,0x30
00003857  0C40              or al,0x40
00003859  0C6A              or al,0x6a
0000385B  0C77              or al,0x77
0000385D  0C05              or al,0x5
0000385F  090F              or [bx],cx
00003861  0CBB              or al,0xbb
00003863  0CFB              or al,0xfb
00003865  0C98              or al,0x98
00003867  0DA10D            or ax,0xda1
0000386A  6E                outsb
0000386B  0C79              or al,0x79
0000386D  297929            sub [bx+di+0x29],di
00003870  50                push ax
00003871  57                push di
00003872  E8B004            call 0x3d25
00003875  C6450EFF          mov byte [di+0xe],0xff
00003879  C6450F00          mov byte [di+0xf],0x0
0000387D  C745120000        mov word [di+0x12],0x0
00003882  5F                pop di
00003883  58                pop ax
00003884  C3                ret
00003885  8AE3              mov ah,bl
00003887  50                push ax
00003888  51                push cx
00003889  E89904            call 0x3d25
0000388C  837D1200          cmp word [di+0x12],0x0
00003890  7403              jz 0x3895
00003892  E9A020            jmp 0x5935
00003895  8805              mov [di],al
00003897  BECC07            mov si,0x7cc
0000389A  E86A00            call 0x3907
0000389D  E84102            call 0x3ae1
000038A0  C6450F00          mov byte [di+0xf],0x0
000038A4  8BDF              mov bx,di
000038A6  FE062E06          inc byte [0x62e]
000038AA  E85FF9            call 0x320c
000038AD  C6062E0600        mov byte [0x62e],0x0
000038B2  0AE4              or ah,ah
000038B4  7411              jz 0x38c7
000038B6  E87902            call 0x3b32
000038B9  FEC4              inc ah
000038BB  7444              jz 0x3901
000038BD  FEC4              inc ah
000038BF  7443              jz 0x3904
000038C1  80EC02            sub ah,0x2
000038C4  E87404            call 0x3d3b
000038C7  59                pop cx
000038C8  803E260704        cmp byte [0x726],0x4
000038CD  7507              jnz 0x38d6
000038CF  41                inc cx
000038D0  E206              loop 0x38d8
000038D2  B98000            mov cx,0x80
000038D5  3D33C9            cmp ax,0xc933
000038D8  58                pop ax
000038D9  8A550E            mov dl,[di+0xe]
000038DC  8ADC              mov bl,ah
000038DE  32FF              xor bh,bh
000038E0  B41F              mov ah,0x1f
000038E2  E8680C            call 0x454d
000038E5  E303              jcxz 0x38ea
000038E7  894C06            mov [si+0x6],cx
000038EA  804C0504          or byte [si+0x5],0x4
000038EE  897512            mov [di+0x12],si
000038F1  C745100000        mov word [di+0x10],0x0
000038F6  F6451C01          test byte [di+0x1c],0x1
000038FA  7404              jz 0x3900
000038FC  804C0520          or byte [si+0x5],0x20
00003900  C3                ret
00003901  E94920            jmp 0x594d
00003904  E94C20            jmp 0x5953
00003907  C745042C01        mov word [di+0x4],0x12c
0000390C  E8A101            call 0x3ab0
0000390F  7203              jc 0x3914
00003911  895504            mov [di+0x4],dx
00003914  C6450202          mov byte [di+0x2],0x2
00003918  E87901            call 0x3a94
0000391B  721F              jc 0x393c
0000391D  B90500            mov cx,0x5
00003920  33DB              xor bx,bx
00003922  2E3A87AF09        cmp al,[cs:bx+0x9af]
00003927  740B              jz 0x3934
00003929  43                inc bx
0000392A  E2F6              loop 0x3922
0000392C  E91E20            jmp 0x594d
0000392F  4E                dec si
00003930  4F                dec di
00003931  45                inc bp
00003932  4D                dec bp
00003933  53                push bx
00003934  885D02            mov [di+0x2],bl
00003937  E85A01            call 0x3a94
0000393A  73F0              jnc 0x392c
0000393C  C6450107          mov byte [di+0x1],0x7
00003940  E86D01            call 0x3ab0
00003943  720D              jc 0x3952
00003945  83FA05            cmp dx,0x5
00003948  72E2              jc 0x392c
0000394A  83FA08            cmp dx,0x8
0000394D  77DD              ja 0x392c
0000394F  885501            mov [di+0x1],dl
00003952  33D2              xor dx,dx
00003954  837D046E          cmp word [di+0x4],0x6e
00003958  7708              ja 0x3962
0000395A  42                inc dx
0000395B  807D0105          cmp byte [di+0x1],0x5
0000395F  7401              jz 0x3962
00003961  42                inc dx
00003962  E82F01            call 0x3a94
00003965  7224              jc 0x398b
00003967  8AD0              mov dl,al
00003969  80EA31            sub dl,0x31
0000396C  80FA01            cmp dl,0x1
0000396F  77BB              ja 0x392c
00003971  D0E2              shl dl,1
00003973  7511              jnz 0x3986
00003975  E81C01            call 0x3a94
00003978  7211              jc 0x398b
0000397A  8AE0              mov ah,al
0000397C  E81501            call 0x3a94
0000397F  3D2E35            cmp ax,0x352e
00003982  75A8              jnz 0x392c
00003984  FEC2              inc dl
00003986  E80B01            call 0x3a94
00003989  73A1              jnc 0x392c
0000398B  885503            mov [di+0x3],dl
0000398E  33C0              xor ax,ax
00003990  C6451C01          mov byte [di+0x1c],0x1
00003994  894508            mov [di+0x8],ax
00003997  C7450AE803        mov word [di+0xa],0x3e8
0000399C  C7450CE803        mov word [di+0xc],0x3e8
000039A1  894516            mov [di+0x16],ax
000039A4  89451A            mov [di+0x1a],ax
000039A7  E8EA00            call 0x3a94
000039AA  7305              jnc 0x39b1
000039AC  75F9              jnz 0x39a7
000039AE  E9C200            jmp 0x3a73
000039B1  8AE0              mov ah,al
000039B3  E8DE00            call 0x3a94
000039B6  3D5352            cmp ax,0x5253
000039B9  7517              jnz 0x39d2
000039BB  804D1C02          or byte [di+0x1c],0x2
000039BF  F6451C20          test byte [di+0x1c],0x20
000039C3  7505              jnz 0x39ca
000039C5  C7450A0000        mov word [di+0xa],0x0
000039CA  E8C700            call 0x3a94
000039CD  72D8              jc 0x39a7
000039CF  E97B1F            jmp 0x594d
000039D2  3D504F            cmp ax,0x4f50
000039D5  7511              jnz 0x39e8
000039D7  E8D600            call 0x3ab0
000039DA  7303              jnc 0x39df
000039DC  BA1027            mov dx,0x2710
000039DF  895506            mov [di+0x6],dx
000039E2  804D1C08          or byte [di+0x1c],0x8
000039E6  EBBF              jmp 0x39a7
000039E8  3D5344            cmp ax,0x4453
000039EB  7508              jnz 0x39f5
000039ED  E8C000            call 0x3ab0
000039F0  89550C            mov [di+0xc],dx
000039F3  EBB2              jmp 0x39a7
000039F5  3D4443            cmp ax,0x4344
000039F8  7508              jnz 0x3a02
000039FA  E8B300            call 0x3ab0
000039FD  895508            mov [di+0x8],dx
00003A00  EBA5              jmp 0x39a7
00003A02  3D5343            cmp ax,0x4353
00003A05  750C              jnz 0x3a13
00003A07  E8A600            call 0x3ab0
00003A0A  89550A            mov [di+0xa],dx
00003A0D  804D1C20          or byte [di+0x1c],0x20
00003A11  EB94              jmp 0x39a7
00003A13  3D4252            cmp ax,0x5242
00003A16  7508              jnz 0x3a20
00003A18  E89500            call 0x3ab0
00003A1B  895516            mov [di+0x16],dx
00003A1E  EB87              jmp 0x39a7
00003A20  3D4254            cmp ax,0x5442
00003A23  7509              jnz 0x3a2e
00003A25  E88800            call 0x3ab0
00003A28  89551A            mov [di+0x1a],dx
00003A2B  E979FF            jmp 0x39a7
00003A2E  3D464C            cmp ax,0x4c46
00003A31  7506              jnz 0x3a39
00003A33  804D1C40          or byte [di+0x1c],0x40
00003A37  EB91              jmp 0x39ca
00003A39  3D4942            cmp ax,0x4249
00003A3C  7514              jnz 0x3a52
00003A3E  E85300            call 0x3a94
00003A41  3C4E              cmp al,0x4e
00003A43  758A              jnz 0x39cf
00003A45  F6451C10          test byte [di+0x1c],0x10
00003A49  7584              jnz 0x39cf
00003A4B  804D1C10          or byte [di+0x1c],0x10
00003A4F  E978FF            jmp 0x39ca
00003A52  3D5341            cmp ax,0x4153
00003A55  7510              jnz 0x3a67
00003A57  E83A00            call 0x3a94
00003A5A  3C43              cmp al,0x43
00003A5C  7506              jnz 0x3a64
00003A5E  80651CFE          and byte [di+0x1c],0xfe
00003A62  EBE1              jmp 0x3a45
00003A64  E968FF            jmp 0x39cf
00003A67  3D4550            cmp ax,0x5045
00003A6A  75F8              jnz 0x3a64
00003A6C  804D1C04          or byte [di+0x1c],0x4
00003A70  E957FF            jmp 0x39ca
00003A73  F6451C08          test byte [di+0x1c],0x8
00003A77  751A              jnz 0x3a93
00003A79  8B4508            mov ax,[di+0x8]
00003A7C  3B450C            cmp ax,[di+0xc]
00003A7F  7703              ja 0x3a84
00003A81  8B450C            mov ax,[di+0xc]
00003A84  B9FFFF            mov cx,0xffff
00003A87  BA0A00            mov dx,0xa
00003A8A  F7E2              mul dx
00003A8C  7202              jc 0x3a90
00003A8E  8BC8              mov cx,ax
00003A90  894D06            mov [di+0x6],cx
00003A93  C3                ret
00003A94  AC                lodsb
00003A95  3C20              cmp al,0x20
00003A97  74FB              jz 0x3a94
00003A99  3C09              cmp al,0x9
00003A9B  74F7              jz 0x3a94
00003A9D  3C2C              cmp al,0x2c
00003A9F  740D              jz 0x3aae
00003AA1  3C61              cmp al,0x61
00003AA3  7202              jc 0x3aa7
00003AA5  04E0              add al,0xe0
00003AA7  0AC0              or al,al
00003AA9  7504              jnz 0x3aaf
00003AAB  4E                dec si
00003AAC  0AC0              or al,al
00003AAE  F9                stc
00003AAF  C3                ret
00003AB0  53                push bx
00003AB1  33D2              xor dx,dx
00003AB3  E8DEFF            call 0x3a94
00003AB6  7227              jc 0x3adf
00003AB8  4E                dec si
00003AB9  E8D8FF            call 0x3a94
00003ABC  F5                cmc
00003ABD  7320              jnc 0x3adf
00003ABF  2C30              sub al,0x30
00003AC1  3C09              cmp al,0x9
00003AC3  7717              ja 0x3adc
00003AC5  98                cbw
00003AC6  D1E2              shl dx,1
00003AC8  7212              jc 0x3adc
00003ACA  8BDA              mov bx,dx
00003ACC  D1E2              shl dx,1
00003ACE  720C              jc 0x3adc
00003AD0  D1E2              shl dx,1
00003AD2  7208              jc 0x3adc
00003AD4  03D3              add dx,bx
00003AD6  7204              jc 0x3adc
00003AD8  03D0              add dx,ax
00003ADA  73DD              jnc 0x3ab9
00003ADC  E96E1E            jmp 0x594d
00003ADF  5B                pop bx
00003AE0  C3                ret
00003AE1  1E                push ds
00003AE2  C51E5E06          lds bx,word [0x65e]
00003AE6  8B472C            mov ax,[bx+0x2c]
00003AE9  1F                pop ds
00003AEA  0BC0              or ax,ax
00003AEC  7438              jz 0x3b26
00003AEE  8B5516            mov dx,[di+0x16]
00003AF1  0BD2              or dx,dx
00003AF3  750D              jnz 0x3b02
00003AF5  BA0002            mov dx,0x200
00003AF8  3DFFFF            cmp ax,0xffff
00003AFB  7402              jz 0x3aff
00003AFD  8BD0              mov dx,ax
00003AFF  895516            mov [di+0x16],dx
00003B02  8BC2              mov ax,dx
00003B04  33D2              xor dx,dx
00003B06  E8F1FC            call 0x37fa
00003B09  E324              jcxz 0x3b2f
00003B0B  894D14            mov [di+0x14],cx
00003B0E  8B451A            mov ax,[di+0x1a]
00003B11  0BC0              or ax,ax
00003B13  7506              jnz 0x3b1b
00003B15  B80002            mov ax,0x200
00003B18  89451A            mov [di+0x1a],ax
00003B1B  33D2              xor dx,dx
00003B1D  E8DAFC            call 0x37fa
00003B20  E307              jcxz 0x3b29
00003B22  894D18            mov [di+0x18],cx
00003B25  C3                ret
00003B26  E92A1E            jmp 0x5953
00003B29  8B4514            mov ax,[di+0x14]
00003B2C  E8F4FC            call 0x3823
00003B2F  E9811E            jmp 0x59b3
00003B32  50                push ax
00003B33  8B4514            mov ax,[di+0x14]
00003B36  E8EAFC            call 0x3823
00003B39  8B4518            mov ax,[di+0x18]
00003B3C  E8E4FC            call 0x3823
00003B3F  58                pop ax
00003B40  C3                ret
00003B41  F6440520          test byte [si+0x5],0x20
00003B45  750F              jnz 0x3b56
00003B47  33DB              xor bx,bx
00003B49  E82200            call 0x3b6e
00003B4C  7204              jc 0x3b52
00003B4E  E83E00            call 0x3b8f
00003B51  43                inc bx
00003B52  4B                dec bx
00003B53  8BC3              mov ax,bx
00003B55  C3                ret
00003B56  52                push dx
00003B57  50                push ax
00003B58  E8D801            call 0x3d33
00003B5B  8AE0              mov ah,al
00003B5D  E866FA            call 0x35c6
00003B60  E8D801            call 0x3d3b
00003B63  8BDA              mov bx,dx
00003B65  58                pop ax
00003B66  5A                pop dx
00003B67  0BDB              or bx,bx
00003B69  74E7              jz 0x3b52
00003B6B  33C0              xor ax,ax
00003B6D  C3                ret
00003B6E  F6440504          test byte [si+0x5],0x4
00003B72  7415              jz 0x3b89
00003B74  F6440510          test byte [si+0x5],0x10
00003B78  7506              jnz 0x3b80
00003B7A  E8BE00            call 0x3c3b
00003B7D  720A              jc 0x3b89
00003B7F  C3                ret
00003B80  806405EF          and byte [si+0x5],0xef
00003B84  8A4408            mov al,[si+0x8]
00003B87  F8                clc
00003B88  C3                ret
00003B89  806405EF          and byte [si+0x5],0xef
00003B8D  F9                stc
00003B8E  C3                ret
00003B8F  884408            mov [si+0x8],al
00003B92  804C0510          or byte [si+0x5],0x10
00003B96  C3                ret
00003B97  E92CFA            jmp 0x35c6
00003B9A  E89601            call 0x3d33
00003B9D  8AE0              mov ah,al
00003B9F  E824FA            call 0x35c6
00003BA2  E89601            call 0x3d3b
00003BA5  F6440510          test byte [si+0x5],0x10
00003BA9  7401              jz 0x3bac
00003BAB  42                inc dx
00003BAC  8BC2              mov ax,dx
00003BAE  EB0D              jmp 0x3bbd
00003BB0  E88001            call 0x3d33
00003BB3  8AE0              mov ah,al
00003BB5  E80EFA            call 0x35c6
00003BB8  E88001            call 0x3d3b
00003BBB  8BC1              mov ax,cx
00003BBD  33D2              xor dx,dx
00003BBF  C3                ret
00003BC0  E86201            call 0x3d25
00003BC3  F6440520          test byte [si+0x5],0x20
00003BC7  750A              jnz 0x3bd3
00003BC9  803C01            cmp byte [si],0x1
00003BCC  7405              jz 0x3bd3
00003BCE  B01A              mov al,0x1a
00003BD0  E8CC00            call 0x3c9f
00003BD3  8A25              mov ah,[di]
00003BD5  51                push cx
00003BD6  33C9              xor cx,cx
00003BD8  E80BFA            call 0x35e6
00003BDB  59                pop cx
00003BDC  E853FF            call 0x3b32
00003BDF  C745120000        mov word [di+0x12],0x0
00003BE4  E82512            call 0x4e0c
00003BE7  E95101            jmp 0x3d3b
00003BEA  885404            mov [si+0x4],dl
00003BED  C3                ret
00003BEE  57                push di
00003BEF  E83301            call 0x3d25
00003BF2  88550E            mov [di+0xe],dl
00003BF5  5F                pop di
00003BF6  C3                ret
00003BF7  06                push es
00003BF8  A802              test al,0x2
00003BFA  7504              jnz 0x3c00
00003BFC  8BD3              mov dx,bx
00003BFE  EB09              jmp 0x3c09
00003C00  E303              jcxz 0x3c05
00003C02  E9F41C            jmp 0x58f9
00003C05  3BDA              cmp bx,dx
00003C07  72F9              jc 0x3c02
00003C09  8BCB              mov cx,bx
00003C0B  C41E3806          les bx,word [0x638]
00003C0F  A801              test al,0x1
00003C11  7517              jnz 0x3c2a
00003C13  57                push di
00003C14  8BFB              mov di,bx
00003C16  B020              mov al,0x20
00003C18  F3AA              rep stosb
00003C1A  5F                pop di
00003C1B  8BCA              mov cx,dx
00003C1D  57                push di
00003C1E  E81A00            call 0x3c3b
00003C21  5F                pop di
00003C22  268807            mov [es:bx],al
00003C25  43                inc bx
00003C26  E2F5              loop 0x3c1d
00003C28  EB0E              jmp 0x3c38
00003C2A  8BCA              mov cx,dx
00003C2C  E8F600            call 0x3d25
00003C2F  268A07            mov al,[es:bx]
00003C32  E86A00            call 0x3c9f
00003C35  43                inc bx
00003C36  E2F7              loop 0x3c2f
00003C38  92                xchg ax,dx
00003C39  07                pop es
00003C3A  C3                ret
00003C3B  F6440510          test byte [si+0x5],0x10
00003C3F  7409              jz 0x3c4a
00003C41  8A4408            mov al,[si+0x8]
00003C44  806405EF          and byte [si+0x5],0xef
00003C48  EB2F              jmp 0x3c79
00003C4A  E8E600            call 0x3d33
00003C4D  8AE0              mov ah,al
00003C4F  E8ABF8            call 0x34fd
00003C52  9C                pushf
00003C53  E8E500            call 0x3d3b
00003C56  9D                popf
00003C57  7505              jnz 0x3c5e
00003C59  E8701F            call 0x5bcc
00003C5C  EBEC              jmp 0x3c4a
00003C5E  50                push ax
00003C5F  57                push di
00003C60  E8C200            call 0x3d25
00003C63  FF4D10            dec word [di+0x10]
00003C66  5F                pop di
00003C67  58                pop ax
00003C68  3C1A              cmp al,0x1a
00003C6A  750D              jnz 0x3c79
00003C6C  F6440520          test byte [si+0x5],0x20
00003C70  7507              jnz 0x3c79
00003C72  F9                stc
00003C73  806405FB          and byte [si+0x5],0xfb
00003C77  EB01              jmp 0x3c7a
00003C79  F8                clc
00003C7A  C3                ret
00003C7B  50                push ax
00003C7C  E8A600            call 0x3d25
00003C7F  58                pop ax
00003C80  F6451C10          test byte [di+0x1c],0x10
00003C84  7406              jz 0x3c8c
00003C86  F6440520          test byte [si+0x5],0x20
00003C8A  7513              jnz 0x3c9f
00003C8C  52                push dx
00003C8D  8A5404            mov dl,[si+0x4]
00003C90  8A750F            mov dh,[di+0xf]
00003C93  E81100            call 0x3ca7
00003C96  88750F            mov [di+0xf],dh
00003C99  5A                pop dx
00003C9A  C3                ret
00003C9B  80651C7F          and byte [di+0x1c],0x7f
00003C9F  8A25              mov ah,[di]
00003CA1  E8CDF8            call 0x3571
00003CA4  E99400            jmp 0x3d3b
00003CA7  FEC6              inc dh
00003CA9  3C20              cmp al,0x20
00003CAB  7216              jc 0x3cc3
00003CAD  E8EBFF            call 0x3c9b
00003CB0  3AF2              cmp dh,dl
00003CB2  720E              jc 0x3cc2
00003CB4  80FAFF            cmp dl,0xff
00003CB7  7409              jz 0x3cc2
00003CB9  B00D              mov al,0xd
00003CBB  E81E00            call 0x3cdc
00003CBE  80651C7F          and byte [di+0x1c],0x7f
00003CC2  C3                ret
00003CC3  FECE              dec dh
00003CC5  3C09              cmp al,0x9
00003CC7  7513              jnz 0x3cdc
00003CC9  F6451C10          test byte [di+0x1c],0x10
00003CCD  74D0              jz 0x3c9f
00003CCF  B020              mov al,0x20
00003CD1  E8D3FF            call 0x3ca7
00003CD4  F6C607            test dh,0x7
00003CD7  75F8              jnz 0x3cd1
00003CD9  B009              mov al,0x9
00003CDB  C3                ret
00003CDC  3C0D              cmp al,0xd
00003CDE  7517              jnz 0x3cf7
00003CE0  E8BCFF            call 0x3c9f
00003CE3  32F6              xor dh,dh
00003CE5  F6451C40          test byte [di+0x1c],0x40
00003CE9  7407              jz 0x3cf2
00003CEB  B00A              mov al,0xa
00003CED  E8AFFF            call 0x3c9f
00003CF0  B00D              mov al,0xd
00003CF2  804D1C80          or byte [di+0x1c],0x80
00003CF6  C3                ret
00003CF7  3C0A              cmp al,0xa
00003CF9  7511              jnz 0x3d0c
00003CFB  F6451C80          test byte [di+0x1c],0x80
00003CFF  749E              jz 0x3c9f
00003D01  80651C7F          and byte [di+0x1c],0x7f
00003D05  F6451C10          test byte [di+0x1c],0x10
00003D09  7494              jz 0x3c9f
00003D0B  C3                ret
00003D0C  3C08              cmp al,0x8
00003D0E  758B              jnz 0x3c9b
00003D10  FECE              dec dh
00003D12  7387              jnc 0x3c9b
00003D14  FEC6              inc dh
00003D16  EB83              jmp 0x3c9b
00003D18  50                push ax
00003D19  E80900            call 0x3d25
00003D1C  58                pop ax
00003D1D  8A650F            mov ah,[di+0xf]
00003D20  C3                ret
00003D21  8A6404            mov ah,[si+0x4]
00003D24  C3                ret
00003D25  E80B00            call 0x3d33
00003D28  BFEA06            mov di,0x6ea
00003D2B  0BC0              or ax,ax
00003D2D  7403              jz 0x3d32
00003D2F  83C71D            add di,0x1d
00003D32  C3                ret
00003D33  8BC7              mov ax,di
00003D35  05F8FF            add ax,0xfff8
00003D38  D1E8              shr ax,1
00003D3A  C3                ret
00003D3B  0AE4              or ah,ah
00003D3D  74FB              jz 0x3d3a
00003D3F  8ADC              mov bl,ah
00003D41  32FF              xor bh,bh
00003D43  BF900B            mov di,0xb90
00003D46  80FB05            cmp bl,0x5
00003D49  7602              jna 0x3d4d
00003D4B  33DB              xor bx,bx
00003D4D  D1E3              shl bx,1
00003D4F  FF21              jmp word near [bx+di]
00003D51  50                push ax
00003D52  53                push bx
00003D53  51                push cx
00003D54  52                push dx
00003D55  56                push si
00003D56  57                push di
00003D57  8B0EBE07          mov cx,[0x7be]
00003D5B  890E2A06          mov [0x62a],cx
00003D5F  E30B              jcxz 0x3d6c
00003D61  BFEA06            mov di,0x6ea
00003D64  B400              mov ah,0x0
00003D66  E87DF8            call 0x35e6
00003D69  E8C6FD            call 0x3b32
00003D6C  8B0EC007          mov cx,[0x7c0]
00003D70  890E2C06          mov [0x62c],cx
00003D74  E30B              jcxz 0x3d81
00003D76  BF0707            mov di,0x707
00003D79  B401              mov ah,0x1
00003D7B  E868F8            call 0x35e6
00003D7E  E8B1FD            call 0x3b32
00003D81  5F                pop di
00003D82  5E                pop si
00003D83  5A                pop dx
00003D84  59                pop cx
00003D85  5B                pop bx
00003D86  58                pop ax
00003D87  C3                ret
00003D88  50                push ax
00003D89  53                push bx
00003D8A  51                push cx
00003D8B  52                push dx
00003D8C  56                push si
00003D8D  57                push di
00003D8E  8B0E2A06          mov cx,[0x62a]
00003D92  E30B              jcxz 0x3d9f
00003D94  BFEA06            mov di,0x6ea
00003D97  E847FD            call 0x3ae1
00003D9A  8BDF              mov bx,di
00003D9C  E86DF4            call 0x320c
00003D9F  8B0E2C06          mov cx,[0x62c]
00003DA3  E30B              jcxz 0x3db0
00003DA5  BF0707            mov di,0x707
00003DA8  E836FD            call 0x3ae1
00003DAB  8BDF              mov bx,di
00003DAD  E85CF4            call 0x320c
00003DB0  5F                pop di
00003DB1  5E                pop si
00003DB2  5A                pop dx
00003DB3  59                pop cx
00003DB4  5B                pop bx
00003DB5  58                pop ax
00003DB6  C3                ret
00003DB7  00D3              add bl,dl
00003DB9  29D3              sub bx,dx
00003DBB  29D3              sub bx,dx
00003DBD  29D3              sub bx,dx
00003DBF  29D3              sub bx,dx
00003DC1  29D3              sub bx,dx
00003DC3  29D3              sub bx,dx
00003DC5  29D3              sub bx,dx
00003DC7  29D3              sub bx,dx
00003DC9  29D3              sub bx,dx
00003DCB  29D3              sub bx,dx
00003DCD  29D3              sub bx,dx
00003DCF  29D3              sub bx,dx
00003DD1  29D3              sub bx,dx
00003DD3  29D3              sub bx,dx
00003DD5  294B59            sub [bp+di+0x59],cx
00003DD8  42                inc dx
00003DD9  44                inc sp
00003DDA  FF5343            call word near [bp+di+0x43]
00003DDD  52                push dx
00003DDE  4E                dec si
00003DDF  FE434F            inc byte [bp+di+0x4f]
00003DE2  4E                dec si
00003DE3  53                push bx
00003DE4  FD                std
00003DE5  43                inc bx
00003DE6  4F                dec di
00003DE7  4D                dec bp
00003DE8  31FC              xor sp,di
00003DEA  43                inc bx
00003DEB  4F                dec di
00003DEC  4D                dec bp
00003DED  32FB              xor bh,bl
00003DEF  4C                dec sp
00003DF0  50                push ax
00003DF1  54                push sp
00003DF2  31FA              xor dx,di
00003DF4  4C                dec sp
00003DF5  50                push ax
00003DF6  54                push sp
00003DF7  32F9              xor bh,cl
00003DF9  4C                dec sp
00003DFA  50                push ax
00003DFB  54                push sp
00003DFC  33F8              xor di,ax
00003DFE  50                push ax
00003DFF  49                dec cx
00003E00  50                push ax
00003E01  45                inc bp
00003E02  F7005352          test word [bx+si],0x5253
00003E06  56                push si
00003E07  06                push es
00003E08  1E                push ds
00003E09  07                pop es
00003E0A  8B7702            mov si,[bx+0x2]
00003E0D  8B0F              mov cx,[bx]
00003E0F  57                push di
00003E10  8D9D8100          lea bx,[di+0x81]
00003E14  49                dec cx
00003E15  7E0D              jng 0x3e24
00003E17  807C013A          cmp byte [si+0x1],0x3a
00003E1B  7507              jnz 0x3e24
00003E1D  AD                lodsw
00003E1E  49                dec cx
00003E1F  E85F2D            call 0x6b81
00003E22  EB04              jmp 0x3e28
00003E24  41                inc cx
00003E25  E80E01            call 0x3f36
00003E28  AB                stosw
00003E29  893E3206          mov [0x632],di
00003E2D  2D403A            sub ax,0x3a40
00003E30  92                xchg ax,dx
00003E31  E307              jcxz 0x3e3a
00003E33  8A04              mov al,[si]
00003E35  E8E000            call 0x3f18
00003E38  7416              jz 0x3e50
00003E3A  E80201            call 0x3f3f
00003E3D  32C0              xor al,al
00003E3F  51                push cx
00003E40  B9FFFF            mov cx,0xffff
00003E43  F2AE              repne scasb
00003E45  4F                dec di
00003E46  807DFF5C          cmp byte [di-0x1],0x5c
00003E4A  7403              jz 0x3e4f
00003E4C  B05C              mov al,0x5c
00003E4E  AA                stosb
00003E4F  59                pop cx
00003E50  E8BD00            call 0x3f10
00003E53  E36E              jcxz 0x3ec3
00003E55  AC                lodsb
00003E56  E8282D            call 0x6b81
00003E59  AA                stosb
00003E5A  3BFB              cmp di,bx
00003E5C  7722              ja 0x3e80
00003E5E  3C2E              cmp al,0x2e
00003E60  7514              jnz 0x3e76
00003E62  80FA84            cmp dl,0x84
00003E65  7319              jnc 0x3e80
00003E67  80FA82            cmp dl,0x82
00003E6A  732D              jnc 0x3e99
00003E6C  F6C602            test dh,0x2
00003E6F  750F              jnz 0x3e80
00003E71  80CE02            or dh,0x2
00003E74  EB21              jmp 0x3e97
00003E76  E89F00            call 0x3f18
00003E79  7559              jnz 0x3ed4
00003E7B  E86A00            call 0x3ee8
00003E7E  EB38              jmp 0x3eb8
00003E80  EB4F              jmp 0x3ed1
00003E82  3C2A              cmp al,0x2a
00003E84  7404              jz 0x3e8a
00003E86  3C3F              cmp al,0x3f
00003E88  7503              jnz 0x3e8d
00003E8A  80CE01            or dh,0x1
00003E8D  80FA83            cmp dl,0x83
00003E90  733F              jnc 0x3ed1
00003E92  80FA82            cmp dl,0x82
00003E95  7502              jnz 0x3e99
00003E97  32D2              xor dl,dl
00003E99  42                inc dx
00003E9A  80FA82            cmp dl,0x82
00003E9D  7319              jnc 0x3eb8
00003E9F  80FA04            cmp dl,0x4
00003EA2  7614              jna 0x3eb8
00003EA4  F6C602            test dh,0x2
00003EA7  750D              jnz 0x3eb6
00003EA9  80FA09            cmp dl,0x9
00003EAC  720A              jc 0x3eb8
00003EAE  B02E              mov al,0x2e
00003EB0  8645FF            xchg al,[di-0x1]
00003EB3  AA                stosb
00003EB4  EBBB              jmp 0x3e71
00003EB6  4F                dec di
00003EB7  4A                dec dx
00003EB8  E29B              loop 0x3e55
00003EBA  80FA82            cmp dl,0x82
00003EBD  7604              jna 0x3ec3
00003EBF  47                inc di
00003EC0  E82500            call 0x3ee8
00003EC3  91                xchg ax,cx
00003EC4  AA                stosb
00003EC5  8BCF              mov cx,di
00003EC7  5F                pop di
00003EC8  2BCF              sub cx,di
00003ECA  8AC6              mov al,dh
00003ECC  07                pop es
00003ECD  5E                pop si
00003ECE  5A                pop dx
00003ECF  5B                pop bx
00003ED0  C3                ret
00003ED1  E9791A            jmp 0x594d
00003ED4  3C20              cmp al,0x20
00003ED6  72F9              jc 0x3ed1
00003ED8  57                push di
00003ED9  51                push cx
00003EDA  BF9C0B            mov di,0xb9c
00003EDD  B90B00            mov cx,0xb
00003EE0  F2AE              repne scasb
00003EE2  74ED              jz 0x3ed1
00003EE4  59                pop cx
00003EE5  5F                pop di
00003EE6  EB9A              jmp 0x3e82
00003EE8  80FA83            cmp dl,0x83
00003EEB  7504              jnz 0x3ef1
00003EED  4F                dec di
00003EEE  4F                dec di
00003EEF  EB1F              jmp 0x3f10
00003EF1  80FA84            cmp dl,0x84
00003EF4  751A              jnz 0x3f10
00003EF6  83EF04            sub di,0x4
00003EF9  4F                dec di
00003EFA  8A05              mov al,[di]
00003EFC  E81900            call 0x3f18
00003EFF  74F8              jz 0x3ef9
00003F01  3B3E3206          cmp di,[0x632]
00003F05  72CA              jc 0x3ed1
00003F07  4F                dec di
00003F08  8A05              mov al,[di]
00003F0A  E80B00            call 0x3f18
00003F0D  75F8              jnz 0x3f07
00003F0F  47                inc di
00003F10  BA8200            mov dx,0x82
00003F13  893E3006          mov [0x630],di
00003F17  C3                ret
00003F18  3C5C              cmp al,0x5c
00003F1A  7402              jz 0x3f1e
00003F1C  3C2F              cmp al,0x2f
00003F1E  C3                ret
00003F1F  57                push di
00003F20  A806              test al,0x6
00003F22  7510              jnz 0x3f34
00003F24  49                dec cx
00003F25  03F9              add di,cx
00003F27  41                inc cx
00003F28  81F98100          cmp cx,0x81
00003F2C  77A3              ja 0x3ed1
00003F2E  AC                lodsb
00003F2F  AA                stosb
00003F30  0AC0              or al,al
00003F32  75F3              jnz 0x3f27
00003F34  5F                pop di
00003F35  C3                ret
00003F36  B419              mov ah,0x19
00003F38  CD21              int byte 0x21
00003F3A  98                cbw
00003F3B  05413A            add ax,0x3a41
00003F3E  C3                ret
00003F3F  51                push cx
00003F40  B05C              mov al,0x5c
00003F42  AA                stosb
00003F43  56                push si
00003F44  8BF7              mov si,di
00003F46  B447              mov ah,0x47
00003F48  CD21              int byte 0x21
00003F4A  7203              jc 0x3f4f
00003F4C  5E                pop si
00003F4D  59                pop cx
00003F4E  C3                ret
00003F4F  E9011A            jmp 0x5953
00003F52  51                push cx
00003F53  56                push si
00003F54  57                push di
00003F55  06                push es
00003F56  8B7702            mov si,[bx+0x2]
00003F59  833F05            cmp word [bx],0x5
00003F5C  722B              jc 0x3f89
00003F5E  807C043A          cmp byte [si+0x4],0x3a
00003F62  7525              jnz 0x3f89
00003F64  BF560E            mov di,0xe56
00003F67  0E                push cs
00003F68  07                pop es
00003F69  26803D00          cmp byte [es:di],0x0
00003F6D  741A              jz 0x3f89
00003F6F  56                push si
00003F70  B90400            mov cx,0x4
00003F73  AC                lodsb
00003F74  E80A2C            call 0x6b81
00003F77  AE                scasb
00003F78  7405              jz 0x3f7f
00003F7A  03F9              add di,cx
00003F7C  5E                pop si
00003F7D  EBEA              jmp 0x3f69
00003F7F  E2F2              loop 0x3f73
00003F81  5E                pop si
00003F82  268A05            mov al,[es:di]
00003F85  0AC0              or al,al
00003F87  EB02              jmp 0x3f8b
00003F89  32C0              xor al,al
00003F8B  07                pop es
00003F8C  5F                pop di
00003F8D  5E                pop si
00003F8E  59                pop cx
00003F8F  C3                ret
00003F90  5C                pop sp
00003F91  107E10            adc [bp+0x10],bh
00003F94  7511              jnz 0x3fa7
00003F96  3210              xor dl,[bx+si]
00003F98  A91116            test ax,0x1611
00003F9B  13D4              adc dx,sp
00003F9D  132E10B5          adc bp,[0xb510]
00003FA1  116812            adc [bx+si+0x12],bp
00003FA4  AD                lodsw
00003FA5  11B11179          adc [bx+di+0x7911],si
00003FA9  29AC12A9          sub [si-0x56ee],bp
00003FAD  12FF              adc bh,bh
00003FAF  4C                dec sp
00003FB0  10C3              adc bl,al
00003FB2  53                push bx
00003FB3  51                push cx
00003FB4  52                push dx
00003FB5  89363606          mov [0x636],si
00003FB9  8B5C01            mov bx,[si+0x1]
00003FBC  F6440580          test byte [si+0x5],0x80
00003FC0  750B              jnz 0x3fcd
00003FC2  F6040A            test byte [si],0xa
00003FC5  7406              jz 0x3fcd
00003FC7  E85303            call 0x431d
00003FCA  E80B01            call 0x40d8
00003FCD  E87A03            call 0x434a
00003FD0  C70636060000      mov word [0x636],0x0
00003FD6  5A                pop dx
00003FD7  59                pop cx
00003FD8  5B                pop bx
00003FD9  E9300E            jmp 0x4e0c
00003FDC  F6040A            test byte [si],0xa
00003FDF  751A              jnz 0x3ffb
00003FE1  B8FFFF            mov ax,0xffff
00003FE4  F6440504          test byte [si+0x5],0x4
00003FE8  7410              jz 0x3ffa
00003FEA  F60424            test byte [si],0x24
00003FED  750A              jnz 0x3ff9
00003FEF  50                push ax
00003FF0  E84201            call 0x4135
00003FF3  58                pop ax
00003FF4  7204              jc 0x3ffa
00003FF6  E8B5FF            call 0x3fae
00003FF9  40                inc ax
00003FFA  C3                ret
00003FFB  E93419            jmp 0x5932
00003FFE  E82800            call 0x4029
00004001  7325              jnc 0x4028
00004003  B90700            mov cx,0x7
00004006  33DB              xor bx,bx
00004008  D1DA              rcr dx,1
0000400A  D1D8              rcr ax,1
0000400C  D1DB              rcr bx,1
0000400E  E2F8              loop 0x4008
00004010  803C01            cmp byte [si],0x1
00004013  7513              jnz 0x4028
00004015  0BDB              or bx,bx
00004017  7408              jz 0x4021
00004019  050100            add ax,0x1
0000401C  83D200            adc dx,0x0
0000401F  EB07              jmp 0x4028
00004021  0BDA              or bx,dx
00004023  0BD8              or bx,ax
00004025  7501              jnz 0x4028
00004027  40                inc ax
00004028  C3                ret
00004029  F60424            test byte [si],0x24
0000402C  7525              jnz 0x4053
0000402E  B80100            mov ax,0x1
00004031  33D2              xor dx,dx
00004033  F6440580          test byte [si+0x5],0x80
00004037  752E              jnz 0x4067
00004039  E8A100            call 0x40dd
0000403C  F60401            test byte [si],0x1
0000403F  7406              jz 0x4047
00004041  2B440C            sub ax,[si+0xc]
00004044  83DA00            sbb dx,0x0
00004047  034410            add ax,[si+0x10]
0000404A  83D200            adc dx,0x0
0000404D  F9                stc
0000404E  7918              jns 0x4068
00004050  E9F718            jmp 0x594a
00004053  8B440C            mov ax,[si+0xc]
00004056  8B540E            mov dx,[si+0xe]
00004059  F6440580          test byte [si+0x5],0x80
0000405D  7408              jz 0x4067
0000405F  91                xchg ax,cx
00004060  8B4406            mov ax,[si+0x6]
00004063  E8ED01            call 0x4253
00004066  91                xchg ax,cx
00004067  F8                clc
00004068  C3                ret
00004069  55                push bp
0000406A  8BEC              mov bp,sp
0000406C  56                push si
0000406D  8B5E06            mov bx,[bp+0x6]
00004070  33C0              xor ax,ax
00004072  33D2              xor dx,dx
00004074  E85000            call 0x40c7
00004077  750B              jnz 0x4084
00004079  E8ADFF            call 0x4029
0000407C  050100            add ax,0x1
0000407F  83D200            adc dx,0x0
00004082  7840              js 0x40c4
00004084  5E                pop si
00004085  5D                pop bp
00004086  CA0200            retf word 0x2
00004089  55                push bp
0000408A  8BEC              mov bp,sp
0000408C  56                push si
0000408D  8B5E0A            mov bx,[bp+0xa]
00004090  E83400            call 0x40c7
00004093  752A              jnz 0x40bf
00004095  F6440580          test byte [si+0x5],0x80
00004099  7524              jnz 0x40bf
0000409B  F6040A            test byte [si],0xa
0000409E  7403              jz 0x40a3
000040A0  E87A02            call 0x431d
000040A3  F60401            test byte [si],0x1
000040A6  7405              jz 0x40ad
000040A8  C7440C0000        mov word [si+0xc],0x0
000040AD  8B4E06            mov cx,[bp+0x6]
000040B0  8B5608            mov dx,[bp+0x8]
000040B3  B002              mov al,0x2
000040B5  E8A501            call 0x425d
000040B8  E81900            call 0x40d4
000040BB  804C0504          or byte [si+0x5],0x4
000040BF  5E                pop si
000040C0  5D                pop bp
000040C1  CA0600            retf word 0x6
000040C4  E98318            jmp 0x594a
000040C7  E83B11            call 0x5205
000040CA  7405              jz 0x40d1
000040CC  807C0300          cmp byte [si+0x3],0x0
000040D0  C3                ret
000040D1  E95818            jmp 0x592c
000040D4  33C0              xor ax,ax
000040D6  EB0F              jmp 0x40e7
000040D8  B80200            mov ax,0x2
000040DB  EB06              jmp 0x40e3
000040DD  B80100            mov ax,0x1
000040E0  3D33C0            cmp ax,0xc033
000040E3  33C9              xor cx,cx
000040E5  8BD1              mov dx,cx
000040E7  53                push bx
000040E8  87CA              xchg cx,dx
000040EA  8B5C01            mov bx,[si+0x1]
000040ED  B442              mov ah,0x42
000040EF  CD21              int byte 0x21
000040F1  72D1              jc 0x40c4
000040F3  5B                pop bx
000040F4  C3                ret
000040F5  F6440580          test byte [si+0x5],0x80
000040F9  751D              jnz 0x4118
000040FB  F6040A            test byte [si],0xa
000040FE  7403              jz 0x4103
00004100  E81A02            call 0x431d
00004103  E8D7FF            call 0x40dd
00004106  52                push dx
00004107  50                push ax
00004108  E8CDFF            call 0x40d8
0000410B  59                pop cx
0000410C  5B                pop bx
0000410D  52                push dx
0000410E  50                push ax
0000410F  8BD3              mov dx,bx
00004111  E8C0FF            call 0x40d4
00004114  58                pop ax
00004115  5A                pop dx
00004116  EB10              jmp 0x4128
00004118  B80100            mov ax,0x1
0000411B  33D2              xor dx,dx
0000411D  F60424            test byte [si],0x24
00004120  7406              jz 0x4128
00004122  8B4408            mov ax,[si+0x8]
00004125  8B540A            mov dx,[si+0xa]
00004128  C3                ret
00004129  885404            mov [si+0x4],dl
0000412C  C3                ret
0000412D  8A6412            mov ah,[si+0x12]
00004130  C3                ret
00004131  8A6404            mov ah,[si+0x4]
00004134  C3                ret
00004135  53                push bx
00004136  51                push cx
00004137  52                push dx
00004138  57                push di
00004139  F6440504          test byte [si+0x5],0x4
0000413D  7441              jz 0x4180
0000413F  803C04            cmp byte [si],0x4
00004142  7441              jz 0x4185
00004144  8D7C13            lea di,[si+0x13]
00004147  8B5C10            mov bx,[si+0x10]
0000414A  3B5C0C            cmp bx,[si+0xc]
0000414D  751A              jnz 0x4169
0000414F  B0FE              mov al,0xfe
00004151  8B5C06            mov bx,[si+0x6]
00004154  06                push es
00004155  1E                push ds
00004156  07                pop es
00004157  E8A601            call 0x4300
0000415A  07                pop es
0000415B  7234              jc 0x4191
0000415D  0BC0              or ax,ax
0000415F  741B              jz 0x417c
00004161  89440C            mov [si+0xc],ax
00004164  33DB              xor bx,bx
00004166  895C10            mov [si+0x10],bx
00004169  8A4013            mov al,[bx+si+0x13]
0000416C  FF4410            inc word [si+0x10]
0000416F  F64405A0          test byte [si+0x5],0xa0
00004173  7517              jnz 0x418c
00004175  3C1A              cmp al,0x1a
00004177  7512              jnz 0x418b
00004179  E832FE            call 0x3fae
0000417C  806405FB          and byte [si+0x5],0xfb
00004180  B01A              mov al,0x1a
00004182  F9                stc
00004183  EB07              jmp 0x418c
00004185  E82600            call 0x41ae
00004188  8A4012            mov al,[bx+si+0x12]
0000418B  F8                clc
0000418C  5F                pop di
0000418D  5A                pop dx
0000418E  59                pop cx
0000418F  5B                pop bx
00004190  C3                ret
00004191  0BF6              or si,si
00004193  740B              jz 0x41a0
00004195  F6440580          test byte [si+0x5],0x80
00004199  9C                pushf
0000419A  E89C06            call 0x4839
0000419D  9D                popf
0000419E  750B              jnz 0x41ab
000041A0  E8B31B            call 0x5d56
000041A3  7503              jnz 0x41a8
000041A5  E9B117            jmp 0x5959
000041A8  E9BD17            jmp 0x5968
000041AB  E98D17            jmp 0x593b
000041AE  8B5C10            mov bx,[si+0x10]
000041B1  3B5C06            cmp bx,[si+0x6]
000041B4  7405              jz 0x41bb
000041B6  43                inc bx
000041B7  895C10            mov [si+0x10],bx
000041BA  C3                ret
000041BB  A1480B            mov ax,[0xb48]
000041BE  894410            mov [si+0x10],ax
000041C1  E96217            jmp 0x5926
000041C4  3BC1              cmp ax,cx
000041C6  741F              jz 0x41e7
000041C8  F6440502          test byte [si+0x5],0x2
000041CC  7407              jz 0x41d5
000041CE  50                push ax
000041CF  40                inc ax
000041D0  3BC1              cmp ax,cx
000041D2  58                pop ax
000041D3  7412              jz 0x41e7
000041D5  F6440580          test byte [si+0x5],0x80
000041D9  9C                pushf
000041DA  89363606          mov [0x636],si
000041DE  E85806            call 0x4839
000041E1  9D                popf
000041E2  75C7              jnz 0x41ab
000041E4  E95D17            jmp 0x5944
000041E7  C3                ret
000041E8  803C04            cmp byte [si],0x4
000041EB  7422              jz 0x420f
000041ED  F6440580          test byte [si+0x5],0x80
000041F1  7404              jz 0x41f7
000041F3  3C1A              cmp al,0x1a
000041F5  7431              jz 0x4228
000041F7  53                push bx
000041F8  8B5C10            mov bx,[si+0x10]
000041FB  3B5C06            cmp bx,[si+0x6]
000041FE  7207              jc 0x4207
00004200  51                push cx
00004201  E81901            call 0x431d
00004204  59                pop cx
00004205  33DB              xor bx,bx
00004207  884013            mov [bx+si+0x13],al
0000420A  FF4410            inc word [si+0x10]
0000420D  EB07              jmp 0x4216
0000420F  53                push bx
00004210  E89BFF            call 0x41ae
00004213  884012            mov [bx+si+0x12],al
00004216  5B                pop bx
00004217  3C0D              cmp al,0xd
00004219  7506              jnz 0x4221
0000421B  C6441200          mov byte [si+0x12],0x0
0000421F  EB07              jmp 0x4228
00004221  3C20              cmp al,0x20
00004223  F5                cmc
00004224  80541200          adc byte [si+0x12],0x0
00004228  C3                ret
00004229  B440              mov ah,0x40
0000422B  3DB43F            cmp ax,0x3fb4
0000422E  53                push bx
0000422F  1E                push ds
00004230  FF7401            push word [si+0x1]
00004233  8EDA              mov ds,dx
00004235  8BD3              mov dx,bx
00004237  5B                pop bx
00004238  CD21              int byte 0x21
0000423A  7202              jc 0x423e
0000423C  3BC1              cmp ax,cx
0000423E  1F                pop ds
0000423F  5B                pop bx
00004240  C3                ret
00004241  50                push ax
00004242  51                push cx
00004243  57                push di
00004244  03F8              add di,ax
00004246  33C0              xor ax,ax
00004248  D1E9              shr cx,1
0000424A  F3AB              rep stosw
0000424C  7301              jnc 0x424f
0000424E  AA                stosb
0000424F  5F                pop di
00004250  59                pop cx
00004251  58                pop ax
00004252  C3                ret
00004253  53                push bx
00004254  91                xchg ax,cx
00004255  E83429            call 0x6b8c
00004258  91                xchg ax,cx
00004259  7238              jc 0x4293
0000425B  5B                pop bx
0000425C  C3                ret
0000425D  A802              test al,0x2
0000425F  7419              jz 0x427a
00004261  0BD2              or dx,dx
00004263  782E              js 0x4293
00004265  7502              jnz 0x4269
00004267  E32A              jcxz 0x4293
00004269  83E901            sub cx,0x1
0000426C  83DA00            sbb dx,0x0
0000426F  F60424            test byte [si],0x24
00004272  7419              jz 0x428d
00004274  894C0C            mov [si+0xc],cx
00004277  89540E            mov [si+0xe],dx
0000427A  8B4C0C            mov cx,[si+0xc]
0000427D  8B540E            mov dx,[si+0xe]
00004280  F60420            test byte [si],0x20
00004283  7508              jnz 0x428d
00004285  50                push ax
00004286  8B4406            mov ax,[si+0x6]
00004289  E8C7FF            call 0x4253
0000428C  58                pop ax
0000428D  C744100000        mov word [si+0x10],0x0
00004292  C3                ret
00004293  E9B416            jmp 0x594a
00004296  51                push cx
00004297  57                push di
00004298  87D1              xchg dx,cx
0000429A  E8C0FF            call 0x425d
0000429D  F6440580          test byte [si+0x5],0x80
000042A1  7545              jnz 0x42e8
000042A3  A810              test al,0x10
000042A5  7505              jnz 0x42ac
000042A7  50                push ax
000042A8  E829FE            call 0x40d4
000042AB  58                pop ax
000042AC  50                push ax
000042AD  C43E3806          les di,word [0x638]
000042B1  E84C00            call 0x4300
000042B4  5A                pop dx
000042B5  7246              jc 0x42fd
000042B7  F60420            test byte [si],0x20
000042BA  7405              jz 0x42c1
000042BC  01440C            add [si+0xc],ax
000042BF  EB09              jmp 0x42ca
000042C1  F6C208            test dl,0x8
000042C4  750A              jnz 0x42d0
000042C6  83440C01          add word [si+0xc],0x1
000042CA  83540E00          adc word [si+0xe],0x0
000042CE  78C3              js 0x4293
000042D0  804C0504          or byte [si+0x5],0x4
000042D4  F6C201            test dl,0x1
000042D7  751C              jnz 0x42f5
000042D9  8BCB              mov cx,bx
000042DB  2BC8              sub cx,ax
000042DD  E31B              jcxz 0x42fa
000042DF  806405FB          and byte [si+0x5],0xfb
000042E3  E85BFF            call 0x4241
000042E6  EB12              jmp 0x42fa
000042E8  A801              test al,0x1
000042EA  74C0              jz 0x42ac
000042EC  015C08            add [si+0x8],bx
000042EF  83540A00          adc word [si+0xa],0x0
000042F3  EBB7              jmp 0x42ac
000042F5  8BCB              mov cx,bx
000042F7  E8CAFE            call 0x41c4
000042FA  5F                pop di
000042FB  59                pop cx
000042FC  C3                ret
000042FD  E991FE            jmp 0x4191
00004300  53                push bx
00004301  51                push cx
00004302  52                push dx
00004303  A801              test al,0x1
00004305  8BCB              mov cx,bx
00004307  8B5C01            mov bx,[si+0x1]
0000430A  8BD7              mov dx,di
0000430C  7503              jnz 0x4311
0000430E  B43F              mov ah,0x3f
00004310  3DB440            cmp ax,0x40b4
00004313  1E                push ds
00004314  06                push es
00004315  1F                pop ds
00004316  CD21              int byte 0x21
00004318  1F                pop ds
00004319  5A                pop dx
0000431A  59                pop cx
0000431B  5B                pop bx
0000431C  C3                ret
0000431D  33C9              xor cx,cx
0000431F  F6440540          test byte [si+0x5],0x40
00004323  7407              jz 0x432c
00004325  806405BF          and byte [si+0x5],0xbf
00004329  E80500            call 0x4331
0000432C  874C10            xchg cx,[si+0x10]
0000432F  E315              jcxz 0x4346
00004331  50                push ax
00004332  53                push bx
00004333  52                push dx
00004334  8D5413            lea dx,[si+0x13]
00004337  8B5C01            mov bx,[si+0x1]
0000433A  B440              mov ah,0x40
0000433C  CD21              int byte 0x21
0000433E  7207              jc 0x4347
00004340  E881FE            call 0x41c4
00004343  5A                pop dx
00004344  5B                pop bx
00004345  58                pop ax
00004346  C3                ret
00004347  E947FE            jmp 0x4191
0000434A  50                push ax
0000434B  B43E              mov ah,0x3e
0000434D  CD21              int byte 0x21
0000434F  72F6              jc 0x4347
00004351  58                pop ax
00004352  C3                ret
00004353  005706            add [bx+0x6],dl
00004356  1E                push ds
00004357  07                pop es
00004358  41                inc cx
00004359  E20D              loop 0x4368
0000435B  B98000            mov cx,0x80
0000435E  F606260704        test byte [0x726],0x4
00004363  7503              jnz 0x4368
00004365  B90002            mov cx,0x200
00004368  F606260720        test byte [0x726],0x20
0000436D  7403              jz 0x4372
0000436F  B90100            mov cx,0x1
00004372  51                push cx
00004373  53                push bx
00004374  50                push ax
00004375  BFC707            mov di,0x7c7
00004378  33F6              xor si,si
0000437A  F606260725        test byte [0x726],0x25
0000437F  7503              jnz 0x4384
00004381  E89E01            call 0x4522
00004384  33DB              xor bx,bx
00004386  803E250701        cmp byte [0x725],0x1
0000438B  7417              jz 0x43a4
0000438D  803E260701        cmp byte [0x726],0x1
00004392  7410              jz 0x43a4
00004394  43                inc bx
00004395  803E250702        cmp byte [0x725],0x2
0000439A  7408              jz 0x43a4
0000439C  803E260702        cmp byte [0x726],0x2
000043A1  7401              jz 0x43a4
000043A3  43                inc bx
000043A4  8BFB              mov di,bx
000043A6  E86401            call 0x450d
000043A9  7211              jc 0x43bc
000043AB  803E260708        cmp byte [0x726],0x8
000043B0  7558              jnz 0x440a
000043B2  83FF01            cmp di,0x1
000043B5  7553              jnz 0x440a
000043B7  E84501            call 0x44ff
000043BA  EB70              jmp 0x442c
000043BC  3D0300            cmp ax,0x3
000043BF  740F              jz 0x43d0
000043C1  803E260701        cmp byte [0x726],0x1
000043C6  750B              jnz 0x43d3
000043C8  3D0500            cmp ax,0x5
000043CB  7450              jz 0x441d
000043CD  E95F15            jmp 0x592f
000043D0  E99815            jmp 0x596b
000043D3  3D0200            cmp ax,0x2
000043D6  7516              jnz 0x43ee
000043D8  BAC707            mov dx,0x7c7
000043DB  33C9              xor cx,cx
000043DD  B43C              mov ah,0x3c
000043DF  CD21              int byte 0x21
000043E1  724C              jc 0x442f
000043E3  93                xchg ax,bx
000043E4  E81801            call 0x44ff
000043E7  8BDF              mov bx,di
000043E9  E82101            call 0x450d
000043EC  7353              jnc 0x4441
000043EE  3D0500            cmp ax,0x5
000043F1  752D              jnz 0x4420
000043F3  803E250700        cmp byte [0x725],0x0
000043F8  7523              jnz 0x441d
000043FA  F606260724        test byte [0x726],0x24
000043FF  740B              jz 0x440c
00004401  0BFF              or di,di
00004403  7418              jz 0x441d
00004405  8BDF              mov bx,di
00004407  4B                dec bx
00004408  EB9A              jmp 0x43a4
0000440A  EB35              jmp 0x4441
0000440C  803E260708        cmp byte [0x726],0x8
00004411  750A              jnz 0x441d
00004413  83FF02            cmp di,0x2
00004416  7505              jnz 0x441d
00004418  BB0100            mov bx,0x1
0000441B  EB87              jmp 0x43a4
0000441D  E980FD            jmp 0x41a0
00004420  3D0200            cmp ax,0x2
00004423  74A8              jz 0x43cd
00004425  803E260702        cmp byte [0x726],0x2
0000442A  7412              jz 0x443e
0000442C  E93915            jmp 0x5968
0000442F  3D0500            cmp ax,0x5
00004432  75EC              jnz 0x4420
00004434  E81F19            call 0x5d56
00004437  7205              jc 0x443e
00004439  3D5200            cmp ax,0x52
0000443C  75EE              jnz 0x442c
0000443E  E90F15            jmp 0x5950
00004441  58                pop ax
00004442  5A                pop dx
00004443  59                pop cx
00004444  87D3              xchg dx,bx
00004446  52                push dx
00004447  51                push cx
00004448  030E4808          add cx,[0x848]
0000444C  B2FF              mov dl,0xff
0000444E  B4FF              mov ah,0xff
00004450  E8FA00            call 0x454d
00004453  59                pop cx
00004454  894C06            mov [si+0x6],cx
00004457  8D7C13            lea di,[si+0x13]
0000445A  03F9              add di,cx
0000445C  8B0E4808          mov cx,[0x848]
00004460  56                push si
00004461  BEC707            mov si,0x7c7
00004464  F3A4              rep movsb
00004466  5E                pop si
00004467  5B                pop bx
00004468  895C01            mov [si+0x1],bx
0000446B  804C0504          or byte [si+0x5],0x4
0000446F  B000              mov al,0x0
00004471  B444              mov ah,0x44
00004473  CD21              int byte 0x21
00004475  8AC2              mov al,dl
00004477  A880              test al,0x80
00004479  742A              jz 0x44a5
0000447B  2483              and al,0x83
0000447D  084405            or [si+0x5],al
00004480  B001              mov al,0x1
00004482  83CA20            or dx,0x20
00004485  B600              mov dh,0x0
00004487  B444              mov ah,0x44
00004489  CD21              int byte 0x21
0000448B  726C              jc 0x44f9
0000448D  F606260724        test byte [0x726],0x24
00004492  7511              jnz 0x44a5
00004494  C744060100        mov word [si+0x6],0x1
00004499  803E260708        cmp byte [0x726],0x8
0000449E  7505              jnz 0x44a5
000044A0  C606260702        mov byte [0x726],0x2
000044A5  A02607            mov al,[0x726]
000044A8  8804              mov [si],al
000044AA  F6440580          test byte [si+0x5],0x80
000044AE  7508              jnz 0x44b8
000044B0  A802              test al,0x2
000044B2  7404              jz 0x44b8
000044B4  804C0540          or byte [si+0x5],0x40
000044B8  3C08              cmp al,0x8
000044BA  753A              jnz 0x44f6
000044BC  E819FC            call 0x40d8
000044BF  8BC8              mov cx,ax
000044C1  0BC2              or ax,dx
000044C3  742C              jz 0x44f1
000044C5  81E98000          sub cx,0x80
000044C9  83DA00            sbb dx,0x0
000044CC  7304              jnc 0x44d2
000044CE  33C9              xor cx,cx
000044D0  33D2              xor dx,dx
000044D2  E8FFFB            call 0x40d4
000044D5  91                xchg ax,cx
000044D6  89363606          mov [0x636],si
000044DA  E858FC            call 0x4135
000044DD  C70636060000      mov word [0x636],0x0
000044E3  7208              jc 0x44ed
000044E5  83C101            add cx,0x1
000044E8  83D200            adc dx,0x0
000044EB  EBE5              jmp 0x44d2
000044ED  E8E4FB            call 0x40d4
000044F0  91                xchg ax,cx
000044F1  C744100000        mov word [si+0x10],0x0
000044F6  07                pop es
000044F7  5F                pop di
000044F8  C3                ret
000044F9  E87C09            call 0x4e78
000044FC  E93C14            jmp 0x593b
000044FF  891E3406          mov [0x634],bx
00004503  E844FE            call 0x434a
00004506  C70634060000      mov word [0x634],0x0
0000450C  C3                ret
0000450D  BAC707            mov dx,0x7c7
00004510  0A1E2407          or bl,[0x724]
00004514  8BD2              mov dx,dx
00004516  8BC3              mov ax,bx
00004518  B43D              mov ah,0x3d
0000451A  CD21              int byte 0x21
0000451C  7203              jc 0x4521
0000451E  93                xchg ax,bx
0000451F  33C0              xor ax,ax
00004521  C3                ret
00004522  56                push si
00004523  33F6              xor si,si
00004525  E8660A            call 0x4f8e
00004528  741F              jz 0x4549
0000452A  807C0300          cmp byte [si+0x3],0x0
0000452E  75F5              jnz 0x4525
00004530  8D5C13            lea bx,[si+0x13]
00004533  035C06            add bx,[si+0x6]
00004536  87DE              xchg bx,si
00004538  57                push di
00004539  AC                lodsb
0000453A  AE                scasb
0000453B  7507              jnz 0x4544
0000453D  0AC0              or al,al
0000453F  75F8              jnz 0x4539
00004541  E9F113            jmp 0x5935
00004544  5F                pop di
00004545  87DE              xchg bx,si
00004547  EBDC              jmp 0x4525
00004549  5E                pop si
0000454A  C3                ret
0000454B  33C9              xor cx,cx
0000454D  51                push cx
0000454E  84262607          test [0x726],ah
00004552  7421              jz 0x4575
00004554  E303              jcxz 0x4559
00004556  83C10D            add cx,0xd
00004559  87D9              xchg bx,cx
0000455B  83C306            add bx,0x6
0000455E  52                push dx
0000455F  B208              mov dl,0x8
00004561  E8DC0A            call 0x5040
00004564  5A                pop dx
00004565  87D9              xchg bx,cx
00004567  8A0E2607          mov cl,[0x726]
0000456B  880C              mov [si],cl
0000456D  884403            mov [si+0x3],al
00004570  885404            mov [si+0x4],dl
00004573  59                pop cx
00004574  C3                ret
00004575  E9BA13            jmp 0x5932
00004578  55                push bp
00004579  8BEC              mov bp,sp
0000457B  8B4606            mov ax,[bp+0x6]
0000457E  8ADC              mov bl,ah
00004580  80E3F0            and bl,0xf0
00004583  80E40F            and ah,0xf
00004586  88262507          mov [0x725],ah
0000458A  881E2407          mov [0x724],bl
0000458E  0AE3              or ah,bl
00004590  7405              jz 0x4597
00004592  E8F116            call 0x5c86
00004595  7212              jc 0x45a9
00004597  32E4              xor ah,ah
00004599  8B5E0A            mov bx,[bp+0xa]
0000459C  8B4E08            mov cx,[bp+0x8]
0000459F  8B560C            mov dx,[bp+0xc]
000045A2  E8A000            call 0x4645
000045A5  5D                pop bp
000045A6  CA0800            retf word 0x8
000045A9  E9B613            jmp 0x5962
000045AC  55                push bp
000045AD  8BEC              mov bp,sp
000045AF  C606250700        mov byte [0x725],0x0
000045B4  C606240700        mov byte [0x724],0x0
000045B9  8B5E0C            mov bx,[bp+0xc]
000045BC  8B0F              mov cx,[bx]
000045BE  E3B5              jcxz 0x4575
000045C0  53                push bx
000045C1  8B5F02            mov bx,[bx+0x2]
000045C4  8A1F              mov bl,[bx]
000045C6  80E3DF            and bl,0xdf
000045C9  B80100            mov ax,0x1
000045CC  80FB49            cmp bl,0x49
000045CF  741B              jz 0x45ec
000045D1  40                inc ax
000045D2  80FB4F            cmp bl,0x4f
000045D5  7415              jz 0x45ec
000045D7  B004              mov al,0x4
000045D9  80FB52            cmp bl,0x52
000045DC  740E              jz 0x45ec
000045DE  B008              mov al,0x8
000045E0  80FB41            cmp bl,0x41
000045E3  7407              jz 0x45ec
000045E5  80FB42            cmp bl,0x42
000045E8  758B              jnz 0x4575
000045EA  B020              mov al,0x20
000045EC  5B                pop bx
000045ED  E82905            call 0x4b19
000045F0  8B5E0A            mov bx,[bp+0xa]
000045F3  8B4E06            mov cx,[bp+0x6]
000045F6  8B5608            mov dx,[bp+0x8]
000045F9  E84900            call 0x4645
000045FC  5D                pop bp
000045FD  CA0800            retf word 0x8
00004600  B406              mov ah,0x6
00004602  56                push si
00004603  57                push di
00004604  50                push ax
00004605  8A4403            mov al,[si+0x3]
00004608  F6D8              neg al
0000460A  98                cbw
0000460B  D1E0              shl ax,1
0000460D  8B3E7209          mov di,[0x972]
00004611  2E3B05            cmp ax,[cs:di]
00004614  7203              jc 0x4619
00004616  E93A13            jmp 0x5953
00004619  47                inc di
0000461A  47                inc di
0000461B  93                xchg ax,bx
0000461C  2E8B39            mov di,[cs:bx+di]
0000461F  87DF              xchg bx,di
00004621  93                xchg ax,bx
00004622  A33C06            mov [0x63c],ax
00004625  58                pop ax
00004626  50                push ax
00004627  86E0              xchg ah,al
00004629  98                cbw
0000462A  57                push di
0000462B  8B3E3C06          mov di,[0x63c]
0000462F  03F8              add di,ax
00004631  2E8B05            mov ax,[cs:di]
00004634  A33C06            mov [0x63c],ax
00004637  5F                pop di
00004638  58                pop ax
00004639  FF163C06          call word near [0x63c]
0000463D  5F                pop di
0000463E  5E                pop si
0000463F  C3                ret
00004640  56                push si
00004641  57                push di
00004642  50                push ax
00004643  EBC3              jmp 0x4608
00004645  56                push si
00004646  A22607            mov [0x726],al
00004649  51                push cx
0000464A  E81E00            call 0x466b
0000464D  E8B50B            call 0x5205
00004650  7564              jnz 0x46b6
00004652  59                pop cx
00004653  41                inc cx
00004654  7405              jz 0x465b
00004656  49                dec cx
00004657  0BC9              or cx,cx
00004659  7E0D              jng 0x4668
0000465B  B40C              mov ah,0xc
0000465D  E8E0FF            call 0x4640
00004660  C7064D0D0000      mov word [0xd4d],0x0
00004666  5E                pop si
00004667  C3                ret
00004668  E98E12            jmp 0x58f9
0000466B  56                push si
0000466C  57                push di
0000466D  06                push es
0000466E  87D3              xchg dx,bx
00004670  BFC707            mov di,0x7c7
00004673  1E                push ds
00004674  07                pop es
00004675  8B0F              mov cx,[bx]
00004677  E343              jcxz 0x46bc
00004679  E8D6F8            call 0x3f52
0000467C  740E              jz 0x468c
0000467E  8B7702            mov si,[bx+0x2]
00004681  51                push cx
00004682  F3A4              rep movsb
00004684  880D              mov [di],cl
00004686  59                pop cx
00004687  42                inc dx
00004688  74DE              jz 0x4668
0000468A  EB16              jmp 0x46a2
0000468C  E875F7            call 0x3e04
0000468F  A801              test al,0x1
00004691  7526              jnz 0x46b9
00004693  42                inc dx
00004694  750A              jnz 0x46a0
00004696  BE6409            mov si,0x964
00004699  E883F8            call 0x3f1f
0000469C  32C0              xor al,al
0000469E  EB09              jmp 0x46a9
000046A0  32C0              xor al,al
000046A2  4A                dec dx
000046A3  7417              jz 0x46bc
000046A5  0AF6              or dh,dh
000046A7  7513              jnz 0x46bc
000046A9  E86D04            call 0x4b19
000046AC  890E4808          mov [0x848],cx
000046B0  87D3              xchg dx,bx
000046B2  07                pop es
000046B3  5F                pop di
000046B4  5E                pop si
000046B5  C3                ret
000046B6  E97C12            jmp 0x5935
000046B9  E99112            jmp 0x594d
000046BC  E96D12            jmp 0x592c
000046BF  55                push bp
000046C0  8BEC              mov bp,sp
000046C2  56                push si
000046C3  57                push di
000046C4  8D7E08            lea di,[bp+0x8]
000046C7  8B4E06            mov cx,[bp+0x6]
000046CA  E310              jcxz 0x46dc
000046CC  8B1D              mov bx,[di]
000046CE  E8340B            call 0x5205
000046D1  7403              jz 0x46d6
000046D3  E82AFF            call 0x4600
000046D6  47                inc di
000046D7  47                inc di
000046D8  E2F2              loop 0x46cc
000046DA  EB03              jmp 0x46df
000046DC  E83001            call 0x480f
000046DF  8BDF              mov bx,di
000046E1  5F                pop di
000046E2  5E                pop si
000046E3  8B4602            mov ax,[bp+0x2]
000046E6  8B5604            mov dx,[bp+0x4]
000046E9  8B6E00            mov bp,[bp+0x0]
000046EC  8BE3              mov sp,bx
000046EE  C7064D0D0000      mov word [0xd4d],0x0
000046F4  52                push dx
000046F5  50                push ax
000046F6  CB                retf
000046F7  56                push si
000046F8  8B364D0D          mov si,[0xd4d]
000046FC  0BF6              or si,si
000046FE  7511              jnz 0x4711
00004700  F606240B01        test byte [0xb24],0x1
00004705  7405              jz 0x470c
00004707  E8C023            call 0x6aca
0000470A  5E                pop si
0000470B  C3                ret
0000470C  E8170C            call 0x5326
0000470F  EB05              jmp 0x4716
00004711  B410              mov ah,0x10
00004713  E8ECFE            call 0x4602
00004716  5E                pop si
00004717  7203              jc 0x471c
00004719  0BE4              or sp,sp
0000471B  C3                ret
0000471C  0BE4              or sp,sp
0000471E  F9                stc
0000471F  C3                ret
00004720  56                push si
00004721  8B364D0D          mov si,[0xd4d]
00004725  0BF6              or si,si
00004727  7504              jnz 0x472d
00004729  5E                pop si
0000472A  E9420C            jmp 0x536f
0000472D  F60420            test byte [si],0x20
00004730  7403              jz 0x4735
00004732  E9FD11            jmp 0x5932
00004735  B412              mov ah,0x12
00004737  E8C8FE            call 0x4602
0000473A  5E                pop si
0000473B  C3                ret
0000473C  56                push si
0000473D  8B364D0D          mov si,[0xd4d]
00004741  0BF6              or si,si
00004743  7504              jnz 0x4749
00004745  5E                pop si
00004746  E99A0E            jmp 0x55e3
00004749  B414              mov ah,0x14
0000474B  EBEA              jmp 0x4737
0000474D  56                push si
0000474E  8B364D0D          mov si,[0xd4d]
00004752  0BF6              or si,si
00004754  7504              jnz 0x475a
00004756  5E                pop si
00004757  E99B0E            jmp 0x55f5
0000475A  B416              mov ah,0x16
0000475C  EBD9              jmp 0x4737
0000475E  56                push si
0000475F  8B364D0D          mov si,[0xd4d]
00004763  0BF6              or si,si
00004765  7504              jnz 0x476b
00004767  5E                pop si
00004768  E9940E            jmp 0x55ff
0000476B  B41A              mov ah,0x1a
0000476D  EBC8              jmp 0x4737
0000476F  56                push si
00004770  8B364D0D          mov si,[0xd4d]
00004774  0BF6              or si,si
00004776  7504              jnz 0x477c
00004778  5E                pop si
00004779  E9830E            jmp 0x55ff
0000477C  B41C              mov ah,0x1c
0000477E  EBB7              jmp 0x4737
00004780  50                push ax
00004781  53                push bx
00004782  56                push si
00004783  C6063E0601        mov byte [0x63e],0x1
00004788  8B364D0D          mov si,[0xd4d]
0000478C  0BF6              or si,si
0000478E  744F              jz 0x47df
00004790  F6440380          test byte [si+0x3],0x80
00004794  741E              jz 0x47b4
00004796  B00D              mov al,0xd
00004798  E885FF            call 0x4720
0000479B  807C03FE          cmp byte [si+0x3],0xfe
0000479F  7411              jz 0x47b2
000047A1  807C03FC          cmp byte [si+0x3],0xfc
000047A5  7F06              jg 0x47ad
000047A7  807C03FA          cmp byte [si+0x3],0xfa
000047AB  7F05              jg 0x47b2
000047AD  B00A              mov al,0xa
000047AF  E86EFF            call 0x4720
000047B2  EB30              jmp 0x47e4
000047B4  803C04            cmp byte [si],0x4
000047B7  751A              jnz 0x47d3
000047B9  F6068A0A04        test byte [0xa8a],0x4
000047BE  7413              jz 0x47d3
000047C0  8B5C06            mov bx,[si+0x6]
000047C3  2B5C10            sub bx,[si+0x10]
000047C6  83EB02            sub bx,0x2
000047C9  B020              mov al,0x20
000047CB  7406              jz 0x47d3
000047CD  E850FF            call 0x4720
000047D0  4B                dec bx
000047D1  EBF8              jmp 0x47cb
000047D3  B00D              mov al,0xd
000047D5  E848FF            call 0x4720
000047D8  B00A              mov al,0xa
000047DA  E843FF            call 0x4720
000047DD  EB05              jmp 0x47e4
000047DF  B00D              mov al,0xd
000047E1  E83CFF            call 0x4720
000047E4  C6063E0600        mov byte [0x63e],0x0
000047E9  5E                pop si
000047EA  5B                pop bx
000047EB  58                pop ax
000047EC  C3                ret
000047ED  8B0F              mov cx,[bx]
000047EF  8B7702            mov si,[bx+0x2]
000047F2  E316              jcxz 0x480a
000047F4  56                push si
000047F5  8B364D0D          mov si,[0xd4d]
000047F9  0BF6              or si,si
000047FB  740E              jz 0x480b
000047FD  807C03FE          cmp byte [si+0x3],0xfe
00004801  7408              jz 0x480b
00004803  5E                pop si
00004804  AC                lodsb
00004805  E818FF            call 0x4720
00004808  E2FA              loop 0x4804
0000480A  C3                ret
0000480B  5E                pop si
0000480C  E97F27            jmp 0x6f8e
0000480F  56                push si
00004810  803E1A0D00        cmp byte [0xd1a],0x0
00004815  751C              jnz 0x4833
00004817  33F6              xor si,si
00004819  E87207            call 0x4f8e
0000481C  7405              jz 0x4823
0000481E  E8DFFD            call 0x4600
00004821  EBF4              jmp 0x4817
00004823  BE360D            mov si,0xd36
00004826  F6440510          test byte [si+0x5],0x10
0000482A  7407              jz 0x4833
0000482C  806405EF          and byte [si+0x5],0xef
00004830  E8CDFD            call 0x4600
00004833  5E                pop si
00004834  C3                ret
00004835  0200              add al,[bx+si]
00004837  1010              adc [bx+si],dl
00004839  53                push bx
0000483A  56                push si
0000483B  8B1E3406          mov bx,[0x634]
0000483F  0BDB              or bx,bx
00004841  750B              jnz 0x484e
00004843  8B363606          mov si,[0x636]
00004847  0BF6              or si,si
00004849  741E              jz 0x4869
0000484B  8B5C01            mov bx,[si+0x1]
0000484E  50                push ax
0000484F  8BDB              mov bx,bx
00004851  B43E              mov ah,0x3e
00004853  CD21              int byte 0x21
00004855  58                pop ax
00004856  33F6              xor si,si
00004858  87363606          xchg si,[0x636]
0000485C  0BF6              or si,si
0000485E  7403              jz 0x4863
00004860  E8A905            call 0x4e0c
00004863  C70634060000      mov word [0x634],0x0
00004869  5E                pop si
0000486A  5B                pop bx
0000486B  C3                ret
0000486C  C606C4070C        mov byte [0x7c4],0xc
00004871  C706C5070010      mov word [0x7c5],0x1000
00004877  8B0E640C          mov cx,[0xc64]
0000487B  A1620C            mov ax,[0xc62]
0000487E  24FE              and al,0xfe
00004880  3BC1              cmp ax,cx
00004882  7208              jc 0x488c
00004884  890E7409          mov [0x974],cx
00004888  A37609            mov [0x976],ax
0000488B  C3                ret
0000488C  BB0298            mov bx,0x9802
0000488F  EA762A5802        jmp word 0x258:word 0x2a76
00004894  A12E0D            mov ax,[0xd2e]
00004897  E86802            call 0x4b02
0000489A  C3                ret
0000489B  A17409            mov ax,[0x974]
0000489E  8B0E7609          mov cx,[0x976]
000048A2  E86507            call 0x500a
000048A5  7303              jnc 0x48aa
000048A7  E90911            jmp 0x59b3
000048AA  C3                ret
000048AB  06                push es
000048AC  33C0              xor ax,ax
000048AE  A32E0D            mov [0xd2e],ax
000048B1  E84E02            call 0x4b02
000048B4  1E                push ds
000048B5  07                pop es
000048B6  FF167809          call word near [0x978]
000048BA  07                pop es
000048BB  C3                ret
000048BC  50                push ax
000048BD  56                push si
000048BE  43                inc bx
000048BF  7420              jz 0x48e1
000048C1  80CB01            or bl,0x1
000048C4  E82300            call 0x48ea
000048C7  731E              jnc 0x48e7
000048C9  51                push cx
000048CA  52                push dx
000048CB  57                push di
000048CC  E85000            call 0x491f
000048CF  7313              jnc 0x48e4
000048D1  E8FF02            call 0x4bd3
000048D4  E81300            call 0x48ea
000048D7  730B              jnc 0x48e4
000048D9  E8B300            call 0x498f
000048DC  E80B00            call 0x48ea
000048DF  7303              jnc 0x48e4
000048E1  E92A10            jmp 0x590e
000048E4  5F                pop di
000048E5  5A                pop dx
000048E6  59                pop cx
000048E7  5E                pop si
000048E8  58                pop ax
000048E9  C3                ret
000048EA  8B364206          mov si,[0x642]
000048EE  8B04              mov ax,[si]
000048F0  A801              test al,0x1
000048F2  7429              jz 0x491d
000048F4  3DFFFF            cmp ax,0xffff
000048F7  7424              jz 0x491d
000048F9  3BD8              cmp bx,ax
000048FB  7720              ja 0x491d
000048FD  7413              jz 0x4912
000048FF  891C              mov [si],bx
00004901  43                inc bx
00004902  2BC3              sub ax,bx
00004904  87DE              xchg bx,si
00004906  03F3              add si,bx
00004908  8904              mov [si],ax
0000490A  89364206          mov [0x642],si
0000490E  83C302            add bx,0x2
00004911  C3                ret
00004912  F9                stc
00004913  11064206          adc [0x642],ax
00004917  8BDE              mov bx,si
00004919  83C302            add bx,0x2
0000491C  C3                ret
0000491D  F9                stc
0000491E  C3                ret
0000491F  8B364206          mov si,[0x642]
00004923  8B164406          mov dx,[0x644]
00004927  E81B00            call 0x4945
0000492A  7318              jnc 0x4944
0000492C  8B364006          mov si,[0x640]
00004930  87164206          xchg dx,[0x642]
00004934  E80E00            call 0x4945
00004937  730B              jnc 0x4944
00004939  3B164206          cmp dx,[0x642]
0000493D  7704              ja 0x4943
0000493F  89164206          mov [0x642],dx
00004943  F9                stc
00004944  C3                ret
00004945  8B04              mov ax,[si]
00004947  A801              test al,0x1
00004949  7422              jz 0x496d
0000494B  40                inc ax
0000494C  742D              jz 0x497b
0000494E  8BFE              mov di,si
00004950  03F8              add di,ax
00004952  48                dec ax
00004953  8B0D              mov cx,[di]
00004955  F6C101            test cl,0x1
00004958  7409              jz 0x4963
0000495A  41                inc cx
0000495B  7422              jz 0x497f
0000495D  03C1              add ax,cx
0000495F  03F9              add di,cx
00004961  EBF0              jmp 0x4953
00004963  8904              mov [si],ax
00004965  3BC3              cmp ax,bx
00004967  731C              jnc 0x4985
00004969  8BC1              mov ax,cx
0000496B  8BF7              mov si,di
0000496D  8BF8              mov di,ax
0000496F  0335              add si,[di]
00004971  83C603            add si,0x3
00004974  83E6FE            and si,0xfffffffffffffffe
00004977  3BF2              cmp si,dx
00004979  76CA              jna 0x4945
0000497B  8BD6              mov dx,si
0000497D  F9                stc
0000497E  C3                ret
0000497F  8904              mov [si],ax
00004981  3BC3              cmp ax,bx
00004983  7207              jc 0x498c
00004985  89364206          mov [0x642],si
00004989  E971FF            jmp 0x48fd
0000498C  8BD6              mov dx,si
0000498E  C3                ret
0000498F  50                push ax
00004990  53                push bx
00004991  51                push cx
00004992  56                push si
00004993  57                push di
00004994  06                push es
00004995  1E                push ds
00004996  07                pop es
00004997  8B364006          mov si,[0x640]
0000499B  8B1C              mov bx,[si]
0000499D  F6C301            test bl,0x1
000049A0  7514              jnz 0x49b6
000049A2  8B4702            mov ax,[bx+0x2]
000049A5  2D0200            sub ax,0x2
000049A8  3BC6              cmp ax,si
000049AA  7547              jnz 0x49f3
000049AC  0337              add si,[bx]
000049AE  83C603            add si,0x3
000049B1  83E6FE            and si,0xfffffffffffffffe
000049B4  EBE5              jmp 0x499b
000049B6  8BFE              mov di,si
000049B8  43                inc bx
000049B9  7426              jz 0x49e1
000049BB  03F3              add si,bx
000049BD  8B1C              mov bx,[si]
000049BF  F6C301            test bl,0x1
000049C2  75F4              jnz 0x49b8
000049C4  8B4702            mov ax,[bx+0x2]
000049C7  2D0200            sub ax,0x2
000049CA  3BC6              cmp ax,si
000049CC  7525              jnz 0x49f3
000049CE  8BC7              mov ax,di
000049D0  050200            add ax,0x2
000049D3  894702            mov [bx+0x2],ax
000049D6  8B0F              mov cx,[bx]
000049D8  83C103            add cx,0x3
000049DB  D1E9              shr cx,1
000049DD  F3A5              rep movsw
000049DF  EBDC              jmp 0x49bd
000049E1  2BF7              sub si,di
000049E3  7403              jz 0x49e8
000049E5  4E                dec si
000049E6  8935              mov [di],si
000049E8  893E4206          mov [0x642],di
000049EC  07                pop es
000049ED  5F                pop di
000049EE  5E                pop si
000049EF  59                pop cx
000049F0  5B                pop bx
000049F1  58                pop ax
000049F2  C3                ret
000049F3  E97B0F            jmp 0x5971
000049F6  50                push ax
000049F7  56                push si
000049F8  A14006            mov ax,[0x640]
000049FB  3BF0              cmp si,ax
000049FD  7416              jz 0x4a15
000049FF  720B              jc 0x4a0c
00004A01  E88BFF            call 0x498f
00004A04  E8CC01            call 0x4bd3
00004A07  E81500            call 0x4a1f
00004A0A  EB10              jmp 0x4a1c
00004A0C  2BC6              sub ax,si
00004A0E  48                dec ax
00004A0F  8904              mov [si],ax
00004A11  89364006          mov [0x640],si
00004A15  E877FF            call 0x498f
00004A18  E8B801            call 0x4bd3
00004A1B  F8                clc
00004A1C  5E                pop si
00004A1D  58                pop ax
00004A1E  C3                ret
00004A1F  53                push bx
00004A20  51                push cx
00004A21  57                push di
00004A22  06                push es
00004A23  1E                push ds
00004A24  07                pop es
00004A25  96                xchg ax,si
00004A26  2BC6              sub ax,si
00004A28  E8DC01            call 0x4c07
00004A2B  3BD8              cmp bx,ax
00004A2D  7242              jc 0x4a71
00004A2F  8B1C              mov bx,[si]
00004A31  F6C301            test bl,0x1
00004A34  750D              jnz 0x4a43
00004A36  014702            add [bx+0x2],ax
00004A39  0337              add si,[bx]
00004A3B  83C603            add si,0x3
00004A3E  83E6FE            and si,0xfffffffffffffffe
00004A41  EBEC              jmp 0x4a2f
00004A43  8B364206          mov si,[0x642]
00004A47  8BCE              mov cx,si
00004A49  2B0E4006          sub cx,[0x640]
00004A4D  D1E9              shr cx,1
00004A4F  83EE02            sub si,0x2
00004A52  8BFE              mov di,si
00004A54  03F8              add di,ax
00004A56  FD                std
00004A57  F3A5              rep movsw
00004A59  FC                cld
00004A5A  01064006          add [0x640],ax
00004A5E  8B1E4206          mov bx,[0x642]
00004A62  03D8              add bx,ax
00004A64  891E4206          mov [0x642],bx
00004A68  8B0E4406          mov cx,[0x644]
00004A6C  2BCB              sub cx,bx
00004A6E  49                dec cx
00004A6F  890F              mov [bx],cx
00004A71  07                pop es
00004A72  5F                pop di
00004A73  59                pop cx
00004A74  5B                pop bx
00004A75  C3                ret
00004A76  E816FF            call 0x498f
00004A79  E85701            call 0x4bd3
00004A7C  E88801            call 0x4c07
00004A7F  FF364206          push word [0x642]
00004A83  E80500            call 0x4a8b
00004A86  8F064206          pop word [0x642]
00004A8A  C3                ret
00004A8B  0BDB              or bx,bx
00004A8D  742B              jz 0x4aba
00004A8F  833E460600        cmp word [0x646],0x0
00004A94  742B              jz 0x4ac1
00004A96  56                push si
00004A97  53                push bx
00004A98  E821FE            call 0x48bc
00004A9B  8B364606          mov si,[0x646]
00004A9F  8B14              mov dx,[si]
00004AA1  89164606          mov [0x646],dx
00004AA5  8F04              pop word [si]
00004AA7  8977FE            mov [bx-0x2],si
00004AAA  895C02            mov [si+0x2],bx
00004AAD  8B162E0D          mov dx,[0xd2e]
00004AB1  895404            mov [si+0x4],dx
00004AB4  8BD3              mov dx,bx
00004AB6  8BDE              mov bx,si
00004AB8  5E                pop si
00004AB9  C3                ret
00004ABA  BA7A09            mov dx,0x97a
00004ABD  BB7C09            mov bx,0x97c
00004AC0  C3                ret
00004AC1  E94D0E            jmp 0x5911
00004AC4  81FBF809          cmp bx,0x9f8
00004AC8  734E              jnc 0x4b18
00004ACA  81FB8009          cmp bx,0x980
00004ACE  F5                cmc
00004ACF  C3                ret
00004AD0  06                push es
00004AD1  1E                push ds
00004AD2  07                pop es
00004AD3  51                push cx
00004AD4  57                push di
00004AD5  B9FFFF            mov cx,0xffff
00004AD8  B000              mov al,0x0
00004ADA  8BFB              mov di,bx
00004ADC  F2AE              repne scasb
00004ADE  8BC1              mov ax,cx
00004AE0  F7D0              not ax
00004AE2  48                dec ax
00004AE3  5F                pop di
00004AE4  59                pop cx
00004AE5  07                pop es
00004AE6  C3                ret
00004AE7  06                push es
00004AE8  1E                push ds
00004AE9  07                pop es
00004AEA  51                push cx
00004AEB  8BCB              mov cx,bx
00004AED  56                push si
00004AEE  8BF2              mov si,dx
00004AF0  E898FF            call 0x4a8b
00004AF3  57                push di
00004AF4  8BFA              mov di,dx
00004AF6  8BD6              mov dx,si
00004AF8  41                inc cx
00004AF9  D1E9              shr cx,1
00004AFB  F3A5              rep movsw
00004AFD  5F                pop di
00004AFE  5E                pop si
00004AFF  59                pop cx
00004B00  07                pop es
00004B01  C3                ret
00004B02  53                push bx
00004B03  BB8009            mov bx,0x980
00004B06  394704            cmp [bx+0x4],ax
00004B09  7C03              jl 0x4b0e
00004B0B  E81000            call 0x4b1e
00004B0E  83C306            add bx,0x6
00004B11  81FBF809          cmp bx,0x9f8
00004B15  72EF              jc 0x4b06
00004B17  5B                pop bx
00004B18  C3                ret
00004B19  E8A8FF            call 0x4ac4
00004B1C  73FA              jnc 0x4b18
00004B1E  E83900            call 0x4b5a
00004B21  50                push ax
00004B22  8BC3              mov ax,bx
00004B24  87064606          xchg ax,[0x646]
00004B28  8907              mov [bx],ax
00004B2A  C74704FFFF        mov word [bx+0x4],0xffff
00004B2F  58                pop ax
00004B30  C3                ret
00004B31  06                push es
00004B32  1E                push ds
00004B33  07                pop es
00004B34  50                push ax
00004B35  51                push cx
00004B36  56                push si
00004B37  57                push di
00004B38  8BF1              mov si,cx
00004B3A  8BC3              mov ax,bx
00004B3C  8BCA              mov cx,dx
00004B3E  8BDA              mov bx,dx
00004B40  E848FF            call 0x4a8b
00004B43  8BFA              mov di,dx
00004B45  93                xchg ax,bx
00004B46  037702            add si,[bx+0x2]
00004B49  E8CDFF            call 0x4b19
00004B4C  93                xchg ax,bx
00004B4D  8BD1              mov dx,cx
00004B4F  41                inc cx
00004B50  D1E9              shr cx,1
00004B52  F3A5              rep movsw
00004B54  5F                pop di
00004B55  5E                pop si
00004B56  59                pop cx
00004B57  58                pop ax
00004B58  07                pop es
00004B59  C3                ret
00004B5A  06                push es
00004B5B  1E                push ds
00004B5C  07                pop es
00004B5D  50                push ax
00004B5E  33C0              xor ax,ax
00004B60  E80300            call 0x4b66
00004B63  58                pop ax
00004B64  07                pop es
00004B65  C3                ret
00004B66  833F00            cmp word [bx],0x0
00004B69  7431              jz 0x4b9c
00004B6B  51                push cx
00004B6C  56                push si
00004B6D  8B7702            mov si,[bx+0x2]
00004B70  263B364006        cmp si,[es:0x640]
00004B75  7223              jc 0x4b9a
00004B77  263B364406        cmp si,[es:0x644]
00004B7C  7719              ja 0x4b97
00004B7E  0144FE            add [si-0x2],ax
00004B81  0BC0              or ax,ax
00004B83  7515              jnz 0x4b9a
00004B85  8B0F              mov cx,[bx]
00004B87  41                inc cx
00004B88  80C901            or cl,0x1
00004B8B  874CFE            xchg cx,[si-0x2]
00004B8E  3BCB              cmp cx,bx
00004B90  7408              jz 0x4b9a
00004B92  06                push es
00004B93  1F                pop ds
00004B94  E9DA0D            jmp 0x5971
00004B97  E89A03            call 0x4f34
00004B9A  5E                pop si
00004B9B  59                pop cx
00004B9C  C3                ret
00004B9D  52                push dx
00004B9E  8B17              mov dx,[bx]
00004BA0  51                push cx
00004BA1  33C9              xor cx,cx
00004BA3  42                inc dx
00004BA4  E88AFF            call 0x4b31
00004BA7  4A                dec dx
00004BA8  59                pop cx
00004BA9  53                push bx
00004BAA  8B5F02            mov bx,[bx+0x2]
00004BAD  03DA              add bx,dx
00004BAF  C60700            mov byte [bx],0x0
00004BB2  5B                pop bx
00004BB3  5A                pop dx
00004BB4  C3                ret
00004BB5  8B364206          mov si,[0x642]
00004BB9  8B04              mov ax,[si]
00004BBB  A801              test al,0x1
00004BBD  740B              jz 0x4bca
00004BBF  40                inc ax
00004BC0  7410              jz 0x4bd2
00004BC2  03C6              add ax,si
00004BC4  3B064406          cmp ax,[0x644]
00004BC8  7408              jz 0x4bd2
00004BCA  8B364406          mov si,[0x644]
00004BCE  89364206          mov [0x642],si
00004BD2  C3                ret
00004BD3  50                push ax
00004BD4  56                push si
00004BD5  57                push di
00004BD6  E80204            call 0x4fdb
00004BD9  3B3E4C06          cmp di,[0x64c]
00004BDD  7424              jz 0x4c03
00004BDF  C60504            mov byte [di],0x4
00004BE2  893E4C06          mov [0x64c],di
00004BE6  3B3E4A06          cmp di,[0x64a]
00004BEA  7204              jc 0x4bf0
00004BEC  893E4A06          mov [0x64a],di
00004BF0  83EF07            sub di,0x7
00004BF3  C705FFFF          mov word [di],0xffff
00004BF7  E8BBFF            call 0x4bb5
00004BFA  893E4406          mov [0x644],di
00004BFE  2BFE              sub di,si
00004C00  4F                dec di
00004C01  893C              mov [si],di
00004C03  5F                pop di
00004C04  5E                pop si
00004C05  58                pop ax
00004C06  C3                ret
00004C07  8B1E4206          mov bx,[0x642]
00004C0B  8B1F              mov bx,[bx]
00004C0D  F6C301            test bl,0x1
00004C10  7405              jz 0x4c17
00004C12  83FBFF            cmp bx,0xffffffffffffffff
00004C15  7503              jnz 0x4c1a
00004C17  BB0100            mov bx,0x1
00004C1A  4B                dec bx
00004C1B  C3                ret
00004C1C  53                push bx
00004C1D  8B1C              mov bx,[si]
00004C1F  871D              xchg bx,[di]
00004C21  891C              mov [si],bx
00004C23  8B5C02            mov bx,[si+0x2]
00004C26  875D02            xchg bx,[di+0x2]
00004C29  895C02            mov [si+0x2],bx
00004C2C  8B5C02            mov bx,[si+0x2]
00004C2F  3B1E4006          cmp bx,[0x640]
00004C33  7209              jc 0x4c3e
00004C35  3B1E4406          cmp bx,[0x644]
00004C39  7303              jnc 0x4c3e
00004C3B  8977FE            mov [bx-0x2],si
00004C3E  8B5D02            mov bx,[di+0x2]
00004C41  3B1E4006          cmp bx,[0x640]
00004C45  7209              jc 0x4c50
00004C47  3B1E4406          cmp bx,[0x644]
00004C4B  7303              jnc 0x4c50
00004C4D  897FFE            mov [bx-0x2],di
00004C50  5B                pop bx
00004C51  C3                ret
00004C52  89364406          mov [0x644],si
00004C56  C704FFFF          mov word [si],0xffff
00004C5A  A34006            mov [0x640],ax
00004C5D  8BF8              mov di,ax
00004C5F  893E4206          mov [0x642],di
00004C63  2BF7              sub si,di
00004C65  4E                dec si
00004C66  8935              mov [di],si
00004C68  C7062E0D0000      mov word [0xd2e],0x0
00004C6E  B91400            mov cx,0x14
00004C71  BF8009            mov di,0x980
00004C74  893E4606          mov [0x646],di
00004C78  BBFFFF            mov bx,0xffff
00004C7B  B88609            mov ax,0x986
00004C7E  06                push es
00004C7F  1E                push ds
00004C80  07                pop es
00004C81  AB                stosw
00004C82  93                xchg ax,bx
00004C83  AB                stosw
00004C84  AB                stosw
00004C85  93                xchg ax,bx
00004C86  050600            add ax,0x6
00004C89  E2F6              loop 0x4c81
00004C8B  07                pop es
00004C8C  C745FA0000        mov word [di-0x6],0x0
00004C91  C3                ret
00004C92  33C0              xor ax,ax
00004C94  268B1E680A        mov bx,[es:0xa68]
00004C99  3BD8              cmp bx,ax
00004C9B  7404              jz 0x4ca1
00004C9D  4B                dec bx
00004C9E  E80600            call 0x4ca7
00004CA1  B83000            mov ax,0x30
00004CA4  BBFFFF            mov bx,0xffff
00004CA7  56                push si
00004CA8  268B364006        mov si,[es:0x640]
00004CAD  8BCB              mov cx,bx
00004CAF  8B1C              mov bx,[si]
00004CB1  F6C301            test bl,0x1
00004CB4  7407              jz 0x4cbd
00004CB6  43                inc bx
00004CB7  742A              jz 0x4ce3
00004CB9  03F3              add si,bx
00004CBB  EBF2              jmp 0x4caf
00004CBD  3BD8              cmp bx,ax
00004CBF  7218              jc 0x4cd9
00004CC1  3BD9              cmp bx,cx
00004CC3  7714              ja 0x4cd9
00004CC5  263B1E4C06        cmp bx,[es:0x64c]
00004CCA  730D              jnc 0x4cd9
00004CCC  8B1F              mov bx,[bx]
00004CCE  43                inc bx
00004CCF  80CB01            or bl,0x1
00004CD2  891C              mov [si],bx
00004CD4  F9                stc
00004CD5  13F3              adc si,bx
00004CD7  EBD6              jmp 0x4caf
00004CD9  0337              add si,[bx]
00004CDB  83C603            add si,0x3
00004CDE  83E6FE            and si,0xfffffffffffffffe
00004CE1  EBCC              jmp 0x4caf
00004CE3  5E                pop si
00004CE4  C3                ret
00004CE5  00E8              add al,ch
00004CE7  1B00              sbb ax,[bx+si]
00004CE9  E8B800            call 0x4da4
00004CEC  7315              jnc 0x4d03
00004CEE  E89EFC            call 0x498f
00004CF1  E81000            call 0x4d04
00004CF4  E8AD00            call 0x4da4
00004CF7  730A              jnc 0x4d03
00004CF9  FF16F809          call word near [0x9f8]
00004CFD  E80400            call 0x4d04
00004D00  E8A100            call 0x4da4
00004D03  C3                ret
00004D04  50                push ax
00004D05  56                push si
00004D06  57                push di
00004D07  E8ABFE            call 0x4bb5
00004D0A  A14406            mov ax,[0x644]
00004D0D  2BC6              sub ax,si
00004D0F  7446              jz 0x4d57
00004D11  8B3E4C06          mov di,[0x64c]
00004D15  3B3E4806          cmp di,[0x648]
00004D19  740D              jz 0x4d28
00004D1B  037D01            add di,[di+0x1]
00004D1E  803D01            cmp byte [di],0x1
00004D21  7505              jnz 0x4d28
00004D23  0345FD            add ax,[di-0x3]
00004D26  EB0C              jmp 0x4d34
00004D28  3D0800            cmp ax,0x8
00004D2B  722A              jc 0x4d57
00004D2D  8BFE              mov di,si
00004D2F  83C707            add di,0x7
00004D32  03F8              add di,ax
00004D34  C704FFFF          mov word [si],0xffff
00004D38  89364406          mov [0x644],si
00004D3C  89364206          mov [0x642],si
00004D40  83C607            add si,0x7
00004D43  C60404            mov byte [si],0x4
00004D46  89364C06          mov [0x64c],si
00004D4A  C60501            mov byte [di],0x1
00004D4D  8945FD            mov [di-0x3],ax
00004D50  894401            mov [si+0x1],ax
00004D53  893E4A06          mov [0x64a],di
00004D57  5F                pop di
00004D58  5E                pop si
00004D59  58                pop ax
00004D5A  C3                ret
00004D5B  8B364806          mov si,[0x648]
00004D5F  803C04            cmp byte [si],0x4
00004D62  743A              jz 0x4d9e
00004D64  803C01            cmp byte [si],0x1
00004D67  7405              jz 0x4d6e
00004D69  2B74FD            sub si,[si-0x3]
00004D6C  EBF1              jmp 0x4d5f
00004D6E  8B44FD            mov ax,[si-0x3]
00004D71  8BFE              mov di,si
00004D73  2BF8              sub di,ax
00004D75  803D04            cmp byte [di],0x4
00004D78  740A              jz 0x4d84
00004D7A  803D01            cmp byte [di],0x1
00004D7D  7505              jnz 0x4d84
00004D7F  0345FD            add ax,[di-0x3]
00004D82  EBED              jmp 0x4d71
00004D84  8944FD            mov [si-0x3],ax
00004D87  2BF0              sub si,ax
00004D89  894401            mov [si+0x1],ax
00004D8C  03F0              add si,ax
00004D8E  3BC3              cmp ax,bx
00004D90  7328              jnc 0x4dba
00004D92  803D04            cmp byte [di],0x4
00004D95  7407              jz 0x4d9e
00004D97  2B7DFD            sub di,[di-0x3]
00004D9A  8BF7              mov si,di
00004D9C  EBC1              jmp 0x4d5f
00004D9E  89364A06          mov [0x64a],si
00004DA2  F9                stc
00004DA3  C3                ret
00004DA4  8B364A06          mov si,[0x64a]
00004DA8  803C01            cmp byte [si],0x1
00004DAB  7507              jnz 0x4db4
00004DAD  8B44FD            mov ax,[si-0x3]
00004DB0  3BC3              cmp ax,bx
00004DB2  7306              jnc 0x4dba
00004DB4  F9                stc
00004DB5  C3                ret
00004DB6  03D8              add bx,ax
00004DB8  EB1A              jmp 0x4dd4
00004DBA  7418              jz 0x4dd4
00004DBC  2BC3              sub ax,bx
00004DBE  3D0600            cmp ax,0x6
00004DC1  72F3              jc 0x4db6
00004DC3  2BF3              sub si,bx
00004DC5  C60401            mov byte [si],0x1
00004DC8  8944FD            mov [si-0x3],ax
00004DCB  2BF0              sub si,ax
00004DCD  894401            mov [si+0x1],ax
00004DD0  03F0              add si,ax
00004DD2  03F3              add si,bx
00004DD4  51                push cx
00004DD5  57                push di
00004DD6  06                push es
00004DD7  1E                push ds
00004DD8  07                pop es
00004DD9  8BFE              mov di,si
00004DDB  2BFB              sub di,bx
00004DDD  893E4A06          mov [0x64a],di
00004DE1  47                inc di
00004DE2  33C0              xor ax,ax
00004DE4  8BCB              mov cx,bx
00004DE6  D1E9              shr cx,1
00004DE8  F3AB              rep stosw
00004DEA  07                pop es
00004DEB  5F                pop di
00004DEC  59                pop cx
00004DED  8814              mov [si],dl
00004DEF  895CFD            mov [si-0x3],bx
00004DF2  2BF3              sub si,bx
00004DF4  895C01            mov [si+0x1],bx
00004DF7  03F3              add si,bx
00004DF9  803C08            cmp byte [si],0x8
00004DFC  7505              jnz 0x4e03
00004DFE  884CFF            mov [si-0x1],cl
00004E01  EB03              jmp 0x4e06
00004E03  894CFB            mov [si-0x5],cx
00004E06  2BF3              sub si,bx
00004E08  83C603            add si,0x3
00004E0B  C3                ret
00004E0C  E86900            call 0x4e78
00004E0F  06                push es
00004E10  1E                push ds
00004E11  07                pop es
00004E12  50                push ax
00004E13  53                push bx
00004E14  51                push cx
00004E15  56                push si
00004E16  57                push di
00004E17  8B364806          mov si,[0x648]
00004E1B  803C04            cmp byte [si],0x4
00004E1E  7451              jz 0x4e71
00004E20  E89E01            call 0x4fc1
00004E23  803C01            cmp byte [si],0x1
00004E26  7405              jz 0x4e2d
00004E28  2B74FD            sub si,[si-0x3]
00004E2B  EBEE              jmp 0x4e1b
00004E2D  8BFE              mov di,si
00004E2F  2B74FD            sub si,[si-0x3]
00004E32  803C04            cmp byte [si],0x4
00004E35  7427              jz 0x4e5e
00004E37  E88701            call 0x4fc1
00004E3A  803C01            cmp byte [si],0x1
00004E3D  74F0              jz 0x4e2f
00004E3F  8BC7              mov ax,di
00004E41  2BC6              sub ax,si
00004E43  E84600            call 0x4e8c
00004E46  8B4CFD            mov cx,[si-0x3]
00004E49  D1E9              shr cx,1
00004E4B  4E                dec si
00004E4C  4F                dec di
00004E4D  FD                std
00004E4E  F3A5              rep movsw
00004E50  FC                cld
00004E51  46                inc si
00004E52  47                inc di
00004E53  C60501            mov byte [di],0x1
00004E56  8945FD            mov [di-0x3],ax
00004E59  894401            mov [si+0x1],ax
00004E5C  EBD4              jmp 0x4e32
00004E5E  8BC7              mov ax,di
00004E60  2BC6              sub ax,si
00004E62  7409              jz 0x4e6d
00004E64  C60501            mov byte [di],0x1
00004E67  8945FD            mov [di-0x3],ax
00004E6A  894401            mov [si+0x1],ax
00004E6D  893E4A06          mov [0x64a],di
00004E71  5F                pop di
00004E72  5E                pop si
00004E73  59                pop cx
00004E74  5B                pop bx
00004E75  58                pop ax
00004E76  07                pop es
00004E77  C3                ret
00004E78  06                push es
00004E79  1E                push ds
00004E7A  07                pop es
00004E7B  50                push ax
00004E7C  56                push si
00004E7D  E83B01            call 0x4fbb
00004E80  33C0              xor ax,ax
00004E82  E80700            call 0x4e8c
00004E85  C60401            mov byte [si],0x1
00004E88  5E                pop si
00004E89  58                pop ax
00004E8A  07                pop es
00004E8B  C3                ret
00004E8C  53                push bx
00004E8D  51                push cx
00004E8E  803C08            cmp byte [si],0x8
00004E91  753C              jnz 0x4ecf
00004E93  8D5C03            lea bx,[si+0x3]
00004E96  2B5CFD            sub bx,[si-0x3]
00004E99  263B1E4D0D        cmp bx,[es:0xd4d]
00004E9E  7505              jnz 0x4ea5
00004EA0  2601064D0D        add [es:0xd4d],ax
00004EA5  837CF700          cmp word [si-0x9],0x0
00004EA9  7422              jz 0x4ecd
00004EAB  8B5CF9            mov bx,[si-0x7]
00004EAE  0147FE            add [bx-0x2],ax
00004EB1  0BC0              or ax,ax
00004EB3  7508              jnz 0x4ebd
00004EB5  C60401            mov byte [si],0x1
00004EB8  E85200            call 0x4f0d
00004EBB  EB4D              jmp 0x4f0a
00004EBD  8B4CF7            mov cx,[si-0x9]
00004EC0  D1E9              shr cx,1
00004EC2  56                push si
00004EC3  8B37              mov si,[bx]
00004EC5  014402            add [si+0x2],ax
00004EC8  43                inc bx
00004EC9  43                inc bx
00004ECA  E2F7              loop 0x4ec3
00004ECC  5E                pop si
00004ECD  EB3B              jmp 0x4f0a
00004ECF  803C02            cmp byte [si],0x2
00004ED2  7529              jnz 0x4efd
00004ED4  8B5CFB            mov bx,[si-0x5]
00004ED7  0BC0              or ax,ax
00004ED9  7509              jnz 0x4ee4
00004EDB  894702            mov [bx+0x2],ax
00004EDE  8944FB            mov [si-0x5],ax
00004EE1  C60401            mov byte [si],0x1
00004EE4  8B4CFD            mov cx,[si-0x3]
00004EE7  8BDE              mov bx,si
00004EE9  2BD9              sub bx,cx
00004EEB  83C303            add bx,0x3
00004EEE  83E908            sub cx,0x8
00004EF1  D1E9              shr cx,1
00004EF3  D1E9              shr cx,1
00004EF5  E86EFC            call 0x4b66
00004EF8  83C304            add bx,0x4
00004EFB  E2F8              loop 0x4ef5
00004EFD  8B5CFB            mov bx,[si-0x5]
00004F00  0107              add [bx],ax
00004F02  803C02            cmp byte [si],0x2
00004F05  7503              jnz 0x4f0a
00004F07  01470A            add [bx+0xa],ax
00004F0A  59                pop cx
00004F0B  5B                pop bx
00004F0C  C3                ret
00004F0D  50                push ax
00004F0E  53                push bx
00004F0F  51                push cx
00004F10  57                push di
00004F11  33C0              xor ax,ax
00004F13  8BD8              mov bx,ax
00004F15  8BC8              mov cx,ax
00004F17  874CF7            xchg cx,[si-0x9]
00004F1A  875CF9            xchg bx,[si-0x7]
00004F1D  41                inc cx
00004F1E  894FFE            mov [bx-0x2],cx
00004F21  D1E9              shr cx,1
00004F23  8B3F              mov di,[bx]
00004F25  8905              mov [di],ax
00004F27  894502            mov [di+0x2],ax
00004F2A  83C302            add bx,0x2
00004F2D  E2F4              loop 0x4f23
00004F2F  5F                pop di
00004F30  59                pop cx
00004F31  5B                pop bx
00004F32  58                pop ax
00004F33  C3                ret
00004F34  50                push ax
00004F35  53                push bx
00004F36  51                push cx
00004F37  52                push dx
00004F38  57                push di
00004F39  92                xchg ax,dx
00004F3A  93                xchg ax,bx
00004F3B  33DB              xor bx,bx
00004F3D  8BF3              mov si,bx
00004F3F  E84900            call 0x4f8b
00004F42  7444              jz 0x4f88
00004F44  8BDE              mov bx,si
00004F46  E87200            call 0x4fbb
00004F49  8B4CF7            mov cx,[si-0x9]
00004F4C  E3EF              jcxz 0x4f3d
00004F4E  D1E9              shr cx,1
00004F50  8B7CF9            mov di,[si-0x7]
00004F53  06                push es
00004F54  1E                push ds
00004F55  07                pop es
00004F56  F2AF              repne scasw
00004F58  07                pop es
00004F59  75E2              jnz 0x4f3d
00004F5B  0155FE            add [di-0x2],dx
00004F5E  0BD2              or dx,dx
00004F60  7520              jnz 0x4f82
00004F62  836CF702          sub word [si-0x9],0x2
00004F66  7410              jz 0x4f78
00004F68  8BF7              mov si,di
00004F6A  4F                dec di
00004F6B  4F                dec di
00004F6C  06                push es
00004F6D  1E                push ds
00004F6E  07                pop es
00004F6F  F3A5              rep movsw
00004F71  07                pop es
00004F72  C7050100          mov word [di],0x1
00004F76  EB0A              jmp 0x4f82
00004F78  33FF              xor di,di
00004F7A  877CF9            xchg di,[si-0x7]
00004F7D  C745FE0300        mov word [di-0x2],0x3
00004F82  5F                pop di
00004F83  5A                pop dx
00004F84  59                pop cx
00004F85  5B                pop bx
00004F86  58                pop ax
00004F87  C3                ret
00004F88  E9E609            jmp 0x5971
00004F8B  06                push es
00004F8C  EB03              jmp 0x4f91
00004F8E  06                push es
00004F8F  1E                push ds
00004F90  07                pop es
00004F91  0BF6              or si,si
00004F93  7508              jnz 0x4f9d
00004F95  268B364806        mov si,[es:0x648]
00004F9A  83C603            add si,0x3
00004F9D  83EE03            sub si,0x3
00004FA0  803C04            cmp byte [si],0x4
00004FA3  7412              jz 0x4fb7
00004FA5  803C08            cmp byte [si],0x8
00004FA8  7405              jz 0x4faf
00004FAA  2B74FD            sub si,[si-0x3]
00004FAD  EBF1              jmp 0x4fa0
00004FAF  2B74FD            sub si,[si-0x3]
00004FB2  83C603            add si,0x3
00004FB5  07                pop es
00004FB6  C3                ret
00004FB7  33F6              xor si,si
00004FB9  07                pop es
00004FBA  C3                ret
00004FBB  83EE02            sub si,0x2
00004FBE  0334              add si,[si]
00004FC0  4E                dec si
00004FC1  50                push ax
00004FC2  57                push di
00004FC3  8B44FD            mov ax,[si-0x3]
00004FC6  8BFE              mov di,si
00004FC8  2BF8              sub di,ax
00004FCA  394501            cmp [di+0x1],ax
00004FCD  7507              jnz 0x4fd6
00004FCF  0BC0              or ax,ax
00004FD1  7403              jz 0x4fd6
00004FD3  5F                pop di
00004FD4  58                pop ax
00004FD5  C3                ret
00004FD6  06                push es
00004FD7  1F                pop ds
00004FD8  E99609            jmp 0x5971
00004FDB  8B3E4A06          mov di,[0x64a]
00004FDF  803D01            cmp byte [di],0x1
00004FE2  750B              jnz 0x4fef
00004FE4  8BC7              mov ax,di
00004FE6  2B45FD            sub ax,[di-0x3]
00004FE9  3B064C06          cmp ax,[0x64c]
00004FED  741A              jz 0x5009
00004FEF  8B3E4C06          mov di,[0x64c]
00004FF3  3B3E4806          cmp di,[0x648]
00004FF7  740C              jz 0x5005
00004FF9  8BC7              mov ax,di
00004FFB  034501            add ax,[di+0x1]
00004FFE  97                xchg ax,di
00004FFF  803D01            cmp byte [di],0x1
00005002  7401              jz 0x5005
00005004  97                xchg ax,di
00005005  893E4A06          mov [0x64a],di
00005009  C3                ret
0000500A  50                push ax
0000500B  53                push bx
0000500C  51                push cx
0000500D  56                push si
0000500E  57                push di
0000500F  8BF1              mov si,cx
00005011  46                inc si
00005012  89364806          mov [0x648],si
00005016  89364A06          mov [0x64a],si
0000501A  89364C06          mov [0x64c],si
0000501E  C60404            mov byte [si],0x4
00005021  C7064E06661D      mov word [0x64e],0x1d66
00005027  83EE07            sub si,0x7
0000502A  803EFA0900        cmp byte [0x9fa],0x0
0000502F  7405              jz 0x5036
00005031  E85EFC            call 0x4c92
00005034  EB03              jmp 0x5039
00005036  E819FC            call 0x4c52
00005039  5F                pop di
0000503A  5E                pop si
0000503B  59                pop cx
0000503C  5B                pop bx
0000503D  58                pop ax
0000503E  C3                ret
0000503F  00E8              add al,ch
00005041  CC                int3
00005042  FD                std
00005043  E80600            call 0x504c
00005046  7201              jc 0x5049
00005048  C3                ret
00005049  E96709            jmp 0x59b3
0000504C  50                push ax
0000504D  53                push bx
0000504E  06                push es
0000504F  83C309            add bx,0x9
00005052  83E3FE            and bx,0xfffffffffffffffe
00005055  80FA08            cmp dl,0x8
00005058  7503              jnz 0x505d
0000505A  83C304            add bx,0x4
0000505D  E844FD            call 0x4da4
00005060  730F              jnc 0x5071
00005062  51                push cx
00005063  52                push dx
00005064  57                push di
00005065  E8F3FC            call 0x4d5b
00005068  7304              jnc 0x506e
0000506A  FF164E06          call word near [0x64e]
0000506E  5F                pop di
0000506F  5A                pop dx
00005070  59                pop cx
00005071  07                pop es
00005072  5B                pop bx
00005073  58                pop ax
00005074  C3                ret
00005075  E817F9            call 0x498f
00005078  E894FD            call 0x4e0f
0000507B  E855FB            call 0x4bd3
0000507E  C3                ret
0000507F  56                push si
00005080  E31A              jcxz 0x509c
00005082  41                inc cx
00005083  3BC8              cmp cx,ax
00005085  7244              jc 0x50cb
00005087  3B0E4806          cmp cx,[0x648]
0000508B  760F              jna 0x509c
0000508D  8BF1              mov si,cx
0000508F  E83C00            call 0x50ce
00005092  8BCE              mov cx,si
00005094  8BF0              mov si,ax
00005096  E85DF9            call 0x49f6
00005099  96                xchg ax,si
0000509A  EB2E              jmp 0x50ca
0000509C  3B064006          cmp ax,[0x640]
000050A0  7606              jna 0x50a8
000050A2  E86AFD            call 0x4e0f
000050A5  E82BFB            call 0x4bd3
000050A8  8BF0              mov si,ax
000050AA  E849F9            call 0x49f6
000050AD  96                xchg ax,si
000050AE  721B              jc 0x50cb
000050B0  3B0E4806          cmp cx,[0x648]
000050B4  7415              jz 0x50cb
000050B6  33F6              xor si,si
000050B8  E81300            call 0x50ce
000050BB  87CE              xchg cx,si
000050BD  0BF6              or si,si
000050BF  740A              jz 0x50cb
000050C1  3BF1              cmp si,cx
000050C3  7606              jna 0x50cb
000050C5  E80600            call 0x50ce
000050C8  8BCE              mov cx,si
000050CA  F8                clc
000050CB  49                dec cx
000050CC  5E                pop si
000050CD  C3                ret
000050CE  50                push ax
000050CF  0BF6              or si,si
000050D1  7508              jnz 0x50db
000050D3  E82EFC            call 0x4d04
000050D6  E82500            call 0x50fe
000050D9  EB1D              jmp 0x50f8
000050DB  8BC6              mov ax,si
000050DD  2B064806          sub ax,[0x648]
000050E1  3D0600            cmp ax,0x6
000050E4  720F              jc 0x50f5
000050E6  C60401            mov byte [si],0x1
000050E9  8944FD            mov [si-0x3],ax
000050EC  89364806          mov [0x648],si
000050F0  2BF0              sub si,ax
000050F2  894401            mov [si+0x1],ax
000050F5  E817FD            call 0x4e0f
000050F8  8B364806          mov si,[0x648]
000050FC  58                pop ax
000050FD  C3                ret
000050FE  51                push cx
000050FF  57                push di
00005100  52                push dx
00005101  8B364C06          mov si,[0x64c]
00005105  89364A06          mov [0x64a],si
00005109  3B364806          cmp si,[0x648]
0000510D  7413              jz 0x5122
0000510F  037401            add si,[si+0x1]
00005112  803C01            cmp byte [si],0x1
00005115  75F2              jnz 0x5109
00005117  8B164806          mov dx,[0x648]
0000511B  E80C00            call 0x512a
0000511E  893E4806          mov [0x648],di
00005122  8B364806          mov si,[0x648]
00005126  5A                pop dx
00005127  5F                pop di
00005128  59                pop cx
00005129  C3                ret
0000512A  06                push es
0000512B  1E                push ds
0000512C  07                pop es
0000512D  8BFE              mov di,si
0000512F  2B7CFD            sub di,[si-0x3]
00005132  47                inc di
00005133  3BF2              cmp si,dx
00005135  742F              jz 0x5166
00005137  037401            add si,[si+0x1]
0000513A  E884FE            call 0x4fc1
0000513D  803C01            cmp byte [si],0x1
00005140  74F1              jz 0x5133
00005142  8D4C01            lea cx,[si+0x1]
00005145  2B4CFD            sub cx,[si-0x3]
00005148  8BC7              mov ax,di
0000514A  2BC1              sub ax,cx
0000514C  E83DFD            call 0x4e8c
0000514F  8BF1              mov si,cx
00005151  8B0C              mov cx,[si]
00005153  D1E9              shr cx,1
00005155  F3A5              rep movsw
00005157  F7D8              neg ax
00005159  4E                dec si
0000515A  8944FD            mov [si-0x3],ax
0000515D  C60401            mov byte [si],0x1
00005160  8905              mov [di],ax
00005162  F7D8              neg ax
00005164  EBCD              jmp 0x5133
00005166  4F                dec di
00005167  8BCA              mov cx,dx
00005169  2BCF              sub cx,di
0000516B  8BF2              mov si,dx
0000516D  894CFD            mov [si-0x3],cx
00005170  C60401            mov byte [si],0x1
00005173  894D01            mov [di+0x1],cx
00005176  07                pop es
00005177  C3                ret
00005178  52                push dx
00005179  53                push bx
0000517A  56                push si
0000517B  268B364806        mov si,[es:0x648]
00005180  263B364C06        cmp si,[es:0x64c]
00005185  760C              jna 0x5193
00005187  8A14              mov dl,[si]
00005189  8B5CFB            mov bx,[si-0x5]
0000518C  FFD1              call cx
0000518E  2B74FD            sub si,[si-0x3]
00005191  EBED              jmp 0x5180
00005193  5E                pop si
00005194  5B                pop bx
00005195  5A                pop dx
00005196  C3                ret
00005197  57                push di
00005198  E840FE            call 0x4fdb
0000519B  3B364C06          cmp si,[0x64c]
0000519F  7206              jc 0x51a7
000051A1  3BFE              cmp di,si
000051A3  7218              jc 0x51bd
000051A5  740D              jz 0x51b4
000051A7  8BC7              mov ax,di
000051A9  2BC6              sub ax,si
000051AB  C60501            mov byte [di],0x1
000051AE  8945FD            mov [di-0x3],ax
000051B1  894401            mov [si+0x1],ax
000051B4  C60404            mov byte [si],0x4
000051B7  89364C06          mov [0x64c],si
000051BB  0BC0              or ax,ax
000051BD  5F                pop di
000051BE  C3                ret
000051BF  50                push ax
000051C0  53                push bx
000051C1  51                push cx
000051C2  52                push dx
000051C3  57                push di
000051C4  E8F4FD            call 0x4fbb
000051C7  83C6F7            add si,0xfffffffffffffff7
000051CA  8BC3              mov ax,bx
000051CC  8BDE              mov bx,si
000051CE  33C9              xor cx,cx
000051D0  8B17              mov dx,[bx]
000051D2  83C202            add dx,0x2
000051D5  E859F9            call 0x4b31
000051D8  8BFA              mov di,dx
000051DA  037F02            add di,[bx+0x2]
000051DD  8945FE            mov [di-0x2],ax
000051E0  87DE              xchg bx,si
000051E2  E875F9            call 0x4b5a
000051E5  830702            add word [bx],0x2
000051E8  8B7C02            mov di,[si+0x2]
000051EB  897F02            mov [bx+0x2],di
000051EE  895DFE            mov [di-0x2],bx
000051F1  8BDE              mov bx,si
000051F3  E82BF9            call 0x4b21
000051F6  5F                pop di
000051F7  5A                pop dx
000051F8  59                pop cx
000051F9  5B                pop bx
000051FA  58                pop ax
000051FB  C3                ret
000051FC  56                push si
000051FD  E8BBFD            call 0x4fbb
00005200  8A5CFF            mov bl,[si-0x1]
00005203  5E                pop si
00005204  C3                ret
00005205  8B364806          mov si,[0x648]
00005209  803C04            cmp byte [si],0x4
0000520C  7416              jz 0x5224
0000520E  803C08            cmp byte [si],0x8
00005211  7505              jnz 0x5218
00005213  385CFF            cmp [si-0x1],bl
00005216  7405              jz 0x521d
00005218  2B74FD            sub si,[si-0x3]
0000521B  EBEC              jmp 0x5209
0000521D  2B74FD            sub si,[si-0x3]
00005220  83C603            add si,0x3
00005223  C3                ret
00005224  33F6              xor si,si
00005226  C3                ret
00005227  50                push ax
00005228  53                push bx
00005229  51                push cx
0000522A  52                push dx
0000522B  E80800            call 0x5236
0000522E  E861FA            call 0x4c92
00005231  5A                pop dx
00005232  59                pop cx
00005233  5B                pop bx
00005234  58                pop ax
00005235  C3                ret
00005236  33C0              xor ax,ax
00005238  2638061A0D        cmp [es:0xd1a],al
0000523D  740F              jz 0x524e
0000523F  268B1E680A        mov bx,[es:0xa68]
00005244  4B                dec bx
00005245  33C9              xor cx,cx
00005247  E80900            call 0x5253
0000524A  26A16A0A          mov ax,[es:0xa6a]
0000524E  BBFFFF            mov bx,0xffff
00005251  33C9              xor cx,cx
00005253  55                push bp
00005254  8BEC              mov bp,sp
00005256  57                push di
00005257  51                push cx
00005258  53                push bx
00005259  50                push ax
0000525A  8BFC              mov di,sp
0000525C  B9E822            mov cx,0x22e8
0000525F  E816FF            call 0x5178
00005262  58                pop ax
00005263  5B                pop bx
00005264  59                pop cx
00005265  5F                pop di
00005266  5D                pop bp
00005267  C3                ret
00005268  80FA08            cmp dl,0x8
0000526B  750B              jnz 0x5278
0000526D  837CF700          cmp word [si-0x9],0x0
00005271  742B              jz 0x529e
00005273  E82900            call 0x529f
00005276  EB26              jmp 0x529e
00005278  8B5CFB            mov bx,[si-0x5]
0000527B  80FA02            cmp dl,0x2
0000527E  751E              jnz 0x529e
00005280  8A4708            mov al,[bx+0x8]
00005283  98                cbw
00005284  D1E0              shl ax,1
00005286  83C30C            add bx,0xc
00005289  03D8              add bx,ax
0000528B  363B1D            cmp bx,[ss:di]
0000528E  720E              jc 0x529e
00005290  363B5D02          cmp bx,[ss:di+0x2]
00005294  7708              ja 0x529e
00005296  33C0              xor ax,ax
00005298  E8F1FB            call 0x4e8c
0000529B  C60401            mov byte [si],0x1
0000529E  C3                ret
0000529F  56                push si
000052A0  8B5CF7            mov bx,[si-0x9]
000052A3  8B74F9            mov si,[si-0x7]
000052A6  8D58FE            lea bx,[bx+si-0x2]
000052A9  AD                lodsw
000052AA  263B064C06        cmp ax,[es:0x64c]
000052AF  7717              ja 0x52c8
000052B1  363B05            cmp ax,[ss:di]
000052B4  7212              jc 0x52c8
000052B6  363B4502          cmp ax,[ss:di+0x2]
000052BA  770C              ja 0x52c8
000052BC  4E                dec si
000052BD  4E                dec si
000052BE  8B07              mov ax,[bx]
000052C0  8904              mov [si],ax
000052C2  C7070100          mov word [bx],0x1
000052C6  4B                dec bx
000052C7  4B                dec bx
000052C8  3BF3              cmp si,bx
000052CA  76DD              jna 0x52a9
000052CC  96                xchg ax,si
000052CD  5E                pop si
000052CE  2B44F9            sub ax,[si-0x7]
000052D1  8944F7            mov [si-0x9],ax
000052D4  7507              jnz 0x52dd
000052D6  C7070100          mov word [bx],0x1
000052DA  8944F9            mov [si-0x7],ax
000052DD  C3                ret
000052DE  7929              jns 0x5309
000052E0  7929              jns 0x530b
000052E2  7929              jns 0x530d
000052E4  9A23822679        call word 0x7926:word 0x8223
000052E9  29A22379          sub [bp+si+0x7923],sp
000052ED  297929            sub [bx+di+0x29],di
000052F0  EF                out dx,ax
000052F1  236E26            and bp,[bp+0x26]
000052F4  7A26              jpe 0x531c
000052F6  82                db 0x82
000052F7  267929            es jns 0x5323
000052FA  7929              jns 0x5325
000052FC  7929              jns 0x5327
000052FE  7929              jns 0x5329
00005300  7929              jns 0x532b
00005302  9A23792979        call word 0x7929:word 0x7923
00005307  299E23C7          sub [bp-0x38dd],bx
0000530B  23A62379          and sp,[bp+0x7923]
0000530F  297929            sub [bx+di+0x29],di
00005312  7929              jns 0x533d
00005314  7929              jns 0x533f
00005316  7929              jns 0x5341
00005318  7929              jns 0x5343
0000531A  FF26FC09          jmp word near [0x9fc]
0000531E  FF26FE09          jmp word near [0x9fe]
00005322  FF26000A          jmp word near [0xa00]
00005326  32C0              xor al,al
00005328  86065006          xchg al,[0x650]
0000532C  0AC0              or al,al
0000532E  7515              jnz 0x5345
00005330  52                push dx
00005331  32D2              xor dl,dl
00005333  E8C908            call 0x5bff
00005336  E8521A            call 0x6d8b
00005339  5A                pop dx
0000533A  74F4              jz 0x5330
0000533C  7307              jnc 0x5345
0000533E  E80600            call 0x5347
00005341  8AC4              mov al,ah
00005343  32E4              xor ah,ah
00005345  F8                clc
00005346  C3                ret
00005347  A25006            mov [0x650],al
0000534A  C3                ret
0000534B  C606510600        mov byte [0x651],0x0
00005350  3CFF              cmp al,0xff
00005352  7525              jnz 0x5379
00005354  32E4              xor ah,ah
00005356  EB21              jmp 0x5379
00005358  A25106            mov [0x651],al
0000535B  58                pop ax
0000535C  C3                ret
0000535D  B4FF              mov ah,0xff
0000535F  EB1C              jmp 0x537d
00005361  8A265106          mov ah,[0x651]
00005365  0AE4              or ah,ah
00005367  75E2              jnz 0x534b
00005369  3CFF              cmp al,0xff
0000536B  74EB              jz 0x5358
0000536D  EB0A              jmp 0x5379
0000536F  50                push ax
00005370  32E4              xor ah,ah
00005372  F606240B20        test byte [0xb24],0x20
00005377  75E8              jnz 0x5361
00005379  3C0D              cmp al,0xd
0000537B  74E0              jz 0x535d
0000537D  52                push dx
0000537E  B201              mov dl,0x1
00005380  F606240B06        test byte [0xb24],0x6
00005385  7403              jz 0x538a
00005387  E85D01            call 0x54e7
0000538A  88165206          mov [0x652],dl
0000538E  8B166C0A          mov dx,[0xa6c]
00005392  F606520602        test byte [0x652],0x2
00005397  752B              jnz 0x53c4
00005399  0AE4              or ah,ah
0000539B  754E              jnz 0x53eb
0000539D  3C1F              cmp al,0x1f
0000539F  7643              jna 0x53e4
000053A1  F606520601        test byte [0x652],0x1
000053A6  7412              jz 0x53ba
000053A8  50                push ax
000053A9  E81B01            call 0x54c7
000053AC  7603              jna 0x53b1
000053AE  E8CB01            call 0x557c
000053B1  E80216            call 0x69b6
000053B4  FEC6              inc dh
000053B6  E82001            call 0x54d9
000053B9  58                pop ax
000053BA  F606520604        test byte [0x652],0x4
000053BF  750D              jnz 0x53ce
000053C1  5A                pop dx
000053C2  58                pop ax
000053C3  C3                ret
000053C4  53                push bx
000053C5  E86101            call 0x5529
000053C8  E8B700            call 0x5482
000053CB  5B                pop bx
000053CC  EBCB              jmp 0x5399
000053CE  3D0DFF            cmp ax,0xff0d
000053D1  7502              jnz 0x53d5
000053D3  32E4              xor ah,ah
000053D5  FF16020A          call word near [0xa02]
000053D9  EBE6              jmp 0x53c1
000053DB  80FC01            cmp ah,0x1
000053DE  F5                cmc
000053DF  E8BA18            call 0x6c9c
000053E2  EB15              jmp 0x53f9
000053E4  F606240B08        test byte [0xb24],0x8
000053E9  75F0              jnz 0x53db
000053EB  F606240B20        test byte [0xb24],0x20
000053F0  7509              jnz 0x53fb
000053F2  80FC01            cmp ah,0x1
000053F5  F5                cmc
000053F6  E86A19            call 0x6d63
000053F9  74BF              jz 0x53ba
000053FB  80FCFF            cmp ah,0xff
000053FE  7402              jz 0x5402
00005400  EB9F              jmp 0x53a1
00005402  32E4              xor ah,ah
00005404  3C7F              cmp al,0x7f
00005406  7502              jnz 0x540a
00005408  B020              mov al,0x20
0000540A  3CFF              cmp al,0xff
0000540C  74AC              jz 0x53ba
0000540E  3C21              cmp al,0x21
00005410  7333              jnc 0x5445
00005412  F606520601        test byte [0x652],0x1
00005417  742C              jz 0x5445
00005419  50                push ax
0000541A  2C07              sub al,0x7
0000541C  3C06              cmp al,0x6
0000541E  7610              jna 0x5430
00005420  2C07              sub al,0x7
00005422  3C07              cmp al,0x7
00005424  740A              jz 0x5430
00005426  2C06              sub al,0x6
00005428  3C08              cmp al,0x8
0000542A  7C18              jl 0x5444
0000542C  3C0B              cmp al,0xb
0000542E  7F14              jg 0x5444
00005430  53                push bx
00005431  51                push cx
00005432  03C0              add ax,ax
00005434  93                xchg ax,bx
00005435  B8BF24            mov ax,0x24bf
00005438  50                push ax
00005439  F8                clc
0000543A  2EFFA7C824        jmp word near [cs:bx+0x24c8]
0000543F  E8D809            call 0x5e1a
00005442  59                pop cx
00005443  5B                pop bx
00005444  58                pop ax
00005445  E972FF            jmp 0x53ba
00005448  243B              and al,0x3b
0000544A  D6                salc
0000544B  25E325            and ax,0x25e3
0000544E  FC                cld
0000544F  25CF25            and ax,0x25cf
00005452  C6                db 0xc6
00005453  25FC25            and ax,0x25fc
00005456  0426              add al,0x26
00005458  0C26              or al,0x26
0000545A  1D263F            sbb ax,0x3f26
0000545D  2651              es push cx
0000545F  2650              es push ax
00005461  53                push bx
00005462  51                push cx
00005463  52                push dx
00005464  50                push ax
00005465  8BD4              mov dx,sp
00005467  BB0400            mov bx,0x4
0000546A  B90100            mov cx,0x1
0000546D  B440              mov ah,0x40
0000546F  CD21              int byte 0x21
00005471  58                pop ax
00005472  7303              jnc 0x5477
00005474  E9C404            jmp 0x593b
00005477  3C0D              cmp al,0xd
00005479  B00A              mov al,0xa
0000547B  74E7              jz 0x5464
0000547D  5A                pop dx
0000547E  59                pop cx
0000547F  5B                pop bx
00005480  58                pop ax
00005481  C3                ret
00005482  0BDB              or bx,bx
00005484  741D              jz 0x54a3
00005486  8BC3              mov ax,bx
00005488  3D0A00            cmp ax,0xa
0000548B  7417              jz 0x54a4
0000548D  F8                clc
0000548E  E85316            call 0x6ae4
00005491  8BC3              mov ax,bx
00005493  3C09              cmp al,0x9
00005495  7208              jc 0x549f
00005497  7422              jz 0x54bb
00005499  3C0D              cmp al,0xd
0000549B  7413              jz 0x54b0
0000549D  7618              jna 0x54b7
0000549F  FE06040A          inc byte [0xa04]
000054A3  C3                ret
000054A4  B80D00            mov ax,0xd
000054A7  F8                clc
000054A8  E83916            call 0x6ae4
000054AB  B80A00            mov ax,0xa
000054AE  EBDD              jmp 0x548d
000054B0  B80A00            mov ax,0xa
000054B3  F8                clc
000054B4  E82D16            call 0x6ae4
000054B7  33C0              xor ax,ax
000054B9  EB07              jmp 0x54c2
000054BB  A0040A            mov al,[0xa04]
000054BE  0408              add al,0x8
000054C0  24F8              and al,0xf8
000054C2  40                inc ax
000054C3  A2040A            mov [0xa04],al
000054C6  C3                ret
000054C7  833E4D0D00        cmp word [0xd4d],0x0
000054CC  7406              jz 0x54d4
000054CE  3A366F0A          cmp dh,[0xa6f]
000054D2  7304              jnc 0x54d8
000054D4  3A36800A          cmp dh,[0xa80]
000054D8  C3                ret
000054D9  89166C0A          mov [0xa6c],dx
000054DD  E8E7FF            call 0x54c7
000054E0  7602              jna 0x54e4
000054E2  FECE              dec dh
000054E4  E94F09            jmp 0x5e36
000054E7  50                push ax
000054E8  33D2              xor dx,dx
000054EA  A0240B            mov al,[0xb24]
000054ED  A802              test al,0x2
000054EF  7417              jz 0x5508
000054F1  39164D0D          cmp [0xd4d],dx
000054F5  7511              jnz 0x5508
000054F7  A818              test al,0x18
000054F9  7505              jnz 0x5500
000054FB  80CA02            or dl,0x2
000054FE  EB0B              jmp 0x550b
00005500  A801              test al,0x1
00005502  7507              jnz 0x550b
00005504  A810              test al,0x10
00005506  7403              jz 0x550b
00005508  80CA01            or dl,0x1
0000550B  A804              test al,0x4
0000550D  7418              jz 0x5527
0000550F  A818              test al,0x18
00005511  7514              jnz 0x5527
00005513  833E4D0D00        cmp word [0xd4d],0x0
00005518  750A              jnz 0x5524
0000551A  A802              test al,0x2
0000551C  7406              jz 0x5524
0000551E  2421              and al,0x21
00005520  3C20              cmp al,0x20
00005522  7503              jnz 0x5527
00005524  80CA04            or dl,0x4
00005527  58                pop ax
00005528  C3                ret
00005529  50                push ax
0000552A  33DB              xor bx,bx
0000552C  80FC01            cmp ah,0x1
0000552F  F5                cmc
00005530  E83018            call 0x6d63
00005533  740B              jz 0x5540
00005535  8BD8              mov bx,ax
00005537  7307              jnc 0x5540
00005539  80FCFF            cmp ah,0xff
0000553C  7502              jnz 0x5540
0000553E  32FF              xor bh,bh
00005540  58                pop ax
00005541  C3                ret
00005542  E85F15            call 0x6aa4
00005545  CB                retf
00005546  B002              mov al,0x2
00005548  E80514            call 0x6950
0000554B  FF16250B          call word near [0xb25]
0000554F  8A16700A          mov dl,[0xa70]
00005553  B601              mov dh,0x1
00005555  C3                ret
00005556  80FE01            cmp dh,0x1
00005559  74FA              jz 0x5555
0000555B  FECE              dec dh
0000555D  B82000            mov ax,0x20
00005560  E95314            jmp 0x69b6
00005563  B8EE25            mov ax,0x25ee
00005566  50                push ax
00005567  B82000            mov ax,0x20
0000556A  50                push ax
0000556B  E90FFE            jmp 0x537d
0000556E  8B166C0A          mov dx,[0xa6c]
00005572  FECE              dec dh
00005574  F6C607            test dh,0x7
00005577  75EA              jnz 0x5563
00005579  FEC6              inc dh
0000557B  C3                ret
0000557C  E8C015            call 0x6b3f
0000557F  75D4              jnz 0x5555
00005581  E9E209            jmp 0x5f66
00005584  F6162707          not byte [0x727]
00005588  FF26250B          jmp word near [0xb25]
0000558C  FEC6              inc dh
0000558E  E836FF            call 0x54c7
00005591  7609              jna 0x559c
00005593  FECE              dec dh
00005595  E83900            call 0x55d1
00005598  7202              jc 0x559c
0000559A  B601              mov dh,0x1
0000559C  C3                ret
0000559D  FECE              dec dh
0000559F  75FB              jnz 0x559c
000055A1  FEC6              inc dh
000055A3  E81900            call 0x55bf
000055A6  72F4              jc 0x559c
000055A8  8A36800A          mov dh,[0xa80]
000055AC  833E4D0D00        cmp word [0xd4d],0x0
000055B1  74E9              jz 0x559c
000055B3  803E6F0AFF        cmp byte [0xa6f],0xff
000055B8  74E2              jz 0x559c
000055BA  8A366F0A          mov dh,[0xa6f]
000055BE  C3                ret
000055BF  3816710A          cmp [0xa71],dl
000055C3  72D7              jc 0x559c
000055C5  3A16700A          cmp dl,[0xa70]
000055C9  72D1              jc 0x559c
000055CB  F9                stc
000055CC  74CE              jz 0x559c
000055CE  F8                clc
000055CF  4A                dec dx
000055D0  C3                ret
000055D1  3A16700A          cmp dl,[0xa70]
000055D5  72C5              jc 0x559c
000055D7  3816710A          cmp [0xa71],dl
000055DB  72BF              jc 0x559c
000055DD  F9                stc
000055DE  74BC              jz 0x559c
000055E0  F8                clc
000055E1  42                inc dx
000055E2  C3                ret
000055E3  F606240B02        test byte [0xb24],0x2
000055E8  8A26040A          mov ah,[0xa04]
000055EC  7504              jnz 0x55f2
000055EE  8A266D0A          mov ah,[0xa6d]
000055F2  FECC              dec ah
000055F4  C3                ret
000055F5  8A26800A          mov ah,[0xa80]
000055F9  C3                ret
000055FA  8A266F0A          mov ah,[0xa6f]
000055FE  C3                ret
000055FF  E9F702            jmp 0x58f9
00005602  92                xchg ax,dx
00005603  48                dec ax
00005604  3DFE00            cmp ax,0xfe
00005607  77F6              ja 0x55ff
00005609  40                inc ax
0000560A  3CFF              cmp al,0xff
0000560C  7418              jz 0x5626
0000560E  3A06800A          cmp al,[0xa80]
00005612  7412              jz 0x5626
00005614  50                push ax
00005615  8A0E6E0A          mov cl,[0xa6e]
00005619  E8D817            call 0x6df4
0000561C  58                pop ax
0000561D  3A06800A          cmp al,[0xa80]
00005621  7203              jc 0x5626
00005623  A0800A            mov al,[0xa80]
00005626  A26F0A            mov [0xa6f],al
00005629  C3                ret
0000562A  55                push bp
0000562B  8BEC              mov bp,sp
0000562D  8B5E08            mov bx,[bp+0x8]
00005630  8B5606            mov dx,[bp+0x6]
00005633  83FAFF            cmp dx,0xffffffffffffffff
00005636  7506              jnz 0x563e
00005638  8A166E0A          mov dl,[0xa6e]
0000563C  32F6              xor dh,dh
0000563E  0AF6              or dh,dh
00005640  75BD              jnz 0x55ff
00005642  83FBFF            cmp bx,0xffffffffffffffff
00005645  7506              jnz 0x564d
00005647  8A1E800A          mov bl,[0xa80]
0000564B  32FF              xor bh,bh
0000564D  0AFF              or bh,bh
0000564F  75AE              jnz 0x55ff
00005651  3A1E800A          cmp bl,[0xa80]
00005655  7506              jnz 0x565d
00005657  3A166E0A          cmp dl,[0xa6e]
0000565B  740C              jz 0x5669
0000565D  51                push cx
0000565E  50                push ax
0000565F  93                xchg ax,bx
00005660  87CA              xchg cx,dx
00005662  E88F17            call 0x6df4
00005665  7298              jc 0x55ff
00005667  58                pop ax
00005668  59                pop cx
00005669  5D                pop bp
0000566A  CA0400            retf word 0x4
0000566D  00568B            add [bp-0x75],dl
00005670  DD8BCB8B          fisttp qword [bp+di-0x7435]
00005674  1F                pop ds
00005675  3B1E2C0D          cmp bx,[0xd2c]
00005679  75F6              jnz 0x5671
0000567B  8BD9              mov bx,cx
0000567D  33C9              xor cx,cx
0000567F  8B5F04            mov bx,[bx+0x4]
00005682  98                cbw
00005683  96                xchg ax,si
00005684  1E                push ds
00005685  8EDB              mov ds,bx
00005687  AD                lodsw
00005688  1F                pop ds
00005689  5E                pop si
0000568A  C3                ret
0000568B  06                push es
0000568C  8EC2              mov es,dx
0000568E  268B0E2200        mov cx,[es:0x22]
00005693  07                pop es
00005694  8BC4              mov ax,sp
00005696  2BC1              sub ax,cx
00005698  722E              jc 0x56c8
0000569A  2D0A00            sub ax,0xa
0000569D  7229              jc 0x56c8
0000569F  3B06120D          cmp ax,[0xd12]
000056A3  7223              jc 0x56c8
000056A5  8F065606          pop word [0x656]
000056A9  8F065806          pop word [0x658]
000056AD  33C0              xor ax,ax
000056AF  55                push bp
000056B0  8BEC              mov bp,sp
000056B2  FF362C0D          push word [0xd2c]
000056B6  56                push si
000056B7  57                push di
000056B8  51                push cx
000056B9  50                push ax
000056BA  2BE1              sub sp,cx
000056BC  892E2C0D          mov [0xd2c],bp
000056C0  FF062E0D          inc word [0xd2e]
000056C4  FF2E5606          jmp word far [0x656]
000056C8  55                push bp
000056C9  8BEC              mov bp,sp
000056CB  E9B102            jmp 0x597f
000056CE  55                push bp
000056CF  8BEC              mov bp,sp
000056D1  E8B303            call 0x5a87
000056D4  BB0690            mov bx,0x9006
000056D7  E91C03            jmp 0x59f6
000056DA  813E480D0094      cmp word [0xd48],0x9400
000056E0  7330              jnc 0x5712
000056E2  B80880            mov ax,0x8008
000056E5  E87903            call 0x5a61
000056E8  B00A              mov al,0xa
000056EA  E881FF            call 0x566e
000056ED  91                xchg ax,cx
000056EE  E322              jcxz 0x5712
000056F0  B80A80            mov ax,0x800a
000056F3  E86B03            call 0x5a61
000056F6  8B4602            mov ax,[bp+0x2]
000056F9  E84F00            call 0x574b
000056FC  7508              jnz 0x5706
000056FE  B80094            mov ax,0x9400
00005701  E85D03            call 0x5a61
00005704  EB0C              jmp 0x5712
00005706  B9362B            mov cx,0x2b36
00005709  E8B303            call 0x5abf
0000570C  B80980            mov ax,0x8009
0000570F  E84F03            call 0x5a61
00005712  B80B80            mov ax,0x800b
00005715  E84903            call 0x5a61
00005718  33C0              xor ax,ax
0000571A  E851FF            call 0x566e
0000571D  BE0200            mov si,0x2
00005720  B90800            mov cx,0x8
00005723  1E                push ds
00005724  8EDB              mov ds,bx
00005726  AC                lodsb
00005727  1F                pop ds
00005728  E88B03            call 0x5ab6
0000572B  E2F6              loop 0x5723
0000572D  B80C80            mov ax,0x800c
00005730  E82E03            call 0x5a61
00005733  8B5604            mov dx,[bp+0x4]
00005736  E80800            call 0x5741
00005739  B03A              mov al,0x3a
0000573B  E87803            call 0x5ab6
0000573E  8B5602            mov dx,[bp+0x2]
00005741  8AC6              mov al,dh
00005743  E85B03            call 0x5aa1
00005746  8AC2              mov al,dl
00005748  E95603            jmp 0x5aa1
0000574B  56                push si
0000574C  1E                push ds
0000574D  50                push ax
0000574E  B00A              mov al,0xa
00005750  E81BFF            call 0x566e
00005753  8EDB              mov ds,bx
00005755  96                xchg ax,si
00005756  33DB              xor bx,bx
00005758  8BD3              mov dx,bx
0000575A  59                pop cx
0000575B  AD                lodsw
0000575C  0BC0              or ax,ax
0000575E  7416              jz 0x5776
00005760  3BC1              cmp ax,cx
00005762  730F              jnc 0x5773
00005764  3BC3              cmp ax,bx
00005766  720B              jc 0x5773
00005768  93                xchg ax,bx
00005769  AD                lodsw
0000576A  7504              jnz 0x5770
0000576C  3BC2              cmp ax,dx
0000576E  76EB              jna 0x575b
00005770  92                xchg ax,dx
00005771  EBE8              jmp 0x575b
00005773  AD                lodsw
00005774  EBE5              jmp 0x575b
00005776  1F                pop ds
00005777  0BDB              or bx,bx
00005779  92                xchg ax,dx
0000577A  5E                pop si
0000577B  C3                ret
0000577C  06                push es
0000577D  BBFFFF            mov bx,0xffff
00005780  8EC3              mov es,bx
00005782  26A00E00          mov al,[es:0xe]
00005786  A25C06            mov [0x65c],al
00005789  33C0              xor ax,ax
0000578B  8EC0              mov es,ax
0000578D  803E5C06FC        cmp byte [0x65c],0xfc
00005792  7607              jna 0x579b
00005794  803E5C06FE        cmp byte [0x65c],0xfe
00005799  7509              jnz 0x57a4
0000579B  26A09604          mov al,[es:0x496]
0000579F  2410              and al,0x10
000057A1  A25D06            mov [0x65d],al
000057A4  26A11004          mov ax,[es:0x410]
000057A8  A35A06            mov [0x65a],ax
000057AB  E89611            call 0x6944
000057AE  33C0              xor ax,ax
000057B0  07                pop es
000057B1  C3                ret
000057B2  56                push si
000057B3  06                push es
000057B4  8B36140D          mov si,[0xd14]
000057B8  C434              les si,word [si]
000057BA  89365E06          mov [0x65e],si
000057BE  8C066006          mov word [0x660],es
000057C2  E86800            call 0x582d
000057C5  800E270D10        or byte [0xd27],0x10
000057CA  07                pop es
000057CB  5E                pop si
000057CC  CB                retf
000057CD  EA142A5802        jmp word 0x258:word 0x2a14
000057D2  EA0D2A5802        jmp word 0x258:word 0x2a0d
000057D7  56                push si
000057D8  06                push es
000057D9  A1DB0C            mov ax,[0xcdb]
000057DC  86E0              xchg ah,al
000057DE  3D0A02            cmp ax,0x20a
000057E1  72EF              jc 0x57d2
000057E3  8B1E140D          mov bx,[0xd14]
000057E7  8B4702            mov ax,[bx+0x2]
000057EA  A36006            mov [0x660],ax
000057ED  8B37              mov si,[bx]
000057EF  89365E06          mov [0x65e],si
000057F3  50                push ax
000057F4  0BC6              or ax,si
000057F6  58                pop ax
000057F7  7414              jz 0x580d
000057F9  83C304            add bx,0x4
000057FC  8EC0              mov es,ax
000057FE  268B442E          mov ax,[es:si+0x2e]
00005802  0906280D          or [0xd28],ax
00005806  8B37              mov si,[bx]
00005808  8B4702            mov ax,[bx+0x2]
0000580B  EBE6              jmp 0x57f3
0000580D  E85B03            call 0x5b6b
00005810  E869FF            call 0x577c
00005813  0BC0              or ax,ax
00005815  75B6              jnz 0x57cd
00005817  9A72365802        call word 0x258:word 0x3672
0000581C  72AF              jc 0x57cd
0000581E  07                pop es
0000581F  5E                pop si
00005820  C3                ret
00005821  8B0C              mov cx,[si]
00005823  E307              jcxz 0x582c
00005825  AD                lodsw
00005826  51                push cx
00005827  FF14              call word near [si]
00005829  59                pop cx
0000582A  E2F9              loop 0x5825
0000582C  CB                retf
0000582D  E80B00            call 0x583b
00005830  8026270DEB        and byte [0xd27],0xeb
00005835  9A8E03E507        call word 0x7e5:word 0x38e
0000583A  C3                ret
0000583B  57                push di
0000583C  06                push es
0000583D  C41E5E06          les bx,word [0x65e]
00005841  268B7F18          mov di,[es:bx+0x18]
00005845  268B4F0E          mov cx,[es:bx+0xe]
00005849  2BCF              sub cx,di
0000584B  1E                push ds
0000584C  07                pop es
0000584D  C706680A0000      mov word [0xa68],0x0
00005853  D1E9              shr cx,1
00005855  33C0              xor ax,ax
00005857  F3AB              rep stosw
00005859  893E6A0A          mov [0xa6a],di
0000585D  8C1E1D0D          mov word [0xd1d],ds
00005861  E83200            call 0x5896
00005864  07                pop es
00005865  5F                pop di
00005866  C3                ret
00005867  55                push bp
00005868  8BEC              mov bp,sp
0000586A  06                push es
0000586B  1E                push ds
0000586C  07                pop es
0000586D  8B4608            mov ax,[bp+0x8]
00005870  8B5E06            mov bx,[bp+0x6]
00005873  3BC3              cmp ax,bx
00005875  731A              jnc 0x5891
00005877  FF163E07          call word near [0x73e]
0000587B  8B4608            mov ax,[bp+0x8]
0000587E  8B5E06            mov bx,[bp+0x6]
00005881  8BCC              mov cx,sp
00005883  FF164007          call word near [0x740]
00005887  8B4608            mov ax,[bp+0x8]
0000588A  8B5E06            mov bx,[bp+0x6]
0000588D  FF164207          call word near [0x742]
00005891  07                pop es
00005892  5D                pop bp
00005893  CA0400            retf word 0x4
00005896  56                push si
00005897  57                push di
00005898  06                push es
00005899  1E                push ds
0000589A  07                pop es
0000589B  33C0              xor ax,ax
0000589D  F606270D10        test byte [0xd27],0x10
000058A2  7514              jnz 0x58b8
000058A4  06                push es
000058A5  C41E5E06          les bx,word [0x65e]
000058A9  268B7F0E          mov di,[es:bx+0xe]
000058AD  268B4F10          mov cx,[es:bx+0x10]
000058B1  07                pop es
000058B2  2BCF              sub cx,di
000058B4  D1E9              shr cx,1
000058B6  F3AB              rep stosw
000058B8  BF3C0D            mov di,0xd3c
000058BB  B91700            mov cx,0x17
000058BE  F3AA              rep stosb
000058C0  8BDC              mov bx,sp
000058C2  9A4B295802        call word 0x258:word 0x294b
000058C7  07                pop es
000058C8  5F                pop di
000058C9  5E                pop si
000058CA  C3                ret
000058CB  83EB04            sub bx,0x4
000058CE  8B3E100D          mov di,[0xd10]
000058D2  B880FE            mov ax,0xfe80
000058D5  AB                stosw
000058D6  40                inc ax
000058D7  40                inc ax
000058D8  3BFB              cmp di,bx
000058DA  72F9              jc 0x58d5
000058DC  CB                retf
000058DD  8C1E1D0D          mov word [0xd1d],ds
000058E1  CB                retf
000058E2  55                push bp
000058E3  8BEC              mov bp,sp
000058E5  8B4606            mov ax,[bp+0x6]
000058E8  A31D0D            mov [0xd1d],ax
000058EB  5D                pop bp
000058EC  CA0200            retf word 0x2
000058EF  00B302B8          add [bp+di-0x47fe],dh
000058F3  B303              mov bl,0x3
000058F5  B8B304            mov ax,0x4b3
000058F8  B8B305            mov ax,0x5b3
000058FB  B8B306            mov ax,0x6b3
000058FE  B8B307            mov ax,0x7b3
00005901  B8B309            mov ax,0x9b3
00005904  B8B30A            mov ax,0xab3
00005907  B8B30B            mov ax,0xbb3
0000590A  B8B30D            mov ax,0xdb3
0000590D  B8B30E            mov ax,0xeb3
00005910  B8B310            mov ax,0x10b3
00005913  B8B313            mov ax,0x13b3
00005916  B8B314            mov ax,0x14b3
00005919  B8B318            mov ax,0x18b3
0000591C  B8B319            mov ax,0x19b3
0000591F  B8B31B            mov ax,0x1bb3
00005922  B8B328            mov ax,0x28b3
00005925  B8B332            mov ax,0x32b3
00005928  B8B333            mov ax,0x33b3
0000592B  B8B334            mov ax,0x34b3
0000592E  B8B335            mov ax,0x35b3
00005931  B8B336            mov ax,0x36b3
00005934  B8B337            mov ax,0x37b3
00005937  B8B338            mov ax,0x38b3
0000593A  B8B339            mov ax,0x39b3
0000593D  B8B33A            mov ax,0x3ab3
00005940  B8B33B            mov ax,0x3bb3
00005943  B8B33D            mov ax,0x3db3
00005946  B8B33E            mov ax,0x3eb3
00005949  B8B33F            mov ax,0x3fb3
0000594C  B8B340            mov ax,0x40b3
0000594F  B8B343            mov ax,0x43b3
00005952  B8B344            mov ax,0x44b3
00005955  B8B345            mov ax,0x45b3
00005958  B8B346            mov ax,0x46b3
0000595B  B8B347            mov ax,0x47b3
0000595E  B8B348            mov ax,0x48b3
00005961  B8B349            mov ax,0x49b3
00005964  B8B34A            mov ax,0x4ab3
00005967  B8B34B            mov ax,0x4bb3
0000596A  B8B34C            mov ax,0x4cb3
0000596D  B8B3FF            mov ax,0xffb3
00005970  B8B300            mov ax,0xb3
00005973  B90090            mov cx,0x9000
00005976  EB31              jmp 0x59a9
00005978  B300              mov bl,0x0
0000597A  B90590            mov cx,0x9005
0000597D  EB2A              jmp 0x59a9
0000597F  B300              mov bl,0x0
00005981  B90790            mov cx,0x9007
00005984  EB23              jmp 0x59a9
00005986  B300              mov bl,0x0
00005988  B90094            mov cx,0x9400
0000598B  EB1C              jmp 0x59a9
0000598D  B300              mov bl,0x0
0000598F  B90098            mov cx,0x9800
00005992  EB15              jmp 0x59a9
00005994  B300              mov bl,0x0
00005996  B90198            mov cx,0x9801
00005999  EB0E              jmp 0x59a9
0000599B  B300              mov bl,0x0
0000599D  B90398            mov cx,0x9803
000059A0  EB07              jmp 0x59a9
000059A2  B300              mov bl,0x0
000059A4  B90498            mov cx,0x9804
000059A7  EB00              jmp 0x59a9
000059A9  B700              mov bh,0x0
000059AB  0BDB              or bx,bx
000059AD  7502              jnz 0x59b1
000059AF  87D9              xchg bx,cx
000059B1  EB43              jmp 0x59f6
000059B3  E949FF            jmp 0x58ff
000059B6  55                push bp
000059B7  8BEC              mov bp,sp
000059B9  E93DFF            jmp 0x58f9
000059BC  5D                pop bp
000059BD  C3                ret
000059BE  55                push bp
000059BF  8BEC              mov bp,sp
000059C1  EB9F              jmp 0x5962
000059C3  5D                pop bp
000059C4  C3                ret
000059C5  55                push bp
000059C6  8BEC              mov bp,sp
000059C8  8B5E06            mov bx,[bp+0x6]
000059CB  0BDB              or bx,bx
000059CD  7404              jz 0x59d3
000059CF  0AFF              or bh,bh
000059D1  7403              jz 0x59d6
000059D3  BB0500            mov bx,0x5
000059D6  EB1E              jmp 0x59f6
000059D8  B10B              mov cl,0xb
000059DA  EB0A              jmp 0x59e6
000059DC  B106              mov cl,0x6
000059DE  3B2E2C0D          cmp bp,[0xd2c]
000059E2  7402              jz 0x59e6
000059E4  B105              mov cl,0x5
000059E6  58                pop ax
000059E7  5A                pop dx
000059E8  9D                popf
000059E9  52                push dx
000059EA  50                push ax
000059EB  E85C0F            call 0x694a
000059EE  8EC3              mov es,bx
000059F0  8EDB              mov ds,bx
000059F2  32FF              xor bh,bh
000059F4  8AD9              mov bl,cl
000059F6  8B0E4407          mov cx,[0x744]
000059FA  E302              jcxz 0x59fe
000059FC  FFE1              jmp cx
000059FE  8BD3              mov dx,bx
00005A00  8BCD              mov cx,bp
00005A02  3B0E2C0D          cmp cx,[0xd2c]
00005A06  7507              jnz 0x5a0f
00005A08  55                push bp
00005A09  8BEC              mov bp,sp
00005A0B  8BDD              mov bx,bp
00005A0D  EB0C              jmp 0x5a1b
00005A0F  8BD9              mov bx,cx
00005A11  E3F5              jcxz 0x5a08
00005A13  8B0F              mov cx,[bx]
00005A15  3B0E2C0D          cmp cx,[0xd2c]
00005A19  75F4              jnz 0x5a0f
00005A1B  53                push bx
00005A1C  8916480D          mov [0xd48],dx
00005A20  53                push bx
00005A21  BE600A            mov si,0xa60
00005A24  9AA1285802        call word 0x258:word 0x28a1
00005A29  C6068A0A00        mov byte [0xa8a],0x0
00005A2E  9A8E03E507        call word 0x7e5:word 0x38e
00005A33  5B                pop bx
00005A34  803E490D98        cmp byte [0xd49],0x98
00005A39  7404              jz 0x5a3f
00005A3B  FF163C07          call word near [0x73c]
00005A3F  C6064C0D00        mov byte [0xd4c],0x0
00005A44  E8DF14            call 0x6f26
00005A47  C3                ret
00005A48  53                push bx
00005A49  B8FF9A            mov ax,0x9aff
00005A4C  E81200            call 0x5a61
00005A4F  58                pop ax
00005A50  E80E00            call 0x5a61
00005A53  B0FF              mov al,0xff
00005A55  50                push ax
00005A56  50                push ax
00005A57  50                push ax
00005A58  BBE507            mov bx,0x7e5
00005A5B  53                push bx
00005A5C  FF36660C          push word [0xc66]
00005A60  CB                retf
00005A61  56                push si
00005A62  0AE4              or ah,ah
00005A64  7502              jnz 0x5a68
00005A66  B410              mov ah,0x10
00005A68  E8B314            call 0x6f1e
00005A6B  0BC0              or ax,ax
00005A6D  7506              jnz 0x5a75
00005A6F  B8FF10            mov ax,0x10ff
00005A72  E8A914            call 0x6f1e
00005A75  8BF0              mov si,ax
00005A77  1E                push ds
00005A78  8EDA              mov ds,dx
00005A7A  AC                lodsb
00005A7B  1F                pop ds
00005A7C  0AC0              or al,al
00005A7E  7405              jz 0x5a85
00005A80  E83300            call 0x5ab6
00005A83  EBF2              jmp 0x5a77
00005A85  5E                pop si
00005A86  C3                ret
00005A87  50                push ax
00005A88  52                push dx
00005A89  B00D              mov al,0xd
00005A8B  E82800            call 0x5ab6
00005A8E  B00A              mov al,0xa
00005A90  E82300            call 0x5ab6
00005A93  8B166C0A          mov dx,[0xa6c]
00005A97  E8A510            call 0x6b3f
00005A9A  89166C0A          mov [0xa6c],dx
00005A9E  5A                pop dx
00005A9F  58                pop ax
00005AA0  C3                ret
00005AA1  50                push ax
00005AA2  D0E8              shr al,1
00005AA4  D0E8              shr al,1
00005AA6  D0E8              shr al,1
00005AA8  D0E8              shr al,1
00005AAA  E80100            call 0x5aae
00005AAD  58                pop ax
00005AAE  240F              and al,0xf
00005AB0  0490              add al,0x90
00005AB2  27                daa
00005AB3  1440              adc al,0x40
00005AB5  27                daa
00005AB6  52                push dx
00005AB7  8BD0              mov dx,ax
00005AB9  B406              mov ah,0x6
00005ABB  CD21              int byte 0x21
00005ABD  5A                pop dx
00005ABE  C3                ret
00005ABF  BB0A00            mov bx,0xa
00005AC2  33D2              xor dx,dx
00005AC4  52                push dx
00005AC5  33D2              xor dx,dx
00005AC7  F7F3              div bx
00005AC9  80C230            add dl,0x30
00005ACC  0BC0              or ax,ax
00005ACE  75F4              jnz 0x5ac4
00005AD0  8BC2              mov ax,dx
00005AD2  FFD1              call cx
00005AD4  58                pop ax
00005AD5  0BC0              or ax,ax
00005AD7  75F9              jnz 0x5ad2
00005AD9  C3                ret
00005ADA  B8FF9A            mov ax,0x9aff
00005ADD  E881FF            call 0x5a61
00005AE0  CB                retf
00005AE1  00BB0100          add [bp+di+0x1],bh
00005AE5  B102              mov cl,0x2
00005AE7  B80044            mov ax,0x4400
00005AEA  CD21              int byte 0x21
00005AEC  F6C280            test dl,0x80
00005AEF  7405              jz 0x5af6
00005AF1  F6C203            test dl,0x3
00005AF4  7504              jnz 0x5afa
00005AF6  080E240B          or [0xb24],cl
00005AFA  B101              mov cl,0x1
00005AFC  4B                dec bx
00005AFD  74E8              jz 0x5ae7
00005AFF  E80200            call 0x5b04
00005B02  C3                ret
00005B03  C3                ret
00005B04  32C0              xor al,al
00005B06  B433              mov ah,0x33
00005B08  CD21              int byte 0x21
00005B0A  88164A07          mov [0x74a],dl
00005B0E  32D2              xor dl,dl
00005B10  B001              mov al,0x1
00005B12  B433              mov ah,0x33
00005B14  CD21              int byte 0x21
00005B16  C3                ret
00005B17  8A164A07          mov dl,[0x74a]
00005B1B  B001              mov al,0x1
00005B1D  B433              mov ah,0x33
00005B1F  CD21              int byte 0x21
00005B21  C3                ret
00005B22  833E480D00        cmp word [0xd48],0x0
00005B27  7405              jz 0x5b2e
00005B29  9AC62B5802        call word 0x258:word 0x2bc6
00005B2E  803E1A0D00        cmp byte [0xd1a],0x0
00005B33  750D              jnz 0x5b42
00005B35  F606240B40        test byte [0xb24],0x40
00005B3A  7403              jz 0x5b3f
00005B3C  E8CB04            call 0x600a
00005B3F  E8B60D            call 0x68f8
00005B42  E8D2FF            call 0x5b17
00005B45  C3                ret
00005B46  F606240B03        test byte [0xb24],0x3
00005B4B  751C              jnz 0x5b69
00005B4D  E837FF            call 0x5a87
00005B50  B80780            mov ax,0x8007
00005B53  E80BFF            call 0x5a61
00005B56  B9C800            mov cx,0xc8
00005B59  B8E803            mov ax,0x3e8
00005B5C  48                dec ax
00005B5D  75FD              jnz 0x5b5c
00005B5F  E2F8              loop 0x5b59
00005B61  B8070C            mov ax,0xc07
00005B64  CD21              int byte 0x21
00005B66  E81EFF            call 0x5a87
00005B69  CB                retf
00005B6A  C3                ret
00005B6B  9C                pushf
00005B6C  33C0              xor ax,ax
00005B6E  1E                push ds
00005B6F  06                push es
00005B70  FA                cli
00005B71  B80035            mov ax,0x3500
00005B74  CD21              int byte 0x21
00005B76  891E4C07          mov [0x74c],bx
00005B7A  8C064E07          mov word [0x74e],es
00005B7E  B80435            mov ax,0x3504
00005B81  CD21              int byte 0x21
00005B83  891E5007          mov [0x750],bx
00005B87  8C065207          mov word [0x752],es
00005B8B  B82435            mov ax,0x3524
00005B8E  CD21              int byte 0x21
00005B90  891E5407          mov [0x754],bx
00005B94  8C065607          mov word [0x756],es
00005B98  07                pop es
00005B99  8CC8              mov ax,cs
00005B9B  8ED8              mov ds,ax
00005B9D  BA582A            mov dx,0x2a58
00005BA0  B80025            mov ax,0x2500
00005BA3  CD21              int byte 0x21
00005BA5  BA5C2A            mov dx,0x2a5c
00005BA8  B80425            mov ax,0x2504
00005BAB  CD21              int byte 0x21
00005BAD  BA0C2D            mov dx,0x2d0c
00005BB0  B82425            mov ax,0x2524
00005BB3  CD21              int byte 0x21
00005BB5  1F                pop ds
00005BB6  9D                popf
00005BB7  C3                ret
00005BB8  50                push ax
00005BB9  33C0              xor ax,ax
00005BBB  A33707            mov [0x737],ax
00005BBE  A35A07            mov [0x75a],ax
00005BC1  A35C07            mov [0x75c],ax
00005BC4  40                inc ax
00005BC5  E8EE10            call 0x6cb6
00005BC8  75FB              jnz 0x5bc5
00005BCA  58                pop ax
00005BCB  C3                ret
00005BCC  FF168E0A          call word near [0xa8e]
00005BD0  F606900A08        test byte [0xa90],0x8
00005BD5  7501              jnz 0x5bd8
00005BD7  C3                ret
00005BD8  8026900AF7        and byte [0xa90],0xf7
00005BDD  E9FB09            jmp 0x65db
00005BE0  52                push dx
00005BE1  833E370700        cmp word [0x737],0x0
00005BE6  7515              jnz 0x5bfd
00005BE8  A15A07            mov ax,[0x75a]
00005BEB  0AC0              or al,al
00005BED  750E              jnz 0x5bfd
00005BEF  0BE4              or sp,sp
00005BF1  E8C210            call 0x6cb6
00005BF4  7407              jz 0x5bfd
00005BF6  A35A07            mov [0x75a],ax
00005BF9  89165C07          mov [0x75c],dx
00005BFD  5A                pop dx
00005BFE  C3                ret
00005BFF  E8DEFF            call 0x5be0
00005C02  750E              jnz 0x5c12
00005C04  0AD2              or dl,dl
00005C06  74F7              jz 0x5bff
00005C08  E8C1FF            call 0x5bcc
00005C0B  EBF2              jmp 0x5bff
00005C0D  E8D0FF            call 0x5be0
00005C10  7410              jz 0x5c22
00005C12  833E370700        cmp word [0x737],0x0
00005C17  754A              jnz 0x5c63
00005C19  A15A07            mov ax,[0x75a]
00005C1C  8B165C07          mov dx,[0x75c]
00005C20  EB07              jmp 0x5c29
00005C22  33C0              xor ax,ax
00005C24  E88F10            call 0x6cb6
00005C27  7311              jnc 0x5c3a
00005C29  80FC80            cmp ah,0x80
00005C2C  750B              jnz 0x5c39
00005C2E  3C20              cmp al,0x20
00005C30  7204              jc 0x5c36
00005C32  3C2C              cmp al,0x2c
00005C34  720B              jc 0x5c41
00005C36  80FC80            cmp ah,0x80
00005C39  F5                cmc
00005C3A  C7065A070000      mov word [0x75a],0x0
00005C40  C3                ret
00005C41  56                push si
00005C42  50                push ax
00005C43  2C20              sub al,0x20
00005C45  BE0000            mov si,0x0
00005C48  32E4              xor ah,ah
00005C4A  D0E0              shl al,1
00005C4C  D0E0              shl al,1
00005C4E  03F0              add si,ax
00005C50  8B04              mov ax,[si]
00005C52  A33707            mov [0x737],ax
00005C55  0BC0              or ax,ax
00005C57  58                pop ax
00005C58  7425              jz 0x5c7f
00005C5A  8B7402            mov si,[si+0x2]
00005C5D  89363907          mov [0x739],si
00005C61  EB01              jmp 0x5c64
00005C63  56                push si
00005C64  8B363907          mov si,[0x739]
00005C68  8A04              mov al,[si]
00005C6A  32E4              xor ah,ah
00005C6C  3CFE              cmp al,0xfe
00005C6E  7507              jnz 0x5c77
00005C70  BAFE00            mov dx,0xfe
00005C73  89165C07          mov [0x75c],dx
00005C77  FF063907          inc word [0x739]
00005C7B  FF0E3707          dec word [0x737]
00005C7F  5E                pop si
00005C80  EBB4              jmp 0x5c36
00005C82  FF268E0A          jmp word near [0xa8e]
00005C86  803EDB0C03        cmp byte [0xcdb],0x3
00005C8B  C3                ret
00005C8C  FB                sti
00005C8D  1E                push ds
00005C8E  53                push bx
00005C8F  E8B80C            call 0x694a
00005C92  8EDB              mov ds,bx
00005C94  803E5F0700        cmp byte [0x75f],0x0
00005C99  7405              jz 0x5ca0
00005C9B  33C0              xor ax,ax
00005C9D  5B                pop bx
00005C9E  1F                pop ds
00005C9F  CF                iret
00005CA0  FA                cli
00005CA1  53                push bx
00005CA2  8BDC              mov bx,sp
00005CA4  8B4F18            mov cx,[bx+0x18]
00005CA7  5B                pop bx
00005CA8  8EC3              mov es,bx
00005CAA  8ED3              mov ss,bx
00005CAC  8BE1              mov sp,cx
00005CAE  FB                sti
00005CAF  51                push cx
00005CB0  8BCF              mov cx,di
00005CB2  A25E07            mov [0x75e],al
00005CB5  80E480            and ah,0x80
00005CB8  740C              jz 0x5cc6
00005CBA  1E                push ds
00005CBB  8EDD              mov ds,bp
00005CBD  F6440480          test byte [si+0x4],0x80
00005CC1  1F                pop ds
00005CC2  7402              jz 0x5cc6
00005CC4  32E4              xor ah,ah
00005CC6  8AC1              mov al,cl
00005CC8  A35807            mov [0x758],ax
00005CCB  A8FF              test al,0xff
00005CCD  750E              jnz 0x5cdd
00005CCF  C6065F0701        mov byte [0x75f],0x1
00005CD4  B40D              mov ah,0xd
00005CD6  CD21              int byte 0x21
00005CD8  C6065F0700        mov byte [0x75f],0x0
00005CDD  C606600700        mov byte [0x760],0x0
00005CE2  E87100            call 0x5d56
00005CE5  7504              jnz 0x5ceb
00005CE7  FE066007          inc byte [0x760]
00005CEB  B430              mov ah,0x30
00005CED  CD21              int byte 0x21
00005CEF  A15807            mov ax,[0x758]
00005CF2  BB720A            mov bx,0xa72
00005CF5  F6C480            test ah,0x80
00005CF8  7513              jnz 0x5d0d
00005CFA  C7070200          mov word [bx],0x2
00005CFE  8B5F02            mov bx,[bx+0x2]
00005D01  C707413A          mov word [bx],0x3a41
00005D05  8A2E5E07          mov ch,[0x75e]
00005D09  002F              add [bx],ch
00005D0B  EB14              jmp 0x5d21
00005D0D  1E                push ds
00005D0E  8EDD              mov ds,bp
00005D10  83C60A            add si,0xa
00005D13  B90800            mov cx,0x8
00005D16  26890F            mov [es:bx],cx
00005D19  268B7F02          mov di,[es:bx+0x2]
00005D1D  FC                cld
00005D1E  F3A4              rep movsb
00005D20  1F                pop ds
00005D21  5D                pop bp
00005D22  A33107            mov [0x731],ax
00005D25  FF168C0A          call word near [0xa8c]
00005D29  803E600700        cmp byte [0x760],0x0
00005D2E  755A              jnz 0x5d8a
00005D30  3C00              cmp al,0x0
00005D32  7456              jz 0x5d8a
00005D34  3C01              cmp al,0x1
00005D36  744C              jz 0x5d84
00005D38  3C09              cmp al,0x9
00005D3A  7445              jz 0x5d81
00005D3C  3C0A              cmp al,0xa
00005D3E  743E              jz 0x5d7e
00005D40  F6C480            test ah,0x80
00005D43  750A              jnz 0x5d4f
00005D45  3C02              cmp al,0x2
00005D47  7444              jz 0x5d8d
00005D49  3C07              cmp al,0x7
00005D4B  743A              jz 0x5d87
00005D4D  EB04              jmp 0x5d53
00005D4F  3C02              cmp al,0x2
00005D51  7428              jz 0x5d7b
00005D53  E9E5FB            jmp 0x593b
00005D56  53                push bx
00005D57  51                push cx
00005D58  52                push dx
00005D59  56                push si
00005D5A  57                push di
00005D5B  06                push es
00005D5C  1E                push ds
00005D5D  55                push bp
00005D5E  E825FF            call 0x5c86
00005D61  720F              jc 0x5d72
00005D63  33DB              xor bx,bx
00005D65  B459              mov ah,0x59
00005D67  CD21              int byte 0x21
00005D69  3D2000            cmp ax,0x20
00005D6C  7403              jz 0x5d71
00005D6E  3D2100            cmp ax,0x21
00005D71  F8                clc
00005D72  5D                pop bp
00005D73  1F                pop ds
00005D74  07                pop es
00005D75  5F                pop di
00005D76  5E                pop si
00005D77  5A                pop dx
00005D78  59                pop cx
00005D79  5B                pop bx
00005D7A  C3                ret
00005D7B  E99CFB            jmp 0x591a
00005D7E  E99CFB            jmp 0x591d
00005D81  E99CFB            jmp 0x5920
00005D84  E9CCFB            jmp 0x5953
00005D87  E9D5FB            jmp 0x595f
00005D8A  E9CCFB            jmp 0x5959
00005D8D  E9CCFB            jmp 0x595c
00005D90  33C0              xor ax,ax
00005D92  1E                push ds
00005D93  1E                push ds
00005D94  07                pop es
00005D95  8ED8              mov ds,ax
00005D97  9C                pushf
00005D98  FA                cli
00005D99  26C5164C07        lds dx,word [es:0x74c]
00005D9E  B80025            mov ax,0x2500
00005DA1  CD21              int byte 0x21
00005DA3  26C5165007        lds dx,word [es:0x750]
00005DA8  B80425            mov ax,0x2504
00005DAB  CD21              int byte 0x21
00005DAD  26C5165407        lds dx,word [es:0x754]
00005DB2  B82425            mov ax,0x2524
00005DB5  CD21              int byte 0x21
00005DB7  9D                popf
00005DB8  1F                pop ds
00005DB9  C3                ret
00005DBA  55                push bp
00005DBB  56                push si
00005DBC  57                push di
00005DBD  CD10              int byte 0x10
00005DBF  5F                pop di
00005DC0  5E                pop si
00005DC1  5D                pop bp
00005DC2  C3                ret
00005DC3  36A0960A          mov al,[ss:0xa96]
00005DC7  EB04              jmp 0x5dcd
00005DC9  36A0950A          mov al,[ss:0xa95]
00005DCD  52                push dx
00005DCE  368B16AE0A        mov dx,[ss:0xaae]
00005DD3  D1E2              shl dx,1
00005DD5  D1E2              shl dx,1
00005DD7  D1E2              shl dx,1
00005DD9  D1E2              shl dx,1
00005DDB  32E4              xor ah,ah
00005DDD  1E                push ds
00005DDE  50                push ax
00005DDF  F7E2              mul dx
00005DE1  8EDA              mov ds,dx
00005DE3  A34E04            mov [0x44e],ax
00005DE6  58                pop ax
00005DE7  A26204            mov [0x462],al
00005DEA  1F                pop ds
00005DEB  5A                pop dx
00005DEC  C3                ret
00005DED  50                push ax
00005DEE  53                push bx
00005DEF  8A3E950A          mov bh,[0xa95]
00005DF3  B403              mov ah,0x3
00005DF5  E8C2FF            call 0x5dba
00005DF8  42                inc dx
00005DF9  86F2              xchg dh,dl
00005DFB  42                inc dx
00005DFC  3AE9              cmp ch,cl
00005DFE  7604              jna 0x5e04
00005E00  8B0E100B          mov cx,[0xb10]
00005E04  C606920AFF        mov byte [0xa92],0xff
00005E09  5B                pop bx
00005E0A  58                pop ax
00005E0B  C3                ret
00005E0C  50                push ax
00005E0D  A1920A            mov ax,[0xa92]
00005E10  3CFF              cmp al,0xff
00005E12  7403              jz 0x5e17
00005E14  E88B00            call 0x5ea2
00005E17  58                pop ax
00005E18  EB3D              jmp 0x5e57
00005E1A  89166C0A          mov [0xa6c],dx
00005E1E  803E9C0A00        cmp byte [0xa9c],0x0
00005E23  7421              jz 0x5e46
00005E25  803EA00A00        cmp byte [0xaa0],0x0
00005E2A  751A              jnz 0x5e46
00005E2C  A1100B            mov ax,[0xb10]
00005E2F  EB18              jmp 0x5e49
00005E31  A10E0B            mov ax,[0xb0e]
00005E34  EB13              jmp 0x5e49
00005E36  803E9C0A00        cmp byte [0xa9c],0x0
00005E3B  75E8              jnz 0x5e25
00005E3D  813E920A0727      cmp word [0xa92],0x2707
00005E43  7501              jnz 0x5e46
00005E45  C3                ret
00005E46  B80727            mov ax,0x2707
00005E49  53                push bx
00005E4A  51                push cx
00005E4B  50                push ax
00005E4C  52                push dx
00005E4D  E80209            call 0x6752
00005E50  803EA00A00        cmp byte [0xaa0],0x0
00005E55  75B5              jnz 0x5e0c
00005E57  4A                dec dx
00005E58  86F2              xchg dh,dl
00005E5A  4A                dec dx
00005E5B  8A3E950A          mov bh,[0xa95]
00005E5F  50                push ax
00005E60  B402              mov ah,0x2
00005E62  E855FF            call 0x5dba
00005E65  58                pop ax
00005E66  803EA00A00        cmp byte [0xaa0],0x0
00005E6B  7530              jnz 0x5e9d
00005E6D  3B06920A          cmp ax,[0xa92]
00005E71  7422              jz 0x5e95
00005E73  91                xchg ax,cx
00005E74  B401              mov ah,0x1
00005E76  E841FF            call 0x5dba
00005E79  F6C520            test ch,0x20
00005E7C  7517              jnz 0x5e95
00005E7E  F606B70704        test byte [0x7b7],0x4
00005E83  7410              jz 0x5e95
00005E85  803EA40A19        cmp byte [0xaa4],0x19
00005E8A  7409              jz 0x5e95
00005E8C  BAD403            mov dx,0x3d4
00005E8F  91                xchg ax,cx
00005E90  B00A              mov al,0xa
00005E92  E8E202            call 0x6177
00005E95  5A                pop dx
00005E96  58                pop ax
00005E97  A3920A            mov [0xa92],ax
00005E9A  59                pop cx
00005E9B  5B                pop bx
00005E9C  C3                ret
00005E9D  E80200            call 0x5ea2
00005EA0  EBF3              jmp 0x5e95
00005EA2  3D0727            cmp ax,0x2707
00005EA5  7477              jz 0x5f1e
00005EA7  06                push es
00005EA8  8A1EA10A          mov bl,[0xaa1]
00005EAC  80FB13            cmp bl,0x13
00005EAF  746E              jz 0x5f1f
00005EB1  33C9              xor cx,cx
00005EB3  8EC1              mov es,cx
00005EB5  80FB40            cmp bl,0x40
00005EB8  750D              jnz 0x5ec7
00005EBA  F606B70706        test byte [0x7b7],0x6
00005EBF  7406              jz 0x5ec7
00005EC1  FF169D0A          call word near [0xa9d]
00005EC5  EB56              jmp 0x5f1d
00005EC7  26FF367C00        push word [es:0x7c]
00005ECC  26FF367E00        push word [es:0x7e]
00005ED1  26C7067C00B00B    mov word [es:0x7c],0xbb0
00005ED8  268C1E7E00        mov word [es:0x7e],ds
00005EDD  3B060E0B          cmp ax,[0xb0e]
00005EE1  9C                pushf
00005EE2  803EA10A08        cmp byte [0xaa1],0x8
00005EE7  7407              jz 0x5ef0
00005EE9  803EA10A0D        cmp byte [0xaa1],0xd
00005EEE  7210              jc 0x5f00
00005EF0  8A1EC90A          mov bl,[0xac9]
00005EF4  80CB80            or bl,0x80
00005EF7  B0DC              mov al,0xdc
00005EF9  9D                popf
00005EFA  740D              jz 0x5f09
00005EFC  FEC8              dec al
00005EFE  EB09              jmp 0x5f09
00005F00  9D                popf
00005F01  B080              mov al,0x80
00005F03  7402              jz 0x5f07
00005F05  FEC0              inc al
00005F07  B387              mov bl,0x87
00005F09  8A3E950A          mov bh,[0xa95]
00005F0D  41                inc cx
00005F0E  B409              mov ah,0x9
00005F10  E8A7FE            call 0x5dba
00005F13  268F067E00        pop word [es:0x7e]
00005F18  268F067C00        pop word [es:0x7c]
00005F1D  07                pop es
00005F1E  C3                ret
00005F1F  52                push dx
00005F20  50                push ax
00005F21  B403              mov ah,0x3
00005F23  E894FE            call 0x5dba
00005F26  33C0              xor ax,ax
00005F28  86C6              xchg al,dh
00005F2A  B103              mov cl,0x3
00005F2C  D3E0              shl ax,cl
00005F2E  D3E2              shl dx,cl
00005F30  8BCA              mov cx,dx
00005F32  8BD0              mov dx,ax
00005F34  FF16D90A          call word near [0xad9]
00005F38  A0C90A            mov al,[0xac9]
00005F3B  8AE0              mov ah,al
00005F3D  C41E6407          les bx,word [0x764]
00005F41  B90800            mov cx,0x8
00005F44  5A                pop dx
00005F45  3B160E0B          cmp dx,[0xb0e]
00005F49  7506              jnz 0x5f51
00005F4B  D1E9              shr cx,1
00005F4D  81C30005          add bx,0x500
00005F51  51                push cx
00005F52  B90400            mov cx,0x4
00005F55  263107            xor [es:bx],ax
00005F58  43                inc bx
00005F59  43                inc bx
00005F5A  E2F9              loop 0x5f55
00005F5C  59                pop cx
00005F5D  81C33801          add bx,0x138
00005F61  E2EE              loop 0x5f51
00005F63  5A                pop dx
00005F64  EBB7              jmp 0x5f1d
00005F66  50                push ax
00005F67  53                push bx
00005F68  51                push cx
00005F69  52                push dx
00005F6A  33C9              xor cx,cx
00005F6C  8A2E700A          mov ch,[0xa70]
00005F70  FECD              dec ch
00005F72  8A1E800A          mov bl,[0xa80]
00005F76  8A3E710A          mov bh,[0xa71]
00005F7A  4B                dec bx
00005F7B  FECF              dec bh
00005F7D  E849FE            call 0x5dc9
00005F80  8BD3              mov dx,bx
00005F82  8A3EB10A          mov bh,[0xab1]
00005F86  33C0              xor ax,ax
00005F88  3AEE              cmp ch,dh
00005F8A  7401              jz 0x5f8d
00005F8C  40                inc ax
00005F8D  B406              mov ah,0x6
00005F8F  E828FE            call 0x5dba
00005F92  E82EFE            call 0x5dc3
00005F95  5A                pop dx
00005F96  59                pop cx
00005F97  5B                pop bx
00005F98  58                pop ax
00005F99  C3                ret
00005F9A  F9                stc
00005F9B  C3                ret
00005F9C  50                push ax
00005F9D  53                push bx
00005F9E  51                push cx
00005F9F  52                push dx
00005FA0  8B1E970A          mov bx,[0xa97]
00005FA4  3807              cmp [bx],al
00005FA6  720B              jc 0x5fb3
00005FA8  8AD0              mov dl,al
00005FAA  32F6              xor dh,dh
00005FAC  03DA              add bx,dx
00005FAE  03DA              add bx,dx
00005FB0  FF5701            call word near [bx+0x1]
00005FB3  5A                pop dx
00005FB4  59                pop cx
00005FB5  5B                pop bx
00005FB6  58                pop ax
00005FB7  C3                ret
00005FB8  B401              mov ah,0x1
00005FBA  FD                std
00005FBB  0BC9              or cx,cx
00005FBD  7410              jz 0x5fcf
00005FBF  AD                lodsw
00005FC0  49                dec cx
00005FC1  0BC0              or ax,ax
00005FC3  B401              mov ah,0x1
00005FC5  7408              jz 0x5fcf
00005FC7  AD                lodsw
00005FC8  0AE4              or ah,ah
00005FCA  7505              jnz 0x5fd1
00005FCC  49                dec cx
00005FCD  0BE4              or sp,sp
00005FCF  FC                cld
00005FD0  C3                ret
00005FD1  FC                cld
00005FD2  E924F9            jmp 0x58f9
00005FD5  E8C4FF            call 0x5f9c
00005FD8  730B              jnc 0x5fe5
00005FDA  3C01              cmp al,0x1
00005FDC  7502              jnz 0x5fe0
00005FDE  32E0              xor ah,al
00005FE0  32C0              xor al,al
00005FE2  E8B7FF            call 0x5f9c
00005FE5  87D9              xchg bx,cx
00005FE7  52                push dx
00005FE8  FF16B90A          call word near [0xab9]
00005FEC  A0A30A            mov al,[0xaa3]
00005FEF  8A0EA40A          mov cl,[0xaa4]
00005FF3  E8730B            call 0x6b69
00005FF6  58                pop ax
00005FF7  3A06B00A          cmp al,[0xab0]
00005FFB  7706              ja 0x6003
00005FFD  3A26B00A          cmp ah,[0xab0]
00006001  7602              jna 0x6005
00006003  33C0              xor ax,ax
00006005  FF16BD0A          call word near [0xabd]
00006009  C3                ret
0000600A  F606B70708        test byte [0x7b7],0x8
0000600F  7518              jnz 0x6029
00006011  33C0              xor ax,ax
00006013  8EC0              mov es,ax
00006015  A0AA07            mov al,[0x7aa]
00006018  263A061004        cmp al,[es:0x410]
0000601D  26A21004          mov [es:0x410],al
00006021  1E                push ds
00006022  07                pop es
00006023  7404              jz 0x6029
00006025  E86307            call 0x678b
00006028  F9                stc
00006029  A1140B            mov ax,[0xb14]
0000602C  8B0E190B          mov cx,[0xb19]
00006030  7219              jc 0x604b
00006032  3A06A00A          cmp al,[0xaa0]
00006036  7512              jnz 0x604a
00006038  3A26A20A          cmp ah,[0xaa2]
0000603C  750C              jnz 0x604a
0000603E  3A2EA40A          cmp ch,[0xaa4]
00006042  7506              jnz 0x604a
00006044  3A0EA30A          cmp cl,[0xaa3]
00006048  7401              jz 0x604b
0000604A  F9                stc
0000604B  9C                pushf
0000604C  E84DFF            call 0x5f9c
0000604F  87D9              xchg bx,cx
00006051  FF16B90A          call word near [0xab9]
00006055  A0A30A            mov al,[0xaa3]
00006058  8A0EA40A          mov cl,[0xaa4]
0000605C  E80A0B            call 0x6b69
0000605F  9D                popf
00006060  730C              jnc 0x606e
00006062  E89208            call 0x68f7
00006065  FF16BB0A          call word near [0xabb]
00006069  E8E70A            call 0x6b53
0000606C  EB03              jmp 0x6071
0000606E  E8ED0A            call 0x6b5e
00006071  FF16BF0A          call word near [0xabf]
00006075  8B166C0A          mov dx,[0xa6c]
00006079  E8CAFD            call 0x5e46
0000607C  A1170B            mov ax,[0xb17]
0000607F  FF16BD0A          call word near [0xabd]
00006083  E867FD            call 0x5ded
00006086  33C9              xor cx,cx
00006088  860E2707          xchg cl,[0x727]
0000608C  E304              jcxz 0x6092
0000608E  FF16250B          call word near [0xb25]
00006092  803EA00A00        cmp byte [0xaa0],0x0
00006097  7405              jz 0x609e
00006099  E8AAFD            call 0x5e46
0000609C  EB06              jmp 0x60a4
0000609E  A1120B            mov ax,[0xb12]
000060A1  E8A5FD            call 0x5e49
000060A4  C3                ret
000060A5  005657            add [bp+0x57],dl
000060A8  06                push es
000060A9  1E                push ds
000060AA  1E                push ds
000060AB  07                pop es
000060AC  83F929            cmp cx,0x29
000060AF  750D              jnz 0x60be
000060B1  51                push cx
000060B2  BFD90A            mov di,0xad9
000060B5  B91700            mov cx,0x17
000060B8  B81A30            mov ax,0x301a
000060BB  F3AB              rep stosw
000060BD  59                pop cx
000060BE  BFA00A            mov di,0xaa0
000060C1  0E                push cs
000060C2  1F                pop ds
000060C3  8BF3              mov si,bx
000060C5  FC                cld
000060C6  F3A4              rep movsb
000060C8  06                push es
000060C9  1F                pop ds
000060CA  A0B20A            mov al,[0xab2]
000060CD  FF16E50A          call word near [0xae5]
000060D1  C606820700        mov byte [0x782],0x0
000060D6  1F                pop ds
000060D7  07                pop es
000060D8  5F                pop di
000060D9  5E                pop si
000060DA  C3                ret
000060DB  57                push di
000060DC  FF366407          push word [0x764]
000060E0  53                push bx
000060E1  50                push ax
000060E2  32E4              xor ah,ah
000060E4  A06207            mov al,[0x762]
000060E7  8BF8              mov di,ax
000060E9  52                push dx
000060EA  8BD1              mov dx,cx
000060EC  33C9              xor cx,cx
000060EE  FF16D90A          call word near [0xad9]
000060F2  8B0E6407          mov cx,[0x764]
000060F6  030ECC0A          add cx,[0xacc]
000060FA  890E7407          mov [0x774],cx
000060FE  5A                pop dx
000060FF  33C9              xor cx,cx
00006101  FF16D90A          call word near [0xad9]
00006105  8B0E6407          mov cx,[0x764]
00006109  890E7607          mov [0x776],cx
0000610D  58                pop ax
0000610E  8BC8              mov cx,ax
00006110  33D2              xor dx,dx
00006112  FF16D90A          call word near [0xad9]
00006116  8B0E6407          mov cx,[0x764]
0000611A  890E7807          mov [0x778],cx
0000611E  8A0E6207          mov cl,[0x762]
00006122  880E7C07          mov [0x77c],cl
00006126  5B                pop bx
00006127  8BCB              mov cx,bx
00006129  33D2              xor dx,dx
0000612B  FF16D90A          call word near [0xad9]
0000612F  8B0E6407          mov cx,[0x764]
00006133  890E7A07          mov [0x77a],cx
00006137  8A0E6207          mov cl,[0x762]
0000613B  880E7D07          mov [0x77d],cl
0000613F  8BC7              mov ax,di
00006141  A26207            mov [0x762],al
00006144  8F066407          pop word [0x764]
00006148  5F                pop di
00006149  C3                ret
0000614A  8B1ED50A          mov bx,[0xad5]
0000614E  8B16D70A          mov dx,[0xad7]
00006152  C3                ret
00006153  50                push ax
00006154  8CD8              mov ax,ds
00006156  360306C507        add ax,[ss:0x7c5]
0000615B  8ED8              mov ds,ax
0000615D  58                pop ax
0000615E  C3                ret
0000615F  50                push ax
00006160  8CC0              mov ax,es
00006162  360306C507        add ax,[ss:0x7c5]
00006167  8EC0              mov es,ax
00006169  58                pop ax
0000616A  C3                ret
0000616B  50                push ax
0000616C  8CD8              mov ax,ds
0000616E  362B06C507        sub ax,[ss:0x7c5]
00006173  8ED8              mov ds,ax
00006175  58                pop ax
00006176  C3                ret
00006177  EE                out dx,al
00006178  86C4              xchg al,ah
0000617A  42                inc dx
0000617B  EE                out dx,al
0000617C  4A                dec dx
0000617D  C3                ret
0000617E  50                push ax
0000617F  52                push dx
00006180  BAC403            mov dx,0x3c4
00006183  B002              mov al,0x2
00006185  EE                out dx,al
00006186  42                inc dx
00006187  B00F              mov al,0xf
00006189  EE                out dx,al
0000618A  BACE03            mov dx,0x3ce
0000618D  B001              mov al,0x1
0000618F  EE                out dx,al
00006190  42                inc dx
00006191  32C0              xor al,al
00006193  EE                out dx,al
00006194  4A                dec dx
00006195  B002              mov al,0x2
00006197  EE                out dx,al
00006198  42                inc dx
00006199  32C0              xor al,al
0000619B  EE                out dx,al
0000619C  4A                dec dx
0000619D  B80300            mov ax,0x3
000061A0  EE                out dx,al
000061A1  42                inc dx
000061A2  32C0              xor al,al
000061A4  EE                out dx,al
000061A5  4A                dec dx
000061A6  B008              mov al,0x8
000061A8  EE                out dx,al
000061A9  42                inc dx
000061AA  B0FF              mov al,0xff
000061AC  EE                out dx,al
000061AD  4A                dec dx
000061AE  B007              mov al,0x7
000061B0  EE                out dx,al
000061B1  42                inc dx
000061B2  B00F              mov al,0xf
000061B4  EE                out dx,al
000061B5  4A                dec dx
000061B6  B005              mov al,0x5
000061B8  EE                out dx,al
000061B9  42                inc dx
000061BA  A0B40A            mov al,[0xab4]
000061BD  2410              and al,0x10
000061BF  EE                out dx,al
000061C0  5A                pop dx
000061C1  58                pop ax
000061C2  C3                ret
000061C3  0000              add [bx+si],al
000061C5  0000              add [bx+si],al
000061C7  2819              sub [bx+di],bl
000061C9  40                inc ax
000061CA  01C8              add ax,cx
000061CC  0000              add [bx+si],al
000061CE  B80F3F            mov ax,0x3f0f
000061D1  02800007          add al,[bx+si+0x700]
000061D5  07                pop es
000061D6  07                pop es
000061D7  0000              add [bx+si],al
000061D9  1835              sbb [di],dh
000061DB  58                pop ax
000061DC  025233            add dl,[bp+si+0x33]
000061DF  E633              out byte 0x33,al
000061E1  53                push bx
000061E2  3438              xor al,0x38
000061E4  356835            xor ax,0x3568
000061E7  90                nop
000061E8  35B535            xor ax,0x35b5
000061EB  833400            xor word [si],0x0
000061EE  0200              add al,[bx+si]
000061F0  50                push ax
000061F1  198002C8          sbb [bx+si-0x37fe],ax
000061F5  0000              add [bx+si],al
000061F7  B80F3F            mov ax,0x3f0f
000061FA  0400              add al,0x0
000061FC  0103              add [bp+di],ax
000061FE  07                pop es
000061FF  07                pop es
00006200  0000              add [bx+si],al
00006202  1835              sbb [di],dh
00006204  58                pop ax
00006205  02DC              add bl,ah
00006207  33E6              xor sp,si
00006209  335334            xor dx,[bp+di+0x34]
0000620C  3835              cmp [di],dh
0000620E  683590            push word 0x9035
00006211  35B535            xor ax,0x35b5
00006214  833400            xor word [si],0x0
00006217  07                pop es
00006218  005019            add [bx+si+0x19],dl
0000621B  D002              rol byte [bp+si],1
0000621D  5E                pop si
0000621E  0100              add [bx+si],ax
00006220  B00F              mov al,0xf
00006222  0204              add al,[si]
00006224  0001              add [bx+di],al
00006226  0007              add [bx],al
00006228  07                pop es
00006229  0000              add [bx+si],al
0000622B  BF3258            mov di,0x5832
0000622E  02E1              add ah,cl
00006230  33E6              xor sp,si
00006232  335334            xor dx,[bp+di+0x34]
00006235  3835              cmp [di],dh
00006237  683563            push word 0x6335
0000623A  34B5              xor al,0xb5
0000623C  358334            xor ax,0x3483
0000623F  0008              add [bx+si],cl
00006241  0808              or [bx+si],cl
00006243  0808              or [bx+si],cl
00006245  0808              or [bx+si],cl
00006247  0018              add [bx+si],bl
00006249  1818              sbb [bx+si],bl
0000624B  1818              sbb [bx+si],bl
0000624D  1818              sbb [bx+si],bl
0000624F  BB9632            mov bx,0x3296
00006252  B007              mov al,0x7
00006254  F606B80701        test byte [0x7b8],0x1
00006259  7520              jnz 0x627b
0000625B  803EB80708        cmp byte [0x7b8],0x8
00006260  7419              jz 0x627b
00006262  2206160B          and al,[0xb16]
00006266  3C07              cmp al,0x7
00006268  7411              jz 0x627b
0000626A  32C0              xor al,al
0000626C  BB4432            mov bx,0x3244
0000626F  80F928            cmp cl,0x28
00006272  7405              jz 0x6279
00006274  BB6D32            mov bx,0x326d
00006277  B002              mov al,0x2
00006279  02C4              add al,ah
0000627B  B92900            mov cx,0x29
0000627E  50                push ax
0000627F  E824FE            call 0x60a6
00006282  58                pop ax
00006283  A3A10A            mov [0xaa1],ax
00006286  F606B70723        test byte [0x7b7],0x23
0000628B  7526              jnz 0x62b3
0000628D  B407              mov ah,0x7
0000628F  F606B70704        test byte [0x7b7],0x4
00006294  740D              jz 0x62a3
00006296  833EB90740        cmp word [0x7b9],0x40
0000629B  7706              ja 0x62a3
0000629D  3C01              cmp al,0x1
0000629F  7602              jna 0x62a3
000062A1  D0EC              shr ah,1
000062A3  8826B00A          mov [0xab0],ah
000062A7  F606B80702        test byte [0x7b8],0x2
000062AC  7405              jz 0x62b3
000062AE  C606AC0A0F        mov byte [0xaac],0xf
000062B3  3C07              cmp al,0x7
000062B5  740D              jz 0x62c4
000062B7  F606B8071C        test byte [0x7b8],0x1c
000062BC  7406              jz 0x62c4
000062BE  C706B50A2835      mov word [0xab5],0x3528
000062C4  A0B20A            mov al,[0xab2]
000062C7  A2940A            mov [0xa94],al
000062CA  C7060C0B7007      mov word [0xb0c],0x770
000062D0  F8                clc
000062D1  C3                ret
000062D2  B150              mov cl,0x50
000062D4  32C0              xor al,al
000062D6  3AD9              cmp bl,cl
000062D8  7462              jz 0x633c
000062DA  3A1EA30A          cmp bl,[0xaa3]
000062DE  F9                stc
000062DF  755B              jnz 0x633c
000062E1  80FF19            cmp bh,0x19
000062E4  7451              jz 0x6337
000062E6  FEC8              dec al
000062E8  F606B7070C        test byte [0x7b7],0xc
000062ED  F9                stc
000062EE  744C              jz 0x633c
000062F0  F606B80702        test byte [0x7b8],0x2
000062F5  F9                stc
000062F6  7544              jnz 0x633c
000062F8  80FF2B            cmp bh,0x2b
000062FB  741B              jz 0x6318
000062FD  F606B70708        test byte [0x7b7],0x8
00006302  F9                stc
00006303  7437              jz 0x633c
00006305  80FF32            cmp bh,0x32
00006308  F9                stc
00006309  7531              jnz 0x633c
0000630B  B90A01            mov cx,0x10a
0000630E  80FB28            cmp bl,0x28
00006311  7410              jz 0x6323
00006313  B90402            mov cx,0x204
00006316  EB0B              jmp 0x6323
00006318  B9E700            mov cx,0xe7
0000631B  80FB28            cmp bl,0x28
0000631E  7408              jz 0x6328
00006320  B9BE01            mov cx,0x1be
00006323  C606B00A03        mov byte [0xab0],0x3
00006328  833EB90740        cmp word [0x7b9],0x40
0000632D  7704              ja 0x6333
0000632F  D02EB00A          shr byte [0xab0],1
00006333  890EAE0A          mov [0xaae],cx
00006337  883EA40A          mov [0xaa4],bh
0000633B  F8                clc
0000633C  9F                lahf
0000633D  50                push ax
0000633E  06                push es
0000633F  57                push di
00006340  BF8407            mov di,0x784
00006343  8B1EAE0A          mov bx,[0xaae]
00006347  B104              mov cl,0x4
00006349  D3E3              shl bx,cl
0000634B  33C0              xor ax,ax
0000634D  B90800            mov cx,0x8
00006350  1E                push ds
00006351  07                pop es
00006352  AB                stosw
00006353  03C3              add ax,bx
00006355  E2FB              loop 0x6352
00006357  5F                pop di
00006358  07                pop es
00006359  58                pop ax
0000635A  9E                sahf
0000635B  C3                ret
0000635C  B128              mov cl,0x28
0000635E  E973FF            jmp 0x62d4
00006361  32C0              xor al,al
00006363  E974FF            jmp 0x62da
00006366  F606B70718        test byte [0x7b7],0x18
0000636B  7417              jz 0x6384
0000636D  B80212            mov ax,0x1202
00006370  803EA40A2B        cmp byte [0xaa4],0x2b
00006375  7503              jnz 0x637a
00006377  B80112            mov ax,0x1201
0000637A  B330              mov bl,0x30
0000637C  55                push bp
0000637D  56                push si
0000637E  57                push di
0000637F  CD10              int byte 0x10
00006381  5F                pop di
00006382  5E                pop si
00006383  5D                pop bp
00006384  A0A10A            mov al,[0xaa1]
00006387  55                push bp
00006388  56                push si
00006389  57                push di
0000638A  B400              mov ah,0x0
0000638C  CD10              int byte 0x10
0000638E  5F                pop di
0000638F  5E                pop si
00006390  5D                pop bp
00006391  803EA40A2B        cmp byte [0xaa4],0x2b
00006396  7405              jz 0x639d
00006398  803EA40A32        cmp byte [0xaa4],0x32
0000639D  7512              jnz 0x63b1
0000639F  B81211            mov ax,0x1112
000063A2  32DB              xor bl,bl
000063A4  55                push bp
000063A5  56                push si
000063A6  57                push di
000063A7  CD10              int byte 0x10
000063A9  5F                pop di
000063AA  5E                pop si
000063AB  5D                pop bp
000063AC  B80707            mov ax,0x707
000063AF  EB0C              jmp 0x63bd
000063B1  B80707            mov ax,0x707
000063B4  3806A10A          cmp [0xaa1],al
000063B8  7503              jnz 0x63bd
000063BA  B80C0C            mov ax,0xc0c
000063BD  A3100B            mov [0xb10],ax
000063C0  A3100B            mov [0xb10],ax
000063C3  A20E0B            mov [0xb0e],al
000063C6  8B166C0A          mov dx,[0xa6c]
000063CA  C606920AFF        mov byte [0xa92],0xff
000063CF  E864FA            call 0x5e36
000063D2  C3                ret
000063D3  A3950A            mov [0xa95],ax
000063D6  8AC4              mov al,ah
000063D8  55                push bp
000063D9  56                push si
000063DA  57                push di
000063DB  B405              mov ah,0x5
000063DD  CD10              int byte 0x10
000063DF  5F                pop di
000063E0  5E                pop si
000063E1  5D                pop bp
000063E2  C3                ret
000063E3  3A1EAB0A          cmp bl,[0xaab]
000063E7  7718              ja 0x6401
000063E9  0AF2              or dh,dl
000063EB  0AF4              or dh,ah
000063ED  7510              jnz 0x63ff
000063EF  3A06AC0A          cmp al,[0xaac]
000063F3  770C              ja 0x6401
000063F5  3C01              cmp al,0x1
000063F7  7206              jc 0x63ff
000063F9  B008              mov al,0x8
000063FB  7402              jz 0x63ff
000063FD  B018              mov al,0x18
000063FF  F8                clc
00006400  C3                ret
00006401  F9                stc
00006402  C3                ret
00006403  8B16B20A          mov dx,[0xab2]
00006407  8A3E8207          mov bh,[0x782]
0000640B  E302              jcxz 0x640f
0000640D  EB04              jmp 0x6413
0000640F  F9                stc
00006410  E98400            jmp 0x6497
00006413  E8A2FB            call 0x5fb8
00006416  7406              jz 0x641e
00006418  3C1F              cmp al,0x1f
0000641A  77F3              ja 0x640f
0000641C  86C2              xchg al,dl
0000641E  E897FB            call 0x5fb8
00006421  7406              jz 0x6429
00006423  3C0F              cmp al,0xf
00006425  77E8              ja 0x640f
00006427  86C6              xchg al,dh
00006429  E88CFB            call 0x5fb8
0000642C  740A              jz 0x6438
0000642E  E302              jcxz 0x6432
00006430  EBDD              jmp 0x640f
00006432  3C0F              cmp al,0xf
00006434  77D9              ja 0x640f
00006436  86C7              xchg al,bh
00006438  803EA10A07        cmp byte [0xaa1],0x7
0000643D  7513              jnz 0x6452
0000643F  F606B70704        test byte [0x7b7],0x4
00006444  740C              jz 0x6452
00006446  8AC6              mov al,dh
00006448  F6D0              not al
0000644A  0AC2              or al,dl
0000644C  2407              and al,0x7
0000644E  7402              jz 0x6452
00006450  32F6              xor dh,dh
00006452  8BC2              mov ax,dx
00006454  A3B20A            mov [0xab2],ax
00006457  86F2              xchg dh,dl
00006459  80E60F            and dh,0xf
0000645C  D0E2              shl dl,1
0000645E  80E210            and dl,0x10
00006461  0AD7              or dl,bh
00006463  80E407            and ah,0x7
00006466  B104              mov cl,0x4
00006468  D2E4              shl ah,cl
0000646A  A810              test al,0x10
0000646C  7403              jz 0x6471
0000646E  80CC80            or ah,0x80
00006471  0AE6              or ah,dh
00006473  8AEC              mov ch,ah
00006475  8826B10A          mov [0xab1],ah
00006479  8826940A          mov [0xa94],ah
0000647D  F606B8071C        test byte [0x7b8],0x1c
00006482  7512              jnz 0x6496
00006484  883E8207          mov [0x782],bh
00006488  8ADA              mov bl,dl
0000648A  32FF              xor bh,bh
0000648C  55                push bp
0000648D  56                push si
0000648E  57                push di
0000648F  B40B              mov ah,0xb
00006491  CD10              int byte 0x10
00006493  5F                pop di
00006494  5E                pop si
00006495  5D                pop bp
00006496  F8                clc
00006497  C3                ret
00006498  0001              add [bx+di],al
0000649A  0203              add al,[bp+di]
0000649C  0405              add al,0x5
0000649E  06                push es
0000649F  07                pop es
000064A0  1011              adc [bx+di],dl
000064A2  1213              adc dl,[bp+di]
000064A4  1415              adc al,0x15
000064A6  16                push ss
000064A7  17                pop ss
000064A8  0001              add [bx+di],al
000064AA  0203              add al,[bp+di]
000064AC  0405              add al,0x5
000064AE  1407              adc al,0x7
000064B0  3839              cmp [bx+di],bh
000064B2  3A3B              cmp bh,[bp+di]
000064B4  3C3D              cmp al,0x3d
000064B6  3E3F              ds aas
000064B8  803EA90700        cmp byte [0x7a9],0x0
000064BD  7428              jz 0x64e7
000064BF  A08207            mov al,[0x782]
000064C2  56                push si
000064C3  57                push di
000064C4  06                push es
000064C5  1E                push ds
000064C6  1E                push ds
000064C7  07                pop es
000064C8  BF9407            mov di,0x794
000064CB  57                push di
000064CC  C536B50A          lds si,word [0xab5]
000064D0  B91000            mov cx,0x10
000064D3  F3A4              rep movsb
000064D5  AA                stosb
000064D6  5A                pop dx
000064D7  1F                pop ds
000064D8  B002              mov al,0x2
000064DA  55                push bp
000064DB  56                push si
000064DC  57                push di
000064DD  B410              mov ah,0x10
000064DF  CD10              int byte 0x10
000064E1  5F                pop di
000064E2  5E                pop si
000064E3  5D                pop bp
000064E4  07                pop es
000064E5  5F                pop di
000064E6  5E                pop si
000064E7  C3                ret
000064E8  803EA90700        cmp byte [0x7a9],0x0
000064ED  741F              jz 0x650e
000064EF  3DFFFF            cmp ax,0xffff
000064F2  7504              jnz 0x64f8
000064F4  3BD0              cmp dx,ax
000064F6  7417              jz 0x650f
000064F8  FF16C30A          call word near [0xac3]
000064FC  7210              jc 0x650e
000064FE  86F8              xchg bh,al
00006500  B000              mov al,0x0
00006502  55                push bp
00006503  56                push si
00006504  57                push di
00006505  B410              mov ah,0x10
00006507  CD10              int byte 0x10
00006509  5F                pop di
0000650A  5E                pop si
0000650B  5D                pop bp
0000650C  F8                clc
0000650D  C3                ret
0000650E  F9                stc
0000650F  C3                ret
00006510  3A1EAB0A          cmp bl,[0xaab]
00006514  77F8              ja 0x650e
00006516  0AF2              or dh,dl
00006518  0AF4              or dh,ah
0000651A  75F2              jnz 0x650e
0000651C  3A06AC0A          cmp al,[0xaac]
00006520  77EC              ja 0x650e
00006522  803EAC0A0F        cmp byte [0xaac],0xf
00006527  750A              jnz 0x6533
00006529  B408              mov ah,0x8
0000652B  22E0              and ah,al
0000652D  D0E4              shl ah,1
0000652F  0AC4              or al,ah
00006531  32E4              xor ah,ah
00006533  F8                clc
00006534  C3                ret
00006535  57                push di
00006536  FC                cld
00006537  8BF9              mov di,cx
00006539  8A0EAB0A          mov cl,[0xaab]
0000653D  41                inc cx
0000653E  3BC1              cmp ax,cx
00006540  7246              jc 0x6588
00006542  26AD              es lodsw
00006544  99                cwd
00006545  83FF04            cmp di,0x4
00006548  7504              jnz 0x654e
0000654A  92                xchg ax,dx
0000654B  26AD              es lodsw
0000654D  92                xchg ax,dx
0000654E  83FAFF            cmp dx,0xffffffffffffffff
00006551  7504              jnz 0x6557
00006553  3BD0              cmp dx,ax
00006555  740A              jz 0x6561
00006557  33DB              xor bx,bx
00006559  51                push cx
0000655A  FF16C30A          call word near [0xac3]
0000655E  59                pop cx
0000655F  7227              jc 0x6588
00006561  E2DF              loop 0x6542
00006563  8A0EAB0A          mov cl,[0xaab]
00006567  41                inc cx
00006568  8BD9              mov bx,cx
0000656A  4E                dec si
0000656B  4E                dec si
0000656C  FD                std
0000656D  4B                dec bx
0000656E  26AD              es lodsw
00006570  99                cwd
00006571  83FF04            cmp di,0x4
00006574  7503              jnz 0x6579
00006576  92                xchg ax,dx
00006577  26AD              es lodsw
00006579  53                push bx
0000657A  51                push cx
0000657B  FF16C10A          call word near [0xac1]
0000657F  59                pop cx
00006580  5B                pop bx
00006581  7205              jc 0x6588
00006583  E2E8              loop 0x656d
00006585  F8                clc
00006586  EB01              jmp 0x6589
00006588  F9                stc
00006589  FC                cld
0000658A  5F                pop di
0000658B  C3                ret
0000658C  55                push bp
0000658D  8BEC              mov bp,sp
0000658F  803E4C0D00        cmp byte [0xd4c],0x0
00006594  7405              jz 0x659b
00006596  EA94295802        jmp word 0x258:word 0x2994
0000659B  C706480D0000      mov word [0xd48],0x0
000065A1  EB09              jmp 0x65ac
000065A3  55                push bp
000065A4  8BEC              mov bp,sp
000065A6  C706480D0000      mov word [0xd48],0x0
000065AC  E80800            call 0x65b7
000065AF  33C0              xor ax,ax
000065B1  50                push ax
000065B2  9A5E02E507        call word 0x7e5:word 0x25e
000065B7  C6064C0DFF        mov byte [0xd4c],0xff
000065BC  BE360A            mov si,0xa36
000065BF  9AA1285802        call word 0x258:word 0x28a1
000065C4  C3                ret
000065C5  56                push si
000065C6  813E480D0098      cmp word [0xd48],0x9800
000065CC  730B              jnc 0x65d9
000065CE  E8E6FF            call 0x65b7
000065D1  BE180A            mov si,0xa18
000065D4  9AA1285802        call word 0x258:word 0x28a1
000065D9  5E                pop si
000065DA  CB                retf
000065DB  B8000C            mov ax,0xc00
000065DE  CD21              int byte 0x21
000065E0  B80D80            mov ax,0x800d
000065E3  A3480D            mov [0xd48],ax
000065E6  E878F4            call 0x5a61
000065E9  EBC1              jmp 0x65ac
000065EB  0000              add [bx+si],al
000065ED  0000              add [bx+si],al
000065EF  0000              add [bx+si],al
000065F1  00803EB7          add [bx+si-0x48c2],al
000065F5  07                pop es
000065F6  007403            add [si+0x3],dh
000065F9  E95501            jmp 0x6751
000065FC  8026240BBF        and byte [0xb24],0xbf
00006601  06                push es
00006602  33DB              xor bx,bx
00006604  8EC3              mov es,bx
00006606  26A01004          mov al,[es:0x410]
0000660A  8AE0              mov ah,al
0000660C  A3AA07            mov [0x7aa],ax
0000660F  4B                dec bx
00006610  8EC3              mov es,bx
00006612  26A00E00          mov al,[es:0xe]
00006616  A2B607            mov [0x7b6],al
00006619  07                pop es
0000661A  BA2100            mov dx,0x21
0000661D  FA                cli
0000661E  EC                in al,dx
0000661F  803EB607FC        cmp byte [0x7b6],0xfc
00006624  7503              jnz 0x6629
00006626  24FB              and al,0xfb
00006628  EE                out dx,al
00006629  FB                sti
0000662A  A2B507            mov [0x7b5],al
0000662D  FA                cli
0000662E  06                push es
0000662F  B8EF35            mov ax,0x35ef
00006632  CD21              int byte 0x21
00006634  891EAC07          mov [0x7ac],bx
00006638  8C06AE07          mov word [0x7ae],es
0000663C  B8F035            mov ax,0x35f0
0000663F  CD21              int byte 0x21
00006641  891EB007          mov [0x7b0],bx
00006645  8C06B207          mov word [0x7b2],es
00006649  07                pop es
0000664A  06                push es
0000664B  B80935            mov ax,0x3509
0000664E  CD21              int byte 0x21
00006650  8CC0              mov ax,es
00006652  07                pop es
00006653  8BD3              mov dx,bx
00006655  1E                push ds
00006656  8ED8              mov ds,ax
00006658  B8EF25            mov ax,0x25ef
0000665B  CD21              int byte 0x21
0000665D  1F                pop ds
0000665E  06                push es
0000665F  B80835            mov ax,0x3508
00006662  CD21              int byte 0x21
00006664  8CC0              mov ax,es
00006666  07                pop es
00006667  8BD3              mov dx,bx
00006669  1E                push ds
0000666A  8ED8              mov ds,ax
0000666C  B8F025            mov ax,0x25f0
0000666F  CD21              int byte 0x21
00006671  1F                pop ds
00006672  1E                push ds
00006673  33C0              xor ax,ax
00006675  8ED8              mov ds,ax
00006677  06                push es
00006678  B81C35            mov ax,0x351c
0000667B  CD21              int byte 0x21
0000667D  891E1205          mov [0x512],bx
00006681  8C061405          mov word [0x514],es
00006685  07                pop es
00006686  1F                pop ds
00006687  B40F              mov ah,0xf
00006689  E82EF7            call 0x5dba
0000668C  8AD7              mov dl,bh
0000668E  8AF7              mov dh,bh
00006690  A2160B            mov [0xb16],al
00006693  8ACC              mov cl,ah
00006695  3C40              cmp al,0x40
00006697  7504              jnz 0x669d
00006699  B004              mov al,0x4
0000669B  EB0D              jmp 0x66aa
0000669D  8AE0              mov ah,al
0000669F  BBC00B            mov bx,0xbc0
000066A2  D7                xlatb
000066A3  80E401            and ah,0x1
000066A6  3C01              cmp al,0x1
000066A8  7602              jna 0x66ac
000066AA  32E4              xor ah,ah
000066AC  50                push ax
000066AD  52                push dx
000066AE  51                push cx
000066AF  E8D900            call 0x678b
000066B2  B030              mov al,0x30
000066B4  33DB              xor bx,bx
000066B6  33C9              xor cx,cx
000066B8  BA1800            mov dx,0x18
000066BB  06                push es
000066BC  B411              mov ah,0x11
000066BE  E8F9F6            call 0x5dba
000066C1  07                pop es
000066C2  42                inc dx
000066C3  59                pop cx
000066C4  8AEA              mov ch,dl
000066C6  5A                pop dx
000066C7  58                pop ax
000066C8  50                push ax
000066C9  51                push cx
000066CA  E808F9            call 0x5fd5
000066CD  59                pop cx
000066CE  58                pop ax
000066CF  3A06A00A          cmp al,[0xaa0]
000066D3  750C              jnz 0x66e1
000066D5  3A0EA30A          cmp cl,[0xaa3]
000066D9  7506              jnz 0x66e1
000066DB  3A2EA40A          cmp ch,[0xaa4]
000066DF  7405              jz 0x66e6
000066E1  800E1B0B01        or byte [0xb1b],0x1
000066E6  A0A00A            mov al,[0xaa0]
000066E9  8A26A20A          mov ah,[0xaa2]
000066ED  A3140B            mov [0xb14],ax
000066F0  A1950A            mov ax,[0xa95]
000066F3  A3170B            mov [0xb17],ax
000066F6  A1A30A            mov ax,[0xaa3]
000066F9  A3190B            mov [0xb19],ax
000066FC  80FC19            cmp ah,0x19
000066FF  7510              jnz 0x6711
00006701  803EA10A07        cmp byte [0xaa1],0x7
00006706  7509              jnz 0x6711
00006708  B80C0C            mov ax,0xc0c
0000670B  A3100B            mov [0xb10],ax
0000670E  A20E0B            mov [0xb0e],al
00006711  E8D9F6            call 0x5ded
00006714  890E120B          mov [0xb12],cx
00006718  F6061B0B01        test byte [0xb1b],0x1
0000671D  7505              jnz 0x6724
0000671F  E8EA03            call 0x6b0c
00006722  7403              jz 0x6727
00006724  BA0101            mov dx,0x101
00006727  3A16A40A          cmp dl,[0xaa4]
0000672B  7506              jnz 0x6733
0000672D  4A                dec dx
0000672E  800E240B80        or byte [0xb24],0x80
00006733  89166C0A          mov [0xa6c],dx
00006737  E82404            call 0x6b5e
0000673A  FB                sti
0000673B  B430              mov ah,0x30
0000673D  CD21              int byte 0x21
0000673F  3C03              cmp al,0x3
00006741  720D              jc 0x6750
00006743  32E4              xor ah,ah
00006745  CD2A              int byte 0x2a
00006747  0AE4              or ah,ah
00006749  7405              jz 0x6750
0000674B  C606B40701        mov byte [0x7b4],0x1
00006750  F8                clc
00006751  CB                retf
00006752  F606240B40        test byte [0xb24],0x40
00006757  7401              jz 0x675a
00006759  C3                ret
0000675A  50                push ax
0000675B  53                push bx
0000675C  51                push cx
0000675D  52                push dx
0000675E  800E240B40        or byte [0xb24],0x40
00006763  F6061B0B01        test byte [0xb1b],0x1
00006768  740E              jz 0x6778
0000676A  E88A01            call 0x68f7
0000676D  FF16BB0A          call word near [0xabb]
00006771  A1950A            mov ax,[0xa95]
00006774  FF16BD0A          call word near [0xabd]
00006778  F606240B80        test byte [0xb24],0x80
0000677D  7403              jz 0x6782
0000677F  E8E4F7            call 0x5f66
00006782  FF16BF0A          call word near [0xabf]
00006786  5A                pop dx
00006787  59                pop cx
00006788  5B                pop bx
00006789  58                pop ax
0000678A  C3                ret
0000678B  50                push ax
0000678C  53                push bx
0000678D  51                push cx
0000678E  06                push es
0000678F  57                push di
00006790  B80200            mov ax,0x2
00006793  A2B707            mov [0x7b7],al
00006796  C706B9071000      mov word [0x7b9],0x10
0000679C  A2B807            mov [0x7b8],al
0000679F  8826A907          mov [0x7a9],ah
000067A3  1E                push ds
000067A4  07                pop es
000067A5  BFC707            mov di,0x7c7
000067A8  33DB              xor bx,bx
000067AA  B41B              mov ah,0x1b
000067AC  E80BF6            call 0x5dba
000067AF  3C1B              cmp al,0x1b
000067B1  7532              jnz 0x67e5
000067B3  C606B80718        mov byte [0x7b8],0x18
000067B8  8A1EF807          mov bl,[0x7f8]
000067BC  C43D              les di,word [di]
000067BE  268B05            mov ax,[es:di]
000067C1  A3A607            mov [0x7a6],ax
000067C4  268A4502          mov al,[es:di+0x2]
000067C8  A2A807            mov [0x7a8],al
000067CB  C606B70710        mov byte [0x7b7],0x10
000067D0  A804              test al,0x4
000067D2  7449              jz 0x681d
000067D4  E8BE00            call 0x6895
000067D7  7252              jc 0x682b
000067D9  C606B70708        mov byte [0x7b7],0x8
000067DE  C606A90701        mov byte [0x7a9],0x1
000067E3  EB38              jmp 0x681d
000067E5  B310              mov bl,0x10
000067E7  B412              mov ah,0x12
000067E9  E8CEF5            call 0x5dba
000067EC  F6C3FC            test bl,0xfc
000067EF  753A              jnz 0x682b
000067F1  A0AB07            mov al,[0x7ab]
000067F4  253000            and ax,0x30
000067F7  3C30              cmp al,0x30
000067F9  7502              jnz 0x67fd
000067FB  FEC4              inc ah
000067FD  3AE7              cmp ah,bh
000067FF  752A              jnz 0x682b
00006801  C606B70704        mov byte [0x7b7],0x4
00006806  C606A90701        mov byte [0x7a9],0x1
0000680B  80E10F            and cl,0xf
0000680E  80E909            sub cl,0x9
00006811  720A              jc 0x681d
00006813  B504              mov ch,0x4
00006815  7402              jz 0x6819
00006817  B501              mov ch,0x1
00006819  882EB807          mov [0x7b8],ch
0000681D  FEC3              inc bl
0000681F  32FF              xor bh,bh
00006821  B106              mov cl,0x6
00006823  D3E3              shl bx,cl
00006825  891EB907          mov [0x7b9],bx
00006829  EB17              jmp 0x6842
0000682B  A0AB07            mov al,[0x7ab]
0000682E  2430              and al,0x30
00006830  3C30              cmp al,0x30
00006832  750E              jnz 0x6842
00006834  FE0EB807          dec byte [0x7b8]
00006838  FE0EB707          dec byte [0x7b7]
0000683C  C706B9070400      mov word [0x7b9],0x4
00006842  F606B7070E        test byte [0x7b7],0xe
00006847  7419              jz 0x6862
00006849  33C0              xor ax,ax
0000684B  FF160A0B          call word near [0xb0a]
0000684F  7411              jz 0x6862
00006851  0806B707          or [0x7b7],al
00006855  833EB90720        cmp word [0x7b9],0x20
0000685A  7706              ja 0x6862
0000685C  C706B9072000      mov word [0x7b9],0x20
00006862  BAFFFF            mov dx,0xffff
00006865  B4EF              mov ah,0xef
00006867  E850F5            call 0x5dba
0000686A  80FAFF            cmp dl,0xff
0000686D  7420              jz 0x688f
0000686F  F606B70701        test byte [0x7b7],0x1
00006874  7413              jz 0x6889
00006876  C606B70720        mov byte [0x7b7],0x20
0000687B  C706B9074000      mov word [0x7b9],0x40
00006881  0AF6              or dh,dh
00006883  750A              jnz 0x688f
00006885  D12EB907          shr word [0x7b9],1
00006889  BABF03            mov dx,0x3bf
0000688C  B001              mov al,0x1
0000688E  EE                out dx,al
0000688F  5F                pop di
00006890  07                pop es
00006891  59                pop cx
00006892  5B                pop bx
00006893  58                pop ax
00006894  C3                ret
00006895  06                push es
00006896  33C0              xor ax,ax
00006898  8EC0              mov es,ax
0000689A  26A18804          mov ax,[es:0x488]
0000689E  F6C401            test ah,0x1
000068A1  7552              jnz 0x68f5
000068A3  A808              test al,0x8
000068A5  7502              jnz 0x68a9
000068A7  3402              xor al,0x2
000068A9  268A261004        mov ah,[es:0x410]
000068AE  8826AB07          mov [0x7ab],ah
000068B2  80E430            and ah,0x30
000068B5  80FC30            cmp ah,0x30
000068B8  7402              jz 0x68bc
000068BA  3402              xor al,0x2
000068BC  A802              test al,0x2
000068BE  7424              jz 0x68e4
000068C0  80FC30            cmp ah,0x30
000068C3  740D              jz 0x68d2
000068C5  8126A607FFFE      and word [0x7a6],0xfeff
000068CB  C606B80710        mov byte [0x7b8],0x10
000068D0  EB23              jmp 0x68f5
000068D2  C606A80700        mov byte [0x7a8],0x0
000068D7  8126A6070001      and word [0x7a6],0x100
000068DD  C606B80708        mov byte [0x7b8],0x8
000068E2  EB11              jmp 0x68f5
000068E4  33C0              xor ax,ax
000068E6  A2A807            mov [0x7a8],al
000068E9  A3A607            mov [0x7a6],ax
000068EC  B002              mov al,0x2
000068EE  A2B707            mov [0x7b7],al
000068F1  A2B807            mov [0x7b8],al
000068F4  F9                stc
000068F5  07                pop es
000068F6  C3                ret
000068F7  C3                ret
000068F8  E421              in al,byte 0x21
000068FA  0C01              or al,0x1
000068FC  EB00              jmp 0x68fe
000068FE  E621              out byte 0x21,al
00006900  FA                cli
00006901  1E                push ds
00006902  33C0              xor ax,ax
00006904  8ED8              mov ds,ax
00006906  C5161205          lds dx,word [0x512]
0000690A  B81C25            mov ax,0x251c
0000690D  CD21              int byte 0x21
0000690F  1F                pop ds
00006910  06                push es
00006911  B8EF35            mov ax,0x35ef
00006914  CD21              int byte 0x21
00006916  8CC0              mov ax,es
00006918  07                pop es
00006919  8BD3              mov dx,bx
0000691B  1E                push ds
0000691C  8ED8              mov ds,ax
0000691E  B80925            mov ax,0x2509
00006921  CD21              int byte 0x21
00006923  1F                pop ds
00006924  1E                push ds
00006925  C516AC07          lds dx,word [0x7ac]
00006929  B8EF25            mov ax,0x25ef
0000692C  CD21              int byte 0x21
0000692E  1F                pop ds
0000692F  1E                push ds
00006930  C516B007          lds dx,word [0x7b0]
00006934  B8F025            mov ax,0x25f0
00006937  CD21              int byte 0x21
00006939  1F                pop ds
0000693A  E421              in al,byte 0x21
0000693C  24FE              and al,0xfe
0000693E  EB00              jmp 0x6940
00006940  E621              out byte 0x21,al
00006942  FB                sti
00006943  C3                ret
00006944  2E8C1E7036        mov word [cs:0x3670],ds
00006949  C3                ret
0000694A  2E8B1E7036        mov bx,[cs:0x3670]
0000694F  C3                ret
00006950  52                push dx
00006951  51                push cx
00006952  53                push bx
00006953  50                push ax
00006954  E8FBFD            call 0x6752
00006957  50                push ax
00006958  E86EF4            call 0x5dc9
0000695B  58                pop ax
0000695C  3C01              cmp al,0x1
0000695E  7418              jz 0x6978
00006960  7718              ja 0x697a
00006962  BA0101            mov dx,0x101
00006965  52                push dx
00006966  8A16A30A          mov dl,[0xaa3]
0000696A  FECA              dec dl
0000696C  8A36A40A          mov dh,[0xaa4]
00006970  FECE              dec dh
00006972  FECE              dec dh
00006974  33C9              xor cx,cx
00006976  EB17              jmp 0x698f
00006978  EB2B              jmp 0x69a5
0000697A  33C9              xor cx,cx
0000697C  8A0E700A          mov cl,[0xa70]
00006980  51                push cx
00006981  49                dec cx
00006982  86E9              xchg ch,cl
00006984  8A16800A          mov dl,[0xa80]
00006988  8A36710A          mov dh,[0xa71]
0000698C  4A                dec dx
0000698D  FECE              dec dh
0000698F  8A3EB10A          mov bh,[0xab1]
00006993  32C0              xor al,al
00006995  B406              mov ah,0x6
00006997  E820F4            call 0x5dba
0000699A  5A                pop dx
0000699B  B601              mov dh,0x1
0000699D  C606920AFF        mov byte [0xa92],0xff
000069A2  E879F4            call 0x5e1e
000069A5  E81BF4            call 0x5dc3
000069A8  58                pop ax
000069A9  5B                pop bx
000069AA  59                pop cx
000069AB  5A                pop dx
000069AC  3C02              cmp al,0x2
000069AE  7702              ja 0x69b2
000069B0  F8                clc
000069B1  C3                ret
000069B2  B000              mov al,0x0
000069B4  F9                stc
000069B5  C3                ret
000069B6  51                push cx
000069B7  52                push dx
000069B8  56                push si
000069B9  50                push ax
000069BA  8BF4              mov si,sp
000069BC  B90100            mov cx,0x1
000069BF  E80500            call 0x69c7
000069C2  58                pop ax
000069C3  5E                pop si
000069C4  5A                pop dx
000069C5  59                pop cx
000069C6  C3                ret
000069C7  53                push bx
000069C8  51                push cx
000069C9  56                push si
000069CA  E885FD            call 0x6752
000069CD  8B1E940A          mov bx,[0xa94]
000069D1  803EA00A00        cmp byte [0xaa0],0x0
000069D6  7407              jz 0x69df
000069D8  E86BF4            call 0x5e46
000069DB  8A1EC90A          mov bl,[0xac9]
000069DF  4A                dec dx
000069E0  86F2              xchg dh,dl
000069E2  4A                dec dx
000069E3  57                push di
000069E4  E80900            call 0x69f0
000069E7  5F                pop di
000069E8  42                inc dx
000069E9  86F2              xchg dh,dl
000069EB  42                inc dx
000069EC  5E                pop si
000069ED  59                pop cx
000069EE  5B                pop bx
000069EF  C3                ret
000069F0  36803EA00A00      cmp byte [ss:0xaa0],0x0
000069F6  7535              jnz 0x6a2d
000069F8  06                push es
000069F9  E84400            call 0x6a40
000069FC  8AE3              mov ah,bl
000069FE  36F606B70702      test byte [ss:0x7b7],0x2
00006A04  7421              jz 0x6a27
00006A06  52                push dx
00006A07  BADA03            mov dx,0x3da
00006A0A  FB                sti
00006A0B  90                nop
00006A0C  FA                cli
00006A0D  EC                in al,dx
00006A0E  A808              test al,0x8
00006A10  7513              jnz 0x6a25
00006A12  A801              test al,0x1
00006A14  75F4              jnz 0x6a0a
00006A16  AC                lodsb
00006A17  93                xchg ax,bx
00006A18  EC                in al,dx
00006A19  A801              test al,0x1
00006A1B  74FB              jz 0x6a18
00006A1D  93                xchg ax,bx
00006A1E  AB                stosw
00006A1F  E2E9              loop 0x6a0a
00006A21  FB                sti
00006A22  5A                pop dx
00006A23  07                pop es
00006A24  C3                ret
00006A25  FB                sti
00006A26  5A                pop dx
00006A27  AC                lodsb
00006A28  AB                stosw
00006A29  E2FC              loop 0x6a27
00006A2B  07                pop es
00006A2C  C3                ret
00006A2D  8BF9              mov di,cx
00006A2F  B90100            mov cx,0x1
00006A32  AC                lodsb
00006A33  B409              mov ah,0x9
00006A35  CD10              int byte 0x10
00006A37  42                inc dx
00006A38  B402              mov ah,0x2
00006A3A  CD10              int byte 0x10
00006A3C  4F                dec di
00006A3D  75F3              jnz 0x6a32
00006A3F  C3                ret
00006A40  33C0              xor ax,ax
00006A42  8EC0              mov es,ax
00006A44  8AC7              mov al,bh
00006A46  97                xchg ax,di
00006A47  D1E7              shl di,1
00006A49  8BC2              mov ax,dx
00006A4B  02C1              add al,cl
00006A4D  2689855004        mov [es:di+0x450],ax
00006A52  36A0A30A          mov al,[ss:0xaa3]
00006A56  F6E4              mul ah
00006A58  02C2              add al,dl
00006A5A  80D400            adc ah,0x0
00006A5D  D1E0              shl ax,1
00006A5F  3603858407        add ax,[ss:di+0x784]
00006A64  97                xchg ax,di
00006A65  368E06A90A        mov es,word [ss:0xaa9]
00006A6A  02D1              add dl,cl
00006A6C  C3                ret
00006A6D  50                push ax
00006A6E  53                push bx
00006A6F  51                push cx
00006A70  52                push dx
00006A71  E8D2F3            call 0x5e46
00006A74  8A0EA30A          mov cl,[0xaa3]
00006A78  2ACE              sub cl,dh
00006A7A  FEC1              inc cl
00006A7C  32ED              xor ch,ch
00006A7E  8A3E950A          mov bh,[0xa95]
00006A82  8A1EB10A          mov bl,[0xab1]
00006A86  803EA00A04        cmp byte [0xaa0],0x4
00006A8B  7504              jnz 0x6a91
00006A8D  8A1EB20A          mov bl,[0xab2]
00006A91  B020              mov al,0x20
00006A93  B409              mov ah,0x9
00006A95  E822F3            call 0x5dba
00006A98  8B166C0A          mov dx,[0xa6c]
00006A9C  E8A7F3            call 0x5e46
00006A9F  5A                pop dx
00006AA0  59                pop cx
00006AA1  5B                pop bx
00006AA2  58                pop ax
00006AA3  C3                ret
00006AA4  50                push ax
00006AA5  52                push dx
00006AA6  B207              mov dl,0x7
00006AA8  B80106            mov ax,0x601
00006AAB  3806BC07          cmp [0x7bc],al
00006AAF  74FA              jz 0x6aab
00006AB1  CD21              int byte 0x21
00006AB3  5A                pop dx
00006AB4  58                pop ax
00006AB5  C3                ret
00006AB6  50                push ax
00006AB7  53                push bx
00006AB8  51                push cx
00006AB9  52                push dx
00006ABA  3CFF              cmp al,0xff
00006ABC  7406              jz 0x6ac4
00006ABE  8AD0              mov dl,al
00006AC0  B406              mov ah,0x6
00006AC2  CD21              int byte 0x21
00006AC4  5A                pop dx
00006AC5  59                pop cx
00006AC6  5B                pop bx
00006AC7  58                pop ax
00006AC8  C3                ret
00006AC9  005351            add [bp+di+0x51],dl
00006ACC  52                push dx
00006ACD  33DB              xor bx,bx
00006ACF  B90100            mov cx,0x1
00006AD2  BAC207            mov dx,0x7c2
00006AD5  B43F              mov ah,0x3f
00006AD7  CD21              int byte 0x21
00006AD9  0BC0              or ax,ax
00006ADB  7403              jz 0x6ae0
00006ADD  A0C207            mov al,[0x7c2]
00006AE0  5A                pop dx
00006AE1  59                pop cx
00006AE2  5B                pop bx
00006AE3  C3                ret
00006AE4  53                push bx
00006AE5  51                push cx
00006AE6  52                push dx
00006AE7  BB0100            mov bx,0x1
00006AEA  B90100            mov cx,0x1
00006AED  A2C207            mov [0x7c2],al
00006AF0  7304              jnc 0x6af6
00006AF2  41                inc cx
00006AF3  A3C207            mov [0x7c2],ax
00006AF6  BAC207            mov dx,0x7c2
00006AF9  B440              mov ah,0x40
00006AFB  CD21              int byte 0x21
00006AFD  5A                pop dx
00006AFE  59                pop cx
00006AFF  5B                pop bx
00006B00  C3                ret
00006B01  8B0EA50A          mov cx,[0xaa5]
00006B05  49                dec cx
00006B06  8B16A70A          mov dx,[0xaa7]
00006B0A  4A                dec dx
00006B0B  C3                ret
00006B0C  803EA00A00        cmp byte [0xaa0],0x0
00006B11  C3                ret
00006B12  A1B20A            mov ax,[0xab2]
00006B15  33DB              xor bx,bx
00006B17  86DC              xchg bl,ah
00006B19  C3                ret
00006B1A  51                push cx
00006B1B  7219              jc 0x6b36
00006B1D  8A0E940A          mov cl,[0xa94]
00006B21  803EB30A00        cmp byte [0xab3],0x0
00006B26  7706              ja 0x6b2e
00006B28  860E0C0B          xchg cl,[0xb0c]
00006B2C  EB04              jmp 0x6b32
00006B2E  860E0D0B          xchg cl,[0xb0d]
00006B32  880E940A          mov [0xa94],cl
00006B36  59                pop cx
00006B37  C3                ret
00006B38  8A16700A          mov dl,[0xa70]
00006B3C  B601              mov dh,0x1
00006B3E  C3                ret
00006B3F  B601              mov dh,0x1
00006B41  3A166E0A          cmp dl,[0xa6e]
00006B45  7504              jnz 0x6b4b
00006B47  8A16710A          mov dl,[0xa71]
00006B4B  3816710A          cmp [0xa71],dl
00006B4F  7401              jz 0x6b52
00006B51  42                inc dx
00006B52  C3                ret
00006B53  E8E2FF            call 0x6b38
00006B56  89166C0A          mov [0xa6c],dx
00006B5A  FF16250B          call word near [0xb25]
00006B5E  FF16760A          call word near [0xa76]
00006B62  C706DE060000      mov word [0x6de],0x0
00006B68  C3                ret
00006B69  A2800A            mov [0xa80],al
00006B6C  A26F0A            mov [0xa6f],al
00006B6F  880E6E0A          mov [0xa6e],cl
00006B73  C606700A01        mov byte [0xa70],0x1
00006B78  51                push cx
00006B79  FEC9              dec cl
00006B7B  880E710A          mov [0xa71],cl
00006B7F  59                pop cx
00006B80  C3                ret
00006B81  3C61              cmp al,0x61
00006B83  7206              jc 0x6b8b
00006B85  3C7A              cmp al,0x7a
00006B87  7702              ja 0x6b8b
00006B89  24DF              and al,0xdf
00006B8B  C3                ret
00006B8C  87DA              xchg bx,dx
00006B8E  F7E1              mul cx
00006B90  50                push ax
00006B91  52                push dx
00006B92  93                xchg ax,bx
00006B93  F7E1              mul cx
00006B95  5A                pop dx
00006B96  7203              jc 0x6b9b
00006B98  03C2              add ax,dx
00006B9A  92                xchg ax,dx
00006B9B  58                pop ax
00006B9C  C3                ret
00006B9D  0000              add [bx+si],al
00006B9F  0102              add [bp+si],ax
00006BA1  0304              add ax,[si]
00006BA3  05060E            add ax,0xe06
00006BA6  FE                db 0xfe
00006BA7  1A1B              sbb bl,[bp+di]
00006BA9  7F16              jg 0x6bc1
00006BAB  1B10              sbb dx,[bx+si]
00006BAD  1118              adc [bx+si],bx
00006BAF  1900              sbb [bx+si],ax
00006BB1  0102              add [bp+si],ax
00006BB3  0304              add ax,[si]
00006BB5  050687            add ax,0x8706
00006BB8  08898A8B          or [bx+di-0x7476],cl
00006BBC  8C8D0E0F          mov word [di+0xf0e],cs
00006BC0  1011              adc [bx+di],dl
00006BC2  1213              adc dl,[bp+di]
00006BC4  1415              adc al,0x15
00006BC6  16                push ss
00006BC7  17                pop ss
00006BC8  1819              sbb [bx+di],bl
00006BCA  1A1B              sbb bl,[bp+di]
00006BCC  9C                pushf
00006BCD  9D                popf
00006BCE  9E                sahf
00006BCF  9F                lahf
00006BD0  90                nop
00006BD1  01828304          add [bp+si+0x483],ax
00006BD5  85868788          test [bp-0x7779],ax
00006BD9  898A8B8C          mov [bp+si-0x7375],cx
00006BDD  8D8E0F10          lea cx,[bp+0x100f]
00006BE1  11921394          adc [bp+si-0x6bed],dx
00006BE5  95                xchg ax,bp
00006BE6  16                push ss
00006BE7  17                pop ss
00006BE8  1819              sbb [bx+di],bl
00006BEA  1A959C9D          sbb dl,[di-0x6264]
00006BEE  9E                sahf
00006BEF  9F                lahf
00006BF0  203B              and [bp+di],bh
00006BF2  213C              and [si],di
00006BF4  223D              and bh,[di]
00006BF6  233E243F          and di,[0x3f24]
00006BFA  254026            and ax,0x2640
00006BFD  41                inc cx
00006BFE  27                daa
00006BFF  42                inc dx
00006C00  284329            sub [bp+di+0x29],al
00006C03  44                inc sp
00006C04  2A852B86          sub al,[di-0x79d5]
00006C08  2C87              sub al,0x87
00006C0A  2D882E            sub ax,0x2e88
00006C0D  892F              mov [bx],bp
00006C0F  8A30              mov dh,[bx+si]
00006C11  8B31              mov si,[bx+di]
00006C13  8C411E            mov word [bx+di+0x1e],es
00006C16  42                inc dx
00006C17  30432E            xor [bp+di+0x2e],al
00006C1A  44                inc sp
00006C1B  204512            and [di+0x12],al
00006C1E  46                inc si
00006C1F  214722            and [bx+0x22],ax
00006C22  48                dec ax
00006C23  234917            and cx,[bx+di+0x17]
00006C26  4A                dec dx
00006C27  244B              and al,0x4b
00006C29  254C26            and ax,0x264c
00006C2C  4D                dec bp
00006C2D  324E31            xor cl,[bp+0x31]
00006C30  4F                dec di
00006C31  185019            sbb [bx+si+0x19],dl
00006C34  51                push cx
00006C35  105213            adc [bp+si+0x13],dl
00006C38  53                push bx
00006C39  1F                pop ds
00006C3A  54                push sp
00006C3B  1455              adc al,0x55
00006C3D  16                push ss
00006C3E  56                push si
00006C3F  2F                das
00006C40  57                push di
00006C41  11582D            adc [bx+si+0x2d],bx
00006C44  59                pop cx
00006C45  155A2C            adc ax,0x2c5a
00006C48  80FD02            cmp ch,0x2
00006C4B  7305              jnc 0x6c52
00006C4D  7506              jnz 0x6c55
00006C4F  740B              jz 0x6c5c
00006C51  47                inc di
00006C52  0C77              or al,0x77
00006C54  0E                push cs
00006C55  4F                dec di
00006C56  10FE              adc dh,bh
00006C58  12521A            adc dl,[bp+si+0x1a]
00006C5B  761C              jna 0x6c79
00006C5D  4D                dec bp
00006C5E  1D4B1E            sbb ax,0x1e4b
00006C61  48                dec ax
00006C62  1F                pop ds
00006C63  50                push ax
00006C64  7F53              jg 0x6cb9
00006C66  FFFF              udw
00006C68  FE03              inc byte [bp+di]
00006C6A  FE0F              dec byte [bx]
00006C6C  FE49FE            dec byte [bx+di-0x2]
00006C6F  51                push cx
00006C70  FE                db 0xfe
00006C71  72FE              jc 0x6c71
00006C73  78FE              js 0x6c73
00006C75  79FE              jns 0x6c75
00006C77  7AFE              jpe 0x6c77
00006C79  7BFE              jpo 0x6c79
00006C7B  7CFE              jl 0x6c7b
00006C7D  7DFE              jnl 0x6c7d
00006C7F  7EFE              jng 0x6c7f
00006C81  7FFE              jg 0x6c81
00006C83  80FE81            cmp dh,0x81
00006C86  FE82FE83          inc byte [bp+si-0x7c02]
00006C8A  FE840000          inc byte [si+0x0]
00006C8E  BBD40B            mov bx,0xbd4
00006C91  A0A30A            mov al,[0xaa3]
00006C94  B103              mov cl,0x3
00006C96  D2E8              shr al,cl
00006C98  A2D50B            mov [0xbd5],al
00006C9B  C3                ret
00006C9C  7217              jc 0x6cb5
00006C9E  3C1F              cmp al,0x1f
00006CA0  7713              ja 0x6cb5
00006CA2  3C0D              cmp al,0xd
00006CA4  7606              jna 0x6cac
00006CA6  3C1C              cmp al,0x1c
00006CA8  720A              jc 0x6cb4
00006CAA  2C0E              sub al,0xe
00006CAC  98                cbw
00006CAD  93                xchg ax,bx
00006CAE  2E8A9F1E3C        mov bl,[cs:bx+0x3c1e]
00006CB3  93                xchg ax,bx
00006CB4  F8                clc
00006CB5  C3                ret
00006CB6  53                push bx
00006CB7  56                push si
00006CB8  9C                pushf
00006CB9  55                push bp
00006CBA  56                push si
00006CBB  57                push di
00006CBC  B401              mov ah,0x1
00006CBE  0A265D06          or ah,[0x65d]
00006CC2  CD16              int byte 0x16
00006CC4  5F                pop di
00006CC5  5E                pop si
00006CC6  5D                pop bp
00006CC7  7507              jnz 0x6cd0
00006CC9  9D                popf
00006CCA  74EC              jz 0x6cb8
00006CCC  32E4              xor ah,ah
00006CCE  EB6D              jmp 0x6d3d
00006CD0  55                push bp
00006CD1  56                push si
00006CD2  57                push di
00006CD3  B400              mov ah,0x0
00006CD5  0A265D06          or ah,[0x65d]
00006CD9  CD16              int byte 0x16
00006CDB  5F                pop di
00006CDC  5E                pop si
00006CDD  5D                pop bp
00006CDE  BE703C            mov si,0x3c70
00006CE1  FC                cld
00006CE2  0AC0              or al,al
00006CE4  750A              jnz 0x6cf0
00006CE6  0AE4              or ah,ah
00006CE8  7527              jnz 0x6d11
00006CEA  0D03FF            or ax,0xff03
00006CED  F9                stc
00006CEE  EB4C              jmp 0x6d3c
00006CF0  F6065D06FF        test byte [0x65d],0xff
00006CF5  7408              jz 0x6cff
00006CF7  3CE0              cmp al,0xe0
00006CF9  7414              jz 0x6d0f
00006CFB  3CF0              cmp al,0xf0
00006CFD  7410              jz 0x6d0f
00006CFF  B400              mov ah,0x0
00006D01  3CFD              cmp al,0xfd
00006D03  7437              jz 0x6d3c
00006D05  7714              ja 0x6d1b
00006D07  3C80              cmp al,0x80
00006D09  7530              jnz 0x6d3b
00006D0B  B0FD              mov al,0xfd
00006D0D  EB0C              jmp 0x6d1b
00006D0F  32C0              xor al,al
00006D11  86C4              xchg al,ah
00006D13  3C54              cmp al,0x54
00006D15  7204              jc 0x6d1b
00006D17  3C72              cmp al,0x72
00006D19  721C              jc 0x6d37
00006D1B  93                xchg ax,bx
00006D1C  2EAD              cs lodsw
00006D1E  85C0              test ax,ax
00006D20  74A7              jz 0x6cc9
00006D22  3AE3              cmp ah,bl
00006D24  75F6              jnz 0x6d1c
00006D26  B480              mov ah,0x80
00006D28  81FECC3C          cmp si,0x3ccc
00006D2C  720E              jc 0x6d3c
00006D2E  B4FF              mov ah,0xff
00006D30  81FEEA3C          cmp si,0x3cea
00006D34  7206              jc 0x6d3c
00006D36  93                xchg ax,bx
00006D37  BAFE00            mov dx,0xfe
00006D3A  92                xchg ax,dx
00006D3B  9E                sahf
00006D3C  5B                pop bx
00006D3D  5E                pop si
00006D3E  5B                pop bx
00006D3F  C3                ret
00006D40  3CFF              cmp al,0xff
00006D42  7511              jnz 0x6d55
00006D44  8A26A10A          mov ah,[0xaa1]
00006D48  80FC04            cmp ah,0x4
00006D4B  7204              jc 0x6d51
00006D4D  80FC07            cmp ah,0x7
00006D50  F5                cmc
00006D51  1AE4              sbb ah,ah
00006D53  EB0D              jmp 0x6d62
00006D55  3C1F              cmp al,0x1f
00006D57  7709              ja 0x6d62
00006D59  2ED7              cs xlatb
00006D5B  98                cbw
00006D5C  247F              and al,0x7f
00006D5E  80FC80            cmp ah,0x80
00006D61  F5                cmc
00006D62  C3                ret
00006D63  730D              jnc 0x6d72
00006D65  80FCFF            cmp ah,0xff
00006D68  7408              jz 0x6d72
00006D6A  80FC80            cmp ah,0x80
00006D6D  740B              jz 0x6d7a
00006D6F  F9                stc
00006D70  EB08              jmp 0x6d7a
00006D72  53                push bx
00006D73  BB303C            mov bx,0x3c30
00006D76  E8C7FF            call 0x6d40
00006D79  5B                pop bx
00006D7A  C3                ret
00006D7B  7305              jnc 0x6d82
00006D7D  3DFFFF            cmp ax,0xffff
00006D80  7508              jnz 0x6d8a
00006D82  53                push bx
00006D83  BB503C            mov bx,0x3c50
00006D86  E8B7FF            call 0x6d40
00006D89  5B                pop bx
00006D8A  C3                ret
00006D8B  7204              jc 0x6d91
00006D8D  3CFE              cmp al,0xfe
00006D8F  7513              jnz 0x6da4
00006D91  3DFFFF            cmp ax,0xffff
00006D94  740E              jz 0x6da4
00006D96  3D8080            cmp ax,0x8080
00006D99  7409              jz 0x6da4
00006D9B  3D10FF            cmp ax,0xff10
00006D9E  B0FE              mov al,0xfe
00006DA0  7402              jz 0x6da4
00006DA2  33C0              xor ax,ax
00006DA4  0BE4              or sp,sp
00006DA6  C3                ret
00006DA7  53                push bx
00006DA8  51                push cx
00006DA9  56                push si
00006DAA  720A              jc 0x6db6
00006DAC  3CFE              cmp al,0xfe
00006DAE  F8                clc
00006DAF  753E              jnz 0x6def
00006DB1  D0E0              shl al,1
00006DB3  92                xchg ax,dx
00006DB4  EB39              jmp 0x6def
00006DB6  3DFFFF            cmp ax,0xffff
00006DB9  740C              jz 0x6dc7
00006DBB  3D8080            cmp ax,0x8080
00006DBE  7407              jz 0x6dc7
00006DC0  3D10FF            cmp ax,0xff10
00006DC3  7506              jnz 0x6dcb
00006DC5  B0FE              mov al,0xfe
00006DC7  0BE4              or sp,sp
00006DC9  EB24              jmp 0x6def
00006DCB  B92E00            mov cx,0x2e
00006DCE  BE703C            mov si,0x3c70
00006DD1  80FC80            cmp ah,0x80
00006DD4  7405              jz 0x6ddb
00006DD6  B110              mov cl,0x10
00006DD8  BECA3C            mov si,0x3cca
00006DDB  93                xchg ax,bx
00006DDC  FC                cld
00006DDD  2EAD              cs lodsw
00006DDF  3AC3              cmp al,bl
00006DE1  7406              jz 0x6de9
00006DE3  E2F8              loop 0x6ddd
00006DE5  33C0              xor ax,ax
00006DE7  EB06              jmp 0x6def
00006DE9  8AC4              mov al,ah
00006DEB  32E4              xor ah,ah
00006DED  9E                sahf
00006DEE  F9                stc
00006DEF  5E                pop si
00006DF0  59                pop cx
00006DF1  5B                pop bx
00006DF2  C3                ret
00006DF3  00E8              add al,ch
00006DF5  5B                pop bx
00006DF6  F9                stc
00006DF7  8A16A30A          mov dl,[0xaa3]
00006DFB  8A36A40A          mov dh,[0xaa4]
00006DFF  8916DC08          mov [0x8dc],dx
00006E03  8A3EA20A          mov bh,[0xaa2]
00006E07  8A1EA00A          mov bl,[0xaa0]
00006E0B  891EDA08          mov [0x8da],bx
00006E0F  86D8              xchg bl,al
00006E11  86F9              xchg bh,cl
00006E13  3ADA              cmp bl,dl
00006E15  7504              jnz 0x6e1b
00006E17  3AFE              cmp bh,dh
00006E19  745F              jz 0x6e7a
00006E1B  53                push bx
00006E1C  FF16B90A          call word near [0xab9]
00006E20  59                pop cx
00006E21  7246              jc 0x6e69
00006E23  3CFF              cmp al,0xff
00006E25  7425              jz 0x6e4c
00006E27  8A26A20A          mov ah,[0xaa2]
00006E2B  3A06A00A          cmp al,[0xaa0]
00006E2F  7408              jz 0x6e39
00006E31  32E4              xor ah,ah
00006E33  3AC4              cmp al,ah
00006E35  7502              jnz 0x6e39
00006E37  FEC4              inc ah
00006E39  E83F00            call 0x6e7b
00006E3C  722B              jc 0x6e69
00006E3E  87D9              xchg bx,cx
00006E40  3A3EA40A          cmp bh,[0xaa4]
00006E44  75D5              jnz 0x6e1b
00006E46  3A1EA30A          cmp bl,[0xaa3]
00006E4A  75CF              jnz 0x6e1b
00006E4C  E8A8FA            call 0x68f7
00006E4F  FF16BB0A          call word near [0xabb]
00006E53  A0A30A            mov al,[0xaa3]
00006E56  8A0EA40A          mov cl,[0xaa4]
00006E5A  E80CFD            call 0x6b69
00006E5D  E8F3FC            call 0x6b53
00006E60  33C0              xor ax,ax
00006E62  FF16BD0A          call word near [0xabd]
00006E66  F8                clc
00006E67  EB11              jmp 0x6e7a
00006E69  8B0EDC08          mov cx,[0x8dc]
00006E6D  A1DA08            mov ax,[0x8da]
00006E70  E80800            call 0x6e7b
00006E73  87D9              xchg bx,cx
00006E75  FF16B90A          call word near [0xab9]
00006E79  F9                stc
00006E7A  C3                ret
00006E7B  51                push cx
00006E7C  8A3E8207          mov bh,[0x782]
00006E80  8A1EB10A          mov bl,[0xab1]
00006E84  8A16940A          mov dl,[0xa94]
00006E88  FF36B20A          push word [0xab2]
00006E8C  E80DF1            call 0x5f9c
00006E8F  8F06B20A          pop word [0xab2]
00006E93  8816940A          mov [0xa94],dl
00006E97  881EB10A          mov [0xab1],bl
00006E9B  883E8207          mov [0x782],bh
00006E9F  A0B20A            mov al,[0xab2]
00006EA2  9C                pushf
00006EA3  FF16E50A          call word near [0xae5]
00006EA7  9D                popf
00006EA8  59                pop cx
00006EA9  C3                ret
00006EAA  56                push si
00006EAB  E8A4F8            call 0x6752
00006EAE  8B0E920A          mov cx,[0xa92]
00006EB2  E891EF            call 0x5e46
00006EB5  8A3E950A          mov bh,[0xa95]
00006EB9  55                push bp
00006EBA  56                push si
00006EBB  57                push di
00006EBC  B408              mov ah,0x8
00006EBE  CD10              int byte 0x10
00006EC0  5F                pop di
00006EC1  5E                pop si
00006EC2  5D                pop bp
00006EC3  0AC0              or al,al
00006EC5  7502              jnz 0x6ec9
00006EC7  B020              mov al,0x20
00006EC9  33DB              xor bx,bx
00006ECB  86E3              xchg ah,bl
00006ECD  91                xchg ax,cx
00006ECE  8B166C0A          mov dx,[0xa6c]
00006ED2  E874EF            call 0x5e49
00006ED5  32ED              xor ch,ch
00006ED7  91                xchg ax,cx
00006ED8  5E                pop si
00006ED9  C3                ret
00006EDA  E875F8            call 0x6752
00006EDD  FF16C70A          call word near [0xac7]
00006EE1  C3                ret
00006EE2  0AE4              or ah,ah
00006EE4  7407              jz 0x6eed
00006EE6  3C01              cmp al,0x1
00006EE8  7731              ja 0x6f1b
00006EEA  A29C0A            mov [0xa9c],al
00006EED  0AFF              or bh,bh
00006EEF  7424              jz 0x6f15
00006EF1  80FB1F            cmp bl,0x1f
00006EF4  7725              ja 0x6f1b
00006EF6  881E110B          mov [0xb11],bl
00006EFA  0AED              or ch,ch
00006EFC  740F              jz 0x6f0d
00006EFE  80F91F            cmp cl,0x1f
00006F01  7718              ja 0x6f1b
00006F03  880E0E0B          mov [0xb0e],cl
00006F07  880E100B          mov [0xb10],cl
00006F0B  EB0C              jmp 0x6f19
00006F0D  881E0E0B          mov [0xb0e],bl
00006F11  881E100B          mov [0xb10],bl
00006F15  0AED              or ch,ch
00006F17  7502              jnz 0x6f1b
00006F19  F8                clc
00006F1A  C3                ret
00006F1B  F9                stc
00006F1C  C3                ret
00006F1D  00509A            add [bx+si-0x66],dl
00006F20  52                push dx
00006F21  03E5              add sp,bp
00006F23  07                pop es
00006F24  C3                ret
00006F25  00558B            add [di-0x75],dl
00006F28  EC                in al,dx
00006F29  E85BEB            call 0x5a87
00006F2C  A1480D            mov ax,[0xd48]
00006F2F  E82FEB            call 0x5a61
00006F32  8B6E04            mov bp,[bp+0x4]
00006F35  813E480D0098      cmp word [0xd48],0x9800
00006F3B  7303              jnc 0x6f40
00006F3D  E89AE7            call 0x56da
00006F40  E844EB            call 0x5a87
00006F43  E966F6            jmp 0x65ac
00006F46  E80100            call 0x6f4a
00006F49  CB                retf
00006F4A  50                push ax
00006F4B  B00D              mov al,0xd
00006F4D  E80200            call 0x6f52
00006F50  58                pop ax
00006F51  C3                ret
00006F52  C7064D0D0000      mov word [0xd4d],0x0
00006F58  E914E4            jmp 0x536f
00006F5B  0AE4              or ah,ah
00006F5D  7502              jnz 0x6f61
00006F5F  B410              mov ah,0x10
00006F61  50                push ax
00006F62  9A5203E507        call word 0x7e5:word 0x352
00006F67  06                push es
00006F68  C7064D0D0000      mov word [0xd4d],0x0
00006F6E  8EC2              mov es,dx
00006F70  93                xchg ax,bx
00006F71  0BD2              or dx,dx
00006F73  7505              jnz 0x6f7a
00006F75  EB0B              jmp 0x6f82
00006F77  E8F5E3            call 0x536f
00006F7A  268A07            mov al,[es:bx]
00006F7D  43                inc bx
00006F7E  0AC0              or al,al
00006F80  75F5              jnz 0x6f77
00006F82  07                pop es
00006F83  C3                ret
00006F84  8B0F              mov cx,[bx]
00006F86  E33F              jcxz 0x6fc7
00006F88  C7064D0D0000      mov word [0xd4d],0x0
00006F8E  8B7702            mov si,[bx+0x2]
00006F91  F606240B26        test byte [0xb24],0x26
00006F96  7529              jnz 0x6fc1
00006F98  A06D0A            mov al,[0xa6d]
00006F9B  98                cbw
00006F9C  48                dec ax
00006F9D  03C1              add ax,cx
00006F9F  0AE4              or ah,ah
00006FA1  751E              jnz 0x6fc1
00006FA3  86F0              xchg dh,al
00006FA5  E81FE5            call 0x54c7
00006FA8  7717              ja 0x6fc1
00006FAA  51                push cx
00006FAB  56                push si
00006FAC  AC                lodsb
00006FAD  3C1F              cmp al,0x1f
00006FAF  760E              jna 0x6fbf
00006FB1  E2F9              loop 0x6fac
00006FB3  5E                pop si
00006FB4  59                pop cx
00006FB5  8B166C0A          mov dx,[0xa6c]
00006FB9  E80BFA            call 0x69c7
00006FBC  E91AE5            jmp 0x54d9
00006FBF  5E                pop si
00006FC0  59                pop cx
00006FC1  AC                lodsb
00006FC2  E8AAE3            call 0x536f
00006FC5  E2FA              loop 0x6fc1
00006FC7  C3                ret
00006FC8  50                push ax
00006FC9  E87EFF            call 0x6f4a
00006FCC  A0240B            mov al,[0xb24]
00006FCF  2403              and al,0x3
00006FD1  3C02              cmp al,0x2
00006FD3  750B              jnz 0x6fe0
00006FD5  3006240B          xor [0xb24],al
00006FD9  E86EFF            call 0x6f4a
00006FDC  0806240B          or [0xb24],al
00006FE0  58                pop ax
00006FE1  C3                ret
00006FE2  8B07              mov ax,[bx]
00006FE4  99                cwd
00006FE5  803E340D02        cmp byte [0xd34],0x2
00006FEA  740A              jz 0x6ff6
00006FEC  803E340D14        cmp byte [0xd34],0x14
00006FF1  7540              jnz 0x7033
00006FF3  8B5702            mov dx,[bx+0x2]
00006FF6  33DB              xor bx,bx
00006FF8  92                xchg ax,dx
00006FF9  B120              mov cl,0x20
00006FFB  23C0              and ax,ax
00006FFD  7D08              jnl 0x7007
00006FFF  B12D              mov cl,0x2d
00007001  F7DA              neg dx
00007003  13C3              adc ax,bx
00007005  F7D8              neg ax
00007007  56                push si
00007008  51                push cx
00007009  BE0109            mov si,0x901
0000700C  881C              mov [si],bl
0000700E  56                push si
0000700F  B90A00            mov cx,0xa
00007012  52                push dx
00007013  33D2              xor dx,dx
00007015  F7F1              div cx
00007017  5B                pop bx
00007018  93                xchg ax,bx
00007019  F7F1              div cx
0000701B  92                xchg ax,dx
0000701C  0430              add al,0x30
0000701E  4E                dec si
0000701F  8804              mov [si],al
00007021  8BC3              mov ax,bx
00007023  0BDA              or bx,dx
00007025  75EB              jnz 0x7012
00007027  58                pop ax
00007028  2BC6              sub ax,si
0000702A  59                pop cx
0000702B  4E                dec si
0000702C  880C              mov [si],cl
0000702E  8BDE              mov bx,si
00007030  40                inc ax
00007031  5E                pop si
00007032  C3                ret
00007033  FF26280B          jmp word near [0xb28]
00007037  06                push es
00007038  1E                push ds
00007039  07                pop es
0000703A  8BFE              mov di,si
0000703C  98                cbw
0000703D  03F9              add di,cx
0000703F  3BC1              cmp ax,cx
00007041  7321              jnc 0x7064
00007043  91                xchg ax,cx
00007044  2BC1              sub ax,cx
00007046  03D0              add dx,ax
00007048  2BF8              sub di,ax
0000704A  B030              mov al,0x30
0000704C  8605              xchg al,[di]
0000704E  3C35              cmp al,0x35
00007050  7212              jc 0x7064
00007052  E30C              jcxz 0x7060
00007054  4F                dec di
00007055  8A05              mov al,[di]
00007057  FEC0              inc al
00007059  3C3A              cmp al,0x3a
0000705B  7206              jc 0x7063
0000705D  42                inc dx
0000705E  E2F4              loop 0x7054
00007060  41                inc cx
00007061  B031              mov al,0x31
00007063  AA                stosb
00007064  4F                dec di
00007065  8AE1              mov ah,cl
00007067  B030              mov al,0x30
00007069  FD                std
0000706A  F3AE              repe scasb
0000706C  FC                cld
0000706D  41                inc cx
0000706E  2AE1              sub ah,cl
00007070  02D4              add dl,ah
00007072  80D600            adc dh,0x0
00007075  BFDF08            mov di,0x8df
00007078  07                pop es
00007079  C3                ret
0000707A  B80200            mov ax,0x2
0000707D  EB26              jmp 0x70a5
0000707F  B80201            mov ax,0x102
00007082  EB21              jmp 0x70a5
00007084  B80202            mov ax,0x202
00007087  EB1C              jmp 0x70a5
00007089  B81400            mov ax,0x14
0000708C  EB17              jmp 0x70a5
0000708E  B81401            mov ax,0x114
00007091  EB12              jmp 0x70a5
00007093  B81402            mov ax,0x214
00007096  EB0D              jmp 0x70a5
00007098  B80300            mov ax,0x3
0000709B  EB08              jmp 0x70a5
0000709D  B80301            mov ax,0x103
000070A0  EB03              jmp 0x70a5
000070A2  B80302            mov ax,0x203
000070A5  55                push bp
000070A6  8BEC              mov bp,sp
000070A8  56                push si
000070A9  A3340D            mov [0xd34],ax
000070AC  8D5E06            lea bx,[bp+0x6]
000070AF  3C03              cmp al,0x3
000070B1  7502              jnz 0x70b5
000070B3  8B1F              mov bx,[bx]
000070B5  F6068A0A02        test byte [0xa8a],0x2
000070BA  754B              jnz 0x7107
000070BC  3C03              cmp al,0x3
000070BE  744D              jz 0x710d
000070C0  E81FFF            call 0x6fe2
000070C3  8BF3              mov si,bx
000070C5  F6068A0A04        test byte [0xa8a],0x4
000070CA  752F              jnz 0x70fb
000070CC  03D8              add bx,ax
000070CE  40                inc ax
000070CF  C60720            mov byte [bx],0x20
000070D2  E8C500            call 0x719a
000070D5  53                push bx
000070D6  FF163A0B          call word near [0xb3a]
000070DA  5B                pop bx
000070DB  53                push bx
000070DC  FF16340B          call word near [0xb34]
000070E0  5B                pop bx
000070E1  FF164607          call word near [0x746]
000070E5  A0350D            mov al,[0xd35]
000070E8  98                cbw
000070E9  48                dec ax
000070EA  7F42              jg 0x712e
000070EC  F6068A0A04        test byte [0xa8a],0x4
000070F1  7444              jz 0x7137
000070F3  B02C              mov al,0x2c
000070F5  FF16320B          call word near [0xb32]
000070F9  EB5D              jmp 0x7158
000070FB  803F20            cmp byte [bx],0x20
000070FE  7502              jnz 0x7102
00007100  46                inc si
00007101  48                dec ax
00007102  E89500            call 0x719a
00007105  EBD4              jmp 0x70db
00007107  FF160209          call word near [0x902]
0000710B  EBD8              jmp 0x70e5
0000710D  8B07              mov ax,[bx]
0000710F  F6068A0A04        test byte [0xa8a],0x4
00007114  74BF              jz 0x70d5
00007116  B022              mov al,0x22
00007118  53                push bx
00007119  FF16320B          call word near [0xb32]
0000711D  5B                pop bx
0000711E  FF16340B          call word near [0xb34]
00007122  FF164607          call word near [0x746]
00007126  B022              mov al,0x22
00007128  FF16320B          call word near [0xb32]
0000712C  EBB7              jmp 0x70e5
0000712E  FF16300B          call word near [0xb30]
00007132  E86E00            call 0x71a3
00007135  EB21              jmp 0x7158
00007137  40                inc ax
00007138  751E              jnz 0x7158
0000713A  FF162C0B          call word near [0xb2c]
0000713E  8AC4              mov al,ah
00007140  32E4              xor ah,ah
00007142  B10E              mov cl,0xe
00007144  F6F1              div cl
00007146  2ACC              sub cl,ah
00007148  91                xchg ax,cx
00007149  98                cbw
0000714A  50                push ax
0000714B  050E00            add ax,0xe
0000714E  FF163A0B          call word near [0xb3a]
00007152  59                pop cx
00007153  7203              jc 0x7158
00007155  E8C400            call 0x721c
00007158  5E                pop si
00007159  5D                pop bp
0000715A  59                pop cx
0000715B  5A                pop dx
0000715C  5B                pop bx
0000715D  A0340D            mov al,[0xd34]
00007160  A803              test al,0x3
00007162  7507              jnz 0x716b
00007164  5B                pop bx
00007165  A808              test al,0x8
00007167  7402              jz 0x716b
00007169  5B                pop bx
0000716A  5B                pop bx
0000716B  52                push dx
0000716C  51                push cx
0000716D  CB                retf
0000716E  92                xchg ax,dx
0000716F  FF162E0B          call word near [0xb2e]
00007173  80FCFF            cmp ah,0xff
00007176  7421              jz 0x7199
00007178  8AC4              mov al,ah
0000717A  EB04              jmp 0x7180
0000717C  92                xchg ax,dx
0000717D  A0800A            mov al,[0xa80]
00007180  FF162C0B          call word near [0xb2c]
00007184  0AF6              or dh,dh
00007186  7508              jnz 0x7190
00007188  2AC4              sub al,ah
0000718A  7204              jc 0x7190
0000718C  3AC2              cmp al,dl
0000718E  7309              jnc 0x7199
00007190  0AE4              or ah,ah
00007192  7404              jz 0x7198
00007194  FF16300B          call word near [0xb30]
00007198  F9                stc
00007199  C3                ret
0000719A  BB200B            mov bx,0xb20
0000719D  8907              mov [bx],ax
0000719F  897702            mov [bx+0x2],si
000071A2  C3                ret
000071A3  56                push si
000071A4  8B364D0D          mov si,[0xd4d]
000071A8  0BF6              or si,si
000071AA  7416              jz 0x71c2
000071AC  C7064D0D0000      mov word [0xd4d],0x0
000071B2  81FE360D          cmp si,0xd36
000071B6  740A              jz 0x71c2
000071B8  F6440580          test byte [si+0x5],0x80
000071BC  7404              jz 0x71c2
000071BE  FF16410B          call word near [0xb41]
000071C2  F6068A0A0D        test byte [0xa8a],0xd
000071C7  C6068A0A00        mov byte [0xa8a],0x0
000071CC  7406              jz 0x71d4
000071CE  BED80B            mov si,0xbd8
000071D1  E83900            call 0x720d
000071D4  5E                pop si
000071D5  C3                ret
000071D6  55                push bp
000071D7  8BEC              mov bp,sp
000071D9  06                push es
000071DA  A03C0B            mov al,[0xb3c]
000071DD  0AC0              or al,al
000071DF  7824              js 0x7205
000071E1  751D              jnz 0x7200
000071E3  56                push si
000071E4  57                push di
000071E5  8B3E0409          mov di,[0x904]
000071E9  4F                dec di
000071EA  4F                dec di
000071EB  8BF4              mov si,sp
000071ED  83C60A            add si,0xa
000071F0  B90300            mov cx,0x3
000071F3  FD                std
000071F4  1E                push ds
000071F5  07                pop es
000071F6  F3A5              rep movsw
000071F8  FC                cld
000071F9  8BEF              mov bp,di
000071FB  83C502            add bp,0x2
000071FE  5F                pop di
000071FF  5E                pop si
00007200  E83400            call 0x7237
00007203  EB03              jmp 0x7208
00007205  E89BFF            call 0x71a3
00007208  07                pop es
00007209  8BE5              mov sp,bp
0000720B  5D                pop bp
0000720C  CB                retf
0000720D  57                push di
0000720E  06                push es
0000720F  1E                push ds
00007210  07                pop es
00007211  BF2C0B            mov di,0xb2c
00007214  B90800            mov cx,0x8
00007217  F3A5              rep movsw
00007219  07                pop es
0000721A  5F                pop di
0000721B  C3                ret
0000721C  0BC9              or cx,cx
0000721E  7E15              jng 0x7235
00007220  51                push cx
00007221  16                push ss
00007222  07                pop es
00007223  BFC707            mov di,0x7c7
00007226  57                push di
00007227  41                inc cx
00007228  D1E9              shr cx,1
0000722A  B82020            mov ax,0x2020
0000722D  F3AB              rep stosw
0000722F  5E                pop si
00007230  59                pop cx
00007231  FF16380B          call word near [0xb38]
00007235  C3                ret
00007236  C3                ret
00007237  C6063C0BFF        mov byte [0xb3c],0xff
0000723C  50                push ax
0000723D  A13F0B            mov ax,[0xb3f]
00007240  A33D0B            mov [0xb3d],ax
00007243  58                pop ax
00007244  C7064D0D0000      mov word [0xd4d],0x0
0000724A  C3                ret
0000724B  55                push bp
0000724C  8BEC              mov bp,sp
0000724E  800E8A0A04        or byte [0xa8a],0x4
00007253  E82CEA            call 0x5c82
00007256  5D                pop bp
00007257  CB                retf
00007258  B601              mov dh,0x1
0000725A  8A166E0A          mov dl,[0xa6e]
0000725E  89166C0A          mov [0xa6c],dx
00007262  C3                ret
00007263  50                push ax
00007264  53                push bx
00007265  51                push cx
00007266  52                push dx
00007267  56                push si
00007268  800E240B08        or byte [0xb24],0x8
0000726D  FF366C0A          push word [0xa6c]
00007271  E8E4FF            call 0x7258
00007274  A02707            mov al,[0x727]
00007277  0AC0              or al,al
00007279  7512              jnz 0x728d
0000727B  E8EFF7            call 0x6a6d
0000727E  5A                pop dx
0000727F  E898EB            call 0x5e1a
00007282  8026240BF7        and byte [0xb24],0xf7
00007287  5E                pop si
00007288  5A                pop dx
00007289  59                pop cx
0000728A  5B                pop bx
0000728B  58                pop ax
0000728C  C3                ret
0000728D  E8B6EB            call 0x5e46
00007290  E86600            call 0x72f9
00007293  50                push ax
00007294  80FC30            cmp ah,0x30
00007297  7407              jz 0x72a0
00007299  86E0              xchg ah,al
0000729B  E84500            call 0x72e3
0000729E  86E0              xchg ah,al
000072A0  E84000            call 0x72e3
000072A3  56                push si
000072A4  AD                lodsw
000072A5  93                xchg ax,bx
000072A6  AD                lodsw
000072A7  96                xchg ax,si
000072A8  8A0E2807          mov cl,[0x728]
000072AC  0ADB              or bl,bl
000072AE  7403              jz 0x72b3
000072B0  E8A900            call 0x735c
000072B3  0BDB              or bx,bx
000072B5  7F04              jg 0x72bb
000072B7  32C0              xor al,al
000072B9  EB07              jmp 0x72c2
000072BB  AC                lodsb
000072BC  3C0D              cmp al,0xd
000072BE  7502              jnz 0x72c2
000072C0  B01B              mov al,0x1b
000072C2  E81E00            call 0x72e3
000072C5  4B                dec bx
000072C6  FEC9              dec cl
000072C8  75E9              jnz 0x72b3
000072CA  021E2807          add bl,[0x728]
000072CE  7403              jz 0x72d3
000072D0  E88900            call 0x735c
000072D3  32C0              xor al,al
000072D5  E80B00            call 0x72e3
000072D8  5E                pop si
000072D9  58                pop ax
000072DA  E85700            call 0x7334
000072DD  FECD              dec ch
000072DF  75B2              jnz 0x7293
000072E1  EB9B              jmp 0x727e
000072E3  50                push ax
000072E4  53                push bx
000072E5  0AC0              or al,al
000072E7  7502              jnz 0x72eb
000072E9  B020              mov al,0x20
000072EB  3C0A              cmp al,0xa
000072ED  7502              jnz 0x72f1
000072EF  B03C              mov al,0x3c
000072F1  51                push cx
000072F2  E87AE0            call 0x536f
000072F5  59                pop cx
000072F6  5B                pop bx
000072F7  58                pop ax
000072F8  C3                ret
000072F9  53                push bx
000072FA  E891F9            call 0x6c8e
000072FD  8B0F              mov cx,[bx]
000072FF  880E2807          mov [0x728],cl
00007303  51                push cx
00007304  BE0000            mov si,0x0
00007307  8A4702            mov al,[bx+0x2]
0000730A  98                cbw
0000730B  50                push ax
0000730C  48                dec ax
0000730D  D1E0              shl ax,1
0000730F  D1E0              shl ax,1
00007311  03F0              add si,ax
00007313  58                pop ax
00007314  E80F00            call 0x7326
00007317  80FC30            cmp ah,0x30
0000731A  7404              jz 0x7320
0000731C  FE0E2807          dec byte [0x728]
00007320  E82700            call 0x734a
00007323  59                pop cx
00007324  5B                pop bx
00007325  C3                ret
00007326  51                push cx
00007327  32E4              xor ah,ah
00007329  B10A              mov cl,0xa
0000732B  F6F1              div cl
0000732D  053030            add ax,0x3030
00007330  86E0              xchg ah,al
00007332  59                pop cx
00007333  C3                ret
00007334  83C604            add si,0x4
00007337  FEC0              inc al
00007339  3C39              cmp al,0x39
0000733B  7E0D              jng 0x734a
0000733D  B030              mov al,0x30
0000733F  FEC4              inc ah
00007341  80FC31            cmp ah,0x31
00007344  7504              jnz 0x734a
00007346  FE0E2807          dec byte [0x728]
0000734A  81FE3000          cmp si,0x30
0000734E  760B              jna 0x735b
00007350  BE0000            mov si,0x0
00007353  B430              mov ah,0x30
00007355  B031              mov al,0x31
00007357  FE062807          inc byte [0x728]
0000735B  C3                ret
0000735C  50                push ax
0000735D  53                push bx
0000735E  F8                clc
0000735F  E8B0F7            call 0x6b12
00007362  93                xchg ax,bx
00007363  E8B4F7            call 0x6b1a
00007366  5B                pop bx
00007367  58                pop ax
00007368  C3                ret
00007369  0002              add [bp+si],al
0000736B  54                push sp
0000736C  47                inc di
0000736D  03A74505          add sp,[bx+0x545]
00007371  9F                lahf
00007372  46                inc si
00007373  06                push es
00007374  29470A            sub [bx+0xa],ax
00007377  82                db 0x82
00007378  45                inc bp
00007379  0BB5460D          or si,[di+0xd46]
0000737D  83450EC4          add word [di+0xe],0xffffffffffffffc4
00007381  46                inc si
00007382  15D546            adc ax,0x46d5
00007385  1C05              sbb al,0x5
00007387  47                inc di
00007388  1DF046            sbb ax,0x46f0
0000738B  085746            or [bx+0x46],dl
0000738E  096345            or [bp+di+0x45],sp
00007391  127C45            adc bh,[si+0x45]
00007394  1404              adc al,0x4
00007396  267F41            es jg 0x73da
00007399  46                inc si
0000739A  50                push ax
0000739B  53                push bx
0000739C  51                push cx
0000739D  52                push dx
0000739E  06                push es
0000739F  1E                push ds
000073A0  07                pop es
000073A1  33DB              xor bx,bx
000073A3  891E0609          mov [0x906],bx
000073A7  891E0809          mov [0x908],bx
000073AB  881E1009          mov [0x910],bl
000073AF  881E1109          mov [0x911],bl
000073B3  881EC707          mov [0x7c7],bl
000073B7  800E240B30        or byte [0xb24],0x30
000073BC  E8F903            call 0x77b8
000073BF  E89200            call 0x7454
000073C2  E8DF00            call 0x74a4
000073C5  E84200            call 0x740a
000073C8  7505              jnz 0x73cf
000073CA  E8B300            call 0x7480
000073CD  EBF3              jmp 0x73c2
000073CF  7205              jc 0x73d6
000073D1  E8C200            call 0x7496
000073D4  EBEC              jmp 0x73c2
000073D6  80FC80            cmp ah,0x80
000073D9  7509              jnz 0x73e4
000073DB  3C80              cmp al,0x80
000073DD  74F2              jz 0x73d1
000073DF  E8A500            call 0x7487
000073E2  EBDE              jmp 0x73c2
000073E4  80FCFF            cmp ah,0xff
000073E7  75D9              jnz 0x73c2
000073E9  3C10              cmp al,0x10
000073EB  7504              jnz 0x73f1
000073ED  B0FE              mov al,0xfe
000073EF  EBE0              jmp 0x73d1
000073F1  3CFF              cmp al,0xff
000073F3  74DC              jz 0x73d1
000073F5  E8BE00            call 0x74b6
000073F8  F6061109FF        test byte [0x911],0xff
000073FD  7502              jnz 0x7401
000073FF  EBC1              jmp 0x73c2
00007401  E86200            call 0x7466
00007404  07                pop es
00007405  5A                pop dx
00007406  59                pop cx
00007407  5B                pop bx
00007408  58                pop ax
00007409  C3                ret
0000740A  E83E00            call 0x744b
0000740D  F606240B01        test byte [0xb24],0x1
00007412  7410              jz 0x7424
00007414  E8B3F6            call 0x6aca
00007417  7510              jnz 0x7429
00007419  8026240BCF        and byte [0xb24],0xcf
0000741E  E82302            call 0x7644
00007421  E923E5            jmp 0x5947
00007424  B201              mov dl,0x1
00007426  E8D6E7            call 0x5bff
00007429  E84FF9            call 0x6d7b
0000742C  9C                pushf
0000742D  E82400            call 0x7454
00007430  3CFE              cmp al,0xfe
00007432  7504              jnz 0x7438
00007434  9D                popf
00007435  33C0              xor ax,ax
00007437  C3                ret
00007438  9D                popf
00007439  C3                ret
0000743A  51                push cx
0000743B  56                push si
0000743C  8BCB              mov cx,bx
0000743E  E308              jcxz 0x7448
00007440  BEC908            mov si,0x8c9
00007443  E8EC00            call 0x7532
00007446  33DB              xor bx,bx
00007448  5E                pop si
00007449  59                pop cx
0000744A  C3                ret
0000744B  F6061009FF        test byte [0x910],0xff
00007450  7508              jnz 0x745a
00007452  EB0C              jmp 0x7460
00007454  50                push ax
00007455  B8C62E            mov ax,0x2ec6
00007458  EB10              jmp 0x746a
0000745A  50                push ax
0000745B  B8B12E            mov ax,0x2eb1
0000745E  EB0A              jmp 0x746a
00007460  50                push ax
00007461  B8AC2E            mov ax,0x2eac
00007464  EB04              jmp 0x746a
00007466  50                push ax
00007467  B8B62E            mov ax,0x2eb6
0000746A  52                push dx
0000746B  8A16240B          mov dl,[0xb24]
0000746F  80E203            and dl,0x3
00007472  80FA03            cmp dl,0x3
00007475  7406              jz 0x747d
00007477  8B166C0A          mov dx,[0xa6c]
0000747B  FFD0              call ax
0000747D  5A                pop dx
0000747E  58                pop ax
0000747F  C3                ret
00007480  E8B7FF            call 0x743a
00007483  E82E03            call 0x77b4
00007486  C3                ret
00007487  50                push ax
00007488  E8AFFF            call 0x743a
0000748B  2C20              sub al,0x20
0000748D  3C0C              cmp al,0xc
0000748F  7C03              jl 0x7494
00007491  E82003            call 0x77b4
00007494  58                pop ax
00007495  C3                ret
00007496  83FB10            cmp bx,0x10
00007499  7C03              jl 0x749e
0000749B  E89CFF            call 0x743a
0000749E  8887C908          mov [bx+0x8c9],al
000074A2  43                inc bx
000074A3  C3                ret
000074A4  50                push ax
000074A5  F606240B01        test byte [0xb24],0x1
000074AA  7508              jnz 0x74b4
000074AC  E831E7            call 0x5be0
000074AF  7503              jnz 0x74b4
000074B1  E886FF            call 0x743a
000074B4  58                pop ax
000074B5  C3                ret
000074B6  56                push si
000074B7  E880FF            call 0x743a
000074BA  BEEA43            mov si,0x43ea
000074BD  81FE1A44          cmp si,0x441a
000074C1  741B              jz 0x74de
000074C3  2E3804            cmp [cs:si],al
000074C6  7405              jz 0x74cd
000074C8  83C603            add si,0x3
000074CB  EBF0              jmp 0x74bd
000074CD  81FE0B44          cmp si,0x440b
000074D1  7305              jnc 0x74d8
000074D3  C606100900        mov byte [0x910],0x0
000074D8  2EFF5401          call word near [cs:si+0x1]
000074DC  EB03              jmp 0x74e1
000074DE  E8D302            call 0x77b4
000074E1  5E                pop si
000074E2  C3                ret
000074E3  53                push bx
000074E4  51                push cx
000074E5  56                push si
000074E6  8B1E0609          mov bx,[0x906]
000074EA  83E307            and bx,0x7
000074ED  B90800            mov cx,0x8
000074F0  2BCB              sub cx,bx
000074F2  BEE80B            mov si,0xbe8
000074F5  E83A00            call 0x7532
000074F8  5E                pop si
000074F9  59                pop cx
000074FA  5B                pop bx
000074FB  C3                ret
000074FC  80361009FF        xor byte [0x910],0xff
00007501  C3                ret
00007502  C3                ret
00007503  8026240BEF        and byte [0xb24],0xef
00007508  FF366C0A          push word [0xa6c]
0000750C  F606240B06        test byte [0xb24],0x6
00007511  7403              jz 0x7516
00007513  E81F01            call 0x7635
00007516  E82B01            call 0x7644
00007519  FE0E1109          dec byte [0x911]
0000751D  8F066C0A          pop word [0xa6c]
00007521  8026240BDF        and byte [0xb24],0xdf
00007526  C3                ret
00007527  8026240BCF        and byte [0xb24],0xcf
0000752C  E81501            call 0x7644
0000752F  E9A9F0            jmp 0x65db
00007532  53                push bx
00007533  52                push dx
00007534  E8E701            call 0x771e
00007537  8BD1              mov dx,cx
00007539  8B1E0609          mov bx,[0x906]
0000753D  F6061009FF        test byte [0x910],0xff
00007542  7407              jz 0x754b
00007544  E82900            call 0x7570
00007547  7221              jc 0x756a
00007549  EB17              jmp 0x7562
0000754B  2B160809          sub dx,[0x908]
0000754F  03D3              add dx,bx
00007551  0BD2              or dx,dx
00007553  7E0D              jng 0x7562
00007555  8B1E0809          mov bx,[0x908]
00007559  E81400            call 0x7570
0000755C  720C              jc 0x756a
0000755E  8B1E0609          mov bx,[0x906]
00007562  E84B00            call 0x75b0
00007565  E8CD01            call 0x7735
00007568  EB03              jmp 0x756d
0000756A  E84702            call 0x77b4
0000756D  5A                pop dx
0000756E  5B                pop bx
0000756F  C3                ret
00007570  50                push ax
00007571  51                push cx
00007572  56                push si
00007573  57                push di
00007574  8B0E0809          mov cx,[0x908]
00007578  03CA              add cx,dx
0000757A  81F9FF00          cmp cx,0xff
0000757E  7603              jna 0x7583
00007580  F9                stc
00007581  EB28              jmp 0x75ab
00007583  53                push bx
00007584  8B0E0809          mov cx,[0x908]
00007588  2BCB              sub cx,bx
0000758A  41                inc cx
0000758B  8B1E0809          mov bx,[0x908]
0000758F  FD                std
00007590  8DB7C707          lea si,[bx+0x7c7]
00007594  8BFE              mov di,si
00007596  03FA              add di,dx
00007598  F3A4              rep movsb
0000759A  FC                cld
0000759B  5B                pop bx
0000759C  B020              mov al,0x20
0000759E  8DBFC707          lea di,[bx+0x7c7]
000075A2  8BCA              mov cx,dx
000075A4  F3AA              rep stosb
000075A6  01160809          add [0x908],dx
000075AA  F8                clc
000075AB  5F                pop di
000075AC  5E                pop si
000075AD  59                pop cx
000075AE  58                pop ax
000075AF  C3                ret
000075B0  56                push si
000075B1  57                push di
000075B2  51                push cx
000075B3  8DBFC707          lea di,[bx+0x7c7]
000075B7  F3A4              rep movsb
000075B9  59                pop cx
000075BA  010E0609          add [0x906],cx
000075BE  5F                pop di
000075BF  5E                pop si
000075C0  C3                ret
000075C1  53                push bx
000075C2  E85901            call 0x771e
000075C5  8B1E0609          mov bx,[0x906]
000075C9  3B1E0809          cmp bx,[0x908]
000075CD  7406              jz 0x75d5
000075CF  E82600            call 0x75f8
000075D2  E86001            call 0x7735
000075D5  5B                pop bx
000075D6  C3                ret
000075D7  53                push bx
000075D8  E84301            call 0x771e
000075DB  8B1E0609          mov bx,[0x906]
000075DF  0BDB              or bx,bx
000075E1  7407              jz 0x75ea
000075E3  4B                dec bx
000075E4  FF0E0609          dec word [0x906]
000075E8  EB06              jmp 0x75f0
000075EA  3B1E0809          cmp bx,[0x908]
000075EE  7406              jz 0x75f6
000075F0  E80500            call 0x75f8
000075F3  E83F01            call 0x7735
000075F6  5B                pop bx
000075F7  C3                ret
000075F8  51                push cx
000075F9  52                push dx
000075FA  56                push si
000075FB  57                push di
000075FC  891E0A09          mov [0x90a],bx
00007600  BA0100            mov dx,0x1
00007603  8B0E0809          mov cx,[0x908]
00007607  2BCB              sub cx,bx
00007609  41                inc cx
0000760A  2BCA              sub cx,dx
0000760C  8DBFC707          lea di,[bx+0x7c7]
00007610  8BF7              mov si,di
00007612  03F2              add si,dx
00007614  F3A4              rep movsb
00007616  29160809          sub [0x908],dx
0000761A  5F                pop di
0000761B  5E                pop si
0000761C  5A                pop dx
0000761D  59                pop cx
0000761E  C3                ret
0000761F  53                push bx
00007620  E8FB00            call 0x771e
00007623  8B1E0609          mov bx,[0x906]
00007627  891E0809          mov [0x908],bx
0000762B  C687C70700        mov byte [bx+0x7c7],0x0
00007630  E80201            call 0x7735
00007633  5B                pop bx
00007634  C3                ret
00007635  53                push bx
00007636  E8E500            call 0x771e
00007639  33DB              xor bx,bx
0000763B  891E0609          mov [0x906],bx
0000763F  E8F300            call 0x7735
00007642  5B                pop bx
00007643  C3                ret
00007644  53                push bx
00007645  E8D600            call 0x771e
00007648  8B1E0809          mov bx,[0x908]
0000764C  891E0609          mov [0x906],bx
00007650  E8E200            call 0x7735
00007653  5B                pop bx
00007654  C3                ret
00007655  53                push bx
00007656  E8C500            call 0x771e
00007659  33DB              xor bx,bx
0000765B  891E0609          mov [0x906],bx
0000765F  891E0A09          mov [0x90a],bx
00007663  891E0809          mov [0x908],bx
00007667  E8CB00            call 0x7735
0000766A  881EC707          mov [0x7c7],bl
0000766E  5B                pop bx
0000766F  C3                ret
00007670  50                push ax
00007671  53                push bx
00007672  8B1E0609          mov bx,[0x906]
00007676  0BDB              or bx,bx
00007678  7408              jz 0x7682
0000767A  4B                dec bx
0000767B  E81801            call 0x7796
0000767E  FF0E0609          dec word [0x906]
00007682  5B                pop bx
00007683  58                pop ax
00007684  C3                ret
00007685  50                push ax
00007686  53                push bx
00007687  8B1E0609          mov bx,[0x906]
0000768B  3B1E0809          cmp bx,[0x908]
0000768F  7507              jnz 0x7698
00007691  5B                pop bx
00007692  58                pop ax
00007693  B020              mov al,0x20
00007695  E9FEFD            jmp 0x7496
00007698  8A87C707          mov al,[bx+0x7c7]
0000769C  E8D0DC            call 0x536f
0000769F  FF060609          inc word [0x906]
000076A3  E81201            call 0x77b8
000076A6  5B                pop bx
000076A7  58                pop ax
000076A8  C3                ret
000076A9  53                push bx
000076AA  8B1E0609          mov bx,[0x906]
000076AE  4B                dec bx
000076AF  43                inc bx
000076B0  3B1E0809          cmp bx,[0x908]
000076B4  7411              jz 0x76c7
000076B6  E84200            call 0x76fb
000076B9  72F4              jc 0x76af
000076BB  3B1E0809          cmp bx,[0x908]
000076BF  7406              jz 0x76c7
000076C1  43                inc bx
000076C2  E83600            call 0x76fb
000076C5  73F4              jnc 0x76bb
000076C7  3B1E0609          cmp bx,[0x906]
000076CB  7405              jz 0x76d2
000076CD  E8B5FF            call 0x7685
000076D0  EBF5              jmp 0x76c7
000076D2  5B                pop bx
000076D3  C3                ret
000076D4  53                push bx
000076D5  8B1E0609          mov bx,[0x906]
000076D9  0BDB              or bx,bx
000076DB  7411              jz 0x76ee
000076DD  4B                dec bx
000076DE  E81A00            call 0x76fb
000076E1  73F6              jnc 0x76d9
000076E3  0BDB              or bx,bx
000076E5  7407              jz 0x76ee
000076E7  4B                dec bx
000076E8  E81000            call 0x76fb
000076EB  72F6              jc 0x76e3
000076ED  43                inc bx
000076EE  3B1E0609          cmp bx,[0x906]
000076F2  7405              jz 0x76f9
000076F4  E879FF            call 0x7670
000076F7  EBF5              jmp 0x76ee
000076F9  5B                pop bx
000076FA  C3                ret
000076FB  50                push ax
000076FC  8A87C707          mov al,[bx+0x7c7]
00007700  3C30              cmp al,0x30
00007702  7214              jc 0x7718
00007704  3C39              cmp al,0x39
00007706  7613              jna 0x771b
00007708  3C41              cmp al,0x41
0000770A  720C              jc 0x7718
0000770C  3C5A              cmp al,0x5a
0000770E  760B              jna 0x771b
00007710  3C61              cmp al,0x61
00007712  7204              jc 0x7718
00007714  3C7A              cmp al,0x7a
00007716  7603              jna 0x771b
00007718  F8                clc
00007719  EB01              jmp 0x771c
0000771B  F9                stc
0000771C  58                pop ax
0000771D  C3                ret
0000771E  53                push bx
0000771F  8B1E0609          mov bx,[0x906]
00007723  891E0A09          mov [0x90a],bx
00007727  891E0C09          mov [0x90c],bx
0000772B  8B1E0809          mov bx,[0x908]
0000772F  891E0E09          mov [0x90e],bx
00007733  5B                pop bx
00007734  C3                ret
00007735  50                push ax
00007736  53                push bx
00007737  51                push cx
00007738  52                push dx
00007739  8B160A09          mov dx,[0x90a]
0000773D  8B0E0C09          mov cx,[0x90c]
00007741  2BCA              sub cx,dx
00007743  E305              jcxz 0x774a
00007745  E84E00            call 0x7796
00007748  E2FB              loop 0x7745
0000774A  8B1E0A09          mov bx,[0x90a]
0000774E  3B1E0809          cmp bx,[0x908]
00007752  7411              jz 0x7765
00007754  8A87C707          mov al,[bx+0x7c7]
00007758  E814DC            call 0x536f
0000775B  3CFF              cmp al,0xff
0000775D  7503              jnz 0x7762
0000775F  E80DDC            call 0x536f
00007762  43                inc bx
00007763  EBE9              jmp 0x774e
00007765  8B0E0E09          mov cx,[0x90e]
00007769  2BCB              sub cx,bx
0000776B  0BC9              or cx,cx
0000776D  7E0E              jng 0x777d
0000776F  51                push cx
00007770  B020              mov al,0x20
00007772  E8FADB            call 0x536f
00007775  E2FB              loop 0x7772
00007777  59                pop cx
00007778  E81B00            call 0x7796
0000777B  E2FB              loop 0x7778
0000777D  8BCB              mov cx,bx
0000777F  8B1E0609          mov bx,[0x906]
00007783  2BCB              sub cx,bx
00007785  E307              jcxz 0x778e
00007787  E80C00            call 0x7796
0000778A  E2FB              loop 0x7787
0000778C  EB03              jmp 0x7791
0000778E  E82700            call 0x77b8
00007791  5A                pop dx
00007792  59                pop cx
00007793  5B                pop bx
00007794  58                pop ax
00007795  C3                ret
00007796  50                push ax
00007797  A0240B            mov al,[0xb24]
0000779A  2403              and al,0x3
0000779C  3C03              cmp al,0x3
0000779E  7412              jz 0x77b2
000077A0  52                push dx
000077A1  8B166C0A          mov dx,[0xa6c]
000077A5  FECE              dec dh
000077A7  7505              jnz 0x77ae
000077A9  8A36800A          mov dh,[0xa80]
000077AD  4A                dec dx
000077AE  E869E6            call 0x5e1a
000077B1  5A                pop dx
000077B2  58                pop ax
000077B3  C3                ret
000077B4  E8EDF2            call 0x6aa4
000077B7  C3                ret
000077B8  50                push ax
000077B9  53                push bx
000077BA  E831DE            call 0x55ee
000077BD  3A26800A          cmp ah,[0xa80]
000077C1  751E              jnz 0x77e1
000077C3  8B1E0609          mov bx,[0x906]
000077C7  3B1E0809          cmp bx,[0x908]
000077CB  750A              jnz 0x77d7
000077CD  B020              mov al,0x20
000077CF  E89DDB            call 0x536f
000077D2  E8C1FF            call 0x7796
000077D5  EB0A              jmp 0x77e1
000077D7  8A87C707          mov al,[bx+0x7c7]
000077DB  E891DB            call 0x536f
000077DE  E8B5FF            call 0x7796
000077E1  5B                pop bx
000077E2  58                pop ax
000077E3  C3                ret
000077E4  55                push bp
000077E5  8BEC              mov bp,sp
000077E7  8C061C09          mov word [0x91c],es
000077EB  89361A09          mov [0x91a],si
000077EF  893E1809          mov [0x918],di
000077F3  8D5E0C            lea bx,[bp+0xc]
000077F6  891E1209          mov [0x912],bx
000077FA  891E0409          mov [0x904],bx
000077FE  1E                push ds
000077FF  07                pop es
00007800  8BF5              mov si,bp
00007802  BF1E09            mov di,0x91e
00007805  B90300            mov cx,0x3
00007808  F3A5              rep movsw
0000780A  890E4D0D          mov [0xd4d],cx
0000780E  8B5E0A            mov bx,[bp+0xa]
00007811  891E2809          mov [0x928],bx
00007815  C47E06            les di,word [bp+0x6]
00007818  268B05            mov ax,[es:di]
0000781B  48                dec ax
0000781C  A32609            mov [0x926],ax
0000781F  47                inc di
00007820  47                inc di
00007821  268A05            mov al,[es:di]
00007824  A22A09            mov [0x92a],al
00007827  47                inc di
00007828  893E1409          mov [0x914],di
0000782C  8C061609          mov word [0x916],es
00007830  8B1E2809          mov bx,[0x928]
00007834  E84DF7            call 0x6f84
00007837  F6062A0901        test byte [0x92a],0x1
0000783C  750A              jnz 0x7848
0000783E  B03F              mov al,0x3f
00007840  E80FF7            call 0x6f52
00007843  B020              mov al,0x20
00007845  E80AF7            call 0x6f52
00007848  E84FFB            call 0x739a
0000784B  F6062A0902        test byte [0x92a],0x2
00007850  7503              jnz 0x7855
00007852  E873F7            call 0x6fc8
00007855  8B260409          mov sp,[0x904]
00007859  A12609            mov ax,[0x926]
0000785C  A32409            mov [0x924],ax
0000785F  BEC707            mov si,0x7c7
00007862  C43E1409          les di,word [0x914]
00007866  268A05            mov al,[es:di]
00007869  47                inc di
0000786A  A2340D            mov [0xd34],al
0000786D  C70644075649      mov word [0x744],0x4956
00007873  06                push es
00007874  1E                push ds
00007875  07                pop es
00007876  FF062E0D          inc word [0xd2e]
0000787A  E8CF0C            call 0x854c
0000787D  FF0E2E0D          dec word [0xd2e]
00007881  07                pop es
00007882  C70644070000      mov word [0x744],0x0
00007888  8A0E340D          mov cl,[0xd34]
0000788C  80E10F            and cl,0xf
0000788F  80F904            cmp cl,0x4
00007892  BB230D            mov bx,0xd23
00007895  7203              jc 0x789a
00007897  FF7702            push word [bx+0x2]
0000789A  FF37              push word [bx]
0000789C  7606              jna 0x78a4
0000789E  FF77FE            push word [bx-0x2]
000078A1  FF77FC            push word [bx-0x4]
000078A4  FF0E2409          dec word [0x924]
000078A8  7440              jz 0x78ea
000078AA  3C2C              cmp al,0x2c
000078AC  74B8              jz 0x7866
000078AE  E899F6            call 0x6f4a
000078B1  B80080            mov ax,0x8000
000078B4  E8A4F6            call 0x6f5b
000078B7  E890F6            call 0x6f4a
000078BA  A12E0D            mov ax,[0xd2e]
000078BD  40                inc ax
000078BE  E841D2            call 0x4b02
000078C1  8B261209          mov sp,[0x912]
000078C5  FF362209          push word [0x922]
000078C9  FF362009          push word [0x920]
000078CD  FF361E09          push word [0x91e]
000078D1  8BEC              mov bp,sp
000078D3  E95AFF            jmp 0x7830
000078D6  FF0E2E0D          dec word [0xd2e]
000078DA  C70644070000      mov word [0x744],0x0
000078E0  53                push bx
000078E1  E866F6            call 0x6f4a
000078E4  58                pop ax
000078E5  E873F6            call 0x6f5b
000078E8  EBC4              jmp 0x78ae
000078EA  0AC0              or al,al
000078EC  75C0              jnz 0x78ae
000078EE  8B1E2809          mov bx,[0x928]
000078F2  E824D2            call 0x4b19
000078F5  C7063D0B9449      mov word [0xb3d],0x4994
000078FB  C6063C0B00        mov byte [0xb3c],0x0
00007900  8B3E1809          mov di,[0x918]
00007904  8B361A09          mov si,[0x91a]
00007908  8E061C09          mov es,word [0x91c]
0000790C  8B2E1E09          mov bp,[0x91e]
00007910  FF2E2009          jmp word far [0x920]
00007914  8B361209          mov si,[0x912]
00007918  A0340D            mov al,[0xd34]
0000791B  98                cbw
0000791C  240E              and al,0xe
0000791E  2BF0              sub si,ax
00007920  89361209          mov [0x912],si
00007924  C3                ret
00007925  55                push bp
00007926  8BEC              mov bp,sp
00007928  B80680            mov ax,0x8006
0000792B  E82DF6            call 0x6f5b
0000792E  B87C09            mov ax,0x97c
00007931  50                push ax
00007932  1E                push ds
00007933  B8440B            mov ax,0xb44
00007936  50                push ax
00007937  9A64485802        call word 0x258:word 0x4864
0000793C  1E                push ds
0000793D  B8820A            mov ax,0xa82
00007940  50                push ax
00007941  9A0E4A5802        call word 0x258:word 0x4a0e
00007946  9A56425802        call word 0x258:word 0x4256
0000794B  5D                pop bp
0000794C  CB                retf
0000794D  00558B            add [di-0x75],dl
00007950  EC                in al,dx
00007951  B00E              mov al,0xe
00007953  E818DD            call 0x566e
00007956  50                push ax
00007957  B00C              mov al,0xc
00007959  E812DD            call 0x566e
0000795C  5B                pop bx
0000795D  894702            mov [bx+0x2],ax
00007960  C60701            mov byte [bx],0x1
00007963  5D                pop bp
00007964  CB                retf
00007965  55                push bp
00007966  8BEC              mov bp,sp
00007968  57                push di
00007969  06                push es
0000796A  9ACE495802        call word 0x258:word 0x49ce
0000796F  97                xchg ax,di
00007970  4F                dec di
00007971  4F                dec di
00007972  1E                push ds
00007973  07                pop es
00007974  33C0              xor ax,ax
00007976  8B5606            mov dx,[bp+0x6]
00007979  EB06              jmp 0x7981
0000797B  92                xchg ax,dx
0000797C  B9FFFF            mov cx,0xffff
0000797F  F2AE              repne scasb
00007981  92                xchg ax,dx
00007982  AF                scasw
00007983  77F6              ja 0x797b
00007985  897F02            mov [bx+0x2],di
00007988  07                pop es
00007989  5F                pop di
0000798A  5D                pop bp
0000798B  CA0200            retf word 0x2
0000798E  B302              mov bl,0x2
00007990  3DB314            cmp ax,0x14b3
00007993  3DB304            cmp ax,0x4b3
00007996  3DB308            cmp ax,0x8b3
00007999  3DB303            cmp ax,0x3b3
0000799C  55                push bp
0000799D  8BEC              mov bp,sp
0000799F  56                push si
000079A0  57                push di
000079A1  06                push es
000079A2  881E340D          mov [0xd34],bl
000079A6  53                push bx
000079A7  1E                push ds
000079A8  07                pop es
000079A9  FF163D0B          call word near [0xb3d]
000079AD  59                pop cx
000079AE  D0E9              shr cl,1
000079B0  720F              jc 0x79c1
000079B2  C47E06            les di,word [bp+0x6]
000079B5  83E107            and cx,0x7
000079B8  F3A5              rep movsw
000079BA  07                pop es
000079BB  5F                pop di
000079BC  5E                pop si
000079BD  5D                pop bp
000079BE  CA0400            retf word 0x4
000079C1  33C0              xor ax,ax
000079C3  1E                push ds
000079C4  FF34              push word [si]
000079C6  50                push ax
000079C7  FF760A            push word [bp+0xa]
000079CA  FF7608            push word [bp+0x8]
000079CD  FF7606            push word [bp+0x6]
000079D0  9A544E5802        call word 0x258:word 0x4e54
000079D5  07                pop es
000079D6  5F                pop di
000079D7  5E                pop si
000079D8  5D                pop bp
000079D9  CA0600            retf word 0x6
000079DC  57                push di
000079DD  06                push es
000079DE  E83200            call 0x7a13
000079E1  5B                pop bx
000079E2  53                push bx
000079E3  E8E30C            call 0x86c9
000079E6  8EDB              mov ds,bx
000079E8  FEC8              dec al
000079EA  7421              jz 0x7a0d
000079EC  4E                dec si
000079ED  06                push es
000079EE  E85B0B            call 0x854c
000079F1  1F                pop ds
000079F2  3C2C              cmp al,0x2c
000079F4  7404              jz 0x79fa
000079F6  0AC0              or al,al
000079F8  7516              jnz 0x7a10
000079FA  E82A00            call 0x7a27
000079FD  BE230D            mov si,0xd23
00007A00  F606340D08        test byte [0xd34],0x8
00007A05  7403              jz 0x7a0a
00007A07  83EE04            sub si,0x4
00007A0A  07                pop es
00007A0B  5F                pop di
00007A0C  C3                ret
00007A0D  E9E6DE            jmp 0x58f6
00007A10  E9DDDE            jmp 0x58f0
00007A13  B00E              mov al,0xe
00007A15  E856DC            call 0x566e
00007A18  97                xchg ax,di
00007A19  803D00            cmp byte [di],0x0
00007A1C  7505              jnz 0x7a23
00007A1E  9ACE495802        call word 0x258:word 0x49ce
00007A23  8B7502            mov si,[di+0x2]
00007A26  C3                ret
00007A27  0AC0              or al,al
00007A29  7502              jnz 0x7a2d
00007A2B  46                inc si
00007A2C  46                inc si
00007A2D  897502            mov [di+0x2],si
00007A30  C3                ret
00007A31  00558B            add [di-0x75],dl
00007A34  EC                in al,dx
00007A35  56                push si
00007A36  57                push di
00007A37  06                push es
00007A38  E82900            call 0x7a64
00007A3B  50                push ax
00007A3C  92                xchg ax,dx
00007A3D  F3AB              rep stosw
00007A3F  7301              jnc 0x7a42
00007A41  AA                stosb
00007A42  59                pop cx
00007A43  F3A4              rep movsb
00007A45  E8D1D0            call 0x4b19
00007A48  07                pop es
00007A49  5F                pop di
00007A4A  5E                pop si
00007A4B  5D                pop bp
00007A4C  CA0800            retf word 0x8
00007A4F  55                push bp
00007A50  8BEC              mov bp,sp
00007A52  56                push si
00007A53  57                push di
00007A54  06                push es
00007A55  E80C00            call 0x7a64
00007A58  91                xchg ax,cx
00007A59  F3A4              rep movsb
00007A5B  91                xchg ax,cx
00007A5C  92                xchg ax,dx
00007A5D  F3AB              rep stosw
00007A5F  73E4              jnc 0x7a45
00007A61  AA                stosb
00007A62  EBE1              jmp 0x7a45
00007A64  8B760C            mov si,[bp+0xc]
00007A67  56                push si
00007A68  AD                lodsw
00007A69  8B34              mov si,[si]
00007A6B  C47E08            les di,word [bp+0x8]
00007A6E  8B4E06            mov cx,[bp+0x6]
00007A71  0BC9              or cx,cx
00007A73  7505              jnz 0x7a7a
00007A75  8B0D              mov cx,[di]
00007A77  8B7D02            mov di,[di+0x2]
00007A7A  2BC8              sub cx,ax
00007A7C  7304              jnc 0x7a82
00007A7E  03C1              add ax,cx
00007A80  33C9              xor cx,cx
00007A82  D1E9              shr cx,1
00007A84  BA2020            mov dx,0x2020
00007A87  5B                pop bx
00007A88  C3                ret
00007A89  00558B            add [di-0x75],dl
00007A8C  EC                in al,dx
00007A8D  8B5E08            mov bx,[bp+0x8]
00007A90  E831D0            call 0x4ac4
00007A93  7325              jnc 0x7aba
00007A95  87DA              xchg bx,dx
00007A97  8B5E06            mov bx,[bp+0x6]
00007A9A  E8BDD0            call 0x4b5a
00007A9D  87DA              xchg bx,dx
00007A9F  8B0F              mov cx,[bx]
00007AA1  8B4702            mov ax,[bx+0x2]
00007AA4  87DA              xchg bx,dx
00007AA6  890F              mov [bx],cx
00007AA8  894702            mov [bx+0x2],ax
00007AAB  E309              jcxz 0x7ab6
00007AAD  93                xchg ax,bx
00007AAE  8947FE            mov [bx-0x2],ax
00007AB1  87DA              xchg bx,dx
00007AB3  E86BD0            call 0x4b21
00007AB6  5D                pop bp
00007AB7  CA0400            retf word 0x4
00007ABA  33C9              xor cx,cx
00007ABC  8B17              mov dx,[bx]
00007ABE  E870D0            call 0x4b31
00007AC1  EBD2              jmp 0x7a95
00007AC3  55                push bp
00007AC4  8BEC              mov bp,sp
00007AC6  56                push si
00007AC7  57                push di
00007AC8  06                push es
00007AC9  1E                push ds
00007ACA  07                pop es
00007ACB  8B7608            mov si,[bp+0x8]
00007ACE  8B7E06            mov di,[bp+0x6]
00007AD1  8B1C              mov bx,[si]
00007AD3  031D              add bx,[di]
00007AD5  7026              jo 0x7afd
00007AD7  E8B1CF            call 0x4a8b
00007ADA  87FA              xchg di,dx
00007ADC  53                push bx
00007ADD  E80D00            call 0x7aed
00007AE0  8BF2              mov si,dx
00007AE2  E80800            call 0x7aed
00007AE5  58                pop ax
00007AE6  07                pop es
00007AE7  5F                pop di
00007AE8  5E                pop si
00007AE9  5D                pop bp
00007AEA  CA0400            retf word 0x4
00007AED  8BDE              mov bx,si
00007AEF  AD                lodsw
00007AF0  91                xchg ax,cx
00007AF1  AD                lodsw
00007AF2  96                xchg ax,si
00007AF3  D1E9              shr cx,1
00007AF5  F3A5              rep movsw
00007AF7  7301              jnc 0x7afa
00007AF9  A4                movsb
00007AFA  E91CD0            jmp 0x4b19
00007AFD  E9F9DD            jmp 0x58f9
00007B00  33D2              xor dx,dx
00007B02  55                push bp
00007B03  8BEC              mov bp,sp
00007B05  56                push si
00007B06  57                push di
00007B07  06                push es
00007B08  1E                push ds
00007B09  07                pop es
00007B0A  8B5E06            mov bx,[bp+0x6]
00007B0D  8B4608            mov ax,[bp+0x8]
00007B10  8B7F02            mov di,[bx+0x2]
00007B13  8B0F              mov cx,[bx]
00007B15  93                xchg ax,bx
00007B16  8B7702            mov si,[bx+0x2]
00007B19  3B0F              cmp cx,[bx]
00007B1B  7602              jna 0x7b1f
00007B1D  8B0F              mov cx,[bx]
00007B1F  3BC0              cmp ax,ax
00007B21  F3A6              repe cmpsb
00007B23  7506              jnz 0x7b2b
00007B25  8B0F              mov cx,[bx]
00007B27  93                xchg ax,bx
00007B28  3B0F              cmp cx,[bx]
00007B2A  93                xchg ax,bx
00007B2B  9C                pushf
00007B2C  0BD2              or dx,dx
00007B2E  7503              jnz 0x7b33
00007B30  E8E6CF            call 0x4b19
00007B33  93                xchg ax,bx
00007B34  E8E2CF            call 0x4b19
00007B37  9D                popf
00007B38  07                pop es
00007B39  5F                pop di
00007B3A  5E                pop si
00007B3B  5D                pop bp
00007B3C  CA0400            retf word 0x4
00007B3F  55                push bp
00007B40  8BEC              mov bp,sp
00007B42  8B4606            mov ax,[bp+0x6]
00007B45  BB0100            mov bx,0x1
00007B48  0AE4              or ah,ah
00007B4A  75B1              jnz 0x7afd
00007B4C  50                push ax
00007B4D  E83BCF            call 0x4a8b
00007B50  87DA              xchg bx,dx
00007B52  8F07              pop word [bx]
00007B54  92                xchg ax,dx
00007B55  5D                pop bp
00007B56  CA0200            retf word 0x2
00007B59  00558B            add [di-0x75],dl
00007B5C  EC                in al,dx
00007B5D  8B5E06            mov bx,[bp+0x6]
00007B60  8B0F              mov cx,[bx]
00007B62  E30B              jcxz 0x7b6f
00007B64  FF7702            push word [bx+0x2]
00007B67  EB08              jmp 0x7b71
00007B69  55                push bp
00007B6A  8BEC              mov bp,sp
00007B6C  8B5E06            mov bx,[bp+0x6]
00007B6F  FF37              push word [bx]
00007B71  E8A5CF            call 0x4b19
00007B74  58                pop ax
00007B75  5D                pop bp
00007B76  CA0200            retf word 0x2
00007B79  55                push bp
00007B7A  8BEC              mov bp,sp
00007B7C  56                push si
00007B7D  8B5E06            mov bx,[bp+0x6]
00007B80  833F00            cmp word [bx],0x0
00007B83  7410              jz 0x7b95
00007B85  8B7702            mov si,[bx+0x2]
00007B88  8B34              mov si,[si]
00007B8A  E88CCF            call 0x4b19
00007B8D  96                xchg ax,si
00007B8E  32E4              xor ah,ah
00007B90  5E                pop si
00007B91  5D                pop bp
00007B92  CA0200            retf word 0x2
00007B95  E961DD            jmp 0x58f9
00007B98  55                push bp
00007B99  8BEC              mov bp,sp
00007B9B  56                push si
00007B9C  57                push di
00007B9D  06                push es
00007B9E  8B5E0A            mov bx,[bp+0xa]
00007BA1  0BDB              or bx,bx
00007BA3  7EF0              jng 0x7b95
00007BA5  8B7E08            mov di,[bp+0x8]
00007BA8  8B7606            mov si,[bp+0x6]
00007BAB  AD                lodsw
00007BAC  92                xchg ax,dx
00007BAD  8B34              mov si,[si]
00007BAF  8B0D              mov cx,[di]
00007BB1  E345              jcxz 0x7bf8
00007BB3  4A                dec dx
00007BB4  7906              jns 0x7bbc
00007BB6  3BD9              cmp bx,cx
00007BB8  7F3E              jg 0x7bf8
00007BBA  EB26              jmp 0x7be2
00007BBC  8B7D02            mov di,[di+0x2]
00007BBF  4B                dec bx
00007BC0  03FB              add di,bx
00007BC2  2BCA              sub cx,dx
00007BC4  93                xchg ax,bx
00007BC5  8BD9              mov bx,cx
00007BC7  2BC8              sub cx,ax
00007BC9  7E2D              jng 0x7bf8
00007BCB  AC                lodsb
00007BCC  1E                push ds
00007BCD  07                pop es
00007BCE  E328              jcxz 0x7bf8
00007BD0  F2AE              repne scasb
00007BD2  7524              jnz 0x7bf8
00007BD4  51                push cx
00007BD5  56                push si
00007BD6  57                push di
00007BD7  8BCA              mov cx,dx
00007BD9  F3A6              repe cmpsb
00007BDB  5F                pop di
00007BDC  5E                pop si
00007BDD  59                pop cx
00007BDE  75EC              jnz 0x7bcc
00007BE0  2BD9              sub bx,cx
00007BE2  87DA              xchg bx,dx
00007BE4  8B5E08            mov bx,[bp+0x8]
00007BE7  E82FCF            call 0x4b19
00007BEA  8B5E06            mov bx,[bp+0x6]
00007BED  E829CF            call 0x4b19
00007BF0  92                xchg ax,dx
00007BF1  07                pop es
00007BF2  5F                pop di
00007BF3  5E                pop si
00007BF4  5D                pop bp
00007BF5  CA0600            retf word 0x6
00007BF8  33DB              xor bx,bx
00007BFA  EBE6              jmp 0x7be2
00007BFC  55                push bp
00007BFD  8BEC              mov bp,sp
00007BFF  BB0100            mov bx,0x1
00007C02  53                push bx
00007C03  FF7608            push word [bp+0x8]
00007C06  FF7606            push word [bp+0x6]
00007C09  9A184C5802        call word 0x258:word 0x4c18
00007C0E  5D                pop bp
00007C0F  CA0400            retf word 0x4
00007C12  55                push bp
00007C13  8BEC              mov bp,sp
00007C15  33C9              xor cx,cx
00007C17  8B5E08            mov bx,[bp+0x8]
00007C1A  8B5606            mov dx,[bp+0x6]
00007C1D  EB0D              jmp 0x7c2c
00007C1F  55                push bp
00007C20  8BEC              mov bp,sp
00007C22  8B5E08            mov bx,[bp+0x8]
00007C25  8B5606            mov dx,[bp+0x6]
00007C28  8B0F              mov cx,[bx]
00007C2A  2BCA              sub cx,dx
00007C2C  E83F00            call 0x7c6e
00007C2F  5D                pop bp
00007C30  CA0400            retf word 0x4
00007C33  E9C3DC            jmp 0x58f9
00007C36  55                push bp
00007C37  8BEC              mov bp,sp
00007C39  8B5E0A            mov bx,[bp+0xa]
00007C3C  8B4E06            mov cx,[bp+0x6]
00007C3F  0BC9              or cx,cx
00007C41  7CF0              jl 0x7c33
00007C43  8B5608            mov dx,[bp+0x8]
00007C46  4A                dec dx
00007C47  87D1              xchg dx,cx
00007C49  7CE8              jl 0x7c33
00007C4B  7505              jnz 0x7c52
00007C4D  E81E00            call 0x7c6e
00007C50  EB18              jmp 0x7c6a
00007C52  8B07              mov ax,[bx]
00007C54  2BC1              sub ax,cx
00007C56  7F08              jg 0x7c60
00007C58  E8BECE            call 0x4b19
00007C5B  B87C09            mov ax,0x97c
00007C5E  EB0A              jmp 0x7c6a
00007C60  3BC2              cmp ax,dx
00007C62  7D02              jnl 0x7c66
00007C64  8BD0              mov dx,ax
00007C66  E8C8CE            call 0x4b31
00007C69  93                xchg ax,bx
00007C6A  5D                pop bp
00007C6B  CA0600            retf word 0x6
00007C6E  0BD2              or dx,dx
00007C70  7C4A              jl 0x7cbc
00007C72  7507              jnz 0x7c7b
00007C74  E8A2CE            call 0x4b19
00007C77  B87C09            mov ax,0x97c
00007C7A  C3                ret
00007C7B  3B17              cmp dx,[bx]
00007C7D  7E08              jng 0x7c87
00007C7F  8B17              mov dx,[bx]
00007C81  0BC9              or cx,cx
00007C83  7D02              jnl 0x7c87
00007C85  33C9              xor cx,cx
00007C87  E8A7CE            call 0x4b31
00007C8A  93                xchg ax,bx
00007C8B  C3                ret
00007C8C  55                push bp
00007C8D  8BEC              mov bp,sp
00007C8F  B220              mov dl,0x20
00007C91  8B5E06            mov bx,[bp+0x6]
00007C94  E80400            call 0x7c9b
00007C97  5D                pop bp
00007C98  CA0200            retf word 0x2
00007C9B  06                push es
00007C9C  B87C09            mov ax,0x97c
00007C9F  0BDB              or bx,bx
00007CA1  7417              jz 0x7cba
00007CA3  7C17              jl 0x7cbc
00007CA5  8BCB              mov cx,bx
00007CA7  8AC2              mov al,dl
00007CA9  8AE2              mov ah,dl
00007CAB  E8DDCD            call 0x4a8b
00007CAE  41                inc cx
00007CAF  87FA              xchg di,dx
00007CB1  D1E9              shr cx,1
00007CB3  1E                push ds
00007CB4  07                pop es
00007CB5  F3AB              rep stosw
00007CB7  8BFA              mov di,dx
00007CB9  93                xchg ax,bx
00007CBA  07                pop es
00007CBB  C3                ret
00007CBC  E93ADC            jmp 0x58f9
00007CBF  55                push bp
00007CC0  8BEC              mov bp,sp
00007CC2  8B5606            mov dx,[bp+0x6]
00007CC5  EB17              jmp 0x7cde
00007CC7  55                push bp
00007CC8  8BEC              mov bp,sp
00007CCA  8B5E06            mov bx,[bp+0x6]
00007CCD  833F00            cmp word [bx],0x0
00007CD0  74EA              jz 0x7cbc
00007CD2  53                push bx
00007CD3  8B5F02            mov bx,[bx+0x2]
00007CD6  58                pop ax
00007CD7  FF37              push word [bx]
00007CD9  93                xchg ax,bx
00007CDA  E83CCE            call 0x4b19
00007CDD  5A                pop dx
00007CDE  8B5E08            mov bx,[bp+0x8]
00007CE1  E8B7FF            call 0x7c9b
00007CE4  5D                pop bp
00007CE5  CA0400            retf word 0x4
00007CE8  55                push bp
00007CE9  8BEC              mov bp,sp
00007CEB  BB0300            mov bx,0x3
00007CEE  E89ACD            call 0x4a8b
00007CF1  87DA              xchg bx,dx
00007CF3  8B4606            mov ax,[bp+0x6]
00007CF6  8807              mov [bx],al
00007CF8  8B4608            mov ax,[bp+0x8]
00007CFB  894701            mov [bx+0x1],ax
00007CFE  92                xchg ax,dx
00007CFF  5D                pop bp
00007D00  CA0400            retf word 0x4
00007D03  B8617A            mov ax,0x7a61
00007D06  EB03              jmp 0x7d0b
00007D08  B8415A            mov ax,0x5a41
00007D0B  55                push bp
00007D0C  8BEC              mov bp,sp
00007D0E  56                push si
00007D0F  57                push di
00007D10  06                push es
00007D11  8B5E06            mov bx,[bp+0x6]
00007D14  8B17              mov dx,[bx]
00007D16  0BD2              or dx,dx
00007D18  741F              jz 0x7d39
00007D1A  33C9              xor cx,cx
00007D1C  E812CE            call 0x4b31
00007D1F  53                push bx
00007D20  87CA              xchg cx,dx
00007D22  8B7702            mov si,[bx+0x2]
00007D25  8BFE              mov di,si
00007D27  93                xchg ax,bx
00007D28  1E                push ds
00007D29  07                pop es
00007D2A  AC                lodsb
00007D2B  3AC3              cmp al,bl
00007D2D  7206              jc 0x7d35
00007D2F  3AC7              cmp al,bh
00007D31  7702              ja 0x7d35
00007D33  3420              xor al,0x20
00007D35  AA                stosb
00007D36  E2F2              loop 0x7d2a
00007D38  5B                pop bx
00007D39  93                xchg ax,bx
00007D3A  07                pop es
00007D3B  5F                pop di
00007D3C  5E                pop si
00007D3D  5D                pop bp
00007D3E  CA0200            retf word 0x2
00007D41  B401              mov ah,0x1
00007D43  3DB402            cmp ax,0x2b4
00007D46  55                push bp
00007D47  8BEC              mov bp,sp
00007D49  57                push di
00007D4A  06                push es
00007D4B  1E                push ds
00007D4C  07                pop es
00007D4D  8B5E06            mov bx,[bp+0x6]
00007D50  8B0F              mov cx,[bx]
00007D52  E329              jcxz 0x7d7d
00007D54  8B7F02            mov di,[bx+0x2]
00007D57  B020              mov al,0x20
00007D59  F6C401            test ah,0x1
00007D5C  7406              jz 0x7d64
00007D5E  F3AE              repe scasb
00007D60  7416              jz 0x7d78
00007D62  41                inc cx
00007D63  4F                dec di
00007D64  8BD7              mov dx,di
00007D66  2B5702            sub dx,[bx+0x2]
00007D69  F6C402            test ah,0x2
00007D6C  740A              jz 0x7d78
00007D6E  03F9              add di,cx
00007D70  4F                dec di
00007D71  FD                std
00007D72  F3AE              repe scasb
00007D74  FC                cld
00007D75  7401              jz 0x7d78
00007D77  41                inc cx
00007D78  87CA              xchg cx,dx
00007D7A  E8B4CD            call 0x4b31
00007D7D  93                xchg ax,bx
00007D7E  07                pop es
00007D7F  5F                pop di
00007D80  5D                pop bp
00007D81  CA0200            retf word 0x2
00007D84  55                push bp
00007D85  8BEC              mov bp,sp
00007D87  B002              mov al,0x2
00007D89  8D5E06            lea bx,[bp+0x6]
00007D8C  E81300            call 0x7da2
00007D8F  5D                pop bp
00007D90  CA0200            retf word 0x2
00007D93  55                push bp
00007D94  8BEC              mov bp,sp
00007D96  B014              mov al,0x14
00007D98  8D5E06            lea bx,[bp+0x6]
00007D9B  E80400            call 0x7da2
00007D9E  5D                pop bp
00007D9F  CA0400            retf word 0x4
00007DA2  A2340D            mov [0xd34],al
00007DA5  E83AF2            call 0x6fe2
00007DA8  93                xchg ax,bx
00007DA9  92                xchg ax,dx
00007DAA  E83ACD            call 0x4ae7
00007DAD  93                xchg ax,bx
00007DAE  C3                ret
00007DAF  55                push bp
00007DB0  8BEC              mov bp,sp
00007DB2  56                push si
00007DB3  57                push di
00007DB4  06                push es
00007DB5  1E                push ds
00007DB6  07                pop es
00007DB7  8B5E06            mov bx,[bp+0x6]
00007DBA  53                push bx
00007DBB  E8CDCC            call 0x4a8b
00007DBE  59                pop cx
00007DBF  53                push bx
00007DC0  8BFA              mov di,dx
00007DC2  1E                push ds
00007DC3  C57608            lds si,word [bp+0x8]
00007DC6  41                inc cx
00007DC7  D1E9              shr cx,1
00007DC9  F3A5              rep movsw
00007DCB  1F                pop ds
00007DCC  58                pop ax
00007DCD  07                pop es
00007DCE  5F                pop di
00007DCF  5E                pop si
00007DD0  5D                pop bp
00007DD1  CA0600            retf word 0x6
00007DD4  55                push bp
00007DD5  8BEC              mov bp,sp
00007DD7  56                push si
00007DD8  57                push di
00007DD9  06                push es
00007DDA  C4760E            les si,word [bp+0xe]
00007DDD  8CC3              mov bx,es
00007DDF  8B4E0C            mov cx,[bp+0xc]
00007DE2  C47E08            les di,word [bp+0x8]
00007DE5  8B4606            mov ax,[bp+0x6]
00007DE8  0BC0              or ax,ax
00007DEA  E329              jcxz 0x7e15
00007DEC  7416              jz 0x7e04
00007DEE  2BC1              sub ax,cx
00007DF0  7305              jnc 0x7df7
00007DF2  8B4E06            mov cx,[bp+0x6]
00007DF5  33C0              xor ax,ax
00007DF7  1E                push ds
00007DF8  8EDB              mov ds,bx
00007DFA  F3A4              rep movsb
00007DFC  91                xchg ax,cx
00007DFD  B020              mov al,0x20
00007DFF  F3AA              rep stosb
00007E01  1F                pop ds
00007E02  EB1D              jmp 0x7e21
00007E04  53                push bx
00007E05  56                push si
00007E06  51                push cx
00007E07  9A2F4E5802        call word 0x258:word 0x4e2f
00007E0C  50                push ax
00007E0D  57                push di
00007E0E  9A0A4B5802        call word 0x258:word 0x4b0a
00007E13  EB0C              jmp 0x7e21
00007E15  96                xchg ax,si
00007E16  74F4              jz 0x7e0c
00007E18  50                push ax
00007E19  06                push es
00007E1A  57                push di
00007E1B  56                push si
00007E1C  9ACF4A5802        call word 0x258:word 0x4acf
00007E21  07                pop es
00007E22  5F                pop di
00007E23  5E                pop si
00007E24  5D                pop bp
00007E25  CA0C00            retf word 0xc
00007E28  55                push bp
00007E29  8BEC              mov bp,sp
00007E2B  56                push si
00007E2C  57                push di
00007E2D  06                push es
00007E2E  1E                push ds
00007E2F  C47E08            les di,word [bp+0x8]
00007E32  8B5606            mov dx,[bp+0x6]
00007E35  C5760E            lds si,word [bp+0xe]
00007E38  8B4E0C            mov cx,[bp+0xc]
00007E3B  8BDA              mov bx,dx
00007E3D  2BD9              sub bx,cx
00007E3F  7513              jnz 0x7e54
00007E41  E34E              jcxz 0x7e91
00007E43  268A05            mov al,[es:di]
00007E46  8604              xchg al,[si]
00007E48  AA                stosb
00007E49  46                inc si
00007E4A  E2F7              loop 0x7e43
00007E4C  8BCB              mov cx,bx
00007E4E  B020              mov al,0x20
00007E50  F3AA              rep stosb
00007E52  EB40              jmp 0x7e94
00007E54  E31A              jcxz 0x7e70
00007E56  87CA              xchg cx,dx
00007E58  E310              jcxz 0x7e6a
00007E5A  87CA              xchg cx,dx
00007E5C  73E5              jnc 0x7e43
00007E5E  8BCA              mov cx,dx
00007E60  F7DB              neg bx
00007E62  C47E0E            les di,word [bp+0xe]
00007E65  C57608            lds si,word [bp+0x8]
00007E68  EBD9              jmp 0x7e43
00007E6A  C47E0E            les di,word [bp+0xe]
00007E6D  C57608            lds si,word [bp+0x8]
00007E70  52                push dx
00007E71  06                push es
00007E72  57                push di
00007E73  52                push dx
00007E74  9A2F4E5802        call word 0x258:word 0x4e2f
00007E79  5A                pop dx
00007E7A  97                xchg ax,di
00007E7B  33DB              xor bx,bx
00007E7D  1E                push ds
00007E7E  56                push si
00007E7F  53                push bx
00007E80  06                push es
00007E81  50                push ax
00007E82  52                push dx
00007E83  9A544E5802        call word 0x258:word 0x4e54
00007E88  57                push di
00007E89  56                push si
00007E8A  9A0A4B5802        call word 0x258:word 0x4b0a
00007E8F  EB03              jmp 0x7e94
00007E91  E888CD            call 0x4c1c
00007E94  1F                pop ds
00007E95  07                pop es
00007E96  5F                pop di
00007E97  5E                pop si
00007E98  5D                pop bp
00007E99  CA0C00            retf word 0xc
00007E9C  E87DCD            call 0x4c1c
00007E9F  CB                retf
00007EA0  55                push bp
00007EA1  8BEC              mov bp,sp
00007EA3  33C9              xor cx,cx
00007EA5  8B5E06            mov bx,[bp+0x6]
00007EA8  8B17              mov dx,[bx]
00007EAA  E884CC            call 0x4b31
00007EAD  93                xchg ax,bx
00007EAE  5D                pop bp
00007EAF  CA0200            retf word 0x2
00007EB2  55                push bp
00007EB3  8BEC              mov bp,sp
00007EB5  8B5E06            mov bx,[bp+0x6]
00007EB8  53                push bx
00007EB9  E89ECC            call 0x4b5a
00007EBC  5B                pop bx
00007EBD  C7070000          mov word [bx],0x0
00007EC1  5D                pop bp
00007EC2  CA0200            retf word 0x2
00007EC5  55                push bp
00007EC6  8BEC              mov bp,sp
00007EC8  FF7606            push word [bp+0x6]
00007ECB  9A204F5802        call word 0x258:word 0x4f20
00007ED0  50                push ax
00007ED1  FF7606            push word [bp+0x6]
00007ED4  9A324F5802        call word 0x258:word 0x4f32
00007ED9  58                pop ax
00007EDA  5D                pop bp
00007EDB  CA0200            retf word 0x2
00007EDE  3B26120D          cmp sp,[0xd12]
00007EE2  7220              jc 0x7f04
00007EE4  FF062E0D          inc word [0xd2e]
00007EE8  FF46F6            inc word [bp-0xa]
00007EEB  CB                retf
00007EEC  FF0E2E0D          dec word [0xd2e]
00007EF0  FF4EF6            dec word [bp-0xa]
00007EF3  7815              js 0x7f0a
00007EF5  83C402            add sp,0x2
00007EF8  8F062E09          pop word [0x92e]
00007EFC  8F062C09          pop word [0x92c]
00007F00  FF2E2C09          jmp word far [0x92c]
00007F04  55                push bp
00007F05  8BEC              mov bp,sp
00007F07  E975DA            jmp 0x597f
00007F0A  55                push bp
00007F0B  8BEC              mov bp,sp
00007F0D  E9E3D9            jmp 0x58f3
00007F10  8F062C09          pop word [0x92c]
00007F14  8F062E09          pop word [0x92e]
00007F18  FF0E2E0D          dec word [0xd2e]
00007F1C  8D66FA            lea sp,[bp-0x6]
00007F1F  5F                pop di
00007F20  5E                pop si
00007F21  8F062C0D          pop word [0xd2c]
00007F25  5D                pop bp
00007F26  FF2E2C09          jmp word far [0x92c]
00007F2A  8BC4              mov ax,sp
00007F2C  2BC1              sub ax,cx
00007F2E  72D4              jc 0x7f04
00007F30  2D0A00            sub ax,0xa
00007F33  72CF              jc 0x7f04
00007F35  3B06120D          cmp ax,[0xd12]
00007F39  72C9              jc 0x7f04
00007F3B  8F062C09          pop word [0x92c]
00007F3F  8F062E09          pop word [0x92e]
00007F43  33C0              xor ax,ax
00007F45  55                push bp
00007F46  8BEC              mov bp,sp
00007F48  FF362C0D          push word [0xd2c]
00007F4C  56                push si
00007F4D  57                push di
00007F4E  51                push cx
00007F4F  50                push ax
00007F50  2BE1              sub sp,cx
00007F52  892E2C0D          mov [0xd2c],bp
00007F56  FF062E0D          inc word [0xd2e]
00007F5A  8BFC              mov di,sp
00007F5C  06                push es
00007F5D  1E                push ds
00007F5E  07                pop es
00007F5F  D1E9              shr cx,1
00007F61  33C0              xor ax,ax
00007F63  F3AB              rep stosw
00007F65  07                pop es
00007F66  FF2E2C09          jmp word far [0x92c]
00007F6A  E81200            call 0x7f7f
00007F6D  E86AEF            call 0x6eda
00007F70  7221              jc 0x7f93
00007F72  5E                pop si
00007F73  07                pop es
00007F74  5D                pop bp
00007F75  58                pop ax
00007F76  5A                pop dx
00007F77  59                pop cx
00007F78  D1E1              shl cx,1
00007F7A  03E1              add sp,cx
00007F7C  52                push dx
00007F7D  50                push ax
00007F7E  CB                retf
00007F7F  5B                pop bx
00007F80  55                push bp
00007F81  8BEC              mov bp,sp
00007F83  06                push es
00007F84  56                push si
00007F85  8B4E06            mov cx,[bp+0x6]
00007F88  8BC1              mov ax,cx
00007F8A  D1E0              shl ax,1
00007F8C  8D7606            lea si,[bp+0x6]
00007F8F  03F0              add si,ax
00007F91  FFE3              jmp bx
00007F93  E963D9            jmp 0x58f9
00007F96  E8E6FF            call 0x7f7f
00007F99  E85F00            call 0x7ffb
00007F9C  7503              jnz 0x7fa1
00007F9E  A06C0A            mov al,[0xa6c]
00007FA1  50                push ax
00007FA2  E85600            call 0x7ffb
00007FA5  7503              jnz 0x7faa
00007FA7  A06D0A            mov al,[0xa6d]
00007FAA  50                push ax
00007FAB  E84D00            call 0x7ffb
00007FAE  92                xchg ax,dx
00007FAF  E84900            call 0x7ffb
00007FB2  93                xchg ax,bx
00007FB3  E84500            call 0x7ffb
00007FB6  91                xchg ax,cx
00007FB7  92                xchg ax,dx
00007FB8  E827EF            call 0x6ee2
00007FBB  72D6              jc 0x7f93
00007FBD  59                pop cx
00007FBE  5A                pop dx
00007FBF  0AF6              or dh,dh
00007FC1  741F              jz 0x7fe2
00007FC3  0AD2              or dl,dl
00007FC5  74CC              jz 0x7f93
00007FC7  8AC2              mov al,dl
00007FC9  3A06710A          cmp al,[0xa71]
00007FCD  760D              jna 0x7fdc
00007FCF  803E270700        cmp byte [0x727],0x0
00007FD4  75BD              jnz 0x7f93
00007FD6  3A066E0A          cmp al,[0xa6e]
00007FDA  75B7              jnz 0x7f93
00007FDC  3A06700A          cmp al,[0xa70]
00007FE0  72B1              jc 0x7f93
00007FE2  FECD              dec ch
00007FE4  7508              jnz 0x7fee
00007FE6  E3AB              jcxz 0x7f93
00007FE8  3A0E800A          cmp cl,[0xa80]
00007FEC  77A5              ja 0x7f93
00007FEE  E861E7            call 0x6752
00007FF1  8AF1              mov dh,cl
00007FF3  E824DE            call 0x5e1a
00007FF6  EAF24F5802        jmp word 0x258:word 0x4ff2
00007FFB  52                push dx
00007FFC  B601              mov dh,0x1
00007FFE  E8B7DF            call 0x5fb8
00008001  7502              jnz 0x8005
00008003  FECE              dec dh
00008005  8AE6              mov ah,dh
00008007  5A                pop dx
00008008  C3                ret
00008009  55                push bp
0000800A  8BEC              mov bp,sp
0000800C  8B5E06            mov bx,[bp+0x6]
0000800F  43                inc bx
00008010  7517              jnz 0x8029
00008012  B80206            mov ax,0x602
00008015  E8F4EA            call 0x6b0c
00008018  7431              jz 0x804b
0000801A  B80101            mov ax,0x101
0000801D  803EE00600        cmp byte [0x6e0],0x0
00008022  7527              jnz 0x804b
00008024  B80007            mov ax,0x700
00008027  EB22              jmp 0x804b
00008029  4B                dec bx
0000802A  83FB02            cmp bx,0x2
0000802D  7743              ja 0x8072
0000802F  8AC3              mov al,bl
00008031  B407              mov ah,0x7
00008033  3C01              cmp al,0x1
00008035  7214              jc 0x804b
00008037  B404              mov ah,0x4
00008039  7710              ja 0x804b
0000803B  E8CEEA            call 0x6b0c
0000803E  742E              jz 0x806e
00008040  B401              mov ah,0x1
00008042  803EE00600        cmp byte [0x6e0],0x0
00008047  7502              jnz 0x804b
00008049  B407              mov ah,0x7
0000804B  E802E9            call 0x6950
0000804E  7222              jc 0x8072
00008050  F6C401            test ah,0x1
00008053  7406              jz 0x805b
00008055  50                push ax
00008056  FF167C0A          call word near [0xa7c]
0000805A  58                pop ax
0000805B  F6C402            test ah,0x2
0000805E  7403              jz 0x8063
00008060  E800F2            call 0x7263
00008063  F6C404            test ah,0x4
00008066  7406              jz 0x806e
00008068  E8CDEA            call 0x6b38
0000806B  E8ACDD            call 0x5e1a
0000806E  5D                pop bp
0000806F  CA0200            retf word 0x2
00008072  E984D8            jmp 0x58f9
00008075  8B166C0A          mov dx,[0xa6c]
00008079  3A36800A          cmp dh,[0xa80]
0000807D  7603              jna 0x8082
0000807F  E8BDEA            call 0x6b3f
00008082  92                xchg ax,dx
00008083  98                cbw
00008084  CB                retf
00008085  55                push bp
00008086  8BEC              mov bp,sp
00008088  A06D0A            mov al,[0xa6d]
0000808B  3A06800A          cmp al,[0xa80]
0000808F  7602              jna 0x8093
00008091  B001              mov al,0x1
00008093  98                cbw
00008094  5D                pop bp
00008095  CA0200            retf word 0x2
00008098  55                push bp
00008099  8BEC              mov bp,sp
0000809B  B80001            mov ax,0x100
0000809E  8B4E06            mov cx,[bp+0x6]
000080A1  E305              jcxz 0x80a8
000080A3  E211              loop 0x80b6
000080A5  B8FFFF            mov ax,0xffff
000080A8  3A062707          cmp al,[0x727]
000080AC  A22707            mov [0x727],al
000080AF  7408              jz 0x80b9
000080B1  E8AFF1            call 0x7263
000080B4  EB03              jmp 0x80b9
000080B6  E80400            call 0x80bd
000080B9  5D                pop bp
000080BA  CA0200            retf word 0x2
000080BD  56                push si
000080BE  57                push di
000080BF  BF0000            mov di,0x0
000080C2  B90C00            mov cx,0xc
000080C5  32D2              xor dl,dl
000080C7  F6065D06FF        test byte [0x65d],0xff
000080CC  7502              jnz 0x80d0
000080CE  49                dec cx
000080CF  49                dec cx
000080D0  FEC2              inc dl
000080D2  B046              mov al,0x46
000080D4  E87BEE            call 0x6f52
000080D7  E82B00            call 0x8105
000080DA  B020              mov al,0x20
000080DC  E873EE            call 0x6f52
000080DF  51                push cx
000080E0  8B0D              mov cx,[di]
000080E2  E313              jcxz 0x80f7
000080E4  8B7502            mov si,[di+0x2]
000080E7  AC                lodsb
000080E8  0AC0              or al,al
000080EA  740B              jz 0x80f7
000080EC  3C0D              cmp al,0xd
000080EE  7502              jnz 0x80f2
000080F0  B020              mov al,0x20
000080F2  E85DEE            call 0x6f52
000080F5  E2F0              loop 0x80e7
000080F7  B00D              mov al,0xd
000080F9  E856EE            call 0x6f52
000080FC  59                pop cx
000080FD  83C704            add di,0x4
00008100  E2CE              loop 0x80d0
00008102  5F                pop di
00008103  5E                pop si
00008104  C3                ret
00008105  8AC2              mov al,dl
00008107  D40A              aam
00008109  0D3030            or ax,0x3030
0000810C  80FC30            cmp ah,0x30
0000810F  7504              jnz 0x8115
00008111  86E0              xchg ah,al
00008113  B020              mov al,0x20
00008115  50                push ax
00008116  8AC4              mov al,ah
00008118  E837EE            call 0x6f52
0000811B  58                pop ax
0000811C  E933EE            jmp 0x6f52
0000811F  55                push bp
00008120  8BEC              mov bp,sp
00008122  8B560A            mov dx,[bp+0xa]
00008125  8B4608            mov ax,[bp+0x8]
00008128  0AE6              or ah,dh
0000812A  7521              jnz 0x814d
0000812C  4A                dec dx
0000812D  48                dec ax
0000812E  3A06800A          cmp al,[0xa80]
00008132  7319              jnc 0x814d
00008134  3A166E0A          cmp dl,[0xa6e]
00008138  7313              jnc 0x814d
0000813A  42                inc dx
0000813B  40                inc ax
0000813C  8AF0              mov dh,al
0000813E  E869ED            call 0x6eaa
00008141  93                xchg ax,bx
00008142  837E0600          cmp word [bp+0x6],0x0
00008146  7501              jnz 0x8149
00008148  93                xchg ax,bx
00008149  5D                pop bp
0000814A  CA0600            retf word 0x6
0000814D  E9A9D7            jmp 0x58f9
00008150  E9A6D7            jmp 0x58f9
00008153  E9D6D7            jmp 0x592c
00008156  55                push bp
00008157  8BEC              mov bp,sp
00008159  56                push si
0000815A  57                push di
0000815B  06                push es
0000815C  1E                push ds
0000815D  07                pop es
0000815E  8B4E08            mov cx,[bp+0x8]
00008161  8B5E06            mov bx,[bp+0x6]
00008164  33F6              xor si,si
00008166  0BDB              or bx,bx
00008168  7415              jz 0x817f
0000816A  80FBFF            cmp bl,0xff
0000816D  7410              jz 0x817f
0000816F  E893D0            call 0x5205
00008172  74DF              jz 0x8153
00008174  8B4410            mov ax,[si+0x10]
00008177  A3480B            mov [0xb48],ax
0000817A  F6040A            test byte [si],0xa
0000817D  7553              jnz 0x81d2
0000817F  89364D0D          mov [0xd4d],si
00008183  0BC9              or cx,cx
00008185  78C9              js 0x8150
00008187  8BD9              mov bx,cx
00008189  E8FFC8            call 0x4a8b
0000818C  E336              jcxz 0x81c4
0000818E  53                push bx
0000818F  0BF6              or si,si
00008191  7424              jz 0x81b7
00008193  F60420            test byte [si],0x20
00008196  741F              jz 0x81b7
00008198  89163806          mov [0x638],dx
0000819C  8C1E3A06          mov word [0x63a],ds
000081A0  8BD9              mov bx,cx
000081A2  B8040A            mov ax,0xa04
000081A5  51                push cx
000081A6  E859C4            call 0x4602
000081A9  59                pop cx
000081AA  5B                pop bx
000081AB  3BC8              cmp cx,ax
000081AD  7415              jz 0x81c4
000081AF  92                xchg ax,dx
000081B0  33C9              xor cx,cx
000081B2  E87CC9            call 0x4b31
000081B5  EB0D              jmp 0x81c4
000081B7  8BFA              mov di,dx
000081B9  E83BC5            call 0x46f7
000081BC  7417              jz 0x81d5
000081BE  7212              jc 0x81d2
000081C0  AA                stosb
000081C1  E2F6              loop 0x81b9
000081C3  5B                pop bx
000081C4  93                xchg ax,bx
000081C5  C7064D0D0000      mov word [0xd4d],0x0
000081CB  07                pop es
000081CC  5F                pop di
000081CD  5E                pop si
000081CE  5D                pop bp
000081CF  CA0400            retf word 0x4
000081D2  E972D7            jmp 0x5947
000081D5  E9D4E3            jmp 0x65ac
000081D8  800E3C0B01        or byte [0xb3c],0x1
000081DD  9A56425802        call word 0x258:word 0x4256
000081E2  C3                ret
000081E3  009B529B          add [bp+di-0x64ae],bl
000081E7  52                push dx
000081E8  9B52              wait push dx
000081EA  F008F0            lock or al,dh
000081ED  088C529B          or [si-0x64ae],cl
000081F1  52                push dx
000081F2  9B52              wait push dx
000081F4  9B52              wait push dx
000081F6  53                push bx
000081F7  57                push di
000081F8  BB6252            mov bx,0x5262
000081FB  BF1200            mov di,0x12
000081FE  53                push bx
000081FF  57                push di
00008200  2EFF11            call word near [cs:bx+di]
00008203  5F                pop di
00008204  5B                pop bx
00008205  4F                dec di
00008206  4F                dec di
00008207  75F5              jnz 0x81fe
00008209  5F                pop di
0000820A  5B                pop bx
0000820B  C3                ret
0000820C  C606360D02        mov byte [0xd36],0x2
00008211  C606390DFA        mov byte [0xd39],0xfa
00008216  C6063A0D50        mov byte [0xd3a],0x50
0000821B  C3                ret
0000821C  32D2              xor dl,dl
0000821E  88165006          mov [0x650],dl
00008222  B401              mov ah,0x1
00008224  E924C3            jmp 0x454b
00008227  8A166F0A          mov dl,[0xa6f]
0000822B  B406              mov ah,0x6
0000822D  E81BC3            call 0x454b
00008230  C606260702        mov byte [0x726],0x2
00008235  C60402            mov byte [si],0x2
00008238  C3                ret
00008239  1400              adc al,0x0
0000823B  1010              adc [bx+si],dl
0000823D  7C23              jl 0x8262
0000823F  5E                pop si
00008240  23D0              and dx,ax
00008242  52                push dx
00008243  D208              ror byte [bx+si],cl
00008245  D208              ror byte [bx+si],cl
00008247  7253              jc 0x829c
00008249  7253              jc 0x829e
0000824B  7253              jc 0x82a0
0000824D  380E00B2          cmp [0xb200],cl
00008251  29B229B2          sub [bp+si-0x4dd7],si
00008255  298C1EF7          sub [si-0x8e2],cx
00008259  52                push dx
0000825A  B229              mov dl,0x29
0000825C  EE                out dx,al
0000825D  52                push dx
0000825E  B229              mov dl,0x29
00008260  B229              mov dl,0x29
00008262  363BF5            ss cmp si,bp
00008265  52                push dx
00008266  F8                clc
00008267  52                push dx
00008268  F752B2            not word [bp+si-0x4e]
0000826B  29B229B4          sub [bp+si-0x4bd7],si
0000826F  02B2FFE9          add dh,[bp+si-0x1601]
00008273  D6                salc
00008274  C2B400            ret word 0xb4
00008277  C3                ret
00008278  B4FF              mov ah,0xff
0000827A  C3                ret
0000827B  005351            add [bp+di+0x51],dl
0000827E  50                push ax
0000827F  B406              mov ah,0x6
00008281  8A953E0B          mov dl,[di+0xb3e]
00008285  E8C3C2            call 0x454b
00008288  56                push si
00008289  BECC07            mov si,0x7cc
0000828C  E85400            call 0x82e3
0000828F  741E              jz 0x82af
00008291  3C42              cmp al,0x42
00008293  753E              jnz 0x82d3
00008295  E84200            call 0x82da
00008298  3C49              cmp al,0x49
0000829A  7537              jnz 0x82d3
0000829C  E83B00            call 0x82da
0000829F  3C4E              cmp al,0x4e
000082A1  7530              jnz 0x82d3
000082A3  E83D00            call 0x82e3
000082A6  752B              jnz 0x82d3
000082A8  5E                pop si
000082A9  804C0520          or byte [si+0x5],0x20
000082AD  EB05              jmp 0x82b4
000082AF  5E                pop si
000082B0  806405DF          and byte [si+0x5],0xdf
000082B4  58                pop ax
000082B5  50                push ax
000082B6  F6D8              neg al
000082B8  98                cbw
000082B9  D1E0              shl ax,1
000082BB  8BF8              mov di,ax
000082BD  E8DA01            call 0x849a
000082C0  E8C7AE            call 0x318a
000082C3  0AE4              or ah,ah
000082C5  7405              jz 0x82cc
000082C7  E842CB            call 0x4e0c
000082CA  EB5B              jmp 0x8327
000082CC  895C01            mov [si+0x1],bx
000082CF  58                pop ax
000082D0  59                pop cx
000082D1  5B                pop bx
000082D2  C3                ret
000082D3  5E                pop si
000082D4  E835CB            call 0x4e0c
000082D7  E973D6            jmp 0x594d
000082DA  803C00            cmp byte [si],0x0
000082DD  7501              jnz 0x82e0
000082DF  C3                ret
000082E0  AC                lodsb
000082E1  EB09              jmp 0x82ec
000082E3  AC                lodsb
000082E4  3C20              cmp al,0x20
000082E6  74FB              jz 0x82e3
000082E8  3C00              cmp al,0x0
000082EA  7405              jz 0x82f1
000082EC  E892E8            call 0x6b81
000082EF  0AC0              or al,al
000082F1  C3                ret
000082F2  7929              jns 0x831d
000082F4  7929              jns 0x831f
000082F6  7929              jns 0x8321
000082F8  2655              es push bp
000082FA  9D                popf
000082FB  53                push bx
000082FC  7929              jns 0x8327
000082FE  FC                cld
000082FF  52                push dx
00008300  7929              jns 0x832b
00008302  7929              jns 0x832d
00008304  EE                out dx,al
00008305  53                push bx
00008306  AB                stosw
00008307  54                push sp
00008308  B054              mov al,0x54
0000830A  90                nop
0000830B  53                push bx
0000830C  7929              jns 0x8337
0000830E  7929              jns 0x8339
00008310  88953E0B          mov [di+0xb3e],dl
00008314  3CFA              cmp al,0xfa
00008316  7504              jnz 0x831c
00008318  88163A0D          mov [0xd3a],dl
0000831C  C3                ret
0000831D  885404            mov [si+0x4],dl
00008320  C3                ret
00008321  D329              shr word [bx+di],cl
00008323  9A29A02980        call word 0x8029:word 0xa029
00008328  FC                cld
00008329  037603            add si,[bp+0x3]
0000832C  E90CD6            jmp 0x593b
0000832F  8AC4              mov al,ah
00008331  98                cbw
00008332  48                dec ax
00008333  D1E0              shl ax,1
00008335  93                xchg ax,bx
00008336  BEF00B            mov si,0xbf0
00008339  58                pop ax
0000833A  8AC4              mov al,ah
0000833C  98                cbw
0000833D  D1E0              shl ax,1
0000833F  D1E0              shl ax,1
00008341  03F0              add si,ax
00008343  8B3E740A          mov di,[0xa74]
00008347  06                push es
00008348  1E                push ds
00008349  07                pop es
0000834A  A5                movsw
0000834B  A5                movsw
0000834C  07                pop es
0000834D  C706720A0400      mov word [0xa72],0x4
00008353  8B87000C          mov ax,[bx+0xc00]
00008357  A33107            mov [0x731],ax
0000835A  2EFFA7A153        jmp word near [cs:bx+0x53a1]
0000835F  50                push ax
00008360  53                push bx
00008361  8B5C01            mov bx,[si+0x1]
00008364  E885AE            call 0x31ec
00008367  0AE4              or ah,ah
00008369  75BC              jnz 0x8327
0000836B  5B                pop bx
0000836C  58                pop ax
0000836D  C3                ret
0000836E  52                push dx
0000836F  81FE360D          cmp si,0xd36
00008373  751D              jnz 0x8392
00008375  F6063B0D10        test byte [0xd3b],0x10
0000837A  7516              jnz 0x8392
0000837C  50                push ax
0000837D  53                push bx
0000837E  32E4              xor ah,ah
00008380  E807AE            call 0x318a
00008383  0AE4              or ah,ah
00008385  75A0              jnz 0x8327
00008387  891E370D          mov [0xd37],bx
0000838B  800E3B0D10        or byte [0xd3b],0x10
00008390  5B                pop bx
00008391  58                pop ax
00008392  53                push bx
00008393  50                push ax
00008394  E8D700            call 0x846e
00008397  8ADC              mov bl,ah
00008399  58                pop ax
0000839A  8AE3              mov ah,bl
0000839C  5B                pop bx
0000839D  8B15              mov dx,[di]
0000839F  81FE360D          cmp si,0xd36
000083A3  7411              jz 0x83b6
000083A5  8A5404            mov dl,[si+0x4]
000083A8  F6440520          test byte [si+0x5],0x20
000083AC  7408              jz 0x83b6
000083AE  E8AEFF            call 0x835f
000083B1  E85E00            call 0x8412
000083B4  EB03              jmp 0x83b9
000083B6  E84100            call 0x83fa
000083B9  887501            mov [di+0x1],dh
000083BC  5A                pop dx
000083BD  C3                ret
000083BE  E85100            call 0x8412
000083C1  3C0A              cmp al,0xa
000083C3  750D              jnz 0x83d2
000083C5  803E3E0601        cmp byte [0x63e],0x1
000083CA  7506              jnz 0x83d2
000083CC  F6450601          test byte [di+0x6],0x1
000083D0  7503              jnz 0x83d5
000083D2  E88AFF            call 0x835f
000083D5  806506FE          and byte [di+0x6],0xfe
000083D9  3C0D              cmp al,0xd
000083DB  751C              jnz 0x83f9
000083DD  804D0601          or byte [di+0x6],0x1
000083E1  81FE360D          cmp si,0xd36
000083E5  740B              jz 0x83f2
000083E7  803C04            cmp byte [si],0x4
000083EA  7506              jnz 0x83f2
000083EC  807C04FF          cmp byte [si+0x4],0xff
000083F0  7407              jz 0x83f9
000083F2  50                push ax
000083F3  B00A              mov al,0xa
000083F5  E867FF            call 0x835f
000083F8  58                pop ax
000083F9  C3                ret
000083FA  E8C1FF            call 0x83be
000083FD  3AF2              cmp dh,dl
000083FF  7210              jc 0x8411
00008401  3C20              cmp al,0x20
00008403  720C              jc 0x8411
00008405  80FAFF            cmp dl,0xff
00008408  7407              jz 0x8411
0000840A  50                push ax
0000840B  B00D              mov al,0xd
0000840D  E8AEFF            call 0x83be
00008410  58                pop ax
00008411  C3                ret
00008412  3C20              cmp al,0x20
00008414  7203              jc 0x8419
00008416  FEC6              inc dh
00008418  C3                ret
00008419  3C0D              cmp al,0xd
0000841B  7503              jnz 0x8420
0000841D  B600              mov dh,0x0
0000841F  C3                ret
00008420  3C08              cmp al,0x8
00008422  7506              jnz 0x842a
00008424  0AF6              or dh,dh
00008426  7402              jz 0x842a
00008428  FECE              dec dh
0000842A  C3                ret
0000842B  8AA53F0B          mov ah,[di+0xb3f]
0000842F  C3                ret
00008430  8A6404            mov ah,[si+0x4]
00008433  C3                ret
00008434  55                push bp
00008435  8BEC              mov bp,sp
00008437  8B5E06            mov bx,[bp+0x6]
0000843A  83FB03            cmp bx,0x3
0000843D  7712              ja 0x8451
0000843F  0BDB              or bx,bx
00008441  7401              jz 0x8444
00008443  4B                dec bx
00008444  D1E3              shl bx,1
00008446  8A874B0B          mov al,[bx+0xb4b]
0000844A  32E4              xor ah,ah
0000844C  40                inc ax
0000844D  5D                pop bp
0000844E  CA0200            retf word 0x2
00008451  E9A5D4            jmp 0x58f9
00008454  55                push bp
00008455  8BEC              mov bp,sp
00008457  8B5606            mov dx,[bp+0x6]
0000845A  0AF6              or dh,dh
0000845C  75F3              jnz 0x8451
0000845E  0AD2              or dl,dl
00008460  74EF              jz 0x8451
00008462  88164A0B          mov [0xb4a],dl
00008466  88163A0D          mov [0xd3a],dl
0000846A  5D                pop bp
0000846B  CA0200            retf word 0x2
0000846E  E82900            call 0x849a
00008471  50                push ax
00008472  86E0              xchg ah,al
00008474  32E4              xor ah,ah
00008476  D1E0              shl ax,1
00008478  054A0B            add ax,0xb4a
0000847B  8BF8              mov di,ax
0000847D  58                pop ax
0000847E  C3                ret
0000847F  50                push ax
00008480  56                push si
00008481  57                push di
00008482  BE360D            mov si,0xd36
00008485  BF0C00            mov di,0xc
00008488  0AE4              or ah,ah
0000848A  7407              jz 0x8493
0000848C  50                push ax
0000848D  86E0              xchg ah,al
0000848F  E8DCFE            call 0x836e
00008492  58                pop ax
00008493  E8D8FE            call 0x836e
00008496  5F                pop di
00008497  5E                pop si
00008498  58                pop ax
00008499  C3                ret
0000849A  53                push bx
0000849B  8BDF              mov bx,di
0000849D  83C3F4            add bx,0xfffffffffffffff4
000084A0  D1EB              shr bx,1
000084A2  8AE3              mov ah,bl
000084A4  5B                pop bx
000084A5  C3                ret
000084A6  53                push bx
000084A7  8B5C01            mov bx,[si+0x1]
000084AA  E857AD            call 0x3204
000084AD  5B                pop bx
000084AE  81FE360D          cmp si,0xd36
000084B2  7403              jz 0x84b7
000084B4  E855C9            call 0x4e0c
000084B7  C3                ret
000084B8  56                push si
000084B9  800E8A0A08        or byte [0xa8a],0x8
000084BE  C7064D0D360D      mov word [0xd4d],0xd36
000084C4  BE060C            mov si,0xc06
000084C7  E843ED            call 0x720d
000084CA  5E                pop si
000084CB  CB                retf
000084CC  55                push bp
000084CD  8BEC              mov bp,sp
000084CF  56                push si
000084D0  8B5E06            mov bx,[bp+0x6]
000084D3  E84B00            call 0x8521
000084D6  E82CCD            call 0x5205
000084D9  7443              jz 0x851e
000084DB  89364D0D          mov [0xd4d],si
000084DF  8B4410            mov ax,[si+0x10]
000084E2  A3480B            mov [0xb48],ax
000084E5  8A4403            mov al,[si+0x3]
000084E8  5E                pop si
000084E9  5D                pop bp
000084EA  CA0200            retf word 0x2
000084ED  55                push bp
000084EE  8BEC              mov bp,sp
000084F0  56                push si
000084F1  8B5E06            mov bx,[bp+0x6]
000084F4  E82A00            call 0x8521
000084F7  E80BCD            call 0x5205
000084FA  7422              jz 0x851e
000084FC  8B4410            mov ax,[si+0x10]
000084FF  A3480B            mov [0xb48],ax
00008502  803C01            cmp byte [si],0x1
00008505  7414              jz 0x851b
00008507  89364D0D          mov [0xd4d],si
0000850B  800E8A0A01        or byte [0xa8a],0x1
00008510  BE060C            mov si,0xc06
00008513  E8F7EC            call 0x720d
00008516  5E                pop si
00008517  5D                pop bp
00008518  CA0200            retf word 0x2
0000851B  E914D4            jmp 0x5932
0000851E  E90BD4            jmp 0x592c
00008521  0BDB              or bx,bx
00008523  74F9              jz 0x851e
00008525  0AFF              or bh,bh
00008527  75F5              jnz 0x851e
00008529  C3                ret
0000852A  55                push bp
0000852B  8BEC              mov bp,sp
0000852D  56                push si
0000852E  8B5E06            mov bx,[bp+0x6]
00008531  E869C6            call 0x4b9d
00008534  8B7702            mov si,[bx+0x2]
00008537  53                push bx
00008538  C606340D08        mov byte [0xd34],0x8
0000853D  E80C00            call 0x854c
00008540  5B                pop bx
00008541  E8D5C5            call 0x4b19
00008544  B81F0D            mov ax,0xd1f
00008547  5E                pop si
00008548  5D                pop bp
00008549  CA0200            retf word 0x2
0000854C  57                push di
0000854D  06                push es
0000854E  55                push bp
0000854F  803E340D03        cmp byte [0xd34],0x3
00008554  7513              jnz 0x8569
00008556  E82F01            call 0x8688
00008559  50                push ax
0000855A  06                push es
0000855B  52                push dx
0000855C  51                push cx
0000855D  9A2F4E5802        call word 0x258:word 0x4e2f
00008562  A3230D            mov [0xd23],ax
00008565  58                pop ax
00008566  E91901            jmp 0x8682
00008569  1E                push ds
0000856A  07                pop es
0000856B  33DB              xor bx,bx
0000856D  C6066B0B01        mov byte [0xb6b],0x1
00008572  E85401            call 0x86c9
00008575  3C26              cmp al,0x26
00008577  7514              jnz 0x858d
00008579  881E6B0B          mov [0xb6b],bl
0000857D  E84601            call 0x86c6
00008580  BB1000            mov bx,0x10
00008583  3C48              cmp al,0x48
00008585  7407              jz 0x858e
00008587  B308              mov bl,0x8
00008589  3C4F              cmp al,0x4f
0000858B  7401              jz 0x858e
0000858D  4E                dec si
0000858E  8BCE              mov cx,si
00008590  FEC5              inc ch
00008592  7502              jnz 0x8596
00008594  33C9              xor cx,cx
00008596  2BCE              sub cx,si
00008598  49                dec cx
00008599  BF1F0D            mov di,0xd1f
0000859C  33C0              xor ax,ax
0000859E  99                cwd
0000859F  53                push bx
000085A0  55                push bp
000085A1  9AB003E507        call word 0x7e5:word 0x3b0
000085A6  5D                pop bp
000085A7  C6066B0B00        mov byte [0xb6b],0x0
000085AC  99                cwd
000085AD  3BDA              cmp bx,dx
000085AF  7403              jz 0x85b4
000085B1  80C980            or cl,0x80
000085B4  F6C120            test cl,0x20
000085B7  7403              jz 0x85bc
000085B9  80C980            or cl,0x80
000085BC  87DA              xchg bx,dx
000085BE  5B                pop bx
000085BF  8AFB              mov bh,bl
000085C1  93                xchg ax,bx
000085C2  E80101            call 0x86c6
000085C5  3C26              cmp al,0x26
000085C7  7420              jz 0x85e9
000085C9  0AE4              or ah,ah
000085CB  741C              jz 0x85e9
000085CD  F6C120            test cl,0x20
000085D0  7517              jnz 0x85e9
000085D2  0BD2              or dx,dx
000085D4  7513              jnz 0x85e9
000085D6  93                xchg ax,bx
000085D7  99                cwd
000085D8  80E17F            and cl,0x7f
000085DB  A31F0D            mov [0xd1f],ax
000085DE  93                xchg ax,bx
000085DF  CD3B              int byte 0x3b
000085E1  06                push es
000085E2  1F                pop ds
000085E3  0DCD39            or ax,0x39cd
000085E6  1E                push ds
000085E7  1F                pop ds
000085E8  0D3C23            or ax,0x233c
000085EB  7505              jnz 0x85f2
000085ED  80C9A8            or cl,0xa8
000085F0  EB29              jmp 0x861b
000085F2  3C21              cmp al,0x21
000085F4  7508              jnz 0x85fe
000085F6  80E1F7            and cl,0xf7
000085F9  80C9A0            or cl,0xa0
000085FC  EB1D              jmp 0x861b
000085FE  3C25              cmp al,0x25
00008600  7507              jnz 0x8609
00008602  F6C180            test cl,0x80
00008605  7510              jnz 0x8617
00008607  EB12              jmp 0x861b
00008609  3C26              cmp al,0x26
0000860B  750D              jnz 0x861a
0000860D  F6C120            test cl,0x20
00008610  7505              jnz 0x8617
00008612  80C980            or cl,0x80
00008615  EB04              jmp 0x861b
00008617  E9F1D2            jmp 0x590b
0000861A  4E                dec si
0000861B  93                xchg ax,bx
0000861C  8A1E340D          mov bl,[0xd34]
00008620  F6C180            test cl,0x80
00008623  7505              jnz 0x862a
00008625  80FB02            cmp bl,0x2
00008628  740E              jz 0x8638
0000862A  F6C120            test cl,0x20
0000862D  750E              jnz 0x863d
0000862F  80FB14            cmp bl,0x14
00008632  7509              jnz 0x863d
00008634  8916250D          mov [0xd25],dx
00008638  A3230D            mov [0xd23],ax
0000863B  EB42              jmp 0x867f
0000863D  F6C101            test cl,0x1
00008640  753A              jnz 0x867c
00008642  87DA              xchg bx,dx
00008644  80FA08            cmp dl,0x8
00008647  7436              jz 0x867f
00008649  CD39              int byte 0x39
0000864B  06                push es
0000864C  1F                pop ds
0000864D  0D80FA            or ax,0xfa80
00008650  0475              add al,0x75
00008652  07                pop es
00008653  CD35              int byte 0x35
00008655  1E                push ds
00008656  230D              and cx,[di]
00008658  EB25              jmp 0x867f
0000865A  80FA02            cmp dl,0x2
0000865D  7405              jz 0x8664
0000865F  80FA14            cmp dl,0x14
00008662  7518              jnz 0x867c
00008664  52                push dx
00008665  E88B00            call 0x86f3
00008668  A3230D            mov [0xd23],ax
0000866B  8916250D          mov [0xd25],dx
0000866F  8BDA              mov bx,dx
00008671  99                cwd
00008672  3BD3              cmp dx,bx
00008674  5A                pop dx
00008675  7408              jz 0x867f
00008677  80FA14            cmp dl,0x14
0000867A  7403              jz 0x867f
0000867C  E97DD2            jmp 0x58fc
0000867F  E84700            call 0x86c9
00008682  CD3D              int byte 0x3d
00008684  5D                pop bp
00008685  07                pop es
00008686  5F                pop di
00008687  C3                ret
00008688  1E                push ds
00008689  06                push es
0000868A  1F                pop ds
0000868B  E83B00            call 0x86c9
0000868E  8AE0              mov ah,al
00008690  3C22              cmp al,0x22
00008692  7403              jz 0x8697
00008694  B42C              mov ah,0x2c
00008696  4E                dec si
00008697  8BD6              mov dx,si
00008699  33C9              xor cx,cx
0000869B  AC                lodsb
0000869C  3AC4              cmp al,ah
0000869E  740B              jz 0x86ab
000086A0  0AC0              or al,al
000086A2  E0F7              loopne 0x869b
000086A4  41                inc cx
000086A5  80FC2C            cmp ah,0x2c
000086A8  7401              jz 0x86ab
000086AA  4E                dec si
000086AB  F7D9              neg cx
000086AD  80FC2C            cmp ah,0x2c
000086B0  750F              jnz 0x86c1
000086B2  4E                dec si
000086B3  8BFE              mov di,si
000086B5  4F                dec di
000086B6  B020              mov al,0x20
000086B8  3AC0              cmp al,al
000086BA  FD                std
000086BB  F3AE              repe scasb
000086BD  FC                cld
000086BE  7401              jz 0x86c1
000086C0  41                inc cx
000086C1  E80500            call 0x86c9
000086C4  1F                pop ds
000086C5  C3                ret
000086C6  AC                lodsb
000086C7  EB0D              jmp 0x86d6
000086C9  AC                lodsb
000086CA  3C20              cmp al,0x20
000086CC  74FB              jz 0x86c9
000086CE  3C09              cmp al,0x9
000086D0  74F7              jz 0x86c9
000086D2  3C0A              cmp al,0xa
000086D4  74F3              jz 0x86c9
000086D6  E8A8E4            call 0x6b81
000086D9  C3                ret
000086DA  53                push bx
000086DB  8BDC              mov bx,sp
000086DD  CD3B              int byte 0x3b
000086DF  07                pop es
000086E0  5B                pop bx
000086E1  C3                ret
000086E2  51                push cx
000086E3  52                push dx
000086E4  8BDC              mov bx,sp
000086E6  CD35              int byte 0x35
000086E8  07                pop es
000086E9  83C404            add sp,0x4
000086EC  C3                ret
000086ED  9AD903E507        call word 0x7e5:word 0x3d9
000086F2  C3                ret
000086F3  9AD903E507        call word 0x7e5:word 0x3d9
000086F8  C3                ret
000086F9  BB230D            mov bx,0xd23
000086FC  8A16340D          mov dl,[0xd34]
00008700  80FA03            cmp dl,0x3
00008703  7308              jnc 0x870d
00008705  7403              jz 0x870a
00008707  8B1F              mov bx,[bx]
00008709  C3                ret
0000870A  E9FED1            jmp 0x590b
0000870D  06                push es
0000870E  53                push bx
0000870F  80FA04            cmp dl,0x4
00008712  7505              jnz 0x8719
00008714  CD35              int byte 0x35
00008716  07                pop es
00008717  EB03              jmp 0x871c
00008719  CD39              int byte 0x39
0000871B  07                pop es
0000871C  E8D4FF            call 0x86f3
0000871F  81E20080          and dx,0x8000
00008723  0BC2              or ax,dx
00008725  5B                pop bx
00008726  07                pop es
00008727  8907              mov [bx],ax
00008729  93                xchg ax,bx
0000872A  C606340D02        mov byte [0xd34],0x2
0000872F  C3                ret
00008730  C7060C0AEC18      mov word [0xa0c],0x18ec
00008736  C706220A1B19      mov word [0xa22],0x191b
0000873C  C706420A2B19      mov word [0xa42],0x192b
00008742  C706640A1419      mov word [0xa64],0x1914
00008748  C7064807821B      mov word [0x748],0x1b82
0000874E  C7064607991B      mov word [0x746],0x1b99
00008754  C706FC098C1E      mov word [0x9fc],0x1e8c
0000875A  CB                retf
0000875B  00C7              add bh,al
0000875D  06                push es
0000875E  42                inc dx
0000875F  07                pop es
00008760  27                daa
00008761  1DCB00            sbb ax,0xcb
00008764  C7064007D322      mov word [0x740],0x22d3
0000876A  C7067809A722      mov word [0x978],0x22a7
00008770  CB                retf
00008771  00C7              add bh,al
00008773  06                push es
00008774  0A0A              or cl,[bp+si]
00008776  57                push di
00008777  28C7              sub bh,al
00008779  06                push es
0000877A  1E                push ds
0000877B  0A10              or dl,[bx+si]
0000877D  2EC706260AAD28    mov word [cs:0xa26],0x28ad
00008784  C706460ABB28      mov word [0xa46],0x28bb
0000878A  CB                retf
0000878B  BE080A            mov si,0xa08
0000878E  9AA1285802        call word 0x258:word 0x28a1
00008793  BE200A            mov si,0xa20
00008796  9AA1285802        call word 0x258:word 0x28a1
0000879B  8B36140D          mov si,[0xd14]
0000879F  8B04              mov ax,[si]
000087A1  8B5402            mov dx,[si+0x2]
000087A4  9A0B275802        call word 0x258:word 0x270b
000087A9  C7062E0D0000      mov word [0xd2e],0x0
000087AF  892E2A0D          mov [0xd2a],bp
000087B3  FF7402            push word [si+0x2]
000087B6  8B04              mov ax,[si]
000087B8  053000            add ax,0x30
000087BB  50                push ax
000087BC  CB                retf
000087BD  EA142A5802        jmp word 0x258:word 0x2a14
000087C2  C706100A622B      mov word [0xa10],0x2b62
000087C8  C7061C0AA22B      mov word [0xa1c],0x2ba2
000087CE  C706280A832B      mov word [0xa28],0x2b83
000087D4  C706500A842B      mov word [0xa50],0x2b84
000087DA  C7065C0A972B      mov word [0xa5c],0x2b97
000087E0  CB                retf
000087E1  00C7              add bh,al
000087E3  06                push es
000087E4  250BE3            and ax,0xe30b
000087E7  42                inc dx
000087E8  CB                retf
000087E9  00B85C4A          add [bx+si+0x4a5c],bh
000087ED  A33F0B            mov [0xb3f],ax
000087F0  A33D0B            mov [0xb3d],ax
000087F3  CB                retf
000087F4  C7063E0A8F18      mov word [0xa3e],0x188f
000087FA  C706380A8F18      mov word [0xa38],0x188f
00008800  C7064E0AEB2B      mov word [0xa4e],0x2beb
00008806  C7065E0A102E      mov word [0xa5e],0x2e10
0000880C  C706620A5852      mov word [0xa62],0x5258
00008812  C706410B9D13      mov word [0xb41],0x139d
00008818  C7068C0AB918      mov word [0xa8c],0x18b9
0000881E  CB                retf
0000881F  00C7              add bh,al
00008821  06                push es
00008822  300A              xor [bp+si],cl
00008824  7652              jna 0x8878
00008826  C706020AFF54      mov word [0xa02],0x54ff
0000882C  C706FE099C52      mov word [0x9fe],0x529c
00008832  C706000AA752      mov word [0xa00],0x52a7
00008838  C7067209B952      mov word [0x972],0x52b9
0000883E  C7061C0BD10D      mov word [0xb1c],0xdd1
00008844  C7061E0B080E      mov word [0xb1e],0xe08
0000884A  CB                retf
0000884B  0000              add [bx+si],al
0000884D  0000              add [bx+si],al
0000884F  0000              add [bx+si],al
00008851  0000              add [bx+si],al
00008853  0000              add [bx+si],al
00008855  0000              add [bx+si],al
00008857  0000              add [bx+si],al
00008859  0000              add [bx+si],al
0000885B  0000              add [bx+si],al
0000885D  0000              add [bx+si],al
0000885F  00558B            add [di-0x75],dl
00008862  EC                in al,dx
00008863  56                push si
00008864  57                push di
00008865  06                push es
00008866  837E0A00          cmp word [bp+0xa],0x0
0000886A  7538              jnz 0x88a4
0000886C  BF680C            mov di,0xc68
0000886F  8B5608            mov dx,[bp+0x8]
00008872  8B4606            mov ax,[bp+0x6]
00008875  48                dec ax
00008876  7507              jnz 0x887f
00008878  E85900            call 0x88d4
0000887B  7227              jc 0x88a4
0000887D  EB4E              jmp 0x88cd
0000887F  8B36B80C          mov si,[0xcb8]
00008883  48                dec ax
00008884  7411              jz 0x8897
00008886  3BF7              cmp si,di
00008888  740D              jz 0x8897
0000888A  8B4402            mov ax,[si+0x2]
0000888D  89460E            mov [bp+0xe],ax
00008890  56                push si
00008891  E84000            call 0x88d4
00008894  5E                pop si
00008895  7336              jnc 0x88cd
00008897  83C604            add si,0x4
0000889A  81FEB80C          cmp si,0xcb8
0000889E  7304              jnc 0x88a4
000088A0  0BD2              or dx,dx
000088A2  7506              jnz 0x88aa
000088A4  B8FFFF            mov ax,0xffff
000088A7  99                cwd
000088A8  EB23              jmp 0x88cd
000088AA  8BDA              mov bx,dx
000088AC  83C30F            add bx,0xf
000088AF  D1DB              rcr bx,1
000088B1  B103              mov cl,0x3
000088B3  D3EB              shr bx,cl
000088B5  B448              mov ah,0x48
000088B7  CD21              int byte 0x21
000088B9  72E9              jc 0x88a4
000088BB  3B06530D          cmp ax,[0xd53]
000088BF  76F4              jna 0x88b5
000088C1  92                xchg ax,dx
000088C2  8904              mov [si],ax
000088C4  895402            mov [si+0x2],dx
000088C7  8936B80C          mov [0xcb8],si
000088CB  33C0              xor ax,ax
000088CD  07                pop es
000088CE  5F                pop di
000088CF  5E                pop si
000088D0  8BE5              mov sp,bp
000088D2  5D                pop bp
000088D3  CB                retf
000088D4  8B4E0E            mov cx,[bp+0xe]
000088D7  8BF7              mov si,di
000088D9  394C02            cmp [si+0x2],cx
000088DC  740C              jz 0x88ea
000088DE  83C604            add si,0x4
000088E1  81FEB80C          cmp si,0xcb8
000088E5  75F2              jnz 0x88d9
000088E7  F9                stc
000088E8  EB2C              jmp 0x8916
000088EA  8BDA              mov bx,dx
000088EC  031C              add bx,[si]
000088EE  7226              jc 0x8916
000088F0  8BD3              mov dx,bx
000088F2  8EC1              mov es,cx
000088F4  3BF7              cmp si,di
000088F6  7508              jnz 0x8900
000088F8  391E180D          cmp [0xd18],bx
000088FC  7313              jnc 0x8911
000088FE  EB16              jmp 0x8916
00008900  83C30F            add bx,0xf
00008903  D1DB              rcr bx,1
00008905  D1EB              shr bx,1
00008907  D1EB              shr bx,1
00008909  D1EB              shr bx,1
0000890B  B44A              mov ah,0x4a
0000890D  CD21              int byte 0x21
0000890F  7205              jc 0x8916
00008911  92                xchg ax,dx
00008912  8704              xchg ax,[si]
00008914  8BD1              mov dx,cx
00008916  C3                ret
00008917  00B430CD          add [si-0x32d0],dh
0000891B  213C              and [si],di
0000891D  027302            add dh,[bp+di+0x2]
00008920  CD20              int byte 0x20
00008922  BF470B            mov di,0xb47
00008925  8B360200          mov si,[0x2]
00008929  2BF7              sub si,di
0000892B  81FE0010          cmp si,0x1000
0000892F  7203              jc 0x8934
00008931  BE0010            mov si,0x1000
00008934  FA                cli
00008935  8ED7              mov ss,di
00008937  81C45E0D          add sp,0xd5e
0000893B  FB                sti
0000893C  730B              jnc 0x8949
0000893E  33C0              xor ax,ax
00008940  36C706660CC102    mov word [ss:0xc66],0x2c1
00008947  EB7D              jmp 0x89c6
00008949  83E4FE            and sp,0xfffffffffffffffe
0000894C  368926640C        mov [ss:0xc64],sp
00008951  8BC6              mov ax,si
00008953  B104              mov cl,0x4
00008955  D3E0              shl ax,cl
00008957  48                dec ax
00008958  36A3620C          mov [ss:0xc62],ax
0000895C  03F7              add si,di
0000895E  89360200          mov [0x2],si
00008962  8CC3              mov bx,es
00008964  2BDE              sub bx,si
00008966  F7DB              neg bx
00008968  B44A              mov ah,0x4a
0000896A  CD21              int byte 0x21
0000896C  368C1ED90C        mov word [ss:0xcd9],ds
00008971  A12C00            mov ax,[0x2c]
00008974  36A3080D          mov [ss:0xd08],ax
00008978  368C1E060D        mov word [ss:0xd06],ds
0000897D  36C706040D8100    mov word [ss:0xd04],0x81
00008984  16                push ss
00008985  07                pop es
00008986  FC                cld
00008987  BF560D            mov di,0xd56
0000898A  B9600D            mov cx,0xd60
0000898D  2BCF              sub cx,di
0000898F  33C0              xor ax,ax
00008991  F3AA              rep stosb
00008993  16                push ss
00008994  1F                pop ds
00008995  C706660C5E02      mov word [0xc66],0x25e
0000899B  A1160D            mov ax,[0xd16]
0000899E  A3680C            mov [0xc68],ax
000089A1  9A32285802        call word 0x258:word 0x2832
000089A6  9A7C01E507        call word 0x7e5:word 0x17c
000089AB  16                push ss
000089AC  1F                pop ds
000089AD  16                push ss
000089AE  07                pop es
000089AF  33ED              xor bp,bp
000089B1  9A5B00D307        call word 0x7d3:word 0x5b
000089B6  50                push ax
000089B7  9A5E02E507        call word 0x7e5:word 0x25e
000089BC  B80300            mov ax,0x3
000089BF  36C706660C5E02    mov word [ss:0xc66],0x25e
000089C6  EA1003E507        jmp word 0x7e5:word 0x310
000089CB  00B430CD          add [si-0x32d0],dh
000089CF  21A3DB0C          and [bp+di+0xcdb],sp
000089D3  B80035            mov ax,0x3500
000089D6  CD21              int byte 0x21
000089D8  891EC70C          mov [0xcc7],bx
000089DC  8C06C90C          mov word [0xcc9],es
000089E0  0E                push cs
000089E1  1F                pop ds
000089E2  B80025            mov ax,0x2500
000089E5  BA6C01            mov dx,0x16c
000089E8  CD21              int byte 0x21
000089EA  16                push ss
000089EB  1F                pop ds
000089EC  8B0E4A0C          mov cx,[0xc4a]
000089F0  E32E              jcxz 0x8a20
000089F2  8E06D90C          mov es,word [0xcd9]
000089F6  268B362C00        mov si,[es:0x2c]
000089FB  C5064C0C          lds ax,word [0xc4c]
000089FF  8CDA              mov dx,ds
00008A01  33DB              xor bx,bx
00008A03  36FF1E480C        call word far [ss:0xc48]
00008A08  7305              jnc 0x8a0f
00008A0A  16                push ss
00008A0B  1F                pop ds
00008A0C  E98D01            jmp 0x8b9c
00008A0F  36C506500C        lds ax,word [ss:0xc50]
00008A14  8CDA              mov dx,ds
00008A16  BB0300            mov bx,0x3
00008A19  36FF1E480C        call word far [ss:0xc48]
00008A1E  16                push ss
00008A1F  1F                pop ds
00008A20  F606060A03        test byte [0xa06],0x3
00008A25  7406              jz 0x8a2d
00008A27  C606060A00        mov byte [0xa06],0x0
00008A2C  CB                retf
00008A2D  8E06D90C          mov es,word [0xcd9]
00008A31  268B0E2C00        mov cx,[es:0x2c]
00008A36  E336              jcxz 0x8a6e
00008A38  8EC1              mov es,cx
00008A3A  33FF              xor di,di
00008A3C  26803D00          cmp byte [es:di],0x0
00008A40  742C              jz 0x8a6e
00008A42  B90C00            mov cx,0xc
00008A45  BEBA0C            mov si,0xcba
00008A48  F3A6              repe cmpsb
00008A4A  740B              jz 0x8a57
00008A4C  B9FF7F            mov cx,0x7fff
00008A4F  33C0              xor ax,ax
00008A51  F2AE              repne scasb
00008A53  7519              jnz 0x8a6e
00008A55  EBE5              jmp 0x8a3c
00008A57  06                push es
00008A58  1E                push ds
00008A59  07                pop es
00008A5A  1F                pop ds
00008A5B  8BF7              mov si,di
00008A5D  BFE20C            mov di,0xce2
00008A60  AC                lodsb
00008A61  98                cbw
00008A62  91                xchg ax,cx
00008A63  AC                lodsb
00008A64  FEC0              inc al
00008A66  7401              jz 0x8a69
00008A68  48                dec ax
00008A69  AA                stosb
00008A6A  E2F7              loop 0x8a63
00008A6C  16                push ss
00008A6D  1F                pop ds
00008A6E  BB0400            mov bx,0x4
00008A71  80A7E20CBF        and byte [bx+0xce2],0xbf
00008A76  B80044            mov ax,0x4400
00008A79  CD21              int byte 0x21
00008A7B  720A              jc 0x8a87
00008A7D  F6C280            test dl,0x80
00008A80  7405              jz 0x8a87
00008A82  808FE20C40        or byte [bx+0xce2],0x40
00008A87  4B                dec bx
00008A88  79E7              jns 0x8a71
00008A8A  BE540C            mov si,0xc54
00008A8D  BF540C            mov di,0xc54
00008A90  E8AF00            call 0x8b42
00008A93  BE160C            mov si,0xc16
00008A96  BF3A0C            mov di,0xc3a
00008A99  E8A600            call 0x8b42
00008A9C  CB                retf
00008A9D  C606060A02        mov byte [0xa06],0x2
00008AA2  E92EFF            jmp 0x89d3
00008AA5  56                push si
00008AA6  57                push di
00008AA7  C606060A04        mov byte [0xa06],0x4
00008AAC  EB03              jmp 0x8ab1
00008AAE  55                push bp
00008AAF  8BEC              mov bp,sp
00008AB1  BE560D            mov si,0xd56
00008AB4  BF560D            mov di,0xd56
00008AB7  E88800            call 0x8b42
00008ABA  BE540C            mov si,0xc54
00008ABD  BF540C            mov di,0xc54
00008AC0  E87F00            call 0x8b42
00008AC3  813E3A0CD6D6      cmp word [0xc3a],0xd6d6
00008AC9  7504              jnz 0x8acf
00008ACB  FF16400C          call word near [0xc40]
00008ACF  EB03              jmp 0x8ad4
00008AD1  55                push bp
00008AD2  8BEC              mov bp,sp
00008AD4  BE540C            mov si,0xc54
00008AD7  BF580C            mov di,0xc58
00008ADA  E86500            call 0x8b42
00008ADD  BE580C            mov si,0xc58
00008AE0  BF580C            mov di,0xc58
00008AE3  E85C00            call 0x8b42
00008AE6  9A1A03E507        call word 0x7e5:word 0x31a
00008AEB  0BC0              or ax,ax
00008AED  740B              jz 0x8afa
00008AEF  837E0600          cmp word [bp+0x6],0x0
00008AF3  7505              jnz 0x8afa
00008AF5  C74606FF00        mov word [bp+0x6],0xff
00008AFA  9AC502E507        call word 0x7e5:word 0x2c5
00008AFF  F606060A04        test byte [0xa06],0x4
00008B04  7408              jz 0x8b0e
00008B06  C606060A00        mov byte [0xa06],0x0
00008B0B  5F                pop di
00008B0C  5E                pop si
00008B0D  CB                retf
00008B0E  8B4606            mov ax,[bp+0x6]
00008B11  B44C              mov ah,0x4c
00008B13  CD21              int byte 0x21
00008B15  8B0E4A0C          mov cx,[0xc4a]
00008B19  E307              jcxz 0x8b22
00008B1B  BB0200            mov bx,0x2
00008B1E  FF1E480C          call word far [0xc48]
00008B22  1E                push ds
00008B23  C516C70C          lds dx,word [0xcc7]
00008B27  B80025            mov ax,0x2500
00008B2A  CD21              int byte 0x21
00008B2C  1F                pop ds
00008B2D  803E0A0D00        cmp byte [0xd0a],0x0
00008B32  740D              jz 0x8b41
00008B34  1E                push ds
00008B35  A00B0D            mov al,[0xd0b]
00008B38  C5160C0D          lds dx,word [0xd0c]
00008B3C  B425              mov ah,0x25
00008B3E  CD21              int byte 0x21
00008B40  1F                pop ds
00008B41  CB                retf
00008B42  3BF7              cmp si,di
00008B44  730E              jnc 0x8b54
00008B46  83EF04            sub di,0x4
00008B49  8B05              mov ax,[di]
00008B4B  0B4502            or ax,[di+0x2]
00008B4E  74F2              jz 0x8b42
00008B50  FF1D              call word far [di]
00008B52  EBEE              jmp 0x8b42
00008B54  C3                ret
00008B55  0000              add [bx+si],al
00008B57  0000              add [bx+si],al
00008B59  0000              add [bx+si],al
00008B5B  0000              add [bx+si],al
00008B5D  0000              add [bx+si],al
00008B5F  0093B79A          add [bp+di-0x6549],dl
00008B63  16                push ss
00008B64  1F                pop ds
00008B65  EAC82A5802        jmp word 0x258:word 0x2ac8
00008B6A  33C0              xor ax,ax
00008B6C  CB                retf
00008B6D  00595A            add [bx+di+0x5a],bl
00008B70  8BDC              mov bx,sp
00008B72  2BD8              sub bx,ax
00008B74  720B              jc 0x8b81
00008B76  3B1E580C          cmp bx,[0xc58]
00008B7A  7205              jc 0x8b81
00008B7C  8BE3              mov sp,bx
00008B7E  52                push dx
00008B7F  51                push cx
00008B80  CB                retf
00008B81  A15A0C            mov ax,[0xc5a]
00008B84  40                inc ax
00008B85  7505              jnz 0x8b8c
00008B87  33C0              xor ax,ax
00008B89  E9D4FF            jmp 0x8b60
00008B8C  52                push dx
00008B8D  51                push cx
00008B8E  FF2E5A0C          jmp word far [0xc5a]
00008B92  55                push bp
00008B93  8BEC              mov bp,sp
00008B95  16                push ss
00008B96  1F                pop ds
00008B97  EAE2295802        jmp word 0x258:word 0x29e2
00008B9C  B80200            mov ax,0x2
00008B9F  E9BEFF            jmp 0x8b60
00008BA2  55                push bp
00008BA3  8BEC              mov bp,sp
00008BA5  56                push si
00008BA6  57                push di
00008BA7  06                push es
00008BA8  1E                push ds
00008BA9  B8CA0A            mov ax,0xaca
00008BAC  8ED8              mov ds,ax
00008BAE  1E                push ds
00008BAF  07                pop es
00008BB0  8B5606            mov dx,[bp+0x6]
00008BB3  BE0800            mov si,0x8
00008BB6  AD                lodsw
00008BB7  3BC2              cmp ax,dx
00008BB9  7410              jz 0x8bcb
00008BBB  40                inc ax
00008BBC  96                xchg ax,si
00008BBD  740C              jz 0x8bcb
00008BBF  97                xchg ax,di
00008BC0  33C0              xor ax,ax
00008BC2  B9FFFF            mov cx,0xffff
00008BC5  F2AE              repne scasb
00008BC7  8BF7              mov si,di
00008BC9  EBEB              jmp 0x8bb6
00008BCB  96                xchg ax,si
00008BCC  99                cwd
00008BCD  0BC0              or ax,ax
00008BCF  7402              jz 0x8bd3
00008BD1  8CDA              mov dx,ds
00008BD3  1F                pop ds
00008BD4  07                pop es
00008BD5  5F                pop di
00008BD6  5E                pop si
00008BD7  8BE5              mov sp,bp
00008BD9  5D                pop bp
00008BDA  CA0200            retf word 0x2
00008BDD  008B0E4A          add [bp+di+0x4a0e],cl
00008BE1  0CE3              or al,0xe3
00008BE3  11BB0100          adc [bp+di+0x1],di
00008BE7  FF1E480C          call word far [0xc48]
00008BEB  BB0400            mov bx,0x4
00008BEE  B83213            mov ax,0x1332
00008BF1  FF1E480C          call word far [0xc48]
00008BF5  CB                retf
00008BF6  0000              add [bx+si],al
00008BF8  0000              add [bx+si],al
00008BFA  0000              add [bx+si],al
00008BFC  0000              add [bx+si],al
00008BFE  0000              add [bx+si],al
00008C00  E8A600            call 0x8ca9
00008C03  CB                retf
00008C04  99                cwd
00008C05  55                push bp
00008C06  8BEC              mov bp,sp
00008C08  53                push bx
00008C09  52                push dx
00008C0A  50                push ax
00008C0B  8BDC              mov bx,sp
00008C0D  CD37              int byte 0x37
00008C0F  07                pop es
00008C10  83C404            add sp,0x4
00008C13  5B                pop bx
00008C14  8BE5              mov sp,bp
00008C16  5D                pop bp
00008C17  CB                retf
00008C18  55                push bp
00008C19  8BEC              mov bp,sp
00008C1B  83EC02            sub sp,0x2
00008C1E  CD3B              int byte 0x3b
00008C20  5E                pop si
00008C21  FECD              dec ch
00008C23  3D588B            cmp ax,0x8b58
00008C26  E55D              in ax,byte 0x5d
00008C28  CB                retf
00008C29  55                push bp
00008C2A  8BEC              mov bp,sp
00008C2C  83EC04            sub sp,0x4
00008C2F  CD37              int byte 0x37
00008C31  5E                pop si
00008C32  FC                cld
00008C33  CD3D              int byte 0x3d
00008C35  58                pop ax
00008C36  5A                pop dx
00008C37  8BE5              mov sp,bp
00008C39  5D                pop bp
00008C3A  CB                retf
00008C3B  55                push bp
00008C3C  8BEC              mov bp,sp
00008C3E  83EC04            sub sp,0x4
00008C41  CD37              int byte 0x37
00008C43  5E                pop si
00008C44  FC                cld
00008C45  CD3D              int byte 0x3d
00008C47  837EFE00          cmp word [bp-0x2],0x0
00008C4B  740A              jz 0x8c57
00008C4D  CD37              int byte 0x37
00008C4F  46                inc si
00008C50  FC                cld
00008C51  CD3B              int byte 0x3b
00008C53  5E                pop si
00008C54  FC                cld
00008C55  CD3D              int byte 0x3d
00008C57  58                pop ax
00008C58  83C402            add sp,0x2
00008C5B  8BE5              mov sp,bp
00008C5D  5D                pop bp
00008C5E  CB                retf
00008C5F  55                push bp
00008C60  8BEC              mov bp,sp
00008C62  CD3A              int byte 0x3a
00008C64  D9CD              fxch st5
00008C66  393E560B          cmp [0xb56],di
00008C6A  CD3D              int byte 0x3d
00008C6C  8A26570B          mov ah,[0xb57]
00008C70  9E                sahf
00008C71  8BE5              mov sp,bp
00008C73  5D                pop bp
00008C74  CB                retf
00008C75  06                push es
00008C76  050B06            add ax,0x60b
00008C79  06                push es
00008C7A  050505            add ax,0x505
00008C7D  06                push es
00008C7E  05052C            add ax,0x2c05
00008C81  81BB25042ED7      cmp word [bp+di+0x425],0xd72e
00008C87  98                cbw
00008C88  16                push ss
00008C89  1F                pop ds
00008C8A  83C408            add sp,0x8
00008C8D  803EDD0C01        cmp byte [0xcdd],0x1
00008C92  740C              jz 0x8ca0
00008C94  8BDC              mov bx,sp
00008C96  817F028708        cmp word [bx+0x2],0x887
00008C9B  7503              jnz 0x8ca0
00008C9D  83C406            add sp,0x6
00008CA0  55                push bp
00008CA1  8BEC              mov bp,sp
00008CA3  93                xchg ax,bx
00008CA4  EA762A5802        jmp word 0x258:word 0x2a76
00008CA9  CD35              int byte 0x35
00008CAB  3E8B0B            mov cx,[ds:bp+di]
00008CAE  CD3D              int byte 0x3d
00008CB0  50                push ax
00008CB1  A08B0B            mov al,[0xb8b]
00008CB4  A28D0B            mov [0xb8d],al
00008CB7  58                pop ax
00008CB8  CD35              int byte 0x35
00008CBA  2E8D0B            lea cx,[cs:bp+di]
00008CBD  FC                cld
00008CBE  893E5A0B          mov [0xb5a],di
00008CC2  89365C0B          mov [0xb5c],si
00008CC6  03CE              add cx,si
00008CC8  890E5E0B          mov [0xb5e],cx
00008CCC  A3660B            mov [0xb66],ax
00008CCF  8916680B          mov [0xb68],dx
00008CD3  33C9              xor cx,cx
00008CD5  0BDB              or bx,bx
00008CD7  7541              jnz 0x8d1a
00008CD9  C706640B0A00      mov word [0xb64],0xa
00008CDF  E88A00            call 0x8d6c
00008CE2  880E580B          mov [0xb58],cl
00008CE6  8B365C0B          mov si,[0xb5c]
00008CEA  C606880B00        mov byte [0xb88],0x0
00008CEF  E8B800            call 0x8daa
00008CF2  0A0E580B          or cl,[0xb58]
00008CF6  F706660BFFFF      test word [0xb66],0xffff
00008CFC  7506              jnz 0x8d04
00008CFE  F7C14218          test cx,0x1842
00008D02  7403              jz 0x8d07
00008D04  80C930            or cl,0x30
00008D07  A16E0B            mov ax,[0xb6e]
00008D0A  8B1E700B          mov bx,[0xb70]
00008D0E  8A2E880B          mov ch,[0xb88]
00008D12  CD35              int byte 0x35
00008D14  2E8B0B            mov cx,[cs:bp+di]
00008D17  CD3D              int byte 0x3d
00008D19  C3                ret
00008D1A  891E640B          mov [0xb64],bx
00008D1E  E87402            call 0x8f95
00008D21  890E580B          mov [0xb58],cx
00008D25  8B3E5A0B          mov di,[0xb5a]
00008D29  CD37              int byte 0x37
00008D2B  06                push es
00008D2C  6E                outsb
00008D2D  0BCD              or cx,bp
00008D2F  391D              cmp [di],bx
00008D31  8B0E580B          mov cx,[0xb58]
00008D35  EBBF              jmp 0x8cf6
00008D37  58                pop ax
00008D38  80C930            or cl,0x30
00008D3B  EB26              jmp 0x8d63
00008D3D  80C930            or cl,0x30
00008D40  58                pop ax
00008D41  9E                sahf
00008D42  7517              jnz 0x8d5b
00008D44  F7DB              neg bx
00008D46  F7DF              neg di
00008D48  83DB00            sbb bx,0x0
00008D4B  8BD3              mov dx,bx
00008D4D  0BD7              or dx,di
00008D4F  7412              jz 0x8d63
00008D51  80C910            or cl,0x10
00008D54  F6C780            test bh,0x80
00008D57  750A              jnz 0x8d63
00008D59  EB05              jmp 0x8d60
00008D5B  F6C780            test bh,0x80
00008D5E  7403              jz 0x8d63
00008D60  80C920            or cl,0x20
00008D63  893E6E0B          mov [0xb6e],di
00008D67  891E700B          mov [0xb70],bx
00008D6B  C3                ret
00008D6C  E8A202            call 0x9011
00008D6F  9F                lahf
00008D70  50                push ax
00008D71  33C9              xor cx,cx
00008D73  8BD9              mov bx,cx
00008D75  E8AD02            call 0x9025
00008D78  72BD              jc 0x8d37
00008D7A  8BF8              mov di,ax
00008D7C  E8A602            call 0x9025
00008D7F  72BF              jc 0x8d40
00008D81  8BD3              mov dx,bx
00008D83  8BEF              mov bp,di
00008D85  D1E7              shl di,1
00008D87  D1D3              rcl bx,1
00008D89  D0D5              rcl ch,1
00008D8B  D1E7              shl di,1
00008D8D  D1D3              rcl bx,1
00008D8F  D0D5              rcl ch,1
00008D91  03FD              add di,bp
00008D93  13DA              adc bx,dx
00008D95  80D500            adc ch,0x0
00008D98  D1E7              shl di,1
00008D9A  D1D3              rcl bx,1
00008D9C  D0D5              rcl ch,1
00008D9E  03F8              add di,ax
00008DA0  83D300            adc bx,0x0
00008DA3  80D500            adc ch,0x0
00008DA6  74D4              jz 0x8d7c
00008DA8  EB93              jmp 0x8d3d
00008DAA  33C9              xor cx,cx
00008DAC  890E600B          mov [0xb60],cx
00008DB0  C706620BEEFF      mov word [0xb62],0xffee
00008DB6  E85802            call 0x9011
00008DB9  7503              jnz 0x8dbe
00008DBB  80CD80            or ch,0x80
00008DBE  E81601            call 0x8ed7
00008DC1  32C9              xor cl,cl
00008DC3  33DB              xor bx,bx
00008DC5  E8C202            call 0x908a
00008DC8  746F              jz 0x8e39
00008DCA  4E                dec si
00008DCB  3C44              cmp al,0x44
00008DCD  7440              jz 0x8e0f
00008DCF  3C45              cmp al,0x45
00008DD1  7433              jz 0x8e06
00008DD3  803E6C0B00        cmp byte [0xb6c],0x0
00008DD8  745F              jz 0x8e39
00008DDA  3C2B              cmp al,0x2b
00008DDC  7404              jz 0x8de2
00008DDE  3C2D              cmp al,0x2d
00008DE0  7557              jnz 0x8e39
00008DE2  4E                dec si
00008DE3  EB24              jmp 0x8e09
00008DE5  803E6A0B00        cmp byte [0xb6a],0x0
00008DEA  7416              jz 0x8e02
00008DEC  46                inc si
00008DED  E89A02            call 0x908a
00008DF0  4E                dec si
00008DF1  4E                dec si
00008DF2  3C2B              cmp al,0x2b
00008DF4  740C              jz 0x8e02
00008DF6  3C2D              cmp al,0x2d
00008DF8  7408              jz 0x8e02
00008DFA  3C39              cmp al,0x39
00008DFC  7705              ja 0x8e03
00008DFE  3C30              cmp al,0x30
00008E00  7201              jc 0x8e03
00008E02  C3                ret
00008E03  58                pop ax
00008E04  EB33              jmp 0x8e39
00008E06  E8DCFF            call 0x8de5
00008E09  81C90204          or cx,0x402
00008E0D  EB06              jmp 0x8e15
00008E0F  E8D3FF            call 0x8de5
00008E12  80C90E            or cl,0xe
00008E15  C706660B0000      mov word [0xb66],0x0
00008E1B  46                inc si
00008E1C  E8F201            call 0x9011
00008E1F  9F                lahf
00008E20  50                push ax
00008E21  E8D001            call 0x8ff4
00008E24  F6C502            test ch,0x2
00008E27  750A              jnz 0x8e33
00008E29  803E6A0B00        cmp byte [0xb6a],0x0
00008E2E  7503              jnz 0x8e33
00008E30  80C940            or cl,0x40
00008E33  58                pop ax
00008E34  9E                sahf
00008E35  7502              jnz 0x8e39
00008E37  F7DB              neg bx
00008E39  F6C501            test ch,0x1
00008E3C  740D              jz 0x8e4b
00008E3E  80E57F            and ch,0x7f
00008E41  33DB              xor bx,bx
00008E43  891E620B          mov [0xb62],bx
00008E47  891E660B          mov [0xb66],bx
00008E4B  8BFB              mov di,bx
00008E4D  033E620B          add di,[0xb62]
00008E51  033E660B          add di,[0xb66]
00008E55  F6C510            test ch,0x10
00008E58  7504              jnz 0x8e5e
00008E5A  2B3E680B          sub di,[0xb68]
00008E5E  81FF3A01          cmp di,0x13a
00008E62  7E03              jng 0x8e67
00008E64  BF3A01            mov di,0x13a
00008E67  81FFA6FE          cmp di,0xfea6
00008E6B  7D03              jnl 0x8e70
00008E6D  BFA6FE            mov di,0xfea6
00008E70  E8BF03            call 0x9232
00008E73  8B3E5A0B          mov di,[0xb5a]
00008E77  833E600B07        cmp word [0xb60],0x7
00008E7C  7603              jna 0x8e81
00008E7E  80C908            or cl,0x8
00008E81  CD35              int byte 0x35
00008E83  C0CD35            ror ch,byte 0x35
00008E86  E1CD              loope 0x8e55
00008E88  37                aaa
00008E89  2E7E0B            {pn} jng 0x8e97
00008E8C  CD3A              int byte 0x3a
00008E8E  D9CD              fxch st5
00008E90  393E890B          cmp [0xb89],di
00008E94  CD3D              int byte 0x3d
00008E96  F6068A0B41        test byte [0xb8a],0x41
00008E9B  7522              jnz 0x8ebf
00008E9D  CD37              int byte 0x37
00008E9F  E2CD              loop 0x8e6e
00008EA1  391D              cmp [di],bx
00008EA3  CD39              int byte 0x39
00008EA5  3E890B            mov [ds:bp+di],cx
00008EA8  CD3D              int byte 0x3d
00008EAA  F606890B10        test byte [0xb89],0x10
00008EAF  7405              jz 0x8eb6
00008EB1  800E880B01        or byte [0xb88],0x1
00008EB6  51                push cx
00008EB7  80E580            and ch,0x80
00008EBA  086D07            or [di+0x7],ch
00008EBD  59                pop cx
00008EBE  C3                ret
00008EBF  8B3E5A0B          mov di,[0xb5a]
00008EC3  CD39              int byte 0x39
00008EC5  D85733            fcom dword [bx+0x33]
00008EC8  C0ABABAB8A        shr byte [bp+di-0x5455],byte 0x8a
00008ECD  E50D              in ax,byte 0xd
00008ECF  F0                lock
00008ED0  7FAB              jg 0x8e7d
00008ED2  5F                pop di
00008ED3  80C901            or cl,0x1
00008ED6  C3                ret
00008ED7  33FF              xor di,di
00008ED9  8BEF              mov bp,di
00008EDB  8BDF              mov bx,di
00008EDD  8BD7              mov dx,di
00008EDF  8BC7              mov ax,di
00008EE1  E87A01            call 0x905e
00008EE4  7239              jc 0x8f1f
00008EE6  0AC0              or al,al
00008EE8  7509              jnz 0x8ef3
00008EEA  FF0E620B          dec word [0xb62]
00008EEE  EBF1              jmp 0x8ee1
00008EF0  E86B01            call 0x905e
00008EF3  50                push ax
00008EF4  E84400            call 0x8f3b
00008EF7  58                pop ax
00008EF8  03D0              add dx,ax
00008EFA  83D300            adc bx,0x0
00008EFD  83D500            adc bp,0x0
00008F00  83D700            adc di,0x0
00008F03  FEC1              inc cl
00008F05  80F912            cmp cl,0x12
00008F08  72E6              jc 0x8ef0
00008F0A  E85101            call 0x905e
00008F0D  73FB              jnc 0x8f0a
00008F0F  80E5FE            and ch,0xfe
00008F12  B83E40            mov ax,0x403e
00008F15  0BFF              or di,di
00008F17  780B              js 0x8f24
00008F19  48                dec ax
00008F1A  E83400            call 0x8f51
00008F1D  EBF6              jmp 0x8f15
00008F1F  80CD01            or ch,0x1
00008F22  33C0              xor ax,ax
00008F24  56                push si
00008F25  8BF7              mov si,di
00008F27  BF740B            mov di,0xb74
00008F2A  92                xchg ax,dx
00008F2B  AB                stosw
00008F2C  93                xchg ax,bx
00008F2D  AB                stosw
00008F2E  95                xchg ax,bp
00008F2F  AB                stosw
00008F30  96                xchg ax,si
00008F31  AB                stosw
00008F32  92                xchg ax,dx
00008F33  AB                stosw
00008F34  5E                pop si
00008F35  CD37              int byte 0x37
00008F37  2E740B            {pn} jz 0x8f45
00008F3A  C3                ret
00008F3B  57                push di
00008F3C  55                push bp
00008F3D  53                push bx
00008F3E  8BC2              mov ax,dx
00008F40  E80E00            call 0x8f51
00008F43  E80B00            call 0x8f51
00008F46  03D0              add dx,ax
00008F48  58                pop ax
00008F49  13D8              adc bx,ax
00008F4B  58                pop ax
00008F4C  13E8              adc bp,ax
00008F4E  58                pop ax
00008F4F  13F8              adc di,ax
00008F51  D1E2              shl dx,1
00008F53  D1D3              rcl bx,1
00008F55  D1D5              rcl bp,1
00008F57  D1D7              rcl di,1
00008F59  C3                ret
00008F5A  58                pop ax
00008F5B  80C940            or cl,0x40
00008F5E  EB2C              jmp 0x8f8c
00008F60  58                pop ax
00008F61  80C930            or cl,0x30
00008F64  32ED              xor ch,ch
00008F66  58                pop ax
00008F67  9E                sahf
00008F68  751A              jnz 0x8f84
00008F6A  80CD80            or ch,0x80
00008F6D  F7DB              neg bx
00008F6F  F7DF              neg di
00008F71  83DB00            sbb bx,0x0
00008F74  8BD3              mov dx,bx
00008F76  0BD7              or dx,di
00008F78  7412              jz 0x8f8c
00008F7A  80C910            or cl,0x10
00008F7D  F6C780            test bh,0x80
00008F80  750A              jnz 0x8f8c
00008F82  EB05              jmp 0x8f89
00008F84  F6C780            test bh,0x80
00008F87  7403              jz 0x8f8c
00008F89  80C920            or cl,0x20
00008F8C  893E6E0B          mov [0xb6e],di
00008F90  891E700B          mov [0xb70],bx
00008F94  C3                ret
00008F95  E87900            call 0x9011
00008F98  9F                lahf
00008F99  50                push ax
00008F9A  33C9              xor cx,cx
00008F9C  8BD9              mov bx,cx
00008F9E  E88400            call 0x9025
00008FA1  72B7              jc 0x8f5a
00008FA3  8BF8              mov di,ax
00008FA5  E87D00            call 0x9025
00008FA8  72BA              jc 0x8f64
00008FAA  50                push ax
00008FAB  8BC7              mov ax,di
00008FAD  F726640B          mul word [0xb64]
00008FB1  8BF8              mov di,ax
00008FB3  8BEA              mov bp,dx
00008FB5  8BC3              mov ax,bx
00008FB7  F726640B          mul word [0xb64]
00008FBB  0BD2              or dx,dx
00008FBD  75A1              jnz 0x8f60
00008FBF  8BD8              mov bx,ax
00008FC1  03DD              add bx,bp
00008FC3  729B              jc 0x8f60
00008FC5  58                pop ax
00008FC6  03F8              add di,ax
00008FC8  83D300            adc bx,0x0
00008FCB  73D8              jnc 0x8fa5
00008FCD  EB92              jmp 0x8f61
00008FCF  CD37              int byte 0x37
00008FD1  E2CD              loop 0x8fa0
00008FD3  391D              cmp [di],bx
00008FD5  CD39              int byte 0x39
00008FD7  3E890B            mov [ds:bp+di],cx
00008FDA  CD3D              int byte 0x3d
00008FDC  F606890B10        test byte [0xb89],0x10
00008FE1  7405              jz 0x8fe8
00008FE3  800E880B01        or byte [0xb88],0x1
00008FE8  51                push cx
00008FE9  80E580            and ch,0x80
00008FEC  086D07            or [di+0x7],ch
00008FEF  59                pop cx
00008FF0  C3                ret
00008FF1  BBFF7F            mov bx,0x7fff
00008FF4  E82E00            call 0x9025
00008FF7  7217              jc 0x9010
00008FF9  81FBCC0C          cmp bx,0xccc
00008FFD  77F2              ja 0x8ff1
00008FFF  80CD02            or ch,0x2
00009002  D1E3              shl bx,1
00009004  03C3              add ax,bx
00009006  D1E3              shl bx,1
00009008  D1E3              shl bx,1
0000900A  03D8              add bx,ax
0000900C  78E3              js 0x8ff1
0000900E  EBE4              jmp 0x8ff4
00009010  C3                ret
00009011  E87600            call 0x908a
00009014  7409              jz 0x901f
00009016  3C2B              cmp al,0x2b
00009018  7405              jz 0x901f
0000901A  3C2D              cmp al,0x2d
0000901C  7403              jz 0x9021
0000901E  4E                dec si
0000901F  0CFF              or al,0xff
00009021  C3                ret
00009022  4E                dec si
00009023  F9                stc
00009024  C3                ret
00009025  E86200            call 0x908a
00009028  74F9              jz 0x9023
0000902A  2C30              sub al,0x30
0000902C  72F4              jc 0x9022
0000902E  3C09              cmp al,0x9
00009030  7E06              jng 0x9038
00009032  2C11              sub al,0x11
00009034  72EC              jc 0x9022
00009036  040A              add al,0xa
00009038  3A06640B          cmp al,[0xb64]
0000903C  7DE4              jnl 0x9022
0000903E  32E4              xor ah,ah
00009040  C3                ret
00009041  46                inc si
00009042  4E                dec si
00009043  33C0              xor ax,ax
00009045  80CD01            or ch,0x1
00009048  F9                stc
00009049  C3                ret
0000904A  F6C528            test ch,0x28
0000904D  75F3              jnz 0x9042
0000904F  800E580B80        or byte [0xb58],0x80
00009054  EBEF              jmp 0x9045
00009056  F6C510            test ch,0x10
00009059  75EF              jnz 0x904a
0000905B  80CD10            or ch,0x10
0000905E  F6C501            test ch,0x1
00009061  75DE              jnz 0x9041
00009063  E82400            call 0x908a
00009066  74DD              jz 0x9045
00009068  3C2E              cmp al,0x2e
0000906A  74EA              jz 0x9056
0000906C  2C30              sub al,0x30
0000906E  72D2              jc 0x9042
00009070  3C09              cmp al,0x9
00009072  77CE              ja 0x9042
00009074  B408              mov ah,0x8
00009076  F6C510            test ch,0x10
00009079  7506              jnz 0x9081
0000907B  FF06620B          inc word [0xb62]
0000907F  B420              mov ah,0x20
00009081  0AEC              or ch,ah
00009083  FF06600B          inc word [0xb60]
00009087  32E4              xor ah,ah
00009089  C3                ret
0000908A  3B365E0B          cmp si,[0xb5e]
0000908E  7323              jnc 0x90b3
00009090  AC                lodsb
00009091  803E6B0B00        cmp byte [0xb6b],0x0
00009096  7410              jz 0x90a8
00009098  3C20              cmp al,0x20
0000909A  74EE              jz 0x908a
0000909C  3C09              cmp al,0x9
0000909E  74EA              jz 0x908a
000090A0  3C0A              cmp al,0xa
000090A2  74E6              jz 0x908a
000090A4  3C0D              cmp al,0xd
000090A6  74E2              jz 0x908a
000090A8  3C61              cmp al,0x61
000090AA  7206              jc 0x90b2
000090AC  3C7A              cmp al,0x7a
000090AE  7702              ja 0x90b2
000090B0  245F              and al,0x5f
000090B2  C3                ret
000090B3  32C0              xor al,al
000090B5  C3                ret
000090B6  0000              add [bx+si],al
000090B8  0000              add [bx+si],al
000090BA  0000              add [bx+si],al
000090BC  00A00240          add [bx+si+0x4002],ah
000090C0  0000              add [bx+si],al
000090C2  0000              add [bx+si],al
000090C4  0000              add [bx+si],al
000090C6  00C8              add al,cl
000090C8  054000            add ax,0x40
000090CB  0000              add [bx+si],al
000090CD  0000              add [bx+si],al
000090CF  0000              add [bx+si],al
000090D1  FA                cli
000090D2  084000            or [bx+si+0x0],al
000090D5  0000              add [bx+si],al
000090D7  0000              add [bx+si],al
000090D9  00409C            add [bx+si-0x64],al
000090DC  0C40              or al,0x40
000090DE  0000              add [bx+si],al
000090E0  0000              add [bx+si],al
000090E2  0000              add [bx+si],al
000090E4  50                push ax
000090E5  C3                ret
000090E6  0F4000            cmovo ax,[bx+si]
000090E9  0000              add [bx+si],al
000090EB  0000              add [bx+si],al
000090ED  0024              add [si],ah
000090EF  F4                hlt
000090F0  124000            adc al,[bx+si+0x0]
000090F3  0000              add [bx+si],al
000090F5  0000              add [bx+si],al
000090F7  8096981640        adc byte [bp+0x1698],0x40
000090FC  0000              add [bx+si],al
000090FE  0000              add [bx+si],al
00009100  0020              add [bx+si],ah
00009102  BCBE19            mov sp,0x19be
00009105  40                inc ax
00009106  0000              add [bx+si],al
00009108  0004              add [si],al
0000910A  BFC91B            mov di,0x1bc9
0000910D  8E34              mov segr6,word [si]
0000910F  40                inc ax
00009110  00A1EDCC          add [bx+di-0x3313],ah
00009114  CE                into
00009115  1BC2              sbb ax,dx
00009117  D34E40            ror word [bp+0x40],cl
0000911A  9E                sahf
0000911B  B570              mov ch,0x70
0000911D  2BA8ADC5          sub bp,[bx+si-0x3a53]
00009121  9D                popf
00009122  6940FD25E5        imul ax,[bx+si-0x3],0xe525
00009127  1A8E4F19          sbb cl,[bp+0x194f]
0000912B  EB83              jmp 0x90b0
0000912D  40                inc ax
0000912E  D7                xlatb
0000912F  95                xchg ax,bp
00009130  43                inc bx
00009131  0E                push cs
00009132  058D29            add ax,0x298d
00009135  AF                scasw
00009136  9E                sahf
00009137  40                inc ax
00009138  A044ED            mov al,[0xed44]
0000913B  81128F81          adc word [bp+si],0x818f
0000913F  82                db 0x82
00009140  B940D5            mov cx,0xd540
00009143  A6                cmpsb
00009144  CF                iret
00009145  FF491F            dec word [bx+di+0x1f]
00009148  78C2              js 0x910c
0000914A  D340E0            rol word [bx+si-0x20],cl
0000914D  8CE9              mov cx,gs
0000914F  80C947            or cl,0x47
00009152  BA93A8            mov dx,0xa893
00009155  41                inc cx
00009156  6B552739          imul dx,[di+0x27],0x39
0000915A  8D                db 0x8d
0000915B  F770E0            div word [bx+si-0x20]
0000915E  7C42              jl 0x91a2
00009160  8EDE              mov ds,si
00009162  F9                stc
00009163  9D                popf
00009164  FB                sti
00009165  EB7E              jmp 0x91e5
00009167  AA                stosb
00009168  51                push cx
00009169  43                inc bx
0000916A  76E3              jna 0x914f
0000916C  CC                int3
0000916D  F2292F            repne sub [bx],bp
00009170  84812644          test [bx+di+0x4426],al
00009174  CDCC              int byte 0xcc
00009176  CC                int3
00009177  CC                int3
00009178  CC                int3
00009179  CC                int3
0000917A  CC                int3
0000917B  CC                int3
0000917C  FB                sti
0000917D  3F                aas
0000917E  0AD7              or dl,bh
00009180  A3703D            mov [0x3d70],ax
00009183  0AD7              or dl,bh
00009185  A3F83F            mov [0x3ff8],ax
00009188  3BDF              cmp bx,di
0000918A  4F                dec di
0000918B  8D976E12          lea dx,[bx+0x126e]
0000918F  83F53F            xor bp,0x3f
00009192  2C65              sub al,0x65
00009194  19E2              sbb dx,sp
00009196  58                pop ax
00009197  17                pop ss
00009198  B7D1              mov bh,0xd1
0000919A  F1                int1
0000919B  3F                aas
0000919C  2384471B          and ax,[si+0x1b47]
000091A0  47                inc di
000091A1  AC                lodsb
000091A2  C5A7EE3F          lds sp,word [bx+0x3fee]
000091A6  B669              mov dh,0x69
000091A8  6C                insb
000091A9  AF                scasw
000091AA  05BD37            add ax,0x37bd
000091AD  86EB              xchg ch,bl
000091AF  3F                aas
000091B0  BC427A            mov sp,0x7a42
000091B3  E5D5              in ax,byte 0xd5
000091B5  94                xchg ax,sp
000091B6  BFD6E7            mov di,0xe7d6
000091B9  3F                aas
000091BA  FD                std
000091BB  CE                into
000091BC  61                popa
000091BD  8411              test [bx+di],dl
000091BF  77CC              ja 0x918d
000091C1  AB                stosw
000091C2  E43F              in al,byte 0x3f
000091C4  5B                pop bx
000091C5  E14D              loope 0x9214
000091C7  C4BE9495          les di,word [bp-0x6a6c]
000091CB  E6C9              out byte 0xc9,al
000091CD  3F                aas
000091CE  53                push bx
000091CF  3B7544            cmp si,[di+0x44]
000091D2  CD14              int byte 0x14
000091D4  BE9AAF            mov si,0xaf9a
000091D7  3F                aas
000091D8  BA9439            mov dx,0x3994
000091DB  45                inc bp
000091DC  AD                lodsw
000091DD  1E                push ds
000091DE  B1CF              mov cl,0xcf
000091E0  94                xchg ax,sp
000091E1  3F                aas
000091E2  C6                db 0xc6
000091E3  E2BC              loop 0x91a1
000091E5  BA3B31            mov dx,0x313b
000091E8  61                popa
000091E9  8B7A3F            mov di,[bp+si+0x3f]
000091EC  59                pop cx
000091ED  C17EB153          sar word [bp-0x4f],byte 0x53
000091F1  7C12              jl 0x9205
000091F3  BB5F3F            mov bx,0x3f5f
000091F6  2F                das
000091F7  8D06BE92          lea ax,[0x92be]
000091FB  8515              test [di],dx
000091FD  FB                sti
000091FE  44                inc sp
000091FF  3F                aas
00009200  A5                movsw
00009201  E939A5            jmp 0x373d
00009204  27                daa
00009205  EA7FA82A3F        jmp word 0x3f2a:word 0xa87f
0000920A  A1E4BC            mov ax,[0xbce4]
0000920D  647C46            fs jl 0x9256
00009210  D0DD              rcr ch,1
00009212  55                push bp
00009213  3E06              ds push es
00009215  CC                int3
00009216  235477            and dx,[si+0x77]
00009219  83FF91            cmp di,0xffffffffffffff91
0000921C  813D3A19          cmp word [di],0x193a
00009220  7A63              jpe 0x9285
00009222  254331            and ax,0x3143
00009225  C0AC3CD138        shr byte [si-0x2ec4],byte 0x38
0000922A  82                db 0x82
0000922B  47                inc di
0000922C  97                xchg ax,di
0000922D  B800FD            mov ax,0xfd00
00009230  D7                xlatb
00009231  3BBB1608          cmp di,[bp+di+0x816]
00009235  0BFF              or di,di
00009237  7905              jns 0x923e
00009239  BBD408            mov bx,0x8d4
0000923C  F7DF              neg di
0000923E  57                push di
0000923F  5F                pop di
00009240  81C34600          add bx,0x46
00009244  0BFF              or di,di
00009246  7427              jz 0x926f
00009248  8BC7              mov ax,di
0000924A  D1EF              shr di,1
0000924C  D1EF              shr di,1
0000924E  D1EF              shr di,1
00009250  57                push di
00009251  250700            and ax,0x7
00009254  74E9              jz 0x923f
00009256  53                push bx
00009257  D0E0              shl al,1
00009259  8AE0              mov ah,al
0000925B  D0E0              shl al,1
0000925D  D0E0              shl al,1
0000925F  02C4              add al,ah
00009261  32E4              xor ah,ah
00009263  03D8              add bx,ax
00009265  CD3C              int byte 0x3c
00009267  9B2F              wait das
00009269  CD3A              int byte 0x3a
0000926B  C9                leave
0000926C  5B                pop bx
0000926D  EBD0              jmp 0x923f
0000926F  C3                ret
00009270  4D                dec bp
00009271  53                push bx
00009272  45                inc bp
00009273  4D                dec bp
00009274  3837              cmp [bx],dh
00009276  050A67            add ax,0x670a
00009279  66772E            ja 0x92aa
0000927C  2E2E47            cs inc di
0000927F  57                push di
00009280  3A00              cmp al,[bx+si]
00009282  E200              loop 0x9284
00009284  D200              rol byte [bx+si],cl
00009286  0301              add ax,[bx+di]
00009288  7C05              jl 0x928f
0000928A  95                xchg ax,bp
0000928B  05B705            add ax,0x5b7
0000928E  FC                cld
0000928F  059905            add ax,0x599
00009292  F1                int1
00009293  00D1              add cl,dl
00009295  E31E              jcxz 0x92b5
00009297  B9300B            mov cx,0xb30
0000929A  8ED9              mov ds,cx
0000929C  2EFF971000        call word near [cs:bx+0x10]
000092A1  1F                pop ds
000092A2  CB                retf
000092A3  4E                dec si
000092A4  4F                dec di
000092A5  3837              cmp [bx],dh
000092A7  3D0D0A            cmp ax,0xa0d
000092AA  1E                push ds
000092AB  0BF6              or si,si
000092AD  745F              jz 0x930e
000092AF  8EC6              mov es,si
000092B1  0E                push cs
000092B2  1F                pop ds
000092B3  33FF              xor di,di
000092B5  FC                cld
000092B6  26803D00          cmp byte [es:di],0x0
000092BA  7452              jz 0x930e
000092BC  B90500            mov cx,0x5
000092BF  BE3300            mov si,0x33
000092C2  F3A6              repe cmpsb
000092C4  740B              jz 0x92d1
000092C6  B9FF7F            mov cx,0x7fff
000092C9  32C0              xor al,al
000092CB  F2AE              repne scasb
000092CD  74E7              jz 0x92b6
000092CF  EB3D              jmp 0x930e
000092D1  B9FF7F            mov cx,0x7fff
000092D4  B020              mov al,0x20
000092D6  F3AE              repe scasb
000092D8  4F                dec di
000092D9  268A15            mov dl,[es:di]
000092DC  0AD2              or dl,dl
000092DE  7425              jz 0x9305
000092E0  33C0              xor ax,ax
000092E2  B9FF7F            mov cx,0x7fff
000092E5  8BDF              mov bx,di
000092E7  F2AE              repne scasb
000092E9  4F                dec di
000092EA  2BFB              sub di,bx
000092EC  8BCF              mov cx,di
000092EE  06                push es
000092EF  1F                pop ds
000092F0  8BD3              mov dx,bx
000092F2  BB0100            mov bx,0x1
000092F5  B440              mov ah,0x40
000092F7  CD21              int byte 0x21
000092F9  0E                push cs
000092FA  1F                pop ds
000092FB  BA3800            mov dx,0x38
000092FE  B90200            mov cx,0x2
00009301  B440              mov ah,0x40
00009303  CD21              int byte 0x21
00009305  1F                pop ds
00009306  C606040000        mov byte [0x4],0x0
0000930B  EB1C              jmp 0x9329
0000930D  90                nop
0000930E  1F                pop ds
0000930F  CD11              int byte 0x11
00009311  2402              and al,0x2
00009313  D0E8              shr al,1
00009315  A20400            mov [0x4],al
00009318  803E040000        cmp byte [0x4],0x0
0000931D  740A              jz 0x9329
0000931F  E80823            call 0xb62a
00009322  7305              jnc 0x9329
00009324  B8FEFF            mov ax,0xfffe
00009327  F9                stc
00009328  C3                ret
00009329  E84F00            call 0x937b
0000932C  E82300            call 0x9352
0000932F  B83213            mov ax,0x1332
00009332  E8B704            call 0x97ec
00009335  33C0              xor ax,ax
00009337  A30000            mov [0x0],ax
0000933A  A30200            mov [0x2],ax
0000933D  A00400            mov al,[0x4]
00009340  98                cbw
00009341  C3                ret
00009342  E87E00            call 0x93c3
00009345  803E040000        cmp byte [0x4],0x0
0000934A  7405              jz 0x9351
0000934C  DBE3              fninit
0000934E  E80823            call 0xb659
00009351  C3                ret
00009352  803E040000        cmp byte [0x4],0x0
00009357  7402              jz 0x935b
00009359  DBE3              fninit
0000935B  A10E00            mov ax,[0xe]
0000935E  A31000            mov [0x10],ax
00009361  33C0              xor ax,ax
00009363  3A060400          cmp al,[0x4]
00009367  7403              jz 0x936c
00009369  9BDBE2            fclex
0000936C  A30C00            mov [0xc],ax
0000936F  A30800            mov [0x8],ax
00009372  C3                ret
00009373  A30000            mov [0x0],ax
00009376  89160200          mov [0x2],dx
0000937A  C3                ret
0000937B  B90B00            mov cx,0xb
0000937E  B83435            mov ax,0x3534
00009381  BF1E00            mov di,0x1e
00009384  CD21              int byte 0x21
00009386  40                inc ax
00009387  891D              mov [di],bx
00009389  8C4502            mov word [di+0x2],es
0000938C  83C704            add di,0x4
0000938F  E2F3              loop 0x9384
00009391  BA1F0B            mov dx,0xb1f
00009394  BEDF0A            mov si,0xadf
00009397  BFC20A            mov di,0xac2
0000939A  803E040000        cmp byte [0x4],0x0
0000939F  7409              jz 0x93aa
000093A1  BAD606            mov dx,0x6d6
000093A4  BE8C06            mov si,0x68c
000093A7  BFB906            mov di,0x6b9
000093AA  1E                push ds
000093AB  0E                push cs
000093AC  1F                pop ds
000093AD  B83425            mov ax,0x2534
000093B0  B90800            mov cx,0x8
000093B3  CD21              int byte 0x21
000093B5  40                inc ax
000093B6  E2FB              loop 0x93b3
000093B8  8BD6              mov dx,si
000093BA  CD21              int byte 0x21
000093BC  40                inc ax
000093BD  8BD7              mov dx,di
000093BF  CD21              int byte 0x21
000093C1  1F                pop ds
000093C2  C3                ret
000093C3  B90B00            mov cx,0xb
000093C6  B83425            mov ax,0x2534
000093C9  BF1E00            mov di,0x1e
000093CC  1E                push ds
000093CD  C515              lds dx,word [di]
000093CF  CD21              int byte 0x21
000093D1  1F                pop ds
000093D2  40                inc ax
000093D3  83C704            add di,0x4
000093D6  E2F4              loop 0x93cc
000093D8  C3                ret
000093D9  800E170002        or byte [0x17],0x2
000093DE  803E040000        cmp byte [0x4],0x0
000093E3  7403              jz 0x93e8
000093E5  80CC02            or ah,0x2
000093E8  C3                ret
000093E9  800E170004        or byte [0x17],0x4
000093EE  C3                ret
000093EF  50                push ax
000093F0  55                push bp
000093F1  83EC1C            sub sp,0x1c
000093F4  8BEC              mov bp,sp
000093F6  D97600            fnstenv [bp+0x0]
000093F9  9BDBE2            fclex
000093FC  FB                sti
000093FD  53                push bx
000093FE  1E                push ds
000093FF  B8300B            mov ax,0xb30
00009402  8ED8              mov ds,ax
00009404  33C0              xor ax,ax
00009406  8A4602            mov al,[bp+0x2]
00009409  32FF              xor bh,bh
0000940B  F6C701            test bh,0x1
0000940E  750A              jnz 0x941a
00009410  A801              test al,0x1
00009412  7406              jz 0x941a
00009414  80CF01            or bh,0x1
00009417  E98400            jmp 0x949e
0000941A  A802              test al,0x2
0000941C  7403              jz 0x9421
0000941E  E9FE01            jmp 0x961f
00009421  F6C704            test bh,0x4
00009424  750C              jnz 0x9432
00009426  A804              test al,0x4
00009428  7408              jz 0x9432
0000942A  80CF04            or bh,0x4
0000942D  E8F102            call 0x9721
00009430  EBD9              jmp 0x940b
00009432  F6C708            test bh,0x8
00009435  750A              jnz 0x9441
00009437  A808              test al,0x8
00009439  7406              jz 0x9441
0000943B  80CF08            or bh,0x8
0000943E  E9CA02            jmp 0x970b
00009441  8A1E0600          mov bl,[0x6]
00009445  80CBC0            or bl,0xc0
00009448  80E3FC            and bl,0xfc
0000944B  F6D3              not bl
0000944D  22C3              and al,bl
0000944F  A801              test al,0x1
00009451  741F              jz 0x9472
00009453  50                push ax
00009454  8B4608            mov ax,[bp+0x8]
00009457  80E407            and ah,0x7
0000945A  3DFA01            cmp ax,0x1fa
0000945D  7505              jnz 0x9464
0000945F  58                pop ax
00009460  0C80              or al,0x80
00009462  EB0E              jmp 0x9472
00009464  8B4608            mov ax,[bp+0x8]
00009467  253803            and ax,0x338
0000946A  3D0001            cmp ax,0x100
0000946D  58                pop ax
0000946E  7502              jnz 0x9472
00009470  3401              xor al,0x1
00009472  9BDBE2            fclex
00009475  9BD96E00          wait fldcw [bp+0x0]
00009479  09060800          or [0x8],ax
0000947D  1F                pop ds
0000947E  5B                pop bx
0000947F  83C41C            add sp,0x1c
00009482  5D                pop bp
00009483  A9FFBF            test ax,0xbfff
00009486  7502              jnz 0x948a
00009488  58                pop ax
00009489  C3                ret
0000948A  50                push ax
0000948B  53                push bx
0000948C  8BDC              mov bx,sp
0000948E  368B4704          mov ax,[ss:bx+0x4]
00009492  36894706          mov [ss:bx+0x6],ax
00009496  5B                pop bx
00009497  58                pop ax
00009498  83C402            add sp,0x2
0000949B  E9D802            jmp 0x9776
0000949E  56                push si
0000949F  53                push bx
000094A0  51                push cx
000094A1  52                push dx
000094A2  57                push di
000094A3  8B5E04            mov bx,[bp+0x4]
000094A6  8B4E02            mov cx,[bp+0x2]
000094A9  F7D3              not bx
000094AB  8ACD              mov cl,ch
000094AD  80E138            and cl,0x38
000094B0  D0E9              shr cl,1
000094B2  D0E9              shr cl,1
000094B4  D3CB              ror bx,cl
000094B6  F6C7C0            test bh,0xc0
000094B9  7452              jz 0x950d
000094BB  8B5608            mov dx,[bp+0x8]
000094BE  81F2E001          xor dx,0x1e0
000094C2  F6C2C0            test dl,0xc0
000094C5  7520              jnz 0x94e7
000094C7  F7C22001          test dx,0x120
000094CB  7540              jnz 0x950d
000094CD  F6C210            test dl,0x10
000094D0  7508              jnz 0x94da
000094D2  F6C208            test dl,0x8
000094D5  7516              jnz 0x94ed
000094D7  EB34              jmp 0x950d
000094D9  90                nop
000094DA  80FA12            cmp dl,0x12
000094DD  740E              jz 0x94ed
000094DF  80FA14            cmp dl,0x14
000094E2  7409              jz 0x94ed
000094E4  EB27              jmp 0x950d
000094E6  90                nop
000094E7  F7C21001          test dx,0x110
000094EB  7520              jnz 0x950d
000094ED  80E73F            and bh,0x3f
000094F0  8B361000          mov si,[0x10]
000094F4  3B361200          cmp si,[0x12]
000094F8  7503              jnz 0x94fd
000094FA  E8DCFE            call 0x93d9
000094FD  83C60C            add si,0xc
00009500  89361000          mov [0x10],si
00009504  9BD9F6            wait fdecstp
00009507  9BDB3C            wait fstp tword [si]
0000950A  E90501            jmp 0x9612
0000950D  F6C303            test bl,0x3
00009510  7403              jz 0x9515
00009512  EB4A              jmp 0x955e
00009514  90                nop
00009515  8B5608            mov dx,[bp+0x8]
00009518  81F2E001          xor dx,0x1e0
0000951C  F6C2C0            test dl,0xc0
0000951F  7409              jz 0x952a
00009521  F7C21001          test dx,0x110
00009525  7503              jnz 0x952a
00009527  EB35              jmp 0x955e
00009529  90                nop
0000952A  0BDB              or bx,bx
0000952C  7406              jz 0x9534
0000952E  E8C000            call 0x95f1
00009531  EB2B              jmp 0x955e
00009533  90                nop
00009534  8B361000          mov si,[0x10]
00009538  3B360E00          cmp si,[0xe]
0000953C  7503              jnz 0x9541
0000953E  E9BE00            jmp 0x95ff
00009541  80CB03            or bl,0x3
00009544  9BD9F7            wait fincstp
00009547  9BDB2C            wait fld tword [si]
0000954A  8B361000          mov si,[0x10]
0000954E  3B360E00          cmp si,[0xe]
00009552  7503              jnz 0x9557
00009554  E892FE            call 0x93e9
00009557  83EE0C            sub si,0xc
0000955A  89361000          mov [0x10],si
0000955E  F6C30C            test bl,0xc
00009561  7550              jnz 0x95b3
00009563  8BF3              mov si,bx
00009565  83E6F0            and si,0xfffffffffffffff0
00009568  0BF6              or si,si
0000956A  741A              jz 0x9586
0000956C  9BDB3E4A00        wait fstp tword [0x4a]
00009571  D1EB              shr bx,1
00009573  D1EB              shr bx,1
00009575  E87900            call 0x95f1
00009578  9BDB2E4A00        wait fld tword [0x4a]
0000957D  D1E3              shl bx,1
0000957F  D1E3              shl bx,1
00009581  80CB03            or bl,0x3
00009584  EB2D              jmp 0x95b3
00009586  8B361000          mov si,[0x10]
0000958A  3B360E00          cmp si,[0xe]
0000958E  7423              jz 0x95b3
00009590  80CB0C            or bl,0xc
00009593  9BD9F7            wait fincstp
00009596  9BD9F7            wait fincstp
00009599  9BDB2C            wait fld tword [si]
0000959C  9BD9F6            wait fdecstp
0000959F  8B361000          mov si,[0x10]
000095A3  3B360E00          cmp si,[0xe]
000095A7  7503              jnz 0x95ac
000095A9  E83DFE            call 0x93e9
000095AC  83EE0C            sub si,0xc
000095AF  89361000          mov [0x10],si
000095B3  8B5608            mov dx,[bp+0x8]
000095B6  81F2E001          xor dx,0x1e0
000095BA  F6C2C0            test dl,0xc0
000095BD  750B              jnz 0x95ca
000095BF  F7C22001          test dx,0x120
000095C3  7519              jnz 0x95de
000095C5  F6C210            test dl,0x10
000095C8  7503              jnz 0x95cd
000095CA  EB46              jmp 0x9612
000095CC  90                nop
000095CD  8ACA              mov cl,dl
000095CF  80E10F            and cl,0xf
000095D2  BAC450            mov dx,0x50c4
000095D5  D3E2              shl dx,cl
000095D7  79F1              jns 0x95ca
000095D9  F6C30C            test bl,0xc
000095DC  75EC              jnz 0x95ca
000095DE  83E207            and dx,0x7
000095E1  D0E2              shl dl,1
000095E3  8BCA              mov cx,dx
000095E5  8BD3              mov dx,bx
000095E7  D3CA              ror dx,cl
000095E9  F6C203            test dl,0x3
000095EC  7524              jnz 0x9612
000095EE  EB0F              jmp 0x95ff
000095F0  90                nop
000095F1  D1CB              ror bx,1
000095F3  D1CB              ror bx,1
000095F5  9BD9F7            wait fincstp
000095F8  F7C30300          test bx,0x3
000095FC  74F3              jz 0x95f1
000095FE  C3                ret
000095FF  80CC04            or ah,0x4
00009602  C6460200          mov byte [bp+0x2],0x0
00009606  9BD96600          wait fldenv [bp+0x0]
0000960A  5F                pop di
0000960B  5A                pop dx
0000960C  59                pop cx
0000960D  5B                pop bx
0000960E  5E                pop si
0000960F  E967FE            jmp 0x9479
00009612  24FE              and al,0xfe
00009614  E80A01            call 0x9721
00009617  5F                pop di
00009618  5A                pop dx
00009619  59                pop cx
0000961A  5B                pop bx
0000961B  5E                pop si
0000961C  E9ECFD            jmp 0x940b
0000961F  800E080002        or byte [0x8],0x2
00009624  51                push cx
00009625  8B4E08            mov cx,[bp+0x8]
00009628  81E13803          and cx,0x338
0000962C  83F908            cmp cx,0x8
0000962F  740D              jz 0x963e
00009631  83F910            cmp cx,0x10
00009634  7408              jz 0x963e
00009636  80E130            and cl,0x30
00009639  83F930            cmp cx,0x30
0000963C  7554              jnz 0x9692
0000963E  F6C7C0            test bh,0xc0
00009641  741D              jz 0x9660
00009643  80E73F            and bh,0x3f
00009646  8B361000          mov si,[0x10]
0000964A  3B361200          cmp si,[0x12]
0000964E  7503              jnz 0x9653
00009650  E886FD            call 0x93d9
00009653  83C60C            add si,0xc
00009656  89361000          mov [0x10],si
0000965A  9BD9F6            wait fdecstp
0000965D  9BDB3C            wait fstp tword [si]
00009660  8B4E08            mov cx,[bp+0x8]
00009663  51                push cx
00009664  81E10004          and cx,0x400
00009668  81C10401          add cx,0x104
0000966C  894E08            mov [bp+0x8],cx
0000966F  E8B700            call 0x9729
00009672  E85400            call 0x96c9
00009675  59                pop cx
00009676  83E138            and cx,0x38
00009679  80F908            cmp cl,0x8
0000967C  7408              jz 0x9686
0000967E  80F910            cmp cl,0x10
00009681  7424              jz 0x96a7
00009683  80F108            xor cl,0x8
00009686  81C9C106          or cx,0x6c1
0000968A  894E08            mov [bp+0x8],cx
0000968D  E89100            call 0x9721
00009690  EB31              jmp 0x96c3
00009692  8B4E08            mov cx,[bp+0x8]
00009695  81E13801          and cx,0x138
00009699  81F90001          cmp cx,0x100
0000969D  7403              jz 0x96a2
0000969F  E87F00            call 0x9721
000096A2  E82400            call 0x96c9
000096A5  EB1C              jmp 0x96c3
000096A7  80CC40            or ah,0x40
000096AA  9BDBE2            fclex
000096AD  9BD9C9            wait fxch st1
000096B0  9BD8D1            wait fcom st1
000096B3  9BD9C9            wait fxch st1
000096B6  9BDDD8            wait fstp st0
000096B9  9BDD3E1600        fstsw [0x16]
000096BE  9B0A061600        wait or al,[0x16]
000096C3  59                pop cx
000096C4  24FD              and al,0xfd
000096C6  E942FD            jmp 0x940b
000096C9  9BDB7E12          wait fstp tword [bp+0x12]
000096CD  9B8B4E1A          wait mov cx,[bp+0x1a]
000096D1  F7C1FF7F          test cx,0x7fff
000096D5  7421              jz 0x96f8
000096D7  F6461980          test byte [bp+0x19],0x80
000096DB  7516              jnz 0x96f3
000096DD  9BDF6E12          wait fild qword [bp+0x12]
000096E1  9BDB7E12          wait fstp tword [bp+0x12]
000096E5  9B837E1A00        wait cmp word [bp+0x1a],0x0
000096EA  740C              jz 0x96f8
000096EC  81E93E40          sub cx,0x403e
000096F0  014E1A            add [bp+0x1a],cx
000096F3  9BDB6E12          wait fld tword [bp+0x12]
000096F7  C3                ret
000096F8  33C9              xor cx,cx
000096FA  894E12            mov [bp+0x12],cx
000096FD  894E14            mov [bp+0x14],cx
00009700  894E16            mov [bp+0x16],cx
00009703  894E18            mov [bp+0x18],cx
00009706  894E1A            mov [bp+0x1a],cx
00009709  EBE8              jmp 0x96f3
0000970B  50                push ax
0000970C  8B4608            mov ax,[bp+0x8]
0000970F  25C001            and ax,0x1c0
00009712  80F401            xor ah,0x1
00009715  3DC000            cmp ax,0xc0
00009718  58                pop ax
00009719  7303              jnc 0x971e
0000971B  E80300            call 0x9721
0000971E  E9EAFC            jmp 0x940b
00009721  F6C440            test ah,0x40
00009724  754F              jnz 0x9775
00009726  80CC40            or ah,0x40
00009729  1E                push ds
0000972A  57                push di
0000972B  56                push si
0000972C  51                push cx
0000972D  53                push bx
0000972E  9BDBE2            fclex
00009731  8B4E08            mov cx,[bp+0x8]
00009734  8BD9              mov bx,cx
00009736  80E507            and ch,0x7
00009739  80CDD8            or ch,0xd8
0000973C  80E3C0            and bl,0xc0
0000973F  80F3C0            xor bl,0xc0
00009742  7409              jz 0x974d
00009744  80E138            and cl,0x38
00009747  80C904            or cl,0x4
0000974A  C5760A            lds si,word [bp+0xa]
0000974D  86E9              xchg ch,cl
0000974F  8C5614            mov word [bp+0x14],ss
00009752  8D7E16            lea di,[bp+0x16]
00009755  897E12            mov [bp+0x12],di
00009758  C646169B          mov byte [bp+0x16],0x9b
0000975C  894E17            mov [bp+0x17],cx
0000975F  C64619CB          mov byte [bp+0x19],0xcb
00009763  FF5E12            call word far [bp+0x12]
00009766  5B                pop bx
00009767  59                pop cx
00009768  5E                pop si
00009769  5F                pop di
0000976A  1F                pop ds
0000976B  9BDD3E1600        fstsw [0x16]
00009770  9B0A061600        wait or al,[0x16]
00009775  C3                ret
00009776  1E                push ds
00009777  53                push bx
00009778  B38B              mov bl,0x8b
0000977A  F6C404            test ah,0x4
0000977D  7538              jnz 0x97b7
0000977F  B38A              mov bl,0x8a
00009781  F6C402            test ah,0x2
00009784  7531              jnz 0x97b7
00009786  B388              mov bl,0x88
00009788  A880              test al,0x80
0000978A  752B              jnz 0x97b7
0000978C  B389              mov bl,0x89
0000978E  F6C401            test ah,0x1
00009791  7524              jnz 0x97b7
00009793  B381              mov bl,0x81
00009795  A801              test al,0x1
00009797  751E              jnz 0x97b7
00009799  B383              mov bl,0x83
0000979B  A804              test al,0x4
0000979D  7518              jnz 0x97b7
0000979F  B384              mov bl,0x84
000097A1  A808              test al,0x8
000097A3  7512              jnz 0x97b7
000097A5  B385              mov bl,0x85
000097A7  A810              test al,0x10
000097A9  750C              jnz 0x97b7
000097AB  B386              mov bl,0x86
000097AD  A820              test al,0x20
000097AF  7506              jnz 0x97b7
000097B1  B387              mov bl,0x87
000097B3  A840              test al,0x40
000097B5  7500              jnz 0x97b7
000097B7  B8300B            mov ax,0xb30
000097BA  8ED8              mov ds,ax
000097BC  A10000            mov ax,[0x0]
000097BF  0B060200          or ax,[0x2]
000097C3  7505              jnz 0x97ca
000097C5  93                xchg ax,bx
000097C6  B44C              mov ah,0x4c
000097C8  CD21              int byte 0x21
000097CA  93                xchg ax,bx
000097CB  5B                pop bx
000097CC  803E040000        cmp byte [0x4],0x0
000097D1  7412              jz 0x97e5
000097D3  803E140000        cmp byte [0x14],0x0
000097D8  750F              jnz 0x97e9
000097DA  A21400            mov [0x14],al
000097DD  2EC606BB0690      mov byte [cs:0x6bb],0x90
000097E3  EB04              jmp 0x97e9
000097E5  FF1E0000          call word far [0x0]
000097E9  1F                pop ds
000097EA  58                pop ax
000097EB  CF                iret
000097EC  A30600            mov [0x6],ax
000097EF  253CFF            and ax,0xff3c
000097F2  803E040000        cmp byte [0x4],0x0
000097F7  7408              jz 0x9801
000097F9  A31A00            mov [0x1a],ax
000097FC  9BD92E1A00        wait fldcw [0x1a]
00009801  A30A00            mov [0xa],ax
00009804  C3                ret
00009805  A10600            mov ax,[0x6]
00009808  C3                ret
00009809  33C0              xor ax,ax
0000980B  3A060400          cmp al,[0x4]
0000980F  740B              jz 0x981c
00009811  9BDD3E1600        fstsw [0x16]
00009816  9BA01600          wait mov al,[0x16]
0000981A  243F              and al,0x3f
0000981C  0B060800          or ax,[0x8]
00009820  25FF1F            and ax,0x1fff
00009823  A30800            mov [0x8],ax
00009826  C3                ret
00009827  25000C            and ax,0xc00
0000982A  803E040000        cmp byte [0x4],0x0
0000982F  7420              jz 0x9851
00009831  9BD93E0A00        fstcw [0xa]
00009836  9B8B0E0A00        wait mov cx,[0xa]
0000983B  80E5F3            and ch,0xf3
0000983E  0BC1              or ax,cx
00009840  A31A00            mov [0x1a],ax
00009843  9BD92E1A00        wait fldcw [0x1a]
00009848  9BD9FC            wait frndint
0000984B  9BD92E0A00        wait fldcw [0xa]
00009850  C3                ret
00009851  8B0E0A00          mov cx,[0xa]
00009855  51                push cx
00009856  80E5F3            and ch,0xf3
00009859  0AE5              or ah,ch
0000985B  88260B00          mov [0xb],ah
0000985F  55                push bp
00009860  E8A414            call 0xad07
00009863  5D                pop bp
00009864  8F060A00          pop word [0xa]
00009868  E86700            call 0x98d2
0000986B  C3                ret
0000986C  25000C            and ax,0xc00
0000986F  803E040000        cmp byte [0x4],0x0
00009874  7429              jz 0x989f
00009876  9BD93E0A00        fstcw [0xa]
0000987B  9B8B0E0A00        wait mov cx,[0xa]
00009880  80E5F3            and ch,0xf3
00009883  0BC1              or ax,cx
00009885  A31A00            mov [0x1a],ax
00009888  9BD92E1A00        wait fldcw [0x1a]
0000988D  9BDB1E1A00        wait fistp dword [0x1a]
00009892  9BD92E0A00        wait fldcw [0xa]
00009897  A11A00            mov ax,[0x1a]
0000989A  8B161C00          mov dx,[0x1c]
0000989E  C3                ret
0000989F  8B0E0A00          mov cx,[0xa]
000098A3  51                push cx
000098A4  80E5F3            and ch,0xf3
000098A7  0AE5              or ah,ch
000098A9  88260B00          mov [0xb],ah
000098AD  55                push bp
000098AE  E81E13            call 0xabcf
000098B1  5D                pop bp
000098B2  8BC2              mov ax,dx
000098B4  8BD3              mov dx,bx
000098B6  E81900            call 0x98d2
000098B9  8B361000          mov si,[0x10]
000098BD  3B360E00          cmp si,[0xe]
000098C1  7503              jnz 0x98c6
000098C3  E823FB            call 0x93e9
000098C6  83EE0C            sub si,0xc
000098C9  89361000          mov [0x10],si
000098CD  8F060A00          pop word [0xa]
000098D1  C3                ret
000098D2  8B0E1600          mov cx,[0x16]
000098D6  090E0800          or [0x8],cx
000098DA  080E0C00          or [0xc],cl
000098DE  F6D1              not cl
000098E0  8A2E0600          mov ch,[0x6]
000098E4  80CDC2            or ch,0xc2
000098E7  80E53F            and ch,0x3f
000098EA  0ACD              or cl,ch
000098EC  F6D1              not cl
000098EE  8A2E1700          mov ch,[0x17]
000098F2  F7C1FFDF          test cx,0xdfff
000098F6  74D9              jz 0x98d1
000098F8  91                xchg ax,cx
000098F9  E97AFE            jmp 0x9776
000098FC  FB                sti
000098FD  9B50              wait push ax
000098FF  55                push bp
00009900  1E                push ds
00009901  56                push si
00009902  8BEC              mov bp,sp
00009904  C57608            lds si,word [bp+0x8]
00009907  8A04              mov al,[si]
00009909  800CC0            or byte [si],0xc0
0000990C  4E                dec si
0000990D  4E                dec si
0000990E  897608            mov [bp+0x8],si
00009911  D0E8              shr al,1
00009913  D0E8              shr al,1
00009915  D0E8              shr al,1
00009917  F6D0              not al
00009919  2418              and al,0x18
0000991B  0C26              or al,0x26
0000991D  86C4              xchg al,ah
0000991F  B09B              mov al,0x9b
00009921  8904              mov [si],ax
00009923  53                push bx
00009924  BB0100            mov bx,0x1
00009927  EB35              jmp 0x995e
00009929  FB                sti
0000992A  9BCF              wait iret
0000992C  50                push ax
0000992D  1E                push ds
0000992E  B8300B            mov ax,0xb30
00009931  8ED8              mov ds,ax
00009933  2EC606BB06CF      mov byte [cs:0x6bb],0xcf
00009939  33C0              xor ax,ax
0000993B  86061400          xchg al,[0x14]
0000993F  FF1E0000          call word far [0x0]
00009943  1F                pop ds
00009944  58                pop ax
00009945  CF                iret
00009946  FB                sti
00009947  9B50              wait push ax
00009949  B8325C            mov ax,0x5c32
0000994C  55                push bp
0000994D  1E                push ds
0000994E  56                push si
0000994F  8BEC              mov bp,sp
00009951  C57608            lds si,word [bp+0x8]
00009954  4E                dec si
00009955  4E                dec si
00009956  897608            mov [bp+0x8],si
00009959  2904              sub [si],ax
0000995B  53                push bx
0000995C  33DB              xor bx,bx
0000995E  54                push sp
0000995F  58                pop ax
00009960  3BC4              cmp ax,sp
00009962  7528              jnz 0x998c
00009964  8B4001            mov ax,[bx+si+0x1]
00009967  25FB30            and ax,0x30fb
0000996A  3DD930            cmp ax,0x30d9
0000996D  7507              jnz 0x9976
0000996F  8A4002            mov al,[bx+si+0x2]
00009972  3CF0              cmp al,0xf0
00009974  7216              jc 0x998c
00009976  8B4001            mov ax,[bx+si+0x1]
00009979  25FFFE            and ax,0xfeff
0000997C  3DDBE2            cmp ax,0xe2db
0000997F  740B              jz 0x998c
00009981  8B4001            mov ax,[bx+si+0x1]
00009984  3DDFE0            cmp ax,0xe0df
00009987  7403              jz 0x998c
00009989  C60490            mov byte [si],0x90
0000998C  5B                pop bx
0000998D  5E                pop si
0000998E  1F                pop ds
0000998F  5D                pop bp
00009990  58                pop ax
00009991  CF                iret
00009992  99                cwd
00009993  0B610B            or sp,[bx+di+0xb]
00009996  7B0B              jpo 0x99a3
00009998  DD0B              fisttp qword [bp+di]
0000999A  97                xchg ax,di
0000999B  0B630B            or sp,[bp+di+0xb]
0000999E  7D0B              jnl 0x99ab
000099A0  DD0B              fisttp qword [bp+di]
000099A2  8F                db 0x8f
000099A3  0B590B            or bx,[bx+di+0xb]
000099A6  730B              jnc 0x99b3
000099A8  DD0B              fisttp qword [bp+di]
000099AA  91                xchg ax,cx
000099AB  0B570B            or dx,[bx+0xb]
000099AE  710B              jno 0x99bb
000099B0  DD0B              fisttp qword [bp+di]
000099B2  53                push bx
000099B3  0B690B            or bp,[bx+di+0xb]
000099B6  830BDD            or word [bp+di],0xffffffffffffffdd
000099B9  0B510B            or dx,[bx+di+0xb]
000099BC  670B810BDD0B89    or ax,[ecx-0x76f422f5]
000099C3  0B550B            or dx,[di+0xb]
000099C6  6F                outsw
000099C7  0BDD              or bx,bp
000099C9  0B4F0B            or cx,[bx+0xb]
000099CC  650B7F0B          or di,[gs:bx+0xb]
000099D0  DD0B              fisttp qword [bp+di]
000099D2  A6                cmpsb
000099D3  0D8F0D            or ax,0xd8f
000099D6  3B1D              cmp bx,[di]
000099D8  361D8A0D          ss sbb ax,0xd8a
000099DC  850D              test [di],cx
000099DE  96                xchg ax,si
000099DF  0D9F0D            or ax,0xd9f
000099E2  A5                movsw
000099E3  0C7F              or al,0x7f
000099E5  0DAF0C            or ax,0xcaf
000099E8  B90C7F            mov cx,0x7f0c
000099EB  0DD10C            or ax,0xcd1
000099EE  7F0D              jg 0x99fd
000099F0  DB0C              fisttp dword [si]
000099F2  E50C              in ax,byte 0xc
000099F4  5D                pop bp
000099F5  0D750D            or ax,0xd75
000099F8  660DF10C390D      or eax,0xd390cf1
000099FE  45                inc bp
000099FF  0D510D            or ax,0xd51
00009A02  F4                hlt
00009A03  13FD              adc di,bp
00009A05  18D4              sbb ah,dl
00009A07  154818            adc ax,0x1848
00009A0A  AF                scasw
00009A0B  1453              adc al,0x53
00009A0D  1908              sbb [bx+si],cx
00009A0F  17                pop ss
00009A10  B018              mov al,0x18
00009A12  D81C              fcomp dword [si]
00009A14  92                xchg ax,dx
00009A15  1B7F0D            sbb di,[bx+0xd]
00009A18  B519              mov ch,0x19
00009A1A  E11C              loope 0x9a38
00009A1C  92                xchg ax,dx
00009A1D  1CE8              sbb al,0xe8
00009A1F  1C0F              sbb al,0xf
00009A21  1A32              sbb dh,[bp+si]
00009A23  1E                push ds
00009A24  191E1E1E          sbb [0x1e1e],bx
00009A28  141E              adc al,0x1e
00009A2A  231E281E          and bx,[0x1e28]
00009A2E  2D1E7F            sub ax,0x7f1e
00009A31  0D8322            or ax,0x2283
00009A34  AB                stosw
00009A35  22C9              and cl,cl
00009A37  21F5              and bp,si
00009A39  217F0D            and [bx+0xd],di
00009A3C  7F0D              jg 0x9a4b
00009A3E  7F0D              jg 0x9a4d
00009A40  7F0D              jg 0x9a4f
00009A42  F8                clc
00009A43  1E                push ds
00009A44  51                push cx
00009A45  230E207F          and cx,[0x7f20]
00009A49  0D971A            or ax,0x1a97
00009A4C  EF                out dx,ax
00009A4D  1C7F              sbb al,0x7f
00009A4F  0D7F0D            or ax,0xd7f
00009A52  60                pusha
00009A53  1DAA1D            sbb ax,0x1daa
00009A56  9C                pushf
00009A57  1DB01D            sbb ax,0x1db0
00009A5A  B71D              mov bh,0x1d
00009A5C  871D              xchg bx,[di]
00009A5E  9C                pushf
00009A5F  1DB01D            sbb ax,0x1db0
00009A62  9C                pushf
00009A63  1D9C1D            sbb ax,0x1d9c
00009A66  9C                pushf
00009A67  1D9C1D            sbb ax,0x1d9c
00009A6A  A31DA3            mov [0xa31d],ax
00009A6D  1D9C1D            sbb ax,0x1d9c
00009A70  BE1DAA            mov si,0xaa1d
00009A73  1D871D            sbb ax,0x1d87
00009A76  9C                pushf
00009A77  1DA31D            sbb ax,0x1da3
00009A7A  56                push si
00009A7B  0E                push cs
00009A7C  C9                leave
00009A7D  0DDD0D            or ax,0xddd
00009A80  CD0D              int byte 0xd
00009A82  CD0D              int byte 0xd
00009A84  C9                leave
00009A85  0DDD0D            or ax,0xddd
00009A88  CD0D              int byte 0xd
00009A8A  DB0D              fisttp dword [di]
00009A8C  DB0D              fisttp dword [di]
00009A8E  43                inc bx
00009A8F  0E                push cs
00009A90  DB0D              fisttp dword [di]
00009A92  C9                leave
00009A93  0DC90D            or ax,0xdc9
00009A96  DD0D              fisttp qword [di]
00009A98  360E              ss push cs
00009A9A  860F              xchg cl,[bx]
00009A9C  E40D              in al,byte 0xd
00009A9E  DD0D              fisttp qword [di]
00009AA0  F8                clc
00009AA1  0DE40D            or ax,0xde4
00009AA4  E40D              in al,byte 0xd
00009AA6  DD0D              fisttp qword [di]
00009AA8  020EDB0D          add cl,[0xddb]
00009AAC  DB0D              fisttp dword [di]
00009AAE  43                inc bx
00009AAF  0E                push cs
00009AB0  DB0D              fisttp dword [di]
00009AB2  F8                clc
00009AB3  0D020E            or ax,0xe02
00009AB6  DD0D              fisttp qword [di]
00009AB8  F8                clc
00009AB9  0D0511            or ax,0x1105
00009ABC  E40D              in al,byte 0xd
00009ABE  DD0D              fisttp qword [di]
00009AC0  F8                clc
00009AC1  0DF30D            or ax,0xdf3
00009AC4  FD                std
00009AC5  0DD40D            or ax,0xdd4
00009AC8  F30DDB0D          rep or ax,0xddb
00009ACC  DB0D              fisttp dword [di]
00009ACE  43                inc bx
00009ACF  0E                push cs
00009AD0  DB0D              fisttp dword [di]
00009AD2  E40D              in al,byte 0xd
00009AD4  E40D              in al,byte 0xd
00009AD6  DD0D              fisttp qword [di]
00009AD8  020E0406          add cl,[0x604]
00009ADC  40                inc ax
00009ADD  42                inc dx
00009ADE  0103              add [bp+di],ax
00009AE0  050700            add ax,0x7
00009AE3  0000              add [bx+si],al
00009AE5  0000              add [bx+si],al
00009AE7  0000              add [bx+si],al
00009AE9  0001              add [bx+di],al
00009AEB  C00001            rol byte [bx+si],byte 0x1
00009AEE  0000              add [bx+si],al
00009AF0  0000              add [bx+si],al
00009AF2  0000              add [bx+si],al
00009AF4  0000              add [bx+si],al
00009AF6  004000            add [bx+si+0x0],al
00009AF9  0300              add ax,[bx+si]
00009AFB  0000              add [bx+si],al
00009AFD  0000              add [bx+si],al
00009AFF  0000              add [bx+si],al
00009B01  C00040            rol byte [bx+si],byte 0x40
00009B04  8002FF            add byte [bp+si],0xff
00009B07  FFFF              udw
00009B09  FFFF              udw
00009B0B  FFFF              udw
00009B0D  FFFF              udw
00009B0F  3F                aas
00009B10  0000              add [bx+si],al
00009B12  0000              add [bx+si],al
00009B14  807FFFFF          cmp byte [bx-0x1],0xff
00009B18  7F7F              jg 0x9b99
00009B1A  0000              add [bx+si],al
00009B1C  0000              add [bx+si],al
00009B1E  0000              add [bx+si],al
00009B20  F0                lock
00009B21  7FFF              jg 0x9b22
00009B23  FFFF              udw
00009B25  FFFF              udw
00009B27  FF                db 0xff
00009B28  EF                out dx,ax
00009B29  7F00              jg 0x9b2b
00009B2B  0000              add [bx+si],al
00009B2D  0000              add [bx+si],al
00009B2F  0000              add [bx+si],al
00009B31  0000              add [bx+si],al
00009B33  0000              add [bx+si],al
00009B35  0100              add [bx+si],ax
00009B37  0000              add [bx+si],al
00009B39  0000              add [bx+si],al
00009B3B  0000              add [bx+si],al
00009B3D  800000            add byte [bx+si],0x0
00009B40  0000              add [bx+si],al
00009B42  8AB16AF6          mov dh,[bx+di-0x996]
00009B46  F4                hlt
00009B47  A23089            mov [0x8930],al
00009B4A  FE                db 0xfe
00009B4B  FF00              inc word [bx+si]
00009B4D  009E5365          add [bp+0x6553],bl
00009B51  C242D7            ret word 0xd742
00009B54  B3DD              mov bl,0xdd
00009B56  0000              add [bx+si],al
00009B58  0000              add [bx+si],al
00009B5A  232C              and bp,[si]
00009B5C  9B6BC191          wait imul ax,cx,0xffffffffffffff91
00009B60  0A86FFFF          or al,[bp-0x1]
00009B64  0000              add [bx+si],al
00009B66  8464DE            test [si-0x22],ah
00009B69  F9                stc
00009B6A  33F3              xor si,bx
00009B6C  04B5              add al,0xb5
00009B6E  0000              add [bx+si],al
00009B70  0000              add [bx+si],al
00009B72  0000              add [bx+si],al
00009B74  0000              add [bx+si],al
00009B76  0000              add [bx+si],al
00009B78  00800100          add [bx+si+0x1],al
00009B7C  0000              add [bx+si],al
00009B7E  35C268            xor ax,0x68c2
00009B81  21A2DA0F          and [bp+si+0xfda],sp
00009B85  C9                leave
00009B86  0100              add [bx+si],ax
00009B88  0000              add [bx+si],al
00009B8A  FE8A1BCD          dec byte [bp+si-0x32e5]
00009B8E  4B                dec bx
00009B8F  789A              js 0x9b2b
00009B91  D401              aam byte 0x1
00009B93  0000              add [bx+si],al
00009B95  00BCF017          add [si+0x17f0],bh
00009B99  5C                pop sp
00009B9A  293B              sub [bp+di],di
00009B9C  AA                stosb
00009B9D  B80000            mov ax,0x0
00009BA0  0000              add [bx+si],al
00009BA2  99                cwd
00009BA3  F7                db 0xf7
00009BA4  CF                iret
00009BA5  FB                sti
00009BA6  849A209A          test [bp+si-0x65e0],bl
00009BAA  FE                db 0xfe
00009BAB  FF00              inc word [bx+si]
00009BAD  00AC79CF          add [si-0x3087],ch
00009BB1  D1                db 0xd1
00009BB2  F717              not word [bx]
00009BB4  72B1              jc 0x9b67
00009BB6  FFFF              udw
00009BB8  0000              add [bx+si],al
00009BBA  0300              add ax,[bx+si]
00009BBC  D47B              aam byte 0x7b
00009BBE  5A                pop dx
00009BBF  D83E5C69          fdivr dword [0x695c]
00009BC3  8F05              pop word [di]
00009BC5  00800037          add [bx+si+0x3700],al
00009BC9  4D                dec bp
00009BCA  D7                xlatb
00009BCB  2CF8              sub al,0xf8
00009BCD  D0D4              rcl ah,1
00009BCF  D6                salc
00009BD0  0C00              or al,0x0
00009BD2  0000              add [bx+si],al
00009BD4  D3DC              rcr sp,cl
00009BD6  17                pop ss
00009BD7  66EE              o32 out dx,al
00009BD9  BBBA82            mov bx,0x82ba
00009BDC  1200              adc al,[bx+si]
00009BDE  8000CB            add byte [bx+si],0xcb
00009BE1  91                xchg ax,cx
00009BE2  58                pop ax
00009BE3  5E                pop si
00009BE4  8B8606F5          mov ax,[bp-0xafa]
00009BE8  1400              adc al,0x0
00009BEA  0000              add [bx+si],al
00009BEC  0300              add ax,[bx+si]
00009BEE  E586              in ax,byte 0x86
00009BF0  2001              and [bx+di],al
00009BF2  0205              add al,[di]
00009BF4  799C              jns 0x9b92
00009BF6  0900              or [bx+si],ax
00009BF8  800063            add byte [bx+si],0x63
00009BFB  66CF              iretd
00009BFD  8870B2            mov [bx+si-0x4e],dh
00009C00  39C9              cmp cx,cx
00009C02  0F0000            sldt word [bx+si]
00009C05  0096FA46          add [bp+0x46fa],dl
00009C09  C7                db 0xc7
00009C0A  FE0C              dec byte [si]
00009C0C  B7E4              mov bh,0xe4
00009C0E  1300              adc ax,[bx+si]
00009C10  8000CB            add byte [bx+si],0xcb
00009C13  91                xchg ax,cx
00009C14  58                pop ax
00009C15  5E                pop si
00009C16  8B8606F5          mov ax,[bp-0xafa]
00009C1A  1400              adc al,0x0
00009C1C  0000              add [bx+si],al
00009C1E  0400              add al,0x0
00009C20  325B08            xor bl,[bp+di+0x8]
00009C23  CF                iret
00009C24  C9                leave
00009C25  A4                movsb
00009C26  50                push ax
00009C27  A6                cmpsb
00009C28  FD                std
00009C29  FF00              inc word [bx+si]
00009C2B  00CE              add dh,cl
00009C2D  D1CA              ror dx,1
00009C2F  D54B              aad byte 0x4b
00009C31  A8F0              test al,0xf0
00009C33  D002              rol byte [bp+si],1
00009C35  0000              add [bx+si],al
00009C37  009F892B          add [bx+0x2b89],bl
00009C3B  E2A8              loop 0x9be5
00009C3D  52                push dx
00009C3E  4A                dec dx
00009C3F  9C                pushf
00009C40  050000            add ax,0x0
00009C43  006542            add [di+0x42],ah
00009C46  50                push ax
00009C47  55                push bp
00009C48  CF                iret
00009C49  E9EF90            jmp 0x2d3b
00009C4C  06                push es
00009C4D  0000              add [bx+si],al
00009C4F  00904BAD          add [bx+si-0x52b5],dl
00009C53  24E6              and al,0xe6
00009C55  E543              in ax,byte 0x43
00009C57  A4                movsb
00009C58  050000            add ax,0x0
00009C5B  0003              add [bp+di],al
00009C5D  0010              add [bx+si],dl
00009C5F  833856            cmp word [bx+si],0x56
00009C62  0A4F62            or cl,[bx+0x62]
00009C65  F0                lock
00009C66  0300              add ax,[bx+si]
00009C68  0000              add [bx+si],al
00009C6A  E7B4              out byte 0xb4,ax
00009C6C  1E                push ds
00009C6D  6D                insw
00009C6E  90                nop
00009C6F  51                push cx
00009C70  50                push ax
00009C71  EE                out dx,al
00009C72  050000            add ax,0x0
00009C75  003B              add [bp+di],bh
00009C77  246D              and al,0x6d
00009C79  5B                pop bx
00009C7A  209050AC          and [bx+si-0x53b0],dl
00009C7E  06                push es
00009C7F  0000              add [bx+si],al
00009C81  00904BAD          add [bx+si-0x52b5],dl
00009C85  24E6              and al,0xe6
00009C87  E543              in ax,byte 0x43
00009C89  A4                movsb
00009C8A  050000            add ax,0x0
00009C8D  0002              add [bp+si],al
00009C8F  00981805          add [bx+si+0x518],bl
00009C93  F4                hlt
00009C94  FC                cld
00009C95  06                push es
00009C96  74F2              jz 0x9c8a
00009C98  050000            add ax,0x0
00009C9B  0008              add [bx+si],cl
00009C9D  AD                lodsw
00009C9E  E114              loope 0x9cb4
00009CA0  54                push sp
00009CA1  3D9BEC            cmp ax,0xec9b
00009CA4  0E                push cs
00009CA5  0000              add [bx+si],al
00009CA7  00AF5FA3          add [bx-0x5ca1],ch
00009CAB  C3                ret
00009CAC  4A                dec dx
00009CAD  D8F0              fdiv st0
00009CAF  FD                std
00009CB0  1400              adc al,0x0
00009CB2  0000              add [bx+si],al
00009CB4  0200              add al,[bx+si]
00009CB6  6F                outsw
00009CB7  777B              ja 0x9d34
00009CB9  388B10A7          cmp [bp+di-0x58f0],cl
00009CBD  DA0A              fimul dword [bp+si]
00009CBF  0000              add [bx+si],al
00009CC1  005DE8            add [di-0x18],bl
00009CC4  7B9B              jpo 0x9c61
00009CC6  82                db 0x82
00009CC7  B103              mov cl,0x3
00009CC9  A01200            mov al,[0x12]
00009CCC  0000              add [bx+si],al
00009CCE  7E83              jng 0x9c53
00009CD0  09E7              or di,sp
00009CD2  14F8              adc al,0xf8
00009CD4  2DB716            sub ax,0x16b7
00009CD7  0000              add [bx+si],al
00009CD9  0003              add [bp+di],al
00009CDB  0004              add [si],al
00009CDD  7799              ja 0x9c78
00009CDF  C2E257            ret word 0x57e2
00009CE2  719B              jno 0x9c7f
00009CE4  FE                db 0xfe
00009CE5  FF00              inc word [bx+si]
00009CE7  009C4F31          add [si+0x314f],bl
00009CEB  F635              div byte [di]
00009CED  5E                pop si
00009CEE  91                xchg ax,cx
00009CEF  DE04              fiadd word [si]
00009CF1  0080008A          add [bx+si-0x7600],al
00009CF5  4B                dec bx
00009CF6  EA7AEDC9D3        jmp word 0xd3c9:word 0xed7a
00009CFB  B208              mov dl,0x8
00009CFD  0000              add [bx+si],al
00009CFF  00C9              add cl,cl
00009D01  2809              sub [bx+di],cl
00009D03  1D2FE4            sbb ax,0xe42f
00009D06  C48A0A00          les cx,word [bp+si+0xa]
00009D0A  800002            add byte [bx+si],0x2
00009D0D  00BB7670          add [bp+di+0x7076],bh
00009D11  3E5B              ds pop bx
00009D13  02AC8E05          add ch,[si+0x58e]
00009D17  00800060          add [bx+si+0x6000],al
00009D1B  EF                out dx,ax
00009D1C  33A9D01F          xor bp,[bx+di+0x1fd0]
00009D20  049C              add al,0x9c
00009D22  0800              or [bx+si],al
00009D24  0000              add [bx+si],al
00009D26  96                xchg ax,si
00009D27  BB836C            mov bx,0x6c83
00009D2A  E0F4              loopne 0x9d20
00009D2C  5F                pop di
00009D2D  C00900            ror byte [bx+di],byte 0x0
00009D30  800055            add byte [bx+si],0x55
00009D33  1E                push ds
00009D34  56                push si
00009D35  8BEC              mov bp,sp
00009D37  C57606            lds si,word [bp+0x6]
00009D3A  4E                dec si
00009D3B  4E                dec si
00009D3C  897606            mov [bp+0x6],si
00009D3F  C70489C0          mov word [si],0xc089
00009D43  5E                pop si
00009D44  1F                pop ds
00009D45  5D                pop bp
00009D46  CF                iret
00009D47  0B0B              or cx,[bp+di]
00009D49  16                push ss
00009D4A  0B10              or dx,[bx+si]
00009D4C  0B1A              or bx,[bp+si]
00009D4E  0BFB              or di,bx
00009D50  50                push ax
00009D51  06                push es
00009D52  1E                push ds
00009D53  57                push di
00009D54  56                push si
00009D55  52                push dx
00009D56  51                push cx
00009D57  53                push bx
00009D58  83EC02            sub sp,0x2
00009D5B  55                push bp
00009D5C  8BEC              mov bp,sp
00009D5E  FC                cld
00009D5F  8BD3              mov dx,bx
00009D61  8BC7              mov ax,di
00009D63  C57E14            lds di,word [bp+0x14]
00009D66  47                inc di
00009D67  47                inc di
00009D68  8B4DFE            mov cx,[di-0x2]
00009D6B  8BD9              mov bx,cx
00009D6D  D0C3              rol bl,1
00009D6F  D0C3              rol bl,1
00009D71  83E303            and bx,0x3
00009D74  D1C3              rol bx,1
00009D76  2EFFA7D70A        jmp word near [cs:bx+0xad7]
00009D7B  8E460E            mov es,word [bp+0xe]
00009D7E  EB0A              jmp 0x9d8a
00009D80  8CDB              mov bx,ds
00009D82  8EC3              mov es,bx
00009D84  EB04              jmp 0x9d8a
00009D86  8CD3              mov bx,ss
00009D88  8EC3              mov es,bx
00009D8A  8C4602            mov word [bp+0x2],es
00009D8D  EB20              jmp 0x9daf
00009D8F  FB                sti
00009D90  50                push ax
00009D91  06                push es
00009D92  1E                push ds
00009D93  57                push di
00009D94  56                push si
00009D95  52                push dx
00009D96  51                push cx
00009D97  53                push bx
00009D98  16                push ss
00009D99  55                push bp
00009D9A  8BEC              mov bp,sp
00009D9C  8CD8              mov ax,ds
00009D9E  8EC0              mov es,ax
00009DA0  FC                cld
00009DA1  8BD3              mov dx,bx
00009DA3  8BC7              mov ax,di
00009DA5  C57E14            lds di,word [bp+0x14]
00009DA8  47                inc di
00009DA9  8B4DFE            mov cx,[di-0x2]
00009DAC  80E934            sub cl,0x34
00009DAF  D0C5              rol ch,1
00009DB1  D0C5              rol ch,1
00009DB3  8ADD              mov bl,ch
00009DB5  83E31F            and bx,0x1f
00009DB8  D1E3              shl bx,1
00009DBA  2EFFA72207        jmp word near [cs:bx+0x722]
00009DBF  8BC2              mov ax,dx
00009DC1  8BF0              mov si,ax
00009DC3  EB48              jmp 0x9e0d
00009DC5  33C0              xor ax,ax
00009DC7  8BF0              mov si,ax
00009DC9  037600            add si,[bp+0x0]
00009DCC  8E4602            mov es,word [bp+0x2]
00009DCF  EB08              jmp 0x9dd9
00009DD1  8BC6              mov ax,si
00009DD3  03D0              add dx,ax
00009DD5  8BC2              mov ax,dx
00009DD7  8BF0              mov si,ax
00009DD9  8A05              mov al,[di]
00009DDB  98                cbw
00009DDC  47                inc di
00009DDD  EB2C              jmp 0x9e0b
00009DDF  33C0              xor ax,ax
00009DE1  8BF0              mov si,ax
00009DE3  037600            add si,[bp+0x0]
00009DE6  8E4602            mov es,word [bp+0x2]
00009DE9  EB08              jmp 0x9df3
00009DEB  8BC6              mov ax,si
00009DED  03D0              add dx,ax
00009DEF  8BC2              mov ax,dx
00009DF1  8BF0              mov si,ax
00009DF3  8B05              mov ax,[di]
00009DF5  47                inc di
00009DF6  47                inc di
00009DF7  EB12              jmp 0x9e0b
00009DF9  8B35              mov si,[di]
00009DFB  47                inc di
00009DFC  47                inc di
00009DFD  EB0E              jmp 0x9e0d
00009DFF  8BC6              mov ax,si
00009E01  8B5600            mov dx,[bp+0x0]
00009E04  8E4602            mov es,word [bp+0x2]
00009E07  8BF0              mov si,ax
00009E09  8BC2              mov ax,dx
00009E0B  03F0              add si,ax
00009E0D  897E14            mov [bp+0x14],di
00009E10  B8300B            mov ax,0xb30
00009E13  8ED8              mov ds,ax
00009E15  C70616000020      mov word [0x16],0x2000
00009E1B  D0ED              shr ch,1
00009E1D  D0ED              shr ch,1
00009E1F  D0ED              shr ch,1
00009E21  D0ED              shr ch,1
00009E23  8ADD              mov bl,ch
00009E25  83E30E            and bx,0xe
00009E28  F6C101            test cl,0x1
00009E2B  7408              jz 0x9e35
00009E2D  2EFF977207        call word near [cs:bx+0x772]
00009E32  E9AC00            jmp 0x9ee1
00009E35  53                push bx
00009E36  E8DC00            call 0x9f15
00009E39  5B                pop bx
00009E3A  8CD8              mov ax,ds
00009E3C  8EC0              mov es,ax
00009E3E  8B361000          mov si,[0x10]
00009E42  8BFE              mov di,si
00009E44  83EF0C            sub di,0xc
00009E47  893E9400          mov [0x94],di
00009E4B  EB47              jmp 0x9e94
00009E4D  897E14            mov [bp+0x14],di
00009E50  33C0              xor ax,ax
00009E52  BF300B            mov di,0xb30
00009E55  8EDF              mov ds,di
00009E57  8EC7              mov es,di
00009E59  A31600            mov [0x16],ax
00009E5C  8ADD              mov bl,ch
00009E5E  D0EB              shr bl,1
00009E60  D0EB              shr bl,1
00009E62  D0EB              shr bl,1
00009E64  D0EB              shr bl,1
00009E66  83E30E            and bx,0xe
00009E69  F6C101            test cl,0x1
00009E6C  7407              jz 0x9e75
00009E6E  2EFF978207        call word near [cs:bx+0x782]
00009E73  EB6C              jmp 0x9ee1
00009E75  E84800            call 0x9ec0
00009E78  723C              jc 0x9eb6
00009E7A  89369400          mov [0x94],si
00009E7E  F6C104            test cl,0x4
00009E81  7404              jz 0x9e87
00009E83  893E9400          mov [0x94],di
00009E87  F6C308            test bl,0x8
00009E8A  7403              jz 0x9e8f
00009E8C  80F302            xor bl,0x2
00009E8F  F6C102            test cl,0x2
00009E92  741B              jz 0x9eaf
00009E94  2EFF976207        call word near [cs:bx+0x762]
00009E99  8B361000          mov si,[0x10]
00009E9D  3B360E00          cmp si,[0xe]
00009EA1  7503              jnz 0x9ea6
00009EA3  E843F5            call 0x93e9
00009EA6  83EE0C            sub si,0xc
00009EA9  89361000          mov [0x10],si
00009EAD  EB32              jmp 0x9ee1
00009EAF  2EFF976207        call word near [cs:bx+0x762]
00009EB4  EB2B              jmp 0x9ee1
00009EB6  E830F5            call 0x93e9
00009EB9  830E160001        or word [0x16],0x1
00009EBE  EB21              jmp 0x9ee1
00009EC0  8B361000          mov si,[0x10]
00009EC4  8BFE              mov di,si
00009EC6  8AD5              mov dl,ch
00009EC8  80E21C            and dl,0x1c
00009ECB  8AF2              mov dh,dl
00009ECD  D0E6              shl dh,1
00009ECF  02D6              add dl,dh
00009ED1  32F6              xor dh,dh
00009ED3  2BFA              sub di,dx
00009ED5  3B3E0E00          cmp di,[0xe]
00009ED9  F8                clc
00009EDA  7F01              jg 0x9edd
00009EDC  F5                cmc
00009EDD  C3                ret
00009EDE  E80E01            call 0x9fef
00009EE1  5D                pop bp
00009EE2  83C402            add sp,0x2
00009EE5  5B                pop bx
00009EE6  59                pop cx
00009EE7  5A                pop dx
00009EE8  5E                pop si
00009EE9  5F                pop di
00009EEA  A11600            mov ax,[0x16]
00009EED  09060800          or [0x8],ax
00009EF1  08060C00          or [0xc],al
00009EF5  F6D0              not al
00009EF7  8A260600          mov ah,[0x6]
00009EFB  80CCC2            or ah,0xc2
00009EFE  80E43F            and ah,0x3f
00009F01  0AC4              or al,ah
00009F03  F6D0              not al
00009F05  8A261700          mov ah,[0x17]
00009F09  A9FFDF            test ax,0xdfff
00009F0C  1F                pop ds
00009F0D  07                pop es
00009F0E  7502              jnz 0x9f12
00009F10  58                pop ax
00009F11  CF                iret
00009F12  E961F8            jmp 0x9776
00009F15  8BD9              mov bx,cx
00009F17  83E306            and bx,0x6
00009F1A  2EFFA79207        jmp word near [cs:bx+0x792]
00009F1F  8BD9              mov bx,cx
00009F21  83E306            and bx,0x6
00009F24  2EFFA79A07        jmp word near [cs:bx+0x79a]
00009F29  E8F3FF            call 0x9f1f
00009F2C  8B361000          mov si,[0x10]
00009F30  3B360E00          cmp si,[0xe]
00009F34  7503              jnz 0x9f39
00009F36  E8B0F4            call 0x93e9
00009F39  83EE0C            sub si,0xc
00009F3C  89361000          mov [0x10],si
00009F40  C3                ret
00009F41  8BD9              mov bx,cx
00009F43  83E306            and bx,0x6
00009F46  2EFFA7A207        jmp word near [cs:bx+0x7a2]
00009F4B  8BD9              mov bx,cx
00009F4D  83E306            and bx,0x6
00009F50  2EFFA7AA07        jmp word near [cs:bx+0x7aa]
00009F55  F7C10600          test cx,0x6
00009F59  7503              jnz 0x9f5e
00009F5B  E9B411            jmp 0xb112
00009F5E  E96B11            jmp 0xb0cc
00009F61  F7C10608          test cx,0x806
00009F65  7424              jz 0x9f8b
00009F67  81F10302          xor cx,0x203
00009F6B  F7C10306          test cx,0x603
00009F6F  7511              jnz 0x9f82
00009F71  81F90481          cmp cx,0x8104
00009F75  7407              jz 0x9f7e
00009F77  C7060C000000      mov word [0xc],0x0
00009F7D  C3                ret
00009F7E  54                push sp
00009F7F  58                pop ax
00009F80  3BC4              cmp ax,sp
00009F82  756B              jnz 0x9fef
00009F84  A10C00            mov ax,[0xc]
00009F87  894612            mov [bp+0x12],ax
00009F8A  C3                ret
00009F8B  F7C10010          test cx,0x1000
00009F8F  740C              jz 0x9f9d
00009F91  F7C10004          test cx,0x400
00009F95  7503              jnz 0x9f9a
00009F97  E9D310            jmp 0xb06d
00009F9A  E9BA10            jmp 0xb057
00009F9D  F7C10004          test cx,0x400
00009FA1  7503              jnz 0x9fa6
00009FA3  E9990F            jmp 0xaf3f
00009FA6  E98D0F            jmp 0xaf36
00009FA9  8ADD              mov bl,ch
00009FAB  D0EB              shr bl,1
00009FAD  83E30E            and bx,0xe
00009FB0  2EFFA7B207        jmp word near [cs:bx+0x7b2]
00009FB5  8ADD              mov bl,ch
00009FB7  D0EB              shr bl,1
00009FB9  83E30E            and bx,0xe
00009FBC  2EFFA7C207        jmp word near [cs:bx+0x7c2]
00009FC1  8ADD              mov bl,ch
00009FC3  D0EB              shr bl,1
00009FC5  83E30E            and bx,0xe
00009FC8  2EFFA7D207        jmp word near [cs:bx+0x7d2]
00009FCD  F7C10600          test cx,0x6
00009FD1  751C              jnz 0x9fef
00009FD3  E91A11            jmp 0xb0f0
00009FD6  80F104            xor cl,0x4
00009FD9  F7C10600          test cx,0x6
00009FDD  7510              jnz 0x9fef
00009FDF  B80100            mov ax,0x1
00009FE2  E95411            jmp 0xb139
00009FE5  F6C106            test cl,0x6
00009FE8  7405              jz 0x9fef
00009FEA  33C0              xor ax,ax
00009FEC  E94A11            jmp 0xb139
00009FEF  830E160040        or word [0x16],0x40
00009FF4  C3                ret
00009FF5  BA0080            mov dx,0x8000
00009FF8  EB1E              jmp 0xa018
00009FFA  BA8000            mov dx,0x80
00009FFD  EB19              jmp 0xa018
00009FFF  33D2              xor dx,dx
0000A001  BB2A08            mov bx,0x82a
0000A004  EB15              jmp 0xa01b
0000A006  33D2              xor dx,dx
0000A008  87F7              xchg si,di
0000A00A  BB4A08            mov bx,0x84a
0000A00D  EB0C              jmp 0xa01b
0000A00F  33D2              xor dx,dx
0000A011  BB4A08            mov bx,0x84a
0000A014  EB05              jmp 0xa01b
0000A016  33D2              xor dx,dx
0000A018  BB0A08            mov bx,0x80a
0000A01B  8A450B            mov al,[di+0xb]
0000A01E  D0E0              shl al,1
0000A020  D0E0              shl al,1
0000A022  0A440B            or al,[si+0xb]
0000A025  98                cbw
0000A026  D1E0              shl ax,1
0000A028  03D8              add bx,ax
0000A02A  32750A            xor dh,[di+0xa]
0000A02D  32540A            xor dl,[si+0xa]
0000A030  8B4D08            mov cx,[di+0x8]
0000A033  8B4408            mov ax,[si+0x8]
0000A036  2EFF27            jmp word near [cs:bx]
0000A039  8BF7              mov si,di
0000A03B  8AD6              mov dl,dh
0000A03D  E85100            call 0xa091
0000A040  88550A            mov [di+0xa],dl
0000A043  C3                ret
0000A044  830E160004        or word [0x16],0x4
0000A049  EB02              jmp 0xa04d
0000A04B  8BF7              mov si,di
0000A04D  830E160001        or word [0x16],0x1
0000A052  EB3D              jmp 0xa091
0000A054  BE7208            mov si,0x872
0000A057  32F2              xor dh,dl
0000A059  80E67F            and dh,0x7f
0000A05C  E81B00            call 0xa07a
0000A05F  08750A            or [di+0xa],dh
0000A062  C3                ret
0000A063  830E160004        or word [0x16],0x4
0000A068  BE7E08            mov si,0x87e
0000A06B  EBEA              jmp 0xa057
0000A06D  830E160004        or word [0x16],0x4
0000A072  BE8A08            mov si,0x88a
0000A075  830E160001        or word [0x16],0x1
0000A07A  8B3E9400          mov di,[0x94]
0000A07E  2EA5              cs movsw
0000A080  2EA5              cs movsw
0000A082  2EA5              cs movsw
0000A084  2EA5              cs movsw
0000A086  2EA5              cs movsw
0000A088  2EA5              cs movsw
0000A08A  83EF0C            sub di,0xc
0000A08D  83EE0C            sub si,0xc
0000A090  C3                ret
0000A091  8B3E9400          mov di,[0x94]
0000A095  3BF7              cmp si,di
0000A097  740C              jz 0xa0a5
0000A099  A5                movsw
0000A09A  A5                movsw
0000A09B  A5                movsw
0000A09C  A5                movsw
0000A09D  A5                movsw
0000A09E  A5                movsw
0000A09F  83EF0C            sub di,0xc
0000A0A2  83EE0C            sub si,0xc
0000A0A5  C3                ret
0000A0A6  F6060B0010        test byte [0xb],0x10
0000A0AB  74C5              jz 0xa072
0000A0AD  32D6              xor dl,dh
0000A0AF  78C1              js 0xa072
0000A0B1  EB86              jmp 0xa039
0000A0B3  8BC6              mov ax,si
0000A0B5  8BDF              mov bx,di
0000A0B7  B90400            mov cx,0x4
0000A0BA  F3A7              repe cmpsw
0000A0BC  7704              ja 0xa0c2
0000A0BE  8BF0              mov si,ax
0000A0C0  EB8B              jmp 0xa04d
0000A0C2  8BF3              mov si,bx
0000A0C4  EB87              jmp 0xa04d
0000A0C6  2BC1              sub ax,cx
0000A0C8  7C08              jl 0xa0d2
0000A0CA  87F7              xchg si,di
0000A0CC  86F2              xchg dh,dl
0000A0CE  03C8              add cx,ax
0000A0D0  F7D8              neg ax
0000A0D2  F7D8              neg ax
0000A0D4  3D4300            cmp ax,0x43
0000A0D7  7E2F              jng 0xa108
0000A0D9  55                push bp
0000A0DA  52                push dx
0000A0DB  8B7508            mov si,[di+0x8]
0000A0DE  BD0100            mov bp,0x1
0000A0E1  32D6              xor dl,dh
0000A0E3  7902              jns 0xa0e7
0000A0E5  F7DD              neg bp
0000A0E7  8B15              mov dx,[di]
0000A0E9  8B4D02            mov cx,[di+0x2]
0000A0EC  8B5D04            mov bx,[di+0x4]
0000A0EF  8B7D06            mov di,[di+0x6]
0000A0F2  0BED              or bp,bp
0000A0F4  7803              js 0xa0f9
0000A0F6  E94904            jmp 0xa542
0000A0F9  83EA01            sub dx,0x1
0000A0FC  83D900            sbb cx,0x0
0000A0FF  83DB00            sbb bx,0x0
0000A102  83DF00            sbb di,0x0
0000A105  E98803            jmp 0xa490
0000A108  55                push bp
0000A109  52                push dx
0000A10A  51                push cx
0000A10B  32D6              xor dl,dh
0000A10D  9C                pushf
0000A10E  8BE8              mov bp,ax
0000A110  AD                lodsw
0000A111  8BD0              mov dx,ax
0000A113  AD                lodsw
0000A114  8BC8              mov cx,ax
0000A116  AD                lodsw
0000A117  8BD8              mov bx,ax
0000A119  AD                lodsw
0000A11A  33F6              xor si,si
0000A11C  87EE              xchg bp,si
0000A11E  0BF6              or si,si
0000A120  7470              jz 0xa192
0000A122  83FE0E            cmp si,0xe
0000A125  7C1A              jl 0xa141
0000A127  0BED              or bp,bp
0000A129  7403              jz 0xa12e
0000A12B  83CA01            or dx,0x1
0000A12E  8BEA              mov bp,dx
0000A130  8BD1              mov dx,cx
0000A132  8BCB              mov cx,bx
0000A134  8BD8              mov bx,ax
0000A136  33C0              xor ax,ax
0000A138  83EE10            sub si,0x10
0000A13B  77E5              ja 0xa122
0000A13D  7453              jz 0xa192
0000A13F  7229              jc 0xa16a
0000A141  83FE06            cmp si,0x6
0000A144  7C33              jl 0xa179
0000A146  95                xchg ax,bp
0000A147  0AC0              or al,al
0000A149  7403              jz 0xa14e
0000A14B  80CC01            or ah,0x1
0000A14E  8AC4              mov al,ah
0000A150  8AE2              mov ah,dl
0000A152  95                xchg ax,bp
0000A153  8AD6              mov dl,dh
0000A155  8AF1              mov dh,cl
0000A157  8ACD              mov cl,ch
0000A159  8AEB              mov ch,bl
0000A15B  8ADF              mov bl,bh
0000A15D  8AF8              mov bh,al
0000A15F  8AC4              mov al,ah
0000A161  32E4              xor ah,ah
0000A163  83EE08            sub si,0x8
0000A166  7711              ja 0xa179
0000A168  7428              jz 0xa192
0000A16A  D1E5              shl bp,1
0000A16C  D1D2              rcl dx,1
0000A16E  D1D1              rcl cx,1
0000A170  D1D3              rcl bx,1
0000A172  D1D0              rcl ax,1
0000A174  46                inc si
0000A175  75F3              jnz 0xa16a
0000A177  EB19              jmp 0xa192
0000A179  87CE              xchg cx,si
0000A17B  F7C53F00          test bp,0x3f
0000A17F  7403              jz 0xa184
0000A181  83CD20            or bp,0x20
0000A184  D1E8              shr ax,1
0000A186  D1DB              rcr bx,1
0000A188  D1DE              rcr si,1
0000A18A  D1DA              rcr dx,1
0000A18C  D1DD              rcr bp,1
0000A18E  E2F4              loop 0xa184
0000A190  8BCE              mov cx,si
0000A192  8BF7              mov si,di
0000A194  8BF8              mov di,ax
0000A196  F7C5FF3F          test bp,0x3fff
0000A19A  7403              jz 0xa19f
0000A19C  83CD01            or bp,0x1
0000A19F  9D                popf
0000A1A0  58                pop ax
0000A1A1  7822              js 0xa1c5
0000A1A3  0314              add dx,[si]
0000A1A5  134C02            adc cx,[si+0x2]
0000A1A8  135C04            adc bx,[si+0x4]
0000A1AB  137C06            adc di,[si+0x6]
0000A1AE  7310              jnc 0xa1c0
0000A1B0  D1DF              rcr di,1
0000A1B2  D1DB              rcr bx,1
0000A1B4  D1D9              rcr cx,1
0000A1B6  D1DA              rcr dx,1
0000A1B8  D1DD              rcr bp,1
0000A1BA  7303              jnc 0xa1bf
0000A1BC  83CD01            or bp,0x1
0000A1BF  40                inc ax
0000A1C0  8BF0              mov si,ax
0000A1C2  E97D03            jmp 0xa542
0000A1C5  2B14              sub dx,[si]
0000A1C7  1B4C02            sbb cx,[si+0x2]
0000A1CA  1B5C04            sbb bx,[si+0x4]
0000A1CD  1B7C06            sbb di,[si+0x6]
0000A1D0  731A              jnc 0xa1ec
0000A1D2  33F6              xor si,si
0000A1D4  F7D7              not di
0000A1D6  F7D3              not bx
0000A1D8  F7D1              not cx
0000A1DA  F7D2              not dx
0000A1DC  F7DD              neg bp
0000A1DE  F5                cmc
0000A1DF  13D6              adc dx,si
0000A1E1  13CE              adc cx,si
0000A1E3  13DE              adc bx,si
0000A1E5  13FE              adc di,si
0000A1E7  8BF0              mov si,ax
0000A1E9  E9A402            jmp 0xa490
0000A1EC  8BF0              mov si,ax
0000A1EE  58                pop ax
0000A1EF  80F480            xor ah,0x80
0000A1F2  50                push ax
0000A1F3  E99A02            jmp 0xa490
0000A1F6  55                push bp
0000A1F7  8AFE              mov bh,dh
0000A1F9  32F2              xor dh,dl
0000A1FB  52                push dx
0000A1FC  03C1              add ax,cx
0000A1FE  40                inc ax
0000A1FF  50                push ax
0000A200  22FA              and bh,dl
0000A202  33DB              xor bx,bx
0000A204  8BEB              mov bp,bx
0000A206  8BCB              mov cx,bx
0000A208  8B04              mov ax,[si]
0000A20A  0BC0              or ax,ax
0000A20C  740C              jz 0xa21a
0000A20E  8B15              mov dx,[di]
0000A210  0BD2              or dx,dx
0000A212  7406              jz 0xa21a
0000A214  F7E2              mul dx
0000A216  8BE8              mov bp,ax
0000A218  8BCA              mov cx,dx
0000A21A  55                push bp
0000A21B  8B04              mov ax,[si]
0000A21D  0BC0              or ax,ax
0000A21F  7410              jz 0xa231
0000A221  8B5502            mov dx,[di+0x2]
0000A224  0BD2              or dx,dx
0000A226  7409              jz 0xa231
0000A228  F7E2              mul dx
0000A22A  03C8              add cx,ax
0000A22C  13DA              adc bx,dx
0000A22E  83D500            adc bp,0x0
0000A231  8B4402            mov ax,[si+0x2]
0000A234  0BC0              or ax,ax
0000A236  740F              jz 0xa247
0000A238  8B15              mov dx,[di]
0000A23A  0BD2              or dx,dx
0000A23C  7409              jz 0xa247
0000A23E  F7E2              mul dx
0000A240  03C8              add cx,ax
0000A242  13DA              adc bx,dx
0000A244  83D500            adc bp,0x0
0000A247  58                pop ax
0000A248  0BC1              or ax,cx
0000A24A  50                push ax
0000A24B  33C9              xor cx,cx
0000A24D  8B04              mov ax,[si]
0000A24F  0BC0              or ax,ax
0000A251  7410              jz 0xa263
0000A253  8B5504            mov dx,[di+0x4]
0000A256  0BD2              or dx,dx
0000A258  7409              jz 0xa263
0000A25A  F7E2              mul dx
0000A25C  03D8              add bx,ax
0000A25E  13EA              adc bp,dx
0000A260  83D100            adc cx,0x0
0000A263  8B4402            mov ax,[si+0x2]
0000A266  0BC0              or ax,ax
0000A268  7410              jz 0xa27a
0000A26A  8B5502            mov dx,[di+0x2]
0000A26D  0BD2              or dx,dx
0000A26F  7409              jz 0xa27a
0000A271  F7E2              mul dx
0000A273  03D8              add bx,ax
0000A275  13EA              adc bp,dx
0000A277  83D100            adc cx,0x0
0000A27A  8B4404            mov ax,[si+0x4]
0000A27D  0BC0              or ax,ax
0000A27F  740F              jz 0xa290
0000A281  8B15              mov dx,[di]
0000A283  0BD2              or dx,dx
0000A285  7409              jz 0xa290
0000A287  F7E2              mul dx
0000A289  03D8              add bx,ax
0000A28B  13EA              adc bp,dx
0000A28D  83D100            adc cx,0x0
0000A290  58                pop ax
0000A291  0BC3              or ax,bx
0000A293  50                push ax
0000A294  33DB              xor bx,bx
0000A296  8B04              mov ax,[si]
0000A298  0BC0              or ax,ax
0000A29A  740A              jz 0xa2a6
0000A29C  F76506            mul word [di+0x6]
0000A29F  03E8              add bp,ax
0000A2A1  13CA              adc cx,dx
0000A2A3  83D300            adc bx,0x0
0000A2A6  8B4402            mov ax,[si+0x2]
0000A2A9  0BC0              or ax,ax
0000A2AB  7410              jz 0xa2bd
0000A2AD  8B5504            mov dx,[di+0x4]
0000A2B0  0BD2              or dx,dx
0000A2B2  7409              jz 0xa2bd
0000A2B4  F7E2              mul dx
0000A2B6  03E8              add bp,ax
0000A2B8  13CA              adc cx,dx
0000A2BA  83D300            adc bx,0x0
0000A2BD  8B4404            mov ax,[si+0x4]
0000A2C0  0BC0              or ax,ax
0000A2C2  7410              jz 0xa2d4
0000A2C4  8B5502            mov dx,[di+0x2]
0000A2C7  0BD2              or dx,dx
0000A2C9  7409              jz 0xa2d4
0000A2CB  F7E2              mul dx
0000A2CD  03E8              add bp,ax
0000A2CF  13CA              adc cx,dx
0000A2D1  83D300            adc bx,0x0
0000A2D4  8B4406            mov ax,[si+0x6]
0000A2D7  8B15              mov dx,[di]
0000A2D9  0BD2              or dx,dx
0000A2DB  7409              jz 0xa2e6
0000A2DD  F7E2              mul dx
0000A2DF  03E8              add bp,ax
0000A2E1  13CA              adc cx,dx
0000A2E3  83D300            adc bx,0x0
0000A2E6  8BD5              mov dx,bp
0000A2E8  81E5FF3F          and bp,0x3fff
0000A2EC  58                pop ax
0000A2ED  0BC5              or ax,bp
0000A2EF  50                push ax
0000A2F0  33ED              xor bp,bp
0000A2F2  52                push dx
0000A2F3  8B4402            mov ax,[si+0x2]
0000A2F6  0BC0              or ax,ax
0000A2F8  740A              jz 0xa304
0000A2FA  F76506            mul word [di+0x6]
0000A2FD  03C8              add cx,ax
0000A2FF  13DA              adc bx,dx
0000A301  83D500            adc bp,0x0
0000A304  8B4404            mov ax,[si+0x4]
0000A307  0BC0              or ax,ax
0000A309  7410              jz 0xa31b
0000A30B  8B5504            mov dx,[di+0x4]
0000A30E  0BD2              or dx,dx
0000A310  7409              jz 0xa31b
0000A312  F7E2              mul dx
0000A314  03C8              add cx,ax
0000A316  13DA              adc bx,dx
0000A318  83D500            adc bp,0x0
0000A31B  8B4406            mov ax,[si+0x6]
0000A31E  8B5502            mov dx,[di+0x2]
0000A321  0BD2              or dx,dx
0000A323  7409              jz 0xa32e
0000A325  F7E2              mul dx
0000A327  03C8              add cx,ax
0000A329  13DA              adc bx,dx
0000A32B  83D500            adc bp,0x0
0000A32E  51                push cx
0000A32F  33C9              xor cx,cx
0000A331  8B4404            mov ax,[si+0x4]
0000A334  0BC0              or ax,ax
0000A336  740A              jz 0xa342
0000A338  F76506            mul word [di+0x6]
0000A33B  03D8              add bx,ax
0000A33D  13EA              adc bp,dx
0000A33F  83D100            adc cx,0x0
0000A342  8B4406            mov ax,[si+0x6]
0000A345  8B5504            mov dx,[di+0x4]
0000A348  0BD2              or dx,dx
0000A34A  7409              jz 0xa355
0000A34C  F7E2              mul dx
0000A34E  03D8              add bx,ax
0000A350  13EA              adc bp,dx
0000A352  83D100            adc cx,0x0
0000A355  8B4406            mov ax,[si+0x6]
0000A358  F76506            mul word [di+0x6]
0000A35B  03C5              add ax,bp
0000A35D  13D1              adc dx,cx
0000A35F  59                pop cx
0000A360  5D                pop bp
0000A361  8BFA              mov di,dx
0000A363  8BD1              mov dx,cx
0000A365  8BCB              mov cx,bx
0000A367  8BD8              mov bx,ax
0000A369  58                pop ax
0000A36A  0BC0              or ax,ax
0000A36C  7403              jz 0xa371
0000A36E  83CD01            or bp,0x1
0000A371  5E                pop si
0000A372  E96101            jmp 0xa4d6
0000A375  F9                stc
0000A376  1BC1              sbb ax,cx
0000A378  32F2              xor dh,dl
0000A37A  55                push bp
0000A37B  52                push dx
0000A37C  56                push si
0000A37D  57                push di
0000A37E  83C606            add si,0x6
0000A381  83C706            add di,0x6
0000A384  B90400            mov cx,0x4
0000A387  FD                std
0000A388  F3A7              repe cmpsw
0000A38A  FC                cld
0000A38B  5F                pop di
0000A38C  5E                pop si
0000A38D  9C                pushf
0000A38E  8BE8              mov bp,ax
0000A390  AD                lodsw
0000A391  8BC8              mov cx,ax
0000A393  AD                lodsw
0000A394  8BD8              mov bx,ax
0000A396  AD                lodsw
0000A397  8BD0              mov dx,ax
0000A399  AD                lodsw
0000A39A  92                xchg ax,dx
0000A39B  8BF7              mov si,di
0000A39D  BF9600            mov di,0x96
0000A3A0  A5                movsw
0000A3A1  A5                movsw
0000A3A2  A5                movsw
0000A3A3  A5                movsw
0000A3A4  33FF              xor di,di
0000A3A6  9D                popf
0000A3A7  720B              jc 0xa3b4
0000A3A9  D1EA              shr dx,1
0000A3AB  D1D8              rcr ax,1
0000A3AD  D1DB              rcr bx,1
0000A3AF  D1D9              rcr cx,1
0000A3B1  D1DF              rcr di,1
0000A3B3  45                inc bp
0000A3B4  55                push bp
0000A3B5  893E1A00          mov [0x1a],di
0000A3B9  E84A00            call 0xa406
0000A3BC  57                push di
0000A3BD  C7061A000000      mov word [0x1a],0x0
0000A3C3  E84000            call 0xa406
0000A3C6  57                push di
0000A3C7  E83C00            call 0xa406
0000A3CA  57                push di
0000A3CB  E83800            call 0xa406
0000A3CE  BD0180            mov bp,0x8001
0000A3D1  D1E1              shl cx,1
0000A3D3  D1D3              rcl bx,1
0000A3D5  D1D0              rcl ax,1
0000A3D7  D1D2              rcl dx,1
0000A3D9  7222              jc 0xa3fd
0000A3DB  BE9600            mov si,0x96
0000A3DE  3B5406            cmp dx,[si+0x6]
0000A3E1  750C              jnz 0xa3ef
0000A3E3  3B4404            cmp ax,[si+0x4]
0000A3E6  7507              jnz 0xa3ef
0000A3E8  3B5C02            cmp bx,[si+0x2]
0000A3EB  7502              jnz 0xa3ef
0000A3ED  3B0C              cmp cx,[si]
0000A3EF  730C              jnc 0xa3fd
0000A3F1  0BC2              or ax,dx
0000A3F3  0BC1              or ax,cx
0000A3F5  0BC3              or ax,bx
0000A3F7  0AC4              or al,ah
0000A3F9  32E4              xor ah,ah
0000A3FB  8BE8              mov bp,ax
0000A3FD  8BD7              mov dx,di
0000A3FF  59                pop cx
0000A400  5B                pop bx
0000A401  5F                pop di
0000A402  5E                pop si
0000A403  E93C01            jmp 0xa542
0000A406  8B369C00          mov si,[0x9c]
0000A40A  33FF              xor di,di
0000A40C  3BD6              cmp dx,si
0000A40E  7362              jnc 0xa472
0000A410  0BD2              or dx,dx
0000A412  7504              jnz 0xa418
0000A414  3BF0              cmp si,ax
0000A416  7741              ja 0xa459
0000A418  F7F6              div si
0000A41A  52                push dx
0000A41B  53                push bx
0000A41C  97                xchg ax,di
0000A41D  33ED              xor bp,bp
0000A41F  8BF5              mov si,bp
0000A421  A19600            mov ax,[0x96]
0000A424  0BC0              or ax,ax
0000A426  7404              jz 0xa42c
0000A428  F7E7              mul di
0000A42A  8BF2              mov si,dx
0000A42C  50                push ax
0000A42D  A19800            mov ax,[0x98]
0000A430  0BC0              or ax,ax
0000A432  7406              jz 0xa43a
0000A434  F7E7              mul di
0000A436  03F0              add si,ax
0000A438  13EA              adc bp,dx
0000A43A  A19A00            mov ax,[0x9a]
0000A43D  0BC0              or ax,ax
0000A43F  7408              jz 0xa449
0000A441  F7E7              mul di
0000A443  03E8              add bp,ax
0000A445  83D200            adc dx,0x0
0000A448  92                xchg ax,dx
0000A449  8B161A00          mov dx,[0x1a]
0000A44D  5B                pop bx
0000A44E  2BD3              sub dx,bx
0000A450  1BCE              sbb cx,si
0000A452  5B                pop bx
0000A453  1BDD              sbb bx,bp
0000A455  5D                pop bp
0000A456  1BE8              sbb bp,ax
0000A458  95                xchg ax,bp
0000A459  92                xchg ax,dx
0000A45A  91                xchg ax,cx
0000A45B  93                xchg ax,bx
0000A45C  7313              jnc 0xa471
0000A45E  4F                dec di
0000A45F  030E9600          add cx,[0x96]
0000A463  131E9800          adc bx,[0x98]
0000A467  13069A00          adc ax,[0x9a]
0000A46B  13169C00          adc dx,[0x9c]
0000A46F  73ED              jnc 0xa45e
0000A471  C3                ret
0000A472  4F                dec di
0000A473  2B0E9600          sub cx,[0x96]
0000A477  1B1E9800          sbb bx,[0x98]
0000A47B  1B069A00          sbb ax,[0x9a]
0000A47F  030E9800          add cx,[0x98]
0000A483  131E9A00          adc bx,[0x9a]
0000A487  13C2              adc ax,dx
0000A489  8B169600          mov dx,[0x96]
0000A48D  F5                cmc
0000A48E  EBC9              jmp 0xa459
0000A490  B004              mov al,0x4
0000A492  0BFF              or di,di
0000A494  751F              jnz 0xa4b5
0000A496  83EE10            sub si,0x10
0000A499  FEC8              dec al
0000A49B  740C              jz 0xa4a9
0000A49D  8BFB              mov di,bx
0000A49F  8BD9              mov bx,cx
0000A4A1  8BCA              mov cx,dx
0000A4A3  8BD5              mov dx,bp
0000A4A5  33ED              xor bp,bp
0000A4A7  EBE9              jmp 0xa492
0000A4A9  BE7208            mov si,0x872
0000A4AC  8B3E9400          mov di,[0x94]
0000A4B0  58                pop ax
0000A4B1  5D                pop bp
0000A4B2  E9C9FB            jmp 0xa07e
0000A4B5  F7C700FF          test di,0xff00
0000A4B9  751B              jnz 0xa4d6
0000A4BB  83EE08            sub si,0x8
0000A4BE  97                xchg ax,di
0000A4BF  8AE0              mov ah,al
0000A4C1  8AC7              mov al,bh
0000A4C3  8AFB              mov bh,bl
0000A4C5  8ADD              mov bl,ch
0000A4C7  8AE9              mov ch,cl
0000A4C9  8ACE              mov cl,dh
0000A4CB  8AF2              mov dh,dl
0000A4CD  97                xchg ax,di
0000A4CE  95                xchg ax,bp
0000A4CF  8AD4              mov dl,ah
0000A4D1  8AE0              mov ah,al
0000A4D3  32C0              xor al,al
0000A4D5  95                xchg ax,bp
0000A4D6  F7C70080          test di,0x8000
0000A4DA  7566              jnz 0xa542
0000A4DC  4E                dec si
0000A4DD  D1E5              shl bp,1
0000A4DF  D1D2              rcl dx,1
0000A4E1  D1D1              rcl cx,1
0000A4E3  D1D3              rcl bx,1
0000A4E5  D1D7              rcl di,1
0000A4E7  F7C70080          test di,0x8000
0000A4EB  74EF              jz 0xa4dc
0000A4ED  EB53              jmp 0xa542
0000A4EF  0BD5              or dx,bp
0000A4F1  0BCA              or cx,dx
0000A4F3  0ACD              or cl,ch
0000A4F5  8AEB              mov ch,bl
0000A4F7  0BC9              or cx,cx
0000A4F9  7444              jz 0xa53f
0000A4FB  830E160020        or word [0x16],0x20
0000A500  D0E8              shr al,1
0000A502  721B              jc 0xa51f
0000A504  D0E8              shr al,1
0000A506  720F              jc 0xa517
0000A508  81F90080          cmp cx,0x8000
0000A50C  771D              ja 0xa52b
0000A50E  7227              jc 0xa537
0000A510  F6C701            test bh,0x1
0000A513  7422              jz 0xa537
0000A515  EB14              jmp 0xa52b
0000A517  58                pop ax
0000A518  50                push ax
0000A519  D0E4              shl ah,1
0000A51B  721A              jc 0xa537
0000A51D  EB0C              jmp 0xa52b
0000A51F  D0E8              shr al,1
0000A521  7214              jc 0xa537
0000A523  58                pop ax
0000A524  50                push ax
0000A525  D0E4              shl ah,1
0000A527  7202              jc 0xa52b
0000A529  EB0C              jmp 0xa537
0000A52B  80C701            add bh,0x1
0000A52E  83D700            adc di,0x0
0000A531  7304              jnc 0xa537
0000A533  BF0080            mov di,0x8000
0000A536  46                inc si
0000A537  33ED              xor bp,bp
0000A539  8BD5              mov dx,bp
0000A53B  8BCA              mov cx,dx
0000A53D  8AD9              mov bl,cl
0000A53F  E9AB00            jmp 0xa5ed
0000A542  A00B00            mov al,[0xb]
0000A545  D0E8              shr al,1
0000A547  725D              jc 0xa5a6
0000A549  D0E8              shr al,1
0000A54B  73A2              jnc 0xa4ef
0000A54D  95                xchg ax,bp
0000A54E  0AC4              or al,ah
0000A550  0AC2              or al,dl
0000A552  8AE6              mov ah,dh
0000A554  80E407            and ah,0x7
0000A557  80E6F8            and dh,0xf8
0000A55A  95                xchg ax,bp
0000A55B  0BED              or bp,bp
0000A55D  74E0              jz 0xa53f
0000A55F  830E160020        or word [0x16],0x20
0000A564  D0E8              shr al,1
0000A566  721B              jc 0xa583
0000A568  D0E8              shr al,1
0000A56A  720F              jc 0xa57b
0000A56C  81FD0004          cmp bp,0x400
0000A570  771D              ja 0xa58f
0000A572  722C              jc 0xa5a0
0000A574  F6C608            test dh,0x8
0000A577  7427              jz 0xa5a0
0000A579  EB14              jmp 0xa58f
0000A57B  58                pop ax
0000A57C  50                push ax
0000A57D  D0E4              shl ah,1
0000A57F  721F              jc 0xa5a0
0000A581  EB0C              jmp 0xa58f
0000A583  D0E8              shr al,1
0000A585  7219              jc 0xa5a0
0000A587  58                pop ax
0000A588  50                push ax
0000A589  D0E4              shl ah,1
0000A58B  7202              jc 0xa58f
0000A58D  EB11              jmp 0xa5a0
0000A58F  33ED              xor bp,bp
0000A591  80C608            add dh,0x8
0000A594  13CD              adc cx,bp
0000A596  13DD              adc bx,bp
0000A598  13FD              adc di,bp
0000A59A  7304              jnc 0xa5a0
0000A59C  BF0080            mov di,0x8000
0000A59F  46                inc si
0000A5A0  33ED              xor bp,bp
0000A5A2  32D2              xor dl,dl
0000A5A4  EB47              jmp 0xa5ed
0000A5A6  D0E8              shr al,1
0000A5A8  0BED              or bp,bp
0000A5AA  7441              jz 0xa5ed
0000A5AC  830E160020        or word [0x16],0x20
0000A5B1  D0E8              shr al,1
0000A5B3  721B              jc 0xa5d0
0000A5B5  D0E8              shr al,1
0000A5B7  720F              jc 0xa5c8
0000A5B9  81FD0080          cmp bp,0x8000
0000A5BD  771D              ja 0xa5dc
0000A5BF  722C              jc 0xa5ed
0000A5C1  F6C201            test dl,0x1
0000A5C4  7427              jz 0xa5ed
0000A5C6  EB14              jmp 0xa5dc
0000A5C8  58                pop ax
0000A5C9  50                push ax
0000A5CA  D0E4              shl ah,1
0000A5CC  721F              jc 0xa5ed
0000A5CE  EB0C              jmp 0xa5dc
0000A5D0  D0E8              shr al,1
0000A5D2  7219              jc 0xa5ed
0000A5D4  58                pop ax
0000A5D5  50                push ax
0000A5D6  D0E4              shl ah,1
0000A5D8  7202              jc 0xa5dc
0000A5DA  EB11              jmp 0xa5ed
0000A5DC  33ED              xor bp,bp
0000A5DE  83C201            add dx,0x1
0000A5E1  13CD              adc cx,bp
0000A5E3  13DD              adc bx,bp
0000A5E5  13FD              adc di,bp
0000A5E7  7304              jnc 0xa5ed
0000A5E9  BF0080            mov di,0x8000
0000A5EC  46                inc si
0000A5ED  A19400            mov ax,[0x94]
0000A5F0  97                xchg ax,di
0000A5F1  8915              mov [di],dx
0000A5F3  894D02            mov [di+0x2],cx
0000A5F6  895D04            mov [di+0x4],bx
0000A5F9  894506            mov [di+0x6],ax
0000A5FC  58                pop ax
0000A5FD  5D                pop bp
0000A5FE  80E480            and ah,0x80
0000A601  88650A            mov [di+0xa],ah
0000A604  81FE0040          cmp si,0x4000
0000A608  7D18              jnl 0xa622
0000A60A  81FE01C0          cmp si,0xc001
0000A60E  7E08              jng 0xa618
0000A610  897508            mov [di+0x8],si
0000A613  C6450B00          mov byte [di+0xb],0x0
0000A617  C3                ret
0000A618  BE7208            mov si,0x872
0000A61B  E860FA            call 0xa07e
0000A61E  88650A            mov [di+0xa],ah
0000A621  C3                ret
0000A622  830E160008        or word [0x16],0x8
0000A627  A00B00            mov al,[0xb]
0000A62A  D0E8              shr al,1
0000A62C  D0E8              shr al,1
0000A62E  D0E8              shr al,1
0000A630  720E              jc 0xa640
0000A632  D0E8              shr al,1
0000A634  7222              jc 0xa658
0000A636  BE7E08            mov si,0x87e
0000A639  E842FA            call 0xa07e
0000A63C  88650A            mov [di+0xa],ah
0000A63F  C3                ret
0000A640  D0E8              shr al,1
0000A642  720A              jc 0xa64e
0000A644  830E160020        or word [0x16],0x20
0000A649  F6C480            test ah,0x80
0000A64C  75E8              jnz 0xa636
0000A64E  BE9608            mov si,0x896
0000A651  E82AFA            call 0xa07e
0000A654  88650A            mov [di+0xa],ah
0000A657  C3                ret
0000A658  830E160020        or word [0x16],0x20
0000A65D  F6C480            test ah,0x80
0000A660  74D4              jz 0xa636
0000A662  EBEA              jmp 0xa64e
0000A664  26AD              es lodsw
0000A666  8BF8              mov di,ax
0000A668  26AD              es lodsw
0000A66A  8AD0              mov dl,al
0000A66C  D1C0              rol ax,1
0000A66E  2401              and al,0x1
0000A670  D0C8              ror al,1
0000A672  86C4              xchg al,ah
0000A674  8B361000          mov si,[0x10]
0000A678  3B361200          cmp si,[0x12]
0000A67C  7503              jnz 0xa681
0000A67E  E858ED            call 0x93d9
0000A681  83C60C            add si,0xc
0000A684  89361000          mov [0x10],si
0000A688  80CA80            or dl,0x80
0000A68B  32F6              xor dh,dh
0000A68D  3CFF              cmp al,0xff
0000A68F  7421              jz 0xa6b2
0000A691  3C00              cmp al,0x0
0000A693  7433              jz 0xa6c8
0000A695  88640A            mov [si+0xa],ah
0000A698  2C7F              sub al,0x7f
0000A69A  98                cbw
0000A69B  894408            mov [si+0x8],ax
0000A69E  88740B            mov [si+0xb],dh
0000A6A1  885407            mov [si+0x7],dl
0000A6A4  897C05            mov [si+0x5],di
0000A6A7  33C0              xor ax,ax
0000A6A9  884404            mov [si+0x4],al
0000A6AC  894402            mov [si+0x2],ax
0000A6AF  8904              mov [si],ax
0000A6B1  C3                ret
0000A6B2  88640A            mov [si+0xa],ah
0000A6B5  B80040            mov ax,0x4000
0000A6B8  B602              mov dh,0x2
0000A6BA  80FA80            cmp dl,0x80
0000A6BD  75DC              jnz 0xa69b
0000A6BF  0BFF              or di,di
0000A6C1  75D8              jnz 0xa69b
0000A6C3  80CE01            or dh,0x1
0000A6C6  EBD3              jmp 0xa69b
0000A6C8  88640A            mov [si+0xa],ah
0000A6CB  80FA80            cmp dl,0x80
0000A6CE  750C              jnz 0xa6dc
0000A6D0  0BFF              or di,di
0000A6D2  7508              jnz 0xa6dc
0000A6D4  B801C0            mov ax,0xc001
0000A6D7  80CE01            or dh,0x1
0000A6DA  EBBF              jmp 0xa69b
0000A6DC  830E160002        or word [0x16],0x2
0000A6E1  2C7F              sub al,0x7f
0000A6E3  98                cbw
0000A6E4  40                inc ax
0000A6E5  48                dec ax
0000A6E6  D1E7              shl di,1
0000A6E8  D0D2              rcl dl,1
0000A6EA  0AD2              or dl,dl
0000A6EC  79F7              jns 0xa6e5
0000A6EE  EBAB              jmp 0xa69b
0000A6F0  F6C102            test cl,0x2
0000A6F3  7507              jnz 0xa6fc
0000A6F5  33C0              xor ax,ax
0000A6F7  8BD8              mov bx,ax
0000A6F9  E99800            jmp 0xa794
0000A6FC  F6C101            test cl,0x1
0000A6FF  7512              jnz 0xa713
0000A701  8B5C05            mov bx,[si+0x5]
0000A704  8A4407            mov al,[si+0x7]
0000A707  8A640A            mov ah,[si+0xa]
0000A70A  D1E0              shl ax,1
0000A70C  B4FF              mov ah,0xff
0000A70E  D1D8              rcr ax,1
0000A710  E98100            jmp 0xa794
0000A713  8A640A            mov ah,[si+0xa]
0000A716  E9A600            jmp 0xa7bf
0000A719  E98E00            jmp 0xa7aa
0000A71C  E9D800            jmp 0xa7f7
0000A71F  8BFE              mov di,si
0000A721  8B361000          mov si,[0x10]
0000A725  8B440A            mov ax,[si+0xa]
0000A728  8A4C0B            mov cl,[si+0xb]
0000A72B  0AC9              or cl,cl
0000A72D  75C1              jnz 0xa6f0
0000A72F  8A4C0A            mov cl,[si+0xa]
0000A732  8B4408            mov ax,[si+0x8]
0000A735  3D8000            cmp ax,0x80
0000A738  7DDF              jnl 0xa719
0000A73A  3D81FF            cmp ax,0xff81
0000A73D  7EDD              jng 0xa71c
0000A73F  047F              add al,0x7f
0000A741  8A6407            mov ah,[si+0x7]
0000A744  86E0              xchg ah,al
0000A746  D0E0              shl al,1
0000A748  D0E1              shl cl,1
0000A74A  D1D8              rcr ax,1
0000A74C  8B14              mov dx,[si]
0000A74E  0B5402            or dx,[si+0x2]
0000A751  0AD6              or dl,dh
0000A753  32F6              xor dh,dh
0000A755  8B5C05            mov bx,[si+0x5]
0000A758  0B5403            or dx,[si+0x3]
0000A75B  7437              jz 0xa794
0000A75D  830E160020        or word [0x16],0x20
0000A762  8A0E0B00          mov cl,[0xb]
0000A766  D0E9              shr cl,1
0000A768  D0E9              shr cl,1
0000A76A  D0E9              shr cl,1
0000A76C  722C              jc 0xa79a
0000A76E  D0E9              shr cl,1
0000A770  7232              jc 0xa7a4
0000A772  81FA0080          cmp dx,0x8000
0000A776  721C              jc 0xa794
0000A778  7705              ja 0xa77f
0000A77A  F6C301            test bl,0x1
0000A77D  7415              jz 0xa794
0000A77F  8AD0              mov dl,al
0000A781  83C301            add bx,0x1
0000A784  150000            adc ax,0x0
0000A787  32D0              xor dl,al
0000A789  7909              jns 0xa794
0000A78B  8BD0              mov dx,ax
0000A78D  D1E2              shl dx,1
0000A78F  80FEFF            cmp dh,0xff
0000A792  7416              jz 0xa7aa
0000A794  93                xchg ax,bx
0000A795  AB                stosw
0000A796  8BC3              mov ax,bx
0000A798  AB                stosw
0000A799  C3                ret
0000A79A  D0E9              shr cl,1
0000A79C  72F6              jc 0xa794
0000A79E  0AE4              or ah,ah
0000A7A0  78DD              js 0xa77f
0000A7A2  EBF0              jmp 0xa794
0000A7A4  0AE4              or ah,ah
0000A7A6  78EC              js 0xa794
0000A7A8  EBD5              jmp 0xa77f
0000A7AA  830E160028        or word [0x16],0x28
0000A7AF  8A0E0B00          mov cl,[0xb]
0000A7B3  D0E9              shr cl,1
0000A7B5  D0E9              shr cl,1
0000A7B7  D0E9              shr cl,1
0000A7B9  7218              jc 0xa7d3
0000A7BB  D0E9              shr cl,1
0000A7BD  7231              jc 0xa7f0
0000A7BF  2E8B1EA408        mov bx,[cs:0x8a4]
0000A7C4  80E480            and ah,0x80
0000A7C7  0AFC              or bh,ah
0000A7C9  2EA1A208          mov ax,[cs:0x8a2]
0000A7CD  AB                stosw
0000A7CE  8BC3              mov ax,bx
0000A7D0  AB                stosw
0000A7D1  EBC6              jmp 0xa799
0000A7D3  D0E9              shr cl,1
0000A7D5  7205              jc 0xa7dc
0000A7D7  F6C480            test ah,0x80
0000A7DA  75E3              jnz 0xa7bf
0000A7DC  2E8B1EA808        mov bx,[cs:0x8a8]
0000A7E1  80E480            and ah,0x80
0000A7E4  0AE7              or ah,bh
0000A7E6  8AC3              mov al,bl
0000A7E8  AB                stosw
0000A7E9  2EA1A608          mov ax,[cs:0x8a6]
0000A7ED  AB                stosw
0000A7EE  EBA9              jmp 0xa799
0000A7F0  F6C480            test ah,0x80
0000A7F3  74CA              jz 0xa7bf
0000A7F5  EBE5              jmp 0xa7dc
0000A7F7  830E160030        or word [0x16],0x30
0000A7FC  F7D8              neg ax
0000A7FE  0582FF            add ax,0xff82
0000A801  3D1800            cmp ax,0x18
0000A804  7D37              jnl 0xa83d
0000A806  91                xchg ax,cx
0000A807  8B14              mov dx,[si]
0000A809  0B5402            or dx,[si+0x2]
0000A80C  8AC2              mov al,dl
0000A80E  0AC6              or al,dh
0000A810  8B5404            mov dx,[si+0x4]
0000A813  8B5C06            mov bx,[si+0x6]
0000A816  0AC0              or al,al
0000A818  7403              jz 0xa81d
0000A81A  80CA01            or dl,0x1
0000A81D  D1EB              shr bx,1
0000A81F  D1DA              rcr dx,1
0000A821  7303              jnc 0xa826
0000A823  80CA01            or dl,0x1
0000A826  E2F5              loop 0xa81d
0000A828  86E5              xchg ah,ch
0000A82A  8A640A            mov ah,[si+0xa]
0000A82D  80E480            and ah,0x80
0000A830  8AC7              mov al,bh
0000A832  8AFB              mov bh,bl
0000A834  8ADE              mov bl,dh
0000A836  8AF2              mov dh,dl
0000A838  32D2              xor dl,dl
0000A83A  E925FF            jmp 0xa762
0000A83D  33C0              xor ax,ax
0000A83F  8BD8              mov bx,ax
0000A841  E950FF            jmp 0xa794
0000A844  8BFE              mov di,si
0000A846  8B361000          mov si,[0x10]
0000A84A  3B361200          cmp si,[0x12]
0000A84E  7503              jnz 0xa853
0000A850  E886EB            call 0x93d9
0000A853  83C60C            add si,0xc
0000A856  89361000          mov [0x10],si
0000A85A  87FE              xchg di,si
0000A85C  26AD              es lodsw
0000A85E  8BE8              mov bp,ax
0000A860  26AD              es lodsw
0000A862  8BD0              mov dx,ax
0000A864  26AD              es lodsw
0000A866  8BC8              mov cx,ax
0000A868  26AD              es lodsw
0000A86A  87FE              xchg di,si
0000A86C  8BD8              mov bx,ax
0000A86E  D1E5              shl bp,1
0000A870  D1D2              rcl dx,1
0000A872  D1D1              rcl cx,1
0000A874  D1D3              rcl bx,1
0000A876  D1E5              shl bp,1
0000A878  D1D2              rcl dx,1
0000A87A  D1D1              rcl cx,1
0000A87C  D1D3              rcl bx,1
0000A87E  D1E5              shl bp,1
0000A880  D1D2              rcl dx,1
0000A882  D1D1              rcl cx,1
0000A884  D1D3              rcl bx,1
0000A886  80CB80            or bl,0x80
0000A889  885C07            mov [si+0x7],bl
0000A88C  894C05            mov [si+0x5],cx
0000A88F  895403            mov [si+0x3],dx
0000A892  896C01            mov [si+0x1],bp
0000A895  0BCD              or cx,bp
0000A897  0BCA              or cx,dx
0000A899  8AF4              mov dh,ah
0000A89B  80E680            and dh,0x80
0000A89E  88740A            mov [si+0xa],dh
0000A8A1  32F6              xor dh,dh
0000A8A3  8834              mov [si],dh
0000A8A5  80E47F            and ah,0x7f
0000A8A8  D1E8              shr ax,1
0000A8AA  D1E8              shr ax,1
0000A8AC  D1E8              shr ax,1
0000A8AE  D1E8              shr ax,1
0000A8B0  3DFF07            cmp ax,0x7ff
0000A8B3  740F              jz 0xa8c4
0000A8B5  3D0000            cmp ax,0x0
0000A8B8  741D              jz 0xa8d7
0000A8BA  2DFF03            sub ax,0x3ff
0000A8BD  894408            mov [si+0x8],ax
0000A8C0  88740B            mov [si+0xb],dh
0000A8C3  C3                ret
0000A8C4  B80040            mov ax,0x4000
0000A8C7  B602              mov dh,0x2
0000A8C9  80FB80            cmp bl,0x80
0000A8CC  75EF              jnz 0xa8bd
0000A8CE  0BC9              or cx,cx
0000A8D0  75EB              jnz 0xa8bd
0000A8D2  80CE01            or dh,0x1
0000A8D5  EBE6              jmp 0xa8bd
0000A8D7  80FB80            cmp bl,0x80
0000A8DA  750B              jnz 0xa8e7
0000A8DC  0BC9              or cx,cx
0000A8DE  7507              jnz 0xa8e7
0000A8E0  B801C0            mov ax,0xc001
0000A8E3  B601              mov dh,0x1
0000A8E5  EBD6              jmp 0xa8bd
0000A8E7  830E160002        or word [0x16],0x2
0000A8EC  2DFF03            sub ax,0x3ff
0000A8EF  8B2C              mov bp,[si]
0000A8F1  8B5402            mov dx,[si+0x2]
0000A8F4  8B4C04            mov cx,[si+0x4]
0000A8F7  8B5C06            mov bx,[si+0x6]
0000A8FA  40                inc ax
0000A8FB  48                dec ax
0000A8FC  D1E5              shl bp,1
0000A8FE  D1D2              rcl dx,1
0000A900  D1D1              rcl cx,1
0000A902  D1D3              rcl bx,1
0000A904  0BDB              or bx,bx
0000A906  79F3              jns 0xa8fb
0000A908  892C              mov [si],bp
0000A90A  895402            mov [si+0x2],dx
0000A90D  894C04            mov [si+0x4],cx
0000A910  895C06            mov [si+0x6],bx
0000A913  32F6              xor dh,dh
0000A915  EBA6              jmp 0xa8bd
0000A917  F6C102            test cl,0x2
0000A91A  7509              jnz 0xa925
0000A91C  33C0              xor ax,ax
0000A91E  AB                stosw
0000A91F  AB                stosw
0000A920  AB                stosw
0000A921  AB                stosw
0000A922  E91901            jmp 0xaa3e
0000A925  F6C101            test cl,0x1
0000A928  753D              jnz 0xa967
0000A92A  8B5401            mov dx,[si+0x1]
0000A92D  8B5C03            mov bx,[si+0x3]
0000A930  8B4405            mov ax,[si+0x5]
0000A933  8A4C07            mov cl,[si+0x7]
0000A936  D0E9              shr cl,1
0000A938  D1D8              rcr ax,1
0000A93A  D1DB              rcr bx,1
0000A93C  D1DA              rcr dx,1
0000A93E  D0E9              shr cl,1
0000A940  D1D8              rcr ax,1
0000A942  D1DB              rcr bx,1
0000A944  D1DA              rcr dx,1
0000A946  D0E9              shr cl,1
0000A948  D1D8              rcr ax,1
0000A94A  D1DB              rcr bx,1
0000A94C  D1DA              rcr dx,1
0000A94E  92                xchg ax,dx
0000A94F  AB                stosw
0000A950  8BC3              mov ax,bx
0000A952  AB                stosw
0000A953  8BC2              mov ax,dx
0000A955  AB                stosw
0000A956  8A7C0A            mov bh,[si+0xa]
0000A959  80E780            and bh,0x80
0000A95C  B8F07F            mov ax,0x7ff0
0000A95F  0AE7              or ah,bh
0000A961  0AC1              or al,cl
0000A963  AB                stosw
0000A964  E9D700            jmp 0xaa3e
0000A967  8A5C0A            mov bl,[si+0xa]
0000A96A  80E380            and bl,0x80
0000A96D  E9ED00            jmp 0xaa5d
0000A970  E9CF00            jmp 0xaa42
0000A973  E90A01            jmp 0xaa80
0000A976  EB9F              jmp 0xa917
0000A978  8BFE              mov di,si
0000A97A  8B361000          mov si,[0x10]
0000A97E  8A4C0B            mov cl,[si+0xb]
0000A981  0AC9              or cl,cl
0000A983  75F1              jnz 0xa976
0000A985  8A4C0A            mov cl,[si+0xa]
0000A988  8B6C08            mov bp,[si+0x8]
0000A98B  81FD0004          cmp bp,0x400
0000A98F  7DDF              jnl 0xa970
0000A991  81FD01FC          cmp bp,0xfc01
0000A995  7EDC              jng 0xa973
0000A997  8A04              mov al,[si]
0000A999  8B5401            mov dx,[si+0x1]
0000A99C  0AC0              or al,al
0000A99E  7403              jz 0xa9a3
0000A9A0  80CA01            or dl,0x1
0000A9A3  F6C207            test dl,0x7
0000A9A6  7452              jz 0xa9fa
0000A9A8  830E160020        or word [0x16],0x20
0000A9AD  A00B00            mov al,[0xb]
0000A9B0  D0E8              shr al,1
0000A9B2  D0E8              shr al,1
0000A9B4  D0E8              shr al,1
0000A9B6  723A              jc 0xa9f2
0000A9B8  D0E8              shr al,1
0000A9BA  7230              jc 0xa9ec
0000A9BC  F6C204            test dl,0x4
0000A9BF  7439              jz 0xa9fa
0000A9C1  F6C20B            test dl,0xb
0000A9C4  7434              jz 0xa9fa
0000A9C6  8B5C03            mov bx,[si+0x3]
0000A9C9  8B4405            mov ax,[si+0x5]
0000A9CC  8A4C07            mov cl,[si+0x7]
0000A9CF  80E17F            and cl,0x7f
0000A9D2  83C208            add dx,0x8
0000A9D5  83D300            adc bx,0x0
0000A9D8  150000            adc ax,0x0
0000A9DB  80D100            adc cl,0x0
0000A9DE  7926              jns 0xaa06
0000A9E0  80E17F            and cl,0x7f
0000A9E3  45                inc bp
0000A9E4  81FD0004          cmp bp,0x400
0000A9E8  7C1C              jl 0xaa06
0000A9EA  EB53              jmp 0xaa3f
0000A9EC  D0E1              shl cl,1
0000A9EE  73D6              jnc 0xa9c6
0000A9F0  EB08              jmp 0xa9fa
0000A9F2  D0E8              shr al,1
0000A9F4  7204              jc 0xa9fa
0000A9F6  D0E1              shl cl,1
0000A9F8  72CC              jc 0xa9c6
0000A9FA  8B5C03            mov bx,[si+0x3]
0000A9FD  8B4405            mov ax,[si+0x5]
0000AA00  8A4C07            mov cl,[si+0x7]
0000AA03  80E17F            and cl,0x7f
0000AA06  D0E9              shr cl,1
0000AA08  D1D8              rcr ax,1
0000AA0A  D1DB              rcr bx,1
0000AA0C  D1DA              rcr dx,1
0000AA0E  D0E9              shr cl,1
0000AA10  D1D8              rcr ax,1
0000AA12  D1DB              rcr bx,1
0000AA14  D1DA              rcr dx,1
0000AA16  D0E9              shr cl,1
0000AA18  D1D8              rcr ax,1
0000AA1A  D1DB              rcr bx,1
0000AA1C  D1DA              rcr dx,1
0000AA1E  92                xchg ax,dx
0000AA1F  AB                stosw
0000AA20  8BC3              mov ax,bx
0000AA22  AB                stosw
0000AA23  8BC2              mov ax,dx
0000AA25  AB                stosw
0000AA26  8BC5              mov ax,bp
0000AA28  05FF03            add ax,0x3ff
0000AA2B  D1E0              shl ax,1
0000AA2D  D1E0              shl ax,1
0000AA2F  D1E0              shl ax,1
0000AA31  D1E0              shl ax,1
0000AA33  0AC1              or al,cl
0000AA35  8A4C0A            mov cl,[si+0xa]
0000AA38  80E180            and cl,0x80
0000AA3B  0AE1              or ah,cl
0000AA3D  AB                stosw
0000AA3E  C3                ret
0000AA3F  83EF06            sub di,0x6
0000AA42  830E160028        or word [0x16],0x28
0000AA47  8A5C0A            mov bl,[si+0xa]
0000AA4A  80E380            and bl,0x80
0000AA4D  8A0E0B00          mov cl,[0xb]
0000AA51  D0E9              shr cl,1
0000AA53  D0E9              shr cl,1
0000AA55  D0E9              shr cl,1
0000AA57  7214              jc 0xaa6d
0000AA59  D0E9              shr cl,1
0000AA5B  721D              jc 0xaa7a
0000AA5D  BEAA08            mov si,0x8aa
0000AA60  2EA5              cs movsw
0000AA62  2EA5              cs movsw
0000AA64  2EA5              cs movsw
0000AA66  2EAD              cs lodsw
0000AA68  0AE3              or ah,bl
0000AA6A  AB                stosw
0000AA6B  EBD1              jmp 0xaa3e
0000AA6D  D0E9              shr cl,1
0000AA6F  7204              jc 0xaa75
0000AA71  0ADB              or bl,bl
0000AA73  75E8              jnz 0xaa5d
0000AA75  BEB208            mov si,0x8b2
0000AA78  EBE6              jmp 0xaa60
0000AA7A  0ADB              or bl,bl
0000AA7C  74DF              jz 0xaa5d
0000AA7E  EBF5              jmp 0xaa75
0000AA80  830E160030        or word [0x16],0x30
0000AA85  81C5FF03          add bp,0x3ff
0000AA89  F7DD              neg bp
0000AA8B  8BCD              mov cx,bp
0000AA8D  83C104            add cx,0x4
0000AA90  8A740A            mov dh,[si+0xa]
0000AA93  80E680            and dh,0x80
0000AA96  8A5407            mov dl,[si+0x7]
0000AA99  8B5C05            mov bx,[si+0x5]
0000AA9C  8B6C03            mov bp,[si+0x3]
0000AA9F  8B4401            mov ax,[si+0x1]
0000AAA2  D0EA              shr dl,1
0000AAA4  D1DB              rcr bx,1
0000AAA6  D1DD              rcr bp,1
0000AAA8  D1D8              rcr ax,1
0000AAAA  E2F6              loop 0xaaa2
0000AAAC  AB                stosw
0000AAAD  8BC5              mov ax,bp
0000AAAF  AB                stosw
0000AAB0  8BC3              mov ax,bx
0000AAB2  AB                stosw
0000AAB3  8BC2              mov ax,dx
0000AAB5  AB                stosw
0000AAB6  EB86              jmp 0xaa3e
0000AAB8  26AD              es lodsw
0000AABA  8BF8              mov di,ax
0000AABC  0BFF              or di,di
0000AABE  7431              jz 0xaaf1
0000AAC0  33ED              xor bp,bp
0000AAC2  8BDD              mov bx,bp
0000AAC4  8BD3              mov dx,bx
0000AAC6  B80F00            mov ax,0xf
0000AAC9  8B361000          mov si,[0x10]
0000AACD  3B361200          cmp si,[0x12]
0000AAD1  7503              jnz 0xaad6
0000AAD3  E803E9            call 0x93d9
0000AAD6  83C60C            add si,0xc
0000AAD9  89361000          mov [0x10],si
0000AADD  32C9              xor cl,cl
0000AADF  884C0B            mov [si+0xb],cl
0000AAE2  8BCF              mov cx,di
0000AAE4  80E580            and ch,0x80
0000AAE7  7902              jns 0xaaeb
0000AAE9  F7DF              neg di
0000AAEB  886C0A            mov [si+0xa],ch
0000AAEE  E95902            jmp 0xad4a
0000AAF1  8B361000          mov si,[0x10]
0000AAF5  3B361200          cmp si,[0x12]
0000AAF9  7503              jnz 0xaafe
0000AAFB  E8DBE8            call 0x93d9
0000AAFE  83C60C            add si,0xc
0000AB01  89361000          mov [0x10],si
0000AB05  33C0              xor ax,ax
0000AB07  8904              mov [si],ax
0000AB09  894402            mov [si+0x2],ax
0000AB0C  894404            mov [si+0x4],ax
0000AB0F  894406            mov [si+0x6],ax
0000AB12  C7440801C0        mov word [si+0x8],0xc001
0000AB17  88640A            mov [si+0xa],ah
0000AB1A  B401              mov ah,0x1
0000AB1C  88640B            mov [si+0xb],ah
0000AB1F  C3                ret
0000AB20  56                push si
0000AB21  8B361000          mov si,[0x10]
0000AB25  F6440B02          test byte [si+0xb],0x2
0000AB29  7538              jnz 0xab63
0000AB2B  F6440B01          test byte [si+0xb],0x1
0000AB2F  752E              jnz 0xab5f
0000AB31  8B4C08            mov cx,[si+0x8]
0000AB34  83F90F            cmp cx,0xf
0000AB37  7F2A              jg 0xab63
0000AB39  8B6C04            mov bp,[si+0x4]
0000AB3C  8B7C06            mov di,[si+0x6]
0000AB3F  8B14              mov dx,[si]
0000AB41  8B5C02            mov bx,[si+0x2]
0000AB44  E82A02            call 0xad71
0000AB47  0BDB              or bx,bx
0000AB49  7518              jnz 0xab63
0000AB4B  8A640A            mov ah,[si+0xa]
0000AB4E  0AE4              or ah,ah
0000AB50  7904              jns 0xab56
0000AB52  F7DA              neg dx
0000AB54  7404              jz 0xab5a
0000AB56  33C2              xor ax,dx
0000AB58  7809              js 0xab63
0000AB5A  5F                pop di
0000AB5B  8BC2              mov ax,dx
0000AB5D  AB                stosw
0000AB5E  C3                ret
0000AB5F  33D2              xor dx,dx
0000AB61  EBF7              jmp 0xab5a
0000AB63  830E160001        or word [0x16],0x1
0000AB68  BA0080            mov dx,0x8000
0000AB6B  EBED              jmp 0xab5a
0000AB6D  26AD              es lodsw
0000AB6F  8BE8              mov bp,ax
0000AB71  26AD              es lodsw
0000AB73  8BF8              mov di,ax
0000AB75  0BC5              or ax,bp
0000AB77  743D              jz 0xabb6
0000AB79  33DB              xor bx,bx
0000AB7B  8BD3              mov dx,bx
0000AB7D  B81F00            mov ax,0x1f
0000AB80  8B361000          mov si,[0x10]
0000AB84  3B361200          cmp si,[0x12]
0000AB88  7503              jnz 0xab8d
0000AB8A  E84CE8            call 0x93d9
0000AB8D  83C60C            add si,0xc
0000AB90  89361000          mov [0x10],si
0000AB94  32C9              xor cl,cl
0000AB96  884C0B            mov [si+0xb],cl
0000AB99  8BCF              mov cx,di
0000AB9B  80E580            and ch,0x80
0000AB9E  790C              jns 0xabac
0000ABA0  83F7FF            xor di,0xffffffffffffffff
0000ABA3  83F5FF            xor bp,0xffffffffffffffff
0000ABA6  83C501            add bp,0x1
0000ABA9  83D700            adc di,0x0
0000ABAC  886C0A            mov [si+0xa],ch
0000ABAF  0BFF              or di,di
0000ABB1  7406              jz 0xabb9
0000ABB3  E99401            jmp 0xad4a
0000ABB6  E938FF            jmp 0xaaf1
0000ABB9  8BFD              mov di,bp
0000ABBB  33ED              xor bp,bp
0000ABBD  2D1000            sub ax,0x10
0000ABC0  E98701            jmp 0xad4a
0000ABC3  56                push si
0000ABC4  E80800            call 0xabcf
0000ABC7  5F                pop di
0000ABC8  8BC2              mov ax,dx
0000ABCA  AB                stosw
0000ABCB  8BC3              mov ax,bx
0000ABCD  AB                stosw
0000ABCE  C3                ret
0000ABCF  8B361000          mov si,[0x10]
0000ABD3  F6440B02          test byte [si+0xb],0x2
0000ABD7  7541              jnz 0xac1a
0000ABD9  F6440B01          test byte [si+0xb],0x1
0000ABDD  7532              jnz 0xac11
0000ABDF  8B4C08            mov cx,[si+0x8]
0000ABE2  83F91F            cmp cx,0x1f
0000ABE5  7F33              jg 0xac1a
0000ABE7  8B6C04            mov bp,[si+0x4]
0000ABEA  8B7C06            mov di,[si+0x6]
0000ABED  8B14              mov dx,[si]
0000ABEF  8B5C02            mov bx,[si+0x2]
0000ABF2  E87C01            call 0xad71
0000ABF5  0BED              or bp,bp
0000ABF7  7521              jnz 0xac1a
0000ABF9  8A640A            mov ah,[si+0xa]
0000ABFC  0AE4              or ah,ah
0000ABFE  790C              jns 0xac0c
0000AC00  83F3FF            xor bx,0xffffffffffffffff
0000AC03  83F2FF            xor dx,0xffffffffffffffff
0000AC06  83C201            add dx,0x1
0000AC09  83D300            adc bx,0x0
0000AC0C  33C3              xor ax,bx
0000AC0E  7806              js 0xac16
0000AC10  C3                ret
0000AC11  33D2              xor dx,dx
0000AC13  8BDA              mov bx,dx
0000AC15  C3                ret
0000AC16  0BDA              or bx,dx
0000AC18  74F6              jz 0xac10
0000AC1A  830E160001        or word [0x16],0x1
0000AC1F  BB0080            mov bx,0x8000
0000AC22  33D2              xor dx,dx
0000AC24  C3                ret
0000AC25  26AD              es lodsw
0000AC27  8BD0              mov dx,ax
0000AC29  26AD              es lodsw
0000AC2B  8BD8              mov bx,ax
0000AC2D  26AD              es lodsw
0000AC2F  8BE8              mov bp,ax
0000AC31  26AD              es lodsw
0000AC33  8BF8              mov di,ax
0000AC35  0BC5              or ax,bp
0000AC37  0BC3              or ax,bx
0000AC39  0BC2              or ax,dx
0000AC3B  743F              jz 0xac7c
0000AC3D  B83F00            mov ax,0x3f
0000AC40  8B361000          mov si,[0x10]
0000AC44  3B361200          cmp si,[0x12]
0000AC48  7503              jnz 0xac4d
0000AC4A  E88CE7            call 0x93d9
0000AC4D  83C60C            add si,0xc
0000AC50  89361000          mov [0x10],si
0000AC54  32C9              xor cl,cl
0000AC56  884C0B            mov [si+0xb],cl
0000AC59  8BCF              mov cx,di
0000AC5B  80E580            and ch,0x80
0000AC5E  7903              jns 0xac63
0000AC60  E89100            call 0xacf4
0000AC63  886C0A            mov [si+0xa],ch
0000AC66  0BFF              or di,di
0000AC68  750F              jnz 0xac79
0000AC6A  0BED              or bp,bp
0000AC6C  750B              jnz 0xac79
0000AC6E  8BFB              mov di,bx
0000AC70  8BEA              mov bp,dx
0000AC72  33DB              xor bx,bx
0000AC74  33D2              xor dx,dx
0000AC76  2D2000            sub ax,0x20
0000AC79  E9CE00            jmp 0xad4a
0000AC7C  E972FE            jmp 0xaaf1
0000AC7F  56                push si
0000AC80  E80E00            call 0xac91
0000AC83  97                xchg ax,di
0000AC84  92                xchg ax,dx
0000AC85  5F                pop di
0000AC86  AB                stosw
0000AC87  8BC3              mov ax,bx
0000AC89  AB                stosw
0000AC8A  8BC5              mov ax,bp
0000AC8C  AB                stosw
0000AC8D  8BC2              mov ax,dx
0000AC8F  AB                stosw
0000AC90  C3                ret
0000AC91  8B361000          mov si,[0x10]
0000AC95  F6440B02          test byte [si+0xb],0x2
0000AC99  754F              jnz 0xacea
0000AC9B  F6440B01          test byte [si+0xb],0x1
0000AC9F  7535              jnz 0xacd6
0000ACA1  8B4C08            mov cx,[si+0x8]
0000ACA4  83F93F            cmp cx,0x3f
0000ACA7  7F41              jg 0xacea
0000ACA9  8B6C04            mov bp,[si+0x4]
0000ACAC  8B7C06            mov di,[si+0x6]
0000ACAF  8B14              mov dx,[si]
0000ACB1  8B5C02            mov bx,[si+0x2]
0000ACB4  E8BA00            call 0xad71
0000ACB7  8A640A            mov ah,[si+0xa]
0000ACBA  0AE4              or ah,ah
0000ACBC  7903              jns 0xacc1
0000ACBE  E83300            call 0xacf4
0000ACC1  33C7              xor ax,di
0000ACC3  781B              js 0xace0
0000ACC5  3B360E00          cmp si,[0xe]
0000ACC9  7503              jnz 0xacce
0000ACCB  E81BE7            call 0x93e9
0000ACCE  83EE0C            sub si,0xc
0000ACD1  89361000          mov [0x10],si
0000ACD5  C3                ret
0000ACD6  33FF              xor di,di
0000ACD8  33ED              xor bp,bp
0000ACDA  8BDD              mov bx,bp
0000ACDC  8BD5              mov dx,bp
0000ACDE  EBE5              jmp 0xacc5
0000ACE0  0BFD              or di,bp
0000ACE2  0BFB              or di,bx
0000ACE4  0BFA              or di,dx
0000ACE6  7502              jnz 0xacea
0000ACE8  EBDB              jmp 0xacc5
0000ACEA  830E160001        or word [0x16],0x1
0000ACEF  BF0080            mov di,0x8000
0000ACF2  EBE4              jmp 0xacd8
0000ACF4  F7D7              not di
0000ACF6  F7D5              not bp
0000ACF8  F7D3              not bx
0000ACFA  F7DA              neg dx
0000ACFC  F5                cmc
0000ACFD  83D300            adc bx,0x0
0000AD00  83D500            adc bp,0x0
0000AD03  83D700            adc di,0x0
0000AD06  C3                ret
0000AD07  8B361000          mov si,[0x10]
0000AD0B  8B4C08            mov cx,[si+0x8]
0000AD0E  83F93F            cmp cx,0x3f
0000AD11  7D20              jnl 0xad33
0000AD13  8B6C04            mov bp,[si+0x4]
0000AD16  8B7C06            mov di,[si+0x6]
0000AD19  8B14              mov dx,[si]
0000AD1B  8B5C02            mov bx,[si+0x2]
0000AD1E  E85000            call 0xad71
0000AD21  33C0              xor ax,ax
0000AD23  0BC7              or ax,di
0000AD25  0BC5              or ax,bp
0000AD27  0BC3              or ax,bx
0000AD29  0BC2              or ax,dx
0000AD2B  7407              jz 0xad34
0000AD2D  B83F00            mov ax,0x3f
0000AD30  E81700            call 0xad4a
0000AD33  C3                ret
0000AD34  C7440801C0        mov word [si+0x8],0xc001
0000AD39  8904              mov [si],ax
0000AD3B  894402            mov [si+0x2],ax
0000AD3E  894404            mov [si+0x4],ax
0000AD41  894406            mov [si+0x6],ax
0000AD44  C6440B01          mov byte [si+0xb],0x1
0000AD48  EBE9              jmp 0xad33
0000AD4A  33C9              xor cx,cx
0000AD4C  D1E2              shl dx,1
0000AD4E  D1D3              rcl bx,1
0000AD50  D1D5              rcl bp,1
0000AD52  D1D7              rcl di,1
0000AD54  7202              jc 0xad58
0000AD56  E2F4              loop 0xad4c
0000AD58  D1DF              rcr di,1
0000AD5A  D1DD              rcr bp,1
0000AD5C  D1DB              rcr bx,1
0000AD5E  D1DA              rcr dx,1
0000AD60  03C1              add ax,cx
0000AD62  894408            mov [si+0x8],ax
0000AD65  8914              mov [si],dx
0000AD67  895C02            mov [si+0x2],bx
0000AD6A  896C04            mov [si+0x4],bp
0000AD6D  897C06            mov [si+0x6],di
0000AD70  C3                ret
0000AD71  33C0              xor ax,ax
0000AD73  83E93F            sub cx,0x3f
0000AD76  F7D9              neg cx
0000AD78  49                dec cx
0000AD79  7C61              jl 0xaddc
0000AD7B  742B              jz 0xada8
0000AD7D  83F940            cmp cx,0x40
0000AD80  7D5B              jnl 0xaddd
0000AD82  83F930            cmp cx,0x30
0000AD85  7E13              jng 0xad9a
0000AD87  0BD3              or dx,bx
0000AD89  0BD5              or dx,bp
0000AD8B  7402              jz 0xad8f
0000AD8D  0C01              or al,0x1
0000AD8F  8BD7              mov dx,di
0000AD91  33FF              xor di,di
0000AD93  8BEF              mov bp,di
0000AD95  8BDF              mov bx,di
0000AD97  83E930            sub cx,0x30
0000AD9A  D1EF              shr di,1
0000AD9C  D1DD              rcr bp,1
0000AD9E  D1DB              rcr bx,1
0000ADA0  D1DA              rcr dx,1
0000ADA2  7302              jnc 0xada6
0000ADA4  D0D0              rcl al,1
0000ADA6  E2F2              loop 0xad9a
0000ADA8  D1EF              shr di,1
0000ADAA  D1DD              rcr bp,1
0000ADAC  D1DB              rcr bx,1
0000ADAE  D1DA              rcr dx,1
0000ADB0  D0D4              rcl ah,1
0000ADB2  0BC0              or ax,ax
0000ADB4  7426              jz 0xaddc
0000ADB6  830E160020        or word [0x16],0x20
0000ADBB  F6060B0004        test byte [0xb],0x4
0000ADC0  7529              jnz 0xadeb
0000ADC2  F6060B0008        test byte [0xb],0x8
0000ADC7  7531              jnz 0xadfa
0000ADC9  0AC2              or al,dl
0000ADCB  22E0              and ah,al
0000ADCD  D0EC              shr ah,1
0000ADCF  730B              jnc 0xaddc
0000ADD1  33C0              xor ax,ax
0000ADD3  83C201            add dx,0x1
0000ADD6  13D8              adc bx,ax
0000ADD8  13E8              adc bp,ax
0000ADDA  13F8              adc di,ax
0000ADDC  C3                ret
0000ADDD  B001              mov al,0x1
0000ADDF  32E4              xor ah,ah
0000ADE1  33FF              xor di,di
0000ADE3  8BEF              mov bp,di
0000ADE5  8BDF              mov bx,di
0000ADE7  8BD7              mov dx,di
0000ADE9  EBC7              jmp 0xadb2
0000ADEB  F6060B0008        test byte [0xb],0x8
0000ADF0  75EA              jnz 0xaddc
0000ADF2  F6440A80          test byte [si+0xa],0x80
0000ADF6  75D9              jnz 0xadd1
0000ADF8  EBE2              jmp 0xaddc
0000ADFA  F6440A80          test byte [si+0xa],0x80
0000ADFE  75DC              jnz 0xaddc
0000AE00  EBCF              jmp 0xadd1
0000AE02  8BFE              mov di,si
0000AE04  8B361000          mov si,[0x10]
0000AE08  3B361200          cmp si,[0x12]
0000AE0C  7503              jnz 0xae11
0000AE0E  E8C8E5            call 0x93d9
0000AE11  83C60C            add si,0xc
0000AE14  89361000          mov [0x10],si
0000AE18  87FE              xchg di,si
0000AE1A  8CC0              mov ax,es
0000AE1C  8CDA              mov dx,ds
0000AE1E  8EC2              mov es,dx
0000AE20  8ED8              mov ds,ax
0000AE22  A5                movsw
0000AE23  A5                movsw
0000AE24  A5                movsw
0000AE25  A5                movsw
0000AE26  8CC0              mov ax,es
0000AE28  8CDA              mov dx,ds
0000AE2A  8EC2              mov es,dx
0000AE2C  8ED8              mov ds,ax
0000AE2E  26AD              es lodsw
0000AE30  83EF08            sub di,0x8
0000AE33  87F7              xchg si,di
0000AE35  8AF4              mov dh,ah
0000AE37  80E680            and dh,0x80
0000AE3A  88740A            mov [si+0xa],dh
0000AE3D  80E47F            and ah,0x7f
0000AE40  32F6              xor dh,dh
0000AE42  3DFF7F            cmp ax,0x7fff
0000AE45  740F              jz 0xae56
0000AE47  3D0000            cmp ax,0x0
0000AE4A  7425              jz 0xae71
0000AE4C  2DFF3F            sub ax,0x3fff
0000AE4F  894408            mov [si+0x8],ax
0000AE52  88740B            mov [si+0xb],dh
0000AE55  C3                ret
0000AE56  B80040            mov ax,0x4000
0000AE59  B602              mov dh,0x2
0000AE5B  817C060080        cmp word [si+0x6],0x8000
0000AE60  75ED              jnz 0xae4f
0000AE62  8B6C04            mov bp,[si+0x4]
0000AE65  0B6C02            or bp,[si+0x2]
0000AE68  0B2C              or bp,[si]
0000AE6A  75E3              jnz 0xae4f
0000AE6C  80CE01            or dh,0x1
0000AE6F  EBDE              jmp 0xae4f
0000AE71  8B6C06            mov bp,[si+0x6]
0000AE74  0B6C04            or bp,[si+0x4]
0000AE77  0B6C02            or bp,[si+0x2]
0000AE7A  0B2C              or bp,[si]
0000AE7C  7507              jnz 0xae85
0000AE7E  B801C0            mov ax,0xc001
0000AE81  B601              mov dh,0x1
0000AE83  EBCA              jmp 0xae4f
0000AE85  830E160030        or word [0x16],0x30
0000AE8A  33ED              xor bp,bp
0000AE8C  892C              mov [si],bp
0000AE8E  896C02            mov [si+0x2],bp
0000AE91  896C04            mov [si+0x4],bp
0000AE94  896C06            mov [si+0x6],bp
0000AE97  EBE5              jmp 0xae7e
0000AE99  F6C701            test bh,0x1
0000AE9C  751F              jnz 0xaebd
0000AE9E  A5                movsw
0000AE9F  A5                movsw
0000AEA0  A5                movsw
0000AEA1  A5                movsw
0000AEA2  B8FF7F            mov ax,0x7fff
0000AEA5  0AE6              or ah,dh
0000AEA7  AB                stosw
0000AEA8  8B361000          mov si,[0x10]
0000AEAC  3B360E00          cmp si,[0xe]
0000AEB0  7503              jnz 0xaeb5
0000AEB2  E834E5            call 0x93e9
0000AEB5  83EE0C            sub si,0xc
0000AEB8  89361000          mov [0x10],si
0000AEBC  C3                ret
0000AEBD  33C0              xor ax,ax
0000AEBF  AB                stosw
0000AEC0  AB                stosw
0000AEC1  AB                stosw
0000AEC2  B80080            mov ax,0x8000
0000AEC5  AB                stosw
0000AEC6  B8FF7F            mov ax,0x7fff
0000AEC9  0AE6              or ah,dh
0000AECB  AB                stosw
0000AECC  8B361000          mov si,[0x10]
0000AED0  3B360E00          cmp si,[0xe]
0000AED4  7503              jnz 0xaed9
0000AED6  E810E5            call 0x93e9
0000AED9  83EE0C            sub si,0xc
0000AEDC  89361000          mov [0x10],si
0000AEE0  C3                ret
0000AEE1  F6C702            test bh,0x2
0000AEE4  75B3              jnz 0xae99
0000AEE6  33C0              xor ax,ax
0000AEE8  AB                stosw
0000AEE9  AB                stosw
0000AEEA  AB                stosw
0000AEEB  AB                stosw
0000AEEC  AB                stosw
0000AEED  8B361000          mov si,[0x10]
0000AEF1  3B360E00          cmp si,[0xe]
0000AEF5  7503              jnz 0xaefa
0000AEF7  E8EFE4            call 0x93e9
0000AEFA  83EE0C            sub si,0xc
0000AEFD  89361000          mov [0x10],si
0000AF01  C3                ret
0000AF02  8BFE              mov di,si
0000AF04  8B361000          mov si,[0x10]
0000AF08  8B4408            mov ax,[si+0x8]
0000AF0B  05FF3F            add ax,0x3fff
0000AF0E  8A740A            mov dh,[si+0xa]
0000AF11  80E680            and dh,0x80
0000AF14  0AE6              or ah,dh
0000AF16  8A7C0B            mov bh,[si+0xb]
0000AF19  0AFF              or bh,bh
0000AF1B  75C4              jnz 0xaee1
0000AF1D  A5                movsw
0000AF1E  A5                movsw
0000AF1F  A5                movsw
0000AF20  A5                movsw
0000AF21  AB                stosw
0000AF22  83EE08            sub si,0x8
0000AF25  3B360E00          cmp si,[0xe]
0000AF29  7503              jnz 0xaf2e
0000AF2B  E8BBE4            call 0x93e9
0000AF2E  83EE0C            sub si,0xc
0000AF31  89361000          mov [0x10],si
0000AF35  C3                ret
0000AF36  8B361000          mov si,[0x10]
0000AF3A  80640A7F          and byte [si+0xa],0x7f
0000AF3E  C3                ret
0000AF3F  8B361000          mov si,[0x10]
0000AF43  80740A80          xor byte [si+0xa],0x80
0000AF47  C3                ret
0000AF48  26AD              es lodsw
0000AF4A  A30A00            mov [0xa],ax
0000AF4D  A30600            mov [0x6],ax
0000AF50  C3                ret
0000AF51  A10600            mov ax,[0x6]
0000AF54  8BFE              mov di,si
0000AF56  AB                stosw
0000AF57  C3                ret
0000AF58  A10C00            mov ax,[0xc]
0000AF5B  8BFE              mov di,si
0000AF5D  AB                stosw
0000AF5E  C3                ret
0000AF5F  8B361000          mov si,[0x10]
0000AF63  8BFE              mov di,si
0000AF65  83EF0C            sub di,0xc
0000AF68  B10F              mov cl,0xf
0000AF6A  8A4508            mov al,[di+0x8]
0000AF6D  240F              and al,0xf
0000AF6F  2AC8              sub cl,al
0000AF71  8B4506            mov ax,[di+0x6]
0000AF74  D3E8              shr ax,cl
0000AF76  8A4D0A            mov cl,[di+0xa]
0000AF79  0AC9              or cl,cl
0000AF7B  7902              jns 0xaf7f
0000AF7D  F7D8              neg ax
0000AF7F  034408            add ax,[si+0x8]
0000AF82  700E              jo 0xaf92
0000AF84  3D0040            cmp ax,0x4000
0000AF87  7D0B              jnl 0xaf94
0000AF89  3D01C0            cmp ax,0xc001
0000AF8C  7E0F              jng 0xaf9d
0000AF8E  894408            mov [si+0x8],ax
0000AF91  C3                ret
0000AF92  7909              jns 0xaf9d
0000AF94  B80040            mov ax,0x4000
0000AF97  C6440B03          mov byte [si+0xb],0x3
0000AF9B  EBF1              jmp 0xaf8e
0000AF9D  B801C0            mov ax,0xc001
0000AFA0  C6440B01          mov byte [si+0xb],0x1
0000AFA4  EBE8              jmp 0xaf8e
0000AFA6  BA0100            mov dx,0x1
0000AFA9  EB02              jmp 0xafad
0000AFAB  33D2              xor dx,dx
0000AFAD  F70616000020      test word [0x16],0x2000
0000AFB3  7402              jz 0xafb7
0000AFB5  87F7              xchg si,di
0000AFB7  8A440A            mov al,[si+0xa]
0000AFBA  8A650A            mov ah,[di+0xa]
0000AFBD  33DB              xor bx,bx
0000AFBF  8A5C0B            mov bl,[si+0xb]
0000AFC2  D0E3              shl bl,1
0000AFC4  D0E3              shl bl,1
0000AFC6  0A5D0B            or bl,[di+0xb]
0000AFC9  D0E3              shl bl,1
0000AFCB  2EFFA7E207        jmp word near [cs:bx+0x7e2]
0000AFD0  32E0              xor ah,al
0000AFD2  7846              js 0xb01a
0000AFD4  0AC0              or al,al
0000AFD6  7904              jns 0xafdc
0000AFD8  87F7              xchg si,di
0000AFDA  86DF              xchg bl,bh
0000AFDC  8B4408            mov ax,[si+0x8]
0000AFDF  3B4508            cmp ax,[di+0x8]
0000AFE2  7C1A              jl 0xaffe
0000AFE4  7F1F              jg 0xb005
0000AFE6  B90400            mov cx,0x4
0000AFE9  83C606            add si,0x6
0000AFEC  83C706            add di,0x6
0000AFEF  FD                std
0000AFF0  F3A7              repe cmpsw
0000AFF2  FC                cld
0000AFF3  7209              jc 0xaffe
0000AFF5  770E              ja 0xb005
0000AFF7  C6060D0040        mov byte [0xd],0x40
0000AFFC  EB40              jmp 0xb03e
0000AFFE  C6060D0001        mov byte [0xd],0x1
0000B003  EB39              jmp 0xb03e
0000B005  C6060D0000        mov byte [0xd],0x0
0000B00A  EB32              jmp 0xb03e
0000B00C  C6060D0041        mov byte [0xd],0x41
0000B011  EB2B              jmp 0xb03e
0000B013  F6060B0010        test byte [0xb],0x10
0000B018  74F2              jz 0xb00c
0000B01A  2480              and al,0x80
0000B01C  78E0              js 0xaffe
0000B01E  EBE5              jmp 0xb005
0000B020  F6060B0010        test byte [0xb],0x10
0000B025  74E5              jz 0xb00c
0000B027  80E480            and ah,0x80
0000B02A  78D9              js 0xb005
0000B02C  EBD0              jmp 0xaffe
0000B02E  F6060B0010        test byte [0xb],0x10
0000B033  74D7              jz 0xb00c
0000B035  32E0              xor ah,al
0000B037  80E480            and ah,0x80
0000B03A  74BB              jz 0xaff7
0000B03C  EBDC              jmp 0xb01a
0000B03E  0BD2              or dx,dx
0000B040  7414              jz 0xb056
0000B042  8B361000          mov si,[0x10]
0000B046  3B360E00          cmp si,[0xe]
0000B04A  7503              jnz 0xb04f
0000B04C  E89AE3            call 0x93e9
0000B04F  83EE0C            sub si,0xc
0000B052  89361000          mov [0x10],si
0000B056  C3                ret
0000B057  8B361000          mov si,[0x10]
0000B05B  8B440A            mov ax,[si+0xa]
0000B05E  86C4              xchg al,ah
0000B060  D1C0              rol ax,1
0000B062  2407              and al,0x7
0000B064  BB6A08            mov bx,0x86a
0000B067  2ED7              cs xlatb
0000B069  A20D00            mov [0xd],al
0000B06C  C3                ret
0000B06D  8B361000          mov si,[0x10]
0000B071  8A440A            mov al,[si+0xa]
0000B074  2480              and al,0x80
0000B076  33DB              xor bx,bx
0000B078  8BD3              mov dx,bx
0000B07A  8A5C0B            mov bl,[si+0xb]
0000B07D  D1E3              shl bx,1
0000B07F  2EFFA70208        jmp word near [cs:bx+0x802]
0000B084  BB0E09            mov bx,0x90e
0000B087  EB1C              jmp 0xb0a5
0000B089  BB1A09            mov bx,0x91a
0000B08C  EB17              jmp 0xb0a5
0000B08E  BB2609            mov bx,0x926
0000B091  EB12              jmp 0xb0a5
0000B093  BB3209            mov bx,0x932
0000B096  EB0D              jmp 0xb0a5
0000B098  BB3E09            mov bx,0x93e
0000B09B  EB08              jmp 0xb0a5
0000B09D  BBBA08            mov bx,0x8ba
0000B0A0  EB03              jmp 0xb0a5
0000B0A2  BBC608            mov bx,0x8c6
0000B0A5  8B361000          mov si,[0x10]
0000B0A9  3B361200          cmp si,[0x12]
0000B0AD  7503              jnz 0xb0b2
0000B0AF  E827E3            call 0x93d9
0000B0B2  83C60C            add si,0xc
0000B0B5  89361000          mov [0x10],si
0000B0B9  8BF3              mov si,bx
0000B0BB  8B3E1000          mov di,[0x10]
0000B0BF  2EA5              cs movsw
0000B0C1  2EA5              cs movsw
0000B0C3  2EA5              cs movsw
0000B0C5  2EA5              cs movsw
0000B0C7  2EA5              cs movsw
0000B0C9  2EA5              cs movsw
0000B0CB  C3                ret
0000B0CC  E8F1ED            call 0x9ec0
0000B0CF  7304              jnc 0xb0d5
0000B0D1  5A                pop dx
0000B0D2  E9E1ED            jmp 0x9eb6
0000B0D5  8BF7              mov si,di
0000B0D7  83C60C            add si,0xc
0000B0DA  3B361000          cmp si,[0x10]
0000B0DE  7F08              jg 0xb0e8
0000B0E0  E8B6EF            call 0xa099
0000B0E3  83C70C            add di,0xc
0000B0E6  EBEF              jmp 0xb0d7
0000B0E8  83EF0C            sub di,0xc
0000B0EB  893E1000          mov [0x10],di
0000B0EF  C3                ret
0000B0F0  F7C1001C          test cx,0x1c00
0000B0F4  741B              jz 0xb111
0000B0F6  8CD8              mov ax,ds
0000B0F8  8EC0              mov es,ax
0000B0FA  E8C3ED            call 0x9ec0
0000B0FD  7304              jnc 0xb103
0000B0FF  5A                pop dx
0000B100  E9B3ED            jmp 0x9eb6
0000B103  B90600            mov cx,0x6
0000B106  8B04              mov ax,[si]
0000B108  8B1D              mov bx,[di]
0000B10A  AB                stosw
0000B10B  891C              mov [si],bx
0000B10D  46                inc si
0000B10E  46                inc si
0000B10F  E2F5              loop 0xb106
0000B111  C3                ret
0000B112  E8ABED            call 0x9ec0
0000B115  7304              jnc 0xb11b
0000B117  5A                pop dx
0000B118  E99BED            jmp 0x9eb6
0000B11B  8B361000          mov si,[0x10]
0000B11F  3B361200          cmp si,[0x12]
0000B123  7503              jnz 0xb128
0000B125  E8B1E2            call 0x93d9
0000B128  83C60C            add si,0xc
0000B12B  89361000          mov [0x10],si
0000B12F  8CD8              mov ax,ds
0000B131  8EC0              mov es,ax
0000B133  87F7              xchg si,di
0000B135  E861EF            call 0xa099
0000B138  C3                ret
0000B139  F7C1C001          test cx,0x1c0
0000B13D  7410              jz 0xb14f
0000B13F  E87EED            call 0x9ec0
0000B142  7304              jnc 0xb148
0000B144  5A                pop dx
0000B145  E96EED            jmp 0x9eb6
0000B148  8CDA              mov dx,ds
0000B14A  8EC2              mov es,dx
0000B14C  E84AEF            call 0xa099
0000B14F  0BC0              or ax,ax
0000B151  7414              jz 0xb167
0000B153  8B361000          mov si,[0x10]
0000B157  3B360E00          cmp si,[0xe]
0000B15B  7503              jnz 0xb160
0000B15D  E889E2            call 0x93e9
0000B160  83EE0C            sub si,0xc
0000B163  89361000          mov [0x10],si
0000B167  C3                ret
0000B168  55                push bp
0000B169  8B3E1000          mov di,[0x10]
0000B16D  893E9400          mov [0x94],di
0000B171  33ED              xor bp,bp
0000B173  C706A0000000      mov word [0xa0],0x0
0000B179  8BF7              mov si,di
0000B17B  83EE0C            sub si,0xc
0000B17E  8B4508            mov ax,[di+0x8]
0000B181  2B4408            sub ax,[si+0x8]
0000B184  A39E00            mov [0x9e],ax
0000B187  8B15              mov dx,[di]
0000B189  8B4D02            mov cx,[di+0x2]
0000B18C  8B5D04            mov bx,[di+0x4]
0000B18F  8B4506            mov ax,[di+0x6]
0000B192  7D06              jnl 0xb19a
0000B194  8B7508            mov si,[di+0x8]
0000B197  EB42              jmp 0xb1db
0000B199  90                nop
0000B19A  3B4406            cmp ax,[si+0x6]
0000B19D  7225              jc 0xb1c4
0000B19F  7712              ja 0xb1b3
0000B1A1  3B5C04            cmp bx,[si+0x4]
0000B1A4  721E              jc 0xb1c4
0000B1A6  770B              ja 0xb1b3
0000B1A8  3B4C02            cmp cx,[si+0x2]
0000B1AB  7217              jc 0xb1c4
0000B1AD  7704              ja 0xb1b3
0000B1AF  3B14              cmp dx,[si]
0000B1B1  7211              jc 0xb1c4
0000B1B3  E8BB00            call 0xb271
0000B1B6  833E9E0000        cmp word [0x9e],0x0
0000B1BB  741B              jz 0xb1d8
0000B1BD  E88700            call 0xb247
0000B1C0  75D8              jnz 0xb19a
0000B1C2  EB14              jmp 0xb1d8
0000B1C4  833E9E0000        cmp word [0x9e],0x0
0000B1C9  740D              jz 0xb1d8
0000B1CB  E88B00            call 0xb259
0000B1CE  E8A000            call 0xb271
0000B1D1  E87300            call 0xb247
0000B1D4  75C4              jnz 0xb19a
0000B1D6  EB00              jmp 0xb1d8
0000B1D8  8B7408            mov si,[si+0x8]
0000B1DB  8B7D0A            mov di,[di+0xa]
0000B1DE  97                xchg ax,di
0000B1DF  8AE0              mov ah,al
0000B1E1  50                push ax
0000B1E2  8BC5              mov ax,bp
0000B1E4  83E5FC            and bp,0xfffffffffffffffc
0000B1E7  0B2EA000          or bp,[0xa0]
0000B1EB  8A260D00          mov ah,[0xd]
0000B1EF  80E4FB            and ah,0xfb
0000B1F2  A801              test al,0x1
0000B1F4  7505              jnz 0xb1fb
0000B1F6  80E4FD            and ah,0xfd
0000B1F9  EB03              jmp 0xb1fe
0000B1FB  80CC02            or ah,0x2
0000B1FE  A802              test al,0x2
0000B200  7509              jnz 0xb20b
0000B202  0BED              or bp,bp
0000B204  7421              jz 0xb227
0000B206  80E4BF            and ah,0xbf
0000B209  EB03              jmp 0xb20e
0000B20B  80CC40            or ah,0x40
0000B20E  A804              test al,0x4
0000B210  7509              jnz 0xb21b
0000B212  0BED              or bp,bp
0000B214  7420              jz 0xb236
0000B216  80E4FE            and ah,0xfe
0000B219  EB03              jmp 0xb21e
0000B21B  80CC01            or ah,0x1
0000B21E  88260D00          mov [0xd],ah
0000B222  33ED              xor bp,bp
0000B224  E969F2            jmp 0xa490
0000B227  A00D00            mov al,[0xd]
0000B22A  A802              test al,0x2
0000B22C  7505              jnz 0xb233
0000B22E  80E4BF            and ah,0xbf
0000B231  EB03              jmp 0xb236
0000B233  80CC40            or ah,0x40
0000B236  A00D00            mov al,[0xd]
0000B239  A840              test al,0x40
0000B23B  7505              jnz 0xb242
0000B23D  80E4FE            and ah,0xfe
0000B240  EBDC              jmp 0xb21e
0000B242  80CC01            or ah,0x1
0000B245  EBD7              jmp 0xb21e
0000B247  F6C480            test ah,0x80
0000B24A  750C              jnz 0xb258
0000B24C  833E9E0000        cmp word [0x9e],0x0
0000B251  7405              jz 0xb258
0000B253  E80300            call 0xb259
0000B256  EBEF              jmp 0xb247
0000B258  C3                ret
0000B259  D1E2              shl dx,1
0000B25B  D1D1              rcl cx,1
0000B25D  D1D3              rcl bx,1
0000B25F  D1D0              rcl ax,1
0000B261  FF0E9E00          dec word [0x9e]
0000B265  D1E5              shl bp,1
0000B267  7201              jc 0xb26a
0000B269  C3                ret
0000B26A  C706A0000100      mov word [0xa0],0x1
0000B270  C3                ret
0000B271  2B14              sub dx,[si]
0000B273  1B4C02            sbb cx,[si+0x2]
0000B276  1B5C04            sbb bx,[si+0x4]
0000B279  1B4406            sbb ax,[si+0x6]
0000B27C  45                inc bp
0000B27D  C3                ret
0000B27E  8B361000          mov si,[0x10]
0000B282  E82C00            call 0xb2b1
0000B285  C3                ret
0000B286  D0DC              rcr ah,1
0000B288  7320              jnc 0xb2aa
0000B28A  D0D0              rcl al,1
0000B28C  7212              jc 0xb2a0
0000B28E  A00B00            mov al,[0xb]
0000B291  A810              test al,0x10
0000B293  751B              jnz 0xb2b0
0000B295  EB09              jmp 0xb2a0
0000B297  F6C402            test ah,0x2
0000B29A  75EA              jnz 0xb286
0000B29C  D0DC              rcr ah,1
0000B29E  7210              jc 0xb2b0
0000B2A0  8BFE              mov di,si
0000B2A2  BE8A08            mov si,0x88a
0000B2A5  E8D6ED            call 0xa07e
0000B2A8  8BF7              mov si,di
0000B2AA  810E16008100      or word [0x16],0x81
0000B2B0  C3                ret
0000B2B1  8B440A            mov ax,[si+0xa]
0000B2B4  A98003            test ax,0x380
0000B2B7  75DE              jnz 0xb297
0000B2B9  56                push si
0000B2BA  BF4A00            mov di,0x4a
0000B2BD  C7450A0000        mov word [di+0xa],0x0
0000B2C2  8B4408            mov ax,[si+0x8]
0000B2C5  48                dec ax
0000B2C6  8B5C06            mov bx,[si+0x6]
0000B2C9  8B4C04            mov cx,[si+0x4]
0000B2CC  8B5402            mov dx,[si+0x2]
0000B2CF  A801              test al,0x1
0000B2D1  7407              jz 0xb2da
0000B2D3  40                inc ax
0000B2D4  D1EB              shr bx,1
0000B2D6  D1D9              rcr cx,1
0000B2D8  D1DA              rcr dx,1
0000B2DA  D1F8              sar ax,1
0000B2DC  894508            mov [di+0x8],ax
0000B2DF  83FBFE            cmp bx,0xfffffffffffffffe
0000B2E2  7203              jc 0xb2e7
0000B2E4  F9                stc
0000B2E5  EB39              jmp 0xb320
0000B2E7  52                push dx
0000B2E8  B875B0            mov ax,0xb075
0000B2EB  F7E3              mul bx
0000B2ED  BDD857            mov bp,0x57d8
0000B2F0  03EA              add bp,dx
0000B2F2  7303              jnc 0xb2f7
0000B2F4  BDFFFF            mov bp,0xffff
0000B2F7  8BD3              mov dx,bx
0000B2F9  33C0              xor ax,ax
0000B2FB  F7F5              div bp
0000B2FD  03E8              add bp,ax
0000B2FF  D1DD              rcr bp,1
0000B301  8BD3              mov dx,bx
0000B303  8BC1              mov ax,cx
0000B305  F7F5              div bp
0000B307  F9                stc
0000B308  13E8              adc bp,ax
0000B30A  D1DD              rcr bp,1
0000B30C  8BD3              mov dx,bx
0000B30E  8BC1              mov ax,cx
0000B310  F7F5              div bp
0000B312  8BF0              mov si,ax
0000B314  58                pop ax
0000B315  F7F5              div bp
0000B317  8BDD              mov bx,bp
0000B319  8BC8              mov cx,ax
0000B31B  83C101            add cx,0x1
0000B31E  13DE              adc bx,si
0000B320  D1DB              rcr bx,1
0000B322  D1D9              rcr cx,1
0000B324  895D06            mov [di+0x6],bx
0000B327  894D04            mov [di+0x4],cx
0000B32A  C745020000        mov word [di+0x2],0x0
0000B32F  C7050000          mov word [di],0x0
0000B333  8BF7              mov si,di
0000B335  5F                pop di
0000B336  893E9400          mov [0x94],di
0000B33A  E8C9EC            call 0xa006
0000B33D  BE4A00            mov si,0x4a
0000B340  E8D3EC            call 0xa016
0000B343  FF4D08            dec word [di+0x8]
0000B346  8BF7              mov si,di
0000B348  C3                ret
0000B349  57                push di
0000B34A  BF8800            mov di,0x88
0000B34D  2EA5              cs movsw
0000B34F  2EA5              cs movsw
0000B351  2EA5              cs movsw
0000B353  2EA5              cs movsw
0000B355  2EA5              cs movsw
0000B357  2EA5              cs movsw
0000B359  BE8800            mov si,0x88
0000B35C  5F                pop di
0000B35D  C3                ret
0000B35E  3AED              cmp ch,ch
0000B360  EB56              jmp 0xb3b8
0000B362  8CC9              mov cx,cs
0000B364  8ED9              mov ds,cx
0000B366  8B4C0A            mov cx,[si+0xa]
0000B369  80E180            and cl,0x80
0000B36C  268B550A          mov dx,[es:di+0xa]
0000B370  80E280            and dl,0x80
0000B373  3AD1              cmp dl,cl
0000B375  7545              jnz 0xb3bc
0000B377  06                push es
0000B378  56                push si
0000B379  57                push di
0000B37A  0AC9              or cl,cl
0000B37C  7908              jns 0xb386
0000B37E  1E                push ds
0000B37F  06                push es
0000B380  1F                pop ds
0000B381  07                pop es
0000B382  87F7              xchg si,di
0000B384  87CA              xchg cx,dx
0000B386  80E501            and ch,0x1
0000B389  80E601            and dh,0x1
0000B38C  3AF5              cmp dh,ch
0000B38E  7528              jnz 0xb3b8
0000B390  0AED              or ch,ch
0000B392  77CA              ja 0xb35e
0000B394  8B4C08            mov cx,[si+0x8]
0000B397  81C1FF3F          add cx,0x3fff
0000B39B  268B5508          mov dx,[es:di+0x8]
0000B39F  81C2FF3F          add dx,0x3fff
0000B3A3  3BCA              cmp cx,dx
0000B3A5  7511              jnz 0xb3b8
0000B3A7  83C606            add si,0x6
0000B3AA  83C706            add di,0x6
0000B3AD  FD                std
0000B3AE  A7                cmpsw
0000B3AF  7507              jnz 0xb3b8
0000B3B1  A7                cmpsw
0000B3B2  7504              jnz 0xb3b8
0000B3B4  A7                cmpsw
0000B3B5  7501              jnz 0xb3b8
0000B3B7  A7                cmpsw
0000B3B8  FC                cld
0000B3B9  5F                pop di
0000B3BA  5E                pop si
0000B3BB  07                pop es
0000B3BC  8CC1              mov cx,es
0000B3BE  8ED9              mov ds,cx
0000B3C0  C3                ret
0000B3C1  BF7C00            mov di,0x7c
0000B3C4  E8D2EC            call 0xa099
0000B3C7  57                push di
0000B3C8  56                push si
0000B3C9  53                push bx
0000B3CA  BF7000            mov di,0x70
0000B3CD  E8C9EC            call 0xa099
0000B3D0  893E9400          mov [0x94],di
0000B3D4  E828EC            call 0x9fff
0000B3D7  5E                pop si
0000B3D8  2EAD              cs lodsw
0000B3DA  91                xchg ax,cx
0000B3DB  5F                pop di
0000B3DC  E89FEC            call 0xa07e
0000B3DF  893E9400          mov [0x94],di
0000B3E3  51                push cx
0000B3E4  56                push si
0000B3E5  BE7000            mov si,0x70
0000B3E8  E814EC            call 0x9fff
0000B3EB  5E                pop si
0000B3EC  83C60C            add si,0xc
0000B3EF  56                push si
0000B3F0  E856FF            call 0xb349
0000B3F3  E820EC            call 0xa016
0000B3F6  5E                pop si
0000B3F7  59                pop cx
0000B3F8  E2E9              loop 0xb3e3
0000B3FA  8BDE              mov bx,si
0000B3FC  5E                pop si
0000B3FD  56                push si
0000B3FE  53                push bx
0000B3FF  E8FDEB            call 0x9fff
0000B402  5E                pop si
0000B403  83C60C            add si,0xc
0000B406  2EAD              cs lodsw
0000B408  91                xchg ax,cx
0000B409  5B                pop bx
0000B40A  57                push di
0000B40B  8BFB              mov di,bx
0000B40D  E86EEC            call 0xa07e
0000B410  51                push cx
0000B411  56                push si
0000B412  BE7000            mov si,0x70
0000B415  893E9400          mov [0x94],di
0000B419  E8FAEB            call 0xa016
0000B41C  5E                pop si
0000B41D  59                pop cx
0000B41E  51                push cx
0000B41F  83C60C            add si,0xc
0000B422  56                push si
0000B423  BE7000            mov si,0x70
0000B426  E8D6EB            call 0x9fff
0000B429  5E                pop si
0000B42A  56                push si
0000B42B  E81BFF            call 0xb349
0000B42E  E8E5EB            call 0xa016
0000B431  5E                pop si
0000B432  59                pop cx
0000B433  E2E9              loop 0xb41e
0000B435  8BF7              mov si,di
0000B437  5F                pop di
0000B438  C3                ret
0000B439  8B361000          mov si,[0x10]
0000B43D  E81E00            call 0xb45e
0000B440  56                push si
0000B441  8B361000          mov si,[0x10]
0000B445  3B361200          cmp si,[0x12]
0000B449  7503              jnz 0xb44e
0000B44B  E88BDF            call 0x93d9
0000B44E  83C60C            add si,0xc
0000B451  89361000          mov [0x10],si
0000B455  8B3E1000          mov di,[0x10]
0000B459  5E                pop si
0000B45A  E83CEC            call 0xa099
0000B45D  C3                ret
0000B45E  BB4A09            mov bx,0x94a
0000B461  E85DFF            call 0xb3c1
0000B464  C3                ret
0000B465  8B3E1000          mov di,[0x10]
0000B469  8BF7              mov si,di
0000B46B  8B440A            mov ax,[si+0xa]
0000B46E  83EF0C            sub di,0xc
0000B471  E81900            call 0xb48d
0000B474  8B361000          mov si,[0x10]
0000B478  8B361000          mov si,[0x10]
0000B47C  3B360E00          cmp si,[0xe]
0000B480  7503              jnz 0xb485
0000B482  E864DF            call 0x93e9
0000B485  83EE0C            sub si,0xc
0000B488  89361000          mov [0x10],si
0000B48C  C3                ret
0000B48D  893E9400          mov [0x94],di
0000B491  E872EB            call 0xa006
0000B494  B000              mov al,0x0
0000B496  BED208            mov si,0x8d2
0000B499  E8C6FE            call 0xb362
0000B49C  7336              jnc 0xb4d4
0000B49E  8BF7              mov si,di
0000B4A0  BF4A00            mov di,0x4a
0000B4A3  E8F3EB            call 0xa099
0000B4A6  56                push si
0000B4A7  BEDE08            mov si,0x8de
0000B4AA  E89CFE            call 0xb349
0000B4AD  893E9400          mov [0x94],di
0000B4B1  E84BEB            call 0x9fff
0000B4B4  BEC608            mov si,0x8c6
0000B4B7  E88FFE            call 0xb349
0000B4BA  E83DEB            call 0x9ffa
0000B4BD  5E                pop si
0000B4BE  57                push di
0000B4BF  8BFE              mov di,si
0000B4C1  BEDE08            mov si,0x8de
0000B4C4  E882FE            call 0xb349
0000B4C7  893E9400          mov [0x94],di
0000B4CB  E848EB            call 0xa016
0000B4CE  5E                pop si
0000B4CF  E83DEB            call 0xa00f
0000B4D2  B001              mov al,0x1
0000B4D4  50                push ax
0000B4D5  8BF7              mov si,di
0000B4D7  BBAE09            mov bx,0x9ae
0000B4DA  E8E4FE            call 0xb3c1
0000B4DD  893E9400          mov [0x94],di
0000B4E1  E822EB            call 0xa006
0000B4E4  58                pop ax
0000B4E5  0AC0              or al,al
0000B4E7  7409              jz 0xb4f2
0000B4E9  BEEA08            mov si,0x8ea
0000B4EC  E85AFE            call 0xb349
0000B4EF  E824EB            call 0xa016
0000B4F2  C3                ret
0000B4F3  8B361000          mov si,[0x10]
0000B4F7  E80100            call 0xb4fb
0000B4FA  C3                ret
0000B4FB  BB1E0A            mov bx,0xa1e
0000B4FE  E8C0FE            call 0xb3c1
0000B501  57                push di
0000B502  87F7              xchg si,di
0000B504  893E9400          mov [0x94],di
0000B508  E8EFEA            call 0x9ffa
0000B50B  8BF7              mov si,di
0000B50D  5F                pop di
0000B50E  893E9400          mov [0x94],di
0000B512  E8F1EA            call 0xa006
0000B515  FF4508            inc word [di+0x8]
0000B518  8BF7              mov si,di
0000B51A  C3                ret
0000B51B  8B3E1000          mov di,[0x10]
0000B51F  8BF7              mov si,di
0000B521  8B440A            mov ax,[si+0xa]
0000B524  83EF0C            sub di,0xc
0000B527  E81900            call 0xb543
0000B52A  8B361000          mov si,[0x10]
0000B52E  8B361000          mov si,[0x10]
0000B532  3B360E00          cmp si,[0xe]
0000B536  7503              jnz 0xb53b
0000B538  E8AEDE            call 0x93e9
0000B53B  83EE0C            sub si,0xc
0000B53E  89361000          mov [0x10],si
0000B542  C3                ret
0000B543  56                push si
0000B544  8BF7              mov si,di
0000B546  BF5800            mov di,0x58
0000B549  E84DEB            call 0xa099
0000B54C  8BFE              mov di,si
0000B54E  5E                pop si
0000B54F  57                push di
0000B550  BF6400            mov di,0x64
0000B553  E843EB            call 0xa099
0000B556  8B5D08            mov bx,[di+0x8]
0000B559  C745080000        mov word [di+0x8],0x0
0000B55E  BEF608            mov si,0x8f6
0000B561  E8FEFD            call 0xb362
0000B564  7704              ja 0xb56a
0000B566  FF4D08            dec word [di+0x8]
0000B569  43                inc bx
0000B56A  53                push bx
0000B56B  BEC608            mov si,0x8c6
0000B56E  E8D8FD            call 0xb349
0000B571  893E9400          mov [0x94],di
0000B575  E882EA            call 0x9ffa
0000B578  8BF7              mov si,di
0000B57A  5B                pop bx
0000B57B  5F                pop di
0000B57C  53                push bx
0000B57D  E86900            call 0xb5e9
0000B580  5B                pop bx
0000B581  33C0              xor ax,ax
0000B583  0BDB              or bx,bx
0000B585  7439              jz 0xb5c0
0000B587  8BD0              mov dx,ax
0000B589  7904              jns 0xb58f
0000B58B  B280              mov dl,0x80
0000B58D  F7DB              neg bx
0000B58F  B91000            mov cx,0x10
0000B592  49                dec cx
0000B593  D1E3              shl bx,1
0000B595  73FB              jnc 0xb592
0000B597  57                push di
0000B598  D1DB              rcr bx,1
0000B59A  BF6400            mov di,0x64
0000B59D  AB                stosw
0000B59E  AB                stosw
0000B59F  AB                stosw
0000B5A0  8BC3              mov ax,bx
0000B5A2  AB                stosw
0000B5A3  8BC1              mov ax,cx
0000B5A5  AB                stosw
0000B5A6  8BC2              mov ax,dx
0000B5A8  AB                stosw
0000B5A9  BF5800            mov di,0x58
0000B5AC  BE6400            mov si,0x64
0000B5AF  893E9400          mov [0x94],di
0000B5B3  E849EA            call 0x9fff
0000B5B6  8BF7              mov si,di
0000B5B8  5F                pop di
0000B5B9  893E9400          mov [0x94],di
0000B5BD  E856EA            call 0xa016
0000B5C0  C3                ret
0000B5C1  8B3E1000          mov di,[0x10]
0000B5C5  8BF7              mov si,di
0000B5C7  8B440A            mov ax,[si+0xa]
0000B5CA  83EF0C            sub di,0xc
0000B5CD  E81900            call 0xb5e9
0000B5D0  8B361000          mov si,[0x10]
0000B5D4  8B361000          mov si,[0x10]
0000B5D8  3B360E00          cmp si,[0xe]
0000B5DC  7503              jnz 0xb5e1
0000B5DE  E808DE            call 0x93e9
0000B5E1  83EE0C            sub si,0xc
0000B5E4  89361000          mov [0x10],si
0000B5E8  C3                ret
0000B5E9  57                push di
0000B5EA  BF4A00            mov di,0x4a
0000B5ED  E8A9EA            call 0xa099
0000B5F0  56                push si
0000B5F1  BE0209            mov si,0x902
0000B5F4  E852FD            call 0xb349
0000B5F7  893E9400          mov [0x94],di
0000B5FB  E818EA            call 0xa016
0000B5FE  5E                pop si
0000B5FF  E80DEA            call 0xa00f
0000B602  FF4508            inc word [di+0x8]
0000B605  8BF7              mov si,di
0000B607  BB6A0A            mov bx,0xa6a
0000B60A  E8B4FD            call 0xb3c1
0000B60D  893E9400          mov [0x94],di
0000B611  E8F2E9            call 0xa006
0000B614  8BF7              mov si,di
0000B616  5F                pop di
0000B617  893E9400          mov [0x94],di
0000B61B  E8E1E9            call 0x9fff
0000B61E  C3                ret
0000B61F  0000              add [bx+si],al
0000B621  0000              add [bx+si],al
0000B623  0000              add [bx+si],al
0000B625  0000              add [bx+si],al
0000B627  0000              add [bx+si],al
0000B629  001E0E1F          add [0x1f0e],bl
0000B62D  B80235            mov ax,0x3502
0000B630  CD21              int byte 0x21
0000B632  891EB223          mov [0x23b2],bx
0000B636  8C06B423          mov word [0x23b4],es
0000B63A  BA1224            mov dx,0x2412
0000B63D  B80225            mov ax,0x2502
0000B640  CD21              int byte 0x21
0000B642  B82335            mov ax,0x3523
0000B645  CD21              int byte 0x21
0000B647  891EB623          mov [0x23b6],bx
0000B64B  8C06B823          mov word [0x23b8],es
0000B64F  BAFA23            mov dx,0x23fa
0000B652  B82325            mov ax,0x2523
0000B655  CD21              int byte 0x21
0000B657  1F                pop ds
0000B658  C3                ret
0000B659  1E                push ds
0000B65A  50                push ax
0000B65B  52                push dx
0000B65C  B80225            mov ax,0x2502
0000B65F  2EC516B223        lds dx,word [cs:0x23b2]
0000B664  CD21              int byte 0x21
0000B666  5A                pop dx
0000B667  58                pop ax
0000B668  1F                pop ds
0000B669  C3                ret
0000B66A  50                push ax
0000B66B  52                push dx
0000B66C  1E                push ds
0000B66D  E8E9FF            call 0xb659
0000B670  2EC516B623        lds dx,word [cs:0x23b6]
0000B675  B82325            mov ax,0x2523
0000B678  CD21              int byte 0x21
0000B67A  1F                pop ds
0000B67B  5A                pop dx
0000B67C  58                pop ax
0000B67D  2EFF2EB623        jmp word far [cs:0x23b6]
0000B682  90                nop
0000B683  2EDD3EB023        fnstsw [cs:0x23b0]
0000B688  51                push cx
0000B689  B90300            mov cx,0x3
0000B68C  E2FE              loop 0xb68c
0000B68E  59                pop cx
0000B68F  2EF606B02380      test byte [cs:0x23b0],0x80
0000B695  7404              jz 0xb69b
0000B697  E855DD            call 0x93ef
0000B69A  CF                iret
0000B69B  2EFF2EB223        jmp word far [cs:0x23b2]
0000B6A0  3C3C              cmp al,0x3c
0000B6A2  46                inc si
0000B6A3  4D                dec bp
0000B6A4  53                push bx
0000B6A5  47                inc di
0000B6A6  3E3E0210          add dl,[ds:bx+si]
0000B6AA  53                push bx
0000B6AB  796E              jns 0xb71b
0000B6AD  7461              jz 0xb710
0000B6AF  7820              js 0xb6d1
0000B6B1  657272            gs jc 0xb726
0000B6B4  6F                outsw
0000B6B5  7200              jc 0xb6b7
0000B6B7  0310              add dx,[bx+si]
0000B6B9  52                push dx
0000B6BA  45                inc bp
0000B6BB  54                push sp
0000B6BC  55                push bp
0000B6BD  52                push dx
0000B6BE  4E                dec si
0000B6BF  207769            and [bx+0x69],dh
0000B6C2  7468              jz 0xb72c
0000B6C4  6F                outsw
0000B6C5  7574              jnz 0xb73b
0000B6C7  20474F            and [bx+0x4f],al
0000B6CA  53                push bx
0000B6CB  55                push bp
0000B6CC  42                inc dx
0000B6CD  0004              add [si],al
0000B6CF  104F75            adc [bx+0x75],cl
0000B6D2  7420              jz 0xb6f4
0000B6D4  6F                outsw
0000B6D5  66204441          o32 and [si+0x41],al
0000B6D9  54                push sp
0000B6DA  41                inc cx
0000B6DB  0005              add [di],al
0000B6DD  10496C            adc [bx+di+0x6c],cl
0000B6E0  6C                insb
0000B6E1  656761            gs a32 popa
0000B6E4  6C                insb
0000B6E5  206675            and [bp+0x75],ah
0000B6E8  6E                outsb
0000B6E9  637469            arpl [si+0x69],si
0000B6EC  6F                outsw
0000B6ED  6E                outsb
0000B6EE  206361            and [bp+di+0x61],ah
0000B6F1  6C                insb
0000B6F2  6C                insb
0000B6F3  0006104F          add [0x4f10],al
0000B6F7  7665              jna 0xb75e
0000B6F9  7266              jc 0xb761
0000B6FB  6C                insb
0000B6FC  6F                outsw
0000B6FD  7700              ja 0xb6ff
0000B6FF  07                pop es
0000B700  104F75            adc [bx+0x75],cl
0000B703  7420              jz 0xb725
0000B705  6F                outsw
0000B706  66206D65          o32 and [di+0x65],ch
0000B70A  6D                insw
0000B70B  6F                outsw
0000B70C  7279              jc 0xb787
0000B70E  0009              add [bx+di],cl
0000B710  105375            adc [bp+di+0x75],dl
0000B713  627363            bound si,[bp+di+0x63]
0000B716  7269              jc 0xb781
0000B718  7074              jo 0xb78e
0000B71A  206F75            and [bx+0x75],ch
0000B71D  7420              jz 0xb73f
0000B71F  6F                outsw
0000B720  66207261          o32 and [bp+si+0x61],dh
0000B724  6E                outsb
0000B725  6765000A          add [gs:edx],cl
0000B729  104475            adc [si+0x75],al
0000B72C  706C              jo 0xb79a
0000B72E  6963617465        imul sp,[bp+di+0x61],0x6574
0000B733  206465            and [si+0x65],ah
0000B736  66696E6974696F6E  imul ebp,[bp+0x69],0x6e6f6974
0000B73E  000B              add [bp+di],cl
0000B740  104469            adc [si+0x69],al
0000B743  7669              jna 0xb7ae
0000B745  7369              jnc 0xb7b0
0000B747  6F                outsw
0000B748  6E                outsb
0000B749  206279            and [bp+si+0x79],ah
0000B74C  207A65            and [bp+si+0x65],bh
0000B74F  726F              jc 0xb7c0
0000B751  000D              add [di],cl
0000B753  105479            adc [si+0x79],dl
0000B756  7065              jo 0xb7bd
0000B758  206D69            and [di+0x69],ch
0000B75B  736D              jnc 0xb7ca
0000B75D  61                popa
0000B75E  7463              jz 0xb7c3
0000B760  68000E            push word 0xe00
0000B763  104F75            adc [bx+0x75],cl
0000B766  7420              jz 0xb788
0000B768  6F                outsw
0000B769  66207374          o32 and [bp+di+0x74],dh
0000B76D  7269              jc 0xb7d8
0000B76F  6E                outsb
0000B770  67207370          and [ebx+0x70],dh
0000B774  61                popa
0000B775  636500            arpl [di+0x0],sp
0000B778  1010              adc [bx+si],dl
0000B77A  53                push bx
0000B77B  7472              jz 0xb7ef
0000B77D  696E672066        imul bp,[bp+0x67],0x6620
0000B782  6F                outsw
0000B783  726D              jc 0xb7f2
0000B785  756C              jnz 0xb7f3
0000B787  61                popa
0000B788  20746F            and [si+0x6f],dh
0000B78B  6F                outsw
0000B78C  20636F            and [bp+di+0x6f],ah
0000B78F  6D                insw
0000B790  706C              jo 0xb7fe
0000B792  657800            gs js 0xb795
0000B795  1310              adc dx,[bx+si]
0000B797  4E                dec si
0000B798  6F                outsw
0000B799  205245            and [bp+si+0x45],dl
0000B79C  53                push bx
0000B79D  55                push bp
0000B79E  4D                dec bp
0000B79F  45                inc bp
0000B7A0  0014              add [si],dl
0000B7A2  105245            adc [bp+si+0x45],dl
0000B7A5  53                push bx
0000B7A6  55                push bp
0000B7A7  4D                dec bp
0000B7A8  45                inc bp
0000B7A9  207769            and [bx+0x69],dh
0000B7AC  7468              jz 0xb816
0000B7AE  6F                outsw
0000B7AF  7574              jnz 0xb825
0000B7B1  206572            and [di+0x72],ah
0000B7B4  726F              jc 0xb825
0000B7B6  7200              jc 0xb7b8
0000B7B8  1810              sbb [bx+si],dl
0000B7BA  44                inc sp
0000B7BB  657669            gs jna 0xb827
0000B7BE  636520            arpl [di+0x20],sp
0000B7C1  7469              jz 0xb82c
0000B7C3  6D                insw
0000B7C4  656F              gs outsw
0000B7C6  7574              jnz 0xb83c
0000B7C8  0019              add [bx+di],bl
0000B7CA  104465            adc [si+0x65],al
0000B7CD  7669              jna 0xb838
0000B7CF  636520            arpl [di+0x20],sp
0000B7D2  6661              popad
0000B7D4  756C              jnz 0xb842
0000B7D6  7400              jz 0xb7d8
0000B7D8  1B10              sbb dx,[bx+si]
0000B7DA  4F                dec di
0000B7DB  7574              jnz 0xb851
0000B7DD  206F66            and [bx+0x66],ch
0000B7E0  207061            and [bx+si+0x61],dh
0000B7E3  7065              jo 0xb84a
0000B7E5  7200              jc 0xb7e7
0000B7E7  27                daa
0000B7E8  104341            adc [bp+di+0x41],al
0000B7EB  53                push bx
0000B7EC  45                inc bp
0000B7ED  20454C            and [di+0x4c],al
0000B7F0  53                push bx
0000B7F1  45                inc bp
0000B7F2  206578            and [di+0x78],ah
0000B7F5  7065              jo 0xb85c
0000B7F7  637465            arpl [si+0x65],si
0000B7FA  640028            add [fs:bx+si],ch
0000B7FD  105661            adc [bp+0x61],dl
0000B800  7269              jc 0xb86b
0000B802  61                popa
0000B803  626C65            bound bp,[si+0x65]
0000B806  207265            and [bp+si+0x65],dh
0000B809  7175              jno 0xb880
0000B80B  6972656400        imul si,[bp+si+0x65],0x64
0000B810  3210              xor dl,[bx+si]
0000B812  46                inc si
0000B813  49                dec cx
0000B814  45                inc bp
0000B815  4C                dec sp
0000B816  44                inc sp
0000B817  206F76            and [bx+0x76],ch
0000B81A  657266            gs jc 0xb883
0000B81D  6C                insb
0000B81E  6F                outsw
0000B81F  7700              ja 0xb821
0000B821  3310              xor dx,[bx+si]
0000B823  49                dec cx
0000B824  6E                outsb
0000B825  7465              jz 0xb88c
0000B827  726E              jc 0xb897
0000B829  61                popa
0000B82A  6C                insb
0000B82B  206572            and [di+0x72],ah
0000B82E  726F              jc 0xb89f
0000B830  7200              jc 0xb832
0000B832  3410              xor al,0x10
0000B834  42                inc dx
0000B835  61                popa
0000B836  64206669          and [fs:bp+0x69],ah
0000B83A  6C                insb
0000B83B  65206E61          and [gs:bp+0x61],ch
0000B83F  6D                insw
0000B840  65206F72          and [gs:bx+0x72],ch
0000B844  206E75            and [bp+0x75],ch
0000B847  6D                insw
0000B848  626572            bound sp,[di+0x72]
0000B84B  0035              add [di],dh
0000B84D  104669            adc [bp+0x69],al
0000B850  6C                insb
0000B851  65206E6F          and [gs:bp+0x6f],ch
0000B855  7420              jz 0xb877
0000B857  666F              outsd
0000B859  756E              jnz 0xb8c9
0000B85B  6400361042        add [fs:0x4210],dh
0000B860  61                popa
0000B861  64206669          and [fs:bp+0x69],ah
0000B865  6C                insb
0000B866  65206D6F          and [gs:di+0x6f],ch
0000B86A  64650037          add [gs:bx],dh
0000B86E  104669            adc [bp+0x69],al
0000B871  6C                insb
0000B872  6520616C          and [gs:bx+di+0x6c],ah
0000B876  7265              jc 0xb8dd
0000B878  61                popa
0000B879  647920            fs jns 0xb89c
0000B87C  6F                outsw
0000B87D  7065              jo 0xb8e4
0000B87F  6E                outsb
0000B880  0038              add [bx+si],bh
0000B882  104649            adc [bp+0x49],al
0000B885  45                inc bp
0000B886  4C                dec sp
0000B887  44                inc sp
0000B888  207374            and [bp+di+0x74],dh
0000B88B  61                popa
0000B88C  7465              jz 0xb8f3
0000B88E  6D                insw
0000B88F  656E              gs outsb
0000B891  7420              jz 0xb8b3
0000B893  61                popa
0000B894  637469            arpl [si+0x69],si
0000B897  7665              jna 0xb8fe
0000B899  0039              add [bx+di],bh
0000B89B  104465            adc [si+0x65],al
0000B89E  7669              jna 0xb909
0000B8A0  636520            arpl [di+0x20],sp
0000B8A3  49                dec cx
0000B8A4  2F                das
0000B8A5  4F                dec di
0000B8A6  206572            and [di+0x72],ah
0000B8A9  726F              jc 0xb91a
0000B8AB  7200              jc 0xb8ad
0000B8AD  3A10              cmp dl,[bx+si]
0000B8AF  46                inc si
0000B8B0  696C652061        imul bp,[si+0x65],0x6120
0000B8B5  6C                insb
0000B8B6  7265              jc 0xb91d
0000B8B8  61                popa
0000B8B9  647920            fs jns 0xb8dc
0000B8BC  657869            gs js 0xb928
0000B8BF  7374              jnc 0xb935
0000B8C1  7300              jnc 0xb8c3
0000B8C3  3B10              cmp dx,[bx+si]
0000B8C5  42                inc dx
0000B8C6  61                popa
0000B8C7  64207265          and [fs:bp+si+0x65],dh
0000B8CB  636F72            arpl [bx+0x72],bp
0000B8CE  64206C65          and [fs:si+0x65],ch
0000B8D2  6E                outsb
0000B8D3  677468            a32 jz 0xb93e
0000B8D6  003D              add [di],bh
0000B8D8  104469            adc [si+0x69],al
0000B8DB  736B              jnc 0xb948
0000B8DD  206675            and [bp+0x75],ah
0000B8E0  6C                insb
0000B8E1  6C                insb
0000B8E2  003E1049          add [0x4910],bh
0000B8E6  6E                outsb
0000B8E7  7075              jo 0xb95e
0000B8E9  7420              jz 0xb90b
0000B8EB  7061              jo 0xb94e
0000B8ED  7374              jnc 0xb963
0000B8EF  20656E            and [di+0x6e],ah
0000B8F2  64206F66          and [fs:bx+0x66],ch
0000B8F6  206669            and [bp+0x69],ah
0000B8F9  6C                insb
0000B8FA  65003F            add [gs:bx],bh
0000B8FD  104261            adc [bp+si+0x61],al
0000B900  64207265          and [fs:bp+si+0x65],dh
0000B904  636F72            arpl [bx+0x72],bp
0000B907  64206E75          and [fs:bp+0x75],ch
0000B90B  6D                insw
0000B90C  626572            bound sp,[di+0x72]
0000B90F  004010            add [bx+si+0x10],al
0000B912  42                inc dx
0000B913  61                popa
0000B914  64206669          and [fs:bp+0x69],ah
0000B918  6C                insb
0000B919  65206E61          and [gs:bp+0x61],ch
0000B91D  6D                insw
0000B91E  65004310          add [gs:bp+di+0x10],al
0000B922  54                push sp
0000B923  6F                outsw
0000B924  6F                outsw
0000B925  206D61            and [di+0x61],ch
0000B928  6E                outsb
0000B929  7920              jns 0xb94b
0000B92B  66696C6573004410  imul ebp,[si+0x65],0x10440073
0000B933  44                inc sp
0000B934  657669            gs jna 0xb9a0
0000B937  636520            arpl [di+0x20],sp
0000B93A  756E              jnz 0xb9aa
0000B93C  61                popa
0000B93D  7661              jna 0xb9a0
0000B93F  696C61626C        imul bp,[si+0x61],0x6c62
0000B944  65004510          add [gs:di+0x10],al
0000B948  43                inc bx
0000B949  6F                outsw
0000B94A  6D                insw
0000B94B  6D                insw
0000B94C  756E              jnz 0xb9bc
0000B94E  6963617469        imul sp,[bp+di+0x61],0x6974
0000B953  6F                outsw
0000B954  6E                outsb
0000B955  2D6275            sub ax,0x7562
0000B958  6666657220        gs jc 0xb97d
0000B95D  6F                outsw
0000B95E  7665              jna 0xb9c5
0000B960  7266              jc 0xb9c8
0000B962  6C                insb
0000B963  6F                outsw
0000B964  7700              ja 0xb966
0000B966  46                inc si
0000B967  105065            adc [bx+si+0x65],dl
0000B96A  726D              jc 0xb9d9
0000B96C  697373696F        imul si,[bp+di+0x73],0x6f69
0000B971  6E                outsb
0000B972  206465            and [si+0x65],ah
0000B975  6E                outsb
0000B976  6965640047        imul sp,[di+0x64],0x4700
0000B97B  104469            adc [si+0x69],al
0000B97E  736B              jnc 0xb9eb
0000B980  206E6F            and [bp+0x6f],ch
0000B983  7420              jz 0xb9a5
0000B985  7265              jc 0xb9ec
0000B987  61                popa
0000B988  647900            fs jns 0xb98b
0000B98B  48                dec ax
0000B98C  104469            adc [si+0x69],al
0000B98F  736B              jnc 0xb9fc
0000B991  2D6D65            sub ax,0x656d
0000B994  646961206572      imul sp,[fs:bx+di+0x20],0x7265
0000B99A  726F              jc 0xba0b
0000B99C  7200              jc 0xb99e
0000B99E  49                dec cx
0000B99F  104164            adc [bx+di+0x64],al
0000B9A2  7661              jna 0xba05
0000B9A4  6E                outsb
0000B9A5  636564            arpl [di+0x64],sp
0000B9A8  206665            and [bp+0x65],ah
0000B9AB  61                popa
0000B9AC  7475              jz 0xba23
0000B9AE  7265              jc 0xba15
0000B9B0  20756E            and [di+0x6e],dh
0000B9B3  61                popa
0000B9B4  7661              jna 0xba17
0000B9B6  696C61626C        imul bp,[si+0x61],0x6c62
0000B9BB  65004A10          add [gs:bp+si+0x10],cl
0000B9BF  52                push dx
0000B9C0  656E              gs outsb
0000B9C2  61                popa
0000B9C3  6D                insw
0000B9C4  65206163          and [gs:bx+di+0x63],ah
0000B9C8  726F              jc 0xba39
0000B9CA  7373              jnc 0xba3f
0000B9CC  206469            and [si+0x69],ah
0000B9CF  736B              jnc 0xba3c
0000B9D1  7300              jnc 0xb9d3
0000B9D3  4B                dec bx
0000B9D4  105061            adc [bx+si+0x61],dl
0000B9D7  7468              jz 0xba41
0000B9D9  2F                das
0000B9DA  46                inc si
0000B9DB  696C652061        imul bp,[si+0x65],0x6120
0000B9E0  636365            arpl [bp+di+0x65],sp
0000B9E3  7373              jnc 0xba58
0000B9E5  206572            and [di+0x72],ah
0000B9E8  726F              jc 0xba59
0000B9EA  7200              jc 0xb9ec
0000B9EC  4C                dec sp
0000B9ED  105061            adc [bx+si+0x61],dl
0000B9F0  7468              jz 0xba5a
0000B9F2  206E6F            and [bp+0x6f],ch
0000B9F5  7420              jz 0xba17
0000B9F7  666F              outsd
0000B9F9  756E              jnz 0xba69
0000B9FB  6400FF            fs add bh,bh
0000B9FE  10556E            adc [di+0x6e],dl
0000BA01  7072              jo 0xba75
0000BA03  696E746162        imul bp,[bp+0x74],0x6261
0000BA08  6C                insb
0000BA09  65206572          and [gs:di+0x72],ah
0000BA0D  726F              jc 0xba7e
0000BA0F  7200              jc 0xba11
0000BA11  00805265          add [bx+si+0x6552],al
0000BA15  646F              fs outsw
0000BA17  206672            and [bp+0x72],ah
0000BA1A  6F                outsw
0000BA1B  6D                insw
0000BA1C  207374            and [bp+di+0x74],dh
0000BA1F  61                popa
0000BA20  7274              jc 0xba96
0000BA22  0001              add [bx+di],al
0000BA24  80427974          add byte [bp+si+0x79],0x74
0000BA28  657320            gs jnc 0xba4b
0000BA2B  667265            jc 0xba93
0000BA2E  650D0006          gs or ax,0x600
0000BA32  8052616E          adc byte [bp+si+0x61],0x6e
0000BA36  646F              fs outsw
0000BA38  6D                insw
0000BA39  2D6E75            sub ax,0x756e
0000BA3C  6D                insw
0000BA3D  626572            bound sp,[di+0x72]
0000BA40  207365            and [bp+di+0x65],dh
0000BA43  65642028          and [fs:bx+si],ch
0000BA47  2D3332            sub ax,0x3233
0000BA4A  37                aaa
0000BA4B  363820            cmp [ss:bx+si],ah
0000BA4E  746F              jz 0xbabf
0000BA50  2033              and [bp+di],dh
0000BA52  3237              xor dh,[bx]
0000BA54  3637              ss aaa
0000BA56  2900              sub [bx+si],ax
0000BA58  07                pop es
0000BA59  80486974          or byte [bx+si+0x69],0x74
0000BA5D  20616E            and [bx+di+0x6e],ah
0000BA60  7920              jns 0xba82
0000BA62  6B657920          imul sp,[di+0x79],0x20
0000BA66  746F              jz 0xbad7
0000BA68  207265            and [bp+si+0x65],dh
0000BA6B  7475              jz 0xbae2
0000BA6D  726E              jc 0xbadd
0000BA6F  20746F            and [si+0x6f],dh
0000BA72  207379            and [bp+di+0x79],dh
0000BA75  7374              jnc 0xbaeb
0000BA77  656D              gs insw
0000BA79  0008              add [bx+si],cl
0000BA7B  802069            and byte [bx+si],0x69
0000BA7E  6E                outsb
0000BA7F  2000              and [bx+si],al
0000BA81  0980206F          or [bx+si+0x6f20],ax
0000BA85  662000            o32 and [bx+si],al
0000BA88  0A806C69          or al,[bx+si+0x696c]
0000BA8C  6E                outsb
0000BA8D  652000            and [gs:bx+si],al
0000BA90  0B806D6F          or ax,[bx+si+0x6f6d]
0000BA94  64756C            fs jnz 0xbb03
0000BA97  652000            and [gs:bx+si],al
0000BA9A  0C80              or al,0x80
0000BA9C  206174            and [bx+di+0x74],ah
0000BA9F  206164            and [bx+di+0x64],ah
0000BAA2  647265            fs jc 0xbb0a
0000BAA5  7373              jnc 0xbb1a
0000BAA7  2000              and [bx+si],al
0000BAA9  0D802A            or ax,0x2a80
0000BAAC  42                inc dx
0000BAAD  7265              jc 0xbb14
0000BAAF  61                popa
0000BAB0  6B2A00            imul bp,[bp+si],0x0
0000BAB3  00905374          add [bx+si+0x7453],dl
0000BAB7  7269              jc 0xbb22
0000BAB9  6E                outsb
0000BABA  67207370          and [ebx+0x70],dh
0000BABE  61                popa
0000BABF  636520            arpl [di+0x20],sp
0000BAC2  636F72            arpl [bx+0x72],bp
0000BAC5  7275              jc 0xbb3c
0000BAC7  7074              jo 0xbb3d
0000BAC9  0005              add [di],al
0000BACB  90                nop
0000BACC  45                inc bp
0000BACD  7272              jc 0xbb41
0000BACF  6F                outsw
0000BAD0  7220              jc 0xbaf2
0000BAD2  696E204558        imul bp,[bp+0x20],0x5845
0000BAD7  45                inc bp
0000BAD8  206669            and [bp+0x69],ah
0000BADB  6C                insb
0000BADC  6500069053        add [gs:0x5390],al
0000BAE1  54                push sp
0000BAE2  4F                dec di
0000BAE3  50                push ax
0000BAE4  0007              add [bx],al
0000BAE6  90                nop
0000BAE7  4F                dec di
0000BAE8  7574              jnz 0xbb5e
0000BAEA  206F66            and [bx+0x66],ch
0000BAED  207374            and [bp+di+0x74],dh
0000BAF0  61                popa
0000BAF1  636B20            arpl [bp+di+0x20],bp
0000BAF4  7370              jnc 0xbb66
0000BAF6  61                popa
0000BAF7  636500            arpl [di+0x0],sp
0000BAFA  00944E6F          add [si+0x6f4e],dl
0000BAFE  206C69            and [si+0x69],ch
0000BB01  6E                outsb
0000BB02  65206E75          and [gs:bp+0x75],ch
0000BB06  6D                insw
0000BB07  626572            bound sp,[di+0x72]
0000BB0A  20696E            and [bx+di+0x6e],ch
0000BB0D  2000              and [bx+si],al
0000BB0F  00985265          add [bx+si+0x6552],bl
0000BB13  7175              jno 0xbb8a
0000BB15  6972657320        imul si,[bp+si+0x65],0x2073
0000BB1A  44                inc sp
0000BB1B  4F                dec di
0000BB1C  53                push bx
0000BB1D  2032              and [bp+si],dh
0000BB1F  2E3130            xor [cs:bx+si],si
0000BB22  206F72            and [bx+0x72],ch
0000BB25  206C61            and [si+0x61],ch
0000BB28  7465              jz 0xbb8f
0000BB2A  7200              jc 0xbb2c
0000BB2C  01984572          add [bx+si+0x7245],bx
0000BB30  726F              jc 0xbba1
0000BB32  7220              jc 0xbb54
0000BB34  647572            fs jnz 0xbba9
0000BB37  696E672072        imul bp,[bp+0x67],0x7220
0000BB3C  756E              jnz 0xbbac
0000BB3E  2D7469            sub ax,0x6974
0000BB41  6D                insw
0000BB42  6520696E          and [gs:bx+di+0x6e],ch
0000BB46  697469616C        imul si,[si+0x69],0x6c61
0000BB4B  697A617469        imul di,[bp+si+0x61],0x6974
0000BB50  6F                outsw
0000BB51  6E                outsb
0000BB52  0002              add [bp+si],al
0000BB54  98                cbw
0000BB55  4F                dec di
0000BB56  7574              jnz 0xbbcc
0000BB58  206F66            and [bx+0x66],ch
0000BB5B  206D65            and [di+0x65],ch
0000BB5E  6D                insw
0000BB5F  6F                outsw
0000BB60  7279              jc 0xbbdb
0000BB62  0003              add [bp+di],al
0000BB64  98                cbw
0000BB65  44                inc sp
0000BB66  4F                dec di
0000BB67  53                push bx
0000BB68  206D65            and [di+0x65],ch
0000BB6B  6D                insw
0000BB6C  6F                outsw
0000BB6D  7279              jc 0xbbe8
0000BB6F  2D6172            sub ax,0x7261
0000BB72  656E              gs outsb
0000BB74  61                popa
0000BB75  206572            and [di+0x72],ah
0000BB78  726F              jc 0xbbe9
0000BB7A  7200              jc 0xbb7c
0000BB7C  0498              add al,0x98
0000BB7E  46                inc si
0000BB7F  61                popa
0000BB80  7220              jc 0xbba2
0000BB82  686561            push word 0x6165
0000BB85  7020              jo 0xbba7
0000BB87  636F72            arpl [bx+0x72],bp
0000BB8A  7275              jc 0xbc01
0000BB8C  7074              jo 0xbc02
0000BB8E  0000              add [bx+si],al
0000BB90  9A52363030        call word 0x3030:word 0x3652
0000BB95  300D              xor [di],cl
0000BB97  0A2D              or ch,[di]
0000BB99  207374            and [bp+di+0x74],dh
0000BB9C  61                popa
0000BB9D  636B20            arpl [bp+di+0x20],bp
0000BBA0  6F                outsw
0000BBA1  7665              jna 0xbc08
0000BBA3  7266              jc 0xbc0b
0000BBA5  6C                insb
0000BBA6  6F                outsw
0000BBA7  770D              ja 0xbbb6
0000BBA9  0A00              or al,[bx+si]
0000BBAB  029A5236          add bl,[bp+si+0x3652]
0000BBAF  3030              xor [bx+si],dh
0000BBB1  320D              xor cl,[di]
0000BBB3  0A2D              or ch,[di]
0000BBB5  20666C            and [bp+0x6c],ah
0000BBB8  6F                outsw
0000BBB9  61                popa
0000BBBA  7469              jz 0xbc25
0000BBBC  6E                outsb
0000BBBD  6720706F          and [eax+0x6f],dh
0000BBC1  696E74206E        imul bp,[bp+0x74],0x6e20
0000BBC6  6F                outsw
0000BBC7  7420              jz 0xbbe9
0000BBC9  6C                insb
0000BBCA  6F                outsw
0000BBCB  61                popa
0000BBCC  6465640D0A00      fs or ax,0xa
0000BBD2  039A5236          add bx,[bp+si+0x3652]
0000BBD6  3030              xor [bx+si],dh
0000BBD8  330D              xor cx,[di]
0000BBDA  0A2D              or ch,[di]
0000BBDC  20696E            and [bx+di+0x6e],ch
0000BBDF  7465              jz 0xbc46
0000BBE1  67657220          gs a32 jc 0xbc05
0000BBE5  646976696465      imul si,[fs:bp+0x69],0x6564
0000BBEB  206279            and [bp+si+0x79],ah
0000BBEE  2030              and [bx+si],dh
0000BBF0  0D0A00            or ax,0xa
0000BBF3  089A5236          or [bp+si+0x3652],bl
0000BBF7  3030              xor [bx+si],dh
0000BBF9  380D              cmp [di],cl
0000BBFB  0A2D              or ch,[di]
0000BBFD  206E6F            and [bp+0x6f],ch
0000BC00  7420              jz 0xbc22
0000BC02  656E              gs outsb
0000BC04  6F                outsw
0000BC05  7567              jnz 0xbc6e
0000BC07  682073            push word 0x7320
0000BC0A  7061              jo 0xbc6d
0000BC0C  636520            arpl [di+0x20],sp
0000BC0F  666F              outsd
0000BC11  7220              jc 0xbc33
0000BC13  61                popa
0000BC14  7267              jc 0xbc7d
0000BC16  756D              jnz 0xbc85
0000BC18  656E              gs outsb
0000BC1A  7473              jz 0xbc8f
0000BC1C  0D0A00            or ax,0xa
0000BC1F  099A5236          or [bp+si+0x3652],bx
0000BC23  3030              xor [bx+si],dh
0000BC25  390D              cmp [di],cx
0000BC27  0A2D              or ch,[di]
0000BC29  206E6F            and [bp+0x6f],ch
0000BC2C  7420              jz 0xbc4e
0000BC2E  656E              gs outsb
0000BC30  6F                outsw
0000BC31  7567              jnz 0xbc9a
0000BC33  682073            push word 0x7320
0000BC36  7061              jo 0xbc99
0000BC38  636520            arpl [di+0x20],sp
0000BC3B  666F              outsd
0000BC3D  7220              jc 0xbc5f
0000BC3F  656E              gs outsb
0000BC41  7669              jna 0xbcac
0000BC43  726F              jc 0xbcb4
0000BC45  6E                outsb
0000BC46  6D                insw
0000BC47  656E              gs outsb
0000BC49  740D              jz 0xbc58
0000BC4B  0A00              or al,[bx+si]
0000BC4D  0C9A              or al,0x9a
0000BC4F  52                push dx
0000BC50  363031            xor [ss:bx+di],dh
0000BC53  320D              xor cl,[di]
0000BC55  0A2D              or ch,[di]
0000BC57  20696C            and [bx+di+0x6c],ch
0000BC5A  6C                insb
0000BC5B  656761            gs a32 popa
0000BC5E  6C                insb
0000BC5F  206E65            and [bp+0x65],ch
0000BC62  61                popa
0000BC63  7220              jc 0xbc85
0000BC65  706F              jo 0xbcd6
0000BC67  696E746572        imul bp,[bp+0x74],0x7265
0000BC6C  207573            and [di+0x73],dh
0000BC6F  650D0A00          gs or ax,0xa
0000BC73  0D9A52            or ax,0x529a
0000BC76  363031            xor [ss:bx+di],dh
0000BC79  330D              xor cx,[di]
0000BC7B  0A2D              or ch,[di]
0000BC7D  20696C            and [bx+di+0x6c],ch
0000BC80  6C                insb
0000BC81  656761            gs a32 popa
0000BC84  6C                insb
0000BC85  206661            and [bp+0x61],ah
0000BC88  7220              jc 0xbcaa
0000BC8A  706F              jo 0xbcfb
0000BC8C  696E746572        imul bp,[bp+0x74],0x7265
0000BC91  207573            and [di+0x73],dh
0000BC94  650D0A00          gs or ax,0xa
0000BC98  0E                push cs
0000BC99  9A52363031        call word 0x3130:word 0x3652
0000BC9E  340D              xor al,0xd
0000BCA0  0A2D              or ch,[di]
0000BCA2  20636F            and [bp+di+0x6f],ah
0000BCA5  6E                outsb
0000BCA6  7472              jz 0xbd1a
0000BCA8  6F                outsw
0000BCA9  6C                insb
0000BCAA  2D4252            sub ax,0x5242
0000BCAD  45                inc bp
0000BCAE  41                inc cx
0000BCAF  4B                dec bx
0000BCB0  20656E            and [di+0x6e],ah
0000BCB3  636F75            arpl [bx+0x75],bp
0000BCB6  6E                outsb
0000BCB7  7465              jz 0xbd1e
0000BCB9  7265              jc 0xbd20
0000BCBB  640D0A00          fs or ax,0xa
0000BCBF  0F                db 0x0f
0000BCC0  9A52363031        call word 0x3130:word 0x3652
0000BCC5  350D0A            xor ax,0xa0d
0000BCC8  2D2075            sub ax,0x7520
0000BCCB  6E                outsb
0000BCCC  657870            gs js 0xbd3f
0000BCCF  65637465          arpl [gs:si+0x65],si
0000BCD3  6420696E          and [fs:bx+di+0x6e],ch
0000BCD7  7465              jz 0xbd3e
0000BCD9  7272              jc 0xbd4d
0000BCDB  7570              jnz 0xbd4d
0000BCDD  740D              jz 0xbcec
0000BCDF  0A00              or al,[bx+si]
0000BCE1  FF9A0D0A          call word far [bp+si+0xa0d]
0000BCE5  7275              jc 0xbd5c
0000BCE7  6E                outsb
0000BCE8  2D7469            sub ax,0x6974
0000BCEB  6D                insw
0000BCEC  65206572          and [gs:di+0x72],ah
0000BCF0  726F              jc 0xbd61
0000BCF2  7220              jc 0xbd14
0000BCF4  00FF              add bh,bh
0000BCF6  FFFF              udw
0000BCF8  0000              add [bx+si],al
0000BCFA  0000              add [bx+si],al
0000BCFC  0000              add [bx+si],al
0000BCFE  0000              add [bx+si],al
0000BD00  0000              add [bx+si],al
0000BD02  0000              add [bx+si],al
0000BD04  0000              add [bx+si],al
0000BD06  0000              add [bx+si],al
0000BD08  0000              add [bx+si],al
0000BD0A  0000              add [bx+si],al
0000BD0C  0000              add [bx+si],al
0000BD0E  A20000            mov [0x0],al
0000BD11  004A01            add [bp+si+0x1],cl
0000BD14  0000              add [bx+si],al
0000BD16  0000              add [bx+si],al
0000BD18  0000              add [bx+si],al
0000BD1A  0000              add [bx+si],al
0000BD1C  0000              add [bx+si],al
0000BD1E  0000              add [bx+si],al
0000BD20  0000              add [bx+si],al
0000BD22  0000              add [bx+si],al
0000BD24  0000              add [bx+si],al
0000BD26  0000              add [bx+si],al
0000BD28  0000              add [bx+si],al
0000BD2A  0000              add [bx+si],al
0000BD2C  0000              add [bx+si],al
0000BD2E  0000              add [bx+si],al
0000BD30  0000              add [bx+si],al
0000BD32  0000              add [bx+si],al
0000BD34  0000              add [bx+si],al
0000BD36  0000              add [bx+si],al
0000BD38  0000              add [bx+si],al
0000BD3A  0000              add [bx+si],al
0000BD3C  0000              add [bx+si],al
0000BD3E  0000              add [bx+si],al
0000BD40  0000              add [bx+si],al
0000BD42  0000              add [bx+si],al
0000BD44  0000              add [bx+si],al
0000BD46  0000              add [bx+si],al
0000BD48  0000              add [bx+si],al
0000BD4A  0000              add [bx+si],al
0000BD4C  0000              add [bx+si],al
0000BD4E  0000              add [bx+si],al
0000BD50  0000              add [bx+si],al
0000BD52  0000              add [bx+si],al
0000BD54  0000              add [bx+si],al
0000BD56  0000              add [bx+si],al
0000BD58  0000              add [bx+si],al
0000BD5A  0000              add [bx+si],al
0000BD5C  0000              add [bx+si],al
0000BD5E  0000              add [bx+si],al
0000BD60  0000              add [bx+si],al
0000BD62  0000              add [bx+si],al
0000BD64  0000              add [bx+si],al
0000BD66  0000              add [bx+si],al
0000BD68  0000              add [bx+si],al
0000BD6A  0000              add [bx+si],al
0000BD6C  0000              add [bx+si],al
0000BD6E  0000              add [bx+si],al
0000BD70  0000              add [bx+si],al
0000BD72  0000              add [bx+si],al
0000BD74  0000              add [bx+si],al
0000BD76  0000              add [bx+si],al
0000BD78  0000              add [bx+si],al
0000BD7A  0000              add [bx+si],al
0000BD7C  0000              add [bx+si],al
0000BD7E  0000              add [bx+si],al
0000BD80  0000              add [bx+si],al
0000BD82  0000              add [bx+si],al
0000BD84  0000              add [bx+si],al
0000BD86  0000              add [bx+si],al
0000BD88  0000              add [bx+si],al
0000BD8A  0000              add [bx+si],al
0000BD8C  0000              add [bx+si],al
0000BD8E  0000              add [bx+si],al
0000BD90  0000              add [bx+si],al
0000BD92  0000              add [bx+si],al
0000BD94  0000              add [bx+si],al
0000BD96  0000              add [bx+si],al
0000BD98  0000              add [bx+si],al
0000BD9A  0000              add [bx+si],al
0000BD9C  0000              add [bx+si],al
0000BD9E  0000              add [bx+si],al
0000BDA0  0000              add [bx+si],al
0000BDA2  0000              add [bx+si],al
0000BDA4  0000              add [bx+si],al
0000BDA6  0000              add [bx+si],al
0000BDA8  0000              add [bx+si],al
0000BDAA  0000              add [bx+si],al
0000BDAC  0000              add [bx+si],al
0000BDAE  0000              add [bx+si],al
0000BDB0  0000              add [bx+si],al
0000BDB2  0000              add [bx+si],al
0000BDB4  0000              add [bx+si],al
0000BDB6  0000              add [bx+si],al
0000BDB8  0000              add [bx+si],al
0000BDBA  0000              add [bx+si],al
0000BDBC  0000              add [bx+si],al
0000BDBE  0000              add [bx+si],al
0000BDC0  0000              add [bx+si],al
0000BDC2  0000              add [bx+si],al
0000BDC4  0000              add [bx+si],al
0000BDC6  0000              add [bx+si],al
0000BDC8  0000              add [bx+si],al
0000BDCA  0000              add [bx+si],al
0000BDCC  0000              add [bx+si],al
0000BDCE  0000              add [bx+si],al
0000BDD0  0000              add [bx+si],al
0000BDD2  0000              add [bx+si],al
0000BDD4  0000              add [bx+si],al
0000BDD6  0000              add [bx+si],al
0000BDD8  0000              add [bx+si],al
0000BDDA  0000              add [bx+si],al
0000BDDC  0000              add [bx+si],al
0000BDDE  0000              add [bx+si],al
0000BDE0  0000              add [bx+si],al
0000BDE2  0000              add [bx+si],al
0000BDE4  0000              add [bx+si],al
0000BDE6  0000              add [bx+si],al
0000BDE8  0000              add [bx+si],al
0000BDEA  0000              add [bx+si],al
0000BDEC  0000              add [bx+si],al
0000BDEE  0000              add [bx+si],al
0000BDF0  0000              add [bx+si],al
0000BDF2  0000              add [bx+si],al
0000BDF4  0000              add [bx+si],al
0000BDF6  0000              add [bx+si],al
0000BDF8  0000              add [bx+si],al
0000BDFA  0000              add [bx+si],al
0000BDFC  0000              add [bx+si],al
0000BDFE  0000              add [bx+si],al
0000BE00  0000              add [bx+si],al
0000BE02  0000              add [bx+si],al
0000BE04  0000              add [bx+si],al
0000BE06  0000              add [bx+si],al
0000BE08  0000              add [bx+si],al
0000BE0A  0000              add [bx+si],al
0000BE0C  0000              add [bx+si],al
0000BE0E  0000              add [bx+si],al
0000BE10  0000              add [bx+si],al
0000BE12  0000              add [bx+si],al
0000BE14  0000              add [bx+si],al
0000BE16  0000              add [bx+si],al
0000BE18  0000              add [bx+si],al
0000BE1A  0000              add [bx+si],al
0000BE1C  0000              add [bx+si],al
0000BE1E  0000              add [bx+si],al
0000BE20  0000              add [bx+si],al
0000BE22  0000              add [bx+si],al
0000BE24  0000              add [bx+si],al
0000BE26  0000              add [bx+si],al
0000BE28  0000              add [bx+si],al
0000BE2A  0000              add [bx+si],al
0000BE2C  0000              add [bx+si],al
0000BE2E  0000              add [bx+si],al
0000BE30  0000              add [bx+si],al
0000BE32  0000              add [bx+si],al
0000BE34  0000              add [bx+si],al
0000BE36  0000              add [bx+si],al
0000BE38  0000              add [bx+si],al
0000BE3A  0000              add [bx+si],al
0000BE3C  0000              add [bx+si],al
0000BE3E  0000              add [bx+si],al
0000BE40  0000              add [bx+si],al
0000BE42  0000              add [bx+si],al
0000BE44  0000              add [bx+si],al
0000BE46  0000              add [bx+si],al
0000BE48  0000              add [bx+si],al
0000BE4A  0000              add [bx+si],al
0000BE4C  0000              add [bx+si],al
0000BE4E  0000              add [bx+si],al
0000BE50  0000              add [bx+si],al
0000BE52  0000              add [bx+si],al
0000BE54  0000              add [bx+si],al
0000BE56  0000              add [bx+si],al
0000BE58  0000              add [bx+si],al
0000BE5A  0000              add [bx+si],al
0000BE5C  0000              add [bx+si],al
0000BE5E  0000              add [bx+si],al
0000BE60  0000              add [bx+si],al
0000BE62  0000              add [bx+si],al
0000BE64  0000              add [bx+si],al
0000BE66  0000              add [bx+si],al
0000BE68  0000              add [bx+si],al
0000BE6A  0000              add [bx+si],al
0000BE6C  0000              add [bx+si],al
0000BE6E  0000              add [bx+si],al
0000BE70  0000              add [bx+si],al
0000BE72  0000              add [bx+si],al
0000BE74  0000              add [bx+si],al
0000BE76  0000              add [bx+si],al
0000BE78  0000              add [bx+si],al
0000BE7A  0000              add [bx+si],al
0000BE7C  0000              add [bx+si],al
0000BE7E  0000              add [bx+si],al
0000BE80  0000              add [bx+si],al
0000BE82  0000              add [bx+si],al
0000BE84  0000              add [bx+si],al
0000BE86  0000              add [bx+si],al
0000BE88  0000              add [bx+si],al
0000BE8A  0000              add [bx+si],al
0000BE8C  0000              add [bx+si],al
0000BE8E  0000              add [bx+si],al
0000BE90  0000              add [bx+si],al
0000BE92  0000              add [bx+si],al
0000BE94  0000              add [bx+si],al
0000BE96  0000              add [bx+si],al
0000BE98  0000              add [bx+si],al
0000BE9A  0000              add [bx+si],al
0000BE9C  0000              add [bx+si],al
0000BE9E  0000              add [bx+si],al
0000BEA0  0000              add [bx+si],al
0000BEA2  0000              add [bx+si],al
0000BEA4  0000              add [bx+si],al
0000BEA6  0000              add [bx+si],al
0000BEA8  0000              add [bx+si],al
0000BEAA  0000              add [bx+si],al
0000BEAC  0000              add [bx+si],al
0000BEAE  0000              add [bx+si],al
0000BEB0  0000              add [bx+si],al
0000BEB2  0000              add [bx+si],al
0000BEB4  0000              add [bx+si],al
0000BEB6  0000              add [bx+si],al
0000BEB8  0000              add [bx+si],al
0000BEBA  0000              add [bx+si],al
0000BEBC  0000              add [bx+si],al
0000BEBE  0000              add [bx+si],al
0000BEC0  0000              add [bx+si],al
0000BEC2  0000              add [bx+si],al
0000BEC4  0000              add [bx+si],al
0000BEC6  0000              add [bx+si],al
0000BEC8  0000              add [bx+si],al
0000BECA  0000              add [bx+si],al
0000BECC  0000              add [bx+si],al
0000BECE  0000              add [bx+si],al
0000BED0  2400              and al,0x0
0000BED2  64005355          add [fs:bp+di+0x55],dl
0000BED6  50                push ax
0000BED7  52                push dx
0000BED8  45                inc bp
0000BED9  4D                dec bp
0000BEDA  41                inc cx
0000BEDB  43                inc bx
0000BEDC  59                pop cx
0000BEDD  205361            and [bp+di+0x61],dl
0000BEE0  7665              jna 0xbf47
0000BEE2  6761              a32 popa
0000BEE4  6D                insw
0000BEE5  65204564          and [gs:di+0x64],al
0000BEE9  69746F722F        imul si,[si+0x6f],0x2f72
0000BEEE  43                inc bx
0000BEEF  686561            push word 0x6165
0000BEF2  7420              jz 0xbf14
0000BEF4  7632              jna 0xbf28
0000BEF6  2E303F            xor [cs:bx],bh
0000BEF9  008C0043          add [si+0x4300],cl
0000BEFD  68616E            push word 0x6e61
0000BF00  67657320          gs a32 jnc 0xbf24
0000BF04  7468              jz 0xbf6e
0000BF06  6520616D          and [gs:bx+di+0x6d],ah
0000BF0A  6F                outsw
0000BF0B  756E              jnz 0xbf7b
0000BF0D  7420              jz 0xbf2f
0000BF0F  6F                outsw
0000BF10  66204D69          o32 and [di+0x69],cl
0000BF14  6E                outsb
0000BF15  657261            gs jc 0xbf79
0000BF18  6C                insb
0000BF19  732C              jnc 0xbf47
0000BF1B  204675            and [bp+0x75],al
0000BF1E  656C              gs insb
0000BF20  2C20              sub al,0x20
0000BF22  45                inc bp
0000BF23  6E                outsb
0000BF24  657267            gs jc 0xbf8e
0000BF27  792C              jns 0xbf55
0000BF29  20466F            and [bp+0x6f],al
0000BF2C  6F                outsw
0000BF2D  6420262050        and [fs:0x5020],ah
0000BF32  6F                outsw
0000BF33  7075              jo 0xbfaa
0000BF35  6C                insb
0000BF36  61                popa
0000BF37  7469              jz 0xbfa2
0000BF39  6F                outsw
0000BF3A  6E                outsb
0000BF3B  DC31              fdiv qword [bx+di]
0000BF3D  00D0              add al,dl
0000BF3F  004E65            add [bp+0x65],cl
0000BF42  7720              ja 0xbf64
0000BF44  696E207632        imul bp,[bp+0x20],0x3276
0000BF49  2E303A            xor [cs:bp+si],bh
0000BF4C  204D61            and [di+0x61],cl
0000BF4F  6E                outsb
0000BF50  2C20              sub al,0x20
0000BF52  54                push sp
0000BF53  7261              jc 0xbfb6
0000BF55  696E202620        imul bp,[bp+0x20],0x2026
0000BF5A  45                inc bp
0000BF5B  7175              jno 0xbfd2
0000BF5D  697020616C        imul si,[bx+si+0x20],0x6c61
0000BF62  6C                insb
0000BF63  20796F            and [bx+di+0x6f],bh
0000BF66  7572              jnz 0xbfda
0000BF68  20506C            and [bx+si+0x6c],dl
0000BF6B  61                popa
0000BF6C  746F              jz 0xbfdd
0000BF6E  6F                outsw
0000BF6F  6E                outsb
0000BF70  7337              jnc 0xbfa9
0000BF72  1900              sbb [bx+si],ax
0000BF74  06                push es
0000BF75  015468            add [si+0x68],dx
0000BF78  65207265          and [gs:bp+si+0x65],dh
0000BF7C  61                popa
0000BF7D  736F              jnc 0xbfee
0000BF7F  6E                outsb
0000BF80  20746F            and [si+0x6f],dh
0000BF83  207365            and [bp+di+0x65],dh
0000BF86  6C                insb
0000BF87  65637420          arpl [gs:si+0x20],si
0000BF8B  7468              jz 0xbff5
0000BF8D  65207A05          and [gs:bp+si+0x5],bh
0000BF91  0024              add [si],ah
0000BF93  014C65            add [si+0x65],cx
0000BF96  7665              jna 0xbffd
0000BF98  6C                insb
0000BF99  AF                scasw
0000BF9A  27                daa
0000BF9B  002E0120          add [0x2001],ch
0000BF9F  6F                outsw
0000BFA0  7220              jc 0xbfc2
0000BFA2  6F                outsw
0000BFA3  7070              jo 0xc015
0000BFA5  6F                outsw
0000BFA6  6E                outsb
0000BFA7  656E              gs outsb
0000BFA9  7420              jz 0xbfcb
0000BFAB  6973207468        imul si,[bp+di+0x20],0x6874
0000BFB0  61                popa
0000BFB1  7420              jz 0xbfd3
0000BFB3  7468              jz 0xc01d
0000BFB5  65206279          and [gs:bp+si+0x79],ah
0000BFB9  7465              jz 0xc020
0000BFBB  206C6F            and [si+0x6f],ch
0000BFBE  636174            arpl [bx+di+0x74],sp
0000BFC1  696F6E73BC        imul bp,[bx+0x6e],0xbc73
0000BFC6  49                dec cx
0000BFC7  005A01            add [bp+si+0x1],bl
0000BFCA  636861            arpl [bx+si+0x61],bp
0000BFCD  6E                outsb
0000BFCE  67657320          gs a32 jnc 0xbff2
0000BFD2  64657065          gs jo 0xc03b
0000BFD6  6E                outsb
0000BFD7  64656E            gs outsb
0000BFDA  7420              jz 0xbffc
0000BFDC  6F                outsw
0000BFDD  6E                outsb
0000BFDE  207468            and [si+0x68],dh
0000BFE1  65206E75          and [gs:bp+0x75],ch
0000BFE5  6D                insw
0000BFE6  626572            bound sp,[di+0x72]
0000BFE9  206F66            and [bx+0x66],ch
0000BFEC  20706C            and [bx+si+0x6c],dh
0000BFEF  61                popa
0000BFF0  6E                outsb
0000BFF1  657473            gs jz 0xc067
0000BFF4  20696E            and [bx+di+0x6e],ch
0000BFF7  207468            and [si+0x68],dh
0000BFFA  6520756E          and [gs:di+0x6e],dh
0000BFFE  6976657273        imul si,[bp+0x65],0x7372
0000C003  652C20            gs sub al,0x20
0000C006  7768              ja 0xc070
0000C008  6963682063        imul sp,[bp+di+0x68],0x6320
0000C00D  68616E            push word 0x6e61
0000C010  676573A5          gs a32 jnc 0xbfb9
0000C014  17                pop ss
0000C015  00A80164          add [bx+si+0x6401],ch
0000C019  657065            gs jo 0xc081
0000C01C  6E                outsb
0000C01D  64696E67206F      imul bp,[fs:bp+0x67],0x6f20
0000C023  6E                outsb
0000C024  207468            and [si+0x68],dh
0000C027  65206C65          and [gs:si+0x65],ch
0000C02B  7665              jna 0xc092
0000C02D  6C                insb
0000C02E  2E3C27            cs cmp al,0x27
0000C031  00C4              add ah,al
0000C033  014C65            add [si+0x65],cx
0000C036  7665              jna 0xc09d
0000C038  6C                insb
0000C039  204F6E            and [bx+0x6e],cl
0000C03C  65202D            and [gs:di],ch
0000C03F  2020              and [bx+si],ah
0000C041  204F70            and [bx+0x70],cl
0000C044  706F              jo 0xc0b5
0000C046  6E                outsb
0000C047  656E              gs outsb
0000C049  743A              jz 0xc085
0000C04B  20576F            and [bx+0x6f],dl
0000C04E  746F              jz 0xc0bf
0000C050  6B2028            imul sp,[bx+si],0x28
0000C053  54                push sp
0000C054  686520            push word 0x2065
0000C057  41                inc cx
0000C058  7065              jo 0xc0bf
0000C05A  296F28            sub [bx+0x28],bp
0000C05D  00F0              add al,dh
0000C05F  014C65            add [si+0x65],cx
0000C062  7665              jna 0xc0c9
0000C064  6C                insb
0000C065  205477            and [si+0x77],dl
0000C068  6F                outsw
0000C069  202D              and [di],ch
0000C06B  2020              and [bx+si],ah
0000C06D  204F70            and [bx+0x70],cl
0000C070  706F              jo 0xc0e1
0000C072  6E                outsb
0000C073  656E              gs outsb
0000C075  743A              jz 0xc0b1
0000C077  20536D            and [bp+di+0x6d],dl
0000C07A  696E652028        imul bp,[bp+0x65],0x2820
0000C07F  41                inc cx
0000C080  6E                outsb
0000C081  742D              jz 0xc0b0
0000C083  6C                insb
0000C084  696B65292E        imul bp,[bp+di+0x65],0x2e29
0000C089  001C              add [si],bl
0000C08B  024C65            add cl,[si+0x65]
0000C08E  7665              jna 0xc0f5
0000C090  6C                insb
0000C091  205468            and [si+0x68],dl
0000C094  7265              jc 0xc0fb
0000C096  65202D            and [gs:di],ch
0000C099  204F70            and [bx+0x70],cl
0000C09C  706F              jo 0xc10d
0000C09E  6E                outsb
0000C09F  656E              gs outsb
0000C0A1  743A              jz 0xc0dd
0000C0A3  204B72            and [bp+di+0x72],cl
0000C0A6  61                popa
0000C0A7  7274              jc 0xc11d
0000C0A9  2028              and [bx+si],ch
0000C0AB  41                inc cx
0000C0AC  6C                insb
0000C0AD  6C                insb
0000C0AE  696761746F        imul sp,[bx+0x61],0x6f74
0000C0B3  722D              jc 0xc0e2
0000C0B5  6C                insb
0000C0B6  696B65292A        imul bp,[bp+di+0x65],0x2a29
0000C0BB  004E02            add [bp+0x2],cl
0000C0BE  4C                dec sp
0000C0BF  657665            gs jna 0xc127
0000C0C2  6C                insb
0000C0C3  20466F            and [bp+0x6f],al
0000C0C6  7572              jnz 0xc13a
0000C0C8  202D              and [di],ch
0000C0CA  2020              and [bx+si],ah
0000C0CC  4F                dec di
0000C0CD  7070              jo 0xc13f
0000C0CF  6F                outsw
0000C0D0  6E                outsb
0000C0D1  656E              gs outsb
0000C0D3  743A              jz 0xc10f
0000C0D5  20524F            and [bp+si+0x4f],dl
0000C0D8  52                push dx
0000C0D9  4E                dec si
0000C0DA  2028              and [bx+si],ch
0000C0DC  42                inc dx
0000C0DD  6967204261        imul sp,[bx+0x20],0x6142
0000C0E2  64204775          and [fs:bx+0x75],al
0000C0E6  7929              jns 0xc111
0000C0E8  3300              xor ax,[bx+si]
0000C0EA  7C02              jl 0xc0ee
0000C0EC  50                push ax
0000C0ED  69636B2074        imul sp,[bp+di+0x6b],0x7420
0000C0F2  686520            push word 0x2065
0000C0F5  4C                dec sp
0000C0F6  657665            gs jna 0xc15e
0000C0F9  6C                insb
0000C0FA  204E75            and [bp+0x75],cl
0000C0FD  6D                insw
0000C0FE  626572            bound sp,[di+0x72]
0000C101  206F66            and [bx+0x66],ch
0000C104  207468            and [si+0x68],dh
0000C107  65205361          and [gs:bp+di+0x61],dl
0000C10B  7665              jna 0xc172
0000C10D  646761            fs a32 popa
0000C110  6D                insw
0000C111  652E204553        and [cs:di+0x53],al
0000C116  43                inc bx
0000C117  20746F            and [si+0x6f],dh
0000C11A  205175            and [bx+di+0x75],dl
0000C11D  69740E0F00        imul si,[si+0xe],0xf
0000C122  B402              mov ah,0x2
0000C124  284129            sub [bx+di+0x29],al
0000C127  203D              and [di],bh
0000C129  204C65            and [si+0x65],cl
0000C12C  7665              jna 0xc193
0000C12E  6C                insb
0000C12F  204F6E            and [bx+0x6e],cl
0000C132  65850F            test [gs:bx],cx
0000C135  00C8              add al,cl
0000C137  0228              add ch,[bx+si]
0000C139  42                inc dx
0000C13A  2920              sub [bx+si],sp
0000C13C  3D204C            cmp ax,0x4c20
0000C13F  657665            gs jna 0xc1a7
0000C142  6C                insb
0000C143  205477            and [si+0x77],dl
0000C146  6F                outsw
0000C147  A6                cmpsb
0000C148  1900              sbb [bx+si],ax
0000C14A  DC02              fadd qword [bp+si]
0000C14C  284329            sub [bp+di+0x29],al
0000C14F  203D              and [di],bh
0000C151  204C65            and [si+0x65],cl
0000C154  7665              jna 0xc1bb
0000C156  6C                insb
0000C157  205468            and [si+0x68],dl
0000C15A  7265              jc 0xc1c1
0000C15C  65206F72          and [gs:bx+0x72],ch
0000C160  20466F            and [bp+0x6f],al
0000C163  7572              jnz 0xc1d7
0000C165  D901              fld dword [bx+di]
0000C167  00FA              add dl,bh
0000C169  024127            add al,[bx+di+0x27]
0000C16C  0900              or [bx+si],ax
0000C16E  0003              add [bp+di],al
0000C170  4C                dec sp
0000C171  657665            gs jna 0xc1d9
0000C174  6C                insb
0000C175  204F6E            and [bx+0x6e],cl
0000C178  65CE              gs into
0000C17A  0100              add [bx+si],ax
0000C17C  0E                push cs
0000C17D  03424C            add ax,[bp+si+0x4c]
0000C180  0900              or [bx+si],ax
0000C182  1403              adc al,0x3
0000C184  4C                dec sp
0000C185  657665            gs jna 0xc1ed
0000C188  6C                insb
0000C189  205477            and [si+0x77],dl
0000C18C  6F                outsw
0000C18D  0401              add al,0x1
0000C18F  0022              add [bp+si],ah
0000C191  034301            add ax,[bp+di+0x1]
0000C194  1300              adc ax,[bx+si]
0000C196  2803              sub [bp+di],al
0000C198  4C                dec sp
0000C199  657665            gs jna 0xc201
0000C19C  6C                insb
0000C19D  205468            and [si+0x68],dl
0000C1A0  7265              jc 0xc207
0000C1A2  65206F72          and [gs:bx+0x72],ch
0000C1A6  20466F            and [bp+0x6f],al
0000C1A9  7572              jnz 0xc21d
0000C1AB  0013              add [bp+di],dl
0000C1AD  004003            add [bx+si+0x3],al
0000C1B0  4C                dec sp
0000C1B1  657665            gs jna 0xc219
0000C1B4  6C                insb
0000C1B5  7320              jnc 0xc1d7
0000C1B7  54                push sp
0000C1B8  687265            push word 0x6572
0000C1BB  6520262046        and [gs:0x4620],ah
0000C1C0  6F                outsw
0000C1C1  7572              jnz 0xc235
0000C1C3  001F              add [bx],bl
0000C1C5  005803            add [bx+si+0x3],bl
0000C1C8  53                push bx
0000C1C9  55                push bp
0000C1CA  50                push ax
0000C1CB  52                push dx
0000C1CC  45                inc bp
0000C1CD  4D                dec bp
0000C1CE  41                inc cx
0000C1CF  43                inc bx
0000C1D0  59                pop cx
0000C1D1  205361            and [bp+di+0x61],dl
0000C1D4  7665              jna 0xc23b
0000C1D6  6761              a32 popa
0000C1D8  6D                insw
0000C1D9  65204564          and [gs:di+0x64],al
0000C1DD  69746F722F        imul si,[si+0x6f],0x2f72
0000C1E2  43                inc bx
0000C1E3  686561            push word 0x6165
0000C1E6  747F              jz 0xc267
0000C1E8  1200              adc al,[bx+si]
0000C1EA  7C03              jl 0xc1ef
0000C1EC  59                pop cx
0000C1ED  6F                outsw
0000C1EE  7520              jnz 0xc210
0000C1F0  61                popa
0000C1F1  7265              jc 0xc258
0000C1F3  204564            and [di+0x64],al
0000C1F6  6974696E67        imul si,[si+0x69],0x676e
0000C1FB  206120            and [bx+di+0x20],ah
0000C1FE  0900              or [bx+si],ax
0000C200  92                xchg ax,dx
0000C201  0320              add sp,[bx+si]
0000C203  53                push bx
0000C204  61                popa
0000C205  7665              jna 0xc26c
0000C207  6761              a32 popa
0000C209  6D                insw
0000C20A  65B020            gs mov al,0x20
0000C20D  00A00349          add [bx+si+0x4903],ah
0000C211  6E                outsb
0000C212  7075              jo 0xc289
0000C214  7420              jz 0xc236
0000C216  7468              jz 0xc280
0000C218  65205361          and [gs:bp+di+0x61],dl
0000C21C  7665              jna 0xc283
0000C21E  6761              a32 popa
0000C220  6D                insw
0000C221  65204669          and [gs:bp+0x69],al
0000C225  6C                insb
0000C226  65204E61          and [gs:bp+0x61],cl
0000C22A  6D                insw
0000C22B  65202D            and [gs:di],ch
0000C22E  3E2019            and [ds:bx+di],bl
0000C231  00C4              add ah,al
0000C233  035374            add dx,[bp+di+0x74]
0000C236  61                popa
0000C237  7473              jz 0xc2ac
0000C239  20666F            and [bp+0x6f],ah
0000C23C  7220              jc 0xc25e
0000C23E  53                push bx
0000C23F  61                popa
0000C240  7665              jna 0xc2a7
0000C242  6761              a32 popa
0000C244  6D                insw
0000C245  65204669          and [gs:bp+0x69],al
0000C249  6C                insb
0000C24A  653A20            cmp ah,[gs:bx+si]
0000C24D  2901              sub [bx+di],ax
0000C24F  00E2              add dl,ah
0000C251  0350E5            add dx,[bx+si-0x1b]
0000C254  1200              adc al,[bx+si]
0000C256  E80329            call 0xeb5c
0000C259  6C                insb
0000C25A  61                popa
0000C25B  746F              jz 0xc2cc
0000C25D  6F                outsw
0000C25E  6E                outsb
0000C25F  7320              jnc 0xc281
0000C261  55                push bp
0000C262  7067              jo 0xc2cb
0000C264  7261              jc 0xc2c7
0000C266  6465643A01        cmp al,[fs:bx+di]
0000C26B  00FE              add dh,bh
0000C26D  0328              add bp,[bx+si]
0000C26F  81000004          add word [bx+si],0x400
0000C273  0402              add al,0x2
0000C275  0008              add [bx+si],cl
0000C277  044E              add al,0x4e
0000C279  4F                dec di
0000C27A  0800              or [bx+si],al
0000C27C  0E                push cs
0000C27D  0429              add al,0x29
0000C27F  43                inc bx
0000C280  7265              jc 0xc2e7
0000C282  646974730900      imul si,[fs:si+0x73],0x9
0000C288  1A04              sbb al,[si]
0000C28A  294D69            sub [di+0x69],cx
0000C28D  6E                outsb
0000C28E  657261            gs jc 0xc2f2
0000C291  6C                insb
0000C292  73AE              jnc 0xc242
0000C294  050028            add ax,0x2800
0000C297  0429              add al,0x29
0000C299  46                inc si
0000C29A  7565              jnz 0xc301
0000C29C  6C                insb
0000C29D  BF0700            mov di,0x7
0000C2A0  3204              xor al,[si]
0000C2A2  29456E            sub [di+0x6e],ax
0000C2A5  657267            gs jc 0xc30f
0000C2A8  79CC              jns 0xc276
0000C2AA  0B00              or ax,[bx+si]
0000C2AC  3E0429            ds add al,0x29
0000C2AF  50                push ax
0000C2B0  6F                outsw
0000C2B1  7075              jo 0xc328
0000C2B3  6C                insb
0000C2B4  61                popa
0000C2B5  7469              jz 0xc320
0000C2B7  6F                outsw
0000C2B8  6E                outsb
0000C2B9  DB05              fild dword [di]
0000C2BB  004E04            add [bp+0x4],cl
0000C2BE  29466F            sub [bp+0x6f],ax
0000C2C1  6F                outsw
0000C2C2  64EE              fs out dx,al
0000C2C4  0100              add [bx+si],ax
0000C2C6  58                pop ax
0000C2C7  0444              add al,0x44
0000C2C9  FB                sti
0000C2CA  0100              add [bx+si],ax
0000C2CC  5E                pop si
0000C2CD  0445              add al,0x45
0000C2CF  0401              add al,0x1
0000C2D1  006404            add [si+0x4],ah
0000C2D4  46                inc si
0000C2D5  0D3D00            or ax,0x3d
0000C2D8  6A04              push word 0x4
0000C2DA  45                inc bp
0000C2DB  6E                outsb
0000C2DC  7465              jz 0xc343
0000C2DE  7220              jc 0xc300
0000C2E0  7468              jz 0xc34a
0000C2E2  65206C65          and [gs:si+0x65],ch
0000C2E6  7474              jz 0xc35c
0000C2E8  657220            gs jc 0xc30b
0000C2EB  636F72            arpl [bx+0x72],bp
0000C2EE  7265              jc 0xc355
0000C2F0  7370              jnc 0xc362
0000C2F2  6F                outsw
0000C2F3  6E                outsb
0000C2F4  64696E672074      imul bp,[fs:bp+0x67],0x7420
0000C2FA  6F                outsw
0000C2FB  207468            and [si+0x68],dh
0000C2FE  65206974          and [gs:bx+di+0x74],ch
0000C302  656D              gs insw
0000C304  20746F            and [si+0x6f],dh
0000C307  206368            and [bp+di+0x68],ah
0000C30A  61                popa
0000C30B  6E                outsb
0000C30C  67652E204F72      and [cs:edi+0x72],cl
0000C312  207479            and [si+0x79],dh
0000C315  7065              jo 0xc37c
0000C317  16                push ss
0000C318  42                inc dx
0000C319  00AC044D          add [si+0x4d04],ch
0000C31D  20746F            and [si+0x6f],dh
0000C320  204D61            and [di+0x61],cl
0000C323  7820              js 0xc345
0000C325  6F                outsw
0000C326  7574              jnz 0xc39c
0000C328  206576            and [di+0x76],ah
0000C32B  657279            gs jc 0xc3a7
0000C32E  7468              jz 0xc398
0000C330  696E672E20        imul bp,[bp+0x67],0x202e
0000C335  53                push bx
0000C336  205361            and [bp+di+0x61],dl
0000C339  7665              jna 0xc3a0
0000C33B  7320              jnc 0xc35d
0000C33D  43                inc bx
0000C33E  68616E            push word 0x6e61
0000C341  6765732E          gs a32 jnc 0xc373
0000C345  205120            and [bx+di+0x20],dl
0000C348  51                push cx
0000C349  7569              jnz 0xc3b4
0000C34B  7473              jz 0xc3c0
0000C34D  207769            and [bx+0x69],dh
0000C350  7468              jz 0xc3ba
0000C352  6F                outsw
0000C353  7574              jnz 0xc3c9
0000C355  206368            and [bp+di+0x68],ah
0000C358  61                popa
0000C359  6E                outsb
0000C35A  67696E674500      imul bp,[esi+0x67],0x45
0000C360  F20439            repne add al,0x39
0000C363  3920              cmp [bx+si],sp
0000C365  4D                dec bp
0000C366  696C6C696F        imul bp,[si+0x6c],0x6f69
0000C36B  6E                outsb
0000C36C  206973            and [bx+di+0x73],ch
0000C36F  204D61            and [di+0x61],cl
0000C372  7820              js 0xc394
0000C374  666F              outsd
0000C376  7220              jc 0xc398
0000C378  43                inc bx
0000C379  7265              jc 0xc3e0
0000C37B  646974732E20      imul si,[fs:si+0x73],0x202e
0000C381  3330              xor si,[bx+si]
0000C383  207468            and [si+0x68],dh
0000C386  6F                outsw
0000C387  7573              jnz 0xc3fc
0000C389  61                popa
0000C38A  6E                outsb
0000C38B  64206973          and [fs:bx+di+0x73],ch
0000C38F  204D61            and [di+0x61],cl
0000C392  7820              js 0xc3b4
0000C394  666F              outsd
0000C396  7220              jc 0xc3b8
0000C398  657665            gs jna 0xc400
0000C39B  7279              jc 0xc416
0000C39D  7468              jz 0xc407
0000C39F  696E672065        imul bp,[bp+0x67],0x6520
0000C3A4  6C                insb
0000C3A5  7365              jnc 0xc40c
0000C3A7  D913              fst dword [bp+di]
0000C3A9  003C              add [si],bh
0000C3AB  054E75            add ax,0x754e
0000C3AE  6D                insw
0000C3AF  626572            bound sp,[di+0x72]
0000C3B2  206F66            and [bx+0x66],ch
0000C3B5  204372            and [bp+di+0x72],al
0000C3B8  65646974733A20    imul si,[fs:si+0x73],0x203a
0000C3BF  16                push ss
0000C3C0  1400              adc al,0x0
0000C3C2  54                push sp
0000C3C3  054E75            add ax,0x754e
0000C3C6  6D                insw
0000C3C7  626572            bound sp,[di+0x72]
0000C3CA  206F66            and [bx+0x66],ch
0000C3CD  204D69            and [di+0x69],cl
0000C3D0  6E                outsb
0000C3D1  657261            gs jc 0xc435
0000C3D4  6C                insb
0000C3D5  733A              jnc 0xc411
0000C3D7  2010              and [bx+si],dl
0000C3D9  006C05            add [si+0x5],ch
0000C3DC  4E                dec si
0000C3DD  756D              jnz 0xc44c
0000C3DF  626572            bound sp,[di+0x72]
0000C3E2  206F66            and [bx+0x66],ch
0000C3E5  204675            and [bp+0x75],al
0000C3E8  656C              gs insb
0000C3EA  3A20              cmp ah,[bx+si]
0000C3EC  1200              adc al,[bx+si]
0000C3EE  80054E            add byte [di],0x4e
0000C3F1  756D              jnz 0xc460
0000C3F3  626572            bound sp,[di+0x72]
0000C3F6  206F66            and [bx+0x66],ch
0000C3F9  20456E            and [di+0x6e],al
0000C3FC  657267            gs jc 0xc466
0000C3FF  793A              jns 0xc43b
0000C401  2012              and [bp+si],dl
0000C403  0096054E          add [bp+0x4e05],dl
0000C407  756D              jnz 0xc476
0000C409  626572            bound sp,[di+0x72]
0000C40C  206F66            and [bx+0x66],ch
0000C40F  205065            and [bx+si+0x65],dl
0000C412  6F                outsw
0000C413  706C              jo 0xc481
0000C415  653A20            cmp ah,[gs:bx+si]
0000C418  0400              add al,0x0
0000C41A  AC                lodsb
0000C41B  055945            add ax,0x4559
0000C41E  53                push bx
0000C41F  2110              and [bx+si],dx
0000C421  00B4054E          add [si+0x4e05],dh
0000C425  756D              jnz 0xc494
0000C427  626572            bound sp,[di+0x72]
0000C42A  206F66            and [bx+0x66],ch
0000C42D  20466F            and [bp+0x6f],al
0000C430  6F                outsw
0000C431  643A20            cmp ah,[fs:bx+si]
0000C434  0100              add [bx+si],ax
0000C436  C8054D00          enter word 0x4d05,byte 0x0
0000C43A  0100              add [bx+si],ax
0000C43C  CE                into
0000C43D  055122            add ax,0x2251
0000C440  0100              add [bx+si],ax
0000C442  D405              aam byte 0x5
0000C444  53                push bx
0000C445  2200              and al,[bx+si]
0000C447  0000              add [bx+si],al
0000C449  0000              add [bx+si],al
0000C44B  0000              add [bx+si],al
0000C44D  0000              add [bx+si],al
0000C44F  00FF              add bh,bh
0000C451  FF01              inc word [bx+di]
0000C453  0000              add [bx+si],al
0000C455  0000              add [bx+si],al
0000C457  0000              add [bx+si],al
0000C459  0000              add [bx+si],al
0000C45B  0000              add [bx+si],al
0000C45D  0000              add [bx+si],al
0000C45F  0000              add [bx+si],al
0000C461  0000              add [bx+si],al
0000C463  0000              add [bx+si],al
0000C465  0000              add [bx+si],al
0000C467  0000              add [bx+si],al
0000C469  0000              add [bx+si],al
0000C46B  0000              add [bx+si],al
0000C46D  0000              add [bx+si],al
0000C46F  0000              add [bx+si],al
0000C471  0000              add [bx+si],al
0000C473  0000              add [bx+si],al
0000C475  0000              add [bx+si],al
0000C477  0000              add [bx+si],al
0000C479  0000              add [bx+si],al
0000C47B  0000              add [bx+si],al
0000C47D  0000              add [bx+si],al
0000C47F  0000              add [bx+si],al
0000C481  0000              add [bx+si],al
0000C483  0000              add [bx+si],al
0000C485  0000              add [bx+si],al
0000C487  0000              add [bx+si],al
0000C489  0000              add [bx+si],al
0000C48B  0000              add [bx+si],al
0000C48D  0000              add [bx+si],al
0000C48F  0000              add [bx+si],al
0000C491  0000              add [bx+si],al
0000C493  0000              add [bx+si],al
0000C495  0000              add [bx+si],al
0000C497  0000              add [bx+si],al
0000C499  0000              add [bx+si],al
0000C49B  0000              add [bx+si],al
0000C49D  0000              add [bx+si],al
0000C49F  0000              add [bx+si],al
0000C4A1  0000              add [bx+si],al
0000C4A3  0000              add [bx+si],al
0000C4A5  0000              add [bx+si],al
0000C4A7  0000              add [bx+si],al
0000C4A9  0000              add [bx+si],al
0000C4AB  0000              add [bx+si],al
0000C4AD  0000              add [bx+si],al
0000C4AF  0000              add [bx+si],al
0000C4B1  0000              add [bx+si],al
0000C4B3  0000              add [bx+si],al
0000C4B5  0000              add [bx+si],al
0000C4B7  0000              add [bx+si],al
0000C4B9  0000              add [bx+si],al
0000C4BB  0000              add [bx+si],al
0000C4BD  0000              add [bx+si],al
0000C4BF  0000              add [bx+si],al
0000C4C1  0000              add [bx+si],al
0000C4C3  0000              add [bx+si],al
0000C4C5  0000              add [bx+si],al
0000C4C7  0000              add [bx+si],al
0000C4C9  0000              add [bx+si],al
0000C4CB  0000              add [bx+si],al
0000C4CD  0000              add [bx+si],al
0000C4CF  0000              add [bx+si],al
0000C4D1  0000              add [bx+si],al
0000C4D3  0000              add [bx+si],al
0000C4D5  0000              add [bx+si],al
0000C4D7  0000              add [bx+si],al
0000C4D9  0000              add [bx+si],al
0000C4DB  0000              add [bx+si],al
0000C4DD  0000              add [bx+si],al
0000C4DF  0000              add [bx+si],al
0000C4E1  0000              add [bx+si],al
0000C4E3  0000              add [bx+si],al
0000C4E5  0000              add [bx+si],al
0000C4E7  0000              add [bx+si],al
0000C4E9  0000              add [bx+si],al
0000C4EB  0000              add [bx+si],al
0000C4ED  0000              add [bx+si],al
0000C4EF  0000              add [bx+si],al
0000C4F1  0000              add [bx+si],al
0000C4F3  0000              add [bx+si],al
0000C4F5  0000              add [bx+si],al
0000C4F7  0000              add [bx+si],al
0000C4F9  0000              add [bx+si],al
0000C4FB  0000              add [bx+si],al
0000C4FD  0000              add [bx+si],al
0000C4FF  0000              add [bx+si],al
0000C501  0000              add [bx+si],al
0000C503  0000              add [bx+si],al
0000C505  0000              add [bx+si],al
0000C507  0000              add [bx+si],al
0000C509  0000              add [bx+si],al
0000C50B  0000              add [bx+si],al
0000C50D  0000              add [bx+si],al
0000C50F  0000              add [bx+si],al
0000C511  0000              add [bx+si],al
0000C513  0000              add [bx+si],al
0000C515  0000              add [bx+si],al
0000C517  0000              add [bx+si],al
0000C519  0000              add [bx+si],al
0000C51B  0000              add [bx+si],al
0000C51D  0000              add [bx+si],al
0000C51F  0000              add [bx+si],al
0000C521  0000              add [bx+si],al
0000C523  0000              add [bx+si],al
0000C525  0000              add [bx+si],al
0000C527  0000              add [bx+si],al
0000C529  0000              add [bx+si],al
0000C52B  0000              add [bx+si],al
0000C52D  0000              add [bx+si],al
0000C52F  0000              add [bx+si],al
0000C531  0000              add [bx+si],al
0000C533  0000              add [bx+si],al
0000C535  0000              add [bx+si],al
0000C537  0000              add [bx+si],al
0000C539  0000              add [bx+si],al
0000C53B  0000              add [bx+si],al
0000C53D  0000              add [bx+si],al
0000C53F  0000              add [bx+si],al
0000C541  0000              add [bx+si],al
0000C543  0000              add [bx+si],al
0000C545  0000              add [bx+si],al
0000C547  0000              add [bx+si],al
0000C549  0000              add [bx+si],al
0000C54B  0000              add [bx+si],al
0000C54D  0000              add [bx+si],al
0000C54F  0000              add [bx+si],al
0000C551  0000              add [bx+si],al
0000C553  0000              add [bx+si],al
0000C555  0000              add [bx+si],al
0000C557  0000              add [bx+si],al
0000C559  0000              add [bx+si],al
0000C55B  0000              add [bx+si],al
0000C55D  0000              add [bx+si],al
0000C55F  0000              add [bx+si],al
0000C561  0000              add [bx+si],al
0000C563  0000              add [bx+si],al
0000C565  0000              add [bx+si],al
0000C567  0000              add [bx+si],al
0000C569  0000              add [bx+si],al
0000C56B  0000              add [bx+si],al
0000C56D  0000              add [bx+si],al
0000C56F  0000              add [bx+si],al
0000C571  0000              add [bx+si],al
0000C573  0000              add [bx+si],al
0000C575  0000              add [bx+si],al
0000C577  0000              add [bx+si],al
0000C579  0000              add [bx+si],al
0000C57B  0000              add [bx+si],al
0000C57D  0000              add [bx+si],al
0000C57F  0000              add [bx+si],al
0000C581  0000              add [bx+si],al
0000C583  0000              add [bx+si],al
0000C585  0000              add [bx+si],al
0000C587  0000              add [bx+si],al
0000C589  0000              add [bx+si],al
0000C58B  0000              add [bx+si],al
0000C58D  0000              add [bx+si],al
0000C58F  0000              add [bx+si],al
0000C591  0000              add [bx+si],al
0000C593  0000              add [bx+si],al
0000C595  0000              add [bx+si],al
0000C597  0000              add [bx+si],al
0000C599  0000              add [bx+si],al
0000C59B  0000              add [bx+si],al
0000C59D  0000              add [bx+si],al
0000C59F  0000              add [bx+si],al
0000C5A1  0000              add [bx+si],al
0000C5A3  0000              add [bx+si],al
0000C5A5  0000              add [bx+si],al
0000C5A7  0000              add [bx+si],al
0000C5A9  0000              add [bx+si],al
0000C5AB  00EA              add dl,ch
0000C5AD  2BEA              sub bp,dx
0000C5AF  2BEA              sub bp,dx
0000C5B1  2BEA              sub bp,dx
0000C5B3  2B00              sub ax,[bx+si]
0000C5B5  00EA              add dl,ch
0000C5B7  2BEA              sub bp,dx
0000C5B9  2B00              sub ax,[bx+si]
0000C5BB  0000              add [bx+si],al
0000C5BD  0000              add [bx+si],al
0000C5BF  0000              add [bx+si],al
0000C5C1  0000              add [bx+si],al
0000C5C3  0000              add [bx+si],al
0000C5C5  0000              add [bx+si],al
0000C5C7  0000              add [bx+si],al
0000C5C9  0000              add [bx+si],al
0000C5CB  0000              add [bx+si],al
0000C5CD  0000              add [bx+si],al
0000C5CF  0000              add [bx+si],al
0000C5D1  0000              add [bx+si],al
0000C5D3  0000              add [bx+si],al
0000C5D5  0000              add [bx+si],al
0000C5D7  0000              add [bx+si],al
0000C5D9  0000              add [bx+si],al
0000C5DB  0000              add [bx+si],al
0000C5DD  0000              add [bx+si],al
0000C5DF  0000              add [bx+si],al
0000C5E1  0000              add [bx+si],al
0000C5E3  0000              add [bx+si],al
0000C5E5  0000              add [bx+si],al
0000C5E7  0000              add [bx+si],al
0000C5E9  0000              add [bx+si],al
0000C5EB  0000              add [bx+si],al
0000C5ED  0000              add [bx+si],al
0000C5EF  0000              add [bx+si],al
0000C5F1  0000              add [bx+si],al
0000C5F3  0000              add [bx+si],al
0000C5F5  0000              add [bx+si],al
0000C5F7  0000              add [bx+si],al
0000C5F9  0000              add [bx+si],al
0000C5FB  0000              add [bx+si],al
0000C5FD  0000              add [bx+si],al
0000C5FF  0000              add [bx+si],al
0000C601  0000              add [bx+si],al
0000C603  0000              add [bx+si],al
0000C605  0000              add [bx+si],al
0000C607  0000              add [bx+si],al
0000C609  0000              add [bx+si],al
0000C60B  0000              add [bx+si],al
0000C60D  0000              add [bx+si],al
0000C60F  0000              add [bx+si],al
0000C611  0000              add [bx+si],al
0000C613  0000              add [bx+si],al
0000C615  0000              add [bx+si],al
0000C617  0000              add [bx+si],al
0000C619  0000              add [bx+si],al
0000C61B  0000              add [bx+si],al
0000C61D  0000              add [bx+si],al
0000C61F  0000              add [bx+si],al
0000C621  0000              add [bx+si],al
0000C623  0000              add [bx+si],al
0000C625  0000              add [bx+si],al
0000C627  0000              add [bx+si],al
0000C629  0000              add [bx+si],al
0000C62B  0000              add [bx+si],al
0000C62D  0000              add [bx+si],al
0000C62F  0000              add [bx+si],al
0000C631  0000              add [bx+si],al
0000C633  0000              add [bx+si],al
0000C635  0000              add [bx+si],al
0000C637  0000              add [bx+si],al
0000C639  0000              add [bx+si],al
0000C63B  0000              add [bx+si],al
0000C63D  0000              add [bx+si],al
0000C63F  0000              add [bx+si],al
0000C641  0000              add [bx+si],al
0000C643  0000              add [bx+si],al
0000C645  0000              add [bx+si],al
0000C647  0000              add [bx+si],al
0000C649  0000              add [bx+si],al
0000C64B  0000              add [bx+si],al
0000C64D  0000              add [bx+si],al
0000C64F  0000              add [bx+si],al
0000C651  0000              add [bx+si],al
0000C653  0000              add [bx+si],al
0000C655  0000              add [bx+si],al
0000C657  0000              add [bx+si],al
0000C659  0000              add [bx+si],al
0000C65B  0000              add [bx+si],al
0000C65D  0000              add [bx+si],al
0000C65F  0000              add [bx+si],al
0000C661  0000              add [bx+si],al
0000C663  0000              add [bx+si],al
0000C665  0000              add [bx+si],al
0000C667  0000              add [bx+si],al
0000C669  0000              add [bx+si],al
0000C66B  0000              add [bx+si],al
0000C66D  0000              add [bx+si],al
0000C66F  0000              add [bx+si],al
0000C671  0000              add [bx+si],al
0000C673  0000              add [bx+si],al
0000C675  0000              add [bx+si],al
0000C677  0000              add [bx+si],al
0000C679  0000              add [bx+si],al
0000C67B  0000              add [bx+si],al
0000C67D  0000              add [bx+si],al
0000C67F  0000              add [bx+si],al
0000C681  0000              add [bx+si],al
0000C683  0000              add [bx+si],al
0000C685  0000              add [bx+si],al
0000C687  0000              add [bx+si],al
0000C689  0000              add [bx+si],al
0000C68B  0000              add [bx+si],al
0000C68D  0000              add [bx+si],al
0000C68F  0000              add [bx+si],al
0000C691  0000              add [bx+si],al
0000C693  0000              add [bx+si],al
0000C695  0000              add [bx+si],al
0000C697  0000              add [bx+si],al
0000C699  0000              add [bx+si],al
0000C69B  0000              add [bx+si],al
0000C69D  0000              add [bx+si],al
0000C69F  0000              add [bx+si],al
0000C6A1  0000              add [bx+si],al
0000C6A3  0000              add [bx+si],al
0000C6A5  0000              add [bx+si],al
0000C6A7  0000              add [bx+si],al
0000C6A9  0000              add [bx+si],al
0000C6AB  0000              add [bx+si],al
0000C6AD  0000              add [bx+si],al
0000C6AF  0000              add [bx+si],al
0000C6B1  0000              add [bx+si],al
0000C6B3  0000              add [bx+si],al
0000C6B5  0000              add [bx+si],al
0000C6B7  0000              add [bx+si],al
0000C6B9  0000              add [bx+si],al
0000C6BB  0000              add [bx+si],al
0000C6BD  0000              add [bx+si],al
0000C6BF  0000              add [bx+si],al
0000C6C1  0000              add [bx+si],al
0000C6C3  0000              add [bx+si],al
0000C6C5  0000              add [bx+si],al
0000C6C7  0000              add [bx+si],al
0000C6C9  0000              add [bx+si],al
0000C6CB  0000              add [bx+si],al
0000C6CD  0000              add [bx+si],al
0000C6CF  0000              add [bx+si],al
0000C6D1  0000              add [bx+si],al
0000C6D3  0000              add [bx+si],al
0000C6D5  0000              add [bx+si],al
0000C6D7  0000              add [bx+si],al
0000C6D9  0000              add [bx+si],al
0000C6DB  0000              add [bx+si],al
0000C6DD  0000              add [bx+si],al
0000C6DF  0000              add [bx+si],al
0000C6E1  0000              add [bx+si],al
0000C6E3  0000              add [bx+si],al
0000C6E5  0000              add [bx+si],al
0000C6E7  0000              add [bx+si],al
0000C6E9  0000              add [bx+si],al
0000C6EB  0000              add [bx+si],al
0000C6ED  0000              add [bx+si],al
0000C6EF  0000              add [bx+si],al
0000C6F1  0000              add [bx+si],al
0000C6F3  0000              add [bx+si],al
0000C6F5  0000              add [bx+si],al
0000C6F7  0000              add [bx+si],al
0000C6F9  0000              add [bx+si],al
0000C6FB  0000              add [bx+si],al
0000C6FD  0000              add [bx+si],al
0000C6FF  0000              add [bx+si],al
0000C701  0000              add [bx+si],al
0000C703  0000              add [bx+si],al
0000C705  0000              add [bx+si],al
0000C707  0000              add [bx+si],al
0000C709  0000              add [bx+si],al
0000C70B  0000              add [bx+si],al
0000C70D  0000              add [bx+si],al
0000C70F  0000              add [bx+si],al
0000C711  0000              add [bx+si],al
0000C713  0000              add [bx+si],al
0000C715  0000              add [bx+si],al
0000C717  0000              add [bx+si],al
0000C719  0000              add [bx+si],al
0000C71B  0000              add [bx+si],al
0000C71D  0000              add [bx+si],al
0000C71F  0000              add [bx+si],al
0000C721  0000              add [bx+si],al
0000C723  0000              add [bx+si],al
0000C725  0000              add [bx+si],al
0000C727  0000              add [bx+si],al
0000C729  0000              add [bx+si],al
0000C72B  0000              add [bx+si],al
0000C72D  0000              add [bx+si],al
0000C72F  0000              add [bx+si],al
0000C731  0000              add [bx+si],al
0000C733  0000              add [bx+si],al
0000C735  0000              add [bx+si],al
0000C737  0000              add [bx+si],al
0000C739  0000              add [bx+si],al
0000C73B  0000              add [bx+si],al
0000C73D  0000              add [bx+si],al
0000C73F  0000              add [bx+si],al
0000C741  0000              add [bx+si],al
0000C743  0000              add [bx+si],al
0000C745  0000              add [bx+si],al
0000C747  0000              add [bx+si],al
0000C749  0000              add [bx+si],al
0000C74B  0000              add [bx+si],al
0000C74D  0000              add [bx+si],al
0000C74F  0000              add [bx+si],al
0000C751  0000              add [bx+si],al
0000C753  0000              add [bx+si],al
0000C755  0000              add [bx+si],al
0000C757  0000              add [bx+si],al
0000C759  0000              add [bx+si],al
0000C75B  0000              add [bx+si],al
0000C75D  0000              add [bx+si],al
0000C75F  0000              add [bx+si],al
0000C761  0000              add [bx+si],al
0000C763  0000              add [bx+si],al
0000C765  0000              add [bx+si],al
0000C767  0000              add [bx+si],al
0000C769  0000              add [bx+si],al
0000C76B  0000              add [bx+si],al
0000C76D  0000              add [bx+si],al
0000C76F  0000              add [bx+si],al
0000C771  0000              add [bx+si],al
0000C773  0000              add [bx+si],al
0000C775  0000              add [bx+si],al
0000C777  0000              add [bx+si],al
0000C779  0000              add [bx+si],al
0000C77B  0000              add [bx+si],al
0000C77D  0000              add [bx+si],al
0000C77F  0000              add [bx+si],al
0000C781  0000              add [bx+si],al
0000C783  0000              add [bx+si],al
0000C785  0000              add [bx+si],al
0000C787  0000              add [bx+si],al
0000C789  0000              add [bx+si],al
0000C78B  0000              add [bx+si],al
0000C78D  0000              add [bx+si],al
0000C78F  0000              add [bx+si],al
0000C791  0000              add [bx+si],al
0000C793  0000              add [bx+si],al
0000C795  0000              add [bx+si],al
0000C797  0000              add [bx+si],al
0000C799  0000              add [bx+si],al
0000C79B  0000              add [bx+si],al
0000C79D  0000              add [bx+si],al
0000C79F  004C50            add [si+0x50],cl
0000C7A2  54                push sp
0000C7A3  2000              and [bx+si],al
0000C7A5  0000              add [bx+si],al
0000C7A7  0002              add [bp+si],al
0000C7A9  0000              add [bx+si],al
0000C7AB  0000              add [bx+si],al
0000C7AD  0000              add [bx+si],al
0000C7AF  0000              add [bx+si],al
0000C7B1  0000              add [bx+si],al
0000C7B3  0000              add [bx+si],al
0000C7B5  0000              add [bx+si],al
0000C7B7  0000              add [bx+si],al
0000C7B9  0000              add [bx+si],al
0000C7BB  0000              add [bx+si],al
0000C7BD  0002              add [bp+si],al
0000C7BF  0000              add [bx+si],al
0000C7C1  0000              add [bx+si],al
0000C7C3  0000              add [bx+si],al
0000C7C5  0000              add [bx+si],al
0000C7C7  0000              add [bx+si],al
0000C7C9  0000              add [bx+si],al
0000C7CB  0000              add [bx+si],al
0000C7CD  0000              add [bx+si],al
0000C7CF  0000              add [bx+si],al
0000C7D1  004908            add [bx+di+0x8],cl
0000C7D4  2E42              cs inc dx
0000C7D6  41                inc cx
0000C7D7  53                push bx
0000C7D8  002E4558          add [0x5845],ch
0000C7DC  45                inc bp
0000C7DD  0000              add [bx+si],al
0000C7DF  0000              add [bx+si],al
0000C7E1  00B51800          add [di+0x18],dh
0000C7E5  0000              add [bx+si],al
0000C7E7  00EA              add dl,ch
0000C7E9  2B7C09            sub di,[si+0x9]
0000C7EC  0000              add [bx+si],al
0000C7EE  7C09              jl 0xc7f9
0000C7F0  FFFF              udw
0000C7F2  FFFF              udw
0000C7F4  FFFF              udw
0000C7F6  FFFF              udw
0000C7F8  FFFF              udw
0000C7FA  FFFF              udw
0000C7FC  FFFF              udw
0000C7FE  FFFF              udw
0000C800  FFFF              udw
0000C802  FFFF              udw
0000C804  FFFF              udw
0000C806  FFFF              udw
0000C808  FFFF              udw
0000C80A  FFFF              udw
0000C80C  FFFF              udw
0000C80E  FFFF              udw
0000C810  FFFF              udw
0000C812  FFFF              udw
0000C814  FFFF              udw
0000C816  FFFF              udw
0000C818  FFFF              udw
0000C81A  FFFF              udw
0000C81C  FFFF              udw
0000C81E  FFFF              udw
0000C820  FFFF              udw
0000C822  FFFF              udw
0000C824  FFFF              udw
0000C826  FFFF              udw
0000C828  FFFF              udw
0000C82A  FFFF              udw
0000C82C  FFFF              udw
0000C82E  FFFF              udw
0000C830  FFFF              udw
0000C832  FFFF              udw
0000C834  FFFF              udw
0000C836  FFFF              udw
0000C838  FFFF              udw
0000C83A  FFFF              udw
0000C83C  FFFF              udw
0000C83E  FFFF              udw
0000C840  FFFF              udw
0000C842  FFFF              udw
0000C844  FFFF              udw
0000C846  FFFF              udw
0000C848  FFFF              udw
0000C84A  FFFF              udw
0000C84C  FFFF              udw
0000C84E  FFFF              udw
0000C850  FFFF              udw
0000C852  FFFF              udw
0000C854  FFFF              udw
0000C856  FFFF              udw
0000C858  FFFF              udw
0000C85A  FFFF              udw
0000C85C  FFFF              udw
0000C85E  FFFF              udw
0000C860  FFFF              udw
0000C862  FFFF              udw
0000C864  FFFF              udw
0000C866  FFFF              udw
0000C868  831D00            sbb word [di],0x0
0000C86B  00C6              add dh,al
0000C86D  23C6              and ax,si
0000C86F  23C6              and ax,si
0000C871  23E0              and sp,ax
0000C873  2401              and al,0x1
0000C875  0000              add [bx+si],al
0000C877  0007              add [bx],al
0000C879  00EA              add dl,ch
0000C87B  2BEA              sub bp,dx
0000C87D  2BEA              sub bp,dx
0000C87F  2BEA              sub bp,dx
0000C881  2BEA              sub bp,dx
0000C883  2BEA              sub bp,dx
0000C885  2BEA              sub bp,dx
0000C887  2B03              sub ax,[bp+di]
0000C889  00EA              add dl,ch
0000C88B  2BEA              sub bp,dx
0000C88D  2BEA              sub bp,dx
0000C88F  2B0A              sub cx,[bp+si]
0000C891  00EA              add dl,ch
0000C893  2BEA              sub bp,dx
0000C895  2BEA              sub bp,dx
0000C897  2BEA              sub bp,dx
0000C899  2BEA              sub bp,dx
0000C89B  2BEA              sub bp,dx
0000C89D  2BEA              sub bp,dx
0000C89F  2BEA              sub bp,dx
0000C8A1  2BEA              sub bp,dx
0000C8A3  2BEA              sub bp,dx
0000C8A5  2B01              sub ax,[bx+di]
0000C8A7  00EA              add dl,ch
0000C8A9  2B04              sub ax,[si]
0000C8AB  00EA              add dl,ch
0000C8AD  2BEA              sub bp,dx
0000C8AF  2BEA              sub bp,dx
0000C8B1  2BEA              sub bp,dx
0000C8B3  2B03              sub ax,[bp+di]
0000C8B5  00EA              add dl,ch
0000C8B7  2BEA              sub bp,dx
0000C8B9  2BEA              sub bp,dx
0000C8BB  2B04              sub ax,[si]
0000C8BD  00EA              add dl,ch
0000C8BF  2BEA              sub bp,dx
0000C8C1  2BEA              sub bp,dx
0000C8C3  2BEA              sub bp,dx
0000C8C5  2B04              sub ax,[si]
0000C8C7  00EA              add dl,ch
0000C8C9  2BEA              sub bp,dx
0000C8CB  2BEA              sub bp,dx
0000C8CD  2BEA              sub bp,dx
0000C8CF  2B03              sub ax,[bp+di]
0000C8D1  00EA              add dl,ch
0000C8D3  2BEA              sub bp,dx
0000C8D5  2BEA              sub bp,dx
0000C8D7  2B00              sub ax,[bx+si]
0000C8D9  0000              add [bx+si],al
0000C8DB  0001              add [bx+di],al
0000C8DD  0118              add [bx+si],bx
0000C8DF  50                push ax
0000C8E0  0118              add [bx+si],bx
0000C8E2  0000              add [bx+si],al
0000C8E4  2907              sub [bx],ax
0000C8E6  EA2BEA2BEA        jmp word 0xea2b:word 0xea2b
0000C8EB  2BEA              sub bp,dx
0000C8ED  2BEA              sub bp,dx
0000C8EF  2B5000            sub dx,[bx+si+0x0]
0000C8F2  0005              add [di],al
0000C8F4  0000              add [bx+si],al
0000C8F6  0000              add [bx+si],al
0000C8F8  C7070000          mov word [bx],0x0
0000C8FC  EA2BEA2B00        jmp word 0x2b:word 0xea2b
0000C901  00FF              add bh,bh
0000C903  FF07              inc word [bx]
0000C905  0000              add [bx+si],al
0000C907  99                cwd
0000C908  0A00              or al,[bx+si]
0000C90A  CF                iret
0000C90B  3200              xor al,[bx+si]
0000C90D  EA2B000000        jmp word 0x0:word 0x2b
0000C912  0028              add [bx+si],ch
0000C914  194001            sbb [bx+si+0x1],ax
0000C917  C80000B8          enter word 0x0,byte 0xb8
0000C91B  0F0002            sldt word [bp+si]
0000C91E  800000            add byte [bx+si],0x0
0000C921  07                pop es
0000C922  07                pop es
0000C923  0000              add [bx+si],al
0000C925  0000              add [bx+si],al
0000C927  0000              add [bx+si],al
0000C929  1A30              sbb dh,[bx+si]
0000C92B  1A30              sbb dh,[bx+si]
0000C92D  1A30              sbb dh,[bx+si]
0000C92F  1A30              sbb dh,[bx+si]
0000C931  1A30              sbb dh,[bx+si]
0000C933  1A30              sbb dh,[bx+si]
0000C935  1A30              sbb dh,[bx+si]
0000C937  1A30              sbb dh,[bx+si]
0000C939  07                pop es
0000C93A  0101              add [bx+di],ax
0000C93C  2800              sub [bx+si],al
0000C93E  800108            add byte [bx+di],0x8
0000C941  0003              add [bp+di],al
0000C943  07                pop es
0000C944  00D5              add ch,dl
0000C946  0033              add [bp+di],dh
0000C948  011A              add [bp+si],bx
0000C94A  301A              xor [bp+si],bl
0000C94C  301A              xor [bp+si],bl
0000C94E  301A              xor [bp+si],bl
0000C950  301A              xor [bp+si],bl
0000C952  301A              xor [bp+si],bl
0000C954  301A              xor [bp+si],bl
0000C956  301A              xor [bp+si],bl
0000C958  301A              xor [bp+si],bl
0000C95A  301A              xor [bp+si],bl
0000C95C  301A              xor [bp+si],bl
0000C95E  301A              xor [bp+si],bl
0000C960  301A              xor [bp+si],bl
0000C962  301A              xor [bp+si],bl
0000C964  301A              xor [bp+si],bl
0000C966  301A              xor [bp+si],bl
0000C968  301A              xor [bp+si],bl
0000C96A  301A              xor [bp+si],bl
0000C96C  301A              xor [bp+si],bl
0000C96E  301A              xor [bp+si],bl
0000C970  301A              xor [bp+si],bl
0000C972  301A              xor [bp+si],bl
0000C974  301A              xor [bp+si],bl
0000C976  301A              xor [bp+si],bl
0000C978  3000              xor [bx+si],al
0000C97A  EA2B700707        jmp word 0x707:word 0x702b
0000C97F  0407              add al,0x7
0000C981  07                pop es
0000C982  0000              add [bx+si],al
0000C984  0000              add [bx+si],al
0000C986  0000              add [bx+si],al
0000C988  0000              add [bx+si],al
0000C98A  1900              sbb [bx+si],ax
0000C98C  E83BE8            call 0xb1ca
0000C98F  3B00              cmp ax,[bx+si]
0000C991  0000              add [bx+si],al
0000C993  0000              add [bx+si],al
0000C995  E83B00            call 0xc9d3
0000C998  7929              jns 0xc9c3
0000C99A  7929              jns 0xc9c5
0000C99C  63267526          arpl [0x2675],sp
0000C9A0  CA3FEF            retf word 0xef3f
0000C9A3  2304              and ax,[si]
0000C9A5  40                inc ax
0000C9A6  06                push es
0000C9A7  40                inc ax
0000C9A8  41                inc cx
0000C9A9  40                inc ax
0000C9AA  FC                cld
0000C9AB  41                inc cx
0000C9AC  FFB642B6          push word [bp-0x49be]
0000C9B0  42                inc dx
0000C9B1  194200            sbb [bp+si+0x0],ax
0000C9B4  0200              add al,[bx+si]
0000C9B6  0002              add [bp+si],al
0000C9B8  0000              add [bx+si],al
0000C9BA  50                push ax
0000C9BB  005000            add [bx+si+0x0],dl
0000C9BE  50                push ax
0000C9BF  0000              add [bx+si],al
0000C9C1  0000              add [bx+si],al
0000C9C3  0000              add [bx+si],al
0000C9C5  0000              add [bx+si],al
0000C9C7  0000              add [bx+si],al
0000C9C9  0000              add [bx+si],al
0000C9CB  0000              add [bx+si],al
0000C9CD  0000              add [bx+si],al
0000C9CF  0000              add [bx+si],al
0000C9D1  0000              add [bx+si],al
0000C9D3  0000              add [bx+si],al
0000C9D5  0000              add [bx+si],al
0000C9D7  0000              add [bx+si],al
0000C9D9  0000              add [bx+si],al
0000C9DB  0000              add [bx+si],al
0000C9DD  90                nop
0000C9DE  0000              add [bx+si],al
0000C9E0  0000              add [bx+si],al
0000C9E2  0000              add [bx+si],al
0000C9E4  0000              add [bx+si],al
0000C9E6  0000              add [bx+si],al
0000C9E8  0000              add [bx+si],al
0000C9EA  0000              add [bx+si],al
0000C9EC  0000              add [bx+si],al
0000C9EE  00FC              add ah,bh
0000C9F0  FFFF              udw
0000C9F2  FFFF              udw
0000C9F4  FFFF              udw
0000C9F6  FE4300            inc byte [bp+di+0x0]
0000C9F9  0000              add [bx+si],al
0000C9FB  0000              add [bx+si],al
0000C9FD  3213              xor dl,[bp+di]
0000C9FF  00BB29D6          add [bp+di-0x29d7],bh
0000CA03  29BB299A          sub [bp+di-0x65d7],di
0000CA07  299A299A          sub [bp+si-0x65d7],bx
0000CA0B  2922              sub [bp+si],sp
0000CA0D  2B2C              sub bp,[si]
0000CA0F  3A3B              cmp bh,[bp+di]
0000CA11  3C3D              cmp al,0x3d
0000CA13  3E5B              ds pop bx
0000CA15  5D                pop bp
0000CA16  7C00              jl 0xca18
0000CA18  0000              add [bx+si],al
0000CA1A  803F00            cmp byte [bx],0x0
0000CA1D  00804300          add [bx+si+0x43],al
0000CA21  0000              add [bx+si],al
0000CA23  00FF              add bh,bh
0000CA25  FFFF              udw
0000CA27  FFFF              udw
0000CA29  FFFF              udw
0000CA2B  FFFF              udw
0000CA2D  FFFF              udw
0000CA2F  FF00              inc word [bx+si]
0000CA31  0000              add [bx+si],al
0000CA33  0001              add [bx+di],al
0000CA35  0102              add [bp+si],ax
0000CA37  0003              add [bp+di],al
0000CA39  0000              add [bx+si],al
0000CA3B  0000              add [bx+si],al
0000CA3D  07                pop es
0000CA3E  080A              or [bp+si],cl
0000CA40  090B              or [bp+di],cx
0000CA42  0C0D              or al,0xd
0000CA44  06                push es
0000CA45  0A01              or al,[bx+di]
0000CA47  006326            add [bp+di+0x26],ah
0000CA4A  7526              jnz 0xca72
0000CA4C  CA3FEF            retf word 0xef3f
0000CA4F  2304              and ax,[si]
0000CA51  40                inc ax
0000CA52  06                push es
0000CA53  40                inc ax
0000CA54  41                inc cx
0000CA55  40                inc ax
0000CA56  FC                cld
0000CA57  41                inc cx
0000CA58  2020              and [bx+si],ah
0000CA5A  2020              and [bx+si],ah
0000CA5C  2020              and [bx+si],ah
0000CA5E  2020              and [bx+si],ah
0000CA60  4C                dec sp
0000CA61  50                push ax
0000CA62  54                push sp
0000CA63  314C50            xor [si+0x50],cx
0000CA66  54                push sp
0000CA67  324C50            xor cl,[si+0x50]
0000CA6A  54                push sp
0000CA6B  334C50            xor cx,[si+0x50]
0000CA6E  54                push sp
0000CA6F  3401              xor al,0x1
0000CA71  0002              add [bp+si],al
0000CA73  0009              add [bx+di],cl
0000CA75  00BC17CD          add [si-0x32e9],bh
0000CA79  17                pop ss
0000CA7A  0018              add [bx+si],bl
0000CA7C  A0176D            mov al,[0x6d17]
0000CA7F  186F18            sbb [bx+0x18],ch
0000CA82  8418              test [bx+si],bl
0000CA84  EE                out dx,al
0000CA85  41                inc cx
0000CA86  0000              add [bx+si],al
0000CA88  D307              rol word [bx],cl
0000CA8A  2C00              sub al,0x0
0000CA8C  D307              rol word [bx],cl
0000CA8E  3400              xor al,0x0
0000CA90  D307              rol word [bx],cl
0000CA92  42                inc dx
0000CA93  00D3              add bl,dl
0000CA95  07                pop es
0000CA96  92                xchg ax,dx
0000CA97  00D3              add bl,dl
0000CA99  07                pop es
0000CA9A  B200              mov dl,0x0
0000CA9C  D307              rol word [bx],cl
0000CA9E  BA00D3            mov dx,0xd300
0000CAA1  07                pop es
0000CAA2  C400              les ax,word [bx+si]
0000CAA4  D307              rol word [bx],cl
0000CAA6  F000D3            lock add bl,dl
0000CAA9  07                pop es
0000CAAA  0000              add [bx+si],al
0000CAAC  4C                dec sp
0000CAAD  024C02            add cl,[si+0x2]
0000CAB0  4C                dec sp
0000CAB1  0200              add al,[bx+si]
0000CAB3  0000              add [bx+si],al
0000CAB5  0000              add [bx+si],al
0000CAB7  0024              add [si],ah
0000CAB9  00870862          add [bx+0x6208],al
0000CABD  0130              add [bx+si],si
0000CABF  0B30              or si,[bx+si]
0000CAC1  04E5              add al,0xe5
0000CAC3  07                pop es
0000CAC4  45                inc bp
0000CAC5  3658              ss pop ax
0000CAC7  02600E            add ah,[bx+si+0xe]
0000CACA  FFFF              udw
0000CACC  FFFF              udw
0000CACE  0000              add [bx+si],al
0000CAD0  0000              add [bx+si],al
0000CAD2  0000              add [bx+si],al
0000CAD4  0000              add [bx+si],al
0000CAD6  0000              add [bx+si],al
0000CAD8  0000              add [bx+si],al
0000CADA  47                inc di
0000CADB  0B00              or ax,[bx+si]
0000CADD  0000              add [bx+si],al
0000CADF  0000              add [bx+si],al
0000CAE1  0000              add [bx+si],al
0000CAE3  0000              add [bx+si],al
0000CAE5  0000              add [bx+si],al
0000CAE7  0000              add [bx+si],al
0000CAE9  0000              add [bx+si],al
0000CAEB  0000              add [bx+si],al
0000CAED  0000              add [bx+si],al
0000CAEF  0000              add [bx+si],al
0000CAF1  0000              add [bx+si],al
0000CAF3  0000              add [bx+si],al
0000CAF5  0000              add [bx+si],al
0000CAF7  0000              add [bx+si],al
0000CAF9  0000              add [bx+si],al
0000CAFB  0000              add [bx+si],al
0000CAFD  0000              add [bx+si],al
0000CAFF  0000              add [bx+si],al
0000CB01  0000              add [bx+si],al
0000CB03  0000              add [bx+si],al
0000CB05  0000              add [bx+si],al
0000CB07  0000              add [bx+si],al
0000CB09  0000              add [bx+si],al
0000CB0B  0000              add [bx+si],al
0000CB0D  0000              add [bx+si],al
0000CB0F  0000              add [bx+si],al
0000CB11  0000              add [bx+si],al
0000CB13  0000              add [bx+si],al
0000CB15  0000              add [bx+si],al
0000CB17  0000              add [bx+si],al
0000CB19  0000              add [bx+si],al
0000CB1B  0000              add [bx+si],al
0000CB1D  0000              add [bx+si],al
0000CB1F  0000              add [bx+si],al
0000CB21  0000              add [bx+si],al
0000CB23  0000              add [bx+si],al
0000CB25  0000              add [bx+si],al
0000CB27  00680C            add [bx+si+0xc],ch
0000CB2A  3B435F            cmp ax,[bp+di+0x5f]
0000CB2D  46                inc si
0000CB2E  49                dec cx
0000CB2F  4C                dec sp
0000CB30  45                inc bp
0000CB31  5F                pop di
0000CB32  49                dec cx
0000CB33  4E                dec si
0000CB34  46                inc si
0000CB35  4F                dec di
0000CB36  0000              add [bx+si],al
0000CB38  0000              add [bx+si],al
0000CB3A  0000              add [bx+si],al
0000CB3C  0000              add [bx+si],al
0000CB3E  0000              add [bx+si],al
0000CB40  0000              add [bx+si],al
0000CB42  0000              add [bx+si],al
0000CB44  0000              add [bx+si],al
0000CB46  0000              add [bx+si],al
0000CB48  0000              add [bx+si],al
0000CB4A  0000              add [bx+si],al
0000CB4C  0000              add [bx+si],al
0000CB4E  0000              add [bx+si],al
0000CB50  1400              adc al,0x0
0000CB52  818181010100      add word [bx+di+0x181],0x1
0000CB58  0000              add [bx+si],al
0000CB5A  0000              add [bx+si],al
0000CB5C  0000              add [bx+si],al
0000CB5E  0000              add [bx+si],al
0000CB60  0000              add [bx+si],al
0000CB62  0000              add [bx+si],al
0000CB64  0000              add [bx+si],al
0000CB66  0000              add [bx+si],al
0000CB68  0000              add [bx+si],al
0000CB6A  0000              add [bx+si],al
0000CB6C  0000              add [bx+si],al
0000CB6E  020D              add cl,[di]
0000CB70  47                inc di
0000CB71  0B4300            or ax,[bp+di+0x0]
0000CB74  0000              add [bx+si],al
0000CB76  0000              add [bx+si],al
0000CB78  0000              add [bx+si],al
0000CB7A  0000              add [bx+si],al
0000CB7C  0000              add [bx+si],al
0000CB7E  0000              add [bx+si],al
0000CB80  60                pusha
0000CB81  0DE10D            or ax,0xde1
0000CB84  E405              in al,byte 0x5
0000CB86  54                push sp
0000CB87  005A00            add [bp+si+0x0],bl
0000CB8A  0000              add [bx+si],al
0000CB8C  0000              add [bx+si],al
0000CB8E  0000              add [bx+si],al
0000CB90  0000              add [bx+si],al
0000CB92  0000              add [bx+si],al
0000CB94  0000              add [bx+si],al
0000CB96  0000              add [bx+si],al
0000CB98  0000              add [bx+si],al
0000CB9A  0000              add [bx+si],al
0000CB9C  0000              add [bx+si],al
0000CB9E  0000              add [bx+si],al
0000CBA0  0000              add [bx+si],al
0000CBA2  0000              add [bx+si],al
0000CBA4  0000              add [bx+si],al
0000CBA6  0000              add [bx+si],al
0000CBA8  0000              add [bx+si],al
0000CBAA  0000              add [bx+si],al
0000CBAC  0000              add [bx+si],al
0000CBAE  0000              add [bx+si],al
0000CBB0  0000              add [bx+si],al
0000CBB2  0000              add [bx+si],al
0000CBB4  0000              add [bx+si],al
0000CBB6  0000              add [bx+si],al
0000CBB8  0000              add [bx+si],al
0000CBBA  0000              add [bx+si],al
0000CBBC  0000              add [bx+si],al
0000CBBE  0000              add [bx+si],al
0000CBC0  0000              add [bx+si],al
0000CBC2  0000              add [bx+si],al
0000CBC4  0000              add [bx+si],al
0000CBC6  0000              add [bx+si],al
0000CBC8  0000              add [bx+si],al
0000CBCA  0000              add [bx+si],al
0000CBCC  0000              add [bx+si],al
0000CBCE  0000              add [bx+si],al
