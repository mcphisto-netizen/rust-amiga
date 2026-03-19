#![no_std]
#![no_main]

use rust_amiga::{amiga_dos, amiga_init, amiga_cleanup};

#[no_mangle]
pub extern "C" fn user_main() -> i32 {
    amiga_init();
    amiga_dos::print("Hello from Rust on Amiga 500!\n");
    amiga_cleanup();
    0
}