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

pub fn is_pressed(ep: &EventPump, value: u8) -> bool {
    ep.keyboard_state()
        .is_scancode_pressed(DEFAULT_CODES[value as usize])
}

pub fn get_scancode(ep: &EventPump) -> Option<u8> {
    for sc in ep.keyboard_state().pressed_scancodes() {
        if let Some(x) = DEFAULT_CODES.iter().position(|elm| elm == &sc) {
            return Some(x as u8);
        }
    }
    None
}
