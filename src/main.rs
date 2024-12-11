#![no_std] // Specify no Standard Library
#![no_main] // Specify no traditional main()
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
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

#[no_mangle]    // The true start of our kernel
pub extern "C" fn _start() -> ! {
    logo();
    
    #[cfg(test)]
    test_main();

    loop {}
}

// Trivial Assertion is a basic test
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);                   // Assert that 1 = 1
}
