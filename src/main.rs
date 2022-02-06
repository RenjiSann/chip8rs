mod chip8;
use chip8::Chip8;
use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::{Sdl, VideoSubsystem};
use std::time::Duration;
use std::{thread, time};

fn init_sdl() -> Result<(Sdl, VideoSubsystem), String> {
    let sdl_ctxt = sdl2::init()?;
    let video_subsystem = sdl_ctxt.video()?;
    Ok((sdl_ctxt, video_subsystem))
}

const OPS_PER_SEC: u64 = 700;

fn main() {
    let sdl_res = init_sdl();
    if let Err(e) = &sdl_res {
        eprintln!("SDL loading error: {}", e);
    }
    let (sdl_ctxt, video_subsys) = sdl_res.unwrap();

    let window = video_subsys
        .window("rust-sdl2 demo", 64, 32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_ctxt.event_pump().unwrap();

    let mut chip = Chip8::newAscii();
    chip.load_default_font();

    if let Err(e) = chip.load_file("programs/ibm_logo.ch8") {
        println!("{}", e);
        return;
    }
    /*
    // Program for testing Input handling
    let my_program = [
        0x60, 0x01, // set V0 to 1
        0xE0, 0x9E, // jump instr if Vx == 1 is pressed
        0x12, 0x00, // ask again
        0xF3, 0x29, // Put I to digit 3 sprite
        0xD0, 0x05, // Draw the letter in 0,0
    ];

    chip.load_program(&my_program);
    */

    // Main loop
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        let inst = chip.fetch();
        chip.execute(&inst);
        thread::sleep(time::Duration::from_millis(100));

        std::thread::sleep(Duration::from_millis(1000 / OPS_PER_SEC));
    }
}
