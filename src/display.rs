use softbuffer::Buffer;
use winit::dpi::PhysicalSize;

pub struct Display {
    frame: [[bool; 32]; 64],
}

impl Display {
    pub fn new() -> Display {
        Display {
            frame: [[false; 32]; 64],
        }
    }

    pub fn render(&self, size: &PhysicalSize<u32>, buffer: &mut Buffer) {
        let unit_width = size.width / 64;
        let unit_height = size.height / 32;

        for index in 0..(size.width * size.height) {
            buffer[index as usize] = 0;
            let x = index % size.width;
            let y = index / size.width;

            let unit_x = x / unit_width;
            let unit_y = y / unit_height;

            if unit_x < 64 && unit_y < 32 && self.frame[unit_x as usize][unit_y as usize] {
                buffer[index as usize] = 255 | (255 << 8) | (255 << 16);
            }
        }
    }

    pub fn clear(&mut self) {
        self.frame = [[false; 32]; 64];
    }

    pub fn display_sprite(&mut self, x: usize, y: usize, sprite: &[u8]) {
        for idx in 0..sprite.len() {
            if y + idx < 32 {
                for byte_idx in 0..8 {
                    if x + byte_idx < 64 {
                        self.frame[x + byte_idx][y + idx] =
                            ((sprite[idx] >> (7 - byte_idx)) & 1) != 0;
                    }
                }
            }
        }
    }
}
