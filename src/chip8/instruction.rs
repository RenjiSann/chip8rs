use super::input;
use super::Chip8;
use rand::random;

use sdl2::EventPump;

#[derive(Debug)]
pub struct ChipInst {
    pub i: u8,     // bits 0-3
    pub x: u8,     // bits 4-7
    pub y: u8,     // bits 8-11
    pub n: u8,     // bits 12-15
    pub nn: u8,    // bits 8-15
    pub nnn: u16,  // bits 4-15
    pub full: u16, // bits 0-15
}

impl ChipInst {
    pub fn new(w: u16) -> Self {
        ChipInst {
            i: ((w & 0xF000) >> 12) as u8,
            x: ((w & 0x0F00) >> 8) as u8,
            y: ((w & 0x00F0) >> 4) as u8,
            n: (w & 0x000F) as u8,
            nn: (w & 0x00FF) as u8,
            nnn: w & 0x0FFF,
            full: w,
        }
    }
}

/**
 * Define all instruction functions
 */
impl Chip8 {
    pub fn execute(&mut self, inst: &ChipInst, ev_pump: Option<&EventPump>) {
        // Match the first half-byte
        match inst.i {
            0x0 => match inst.nnn {
                0x0E0 => self.inst_00E0(inst, ev_pump),
                0x0EE => self.inst_00EE(inst, ev_pump),
                _ => panic!(
                    "Unknown instruction at {:#03x}: {:04x}",
                    self.pc - 2,
                    inst.full
                ),
            },
            0x1 => self.inst_1NNN(inst, ev_pump),
            0x2 => self.inst_2NNN(inst, ev_pump),
            0x3 => self.inst_3XNN(inst, ev_pump),
            0x4 => self.inst_4XNN(inst, ev_pump),
            0x5 => self.inst_5XY0(inst, ev_pump),
            0x6 => self.inst_6XNN(inst, ev_pump),
            0x7 => self.inst_7XNN(inst, ev_pump),
            0x8 => match inst.n {
                0x0 => self.inst_8XY0(inst, ev_pump),
                0x1 => self.inst_8XY1(inst, ev_pump),
                0x2 => self.inst_8XY2(inst, ev_pump),
                0x3 => self.inst_8XY3(inst, ev_pump),
                0x4 => self.inst_8XY4(inst, ev_pump),
                0x5 => self.inst_8XY5(inst, ev_pump),
                0x6 => self.inst_8XY6(inst, ev_pump),
                0x7 => self.inst_8XY7(inst, ev_pump),
                0xe => self.inst_8XYE(inst, ev_pump),
                0xA1 => self.inst_EXA1(inst, ev_pump),
                _ => panic!(
                    "Unknown instruction at {:#03x}: {:04x}",
                    self.pc - 2,
                    inst.full
                ),
            },
            0x9 => self.inst_9XY0(inst, ev_pump),
            0xa => self.inst_ANNN(inst, ev_pump),
            0xb => self.inst_BNNN(inst, ev_pump),
            0xc => self.inst_CXNN(inst, ev_pump),
            0xd => self.inst_DXYN(inst, ev_pump),
            0xe => match inst.nn {
                0x9E => self.inst_EX9E(inst, ev_pump),
                0xA1 => self.inst_EXA1(inst, ev_pump),
                _ => panic!(
                    "Unknown instruction at {:#03x}: {:04x}",
                    self.pc - 2,
                    inst.full
                ),
            },
            0xf => match inst.nn {
                0x07 => self.inst_FX07(inst, ev_pump),
                0x0a => self.inst_FX0A(inst, ev_pump),
                0x15 => self.inst_FX15(inst, ev_pump),
                0x18 => self.inst_FX18(inst, ev_pump),
                0x1e => self.inst_FX1E(inst, ev_pump),
                0x29 => self.inst_FX29(inst, ev_pump),
                0x33 => self.inst_FX33(inst, ev_pump),
                0x55 => self.inst_FX55(inst, ev_pump),
                0x65 => self.inst_FX65(inst, ev_pump),
                _ => panic!(
                    "Unknown instruction at {:#03x}: {:04x}",
                    self.pc - 2,
                    inst.full
                ),
            },
            _ => panic!(
                "Unknown instruction at {:#03x}: {:04x}",
                self.pc - 2,
                inst.full
            ),
        }
    }

    /**
     * All instructions have the same signature for potential
     * function pointer stuff later
     */

    #[allow(non_snake_case)]
    fn inst_00E0(&mut self, _inst: &ChipInst, _ep: Option<&EventPump>) {
        // Just clear the screen
        self.disp.clear();
        self.disp.render();
    }

