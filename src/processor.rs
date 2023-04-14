use rand::Rng;
use crate::Screen;

pub struct Cpu {
    screen: Screen,
    memory: [u8; 4096],
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: usize,
    registers: [u8; 16],
    flag_register: bool,
    i_register: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        let screen = Screen::new();

        Cpu {
            memory: [0; 4096],
            screen: Screen::new(),
            program_counter: 0x200,
            stack: [0; 16],
            stack_pointer: 0,
            registers: [0; 16],
            flag_register: false,
            i_register: 0,
        }
    }

    pub fn edit_memory(&mut self, address: usize, data: u8) {
        self.memory[address] = data;
    }

    pub fn process_instruction(&mut self) {

        let first_byte: u8 = self.memory[(self.program_counter) as usize];
        let second_byte: u8 = self.memory[(self.program_counter + 1) as usize];

        let instruction: u16 = ((first_byte as u16) << 8) | second_byte as u16;
        
        let address: u16 = instruction & 0x0FFF;
        let nibble: u8 = (instruction & 0x000F) as u8;
        let x: usize = ((instruction & 0x0F00) >> 8) as usize;
        let y: usize = ((instruction & 0x00F0) >> 4) as usize;
        let byte: u8 = (instruction & 0x00FF) as u8;
        let opcode: u8 = ((instruction & 0xF000) >> 12) as u8; 

        match opcode {
            0x0 => {
                match (instruction & 0x00FF) as u8 {
                    0xE0 => self.clear_screen(),
                    0xEE => self.return_from_subroutine(),
                    0x00 => (), //Do nothing
                    _ => panic!("oopsy woopsy"),
                }
            }
            0x1 => self.jump(address),
            0x2 => self.call_subroutine(address),
            0x3 => self.skip_equal(x, byte),
            0x4 => self.skip_not_equal(x, byte),
            0x5 => self.skip_equal_register(x, y as usize),
            0x6 => self.load(x, byte),
            0x7 => self.add(x, byte),
            0x8 => {
                //arithmetic operations
                match nibble {
                    0x0 => self.load_register(x, y),
                    0x1 => self.or(x, y),
                    0x2 => self.and(x, y),
                    0x3 => self.xor(x, y),
                    0x4 => self.add_register(x, y),
                    0x5 => self.subtract_first(x, y),
                    0x6 => self.shift_right(x),
                    0x7 => self.subtract_second(x, y),
                    0xE => self.shift_left(x),
                    _ => panic!("oopsy woopsy"),
                }
            }
            0x9 => self.skip_not_equal_register(x, y),
            0xA => self.load_i(address),
            0xB => self.jump_offset(address),
            0xC => self.random(x, byte),
            0xD => self.draw(x, y, nibble),
            //more to come
            0xF => {
                match byte {
                    0x1E => self.add_i(x),
                    _ => panic!("oopsy woopsy"),
                }
            }
            _ => panic!("oopsy woopsy"),
        }
        self.program_counter += 2;
    }

    fn clear_screen(&mut self) {
        self.screen.clear_screen();
    }

    fn return_from_subroutine(&mut self) {
        self.stack_pointer -= 1;
        self.program_counter = self.stack[self.stack_pointer];
    }

    fn jump(&mut self, address: u16) {
        self.program_counter = address;
    }

    fn call_subroutine(&mut self, address: u16) {
        self.stack[self.stack_pointer] = self.program_counter;
        self.program_counter = address;
        self.stack_pointer += 1;
    }

    fn skip_equal(&mut self, register: usize, value: u8) {
        if self.registers[register] == value {
            self.program_counter += 2;
        }
    }

    fn skip_not_equal(&mut self, register: usize, value: u8) {
        if self.registers[register] != value {
            self.program_counter += 2;
        }
    }

    fn skip_equal_register(&mut self, first_register: usize, second_register: usize) {
        if self.registers[first_register] == self.registers[second_register] {
            self.program_counter += 2;
        }
    }

    fn load(&mut self, register: usize, value: u8) {
        self.registers[register] = value;
    }

    fn add(&mut self, register: usize, value: u8) {
        self.registers[register] = self.registers[register] + value;
    }

    fn load_register(&mut self, first_register: usize, second_register: usize) {
        self.registers[first_register] = self.registers[second_register];
    }

    fn or(&mut self, first_register: usize, second_register: usize) {
        self.registers[first_register] = self.registers[first_register] | self.registers[second_register];
    }

    fn and(&mut self, first_register: usize, second_register: usize) {
        self.registers[first_register] = self.registers[first_register] & self.registers[second_register];
    }

    fn xor(&mut self, first_register: usize, second_register: usize) {
        self.registers[first_register] = self.registers[first_register] ^ self.registers[second_register];
    }

    fn add_register(&mut self, first_register: usize, second_register: usize) {
        self.registers[first_register] = self.registers[first_register] + self.registers[second_register];
        todo!("add carry");
    }
    
    fn subtract_first(&mut self, first_register: usize, second_register: usize) {
        self.flag_register = self.registers[first_register] > self.registers[second_register];
        self.registers[first_register] = self.registers[first_register] - self.registers[second_register];
    }
    
    fn shift_right(&mut self, register: usize) {
        const LEAST_SIGNIFICANT_BIT_MASK: u8 = 0b00000001;
        self.flag_register = (self.registers[register] & LEAST_SIGNIFICANT_BIT_MASK) == LEAST_SIGNIFICANT_BIT_MASK;
        self.registers[register] >> 1;
    }

    fn subtract_second(&mut self, first_register: usize, second_register: usize) {
        self.flag_register = self.registers[second_register] > self.registers[first_register];
        self.registers[first_register] = self.registers[second_register] - self.registers[first_register];
    }

    fn shift_left(&mut self, register: usize) {
        const most_significant_bit_mask: u8 = 0b10000000;
        self.flag_register = (self.registers[register] & most_significant_bit_mask) == most_significant_bit_mask;
        self.registers[register] = self.registers[register] << 1;
    }

    fn skip_not_equal_register(&mut self, first_register: usize, second_register: usize) {
        if self.registers[first_register] != self.registers[second_register] {
            self.program_counter += 2;
        }
    }

    fn load_i(&mut self, address: u16) {
        self.i_register = address;
    }

    fn jump_offset(&mut self, address: u16) {
        self.program_counter = (self.registers[0] as u16) + address;
    }

    fn random(&mut self, register: usize, value: u8) {
        let random_value: u8 = rand::thread_rng().gen();
        self.registers[register] = random_value & value;
    }

    fn draw(&mut self, first_register: usize, second_register: usize, bytes: u8) {
        let mut x_cor = self.registers[first_register];
        let mut y_cor = self.registers[second_register];

        const MASK: u8 = 0b10000000;

        //sanity check
        if bytes > 15 {
            panic!("Attempting to draw too many bytes!");
        }

        for row in self.i_register..self.i_register + (bytes as u16) {
            //println!("{:#b}", self.memory[index as usize]);
            //each byte
            for bit in 0..=7 {
                if (self.memory[row as usize] & MASK >> bit) > 0 {
                    self.screen.toggle_pixel(x_cor, y_cor);
                }
                x_cor += 1;
            }
            y_cor += 1;
            x_cor -=8;
        }
        self.screen.print_display();
    }

    fn skip_pressed(&mut self, register: usize) {
        todo!("write keyboard implementation");
    }

    fn skip_not_pressed(&mut self, register: usize) {
        todo!("write keyboard implementation");
    }

    fn add_i(&mut self, register: usize) {
        self.i_register = self.i_register + (self.registers[register] as u16);
    }
}