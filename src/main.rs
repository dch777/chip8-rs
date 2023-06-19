use std::fs::File;
use std::num::NonZeroU32;
use std::time::{Duration, Instant};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod constants;
mod cpu;
mod display;
mod memory;

fn main() {
    let mut input = WinitInputHelper::new();

    let event_loop = EventLoop::new();
    let size = LogicalSize::new(constants::WIDTH as f64, constants::HEIGHT as f64);
    let window = WindowBuilder::new()
        .with_title("Chip8")
        .with_inner_size(size)
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let rom: File = File::open("IBM Logo.ch8").unwrap();

    let mut chip8 = cpu::CPU::new();
    chip8.memory.load(rom);

    event_loop.run(move |event, _, control_flow| {
        let size = window.inner_size();
        surface
            .resize(
                NonZeroU32::new(size.width).unwrap(),
                NonZeroU32::new(size.height).unwrap(),
            )
            .unwrap();

        let mut buffer = surface.buffer_mut().unwrap();

        chip8.step();
        chip8.display.render(&size, &mut buffer);

        buffer.present().unwrap();

        if input.update(&event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        *control_flow =
            //ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(constants::DELAY));
            ControlFlow::Poll;
    });
}
