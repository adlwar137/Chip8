use rand::Rng;


pub struct Chip8 {
    screen: Screen,
    memory: [u8; 4096],
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: usize,
    registers: [u8; 16],
    flag_register: bool,
    i_register: u16,
}

impl Chip8 {
    pub fn new() -> Chip8 {
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

        Chip8 {
            memory: reserved_memory,
            screen: screen,
            program_counter: 0,
            stack: [0; 16],
            stack_pointer: 0,
            registers: [0; 16],
            flag_register: false,
            i_register: 0,
        }
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

    fn xor(&mut self, registerA: usize, registerB: usize) {
        self.registers[registerA] = self.registers[registerA] ^ self.registers[registerB];
    }

    fn add_register(&mut self, registerA: usize, registerB: usize) {
        self.registers[registerA] = self.registers[registerA] + self.registers[registerB];
        todo!("add carry");
    }
    
    fn subtract_first(&mut self, registerA: usize, registerB: usize) {
        self.flag_register = self.registers[registerA] > self.registers[registerB];
        self.registers[registerA] = self.registers[registerA] - self.registers[registerB];
    }
    
    fn shift_right(&mut self, register: usize) {
        const least_significant_bit_mask: u8 = 0b00000001;
        self.flag_register = (self.registers[register] & least_significant_bit_mask) == least_significant_bit_mask;
        self.registers[register] >> 1;
    }

    fn subtract_second(&mut self, registerA: usize, registerB: usize) {
        self.flag_register = self.registers[registerB] > self.registers[registerA];
        self.registers[registerA] = self.registers[registerB] - self.registers[registerA];
    }

    fn shift_left(&mut self, register: usize) {
        const most_significant_bit_mask: u8 = 0b10000000;
        self.flag_register = (self.registers[register] & most_significant_bit_mask) == most_significant_bit_mask;
        self.registers[register] = self.registers[register] << 1;
    }

    fn skip_not_equal_register(&mut self, registerA: usize, registerB: usize) {
        if self.registers[registerA] != self.registers[registerB] {
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

    pub fn draw(&mut self, first_register: usize, second_register: usize, bytes: u8) {
        let mut x_cor = self.registers[first_register];
        let mut y_cor = self.registers[second_register];

        //const mask: u8 1 >> 8;

        //println!("{}", mask);

        if bytes > 15 {
            panic!("Attempting to draw too many bytes!");
        }
        for index in self.i_register..(bytes as u16) {
            println!("{:#b}", self.memory[index as usize]);
            //each byte
            for bit in 0..=8 {

            }
        }
    }

}

const SCREEN_HEIGHT: u8 = 32;
const SCREEN_WIDTH: u8 = 64;

pub struct Screen {
    data: [u64; 32],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            data: [0; 32],
        }
    }

    pub fn build(data: [u64; 32]) -> Screen {
        Screen {
            data: data,
        }
    }

    pub fn clear_screen(&mut self) {
        self.data = [0; 32];
    }

    pub fn toggle_pixel(&mut self, xcor: u8, ycor: u8) {
        let shift_left = SCREEN_WIDTH - (xcor % SCREEN_WIDTH) - 1;
        let shift_down = ycor % SCREEN_HEIGHT;
        let lowest: u64 = 1;

        let xor_mask = lowest << shift_left;

        self.data[shift_down as usize] = self.data[shift_down as usize] ^ xor_mask;
    }

    pub fn get_pixel(&self, xcor: u8, ycor: u8) -> bool {
        let shift_left = SCREEN_WIDTH - (xcor % SCREEN_WIDTH) - 1;
        let shift_down = ycor % SCREEN_HEIGHT;
        let lowest: u64 = 1;

        let and_mask = lowest << shift_left;

        let result = self.data[shift_down as usize] & and_mask;
        result > 0
    }

    pub fn print_display(&self) {
        for col in 0..SCREEN_HEIGHT {
            let mut col_out = String::new();
            for row in 0..SCREEN_WIDTH {
                if self.get_pixel(row, col) {
                    col_out.push_str("█");
                } else {
                    col_out.push_str(" ");
                }
            }
            println!("{}", col_out);
        }
    }
}