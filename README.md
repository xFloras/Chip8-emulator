# chip-8

A CHIP-8 emulator written in Rust.

![Rust](https://img.shields.io/badge/Rust-1.75+-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)

## What is CHIP-8?

CHIP-8 is an interpreted programming language from the mid-1970s, originally designed for early microcomputers. It became popular as a platform for simple games and is widely used today as a beginner emulator project due to its simplicity — 35 opcodes, a 64×32 monochrome display, and 4KB of memory.

## Features

- 34 CHIP-8 instructions implemented (excludes `0NNN` — machine language subroutine call, not used by any modern ROM)
- 64×32 pixel display scaled to 640×320 window
- Keyboard input mapped to the standard CHIP-8 hex keypad

## Requirements

- Rust 1.75+
- Cargo

## Building

```bash
git clone https://github.com/yourusername/chip-8
cd chip-8
cargo build --release
```

## Running

```bash
cargo run --release -- --path <path-to-rom>
```

Example:
```bash
cargo run --release -- --path roms/IBM\ Logo.ch8
```

## Keyboard Layout

The original CHIP-8 hex keypad is mapped to the following keys:

| CHIP-8 | Keyboard |
|--------|----------|
| `1 2 3 C` | `1 2 3 4` |
| `4 5 6 D` | `Q W E R` |
| `7 8 9 E` | `A S D F` |
| `A 0 B F` | `Z X C V` |

Keys are position-based (using `PhysicalKey`) so the mapping works correctly regardless of keyboard layout (QWERTY, AZERTY, etc.).

## Project Structure

```
src/
  main.rs          # entry point, wires everything together
  cpu.rs           # CPU struct, instruction execution, opcode fetch
  emulator.rs      # winit ApplicationHandler, render loop, keyboard input
  instructions.rs  # instruction enum and decoder
  screen.rs        # 64x32 pixel buffer and renderer
  delay_timer.rs   # 60Hz timer running on a background thread
```

## Implementation Notes

- **Display** — uses [`pixels`](https://github.com/parasyte/pixels) for GPU-accelerated rendering via `winit`
- **Timers** — delay and sound timers run on independent threads using `Arc<Mutex<u8>>`, decrementing at 60Hz regardless of CPU speed
- **CPU speed** — ticks on every `about_to_wait` event from winit
- **Sprites** — drawn with XOR per the spec; `VF` is set to `1` on collision
- **WaitKey (FX0A)** — implemented by rewinding `PC` and blocking execution until a key is detected in the keys array

## Test ROMs

A good set of public domain test ROMs for verifying correctness:

- [chip8-test-rom](https://github.com/corax89/chip8-test-rom) — visually shows which opcodes pass/fail
- [IBM Logo](https://github.com/loktar00/chip8/blob/master/roms/IBM%20Logo.ch8) — minimal ROM using only a handful of opcodes, good for testing display

## Resources

- [Tobias V. Langhoff's CHIP-8 guide](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/) — plain English explanation of every opcode
- [Instruction set](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set) — complete opcode table
- [Games] (https://github.com/netpro2k/Chip8/tree/master/games) - example games

## License

MIT
