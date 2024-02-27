use crate::instructions::Instruction;

#[derive(PartialEq, Eq, Debug)]
pub struct ParseInstructionError;

pub fn parse_instruction(source: [u8; 2]) -> Instruction {
    let [high, low] = source;
    match high >> 4 {
        0x0 if low == 0xE0 => Instruction::ClearScreen,
        0x0 if low == 0xEE => Instruction::Return,
        0x0 => Instruction::Sys(u16::from_be_bytes(source)),
        0x1 => Instruction::Jump(u16::from_be_bytes([high ^ (1 << 4), low])),
        0x2 => Instruction::Call(u16::from_be_bytes([high ^ (2 << 4), low])),
        0x3 => Instruction::SkipIfEqual(high & 0xF, low),
        0x4 => Instruction::SkipIfNotEqual(high & 0xF, low),
        0x5 => Instruction::SkipIfEqualReg(high & 0xF, low >> 4),
        0x6 => Instruction::Set(high & 0xF, low),
        0x7 => Instruction::Add(high & 0xF, low),
        0x8 if low << 4 == 0x0 => Instruction::SetReg(high & 0xF, low >> 4),
        0x8 if low << 4 == 0x10 => Instruction::Or(high & 0xF, low >> 4),
        0x8 if low << 4 == 0x20 => Instruction::And(high & 0xF, low >> 4),
        0x8 if low << 4 == 0x30 => Instruction::Xor(high & 0xF, low >> 4),
        0x8 if low << 4 == 0x40 => Instruction::AddReg(high & 0xF, low >> 4),
        0x8 if low << 4 == 0x50 => Instruction::SubReg(high & 0xF, low >> 4),
        0x8 if low << 4 == 0x60 => Instruction::ShiftRight(high & 0xF, Some(low >> 4)),
        0x8 if low << 4 == 0x70 => Instruction::SubN(high & 0xF, low >> 4),
        0x8 if low << 4 == 0xE0 => Instruction::ShiftLeft(high & 0xF, Some(low >> 4)),
        0x9 if low << 4 == 0x00 => Instruction::SkipIfNotEqualReg(high & 0xF, low >> 4),
        0xA => Instruction::SetI(u16::from_be_bytes([high ^ (0xA << 4), low])),
        0xB => Instruction::JumpToPlusV0(u16::from_be_bytes([high ^ (0xB << 4), low])),
        0xC => Instruction::SetRandom(high & 0xF, low),
        0xD => Instruction::Display(high & 0xF, low >> 4, low & 0xF),
        0xE if low == 0x9E => Instruction::SkipIfKeyPressed(high & 0xF),
        0xE if low == 0xA1 => Instruction::SkipIfKeyNotPressed(high & 0xF),
        0xF if low == 0x07 => Instruction::SetDelayTimer(high & 0xF),
        0xF if low == 0x0A => Instruction::WaitForKey(high & 0xF),
        0xF if low == 0x15 => Instruction::SetDelayTimerReg(high & 0xF),
        0xF if low == 0x18 => Instruction::SetSoundTimerReg(high & 0xF),
        0xF if low == 0x1E => Instruction::AddToI(high & 0xF),
        0xF if low == 0x29 => Instruction::SetSpriteLocation(high & 0xF),
        0xF if low == 0x33 => Instruction::StoreBCD(high & 0xF),
        0xF if low == 0x55 => Instruction::StoreRegRange(high & 0xF),
        0xF if low == 0x65 => Instruction::LoadRegRange(high & 0xF),
        _ => Instruction::Data(u16::from_be_bytes(source)),
    }
}

#[cfg(test)]
mod test {
    use std::io::Read;

    use crate::instructions::Instruction;

    use super::parse_instruction;

    const DATA: [u8; 132] = [
        // Offset 0x00000000 to 0x00000083
        0x00, 0xE0, 0xA2, 0x2A, 0x60, 0x0C, 0x61, 0x08, 0xD0, 0x1F, 0x70, 0x09, 0xA2, 0x39, 0xD0,
        0x1F, 0xA2, 0x48, 0x70, 0x08, 0xD0, 0x1F, 0x70, 0x04, 0xA2, 0x57, 0xD0, 0x1F, 0x70, 0x08,
        0xA2, 0x66, 0xD0, 0x1F, 0x70, 0x08, 0xA2, 0x75, 0xD0, 0x1F, 0x12, 0x28, 0xFF, 0x00, 0xFF,
        0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0x3C, 0x00, 0xFF, 0x00, 0xFF, 0xFF, 0x00, 0xFF,
        0x00, 0x38, 0x00, 0x3F, 0x00, 0x3F, 0x00, 0x38, 0x00, 0xFF, 0x00, 0xFF, 0x80, 0x00, 0xE0,
        0x00, 0xE0, 0x00, 0x80, 0x00, 0x80, 0x00, 0xE0, 0x00, 0xE0, 0x00, 0x80, 0xF8, 0x00, 0xFC,
        0x00, 0x3E, 0x00, 0x3F, 0x00, 0x3B, 0x00, 0x39, 0x00, 0xF8, 0x00, 0xF8, 0x03, 0x00, 0x07,
        0x00, 0x0F, 0x00, 0xBF, 0x00, 0xFB, 0x00, 0xF3, 0x00, 0xE3, 0x00, 0x43, 0xE0, 0x00, 0xE0,
        0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0x80, 0x00, 0xE0, 0x00, 0xE0,
    ];

    #[test]
    fn parse_instructions() {
        let mut instructions_bytes: Vec<u8> = vec![];
        let _ = DATA.take(8).read_to_end(&mut instructions_bytes);
        let instructions = instructions_bytes
            .chunks(2)
            .map(|chunk| parse_instruction([chunk[0], chunk[1]]))
            .collect::<Vec<Instruction>>();
        assert_eq!(
            instructions,
            [
                Instruction::ClearScreen,
                Instruction::SetI(0x22A),
                Instruction::Set(0, 0x0C),
                Instruction::Set(1, 0x08)
            ]
        )
    }

    #[test]
    fn parse_instructions_file() {
        let instructions = DATA
            .chunks(2)
            .map(|chunk| parse_instruction([chunk[0], chunk[1]]))
            .collect::<Vec<Instruction>>();
        dbg!(instructions);
    }
}
