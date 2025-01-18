00E0 -> CLS (Clear the display)
A22A -> LD I, addr (I = 0x22A)
600C -> LD V0, 0C (V0 = 0x0C)
6108 -> LD V1, 08 (V1 = 0x08)
D01F -> DRW Vx, Vy, nibble (display(I :: 22A, V0 :: 0x0C, V1 :: 0x08, 0xF); VF = collision) ;; Check for pixel collision ;; Also pick 0xF bytes to draw
7009 -> ADD V0, 09 (V0 += 0x09) ;; V0 = 0x15
A239 -> LD I, 239 (I = 0x239)
D01F -> DRW Vx, Vy, nibble (display(I :: 239, V0 :: 0x15, V1 :: 0x08, 0xF))
A248 -> LD I, 248 (I = 0x248)
7008 -> ADD V0, 08 (V0 += 0x08) ;; V0 = 0x1D
D01F -> DRW Vx, Vy, nibble (display(I :: 248, V0 :: 0x15, V1 :: 0x08, 0xF))
7004
A257
D01F
7008
A266
D01F
7008
A275
D01F
1228
FF00
FF00
3C00
3C00
3C00
3C00
FF00
FFFF
00FF

-- Rest of data
0038003F
003F0038
00FF00FF
8000E000
E0008000
8000E000
E00080F8
00FC003E
003F003B
003900F8
00F80300
07000F00
BF00FB00
F300E300
43E000E0
00800080
00800080
00E000E0
