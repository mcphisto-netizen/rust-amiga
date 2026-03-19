//! String handling utilities for AmigaOS

use core::fmt::{self, Write};

/// Fixed-size string buffer for Amiga
pub struct StringBuffer<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> StringBuffer<N> {
    pub const fn new() -> Self {
        Self { buf: [0u8; N], len: 0 }
    }
    
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }
    
    pub fn clear(&mut self) { self.len = 0 }
    pub fn remaining(&self) -> usize { N - self.len }
}

impl<const N: usize> Default for StringBuffer<N> {
    fn default() -> Self { Self::new() }
}

impl<const N: usize> Write for StringBuffer<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > N { return Err(fmt::Error); }
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }
}

/// Convert a Rust string to a null-terminated C string in-place
pub fn to_cstring<'a, const N: usize>(
    buf: &'a mut [u8; N], 
    s: &str
) -> Option<&'a mut [u8]> {
    if s.len() >= N { return None; }
    buf[..s.len()].copy_from_slice(s.as_bytes());
    buf[s.len()] = 0;
    Some(&mut buf[..=s.len()])
}

/// Copy a C string to a Rust string (with length limit)
pub unsafe fn from_cstring<'a>(cstr: *const u8, max_len: usize) -> &'a str {
    let mut len = 0;
    while len < max_len && *cstr.add(len) != 0 { len += 1; }
    let slice = core::slice::from_raw_parts(cstr, len);
    core::str::from_utf8_unchecked(slice)
}

/// Compare two C strings
pub unsafe fn strcmp(a: *const u8, b: *const u8) -> i32 {
    let mut i = 0;
    loop {
        let av = *a.add(i);
        let bv = *b.add(i);
        if av != bv { return av as i32 - bv as i32; }
        if av == 0 { return 0; }
        i += 1;
    }
}