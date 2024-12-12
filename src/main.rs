#![no_std] // Specify no Standard Library
#![no_main] // Specify no traditional main()
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};

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
    use rust_os::memory;
    use rust_os::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr};

    logo();     // Simple RustOS logo to show that it's loaded.
    
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // map an unused page
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // Write the string "New!" to the screen through the new mapping
    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

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
