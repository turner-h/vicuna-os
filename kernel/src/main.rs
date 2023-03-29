#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use kernel::framebuffer;
use kernel::println;
use kernel::allocator;
use kernel::memory::{self, BootInfoFrameAllocator};
use kernel::serial_println;
use kernel::task::Task; 
use kernel::task::executor::Executor;
use kernel::task::keyboard;
use kernel::ata;

use bootloader_api::{ BootInfo, entry_point };
use bootloader_api::config::{ BootloaderConfig, Mapping };
use bootloader_api::info::FrameBufferInfo;

use core::slice;
use core::panic::PanicInfo;

use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;
use alloc::format;

use x86_64::VirtAddr;
use x86_64::structures::port::{ PortRead };

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    kernel::init();

    serial_println!("we in that kernel_main fr");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_regions)
    };

    let info = FrameBufferInfo::from(boot_info.framebuffer.as_ref().unwrap().info());
    let buf_ptr = boot_info.framebuffer.as_mut().unwrap().buffer().as_ptr();
    let buf_len = boot_info.framebuffer.as_ref().unwrap().buffer().len();
    let buf: &'static mut [u8] = unsafe { slice::from_raw_parts_mut(buf_ptr.cast_mut(), buf_len) };

    framebuffer::init(buf.as_mut(), info);

    println!("Hello World{}", "!");

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

    let heap_value = Box::new(41);
    println!("heap value: {:?}", heap_value);

    unsafe{
        //ata::send_write_master(0x00000000, 1);
        let buf = ata::send_read_master(0, 1); 
        let mut hex_str = "".to_string();

        let mut split_buf: Vec<u8> = Vec::new();
        for val in buf {
            let val_bytes = val.to_be_bytes();
            split_buf.push(val_bytes[0]);
            split_buf.push(val_bytes[1]);
        }

        for val in &split_buf[0x1BE..0x1FE]{
            hex_str = hex_str + format!("{:X}", val).to_string().as_str() + " ";
        }
        println!("{:?}", hex_str);
    }

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    kernel::hlt_loop();
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async value: {}", number);
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        serial_println!("Port A: {:b}", <u16 as PortRead>::read_from_port(0x92));
        serial_println!("Port B: {:b}", <u16 as PortRead>::read_from_port(0x61));
    }
    serial_println!("{}", info);
    println!("{}", info);
    kernel::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
