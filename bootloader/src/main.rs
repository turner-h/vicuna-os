#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    
    unsafe {
        *vga_buffer = b'R';
        *vga_buffer.offset(1) = 0x0b;
        *vga_buffer.offset(2) = b'u';
        *vga_buffer.offset(3) = 0x0b;
        *vga_buffer.offset(4) = b's';
        *vga_buffer.offset(5) = 0x0b;
        *vga_buffer.offset(6) = b't';
        *vga_buffer.offset(7) = 0x0b;
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}