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
}

impl Context {
    pub fn new(data: &[u8]) -> Context {
        let mut memory = Vec::new();
        for _ in 0..0x1FF {
            memory.push(0u8);
        } 
        memory.append(&mut Vec::from(data));
        Context {
            data: Vec::from(data),
            memory_map: memory,
            ..Default::default()
        }
    }

    pub fn step(&mut self) {
        let bytes: [u8; 2] = [
            self.memory_map[self.program_counter as usize],
            self.memory_map[(self.program_counter + 1) as usize],
        ];
        let instruction = parse_instruction(bytes);

        match instruction {
            Instruction::ClearScreen => {
                // Send clear screen command
            }
            Instruction::Sys(_) => {
                // This is suppossed to set the program counter to a 
                // machine code routine. This can be ignored.
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
                self.registers[x as usize] = value
            }
            Instruction::Add(x, value) => {
                // Vx = Vx + kk
                self.registers[x as usize] += value
            }
            Instruction::SetReg(x, y) => {
                // Vx = Vy
                self.registers[x as usize] = self.registers[y as usize]
            }
            Instruction::Or(x, y) => {
                // Vx = Vx | Vy
                self.registers[x as usize] |= self.registers[y as usize]
            }
            Instruction::And(x, y) => {
                // Vx = Vx & Vy
                self.registers[x as usize] &= self.registers[y as usize]
            }
            Instruction::Xor(x, y) => {
                // Vx = Vx ^ Vy
                self.registers[x as usize] ^= self.registers[y as usize]
            }
            Instruction::AddReg(x, y) => {
                // let sum = Vx + Vy
                // if sum > 0xFF { VF = 0x1 }
                // Vx = (sum << 4) >> 4
                let sum = (self.registers[x as usize] as u16) + (self.registers[y as usize] as u16);
                if sum > 0xFF {
                    self.registers[0xF] = 0x1
                }
                self.registers[x as usize] = ((sum << 4) >> 4) as u8
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
            }
            Instruction::SetI(address) => {
                self.i_register = address;
            }
            Instruction::JumpToPlusV0(address) => {
                // PC set to nnn + V0
                self.program_counter = address + (self.registers[0] as u16)
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
            Instruction::Data(_) => todo!(),
        }
    }

    fn increment_program_counter(&mut self, times: u16) {
        self.program_counter += 2 * times;
    }
}

#[cfg(test)]
mod test {

    use super::Context;

    #[test]
    fn set_i_register_in_context() {
        let test_data = [0x00, 0xE0, 0xA0, 0x12, 0x00, 0xE0];
        let mut context = Context::new(&test_data);
        context.step();
        context.step();

        assert_eq!(context.i_register, 0x012);
    }
}
