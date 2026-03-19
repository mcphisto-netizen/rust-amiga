//! Bindings to audio.device for Paula chip audio playback

use crate::ffi::*;

extern "C" {
    #[link_name = "audio_open"]
    pub fn audio_open_sys() -> i32;
    
    #[link_name = "audio_close"]
    pub fn audio_close_sys();
    
    pub fn audio_alloc(ch: UWORD) -> i32;
    pub fn audio_free(ch: UWORD);
    pub fn audio_play(ch: UWORD,  APTR, len: ULONG, period: UWORD);
}

// === Audio Channel Constants ===
pub const LEFT0: UWORD = 0x0001;
pub const RIGHT0: UWORD = 0x0002;
pub const LEFT1: UWORD = 0x0004;
pub const RIGHT1: UWORD = 0x0008;
pub const LEFT2: UWORD = 0x0010;
pub const RIGHT2: UWORD = 0x0020;
pub const LEFT3: UWORD = 0x0040;
pub const RIGHT3: UWORD = 0x0080;

// === Safe Rust API ===

pub fn audio_open() -> bool {
    unsafe { audio_open_sys() != 0 }
}

pub fn audio_close() {
    unsafe { audio_close_sys() }
}

pub fn alloc_channel(channels: u16) -> bool {
    unsafe { audio_alloc(channels) != 0 }
}

pub fn free_channel(channels: u16) {
    unsafe { audio_free(channels) }
}

pub unsafe fn play_sample(channels: u16,  *const u8, len: u32, period: u16) {
    audio_play(channels, data as APTR, len, period);
}

pub fn play_sample_slice(channels: u16,  &[u8], period: u16) {
    unsafe {
        play_sample(channels, data.as_ptr(), data.len() as u32, period);
    }
}