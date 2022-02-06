use super::display::Display;
use super::ChipRenderer;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;
use sdl2::video::Window;

pub struct SDLDisplay {
    disp: Display,
    canvas: WindowCanvas,
}

impl SDLDisplay {
    pub fn new(win: Window) -> Result<Self, sdl2::IntegerOrSdlError> {
        let mut canvas = win.into_canvas().build()?;

        // Set the background to black;
        canvas.set_draw_color(Color::BLACK);
        canvas.clear();
        canvas.present();

        Ok(SDLDisplay {
            disp: Display::new(),
            canvas: canvas
        })
    }
}

impl ChipRenderer for SDLDisplay {
    fn render(&mut self) {
        let size = self.canvas.output_size();
        if let Err(e) = &size {
            panic!("Panic on size: {}", e);
        }

        let (width, height) = size.unwrap();
        let pt_width = width / 64;
        let pt_height = height / 32;

        let mut pixel;
        let mut rect: Rect;

        // First, clear the screen
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();
        self.canvas.set_draw_color(Color::WHITE);

        for row in 0..32 {
            for col in 0..64 {
                // Pixel is true only if (row, col) is 1
                pixel = self.disp.tab[row as usize] & (1 << (63 - col)) != 0;

                // Only draw if white
                if pixel {
                    rect = Rect::from((
                        col * pt_width as i32,
                        row * pt_height as i32,
                        pt_width,
                        pt_height,
                    ));
                    if let Err(e) = self.canvas.fill_rect(rect) {
                        panic!("Error on fill rect: {}", e);
                    }
                }
            }
        }

        self.canvas.present();
    }

    fn clear(&mut self) {
        self.disp.clear();
    }

    fn draw_sprite(&mut self, x: u8, y: u8, byte: u8) {
        self.disp.draw_sprite(x, y, byte)
    }
}
