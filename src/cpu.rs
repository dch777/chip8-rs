use crate::display;
use crate::keyboard;
use crate::memory;
use rand::{rngs::ThreadRng, Rng};

pub struct CPU {
    V: [u8; 16],
    I: u16,

    pc: u16,
    sp: u8,

    dt: u8,
    st: u8,

    rng: ThreadRng,

    pub memory: memory::Memory,
    pub display: display::Display,
    pub keyboard: keyboard::Keyboard,
}

impl CPU {
    pub fn new() -> CPU {
        CPU {
            V: [0; 16],
            I: 0,

            pc: 0x200,
            sp: 0,

            dt: 0,
            st: 0,

            rng: rand::thread_rng(),

            memory: memory::Memory::new(),
            display: display::Display::new(),
            keyboard: keyboard::Keyboard::new(),
        }
    }

    pub fn step(&mut self) {
        let inst: u16 = self.memory.ram[self.pc as usize + 1] as u16
            + ((self.memory.ram[self.pc as usize] as u16) << 8);

        let x = self.memory.ram[self.pc as usize] & 0xf;
        let y = (self.memory.ram[self.pc as usize + 1] & 0xf0) >> 4;
        let n = self.memory.ram[self.pc as usize + 1] & 0xf;
        let nn = self.memory.ram[self.pc as usize + 1];
        let nnn = self.memory.ram[self.pc as usize + 1] as u16 + ((x as u16) << 8);

        // println!("{:#06x}: {:#06x}", self.pc, inst);
        self.pc += 2;

        match inst {
            0x00E0 => self.inst_00e0(),
            0x00EE => self.inst_00ee(),
            0x1000..=0x1fff => self.inst_1nnn(nnn),
            0x2000..=0x2fff => self.inst_2nnn(nnn),
            0x3000..=0x3fff => self.inst_3xnn(x.into(), nn),
            0x4000..=0x4fff => self.inst_4xnn(x.into(), nn),
            0x5000..=0x5fff => self.inst_5xy0(x.into(), y.into()),
            0x6000..=0x6fff => self.inst_6xnn(x.into(), nn),
            0x7000..=0x7fff => self.inst_7xnn(x.into(), nn),
            0x8000..=0x8fff => {
                let lsb = inst & 0xf;
                match lsb {
                    0x0 => self.inst_8xy0(x.into(), y.into()),
                    0x1 => self.inst_8xy1(x.into(), y.into()),
                    0x2 => self.inst_8xy2(x.into(), y.into()),
                    0x3 => self.inst_8xy3(x.into(), y.into()),
                    0x4 => self.inst_8xy4(x.into(), y.into()),
                    0x5 => self.inst_8xy5(x.into(), y.into()),
                    0x6 => self.inst_8xy6(x.into(), y.into()),
                    0x7 => self.inst_8xy7(x.into(), y.into()),
                    0xe => self.inst_8xye(x.into(), y.into()),
                    _ => println!("undefined"),
                }
            }
            0x9000..=0x9ff0 => self.inst_9xy0(x.into(), y.into()),
            0xa000..=0xafff => self.inst_annn(nnn),
            0xb000..=0xbfff => self.inst_bnnn(nnn),
            0xc000..=0xcfff => self.inst_cxnn(x.into(), nn),
            0xd000..=0xdfff => self.inst_dxyn(x.into(), y.into(), n.into()),
            0xe000..=0xefff => {
                let lsb = inst & 0xff;
                match lsb {
                    0x9e => self.inst_ex9e(x.into()),
                    0xa1 => self.inst_exa1(x.into()),
                    _ => println!("undefined"),
                }
            }
            0xf000..=0xffff => {
                let lsb = inst & 0xff;
                match lsb {
                    0x7 => self.inst_fx07(x.into()),
                    0xa => self.inst_fx0a(x.into()),
                    0x15 => self.inst_fx15(x.into()),
                    0x18 => self.inst_fx18(x.into()),
                    0x1e => self.inst_fx1e(x.into()),
                    0x29 => self.inst_fx29(x.into()),
                    0x33 => self.inst_fx33(x.into()),
                    0x55 => self.inst_fx55(x.into()),
                    0x65 => self.inst_fx65(x.into()),
                    _ => println!("undefined"),
                }
            }
            _ => println!("undefined"),
        }
    }

    fn inst_00e0(&mut self) {
        self.display.clear();
    }

    fn inst_00ee(&mut self) {
        self.pc = self.memory.stack[self.sp as usize];
        if self.sp > 0 {
            self.sp -= 1;
        }
    }

    fn inst_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn inst_2nnn(&mut self, nnn: u16) {
        self.sp += 1;
        self.memory.stack[self.sp as usize] = self.pc;
        self.pc = nnn
    }

    fn inst_3xnn(&mut self, x: usize, nn: u8) {
        if self.V[x] == nn {
            self.pc += 2;
        }
    }

