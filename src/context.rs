#[cfg(test)]
mod test {

    use crate::instructions::Instruction;

    #[test]
    fn read_sys_instruction() {
        assert_instruction([0x01, 0x23], Instruction::Sys(0x123))
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

    fn assert_instruction(input: [u8; 2], output: Instruction) {
        let instruction = read_instruction(input);

        assert_eq!(instruction, output);
    }

    fn read_instruction(source: [u8; 2]) -> Instruction {
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
            [upper, lower] if upper >> 4 == 0x5 && lower << 4 == 0 => {
                Instruction::SkipIfEqualReg(upper & 0xF, lower >> 4)
            }
            [upper, lower] if upper >> 4 == 0x6 => Instruction::Set(upper & 0xF, lower),
            [upper, lower] if upper >> 4 == 0x7 => Instruction::Add(upper & 0xF, lower),
            _ => Instruction::Sys(0x00u16), // TODO: Return Error instead
        }
    }
}
