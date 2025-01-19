use std::default;

use crate::{instructions::Instruction, parser::parse_instruction};

#[derive(Default)]
pub struct Context {
    pub registers: [u8; 16],
    pub i_register: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: u16,
    pub stack_pointer: Vec<u16>,
    pub data: Vec<u8>,
    pub keyboard_input: Option<u8>,
    pub memory_map: Vec<u8>,
    pub graphics_buffer: Vec<Vec<u8>>,
}

impl Context {
    pub fn new(data: &[u8]) -> Context {
        let mut memory = vec![0u8; 0x200];
        memory.append(&mut Vec::from(data));
        let mut graphics = vec![vec![0; 64]; 32];
        Context {
            data: Vec::from(data),
            memory_map: memory,
            program_counter: 0x200,
            graphics_buffer: graphics,
            ..Default::default()
        }
    }

    pub fn step(&mut self) -> Instruction {
        let bytes: [u8; 2] = [
            self.memory_map[self.program_counter as usize],
            self.memory_map[(self.program_counter + 1) as usize],
        ];
        let instruction = parse_instruction(bytes);

        match instruction {
            Instruction::ClearScreen => {
                // Send clear screen command
                self.increment_program_counter(1)
            }
            Instruction::Sys(_) => {
                // This is suppossed to set the program counter to a
                // machine code routine. This can be ignored.
                self.increment_program_counter(1)
            }
            Instruction::Return => {
                // Set the program counter to the address at the top of the SP
                if let Some(address) = self.stack_pointer.pop() {
                    self.program_counter = address
                }
            }
            Instruction::Jump(address) => {
                // Set the program counter to a new address
                self.program_counter = address;
            }
            Instruction::Call(address) => {
                // Increments the stack pointer, put the the current PC
                // at the top of the stack. PC is set to address.
                self.stack_pointer.push(self.program_counter);
                self.program_counter = address
            }
            Instruction::SkipIfEqual(x, value) => {
                // if Vx == kk { increment PC twice }
                if self.registers[x as usize] == value {
                    self.increment_program_counter(2);
                } else {
                    self.increment_program_counter(1)
                }
            }
            Instruction::SkipIfNotEqual(x, value) => {
                // if Vx != kk { increment PC twice }
                if self.registers[x as usize] != value {
                    self.increment_program_counter(2);
                } else {
                    self.increment_program_counter(1)
                }
            }
            Instruction::SkipIfEqualReg(x, y) => {
                // if Vx == Vy { increment PC twice }
                if self.registers[x as usize] == self.registers[y as usize] {
                    self.increment_program_counter(2);
                } else {
                    self.increment_program_counter(1)
                }
            }
            Instruction::Set(x, value) => {
                // Vx = kk
                self.registers[x as usize] = value;
                self.increment_program_counter(1)
            }
            Instruction::Add(x, value) => {
                // Vx = Vx + kk
                self.registers[x as usize] += value;
                self.increment_program_counter(1)
            }
            Instruction::SetReg(x, y) => {
                // Vx = Vy
                self.registers[x as usize] = self.registers[y as usize];
                self.increment_program_counter(1)
            }
            Instruction::Or(x, y) => {
                // Vx = Vx | Vy
                self.registers[x as usize] |= self.registers[y as usize];
                self.increment_program_counter(1)
            }
            Instruction::And(x, y) => {
                // Vx = Vx & Vy
                self.registers[x as usize] &= self.registers[y as usize];
                self.increment_program_counter(1)
            }
            Instruction::Xor(x, y) => {
                // Vx = Vx ^ Vy
                self.registers[x as usize] ^= self.registers[y as usize];
                self.increment_program_counter(1)
            }
            Instruction::AddReg(x, y) => {
                // let sum = Vx + Vy
                // if sum > 0xFF { VF = 0x1 }
                // Vx = (sum << 4) >> 4
                let sum = (self.registers[x as usize] as u16) + (self.registers[y as usize] as u16);
                if sum > 0xFF {
                    self.registers[0xF] = 0x1
                }
                self.registers[x as usize] = ((sum << 4) >> 4) as u8;
                self.increment_program_counter(1)
            }
            Instruction::SubReg(x, y) => {
                // let sub = Vx - Vy
                // VF = Vx > Vy
                // Vx = sub
            }
            Instruction::ShiftRight(x, y) => {
                // Vx = Vx << (Vy or 1)
                // VF = ?
            }
            Instruction::SubN(x, y) => {
                // Vx = Vy - Vx
                // VF = Vy > Vx
            }
            Instruction::ShiftLeft(x, y) => {
                // Vx = Vx >> (Vy or 1)
                // VF = ?
            }
            Instruction::SkipIfNotEqualReg(x, y) => {
                // if Vx != Vy { increment PC twice }
                if self.registers[x as usize] != self.registers[y as usize] {
                    self.increment_program_counter(2)
                }
                self.increment_program_counter(1)
            }
            Instruction::SetI(address) => {
                self.i_register = address;
                self.increment_program_counter(1)
            }
            Instruction::JumpToPlusV0(address) => {
                // PC set to nnn + V0
                self.program_counter = address + (self.registers[0] as u16);
                self.increment_program_counter(1)
            }
            Instruction::SetRandom(x, value) => {
                // Vx = random | kk
            }
            Instruction::Display(x, y, n) => {
                // let sprites = [I..n]
                // Send Draw(sprites, x, y)
                // - values from the Draw command should be XORed on the existing screen
                // - if any pixel is erased, VF = 1 else 0
                // - use modulo for the coordinates of the display
                let x = self.registers[x as usize];
                let y = self.registers[y as usize];
                let start = self.i_register as usize;
                let end = (self.i_register + n as u16) as usize;
                let range_i = start..end;
                let sprites = &self.memory_map[range_i];
                let mut collision = 0;

                // For each byte from sprites range
                for (index, bit) in sprites.iter().enumerate() {
                    let pixel_row = &mut self.graphics_buffer[(y as usize) + index];
                    let is_collision = proccess_graphics_row(pixel_row, x, *bit);
                    collision |= is_collision as u8;
                }
                self.registers[0xF] = collision;
                self.increment_program_counter(1);
            }
            Instruction::SkipIfKeyPressed(key) => {
                // - if keyboard_input == x { increment PC twice }
            }
            Instruction::SkipIfKeyNotPressed(key) => {
                // - if keyboard_input != x { increment PC twice }
            }
            Instruction::SetDelayTimer(x) => {
                // Vx = delay_timer
            }
            Instruction::WaitForKey(x) => {
                // Stops execution. Wait for key press
                // Vx = key
            }
            Instruction::SetDelayTimerReg(x) => {
                // delay_timer = Vx
            }
            Instruction::SetSoundTimerReg(x) => {
                // sound_timer = Vx
            }
            Instruction::AddToI(x) => {
                // i_register = i_register + Vx
            }
            Instruction::SetSpriteLocation(x) => {
                // i_register = sprite_location[Vx]
            }
            Instruction::StoreBCD(x) => {
                // let bcd = BCD(Vx)
                // memory_map[i_register] = BCD.0
                // memory_map[i_register+1] = BCD.1
                // memory_map[i_register+2] = BCD.2
            }
            Instruction::StoreRegRange(x) => {
                // for i in 0..=x {
                //     memory_map[i_register + i] = i_register + i
                // }
            }
            Instruction::LoadRegRange(x) => {
                // for i in 0..=x {
                //     V[i] = memory_map[i_register + i]
                // }
            }
            Instruction::SetI(address) => {
                self.i_register = address;
                self.increment_program_counter(1);
            }
            _ => {
                self.increment_program_counter(1);
            }
        };

        instruction
    }

