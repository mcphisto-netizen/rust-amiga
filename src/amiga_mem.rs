//! Memory management via exec.library

use crate::ffi::*;

extern "C" {
    fn AllocMem(size: ULONG, flags: ULONG) -> APTR;
    fn FreeMem(mem: APTR, size: ULONG);
    
    pub fn nim_alloc(size: usize) -> *mut c_void;
    pub fn nim_dealloc(p: *mut c_void, size: usize);
    pub fn nim_alloc0(size: usize) -> *mut c_void;
}

// === Memory Flags ===
pub const MEMF_ANY: ULONG = 0x00000000;
pub const MEMF_CHIP: ULONG = 0x00000001;
pub const MEMF_FAST: ULONG = 0x00000002;
pub const MEMF_PUBLIC: ULONG = 0x00000004;
pub const MEMF_CLEAR: ULONG = 0x00010000;

// === Safe Allocation API ===

/// Allocate uninitialized memory
pub fn alloc(size: usize) -> Option<*mut u8> {
    let ptr = unsafe { nim_alloc(size) };
    if ptr.is_null() { None } else { Some(ptr as *mut u8) }
}

/// Allocate zero-initialized memory
pub fn alloc_zeroed(size: usize) -> Option<*mut u8> {
    let ptr = unsafe { nim_alloc0(size) };
    if ptr.is_null() { None } else { Some(ptr as *mut u8) }
}

/// Free previously allocated memory
pub unsafe fn dealloc(ptr: *mut u8, size: usize) {
    nim_dealloc(ptr as *mut c_void, size);
}

/// Allocate with specific memory type flags
pub fn alloc_with_flags(size: usize, flags: ULONG) -> Option<*mut u8> {
    let ptr = unsafe { AllocMem(size as ULONG, flags) };
    if ptr.is_null() { None } else { Some(ptr as *mut u8) }
}

// === Simple Stack Allocator ===

pub struct BumpAllocator {
    base: *mut u8,
    current: *mut u8,
    end: *mut u8,
}

impl BumpAllocator {
    pub unsafe fn new(base: *mut u8, size: usize) -> Self {
        Self {
            base,
            current: base,
            end: base.add(size),
        }
    }
    
    pub fn alloc(&mut self, size: usize, align: usize) -> Option<*mut u8> {
        let ptr = self.current as usize;
        let aligned = (ptr + align - 1) & !(align - 1);
        let new_current = aligned + size;
        
        if new_current <= self.end as usize {
            self.current = new_current as *mut u8;
            Some(aligned as *mut u8)
        } else {
            None
        }
    }
    
    pub fn reset(&mut self) {
        self.current = self.base;
    }
}