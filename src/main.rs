use std::fs;

use parser::parse_instruction;

use crate::{instructions::Instruction, parser::ParseInstructionError};

mod context;
mod instructions;
mod parser;

fn main() {
    let ch8_file = fs::read("assets/ibm_logo.ch8");
    if let Ok(bytes) = ch8_file {
        let chunks = bytes.chunks(2);
        let instructions: Vec<Result<Instruction, ParseInstructionError>> = chunks
            .map(|word| parse_instruction([word[0], word[1]]))
            .collect();
        dbg!(instructions);
    }
}
