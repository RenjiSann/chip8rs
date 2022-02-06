// Exported modules
pub mod ascii_display;

// Avoid verbosity
pub use ascii_display::AsciiDisplay;

/**
 * Define the functions a Chip8 screen renderer should have
 */
pub trait ChipRenderer {
    fn new() -> Self;
    fn render(&self);
    fn clear(&mut self);
    fn draw_sprite(&mut self, x: u8, y: u8, byte: u8);
}

// INTERNAL USE
mod display;
