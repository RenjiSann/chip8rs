mod chip8;
use chip8::renderer;
use chip8::Chip8;
use std::{thread, time};

fn main() {

    let mut chip = Chip8::new();
    chip.load_default_font();
    if let Err(e) = chip.load_file("programs/test_opcode.ch8")
    {
        println!("{}",e);
        return;
    }

    loop {
        let inst = chip.fetch();
        chip.execute(&inst);
        thread::sleep(time::Duration::from_millis(100));
    }
}
