//! Common FFI types and helpers for AmigaOS

use core::ffi::c_void;

// === Re-export c_void for downstream modules ===
pub use core::ffi::c_void;

// === Basic AmigaOS Types ===
pub type APTR = *mut c_void;
pub type BPTR = u32;
pub type STRPTR = *mut u8;
pub type CONST_STRPTR = *const u8;
pub type LONG = i32;
pub type ULONG = u32;
pub type WORD = i16;
pub type UWORD = u16;
pub type BYTE = i8;
pub type UBYTE = u8;

// === Library Base Pointers ===
#[cfg(target_arch = "m68k")]
extern "C" {
    pub static mut SysBase: APTR;
    pub static mut DOSBase: APTR;
    pub static mut GfxBase: APTR;
    pub static mut IntuitionBase: APTR;
    pub static mut AudioBase: APTR;
}

#[cfg(not(target_arch = "m68k"))]
// Stub for cross-compilation on host - using AtomicPtr to avoid static mut UB
pub mod stub_bases {
    use super::*;
    use core::sync::atomic::{AtomicPtr, Ordering};

    static SYS_BASE: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());
    static DOS_BASE: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());
    static GFX_BASE: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());
    static INTUI_BASE: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());
    static AUDIO_BASE: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());

    #[inline]
    pub fn get_sysbase() -> APTR { SYS_BASE.load(Ordering::Relaxed) }
    #[inline]
    pub fn get_dosbase() -> APTR { DOS_BASE.load(Ordering::Relaxed) }
    #[inline]
    pub fn get_gfxbase() -> APTR { GFX_BASE.load(Ordering::Relaxed) }
    #[inline]
    pub fn get_intuibase() -> APTR { INTUI_BASE.load(Ordering::Relaxed) }
    #[inline]
    pub fn get_audiobase() -> APTR { AUDIO_BASE.load(Ordering::Relaxed) }
}
#[cfg(not(target_arch = "m68k"))]
pub use stub_bases::{get_sysbase, get_dosbase, get_gfxbase, get_intuibase, get_audiobase};

// === Alignment Helper ===
#[repr(align(2))]
pub struct Aligned2<T>(pub T);

impl<T> Aligned2<T> {
    #[inline]
    pub const fn new(val: T) -> Self { Self(val) }
    #[inline]
    pub fn into_inner(self) -> T { self.0 }
}

// === Safe wrapper for Amiga pointers ===
#[derive(Copy, Clone, Debug)]
#[repr(transparent)]
pub struct AmigaPtr<T>(pub *mut T);

impl<T> AmigaPtr<T> {
    #[inline]
    pub const fn null() -> Self { Self(core::ptr::null_mut()) }
    #[inline]
    pub fn is_null(self) -> bool { self.0.is_null() }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> Option<&mut T> { self.0.as_mut() }
    #[inline]
    pub unsafe fn as_ref(&self) -> Option<&T> { self.0.as_ref() }
}

impl<T> From<*mut T> for AmigaPtr<T> {
    fn from(ptr: *mut T) -> Self { Self(ptr) }
}

impl<T> From<AmigaPtr<T>> for *mut T {
    fn from(ptr: AmigaPtr<T>) -> Self { ptr.0 }
}