use super::renderer;
use super::Chip8;
use sdl2::keyboard::Scancode;
use sdl2::EventPump;

const DEFAULT_CODES: [Scancode; 16] = [
    Scancode::Num0,
    Scancode::Num1,
    Scancode::Num2,
    Scancode::Num3,
    Scancode::Num4,
    Scancode::Num5,
    Scancode::Num6,
    Scancode::Num7,
    Scancode::Num8,
    Scancode::Num9,
    Scancode::A,
    Scancode::B,
    Scancode::C,
    Scancode::D,
    Scancode::E,
    Scancode::F,
];

pub fn isPressed(event_pmp: &EventPump, value: u8) -> bool {
    event_pmp.keyboard_state().is_scancode_pressed(DEFAULT_CODES[value as usize])
}
