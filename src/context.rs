#[cfg(test)]
mod test {

    use crate::instructions::Instruction;

    #[test]
    fn read_clear_screen_instruction() {
        let source = [0x00u8, 0xEEu8];

        let instruction = read_instruction(source);

        assert!(instruction == Instruction::ClearScreen);
    }

    #[test]
    fn read_set_instruction() {
        let source = [0x60u8, 0x20u8];

        let instruction = read_instruction(source);

        assert!(instruction == Instruction::Set(0x0, 0x20));
    }

    #[test]
    fn read_sys_instruction() {
        let source = [0x01u8, 0x23u8];

        let instruction = read_instruction(source);

        assert!(instruction == Instruction::Sys(0x123u16))
    }

    fn read_instruction(source: [u8; 2]) -> Instruction {
        match source {
            [upper, _] if upper & 0x0 == 0x0 => Instruction::Sys(u16::from_be_bytes(source)),
            [0x00u8, 0xEEu8] => Instruction::ClearScreen,
            [upper, lower] if upper & 0x60 == 0x60 => Instruction::Set(upper ^ 0x60, lower),
            _ => Instruction::Sys(0x00u16),
        }
    }
}
