mod chip8;
use chip8::renderer;
use chip8::Chip8;
use std::{thread, time};

fn main() {
    println!("Hello, world!");

    let mut chip = Chip8::new();
    chip.load_default_font();
    chip.load_file("programs/ibm_logo.ch8");

    loop {
        let inst = chip.fetch();
        chip.execute(&inst);
        thread::sleep(time::Duration::from_millis(100));
    }
}
