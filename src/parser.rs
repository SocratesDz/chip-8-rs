use crate::instructions::Instruction;

#[derive(PartialEq, Eq, Debug)]
pub struct ParseInstructionError;

pub fn parse_instruction(source: [u8; 2]) -> Result<Instruction, ParseInstructionError> {
    match source {
        [0x00u8, 0xE0u8] => Ok(Instruction::ClearScreen),
        [0x00u8, 0xEEu8] => Ok(Instruction::Return),
        [upper, _] if upper >> 4 == 0x0 => Ok(Instruction::Sys(u16::from_be_bytes(source))),
        [upper, lower] if upper >> 4 == 0x1 => Ok(Instruction::Jump(u16::from_be_bytes([
            upper ^ (1 << 4),
            lower,
        ]))),
        [upper, lower] if upper >> 4 == 0x2 => Ok(Instruction::Call(u16::from_be_bytes([
            upper ^ (2 << 4),
            lower,
        ]))),
        [upper, lower] if upper >> 4 == 0x3 => Ok(Instruction::SkipIfEqual(upper & 0xF, lower)),
        [upper, lower] if upper >> 4 == 0x4 => Ok(Instruction::SkipIfNotEqual(upper & 0xF, lower)),
        [upper, lower] if upper >> 4 == 0x5 && lower << 4 == 0x0 => {
            Ok(Instruction::SkipIfEqualReg(upper & 0xF, lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0x6 => Ok(Instruction::Set(upper & 0xF, lower)),
        [upper, lower] if upper >> 4 == 0x7 => Ok(Instruction::Add(upper & 0xF, lower)),
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x0 => {
            Ok(Instruction::SetReg(upper & 0xF, lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x10 => {
            Ok(Instruction::Or(upper & 0xF, lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x20 => {
            Ok(Instruction::And(upper & 0xF, lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x30 => {
            Ok(Instruction::Xor(upper & 0xF, lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x40 => {
            Ok(Instruction::AddReg(upper & 0xF, lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x50 => {
            Ok(Instruction::SubReg(upper & 0xF, lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x60 => {
            Ok(Instruction::ShiftRight(upper & 0xF, Some(lower >> 4)))
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0x70 => {
            Ok(Instruction::SubN(upper & 0xF, lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0x8 && lower << 4 == 0xE0 => {
            Ok(Instruction::ShiftLeft(upper & 0xF, Some(lower >> 4)))
        }
        [upper, lower] if upper >> 4 == 0x9 && lower << 4 == 0x00 => {
            Ok(Instruction::SkipIfNotEqualReg(upper & 0xF, lower >> 4))
        }
        [upper, lower] if upper >> 4 == 0xA => Ok(Instruction::SetI(u16::from_be_bytes([
            upper ^ (0xA << 4),
            lower,
        ]))),
        [upper, lower] if upper >> 4 == 0xB => Ok(Instruction::JumpToPlusV0(u16::from_be_bytes([
            upper ^ (0xB << 4),
            lower,
        ]))),
        [upper, lower] if upper >> 4 == 0xC => Ok(Instruction::SetRandom(upper & 0xF, lower)),
        [upper, lower] if upper >> 4 == 0xD => {
            Ok(Instruction::Display(upper & 0xF, lower >> 4, lower & 0xF))
        }
        [upper, 0x9E] if upper >> 4 == 0xE => Ok(Instruction::SkipIfKeyPressed(upper & 0xF)),
        [upper, 0xA1] if upper >> 4 == 0xE => Ok(Instruction::SkipIfKeyNotPressed(upper & 0xF)),
        [upper, 0x07] if upper >> 4 == 0xF => Ok(Instruction::SetDelayTimer(upper & 0xF)),
        [upper, 0x0A] if upper >> 4 == 0xF => Ok(Instruction::WaitForKey(upper & 0xF)),
        [upper, 0x15] if upper >> 4 == 0xF => Ok(Instruction::SetDelayTimerReg(upper & 0xF)),
        [upper, 0x18] if upper >> 4 == 0xF => Ok(Instruction::SetSoundTimerReg(upper & 0xF)),
        [upper, 0x1E] if upper >> 4 == 0xF => Ok(Instruction::AddToI(upper & 0xF)),
        [upper, 0x29] if upper >> 4 == 0xF => Ok(Instruction::SetSpriteLocation(upper & 0xF)),
        [upper, 0x33] if upper >> 4 == 0xF => Ok(Instruction::StoreBCD(upper & 0xF)),
        [upper, 0x55] if upper >> 4 == 0xF => Ok(Instruction::StoreRegRange(upper & 0xF)),
        [upper, 0x65] if upper >> 4 == 0xF => Ok(Instruction::LoadRegRange(upper & 0xF)),
        _ => Ok(Instruction::Data(u16::from_be_bytes(source))),
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
            .flat_map(|chunk| parse_instruction([chunk[0], chunk[1]]))
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
            .flat_map(|chunk| parse_instruction([chunk[0], chunk[1]]))
            .collect::<Vec<Instruction>>();
        dbg!(instructions);
    }
}
