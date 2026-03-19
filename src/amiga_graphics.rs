//! Bindings to graphics.library for low-level drawing operations

use crate::ffi::*;

// === Opaque Amiga Types ===
pub type RastPort = *mut c_void;
pub type BitMap = *mut c_void;
pub type ViewPort = *mut c_void;

extern "C" {
    #[link_name = "gfx_open"]
    pub fn gfx_open_sys() -> i32;
    
    #[link_name = "gfx_close"]
    pub fn gfx_close_sys();
    
    pub fn gfx_SetAPen(rp: RastPort, pen: UWORD);
    pub fn gfx_Move(rp: RastPort, x: WORD, y: WORD);
    pub fn gfx_Draw(rp: RastPort, x: WORD, y: WORD);
    pub fn gfx_DrawEllipse(rp: RastPort, x: WORD, y: WORD, rx: WORD, ry: WORD);
    pub fn gfx_RectFill(rp: RastPort, x1: WORD, y1: WORD, x2: WORD, y2: WORD);
    pub fn gfx_BltClear(mem: APTR, bytes: ULONG, flags: ULONG);
    pub fn gfx_LoadRGB4(vp: ViewPort, colors: *const UWORD, count: WORD);
}

// === Drawing Mode Constants ===
pub const JAM1: u32 = 0;
pub const JAM2: u32 = 1;
pub const COMPLEMENT: u32 = 2;
pub const INVERSECOMPLEMENT: u32 = 3;
pub const BLT_CLEAR: ULONG = 0;
pub const BLT_SET: ULONG = 1;

// === Safe Rust API ===

pub fn gfx_open() -> bool {
    unsafe { gfx_open_sys() != 0 }
}

pub fn gfx_close() {
    unsafe { gfx_close_sys() }
}

#[inline]
pub fn set_apen(rp: RastPort, pen: u16) {
    unsafe { gfx_SetAPen(rp, pen) }
}

#[inline]
pub fn move_to(rp: RastPort, x: i16, y: i16) {
    unsafe { gfx_Move(rp, x, y) }
}

#[inline]
pub fn draw_line(rp: RastPort, x: i16, y: i16) {
    unsafe { gfx_Draw(rp, x, y) }
}

#[inline]
pub fn draw_ellipse(rp: RastPort, x: i16, y: i16, rx: i16, ry: i16) {
    unsafe { gfx_DrawEllipse(rp, x, y, rx, ry) }
}

#[inline]
pub fn rect_fill(rp: RastPort, x1: i16, y1: i16, x2: i16, y2: i16) {
    unsafe { gfx_RectFill(rp, x1, y1, x2, y2) }
}

#[inline]
pub fn blt_clear(mem: *mut c_void, bytes: u32, flags: u32) {
    unsafe { gfx_BltClear(mem, bytes, flags) }
}

#[inline]
pub fn load_rgb4(vp: ViewPort, colors: &[u16]) {
    unsafe { gfx_LoadRGB4(vp, colors.as_ptr(), colors.len() as WORD) }
}

#[inline]
pub const fn rgb4(r: u8, g: u8, b: u8) -> u16 {
    ((r as u16) & 0x0F) | (((g as u16) & 0x0F) << 4) | (((b as u16) & 0x0F) << 8)
}