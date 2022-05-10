use sdl2::audio::{AudioCallback, AudioDevice, AudioSpecDesired};
use sdl2::AudioSubsystem;

const BUZZER_VOLUME: f32 = 0.02;

/**
 * Structure for generating the sound.
 */
struct SquareWave {
    phase_inc: f32,
    phase: f32,
    volume: f32,
}

impl AudioCallback for SquareWave {
    type Channel = f32;

    fn callback(&mut self, out: &mut [f32]) {
        // Generate a square wave
        for x in out.iter_mut() {
            *x = if self.phase <= 0.5 {
                self.volume
            } else {
                -self.volume
            };
            self.phase = (self.phase + self.phase_inc) % 1.0;
        }
    }
}

/**
 * @brief This structure holds the logic for Chip8's sound system.
 */
pub struct Buzzer {
    device: AudioDevice<SquareWave>,
}

impl Buzzer {
    // Create a Buzzer object.
    pub fn new(audio: &AudioSubsystem) -> Result<Self, String> {
        // Create an object for the
        let desired_spec = AudioSpecDesired {
            freq: Some(44100),
            channels: Some(1), // mono
            samples: None,     // default sample size
        };

        Ok(Buzzer {
            device: audio.open_playback(None, &desired_spec, |spec|
                // initialize the audio callback
                SquareWave {
                    phase_inc: 440.0 / spec.freq as f32,
                    phase: 0.5,
                    volume: BUZZER_VOLUME,
                })?,
        })
    }

    /**
     * @brief Start buzzing
     */
    pub fn start(&self) {
        self.device.resume();
    }

    /**
     * @brief Stop buzzing
     */
    pub fn stop(&self) {
        self.device.pause();
    }
}
