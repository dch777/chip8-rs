use crate::display;
use crate::memory;

pub struct CPU {
    V: [u8; 16],
    I: u16,

    pc: u16,
    sp: u8,

    dt: u8,
    st: u8,

    pub memory: memory::Memory,
    pub display: display::Display,
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

            memory: memory::Memory::new(),
            display: display::Display::new(),
        }
    }

    pub fn step(&mut self) {
        let inst: u16 = self.memory.get_byte(self.pc as usize + 1) as u16
            + ((self.memory.get_byte(self.pc as usize) as u16) << 8);

        let x = self.memory.get_byte(self.pc as usize) & 0xf;
        let y = (self.memory.get_byte(self.pc as usize + 1) & 0xf0) >> 4;
        let n = self.memory.get_byte(self.pc as usize + 1) & 0xf;
        let nn = self.memory.get_byte(self.pc as usize + 1);
        let nnn = self.memory.get_byte(self.pc as usize + 1) as u16 + ((x as u16) << 8);

        self.pc += 2;

        match inst {
            0x00E0 => self.inst_00e0(),
            0x00EE => self.inst_00ee(),
            0x1000..=0x1fff => self.inst_1nnn(nnn),
            0x6000..=0x6fff => self.inst_6xnn(x.into(), nn),
            0x7000..=0x7fff => self.inst_7xnn(x.into(), nn),
            0xa000..=0xafff => self.inst_annn(nnn),
            0xd000..=0xdfff => self.inst_dxyn(x.into(), y.into(), n),
            _ => (),
        }
    }

    fn inst_00e0(&mut self) {
        self.display.clear();
    }

    fn inst_00ee(&mut self) {
        self.pc = self.sp.into();
        if self.sp > 0 {
            self.sp -= 2;
        }
    }

    fn inst_1nnn(&mut self, nnn: u16) {
        self.pc = nnn;
    }

    fn inst_6xnn(&mut self, x: usize, nn: u8) {
        self.V[x] = nn;
    }

    fn inst_7xnn(&mut self, x: usize, nn: u8) {
        self.V[x] += nn;
    }

    fn inst_annn(&mut self, nnn: u16) {
        self.I = nnn;
    }

    fn inst_dxyn(&mut self, x: usize, y: usize, n: u8) {
        let sprite_x = self.V[x] % 64;
        let sprite_y = self.V[y] % 32;

        let sprite: Vec<u8> = (0..n)
            .map(|i| self.memory.get_byte(self.I as usize + i as usize))
            .collect();

        self.display
            .display_sprite(sprite_x as usize, sprite_y as usize, &sprite);
    }
}
