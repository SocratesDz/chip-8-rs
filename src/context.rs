use std::default;

use crate::{instructions::Instruction, parser::parse_instruction};

#[derive(Default)]
pub struct Context {
    pub registers: [u8; 16],
    pub i_register: u16,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub program_counter: u8,
    pub stack_pointer: Vec<u8>,
    pub data: Vec<u8>,
    pub keyboard_input: Option<u8>,
}

impl Context {
    fn init(data: &[u8]) -> Context {
        Context {
            data: Vec::from(data),
            ..Default::default()
        }
    }

    fn step(&mut self) -> Instruction {
        let bytes: [u8; 2] = [
            self.data[self.program_counter as usize],
            self.data[(self.program_counter + 1) as usize],
        ];
        let instruction = parse_instruction(bytes);

        match instruction {
            Instruction::ClearScreen => {
                // Send clear screen command
            }
            Instruction::Sys(_) => {
                // This is suppossed to set the program counter to a machine code routine. This can be ignored.
            }
            Instruction::Return => {
                // Set the program counter to the address at the top of the SP
            }
            Instruction::Jump(_) => {
                // Set the program counter to a new address
            }
            Instruction::Call(_) => {
                // Increments the stack pointer, put the the current PC at the top of the stack. PC is set to address.
            }
            Instruction::SkipIfEqual(_, _) => {
                // if Vx == kk { increment PC twice }
            }
            Instruction::SkipIfNotEqual(_, _) => {
                // if Vx != kk { increment PC twice }
            }
            Instruction::SkipIfEqualReg(_, _) => {
                // if Vx == Vy { increment PC twice }
            }
            Instruction::Set(_, _) => {
                // Vx = kk
            }
            Instruction::Add(_, _) => {
                // Vx = Vx + kk
            }
            Instruction::SetReg(_, _) => {
                // Vx = Vy
            }
            Instruction::Or(_, _) => {
                // Vx = Vx | Vy
            }
            Instruction::And(_, _) => {
                // Vx = Vx & Vy
            }
            Instruction::Xor(_, _) => {
                // Vx = Vx ^ Vy
            }
            Instruction::AddReg(_, _) => {
                // let sum = Vx + Vy
                // if sum > 0xFF { VF = 0x1 }
                // Vx = (sum << 4) >> 4
            }
            Instruction::SubReg(_, _) => {
                // let sub = Vx - Vy
                // VF = Vx > Vy
                // Vx = sub
            }
            Instruction::ShiftRight(_, _) => {
                // Vx = Vx << (Vy or 1)
                // VF = ?
            }
            Instruction::SubN(_, _) => {
                // Vx = Vy - Vx
                // VF = Vy > Vx
            }
            Instruction::ShiftLeft(_, _) => {
                // Vx = Vx >> (Vy or 1)
                // VF = ?
            }
            Instruction::SkipIfNotEqualReg(_, _) => {
                // if Vx != Vy { increment PC twice }
            }
            Instruction::SetI(address) => {
                self.i_register = address;
            }
            Instruction::JumpToPlusV0(_) => {
                // PC set to nnn + V0
            }
            Instruction::SetRandom(_, _) => {
                // Vx = random | kk
            }
            Instruction::Display(_, _, _) => {
                // let sprites = [I..n]
                // Send Draw(sprites, x, y)
                // - values from the Draw command should be XORed on the existing screen
                // - if any pixel is erased, VF = 1 else 0
                // - use modulo for the coordinates of the display
            }
            Instruction::SkipIfKeyPressed(_) => {
                // - if keyboard_input == x { increment PC twice }
            }
            Instruction::SkipIfKeyNotPressed(_) => {
                // - if keyboard_input != x { increment PC twice }
            }
            Instruction::SetDelayTimer(_) => {
                // Vx = delay_timer
            }
            Instruction::WaitForKey(_) => {
                // Stops execution. Wait for key press
                // Vx = key
            }
            Instruction::SetDelayTimerReg(_) => {
                // delay_timer = Vx
            }
            Instruction::SetSoundTimerReg(_) => {
                // sound_timer = Vx
            }
            Instruction::AddToI(_) => {
                // i_register = i_register + Vx
            }
            Instruction::SetSpriteLocation(_) => {
                // i_register = sprite_location[Vx]
            }
            Instruction::StoreBCD(_) => {
                // let bcd = BCD(Vx)
                // memory_map[i_register] = BCD.0
                // memory_map[i_register+1] = BCD.1
                // memory_map[i_register+2] = BCD.2
            }
            Instruction::StoreRegRange(_) => {
                // for i in 0..=x {
                //     memory_map[i_register + i] = i_register + i
                // }
            }
            Instruction::LoadRegRange(_) => {
                // for i in 0..=x {
                //     V[i] = memory_map[i_register + i]
                // }
            }
            Instruction::Data(_) => todo!(),
        }

        self.program_counter += 2;

        return instruction;
    }
}

#[cfg(test)]
mod test {

    use super::Context;

    #[test]
    fn set_i_register_in_context() {
        let test_data = [0x00, 0xE0, 0xA0, 0x12, 0x00, 0xE0];
        let mut context = Context::init(&test_data);
        context.step();
        context.step();

        assert_eq!(context.i_register, 0x012);
    }
}