    fn increment_program_counter(&mut self, times: u16) {
        self.program_counter += 2 * times;
    }
}

pub fn proccess_graphics_row(input: &mut Vec<u8>, x: u8, pixels: u8) -> bool {
    let index = (x as usize / 0xF) % input.len();
    let octet = input[index] as usize;
    let mut collision: bool;

    let result = octet ^ (pixels as usize >> x);
    input[index] = result as u8;
    collision = result != octet | (pixels as usize >> x);

    let remainder = ((pixels >> x) << x) ^ pixels;

    if remainder > 0 {
        let carry = remainder << (8 - x);

        if carry > 0 {
            let index = (index + 1) % input.len();
            let octet = input[index] as usize;
            let result = octet ^ (carry as usize);
            collision = result != octet | (carry as usize);
            input[index] = result as u8;
        }
    }

    collision
}

#[cfg(test)]
mod test {

    use super::{proccess_graphics_row, Context};

    #[test]
    fn set_i_register_in_context() {
        let test_data = [0x00, 0xE0, 0xA0, 0x12, 0x00, 0xE0];
        let mut context = Context::new(&test_data);
        context.step();
        context.step();

        assert_eq!(context.i_register, 0x012);
    }

    #[test]
    fn test_graphics_bitmasking() {
        // Screen width: 64px
        const WIDTH: usize = 8;
        const HEIGHT: usize = 5;
        let mut graphics_buffer = vec![
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
            vec![0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0, 0x0],
        ];
        let x: u8 = 0x7;
        let y: u8 = 0x0;

        let pixels = [0x81, 0x81, 0xFF, 0x81, 0x81];

        for (i, pixel) in pixels.iter().enumerate() {
            let pixel_row = &mut graphics_buffer[(y as usize) + i];
            proccess_graphics_row(pixel_row, x, *pixel);
        }
        graphics_buffer.iter().for_each(|row| {
            println!(
                "{:08b} {:08b} {:08b} {:08b} {:08b} {:08b} {:08b} {:08b}",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7]
            );
        });
    }

    #[test]
    fn test_context_display_instruction() {
        let test_data = [
            0x00, 0xE0, 0xA2, 0x06, 0xD0, 0x05, 0x81, 0x81, 0xFF, 0x81, 0x81, 0x00,
        ];
        let mut context = Context::new(&test_data);
        context.step();
        context.step();

        assert_eq!(context.i_register, 0x206);

        context.step();

        context.graphics_buffer.iter().for_each(|row| {
            println!(
                "{:08b} {:08b} {:08b} {:08b} {:08b} {:08b} {:08b} {:08b}",
                row[0], row[1], row[2], row[3], row[4], row[5], row[6], row[7]
            );
        });

        assert_eq!(context.registers[0xF], 0x0);
    }
}
