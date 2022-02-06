use super::display::Display;
use super::ChipRenderer;

const ZERO: char = '-';
const ONE: char = '@';

pub struct AsciiDisplay {
    disp: Display,
}

impl AsciiDisplay{
    pub fn new() -> AsciiDisplay {
        AsciiDisplay {
            disp: Display::new(),
        }
    }
}

impl ChipRenderer for AsciiDisplay {

    fn render(&mut self) {
        // Clear the terminal
        print!("\x1B[2J\x1B[1;1H");

        // Temporary array
        let mut chr_arr = [ZERO; 64];

        // For each line
        for line in self.disp.tab {
            for i in 0..64 {
                chr_arr[i] = if line & (1 << (63 - i)) != 0 {
                    ONE
                } else {
                    ZERO
                };
            }
            println!("{}", String::from_iter(chr_arr.iter()));
        }
    }

    fn clear(&mut self) {
        self.disp.clear()
    }

    fn draw_sprite(&mut self, x: u8, y: u8, byte: u8) {
        self.disp.draw_sprite(x, y, byte)
    }
}
