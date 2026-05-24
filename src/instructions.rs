

pub type Register = u8;
pub type Address = u16;
pub type Value = u8;




#[derive(Debug, PartialEq)]
pub enum Instruction {
    ClearScreen,                                        // 00E0
    Jump(Address),                                      // 1NNN
    SetRegister {reg: Register, val: Value },               // 6XNN
    AddValue { reg: Register, val: Value },                // 7XNN
    SetI(Address),                                      // ANNN
    Display { vx: Register, vy: Register, val: Value },    // DXYN
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
                _ => Instruction::Unknown(nnn)
            },
            0x1 => Instruction::Jump(nnn),
            0x6 => Instruction::SetRegister { reg: nibble2, val: kk },
            0x7 => Instruction::AddValue { reg: nibble2, val: kk },
            0xA => Instruction::SetI(nnn),
            0xD => Instruction::Display { vx: nibble2, vy: nibble3, val: nibble4 },
           _ => Instruction::Unknown(nnn)

        }


    }
}
