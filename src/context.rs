#[cfg(test)]
mod test {
    use crate::{instructions::Instruction, parser::parse_insruction};

    fn assert_instruction(input: [u8; 2], output: Instruction) {
        let instruction = parse_insruction(input);

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
        assert_instruction([0x88, 0x76], Instruction::SHR(0x8, Some(0x7)))
    }

    #[test]
    fn read_sub_inverted_instruction() {
        assert_instruction([0x80, 0x97], Instruction::SubN(0x0, 0x9))
    }

    #[test]
    fn read_shl_instruction() {
        assert_instruction([0x82, 0x2E], Instruction::SHL(0x2, Some(0x2)))
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
