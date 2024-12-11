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
    use x86_64::{structures::paging::Translate, VirtAddr};

    logo();     // Simple RustOS logo to show that it's loaded.
    
    rust_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        println!("{:?} -> {:?}", virt, phys);
    }

    // Don't touch, below.
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
