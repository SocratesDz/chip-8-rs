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

    fn read_instruction(source: [u8; 2]) -> Instruction {
        match source {
            [0x00u8, 0xEEu8] => Instruction::ClearScreen,
            [upper, lower] if upper & 0x60 == 0x60 => Instruction::Set(upper ^ 0x60, lower),
            _ => Instruction::Sys(0x00u8)
        }
    }
}
