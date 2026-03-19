#![no_std]
#![cfg_attr(target_arch = "m68k", no_main)]
#![allow(non_camel_case_types, non_snake_case)]

//! # rust-amiga
//! 
//! Write modern Rust code for the Commodora Amiga 500 (Motorola 68000).
//! 
//! This crate provides safe(ish) Rust bindings to AmigaOS libraries:
//! - `dos.library` - File I/O and filesystem operations
//! - `graphics.library` - Low-level graphics and drawing
//! - `intuition.library` - Windowing system and GUI
//! - `audio.device` - Paula chip audio playback
//! - `exec.library` - Memory allocation and system services

#[cfg(target_arch = "m68k")]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Re-export all modules
pub mod ffi;
#[cfg(feature = "dos")]
pub mod amiga_dos;
#[cfg(feature = "graphics")]
pub mod amiga_graphics;
#[cfg(feature = "audio")]
pub mod amiga_audio;
#[cfg(feature = "intuition")]
pub mod amiga_intuition;
pub mod amiga_mem;
pub mod amiga_string;
#[cfg(feature = "exceptions")]
pub mod amiga_exceptions;
pub mod amiga_seq;

// Convenience re-exports
#[cfg(feature = "dos")]
pub use amiga_dos::*;
#[cfg(feature = "graphics")]
pub use amiga_graphics::*;
#[cfg(feature = "audio")]
pub use amiga_audio::*;
#[cfg(feature = "intuition")]
pub use amiga_intuition::*;
pub use amiga_mem::*;
pub use amiga_string::*;
#[cfg(feature = "exceptions")]
pub use amiga_exceptions::*;
pub use amiga_seq::*;

/// Initialize all AmigaOS libraries used by enabled features
#[cfg(target_arch = "m68k")]
pub fn amiga_init() -> bool {
    let mut success = true;
    
    #[cfg(feature = "graphics")]
    if !amiga_graphics::gfx_open() {
        success = false;
    }
    
    #[cfg(feature = "intuition")]
    if !amiga_intuition::intu_open() {
        success = false;
    }
    
    #[cfg(feature = "audio")]
    if !amiga_audio::audio_open() {
        success = false;
    }
    
    success
}

/// Cleanup all AmigaOS libraries
#[cfg(target_arch = "m68k")]
pub fn amiga_cleanup() {
    #[cfg(feature = "audio")]
    amiga_audio::audio_close();
    
    #[cfg(feature = "intuition")]
    amiga_intuition::intu_close();
    
    #[cfg(feature = "graphics")]
    amiga_graphics::gfx_close();
}

/// Entry point wrapper - call this from your main function
#[cfg(target_arch = "m68k")]
#[no_mangle]
pub extern "C" fn _main() -> i32 {
    #[cfg(not(test))]
    {
        extern "C" {
            fn user_main() -> i32;
        }
        unsafe { user_main() }
    }
    #[cfg(test)]
    0
}