# rust-amiga

Write modern Rust code for the Commodora Amiga 500 (Motorola 68000).

## Features

- `dos.library` - File I/O and filesystem operations
- `graphics.library` - Low-level graphics and drawing
- `intuition.library` - Windowing system and GUI
- `audio.device` - Paula chip audio playback
- `exec.library` - Memory allocation and system services

## Build Requirements

- Rust nightly with m68k target support
- VBCC compiler (`vc`) for m68k
- GNU `ar` for static library creation (included in most m68k cross-toolchains)
- Amiga NDK headers in `c/` directory

## Build Instructions

```bash
# Set environment variables
export VC=/opt/vbcc/bin/vc
export AR=ar
export PATH=/opt/vbcc/bin:$PATH

# Build for Amiga
cargo build --target m68k-amigaos.json --release

# Test on host (cross-compilation check)
cargo check --tests

# Build example
cargo build --target m68k-amigaos.json --example hola --release