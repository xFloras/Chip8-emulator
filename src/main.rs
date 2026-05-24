mod instructions;
mod screen;

use instructions::Instruction;
use screen::{Screen, SCREEN_HEIGHT, SCREEN_WIDTH};
use pixels::{Pixels, SurfaceTexture, wgpu::InstanceFlags};
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};
use std::sync::Arc;

const FONT: [u8; 80] = [
0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
0x20, 0x60, 0x20, 0x20, 0x70, // 1
0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
0x90, 0x90, 0xF0, 0x10, 0x10, // 4
0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
0xF0, 0x10, 0x20, 0x40, 0x40, // 7
0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
0xF0, 0x90, 0xF0, 0x90, 0x90, // A
0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
0xF0, 0x80, 0x80, 0x80, 0xF0, // C
0xE0, 0x90, 0x90, 0x90, 0xE0, // D
0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

struct Emulator<'a> {
    screen: Screen,
    cpu: Cpu,
    pixels: Option<Pixels<'a>>,
    window: Option<Arc<Window>>,

}
impl<'a> ApplicationHandler for Emulator<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("CHIP-8 Emulator")
            .with_inner_size(winit::dpi::LogicalSize::new(640.0, 320.0));
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let window_size = window.inner_size();

        self.window = Some(window);
        let window_ptr = Arc::as_ptr(self.window.as_ref().unwrap());
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, unsafe {&*window_ptr});
        let pixels = Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap();

        self.pixels = Some(pixels);
    }
    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        let raw_opcode = self.cpu.read_opcode();
        let instruction = Instruction::decode(raw_opcode);
        self.cpu.execute(instruction, &mut self.screen);

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(&mut self,
    event_loop: &winit::event_loop::ActiveEventLoop,
    _window_id: winit::window::WindowId,
    event: WindowEvent
    ) {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = &mut self.pixels {
                    self.screen.render(pixels);


                    if let Err(err) = pixels.render() {
                        eprintln!("Rendering error: {}", err);
                        event_loop.exit();
                        return;
                    }
                }
                if let Some(window) = &self.window {
                    window.request_redraw();
                }

            }
            _ => {}
        }
    }
}



struct Cpu {
    pub registers: [u8; 16], // general-purpose registers V0-VF(also used as a carry flag)
    pub pc: u16,             // program counter
    pub mem: [u8; 4096],     // 4kB of memory
    pub stack: Vec<u16>,     // stack for CALL/RET
    pub index: u16,          // I register

}

impl Cpu {
    pub fn new() -> Self {
        Self {
            registers: [0u8; 16],
            pc: 0x200,
            mem: [0u8; 4096],
            stack: Vec::new(),
            index: 0x0
        }
    }

    pub fn execute(&mut self, instr: Instruction, screen: &mut Screen) {
        match instr {
            Instruction::ClearScreen => screen.clear(),
            Instruction::Jump(address) => self.pc = address,
            Instruction::SetRegister { reg, val } => self.registers[reg as usize] = val,
            Instruction::AddValue { reg, val } => self.registers[reg as usize] += val,
            Instruction::SetI(address) => self.index = address,
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



            },
            Instruction::Unknown(val) => println!("Unknown instruction {val}"),
        }
    }

    pub fn load_rom(&mut self, data: &[u8]) {
        for (i, &byte) in data.iter().enumerate(){
            self.mem[0x200 + i] = byte;
        }
    }

    pub fn read_opcode(&mut self) -> u16 {
        let low_byte: u16 = self.mem[(self.pc+1) as usize] as u16;
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

fn main() {

    let mut cpu = Cpu::new();

    cpu.load_font(&FONT);
    let rom = std::fs::read("IBM Logo.ch8").expect("Failed to read file.");
    cpu.load_rom(&rom);

    let mut app = Emulator {
        screen: Screen::new(),
        cpu,
        pixels: None,
        window: None,
    };

        let event_loop = winit::event_loop::EventLoop::new().unwrap();
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        event_loop.run_app(&mut app).unwrap();

}
