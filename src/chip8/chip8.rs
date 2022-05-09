use super::{Buzzer, Chip8, ChipCfg, ChipInst, SDLDisplay, DEFAULT_FONT};
use std::fs::File;
use std::io;
use std::io::Read;

use sdl2::video;
use sdl2::AudioSubsystem;

impl Chip8 {
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
            disp: SDLDisplay::new(win)?,
            audio: buzzer,
            config: Default::default(),
            exit: false,
        })
    }
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
        if self.st == 0 {
            self.audio.stop()
        } else {
            self.audio.start()
        }
    }
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
