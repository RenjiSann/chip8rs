---
Author: Dorian 'Renji' Péron
Date: 02-2022
---
# chip8rs

---

A small chip8 emulator written in Rust.

In order to train myself with traits and because I didn't want to
create a graphical interface from the beginning, I first made a structure
responsible for writing the screen out in the terminal.
Then I used SDL2 to create a graphical window.

## TODO

- [x] Implement a graphical interface instead of ascii drawing
- [ ] Handling inputs
- [ ] Handling timers
- [ ] Handling sound
- [x] Debug info printing

I also try to organize my code as well as possible, to learn how the module and crates system works.

## What I learnt from this project

- Rust module system. Especially I understood the difference between `use` and `mod`.
- Rust bindings to SDL2 library.
- Rust traits and generics.

## Sources

If you are interested in making your own chip8 implementation, here is the  [guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) I used.

I also got inspiration from [this implementation](https://github.com/Clotildelevou/CHIP8-emulator).
