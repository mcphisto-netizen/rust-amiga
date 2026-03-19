//! Exception handling via setjmp/longjmp

use crate::ffi::*;
use core::cell::UnsafeCell;

pub type JmpBuf = [NI32; 20];
pub type PBuf = *mut JmpBuf;
pub type NI32 = u32;

extern "C" {
    pub fn nim_setjmp(env: PBuf) -> NI32;
    pub fn nim_longjmp(env: PBuf, val: NI32) -> !;
}

// Target path: static mut is benign on single-tasking Amiga
#[cfg(target_arch = "m68k")]
static mut ENV_STACK: [*mut JmpBuf; 16] = [core::ptr::null_mut(); 16];

#[cfg(target_arch = "m68k")]
static mut ENV_TOP: usize = 0;

// Host path: use UnsafeCell for soundness
#[cfg(not(target_arch = "m68k"))]
static ENV_STACK: UnsafeCell<[*mut JmpBuf; 16]> = UnsafeCell::new([core::ptr::null_mut(); 16]);

#[cfg(not(target_arch = "m68k"))]
static ENV_TOP: UnsafeCell<usize> = UnsafeCell::new(0);

// Accessors
#[cfg(target_arch = "m68k")]
unsafe fn get_env_top() -> usize { ENV_TOP }

#[cfg(target_arch = "m68k")]
unsafe fn set_env_top(val: usize) { ENV_TOP = val; }

#[cfg(target_arch = "m68k")]
#[allow(static_mut_refs)]
unsafe fn get_env_stack() -> *mut [*mut JmpBuf; 16] { 
    core::ptr::addr_of_mut!(ENV_STACK) 
}

// Host accessors
#[cfg(not(target_arch = "m68k"))]
unsafe fn get_env_top() -> usize { *ENV_TOP.get() }

#[cfg(not(target_arch = "m68k"))]
unsafe fn set_env_top(val: usize) { *ENV_TOP.get() = val; }

#[cfg(not(target_arch = "m68k"))]
unsafe fn get_env_stack() -> *mut [*mut JmpBuf; 16] { ENV_STACK.get() }

pub unsafe fn push_env(env: *mut JmpBuf) {
    let top = get_env_top();
    if top < 16 {
        let stack = get_env_stack();
        (*stack)[top] = env;
        set_env_top(top + 1);
    }
}

pub unsafe fn pop_env() {
    let top = get_env_top();
    if top > 0 {
        let stack = get_env_stack();
        (*stack)[top - 1] = core::ptr::null_mut();
        set_env_top(top - 1);
    }
}

pub fn current_env() -> *mut JmpBuf {
    unsafe {
        let top = get_env_top();
        if top > 0 {
            let stack = get_env_stack();
            (*stack)[top - 1]
        } else {
            core::ptr::null_mut()
        }
    }
}

#[macro_export]
macro_rules! try_block {
    ($body:expr, $handler:expr) => {{
        let mut env: $crate::amiga_exceptions::JmpBuf = [0; 20];
        unsafe {
            $crate::amiga_exceptions::push_env(&mut env);
            if $crate::amiga_exceptions::setjmp(&mut env) == 0 {
                let result = (|| $body)();
                $crate::amiga_exceptions::pop_env();
                Some(result)
            } else {
                $crate::amiga_exceptions::pop_env();
                $handler;
                None
            }
        }
    }};
}

pub fn throw(code: NI32) -> ! {
    unsafe {
        let env = current_env();
        if !env.is_null() {
            nim_longjmp(env, if code == 0 { 1 } else { code });
        }
        loop {}
    }
}

pub fn setjmp(env: &mut JmpBuf) -> NI32 {
    unsafe { nim_setjmp(env) }
}

pub fn longjmp(env: &mut JmpBuf, val: NI32) -> ! {
    unsafe { nim_longjmp(env as *mut JmpBuf, val) }
}

pub const EX_NONE: NI32 = 0;
pub const EX_GENERIC: NI32 = 1;
pub const EX_OUT_OF_MEMORY: NI32 = 2;
pub const EX_FILE_NOT_FOUND: NI32 = 3;
pub const EX_IO_ERROR: NI32 = 4;