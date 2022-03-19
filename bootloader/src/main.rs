#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;
use core::arch::asm;

static HELLO: &[u8] = b"Hello from Rust!!!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe { asm!("mov dword ptr [0xb8000], 0x07690748"); }

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}