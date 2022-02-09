mod chip8;
use chip8::Chip8;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::{AudioSubsystem, Sdl, VideoSubsystem};

use std::thread;
use std::time::Duration;

fn init_sdl() -> Result<(Sdl, VideoSubsystem, AudioSubsystem), String> {
    let sdl_ctxt = sdl2::init()?;
    let video_ssys = sdl_ctxt.video()?;
    let audio_ssys = sdl_ctxt.audio()?;
    Ok((sdl_ctxt, video_ssys, audio_ssys))
}

const OPS_PER_SEC: u64 = 700;

fn main() {
    let sdl_res = init_sdl();
    if let Err(e) = &sdl_res {
        eprintln!("SDL loading error: {}", e);
    }
    let (sdl_ctxt, video_subsys, audio_subsys) = sdl_res.unwrap();

    let window = video_subsys
        .window("rust-sdl2 demo", 800, 400)
        .position_centered()
        .build()
        .unwrap();

    let mut event_pump = sdl_ctxt.event_pump().unwrap();

    let mut chip = Chip8::new_sdl(window).unwrap();
    //let mut chip = Chip8::new_ascii();
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
        // Check events
        
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
        
        if chip.has_exited() {
            break 'running;
        }

        dbg!(event_pump.keyboard_state().is_scancode_pressed(Scancode::A));

        let inst = chip.fetch();
        chip.execute(&inst);

        thread::sleep(Duration::from_millis(1000 / OPS_PER_SEC));
        //thread::sleep(Duration::from_millis(1000));
    }
}
