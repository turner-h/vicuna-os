#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::format;
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
use core::panic::PanicInfo;
use alloc::boxed::Box;
use alloc::string::{ToString, String};
use alloc::vec::Vec;
use x86_64::VirtAddr;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    //println!("Hello World{}", "!");
    serial_println!("we made it to kernel_main");

    kernel::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_regions)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

    let heap_value = Box::new(41);
    println!("heap value: {:?}", heap_value); 

    unsafe{ 
        //ata::send_write_master(0x0FFFFFFE, 1); 
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
    //executor.spawn(Task::new(example_task()));
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