    #[allow(non_snake_case)]
    fn inst_00EE(&mut self, _inst: &ChipInst, _ep: Option<&EventPump>) {
        // 'ret' instruction
        self.pc = self.stack[self.sp as usize];
        self.sp -= 1;
    }

    #[allow(non_snake_case)]
    fn inst_1NNN(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Simple jump
        self.pc = inst.nnn;
    }

    #[allow(non_snake_case)]
    fn inst_2NNN(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Function call
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc;
        self.pc = inst.nnn;
    }

    #[allow(non_snake_case)]
    fn inst_3XNN(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Skip next instruction if Vx == NN
        if self.v[inst.x as usize] == inst.nn {
            self.pc += 2;
        }
    }

    #[allow(non_snake_case)]
    fn inst_4XNN(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Skip next instruction if Vx != NN
        if self.v[inst.x as usize] != inst.nn {
            self.pc += 2;
        }
    }

    #[allow(non_snake_case)]
    fn inst_5XY0(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Skip next instruction if Vx == Vy
        if self.v[inst.x as usize] == self.v[inst.y as usize] {
            self.pc += 2
        }
    }

    #[allow(non_snake_case)]
    fn inst_6XNN(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to NN
        self.v[inst.x as usize] = inst.nn;
    }

    #[allow(non_snake_case)]
    fn inst_7XNN(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to Vx + NN with no carry set
        let vx = &mut self.v[inst.x as usize];
        *vx = (*vx).wrapping_add(inst.nn);
    }

    #[allow(non_snake_case)]
    fn inst_8XY0(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to Vy
        self.v[inst.x as usize] = self.v[inst.y as usize];
    }

    #[allow(non_snake_case)]
    fn inst_8XY1(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to Vx | Vy
        self.v[inst.x as usize] |= self.v[inst.y as usize];
    }

    #[allow(non_snake_case)]
    fn inst_8XY2(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to Vx & Vy
        self.v[inst.x as usize] &= self.v[inst.y as usize];
    }

    #[allow(non_snake_case)]
    fn inst_8XY3(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to Vx ^ Vy
        self.v[inst.x as usize] ^= self.v[inst.y as usize];
    }

    #[allow(non_snake_case)]
    fn inst_8XY4(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to Vx + Vy (with carry set)
        let x: u16 = self.v[inst.x as usize] as u16 + self.v[inst.y as usize] as u16;
        self.v[inst.x as usize] = x as u8;
        self.v[0xF] = if x > (1 << 8) { 1 } else { 0 };
    }

    #[allow(non_snake_case)]
    fn inst_8XY5(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to Vx - Vy and set carry to 0 if Vy > Vx
        let xx = self.v[inst.x as usize];
        let yy = self.v[inst.y as usize];
        let carry = if yy > xx { 0 } else { 1 };
        self.v[inst.x as usize] = xx.wrapping_sub(yy);
        self.v[0xF] = carry;
    }

    #[allow(non_snake_case)]
    fn inst_8XY6(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to Vy, then shift Vx by 1 on the
        // right and set carry to the shifted out bit
        let y = &mut self.v[inst.y as usize];
        *y >>= 1;
        self.v[0xF] = *y & 0x01;
    }

    #[allow(non_snake_case)]
    fn inst_8XY7(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to Vy - Vx and set carry to 0 if Vx > Vy
        let xx = self.v[inst.x as usize];
        let yy = self.v[inst.y as usize];
        let carry = if yy < xx { 0 } else { 1 };
        self.v[inst.x as usize] = yy.wrapping_sub(xx);
        self.v[0xF] = carry;
    }

    #[allow(non_snake_case)]
    fn inst_8XYE(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to Vy, then shift Vx by 1 on the
        // left and set carry to the shifted out bit
        let y = &mut self.v[inst.x as usize];
        *y <<= 1;
        self.v[0xF] = *y & 0x80;
    }

    #[allow(non_snake_case)]
    fn inst_9XY0(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Skip next instruction if Vx != Vy
        if self.v[inst.x as usize] != self.v[inst.y as usize] {
            self.pc = self.pc.wrapping_add(2);
        }
    }

    #[allow(non_snake_case)]
    fn inst_ANNN(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set I to NNN
        self.i = inst.nnn;
    }

    #[allow(non_snake_case)]
    fn inst_BNNN(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Warning, legacy and modern implementation differ
        if self.config.off_jump_legacy {
            // Legacy: Set PC to V0 + NNN
            self.pc = (self.v[0x0] as u16).wrapping_add(inst.nnn);
        } else {
            // Modern: Set PC to VX + XNN
            self.pc = (self.v[inst.x as usize] as u16).wrapping_add(inst.nnn);
        }
    }

