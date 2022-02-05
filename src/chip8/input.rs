use super::Chip8;
use sdl2::keyboard::Scancode;
use sdl2::keyboard::Keycode;
use std::collections::HashSet;

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

pub fn isPressed(chip: &Chip8, value: u8) -> bool {
    false
}
