//nnn or addr - A 12-bit value, the lowest 12 bits 
//of the instruction

//n or nibble - A 4-bit value, the lowest 4 bits of 
//the instruction

//x - A 4-bit value, the lower 4 bits of the high 
//byte of the instruction

//y - A 4-bit value, the upper 4 bits of the low
//byte of the instruction

//kk or byte - An 8-bit value, the lowest 8 bits 
//of the instruction

//Register referred to as Vx where x is the register

//Instruction set

//00E0 - CLS
//Clear the display.

//00EE - RET
//Return from subroutine.
//The interpreter sets the program counter to the 
//address at the top of the stack, then subtracts
//1 from the stack pointer.

//1nnn - JP addr
//Jump to location nnn.
//The interpreter sets the program counter to nnn.

//2nnn - CALL addr
//Call subroutine at nnn.
//The interpreter increments the stack pointer,
//then puts the current PC on the top of the stack.
//The PC is then set to nnn.

//4xkk - SNE Vx, byte
//Skip next instruction if Vx != kk.
// The interpreter compares register Vx to kk, and 
//if they are not equal, increments the program counter
//by 2.

//5xy0 - SE Vx, Vy
//Skip next instruction if Vx = Vy.
//The interpreter compares the register Vx to register
//Vy, and if they are equal, increments the program
//counter by 2.

//6xkk - LD Vx, byte
//Set Vx = kk.
//The interpreter puts the value kk into register Vx.

//7xkk - ADD Vx, byte
//Set Vx = Vx + kk.
//Adds the value kk to the value of register Vx, then 
//stores the result in Vx.

//8xy0 - LD Vx, Vy
//Set Vx = Vy.
//Stores the value of register Vy in register Vx.
use chip8::Chip8;
use chip8::Screen;

fn main() {
    let mut chippy = Chip8::new();
    
    chippy.draw(1, 2, 5);

    let mut screen = Screen::new();

    for i in 0..64 {
        screen.toggle_pixel(i,i);
    }

    screen.print_display();
}