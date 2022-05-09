pub mod buzzer;
pub mod chip_debug;
pub mod input;
pub mod instruction;
pub mod renderer;
pub mod chip8;

use buzzer::Buzzer;
use instruction::ChipInst;
use renderer::SDLDisplay;


/**
 * Retro-compatibility options
 */
#[derive(Debug)]
struct ChipCfg {
    font_start: u16,       // Starting address of the fonts bytes
    off_jump_legacy: bool, // If true, BNNN will jump to NNN + V0. Else, to NNN + Vx
    reg_save_legacy: bool, // If true, FX55 and FX65 will alter the value of I
    index_add_carry: bool, // If true, carry will be set when I overflows with FX1E
}

/**
 * @brief Define the data structure for our Chip8 representation.
 */
pub struct Chip8 {
    i: u16,  // 16-bit index register
    pc: u16, // 16-bit program counter
    dt: u8,  // 8-bit delay timer
    st: u8,  // 8-bit sound timer
    sp: u8,  // 8-bit stack pointer

    v: [u8; 16],          // 16 multi-purpose 8-bit registers
    stack: [u16; 32],     // 32 words deep call-stack
    mem: [u8; 4096usize], // 4 KiB RAM

    disp: SDLDisplay, // The output display
    audio: Buzzer,    // The audio output

    config: ChipCfg, // Chip configuration

    exit: bool, // Boolean set to true if chip should be killed
}

/**
 * Hexadecimal digits sprites to print on screen.
 */
const DEFAULT_FONT: [u8; 80] = [
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
