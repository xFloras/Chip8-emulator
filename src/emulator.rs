use crate::cpu::Cpu;
use crate::instructions::Instruction;
use crate::screen::{SCREEN_HEIGHT, SCREEN_WIDTH, Screen};
use core::time::Duration;
use pixels::{Pixels, SurfaceTexture};
use std::sync::Arc;
use std::time::Instant;
use winit::event::{ElementState, KeyEvent};
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

pub struct Emulator<'a> {
    pub screen: Screen,
    pub cpu: Cpu,
    pub pixels: Option<Pixels<'a>>,
    pub window: Option<Arc<Window>>,
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
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, unsafe {
            &*window_ptr
        });
        let pixels =
            Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap();

        self.pixels = Some(pixels);
    }
    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        /*
         * executes 7 opcodes per frame
         */
        for _ in 0..7 {
            if let Some(vx) = self.cpu.wait_for_key {
                if let Some(key) = self.cpu.keys.iter().position(|&k| k == 1) {
                    self.cpu.registers[vx as usize] = key as u8;
                    self.cpu.wait_for_key = None;
                }
                break;
            }

            let raw_opcode = self.cpu.read_opcode();
            let instruction = Instruction::decode(raw_opcode);
            self.cpu.execute(instruction, &mut self.screen);
        }
        event_loop.set_control_flow(winit::event_loop::ControlFlow::WaitUntil(
            Instant::now() + Duration::from_millis(16),
        ));

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
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
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        ..
                    },
                ..
            } => {
                let pressed = state == ElementState::Pressed;
                match key_code {
                    KeyCode::KeyX => self.cpu.keys[0x0] = pressed as u8,
                    KeyCode::KeyZ => self.cpu.keys[0xA] = pressed as u8,
                    KeyCode::KeyC => self.cpu.keys[0xB] = pressed as u8,
                    KeyCode::KeyV => self.cpu.keys[0xF] = pressed as u8,
                    KeyCode::KeyA => self.cpu.keys[0x7] = pressed as u8,
                    KeyCode::KeyS => self.cpu.keys[0x8] = pressed as u8,
                    KeyCode::KeyD => self.cpu.keys[0x9] = pressed as u8,
                    KeyCode::KeyF => self.cpu.keys[0xE] = pressed as u8,
                    KeyCode::KeyQ => self.cpu.keys[0x4] = pressed as u8,
                    KeyCode::KeyW => self.cpu.keys[0x5] = pressed as u8,
                    KeyCode::KeyE => self.cpu.keys[0x6] = pressed as u8,
                    KeyCode::KeyR => self.cpu.keys[0xD] = pressed as u8,
                    KeyCode::Digit1 => self.cpu.keys[0x1] = pressed as u8,
                    KeyCode::Digit2 => self.cpu.keys[0x2] = pressed as u8,
                    KeyCode::Digit3 => self.cpu.keys[0x3] = pressed as u8,
                    KeyCode::Digit4 => self.cpu.keys[0xC] = pressed as u8,
                    _ => {}
                }
            }
            _ => {}
        }
    }
}