    #[allow(non_snake_case)]
    fn inst_CXNN(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to NN & random
        self.v[inst.x as usize] = random::<u8>() & inst.nn;
    }

    #[allow(non_snake_case)]
    fn inst_DXYN(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Draw a sprite on the screen, starting at coordinates
        // (Vx % 64, Vy % 32), being N pixel tall and 8 pixels large,
        // taking sprites from mem[I]
        let vx = self.v[inst.x as usize] % 64;
        let vy = self.v[inst.y as usize] % 32;

        for i in 0..inst.n {
            if vy + i >= 32 {
                break;
            }
            self.disp.draw_sprite(
                vx,
                vy.wrapping_add(i),
                self.mem[(self.i + (i as u16)) as usize],
            )
        }

        // Update display
        self.disp.render();
    }

    #[allow(non_snake_case)]
    fn inst_EX9E(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Skip next instruction if the key Vx is pressed
        // Check if an event pump is provided
        if let Some(ep) = _ep {
            if input::is_pressed(ep, self.v[inst.x as usize]) {
                self.pc += 2
            }
        }
    }

    #[allow(non_snake_case)]
    fn inst_EXA1(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Skip next instruction if the key Vx is not pressed
        // Check if an event pump is provided
        if let Some(ep) = _ep {
            if !input::is_pressed(ep, self.v[inst.x as usize]) {
                self.pc += 2
            }
        }
    }

    #[allow(non_snake_case)]
    fn inst_FX07(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set Vx to DT
        self.v[inst.x as usize] = self.dt;
    }

    #[allow(non_snake_case)]
    fn inst_FX0A(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Get the pressed key and put it in Vx
        // Check if an event pump is provided
        if let Some(ep) = &_ep {
            // Check if there is a key currently pressed
            if let Some(u) = input::get_scancode(ep) {
                self.v[inst.x as usize] = u;
            }
            // If no key is pressed, wait for one to be pressed
            else {
                self.pc = self.pc.wrapping_sub(2);
            }
        }
    }

    #[allow(non_snake_case)]
    fn inst_FX15(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set DT to Vx
        self.dt = self.v[inst.x as usize];
    }

    #[allow(non_snake_case)]
    fn inst_FX18(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Set ST to Vx
        self.st = self.v[inst.x as usize];
    }

    #[allow(non_snake_case)]
    fn inst_FX1E(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Add Vx to I
        // if enabled, set carry bit to 1 if I goes
        // from 0x0FFF to 0x1000+
        let save_i = self.i;
        self.i += self.v[inst.x as usize] as u16;

        // If needed, set the carry bit
        if self.config.index_add_carry {
            self.v[0xF] = if save_i == 0x0FFF && self.i >= 0x1000 {
                1
            } else {
                0
            };
        }
    }

    #[allow(non_snake_case)]
    fn inst_FX29(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Put I at the address of the font character in Vx
        let vx = self.v[inst.x as usize];

        // font_start is the first drawing byte of '0',
        // and there are 5 bytes per character
        self.i = self.config.font_start + ((vx as u16) * 5);
    }

    #[allow(non_snake_case)]
    fn inst_FX33(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Put the decimal digit values of Vx into
        // I, I + 1, and I + 2
        // Should never overflow as max value of Vx is 255
        let vx = self.v[inst.x as usize];
        let i: usize = self.i as usize;
        self.mem[i] = vx / 100;
        self.mem[i + 1] = (vx / 10) % 10;
        self.mem[i + 2] = vx % 10;
    }

    #[allow(non_snake_case)]
    fn inst_FX55(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Store V0 up to Vx from I to I + x
        // (Vx is included)
        let i: usize = self.i as usize;
        for k in 0..((inst.x as usize) + 1) {
            self.mem[i + k] = self.v[k];
        }

        // If enabled, set the index to follow the
        // legacy behavior
        if self.config.reg_save_legacy {
            self.i += (inst.x as u16) + 1;
        }
    }

    #[allow(non_snake_case)]
    fn inst_FX65(&mut self, inst: &ChipInst, _ep: Option<&EventPump>) {
        // Load memory I to I + x in V0 to Vx
        // (Vx is included)
        let i: usize = self.i as usize;
        for k in 0..((inst.x as usize) + 1) {
            self.v[k] = self.mem[i + k];
        }

        // If enabled, set the index to follow the
        // legacy behavior
        if self.config.reg_save_legacy {
            self.i += (inst.x as u16) + 1;
        }
    }
}
