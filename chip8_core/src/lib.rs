// REFERENCES LIST:
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM
// https://github.com/aquova/chip8-book

// The original implementation of the Chip-8 language used a 64x32-pixel monochrome display
pub const SCREEN_WIDTH: usize = 64;
pub const SCREEN_HEIGHT:usize = 32;

// The Chip-8 language is capable of accessing up to 4KB (4,096 bytes) of RAM.
const RAM_SIZE: usize = 4096;

// Chip-8 has 16 general purpose 8-bit registers
const NUM_REGISTERS: usize = 16;
const STACK_SIZE:usize = 16;

// The computers which originally used the Chip-8 Language had a 16-key hexadecimal keypad
const NUM_KEYS:usize = 16;

// Most Chip-8 programs start at location 0x200 (512), but some begin at 0x600 (1536). 
// Programs beginning at 0x600 are intended for the ETI 660 computer.
const START_ADDRESS: u16 = 0x200;

// Chip-8 draws graphics on screen through the use of sprites. 
// A sprite is a group of bytes which are a binary representation of the desired picture.
// Chip-8 sprites may be up to 15 bytes, for a possible sprite size of 8x15.
// The data should be stored in the interpreter area of Chip-8 memory (0x000 to 0x1FF).
const NUM_SPRITES: usize = 16 * 5;
const SPRITES:[u8;NUM_SPRITES] = [
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
    0xF0, 0x80, 0xF0, 0x80, 0x80 // 
];
pub struct Emu{
    program_counter: u16,
    ram: [u8;RAM_SIZE],
    screen: [bool; SCREEN_WIDTH*SCREEN_HEIGHT],
    
    v_reg:[u8;NUM_REGISTERS],
    // There is a 16-bit register called I. 
    // This register is generally used to store memory addresses, so only the lowest (rightmost) 12 bits are usually used.
    i_reg:u16,
    stack_pointer:u16,
    stack:[u16;STACK_SIZE],
    keys:[bool;NUM_KEYS],
    delay_timer:u8,
    sound_timer:u8
}
impl Emu{
    pub fn new()->Self{
        let mut emu = Self { 
            program_counter: START_ADDRESS,
            ram: [0;RAM_SIZE], 
            screen: [false;SCREEN_WIDTH*SCREEN_HEIGHT], 
            v_reg: [0;NUM_REGISTERS], 
            i_reg: 0, 
            stack_pointer: 0, 
            stack: [0;STACK_SIZE],
            keys: [false;NUM_KEYS], 
            delay_timer: 0, 
            sound_timer: 0 
        };
        emu.ram[0..NUM_SPRITES].copy_from_slice(&SPRITES);
        emu
    }
    pub fn reset(&mut self){
        self.program_counter = START_ADDRESS;
        self.ram = [0;RAM_SIZE];
        self.screen = [false;SCREEN_WIDTH*SCREEN_HEIGHT];
        self.v_reg = [0; NUM_REGISTERS];
        self.i_reg = 0;
        self.stack_pointer = 0;
        self.stack = [0; STACK_SIZE];
        self.keys = [false; NUM_KEYS];
        self.delay_timer = 0;
        self.sound_timer = 0;
        self.ram[..NUM_SPRITES].copy_from_slice(&SPRITES);
    }
    pub fn push(&mut self, val: u16){
        self.stack[self.stack_pointer as usize] = val;
        self.stack_pointer +=1;
    }
    pub fn pop(&mut self) -> u16{
        self.stack_pointer-=1;
        self.stack[self.stack_pointer as usize]
    }
    
}
