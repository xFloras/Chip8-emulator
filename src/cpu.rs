use crate::delay_timer::DelayTimer;
use crate::instructions::Instruction;
use crate::screen::{SCREEN_HEIGHT, SCREEN_WIDTH, Screen};

extern crate rand;
use rand::random;

pub struct Cpu {
    pub registers: [u8; 16], // general-purpose registers V0-VF(also used as a carry flag)
    pub pc: u16,             // program counter
    pub mem: [u8; 4096],     // 4kB of memory
    pub stack: Vec<u16>,     // stack for CALL/RET
    pub index: u16,          // I register
    pub keys: [u8; 16],
    pub delay_timer: DelayTimer,
    pub sound_timer: DelayTimer,
    pub wait_for_key: Option<u8>,
}

impl Cpu {
    pub fn new() -> Self {
        let delay_timer = DelayTimer::new();
        let sound_timer = DelayTimer::new();
        delay_timer.run();
        sound_timer.run();

        Self {
            registers: [0u8; 16],
            pc: 0x200,
            mem: [0u8; 4096],
            stack: Vec::new(),
            index: 0x0,
            keys: [0u8; 16],
            delay_timer: delay_timer,
            sound_timer: sound_timer,
            wait_for_key: None,
        }
    }

    pub fn execute(&mut self, instr: Instruction, screen: &mut Screen) {
        match instr {
            Instruction::ClearScreen => screen.clear(),
            Instruction::Jump(address) => self.pc = address,
            Instruction::Ret => self.pc = self.stack.pop().unwrap(),
            Instruction::Call(address) => {
                self.stack.push(self.pc);
                self.pc = address;
            }
            Instruction::SetRegister { reg, val } => self.registers[reg as usize] = val,
            Instruction::AddValue { reg, val } => {
                self.registers[reg as usize] = self.registers[reg as usize].wrapping_add(val)
            }
            Instruction::SetI(address) => self.index = address,
            Instruction::SkipIE { vx, val } => {
                if self.registers[vx as usize] == val {
                    self.pc += 2;
                }
            }
            Instruction::SkipNE { vx, val } => {
                if self.registers[vx as usize] != val {
                    self.pc += 2;
                }
            }
            Instruction::SkipRE { vx, vy } => {
                if self.registers[vx as usize] == self.registers[vy as usize] {
                    self.pc += 2;
                }
            }
            Instruction::Mov { vx, vy } => {
                self.registers[vx as usize] = self.registers[vy as usize]
            }
            Instruction::Or { vx, vy } => {
                self.registers[vx as usize] |= self.registers[vy as usize]
            }
            Instruction::And { vx, vy } => {
                self.registers[vx as usize] &= self.registers[vy as usize]
            }
            Instruction::Xor { vx, vy } => {
                self.registers[vx as usize] ^= self.registers[vy as usize]
            }
            Instruction::AddReg { vx, vy } => {
                let vx_val = self.registers[vx as usize];
                let vy_val = self.registers[vy as usize];
                let (result, carry) = vx_val.overflowing_add(vy_val);
                self.registers[vx as usize] = result;
                self.registers[0xF] = carry as u8;
            }
            Instruction::SubReg { vx, vy } => {
                let vx_val = self.registers[vx as usize];
                let vy_val = self.registers[vy as usize];
                let (result, borrow) = vx_val.overflowing_sub(vy_val);
                self.registers[vx as usize] = result;
                self.registers[0xF] = !borrow as u8;
            }
            Instruction::Msr { vx, vy } => {
                let vy_val = self.registers[vy as usize];
                self.registers[0xF] = vy_val & 0x1;
                self.registers[vx as usize] = vy_val >> 1;
            }
            Instruction::RevSub { vx, vy } => {
                let vx_val = self.registers[vx as usize];
                let vy_val = self.registers[vy as usize];
                let (result, borrow) = vy_val.overflowing_sub(vx_val);
                self.registers[vx as usize] = result;
                self.registers[0xF] = !borrow as u8;
            }
            Instruction::Msl { vx, vy } => {
                let vy_val = self.registers[vy as usize];
                self.registers[0xF] = (vy_val >> 7) & 0x1;
                self.registers[vx as usize] = vy_val << 1;
            }
            Instruction::SkipRN { vx, vy } => {
                if self.registers[vx as usize] != self.registers[vy as usize] {
                    self.pc += 2;
                }
            }
            Instruction::JumpAdd(nnn) => {
                self.pc = nnn + self.registers[0] as u16;
            }
            Instruction::RandMask { vx, val } => {
                let random: u8 = random();
                self.registers[vx as usize] = random & val;
            }
            Instruction::SkipIK { vx } => {
                if self.keys[self.registers[vx as usize] as usize] == 1 {
                    self.pc += 2;
                }
            }
            Instruction::SkipNK { vx } => {
                if self.keys[self.registers[vx as usize] as usize] == 0 {
                    self.pc += 2;
                }
            }
            Instruction::MovDT { vx } => self.registers[vx as usize] = self.delay_timer.get(),
            Instruction::SetDT { vx } => self.delay_timer.set(self.registers[vx as usize]),
            Instruction::SetST { vx } => self.sound_timer.set(self.registers[vx as usize]),
            Instruction::AddI { vx } => self.index += self.registers[vx as usize] as u16,
            Instruction::SetIR { vx } => {
                self.index = (self.registers[vx as usize] * 5) as u16 + 0x050
            }
            Instruction::BCD { vx } => {
                let mut number = self.registers[vx as usize];
                for i in (0..3).rev() {
                    self.mem[self.index as usize + i as usize] = number % 10;
                    number /= 10;
                }
            }
            Instruction::StoreMem { vx } => {
                for i in 0..=vx {
                    self.mem[self.index as usize + i as usize] = self.registers[i as usize];
                }
                self.index += vx as u16 + 1;
            }
            Instruction::FillReg { vx } => {
                for i in 0..=vx {
                    self.registers[i as usize] = self.mem[self.index as usize + i as usize];
                }
                self.index += vx as u16 + 1;
            }
            Instruction::WaitK { vx } => {
                self.wait_for_key = Some(vx);
                self.pc -= 2;
            }
            Instruction::Display { vx, vy, val } => {
                let start_x = self.registers[vx as usize] as usize % SCREEN_WIDTH;
                let start_y = self.registers[vy as usize] as usize % SCREEN_HEIGHT;
                for row in 0..val {
                    let sprite_byte = self.mem[self.index as usize + row as usize];

                    for col in 0..8 {
                        let mask = 0x80 >> col;
                        if (sprite_byte & mask) != 0 {
                            let x = (start_x + col) % SCREEN_WIDTH;
                            let y = (start_y + row as usize) % SCREEN_HEIGHT;
                            let before = screen.grid[y * SCREEN_WIDTH + x];
                            screen.set_pixel(x, y, 1);
                            let after = screen.grid[y * SCREEN_WIDTH + x];
                            self.registers[0xF] = if before == 1 && after == 0 { 1 } else { 0 };
                        }
                    }
                }
            }
            Instruction::Unknown(val) => println!("Unknown instruction {val}"),
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate() {
            self.mem[0x200 + i] = byte;
        }
    }

    pub fn read_opcode(&mut self) -> u16 {
        let low_byte: u16 = self.mem[(self.pc + 1) as usize] as u16;
        let high_byte: u16 = self.mem[self.pc as usize] as u16;
        let opcode: u16 = (high_byte << 8) | low_byte;
        self.pc += 2;
        opcode
    }
    /*
     * Loads font from address 0x050 to 0x09F
     */
    pub fn load_font(&mut self, font: &[u8; 80]) {
        for (i, &sprite) in font.iter().enumerate() {
            self.mem[0x050 + i] = sprite;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cpu() -> Cpu {
        let mut cpu = Cpu::new();
        cpu.load_font(&FONT);
        cpu
    }

    #[test]
    fn test_set_register() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        cpu.execute(Instruction::SetRegister { reg: 0, val: 42 }, &mut screen);
        assert_eq!(cpu.registers[0], 42);
    }

    #[test]
    fn test_add_value() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        cpu.registers[0] = 10;
        cpu.execute(Instruction::AddValue { reg: 0, val: 5 }, &mut screen);
        assert_eq!(cpu.registers[0], 15);
    }

    #[test]
    fn test_add_reg_carry() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        cpu.registers[0] = 200;
        cpu.registers[1] = 100;
        cpu.execute(Instruction::AddReg { vx: 0, vy: 1 }, &mut screen);
        assert_eq!(cpu.registers[0], 44); // 300 wraps to 44
        assert_eq!(cpu.registers[0xF], 1); // carry set
    }

