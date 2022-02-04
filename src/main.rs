mod chip8;
use chip8::renderer;
use chip8::Chip8;
use std::{thread, time};

fn main() {
    println!("Hello, world!");

    let mut chip = Chip8::new();
    chip.load_default_font();

    let mut display = renderer::AsciiDisplay::new();
    display.render();

    for i in 0..32 {
        display.draw_sprite(i, i, 127);
        display.render();
        thread::sleep(time::Duration::from_millis(100));
    }
    display.clear();
    display.render();
}
