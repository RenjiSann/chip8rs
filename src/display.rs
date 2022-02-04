pub struct Display {
    pub tab: [u64; 32],
}

impl Display {
    fn new() -> Display {
        Display { tab: [0; 32] }
    }
    fn clear(&mut self) {
        // Set all the values to 0
        for i in self.tab.iter_mut() {
            *i = 0u64;
        }
    }
    fn draw_sprite(&mut self, x: u8, y: u8, byte: u8) {
        // Get the 'line' to update
        let mut val: u64 = self.tab[y as usize];

        // Compute the mask, thanks to bit shifting
        let mask = if x > 56 {
            (byte as u64) >> (x - 56)
        } else {
            (byte as u64) << (56 - x)
        };

        // Apply the mask
        val = val ^ mask;

        // Update the array
        self.tab[y as usize] = val;
    }
}

const ZERO: char = '-';
const ONE: char = '@';

pub struct AsciiDisplay {
    disp: Display,
}

impl AsciiDisplay {
    pub fn new() -> AsciiDisplay {
        AsciiDisplay {
            disp: Display::new(),
        }
    }
    pub fn render(&self) {
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
    pub fn clear(&mut self) {
        self.disp.clear()
    }
    pub fn draw_sprite(&mut self, x: u8, y: u8, byte: u8) {
        self.disp.draw_sprite(x, y, byte)
    }
}
