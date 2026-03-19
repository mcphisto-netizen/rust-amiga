//! Bindings to dos.library for file I/O and filesystem operations

use crate::ffi::*;

extern "C" {
    pub fn nim_open(name: STRPTR, mode: LONG) -> BPTR;
    pub fn nim_read(fh: BPTR, buf: *mut c_void, len: LONG) -> LONG;
    pub fn nim_write(fh: BPTR, buf: *const c_void, len: LONG) -> LONG;
    pub fn nim_close(fh: BPTR) -> LONG;
    pub fn nim_output() -> BPTR;
    pub fn nim_lock(name: STRPTR, access: LONG) -> BPTR;
    pub fn nim_unlock(lock: BPTR);
    pub fn nim_examine(lock: BPTR, fib: *mut c_void) -> LONG;
    pub fn nim_nextdosentry(lock: BPTR, fib: *mut c_void, mode: LONG) -> LONG;
}

// === File Access Modes ===
pub const MODE_OLDFILE: LONG = 1005;
pub const MODE_NEWFILE: LONG = 1006;
pub const MODE_READWRITE: LONG = 1004;
pub const MODE_APPEND: LONG = 1007;

// === Lock Access Types ===
pub const ACCESS_READ: LONG = -2;
pub const ACCESS_WRITE: LONG = -1;

// === Safe Rust API ===

/// Open a file on AmigaDOS
pub unsafe fn open(name: *const u8, mode: LONG) -> Option<BPTR> {
    let result = nim_open(name as STRPTR, mode);
    if result != 0 { Some(result) } else { None }
}

/// Read from an open file handle
pub unsafe fn read(fh: BPTR, buf: *mut u8, len: usize) -> Option<usize> {
    let result = nim_read(fh, buf as *mut c_void, len as LONG);
    if result >= 0 { Some(result as usize) } else { None }
}

/// Write to an open file handle
pub unsafe fn write(fh: BPTR, buf: *const u8, len: usize) -> Option<usize> {
    let result = nim_write(fh, buf as *const c_void, len as LONG);
    if result >= 0 { Some(result as usize) } else { None }
}

/// Close an open file handle
pub fn close(fh: BPTR) -> bool {
    unsafe { nim_close(fh) != 0 }
}

/// Get the standard output file handle
pub fn output() -> Option<BPTR> {
    let result = unsafe { nim_output() };
    if result != 0 { Some(result) } else { None }
}

/// Print a string to standard output
pub fn print(s: &str) {
    if let Some(out) = output() {
        unsafe {
            let _ = write(out, s.as_ptr(), s.len());
        }
    }
}

/// Print a string with newline to standard output
#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut buf = $crate::amiga_string::StringBuffer::<256>::new();
        let _ = write!(&mut buf, $($arg)*);
        let _ = write!(&mut buf, "\n");
        $crate::amiga_dos::print(buf.as_str());
    }};
}