use crate::display::Screen;
use rand::Rng;

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
        let mut reserved_memory: [u8; 4096] = [0; 4096];

        //zero
        reserved_memory[0] = 0b11110000; // ****
        reserved_memory[1] = 0b10010000; // *  *
        reserved_memory[2] = 0b10010000; // *  *
        reserved_memory[3] = 0b10010000; // *  *
        reserved_memory[4] = 0b11110000; // ****
        //one
        reserved_memory[5] = 0b00100000; //   *
        reserved_memory[6] = 0b01100000; //  **
        reserved_memory[7] = 0b00100000; //   *
        reserved_memory[8] = 0b00100000; //   *
        reserved_memory[9] = 0b01110000; //  ***
        //two
        reserved_memory[10] = 0b11110000; // ****
        reserved_memory[11] = 0b00010000; //    *
        reserved_memory[12] = 0b11110000; // ****
        reserved_memory[13] = 0b10000000; // *
        reserved_memory[14] = 0b11110000; // ****

        let screen = Screen::new();

        Cpu {
            memory: reserved_memory,
            screen: Screen::new(),
            program_counter: 0,
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
        const least_significant_bit_mask: u8 = 0b00000001;
        self.flag_register = (self.registers[register] & least_significant_bit_mask) == least_significant_bit_mask;
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
}