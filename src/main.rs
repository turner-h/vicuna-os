#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(vicuna_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use vicuna_os::println;
use vicuna_os::allocator;
use vicuna_os::memory::{self, BootInfoFrameAllocator};
use vicuna_os::task::Task; 
use vicuna_os::task::executor::Executor;
use vicuna_os::task::keyboard;

use bootloader::{ BootInfo, entry_point };
use core::panic::PanicInfo;
use alloc::boxed::Box;
use x86_64::VirtAddr;

entry_point!(kernel_main);

#[no_mangle]
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("Hello World{}", "!");

    vicuna_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed");

    let heap_value = Box::new(41);
    println!("heap value: {:?}", heap_value);

    let mut executor = Executor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    vicuna_os::hlt_loop();
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
    vicuna_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vicuna_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
