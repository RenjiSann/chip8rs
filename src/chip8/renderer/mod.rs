// Exported modules
pub mod ascii_display;
pub mod sdl_display;

// Avoid verbosity
pub use ascii_display::AsciiDisplay;
pub use sdl_display::SDLDisplay;

/**
 * Define the functions a Chip8 screen renderer should have
 */
pub trait ChipRenderer {
    fn render(&mut self);
    fn clear(&mut self);
    fn draw_sprite(&mut self, x: u8, y: u8, byte: u8);
}

// INTERNAL USE
mod display;
