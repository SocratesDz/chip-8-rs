# Notes about CHIP-8 specification

**Full Chip-8 specs:** http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

### Modules of Chip-8
- Memory
- Registers
- Display
- Keyboard
- Timers

### Memory
- Max memory: 4KB (4096 bytes)
- Range: `0x000` - `0xFFF`
- Memory map:
  - `0x000` - `0x1FF`: Reserved for interpreter
  - `0x200` - `0xFFF`: Chip-8 Program / Data Space

### Registers
- 16 general 8-bit registers
- 1 Memory addresses register (`I`)
- 1 delay register (`DT`)
- 1 sound timer register (`ST`)
- 1 program counter register (`PC`)
- 1 stack pointer (8-bit) register (`SP`)

### Keyboard
- Keyboard: keys from 0-F (hex)
- Map this to other configurations depending on the implementation.

### Display
- 64x32 pixel monochrome display. 2:1 aspect ratio
- (Super Chip-48 supports 128x64 pixel)
- Sprites may be up to 15 bytes, for a possible sprite size of 8x15 pixels. Each pixel is represented by a bit.
  - e.g., this is a sprite of size 8x5 that represents a drawing of the number 0: 
  ```
  | "0"  | Binary   | Hex  |
  |------+----------+------|
  | **** | 11110000 | 0xF0 |
  | *  * | 10010000 | 0x90 |
  | *  * | 10010000 | 0x90 |
  | *  * | 10010000 | 0x90 |
  | **** | 11110000 | 0xF0 |
  ```


### Timers & sound
- `DT` is active whenever its register is non-zero. It substracts 1 from the value of DT at a rate of **60Hz**. It deactivates when it reaches 0.
- `ST` behaves like `DT`. When it's active, the Chip-8 buzzer will sound.
- The sound is only one tone. Frequency to be decided by the implementation.

### Instructions
- Chip-8 has 36 different instructions
- Super Chip-48 added 10 more instructions for a total of 46
- Every instruction is two bytes long