mod chip8;
use chip8::Chip8;
use std::{thread, time};

fn main() {

    let mut chip = Chip8::new();
    chip.load_default_font();

    /*
    if let Err(e) = chip.load_file("programs/test_opcode.ch8")
    {
        println!("{}",e);
        return;
    }
    */
    let my_program = [
        0x60,
        0x01, // set V0 to 1
        0xE0,
        0x9E, // jump instr if Vx == 1 is pressed
        0x12,
        0x00, // ask again
        0xF3,
        0x29, // Put I to digit 3 sprite
        0xD0,
        0x05, // Draw the letter in 0,0
    ];

    chip.load_program(&my_program);

    loop {
        let inst = chip.fetch();
        chip.execute(&inst);
        thread::sleep(time::Duration::from_millis(100));
    }
}
