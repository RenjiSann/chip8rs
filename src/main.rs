mod display;

fn main() {
    println!("Hello, world!");

    let mut display = display::AsciiDisplay::new();
    display.render();

    for i in 0..32 {
            display.draw_sprite(i, i, 127);
            display.render();
    }
    display.clear();
    display.render();
}
