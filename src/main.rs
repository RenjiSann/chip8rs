mod chip8;
use chip8::Chip8;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::{AudioSubsystem, EventPump, VideoSubsystem};

use std::thread;
use std::time;

const OPS_PER_SEC: u64 = 700;
const TIMER_FREQ_UPDATE: f32 = 1. / 60.;

/**
 * @brief Initializes SDL with video and audio system.
 */
fn init_sdl() -> Result<(VideoSubsystem, AudioSubsystem, EventPump), String> {
    let sdl_ctxt = sdl2::init()?;
    let video_ssys = sdl_ctxt.video()?;
    let audio_ssys = sdl_ctxt.audio()?;
    let event_pump = sdl_ctxt.event_pump()?;
    Ok((video_ssys, audio_ssys, event_pump))
}

fn main() {
    // Check arguments to get the program to load.
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments !");
        panic!();
    }

    // Initialize SDL.
    let sdl_res = init_sdl();
    if let Err(e) = &sdl_res {
        eprintln!("SDL loading error: {}", e);
        panic!();
    }
    let (video_subsys, audio_subsys, mut event_pump) = sdl_res.unwrap();

    // Create a window for SDL.
    let window = video_subsys
        .window("rust-sdl2 demo", 800, 400)
        .position_centered()
        .build()
        .unwrap();

    // Create the Chip8 emulator.
    let mut chip = Chip8::new_sdl(window, &audio_subsys).unwrap();
    chip.load_default_font();

    // Load the program given in arguments.
    if let Err(e) = chip.load_file(&args[1]) {
        eprintln!("{}", e);
        panic!();
    }

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
