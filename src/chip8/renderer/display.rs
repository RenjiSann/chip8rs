pub struct Display {
    pub tab: [u64; 32],
}

impl Display {
    pub fn new() -> Display {
        Display { tab: [0; 32] }
    }
    pub fn clear(&mut self) {
        // Set all the values to 0
        for i in self.tab.iter_mut() {
            *i = 0u64;
        }
    }
    pub fn draw_sprite(&mut self, x: u8, y: u8, byte: u8) {
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