//! Bindings to intuition.library for windowing and user interface

use crate::ffi::*;
use crate::amiga_graphics::RastPort;

pub type Window = *mut c_void;

extern "C" {
    #[link_name = "intu_open"]
    pub fn intu_open_sys() -> i32;
    
    #[link_name = "intu_close"]
    pub fn intu_close_sys();
    
    pub fn intu_OpenWindow(w: WORD, h: WORD) -> Window;
    pub fn intu_CloseWindow(win: Window);
    pub fn intu_GetRastPort(win: Window) -> RastPort;
    pub fn intu_WaitClose(win: Window);
}

// === Safe Rust API ===

pub fn intu_open() -> bool {
    unsafe { intu_open_sys() != 0 }
}

pub fn intu_close() {
    unsafe { intu_close_sys() }
}

pub fn open_simple_window(width: i16, height: i16) -> Option<Window> {
    let win = unsafe { intu_OpenWindow(width, height) };
    if win.is_null() { None } else { Some(win) }
}

pub fn close_window(win: Window) {
    unsafe { intu_CloseWindow(win) }
}

pub fn get_rastport(win: Window) -> Option<RastPort> {
    let rp = unsafe { intu_GetRastPort(win) };
    if rp.is_null() { None } else { Some(rp) }
}

pub fn wait_for_close(win: Window) {
    unsafe { intu_WaitClose(win) }
}

// === RAII Window Wrapper ===

pub struct AmigaWindow {
    handle: Window,
}

impl AmigaWindow {
    pub fn new(width: i16, height: i16) -> Option<Self> {
        open_simple_window(width, height).map(|h| Self { handle: h })
    }
    
    pub fn rastport(&self) -> Option<RastPort> {
        get_rastport(self.handle)
    }
    
    pub fn wait_close(self) {
        wait_for_close(self.handle);
    }
}

impl Drop for AmigaWindow {
    fn drop(&mut self) {
        if !self.handle.is_null() {
            close_window(self.handle);
        }
    }
}