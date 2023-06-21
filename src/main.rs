use std::env;
use std::fs::File;
use std::num::NonZeroU32;
use std::thread::sleep;
use std::time::{Duration, Instant};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod constants;
mod cpu;
mod display;
mod keyboard;
mod memory;

fn main() {
    let mut input = WinitInputHelper::new();
    let args: Vec<_> = env::args().collect();

    let event_loop = EventLoop::new();
    let size = LogicalSize::new(constants::WIDTH as f64, constants::HEIGHT as f64);
    let window = WindowBuilder::new()
        .with_title("Chip8")
        .with_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let rom: File = File::open(&args[1]).unwrap();

    let mut chip8 = cpu::CPU::new();
    chip8.memory.load(rom);

    event_loop.run(move |event, _, control_flow| {
        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::Key1) {
                chip8.keyboard.pressed(1);
            } else if input.key_released(VirtualKeyCode::Key1) {
                chip8.keyboard.released(1);
            }

            if input.key_pressed(VirtualKeyCode::Key2) {
                chip8.keyboard.pressed(2);
            } else if input.key_released(VirtualKeyCode::Key2) {
                chip8.keyboard.released(2);
            }

            if input.key_pressed(VirtualKeyCode::Key3) {
                chip8.keyboard.pressed(3);
            } else if input.key_released(VirtualKeyCode::Key3) {
                chip8.keyboard.released(3);
            }

            if input.key_pressed(VirtualKeyCode::Key4) {
                chip8.keyboard.pressed(0xC);
            } else if input.key_released(VirtualKeyCode::Key4) {
                chip8.keyboard.released(0xC);
            }

            if input.key_pressed(VirtualKeyCode::Q) {
                chip8.keyboard.pressed(4);
            } else if input.key_released(VirtualKeyCode::Q) {
                chip8.keyboard.released(4);
            }

            if input.key_pressed(VirtualKeyCode::W) {
                chip8.keyboard.pressed(5);
            } else if input.key_released(VirtualKeyCode::W) {
                chip8.keyboard.released(5);
            }

            if input.key_pressed(VirtualKeyCode::E) {
                chip8.keyboard.pressed(6);
            } else if input.key_released(VirtualKeyCode::E) {
                chip8.keyboard.released(6);
            }

            if input.key_pressed(VirtualKeyCode::R) {
                chip8.keyboard.pressed(0xD);
            } else if input.key_released(VirtualKeyCode::R) {
                chip8.keyboard.released(0xD);
            }

            if input.key_pressed(VirtualKeyCode::A) {
                chip8.keyboard.pressed(7);
            } else if input.key_released(VirtualKeyCode::A) {
                chip8.keyboard.released(7);
            }

            if input.key_pressed(VirtualKeyCode::S) {
                chip8.keyboard.pressed(8);
            } else if input.key_released(VirtualKeyCode::S) {
                chip8.keyboard.released(8);
            }

            if input.key_pressed(VirtualKeyCode::D) {
                chip8.keyboard.pressed(9);
            } else if input.key_released(VirtualKeyCode::D) {
                chip8.keyboard.released(9);
            }

            if input.key_pressed(VirtualKeyCode::F) {
                chip8.keyboard.pressed(0xE);
            } else if input.key_released(VirtualKeyCode::F) {
                chip8.keyboard.released(0xE);
            }

            if input.key_pressed(VirtualKeyCode::Z) {
                chip8.keyboard.pressed(0xA);
            } else if input.key_released(VirtualKeyCode::Z) {
                chip8.keyboard.released(0xA);
            }

            if input.key_pressed(VirtualKeyCode::X) {
                chip8.keyboard.pressed(0);
            } else if input.key_released(VirtualKeyCode::X) {
                chip8.keyboard.released(0);
            }

            if input.key_pressed(VirtualKeyCode::C) {
                chip8.keyboard.pressed(0xB);
            } else if input.key_released(VirtualKeyCode::C) {
                chip8.keyboard.released(0xB);
            }

            if input.key_pressed(VirtualKeyCode::V) {
                chip8.keyboard.pressed(0xF);
            } else if input.key_released(VirtualKeyCode::V) {
                chip8.keyboard.released(0xF);
            }
        }

        let size = window.inner_size();
        surface
            .resize(
                NonZeroU32::new(size.width).unwrap(),
                NonZeroU32::new(size.height).unwrap(),
            )
            .unwrap();

        let mut buffer = surface.buffer_mut().unwrap();

        if let Event::MainEventsCleared = event {
            chip8.step();
            chip8.display.render(&size, &mut buffer);
            sleep(Duration::from_millis(constants::DELAY));
        }
        buffer.present().unwrap();

        // control_flow.set_wait_timeout(Duration::from_millis(constants::DELAY));
    });
}
