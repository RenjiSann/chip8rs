use super::Chip8;
use std::cmp::min;
use std::fmt;

impl fmt::Debug for Chip8 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Build a string for showing a part of the memory
        // From index to index + 10

        let low = self.pc as usize;
        let up = min(1 + (self.pc as usize) + 10, 0x1000);

        let mut mem_str = String::new();
        for i in low..up {
            mem_str.push_str(&format!("{:#05X}: {:02x}   ", i, self.mem[i]));
        }

        // Pretty print the registers
        let mut regs = String::new();
        for i in 0..4 {
            for j in 0..3 {
                regs.push_str(&format!("{:#03X}: {}   ", 4 * i + j, self.v[4 * i + j]));
            }
            regs.push_str(&format!("{:#03X}: {}", 4 * i + 3, self.v[4 * i + 3]));
        }

        f.debug_struct("Chip8 state:")
            .field("index", &self.i)
            .field("Program Counter", &self.pc)
            .field("delay timer", &self.dt)
            .field("sound timer", &self.st)
            .field("stack pointer", &self.sp)
            .field("registers", &regs)
            .field("memory", &mem_str)
            .finish_non_exhaustive()
    }
}
