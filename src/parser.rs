use crate::instructions::Instruction;

pub fn parse_insruction(source: [u8; 2]) -> Instruction {
    match source {
        [0x00u8, 0xE0u8] => Instruction::ClearScreen,
        [0x00u8, 0xEEu8] => Instruction::Return,
        [upper, _] if upper >> 4 == 0x0 => Instruction::Sys(u16::from_be_bytes(source)),
        [upper, lower] if upper >> 4 == 0x1 => {
            Instruction::Jump(u16::from_be_bytes([upper ^ (1 << 4), lower]))
        }
        [upper, lower] if upper >> 4 == 0x2 => {
            Instruction::Call(u16::from_be_bytes([upper ^ (2 << 4), lower]))
        }
        [upper, lower] if upper >> 4 == 0x3 => Instruction::SkipIfEqual(upper & 0xF, lower),
        [upper, lower] if upper >> 4 == 0x4 => Instruction::SkipIfNotEqual(upper & 0xF, lower),
        [upper, lower] if upper >> 4 == 0x5 && lower << 4 == 0x0 => {
            Instruction::SkipIfEqualReg(upper & 0xF, lower >> 4)
        }
        [upper, lower] if upper >> 4 == 0x6 => Instruction::Set(upper & 0xF, lower),
        [upper, lower] if upper >> 4 == 0x7 => Instruction::Add(upper & 0xF, lower),
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x0 => {
            Instruction::SetReg(upper & 0xF, lower >> 4)
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x10 => {
            Instruction::Or(upper & 0xF, lower >> 4)
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x20 => {
            Instruction::And(upper & 0xF, lower >> 4)
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x30 => {
            Instruction::Xor(upper & 0xF, lower >> 4)
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x40 => {
            Instruction::AddReg(upper & 0xF, lower >> 4)
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x50 => {
            Instruction::SubReg(upper & 0xF, lower >> 4)
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x60 => {
            Instruction::SHR(upper & 0xF, Some(lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x70 => {
            Instruction::SubN(upper & 0xF, lower >> 4)
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0xE0 => {
            Instruction::SHL(upper & 0xF, Some(lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0x9 && lower << 4 == 0x00 => {
            Instruction::SkipIfNotEqualReg(upper & 0xF, lower >> 4)
        }
        [upper, lower] if upper >> 4 == 0xA => {
            Instruction::SetI(u16::from_be_bytes([upper ^ (0xA << 4), lower]))
        }
        [upper, lower] if upper >> 4 == 0xB => {
            Instruction::JumpToPlusV0(u16::from_be_bytes([upper ^ (0xB << 4), lower]))
        }
        [upper, lower] if upper >> 4 == 0xC => Instruction::SetRandom(upper & 0xF, lower),
        [upper, lower] if upper >> 4 == 0xD => {
            Instruction::Display(upper & 0xF, lower >> 4, lower & 0xF)
        }
        [upper, 0x9E] if upper >> 4 == 0xE => Instruction::SkipIfKeyPressed(upper & 0xF),
        [upper, 0xA1] if upper >> 4 == 0xE => Instruction::SkipIfKeyNotPressed(upper & 0xF),
        [upper, 0x07] if upper >> 4 == 0xF => Instruction::SetDelayTimer(upper & 0xF),
        [upper, 0x0A] if upper >> 4 == 0xF => Instruction::WaitForKey(upper & 0xF),
        [upper, 0x15] if upper >> 4 == 0xF => Instruction::SetDelayTimerReg(upper & 0xF),
        [upper, 0x18] if upper >> 4 == 0xF => Instruction::SetSoundTimerReg(upper & 0xF),
        [upper, 0x1E] if upper >> 4 == 0xF => Instruction::AddToI(upper & 0xF),
        [upper, 0x29] if upper >> 4 == 0xF => Instruction::SetSpriteLocation(upper & 0xF),
        [upper, 0x33] if upper >> 4 == 0xF => Instruction::StoreBCD(upper & 0xF),
        [upper, 0x55] if upper >> 4 == 0xF => Instruction::StoreRegRange(upper & 0xF),
        [upper, 0x65] if upper >> 4 == 0xF => Instruction::LoadRegRange(upper & 0xF),
        _ => Instruction::Sys(0x00u16), // TODO: Return Error instead
    }
}
