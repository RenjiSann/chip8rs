// Exported modules
pub mod ascii_display;
pub use ascii_display::AsciiDisplay;

// Internal use modules
mod display;

pub trait ChipRenderer {
    fn new() -> Self;
    fn render(&self);
    fn clear(&mut self);
    fn draw_sprite(&mut self, x: u8, y: u8, byte: u8);
}