    fn inst_4xnn(&mut self, x: usize, nn: u8) {
        if self.V[x] != nn {
            self.pc += 2;
        }
    }

    fn inst_5xy0(&mut self, x: usize, y: usize) {
        if self.V[x] == self.V[y] {
            self.pc += 2;
        }
    }

    fn inst_6xnn(&mut self, x: usize, nn: u8) {
        self.V[x] = nn;
    }

    fn inst_7xnn(&mut self, x: usize, nn: u8) {
        self.V[x] += nn;
    }

    fn inst_8xy0(&mut self, x: usize, y: usize) {
        self.V[x] = self.V[y];
    }

    fn inst_8xy1(&mut self, x: usize, y: usize) {
        self.V[x] |= self.V[y];
    }

    fn inst_8xy2(&mut self, x: usize, y: usize) {
        self.V[x] &= self.V[y];
    }

    fn inst_8xy3(&mut self, x: usize, y: usize) {
        self.V[x] = self.V[x] ^ self.V[y];
    }

    fn inst_8xy4(&mut self, x: usize, y: usize) {
        if 255 - self.V[x] < self.V[y] {
            self.V[15] = 1;
        } else {
            self.V[15] = 0;
        }
        self.V[x] += self.V[y];
    }

    fn inst_8xy5(&mut self, x: usize, y: usize) {
        if self.V[x] > self.V[x] {
            self.V[15] = 1;
        } else {
            self.V[15] = 0;
        }
        self.V[x] = self.V[x] - self.V[y];
    }

    fn inst_8xy6(&mut self, x: usize, y: usize) {
        if self.V[x] & 1 == 1 {
            self.V[15] = 1;
        } else {
            self.V[15] = 0;
        }
        self.V[x] = self.V[x] / 2;
    }

    fn inst_8xy7(&mut self, x: usize, y: usize) {
        if self.V[y] > self.V[x] {
            self.V[15] = 1;
        } else {
            self.V[15] = 0;
        }
        self.V[x] = self.V[y] - self.V[x];
    }

    fn inst_8xye(&mut self, x: usize, y: usize) {
        if self.V[x] & 128 == 128 {
            self.V[15] = 1;
        } else {
            self.V[15] = 0;
        }
        self.V[x] *= 2;
    }

    fn inst_9xy0(&mut self, x: usize, y: usize) {
        if self.V[x] != self.V[y] {
            self.pc += 2;
        }
    }

    fn inst_annn(&mut self, nnn: u16) {
        self.I = nnn;
    }

    fn inst_bnnn(&mut self, nnn: u16) {
        self.pc = self.V[0] as u16 + nnn;
    }

    fn inst_cxnn(&mut self, x: u8, nn: u8) {
        let rand: u8 = self.rng.gen();

        self.V[x as usize] = rand & nn;
    }

    fn inst_dxyn(&mut self, x: usize, y: usize, n: usize) {
        let sprite_x = self.V[x] % 64;
        let sprite_y = self.V[y] % 32;

        let sprite = &self.memory.ram[self.I as usize..self.I as usize + n];

        self.display
            .display_sprite(sprite_x as usize, sprite_y as usize, &sprite);
    }

    fn inst_ex9e(&mut self, x: u8) {
        if self.keyboard.is_pressed(self.V[x as usize].into()) {
            self.pc += 2;
        }
    }

    fn inst_exa1(&mut self, x: u8) {
        if !self.keyboard.is_pressed(self.V[x as usize].into()) {
            self.pc += 2;
        }
    }

    fn inst_fx07(&mut self, x: u8) {
        self.V[x as usize] = self.dt;
    }

    fn inst_fx0a(&mut self, x: u8) {
        match self.keyboard.any_pressed() {
            Some(key) => self.V[x as usize] = key as u8,
            None => self.pc -= 2,
        }
    }

    fn inst_fx15(&mut self, x: u8) {
        self.dt = self.V[x as usize];
    }

    fn inst_fx18(&mut self, x: u8) {
        self.st = self.V[x as usize];
    }

    fn inst_fx1e(&mut self, x: u8) {
        self.I += self.V[x as usize] as u16;
    }

    fn inst_fx29(&mut self, x: u8) {
        self.I = self.V[x as usize] as u16 * 5;
    }

    fn inst_fx33(&mut self, x: u8) {
        self.memory.ram[self.I as usize] = (self.V[x as usize] / 100) % 10;
        self.memory.ram[self.I as usize + 1] = (self.V[x as usize] / 10) % 10;
        self.memory.ram[self.I as usize + 2] = self.V[x as usize] % 10;
    }

    fn inst_fx55(&mut self, x: u8) {
        for addr in 0..(x + 1) as usize {
            self.memory.ram[self.I as usize + addr] = self.V[addr];
        }
    }

    fn inst_fx65(&mut self, x: u8) {
        for addr in 0..(x + 1) as usize {
            self.V[addr] = self.memory.ram[self.I as usize + addr];
        }
    }
}
