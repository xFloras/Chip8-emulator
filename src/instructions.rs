

pub type Register = u8;
pub type Address = u16;
pub type Value = u8;




#[derive(Debug, PartialEq)]
pub enum Instruction {
    ClearScreen,                                        // 00E0
    Jump(Address),                                      // 1NNN
    Ret,                                                // 00EE
    SetRegister {reg: Register, val: Value },           // 6XNN
    AddValue { reg: Register, val: Value },             // 7XNN
    SetI(Address),                                      // ANNN
    Display { vx: Register, vy: Register, val: Value }, // DXYN
    Call(Address),                                      // 2NNN
    SkipIE { vx: Register, val: Value },                // 3XNN
    SkipNE { vx: Register, val: Value },                // 4XNN
    SkipRE { vx: Register, vy: Register },              // 5XY0
    Mov { vx: Register, vy: Register },                 // 8XY0
    Or { vx: Register, vy: Register },                  // 8XY1
    And { vx: Register, vy: Register },                 // 8XY2
    Xor { vx: Register, vy: Register },                 // 8XY3
    AddReg { vx: Register, vy: Register },              // 8XY4
    SubReg { vx: Register, vy: Register },              // 8XY5
    Msr { vx: Register, vy: Register },                 // 8XY6
    RevSub { vx: Register, vy: Register },              // 8XY7
    Msl { vx: Register, vy: Register },                 // 8XYE
    SkipRN { vx: Register, vy: Register },              // 9XY0
    JumpAdd(Address),                                   // BNNN
    RandMask { vx: Register, val: Value },              // CXNN
    SkipIK { vx: Register },                            // EX9E
    SkipNK { vx: Register },                            // EXA1
    MovDT { vx: Register },                             // FX07
    SetDT { vx: Register },                             // FX15
    SetST { vx: Register },                             // FX18
    AddI { vx: Register },                              // FX1E
    SetIR { vx: Register },                             // FX29
    BCD { vx: Register },                               // FX33
    StoreMem { vx: Register },                          // FX55
    FillReg { vx: Register },                           // FX65
    WaitK { vx: Register },                             // FX0A
    Unknown(u16),
}

impl Instruction {
    pub fn decode(raw_opcode: u16) -> Self {
        let nibble1: u8 = ((raw_opcode & 0xF000) >> 12) as u8;
        let nibble2: u8 = ((raw_opcode & 0x0F00) >> 8) as u8;
        let nibble3: u8 = ((raw_opcode & 0x00F0) >> 4) as u8;
        let nibble4: u8 =  (raw_opcode & 0x000F) as u8;

        let kk: Value = (raw_opcode & 0x00FF) as u8;
        let nnn: Address = (raw_opcode & 0x0FFF) as u16;

        match nibble1 {
            0x0 => match nnn {
                0x0E0 => Instruction::ClearScreen,
                0x0EE => Instruction::Ret,
                _ => Instruction::Unknown(nnn)

            },
            0x1 => Instruction::Jump(nnn),
            0x2 => Instruction::Call(nnn),
            0x3 => Instruction::SkipIE { vx: nibble2, val: kk },
            0x4 => Instruction::SkipNE { vx: nibble2, val: kk },
            0x5 => Instruction::SkipRE { vx: nibble2, vy: nibble3 },
            0x6 => Instruction::SetRegister { reg: nibble2, val: kk },
            0x7 => Instruction::AddValue { reg: nibble2, val: kk },
            0xA => Instruction::SetI(nnn),
            0xB => Instruction::JumpAdd(nnn),
            0xD => Instruction::Display { vx: nibble2, vy: nibble3, val: nibble4 },
            0xE => match nibble3 {
                0x9 => Instruction::SkipIK { vx: nibble2 },
                0xA => Instruction::SkipNK { vx: nibble2 },
                _ => Instruction::Unknown(nnn),
            },
            0xF => match kk {
                0x07 => Instruction::MovDT { vx: nibble2 },
                0x15 => Instruction::SetDT { vx: nibble2 },
                0x18 => Instruction::SetST { vx: nibble2 },
                0x1E => Instruction::AddI { vx: nibble2 },
                0x29 => Instruction::SetIR { vx: nibble2 },
                0x33 => Instruction::BCD { vx: nibble2 },
                0x55 => Instruction::StoreMem { vx: nibble2 },
                0x65 => Instruction::FillReg { vx: nibble2 },
                0x0A => Instruction::WaitK { vx: nibble2 },
                _ => Instruction::Unknown(nnn),
            }
            0x8 => match nibble4 {
                0x0 => Instruction::Mov { vx: nibble2, vy: nibble3 },
                0x1 => Instruction::Or { vx: nibble2, vy: nibble3 },
                0x2 => Instruction::And { vx: nibble2, vy: nibble3 },
                0x3 => Instruction::Xor { vx: nibble2, vy: nibble3 },
                0x4 => Instruction::AddReg { vx: nibble2, vy: nibble3 },
                0x5 => Instruction::SubReg { vx: nibble2, vy: nibble3 },
                0x6 => Instruction::Msr { vx: nibble2, vy: nibble3 },
                0x7 => Instruction::RevSub { vx: nibble2, vy: nibble3 },
                0xE => Instruction::Msl { vx: nibble2, vy: nibble3 },
                _ => Instruction::Unknown(nnn)
            }
            0x9 => Instruction::SkipRN { vx: nibble2, vy: nibble3 },
            0xC => Instruction::RandMask{ vx: nibble2, val: kk },

           _ => Instruction::Unknown(nnn),
        }


    }
}
