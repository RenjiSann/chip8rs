/**
 * Simple Structure allowing to munipulate pixels
 * of a 64x32 screen.
 *
 * Because the screen is only 64 pixels large, we can
 * use u64 types to hold all the bits for a line.
 * This approach prevents from memory waste that the
 * '1 char per pixel' method would have produced.
 */
pub struct Display {
    pub tab: [u64; 32],
}

impl Display {
    pub fn new() -> Display {
        Display { tab: [0; 32] }
    }

    /**
     * Set all the pixels to 0
     */
    pub fn clear(&mut self) {
        for i in self.tab.iter_mut() {
            *i = 0u64;
        }
    }

    /**
     * Apply a byte to the screen.
     * 
     * The bit 0 of the given byte is applied on
     * the pixel (x, y), the bit 1 on (x + 1, y), etc...
     * 'Applying' the sprite only does a XOR between the 
     * actual value and the byte value.
     */
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