    #[test]
    fn test_add_reg_no_carry() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        cpu.registers[0] = 100;
        cpu.registers[1] = 100;
        cpu.execute(Instruction::AddReg { vx: 0, vy: 1 }, &mut screen);
        assert_eq!(cpu.registers[0], 200);
        assert_eq!(cpu.registers[0xF], 0); // no carry
    }

    #[test]
    fn test_sub_reg_borrow() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        cpu.registers[0] = 50;
        cpu.registers[1] = 100;
        cpu.execute(Instruction::SubReg { vx: 0, vy: 1 }, &mut screen);
        assert_eq!(cpu.registers[0xF], 0); // borrow occurred
    }

    #[test]
    fn test_sub_reg_no_borrow() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        cpu.registers[0] = 100;
        cpu.registers[1] = 50;
        cpu.execute(Instruction::SubReg { vx: 0, vy: 1 }, &mut screen);
        assert_eq!(cpu.registers[0], 50);
        assert_eq!(cpu.registers[0xF], 1); // no borrow
    }

    #[test]
    fn test_jump() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        cpu.execute(Instruction::Jump(0x300), &mut screen);
        assert_eq!(cpu.pc, 0x300);
    }

    #[test]
    fn test_skip_ie_equal() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        cpu.registers[0] = 5;
        cpu.execute(Instruction::SkipIE { vx: 0, val: 5 }, &mut screen);
        assert_eq!(cpu.pc, 0x202); // skipped one instruction
    }

    #[test]
    fn test_skip_ie_not_equal() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        cpu.registers[0] = 5;
        cpu.execute(Instruction::SkipIE { vx: 0, val: 9 }, &mut screen);
        assert_eq!(cpu.pc, 0x200); // did not skip
    }

    #[test]
    fn test_msr() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        cpu.registers[1] = 0b10110111;
        cpu.execute(Instruction::Msr { vx: 0, vy: 1 }, &mut screen);
        assert_eq!(cpu.registers[0], 0b01011011);
        assert_eq!(cpu.registers[0xF], 1); // LSB was 1
    }

    #[test]
    fn test_clear_screen() {
        let mut cpu = make_cpu();
        let mut screen = Screen::new();
        screen.set_pixel(5, 5, 1);
        cpu.execute(Instruction::ClearScreen, &mut screen);
        assert_eq!(screen.grid[5 * SCREEN_WIDTH + 5], 0);
    }
}
