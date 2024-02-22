#[derive(PartialEq)]
pub enum Instruction {
    // 0nnn SYS addr
    Sys(u8),

    // 00E0 CLS
    ClearScreen,

    // 00EE RET
    Return,

    // 1nnn JP addr
    Jump(u8),

    // 2nnn CALL addr
    Call(u8),

    // 3xkk SE Vx, byte
    SkipIfEqual(u8, u8),

    // 4xkk SNE Vx, byte
    SkipIfNotEqual(u8, u8),

    // 5xy0 SE Vx, Vy
    SkipIfEqualReg(u8, u8),

    // 6xkk LD Vx, byte
    Set(u8, u8),

    // 7xkk ADD Vx, byte
    Add(u8, u8),

    // 8xy0 LD Vx, Vy
    SetReg(u8, u8),

    // 8xy1 OR Vx, Vy
    Or(u8, u8),

    // 8xy2 AND Vx, Vy
    And(u8, u8),

    // 8xy3 XOR Vx, Vy
    Xor(u8, u8),

    // 8xy4 ADD Vx, Vy
    // Set VF = carry
    AddReg(u8, u8),

    // 8xy5 SUB Vx, Vy
    // Set VF = NOT borrow
    SubReg(u8, u8),

    // 8xy6 SHR Vx {, Vy}
    SHR(u8, Option<u8>),

    // 8xy7 SUBN Vx, Vy
    // Set VF = NOT borrow
    SubN(u8, u8),

    // 8xyE SHL Vx {, Vy}
    SHL(u8, Option<u8>),

    // 9xy0 SNE Vx, Vy
    SkipIfNotEqualReg(u8, u8),

    // Annn LD I, addr
    SetI(u8),

    // Bnnn JP V0, addr
    JumpToPlusV0(u8),

    // Cxkk RND Vx, byte
    SetRandom(u8, u8),

    // Dxyn DRW Vx, Vy, nibble
    // Set VF = collision
    Display(u8, u8, u8),

    // Ex9E SKP Vx
    SkipIfKeyPressed(u8),

    // ExA1 SKNP Vx
    SkipIfKeyNotPressed(u8),

    // Fx07 LD Vx, DT
    SetDelayTimer(u8),

    // Fx0A LD Vx, K
    WaitForKey(u8, u8),

    // Fx15 LD DT, Vx
    SetDelayTimerReg(u8, u8),

    // Fx18 LD ST, Vx
    SetSoundTimerReg(u8),

    // Fx1E ADD I, Vx
    AddToI(u8),

    // Fx29 LD F, Vx
    SetSpriteLocation(u8),

    // Fx33 LD B, Vx
    StoreBCD(u8),

    // Fx55 LD [I], Vx
    StoreRegRange(u8),

    // Fx65 LD Vx, [I]
    LoadRegRange(u8),
}
