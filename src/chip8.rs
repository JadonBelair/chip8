use crate::{keyboard::Keyboard, display::Display};
use rand::{thread_rng, prelude::ThreadRng, Rng};

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
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

pub struct Chip8 {
    pub keyboard: Keyboard,
    pub display: Display,

    ram: [u8; 4096],

    v: [u8; 16],
    i: u16,

    dt: u8,
    st: u8,

    pc: usize,

    stack: Vec<usize>,

    rng: ThreadRng
}

impl Chip8 {
    pub fn new() -> Self {
        let mut ram = [0; 4096];

        for i in 0..FONT.len() {
            ram[i] = FONT[i];
        }

        Self {
            keyboard: Keyboard::new(),
            display: Display::new(),
            ram,
            v: [0; 16],
            i: 0,
            dt: 0,
            st: 0,
            pc: 0x200,
            stack: Vec::new(),
            rng: thread_rng()
        }
    }

    pub fn load_rom(&mut self, bytes: &Vec<u8>) {
        for i in 0..bytes.len() {
            self.ram[0x200 + i] = bytes[i];
        }
    }

    pub fn run_instruction(&mut self) {

        let hi = self.ram[self.pc] as u16;
        let low = self.ram[self.pc + 1] as u16;

        self.pc += 2;

        let instruction = (hi << 8) | low;

        let nnn = instruction & 0xFFF;
        let nn = (instruction & 0xFF) as u8;
        let n = (instruction & 0xF) as u8;

        let x = ((instruction << 4) >> 12) as usize;
        let y = ((instruction << 8) >> 12) as usize;

        match (instruction & 0xF000) >> 12 {
            0x0 => {
                match nn {
                    0xE0 => self.display.clear(),
                    0xEE => self.pc = self.stack.pop().unwrap(),
                    _ => panic!("Unrecognized 0x0nnn instruction: {:X?}", instruction)
                }
            },
            0x1 => self.pc = nnn as usize,
            0x2 => {
                self.stack.push(self.pc);
                self.pc = nnn as usize;
            },
            0x3 => if self.v[x] == nn {
                self.pc += 2;
            },
            0x4 => if self.v[x] != nn {
                self.pc += 2;
            },
            0x5 => if self.v[x] == self.v[y] {
                self.pc += 2;
            },
            0x6 => self.v[x] = nn,
            0x7 => self.v[x] = self.v[x].wrapping_add(nn),
            0x8 => {

                let vx = self.v[x];
                let vy = self.v[y];

                match n {
                    0x0 => self.v[x] = vy,
                    0x1 => self.v[x] = vx | vy,
                    0x2 => self.v[x] = vx & vy,
                    0x3 => self.v[x] = vx ^ vy,
                    0x4 => {
                        let sum = vx as u16 + vy as u16;
                        self.v[0xF] = if sum > u8::MAX as u16 {1} else {0};
                        self.v[x] = sum as u8;
                    },
                    0x5 => {
                        self.v[0xF] = if vx >= vy {1} else {0};
                        self.v[x] = vx.wrapping_sub(vy);
                    },
                    0x6 => {
                        self.v[0xF] = if vx & 1 == 1 {1} else {0};
                        self.v[x] = vx >> 1;
                    },
                    0x7 => {
                        self.v[0xF] = if vy >= vx {1} else {0};
                        self.v[x] = vy.wrapping_sub(vx);
                    },
                    0xE => {
                        self.v[0xF] = if vx & 0x80 > 0 {1} else {0};
                        self.v[x] = vx << 1;
                    },
                    _ => panic!("Unrecognized 0x8nnn instruction: {:X?}", instruction)
                }
            },
            0x9 => if self.v[x] != self.v[y] {
                self.pc += 2;
            },
            0xA => self.i = nnn,
            0xB => self.pc = (nnn.wrapping_add(self.v[0] as u16)) as usize,
            0xC => self.v[x] = self.rng.gen_range(0..=255) & nn,
            0xD => {
                let vx = self.v[x];
                let vy = self.v[y];
                let col = self.draw_sprite(vx, vy, n);

                self.v[0xF] = if col {1} else {0}; 
            },
            0xE => {
                match nn {
                    0x9E => if self.keyboard.is_down(self.v[x]) {
                        self.pc += 2;
                    },
                    0xA1 => if !self.keyboard.is_down(self.v[x]) {
                        self.pc += 2;
                    },
                    _ => panic!("Unrecognized 0xEnnn instruction: {:X?}", instruction)
                }
            },
            0xF => {
                match nn {
                    0x07 => self.v[x] = self.dt,
                    0x0A => {
                        if let Some(key) = self.keyboard.just_pressed() {
                            self.v[x] = key;
                        } else {
                            self.pc -= 2;
                        }
                    },
                    0x15 => self.dt = self.v[x],
                    0x18 => self.st = self.v[x],
                    0x1E => self.i = self.i.wrapping_add(self.v[x] as u16),
                    0x29 => self.i = (self.v[x] as u16) * 5,
                    0x33 => {
                        let hundreds: u8 = self.v[x] / 100;
                        let tens: u8 = (self.v[x] - (hundreds * 100)) / 10;
                        let ones: u8 = self.v[x] - (hundreds * 100) - (tens * 10);

                        self.ram[self.i as usize] = hundreds;
                        self.ram[self.i as usize + 1] = tens;
                        self.ram[self.i as usize + 2] = ones;

                    },
                    0x55 => for i in 0..=x {
                        self.ram[self.i as usize + i] = self.v[i];
                    },
                    0x65 => for i in 0..=x {
                        self.v[i] = self.ram[self.i as usize + i];
                    },
                    _ => panic!("Unrecognized 0xFnnn instruction: {:X?}", instruction)
                }
            },
            _ => panic!("Bad instruction: {:X?}", instruction)
        }

        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
        
    }

    pub fn get_display(&self) -> [[u8; 64]; 32] {
        return self.display.get_screen();
    }

    pub fn draw_sprite(&mut self, x: u8, y: u8, height: u8) -> bool {
        let mut col = false;

        for index in 0..height {
            let byte = self.ram[(self.i + index as u16) as usize];
            if self.display.draw_byte(byte, x.into(), (y + index).into()) {
                col = true;
            }
        }

        col
    }

    pub fn reset(&mut self) {
        self.display.clear();

        for i in (0x200)..4096 {
            self.ram[i] = 0;
        }
    
        self.v = [0; 16];
        self.i = 0;
    
        self.dt = 0;
        self.st = 0;
    
        self.pc = 0x200;
    
        self.stack.clear();
    }
}