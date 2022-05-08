pub mod buzzer;
pub mod chip_debug;
pub mod input;
pub mod instruction;
pub mod renderer;

use buzzer::Buzzer;

use renderer::{ChipRenderer, AsciiDisplay, SDLDisplay};

use instruction::ChipInst;
use std::fs::File;
use std::io;
use std::io::Read;

use sdl2::video;
use sdl2::AudioSubsystem;

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

impl Default for ChipCfg {
    fn default() -> Self {
        ChipCfg {
            font_start: 0x050,
            off_jump_legacy: false,
            reg_save_legacy: false,
            index_add_carry: false,
        }
    }
}

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

pub struct Chip8 {
    i: u16,  // 16-bit index register
    pc: u16, // 16-bit program counter
    dt: u8,  // 8-bit delay timer
    st: u8,  // 8-bit sound timer
    sp: u8,  // 8-bit stack pointer

    v: [u8; 16],          // 16 multi-purpose 8-bit registers
    stack: [u16; 32],     // 32 words deep call-stack
    mem: [u8; 4096usize], // 4 KiB RAM

    disp: Box<dyn ChipRenderer>, // The output display
    audio: Option<Buzzer>,       // The audio output

    config: ChipCfg, // Chip configuration

    exit: bool, // Boolean set to true if chip should be killed
}

impl Chip8 {
    /**
     * @brief Create a Chip8 emulator without SDL, meaning without input and
     * sound, and in a terminal window.
     */
    pub fn new_ascii() -> Self {
        Chip8 {
            i: 0,
            pc: 0x200,
            dt: 0,
            st: 0,
            sp: 0,
            v: [0; 16],
            stack: [0; 32],
            mem: [0; 4096],
            disp: Box::new(AsciiDisplay::new()),
            audio: None,
            config: Default::default(),
            exit: false,
        }
    }

    /**
     * @brief Create a Chip8 emulator with SDL.
     */
    pub fn new_sdl(
        win: video::Window,
        audio: &AudioSubsystem,
    ) -> Result<Self, sdl2::IntegerOrSdlError> {
        // Create the buzzer, linked to SDL's audio subsystem.
        let buzzer = Buzzer::new(audio).unwrap();

        Ok(Chip8 {
            i: 0,
            pc: 0x200,
            dt: 0,
            st: 0,
            sp: 0,
            v: [0; 16],
            stack: [0; 32],
            mem: [0; 4096],
            disp: Box::new(SDLDisplay::new(win)?),
            audio: Some(buzzer),
            config: Default::default(),
            exit: false,
        })
    }
}

impl Chip8 {
    /**
     * Load a program from the bytes of a file
     */
    pub fn load_file(&mut self, path: &str) -> Result<(), io::Error> {
        // Load the file
        let mut f: File = File::open(path)?;

        // Start writing at address 0x200 (512)
        // Because 0x0 - 0x1FF is kept for internal use
        let addr = 0x200;
        f.read(&mut self.mem[addr..])?;
        Ok(())
    }

    /**
     * Load a program from a byte array
     */
    pub fn load_program(&mut self, arr: &[u8]) {
        let startprgm: &mut [u8] = &mut self.mem[0x200..];

        if startprgm.len() < arr.len() {
            panic!("Program is to large !");
        }

        for i in 0..arr.len() {
            startprgm[i] = arr[i];
        }
    }

    pub fn load_font(&mut self, path: &str) -> Result<(), io::Error> {
        // Load the file
        let mut f: File = File::open(path)?;

        // Start writing at font_beg, as given by the config
        // Then write 80 (5 * 16) bytes
        let font_beg = self.config.font_start as usize;
        let font_end = font_beg + (5 * 16) as usize;
        f.read(&mut self.mem[font_beg..font_end])?;
        Ok(())
    }

    pub fn load_default_font(&mut self) {
        // Load the hardcoded font
        let offset = self.config.font_start as usize;
        for i in 0..DEFAULT_FONT.len() {
            self.mem[offset + i] = DEFAULT_FONT[i];
        }
    }

    pub fn fetch(&mut self) -> ChipInst {
        let b1 = self.mem[self.pc as usize];
        let b2 = self.mem[(self.pc + 1) as usize];
        self.pc += 2;
        let w: u16 = ((b1 as u16) << 8) | (b2 as u16);
        ChipInst::new(w)
    }

    pub fn has_exited(&self) -> bool {
        self.exit
    }

    pub fn update_timers(&mut self) {
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            self.st -= 1;
        }
    }

    pub fn refresh_buzzer(&mut self) {
        if let Some(buz) = &self.audio {
            if self.st == 0 {
                buz.stop()
            } else {
                buz.start()
            }
        }
    }
}
