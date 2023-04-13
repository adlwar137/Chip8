pub struct Chip8 {
    screen: Screen,
    memory: [u8; 4096],
    program_counter: u16,
    stack: [u16; 16],
    stack_pointer: usize,
    registers: [u8; 16],
    flag_register: bool,
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

    fn skip_equal_register(&mut self, registerA: usize, registerB: usize) {
        if self.registers[registerA] == self.registers[registerB] {
            self.program_counter += 2;
        }
    }

    fn load(&mut self, register: usize, value: u8) {
        self.registers[register] = value;
    }

    fn add(&mut self, register: usize, value: u8) {
        self.registers[register] = self.registers[register] + value;
    }

    fn load_register(&mut self, registerA: usize, registerB, usize) {
        self.registers[registerA] = self.registers[registerB];
    }

    fn or(&mut self, registerA: usize, registerB: usize) {
        self.registers[registerA] = self.registers[registerA] | self.registers[registerB];
    }

    fn and(&mut self, registerA: usize, registerB: usize) {
        self.registers[registerA] = self.registers[registerA] & self.registers[registerB];
    }

    fn xor(&mut self, registerA: usize, registerB: usize) {
        self.registers[registerA] = self.registers[registerA] ^ self.registers[registerB];
    }

    fn add_register(&mut self, registerA: usize, registerB: usize) {
        self.registers[registerA] = self.registers[registerA] + self.registers[registerB];
        todo!("add carry");
    }

    fn shift_right(&mut self, register: usize) {
        
    }
}

pub struct Screen {
    data: [u32; 64],
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            data: [0; 64]
        }
    }

    fn clear_screen(&mut self) {
        self.data = [0; 64];
    }
}