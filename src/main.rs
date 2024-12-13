#![no_std] // Specify no Standard Library
#![no_main] // Specify no traditional main()
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use x86_64::VirtAddr;
use rust_os::task::{keyboard, simple_executor::SimpleExecutor, Task};

mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info)
}

pub fn logo() {
    println!(r" _____           _    ____   _____ ");
    println!(r"|  __ \         | |  / __ \ / ____|");
    println!(r"| |__) |   _ ___| |_| |  | | (___  ");
    println!(r"|  _  / | | / __| __| |  | |\___ \ ");
    println!(r"| | \ \ |_| \__ \ |_| |__| |____) |");
    println!(r"|_|  \_\__,_|___/\__|\____/|_____/ ");  
}


entry_point!(kernel_main);     // The true start of our kernel

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os::allocator;
    use rust_os::memory::{self, BootInfoFrameAllocator};
    logo();     // Simple RustOS logo to show that it's loaded.
    
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
    .expect("heap initialization failed");

    // Test Code
    let heap_value = Box::new(41);
    println!("heap_value at {:p}", heap_value);

    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));

    let mut executor = SimpleExecutor::new();
    executor.spawn(Task::new(example_task()));
    executor.spawn(Task::new(keyboard::print_keypresses()));
    executor.run();

    // !IMPORTANT! This stuff works, don't touch.
    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os::hlt_loop();
}

// Trivial Assertion is a basic test
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);                   // Assert that 1 = 1
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}
