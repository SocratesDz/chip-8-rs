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
    use crate::parser::parse_instruction;
    use crate::test_data::DATA;

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

    fn assert_instruction(input: [u8; 2], output: Instruction) {
        let instruction = parse_instruction(input);

        assert_eq!(instruction, output);
    }

    #[test]
    fn read_sys_instruction() {
        assert_instruction([0x07, 0x23], Instruction::Sys(0x723))
    }

    #[test]
    fn read_clear_screen_instruction() {
        assert_instruction([0x00, 0xE0], Instruction::ClearScreen)
    }

    #[test]
    fn read_return_instruction() {
        assert_instruction([0x00, 0xEE], Instruction::Return)
    }

    #[test]
    fn read_jump_instruction() {
        assert_instruction([0x14, 0x20], Instruction::Jump(0x420))
    }

    #[test]
    fn read_call_instruction() {
        assert_instruction([0x29, 0xA2], Instruction::Call(0x9A2))
    }

    #[test]
    fn read_skip_if_equal_instruction() {
        assert_instruction([0x38, 0x58], Instruction::SkipIfEqual(0x8, 0x58))
    }

    #[test]
    fn read_skip_if_not_equal_instruction() {
        assert_instruction([0x42, 0x13], Instruction::SkipIfNotEqual(0x2, 0x13))
    }

    #[test]
    fn read_skip_if_equal_with_register_instruction() {
        assert_instruction([0x53, 0x40], Instruction::SkipIfEqualReg(0x3, 0x4))
    }

    #[test]
    fn read_set_instruction() {
        assert_instruction([0x60, 0x20], Instruction::Set(0x0, 0x20))
    }

    #[test]
    fn read_add_instruction() {
        assert_instruction([0x71, 0x42], Instruction::Add(0x1, 0x42))
    }

    #[test]
    fn read_set_with_register_instruction() {
        assert_instruction([0x85, 0x70], Instruction::SetReg(0x5, 0x7))
    }

    #[test]
    fn read_or_instruction() {
        assert_instruction([0x86, 0x31], Instruction::Or(0x6, 0x3))
    }

    #[test]
    fn read_and_instruction() {
        assert_instruction([0x8C, 0xB2], Instruction::And(0xC, 0xB))
    }

    #[test]
    fn read_xor_instruction() {
        assert_instruction([0x8D, 0x53], Instruction::Xor(0xD, 0x5))
    }

    #[test]
    fn read_add_with_register_instruction() {
        assert_instruction([0x89, 0x14], Instruction::AddReg(0x9, 0x1))
    }

    #[test]
    fn read_sub_with_register_instruction() {
        assert_instruction([0x8A, 0x25], Instruction::SubReg(0xA, 0x2))
    }

    #[test]
    fn read_shr_instruction() {
        assert_instruction([0x88, 0x76], Instruction::ShiftRight(0x8, Some(0x7)))
    }

    #[test]
    fn read_sub_inverted_instruction() {
        assert_instruction([0x80, 0x97], Instruction::SubN(0x0, 0x9))
    }

    #[test]
    fn read_shl_instruction() {
        assert_instruction([0x82, 0x2E], Instruction::ShiftLeft(0x2, Some(0x2)))
    }

    #[test]
    fn read_skip_if_not_equal_with_register_instruction() {
        assert_instruction([0x91, 0x00], Instruction::SkipIfNotEqualReg(0x1, 0x0))
    }

    #[test]
    fn read_set_i_instruction() {
        assert_instruction([0xA2, 0x77], Instruction::SetI(0x277))
    }

    #[test]
    fn read_jump_to_plus_v0() {
        assert_instruction([0xB3, 0x51], Instruction::JumpToPlusV0(0x351))
    }

    #[test]
    fn read_set_random_instruction() {
        assert_instruction([0xC9, 0x88], Instruction::SetRandom(0x9, 0x88))
    }

    #[test]
    fn read_display_instruction() {
        assert_instruction([0xD0, 0xCA], Instruction::Display(0x0, 0xC, 0xA))
    }

    #[test]
    fn read_skip_if_key_pressed_instruction() {
        assert_instruction([0xE7, 0x9E], Instruction::SkipIfKeyPressed(0x7))
    }

    #[test]
    fn read_skip_if_key_not_pressed_instruction() {
        assert_instruction([0xE3, 0xA1], Instruction::SkipIfKeyNotPressed(0x3))
    }

    #[test]
    fn read_set_delay_timer_instruction() {
        assert_instruction([0xF1, 0x07], Instruction::SetDelayTimer(0x1))
    }

    #[test]
    fn read_wait_for_key_instruction() {
        assert_instruction([0xF8, 0x0A], Instruction::WaitForKey(0x8))
    }

    #[test]
    fn read_set_delay_timer_with_register_instruction() {
        assert_instruction([0xF6, 0x15], Instruction::SetDelayTimerReg(0x6))
    }

    #[test]
    fn read_set_sound_timer_with_register_instruction() {
        assert_instruction([0xF6, 0x18], Instruction::SetSoundTimerReg(0x6))
    }

    #[test]
    fn read_add_to_i_instruction() {
        assert_instruction([0xF2, 0x1E], Instruction::AddToI(0x2))
    }

    #[test]
    fn read_set_sprite_location_instruction() {
        assert_instruction([0xF9, 0x29], Instruction::SetSpriteLocation(0x9))
    }

    #[test]
    fn read_store_bcd_instruction() {
        assert_instruction([0xF5, 0x33], Instruction::StoreBCD(0x5))
    }

    #[test]
    fn read_store_register_range_instruction() {
        assert_instruction([0xF1, 0x55], Instruction::StoreRegRange(0x1))
    }

    #[test]
    fn read_load_register_range_instruction() {
        assert_instruction([0xFD, 0x65], Instruction::LoadRegRange(0xD))
    }
}
