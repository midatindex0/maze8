use raylib::prelude::*;
use std::{fs::File, io::Read};

const IPS: f32 = 50.0; // 50 instructions per second

struct Cpu {
    reg_a: u8,
    reg_b: u8,
    ram: [u8; 65536],
    flash: [u8; 65536],
    flags: u8,
    ram_ext: u8,
    pc: u16,
    halted: bool,
}

impl Cpu {
    fn new(flash: [u8; 65536]) -> Self {
        Self {
            reg_a: 0,
            reg_b: 0,
            ram: [0; 65536],
            flash: flash,
            flags: 0,
            ram_ext: 0,
            pc: 0,
            halted: false,
        }
    }

    fn load_next(&mut self) -> u8 {
        let b = self.flash[self.pc as usize];
        self.pc += 1;
        b
    }

    fn get_pixels(&self, buf: &mut [u8; 32768]) {
        buf.copy_from_slice(&self.ram[32768..65536]);
    }

    fn step(&mut self) {
        if !self.halted {
            let instruction = self.load_next();
            match instruction {
                0x00 => {} // Nop
                // ------------------------
                0x01 => {
                    println!("Output: {}", self.reg_a);
                } // output from A reg
                0x02 => {
                    println!("Output: {}", self.reg_b);
                } // output from B reg
                // --------------------------
                0x10 => {
                    self.reg_a = self.load_next();
                } // Load imediate to A reg
                0x11 => {
                    self.reg_b = self.load_next();
                } // Load imediate to B reg
                0x12 => {
                    self.ram_ext = self.load_next();
                } // Load immediate to ram_ext
                // -------------------------------
                0x20 => {
                    self.ram[((self.ram_ext as usize) << 8) | self.load_next() as usize] =
                        self.reg_a;
                }
                0x21 => {
                    self.ram[((self.ram_ext as usize) << 8) | self.load_next() as usize] =
                        self.reg_b;
                }
                0x22 => {
                    self.reg_a =
                        self.ram[((self.ram_ext as usize) << 8) | self.load_next() as usize];
                }
                0x23 => {
                    self.reg_b =
                        self.ram[((self.ram_ext as usize) << 8) | self.load_next() as usize];
                }
                0x24 => {
                    self.reg_a = self.reg_b;
                }
                0x25 => {
                    self.reg_b = self.reg_a;
                }
                0x2d => {
                    self.ram_ext = self.reg_a;
                }
                0x2e => {
                    self.ram_ext = self.reg_b;
                }
                0x2f => {
                    self.ram_ext =
                        self.ram[((self.ram_ext as usize) << 8) | self.load_next() as usize];
                }
                // -------------------------------
                0x30 => {
                    self.reg_a =
                        self.ram[((self.load_next() as usize) << 8) | self.load_next() as usize];
                }
                0x31 => {
                    self.reg_a = self.ram[((self.reg_a as usize) << 8) | self.reg_b as usize];
                }
                0x32 => {
                    self.reg_b = self.ram[((self.reg_a as usize) << 8) | self.reg_b as usize];
                }
                // -------------------------------
                0x40 => {
                    let (res, carry) = self.reg_a.overflowing_add(self.reg_b);
                    self.reg_a = res;
                    if carry {
                        self.flags = (self.flags & !1) | 1;
                        println!("[DEBUG] --- Addition overflow ---");
                    } else {
                        self.flags = self.flags & !1;
                    }
                } // Add
                0x41 => {
                    let (res, carry) = self.reg_a.overflowing_add(self.reg_b + self.flags & 0x1);
                    self.reg_a = res;
                    if carry {
                        self.flags = (self.flags & !1) | 1;
                        println!("[DEBUG] --- Addition overflow ---");
                    } else {
                        self.flags = self.flags & !1;
                    }
                } // Add with carry
                // -------------------------------
                0xf0 => {
                    self.pc = ((self.load_next() as u16) << 8) | self.load_next() as u16;
                } // Jump
                0xf1 => {
                    self.pc = self.reg_a as u16;
                }
                0xf2 => {
                    self.pc = ((self.reg_a as u16) << 8) | self.reg_b as u16;
                }
                0xf3 => {
                    if self.flags & 0x1 == 0x1 {
                        self.pc = ((self.load_next() as u16) << 8) | self.load_next() as u16;
                    } else {
                        self.load_next();
                        self.load_next();
                    }
                } // Jump if carry
                0xf4 => {
                    if self.flags & 0x1 == 0x1 {
                        self.pc = self.reg_a as u16;
                    } else {
                        self.load_next();
                        self.load_next();
                    }
                }
                0xf5 => {
                    if self.flags & 0x1 == 0x1 {
                        self.pc = ((self.reg_a as u16) << 8) | self.reg_b as u16;
                    } else {
                        self.load_next();
                        self.load_next();
                    }
                }
                0xf6 => {
                    if self.reg_a == 0 {
                        self.pc = ((self.load_next() as u16) << 8) | self.load_next() as u16;
                    } else {
                        self.load_next();
                        self.load_next();
                    }
                } // Jump if A = 0
                0xf7 => {
                    if self.reg_a == 0 {
                        self.pc = self.reg_b as u16;
                    } else {
                        self.load_next();
                        self.load_next();
                    }
                }
                // --------------------------------
                0xff => {
                    self.halted = true;
                } // Halt cpu
                _ => {
                    unimplemented!();
                }
            }
        }
    }
}

fn main() {
    let mut buf = [0u8; 65536];
    let file_name = std::env::args()
        .collect::<Vec<String>>()
        .get(1)
        .expect("No file name provided")
        .clone();

    let mut s = File::open(file_name).expect("Could not open file");

    println!(
        "Loaded {} bytes into flash",
        s.read(&mut buf).expect("Could not real file")
    );

    let mut cpu = Cpu::new(buf);
    let mut pixels = [0u8; 32768];
    let rmap: [u8; 8] = [0, 48, 80, 112, 144, 176, 208, 255];
    let gmap: [u8; 8] = [0, 48, 80, 112, 144, 176, 208, 255];
    let bmap: [u8; 4] = [0, 85, 170, 255];
    let x = 228;
    let y = 142;
    let magnifier = 4;

    let (mut rl, thread) = raylib::init()
        .size((x - 1) * magnifier, (y - 1) * magnifier)
        .title("Maze8")
        .build();

    println!("----- Running Maze8 -----");
    while !rl.window_should_close() && !cpu.halted {
        cpu.step();
        let mut d = rl.begin_drawing(&thread);
        cpu.get_pixels(&mut pixels);

        for (i, pixel) in pixels[..(x * y) as usize].iter().enumerate() {
            let r = rmap[(pixel & 0b11100000 >> 5) as usize];
            let g = gmap[(pixel & 0b00011100 >> 2) as usize];
            let b = bmap[(pixel & 0b00000011) as usize];
            d.draw_rectangle(
                (i as i32 % x) * magnifier - magnifier / 2,
                (i as i32 / x) * magnifier - magnifier / 2,
                magnifier,
                magnifier,
                Color { r, g, b, a: 255 },
            );
        }

        std::thread::sleep(std::time::Duration::from_secs_f32(1.0 / IPS));
    }
}
