mod chip8;
use chip8::Chip8;

use sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{AudioSubsystem, Sdl, VideoSubsystem};

use std::thread;
use std::time;

const OPS_PER_SEC: u64 = 700;
const TIMER_FREQ_UPDATE: f32 = 1. / 60.;

fn init_sdl() -> Result<(Sdl, VideoSubsystem, AudioSubsystem), String> {
    let sdl_ctxt = sdl2::init()?;
    let video_ssys = sdl_ctxt.video()?;
    let audio_ssys = sdl_ctxt.audio()?;
    Ok((sdl_ctxt, video_ssys, audio_ssys))
}

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

    // Create an SDL event system
    let mut event_pump = sdl_ctxt.event_pump().unwrap();

    let mut chip = Chip8::new_sdl(window, &audio_subsys).unwrap();
    chip.load_default_font();

    /*
    if let Err(e) = chip.load_file("programs/delay_timer_test.ch8") {
        println!("{}", e);
        return;
    }
    */
    // Program for testing Input handling
    let _simple_input = [
        0x60, 0x01, // set V0 to 1
        0xE0, 0x9E, // jump instr if Vx == 1 is pressed
        0x12, 0x00, // ask again
        0xF3, 0x29, // Put I to digit 3 sprite
        0xD0, 0x05, // Draw the letter in 0,0
        0x12, 0x0a, // Loop again
    ];

    let _my_program = [
        0xF2, 0x0A, // Wait for an input and put it in A2
        0x00, 0xE0, // Clear screen
        0xF2, 0x29, // select the entered letter sprite
        0xD0, 0x05, // Draw the letter in V0,V0
        0x12, 0x00, // Loop
    ];

    let _test_buzzer = [
        0x60, 0xFF, // V0 <- 255
        0xF0, 0x18, // ST <- V0
        0x61, 0x00, // V1 <- 00 | LABEL
        0x12, 0x04, // Loop  LABEL
    ];

    chip.load_program(&_test_buzzer);

    // Timer
    let mut timer = time::Instant::now();

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

        // Fetch and execute the next instruction.
        let inst = chip.fetch();
        chip.execute(&inst, Some(&event_pump));

        // Update timers
        let now = time::Instant::now();
        let diff = now - timer;
        if diff.as_secs_f32() >= TIMER_FREQ_UPDATE {
            chip.update_timers();
            timer = now;
        }

        // Change buzzer sound if needed.
        chip.refresh_buzzer();

        // Simulate old computer performance.
        thread::sleep(time::Duration::from_millis(1000 / OPS_PER_SEC));
    }
}
