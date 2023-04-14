use chip8::Chip8;

const OFFSET: i32 = 0x200;

fn main() {
    let rom: Vec<u8> = vec![
        0x63, 0x05, //0x200 LD      V3,     5
        0xD1, 0x25, //0x202 DRW     V1,     V2,     5
        0xF3, 0x1E, //0x204 ADDI    V3
        0x71, 0x05, //0x206 ADD     V1,     5
        0x12, 0x00  //0x208 JMP     0x202
    ];
    let mut chippy = Chip8::new();
    chippy.load_rom(rom);
    for _ in OFFSET..OFFSET + 40 {
        chippy.tick();
    }
}