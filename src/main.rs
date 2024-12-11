#![no_std] // Specify no Standard Library
#![no_main] // Specify no traditional main()
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
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

#[cfg(test)]        // Test Panic Handler (prints to serial out)
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
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


// QEMU EXIT
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

// Test Traits
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]")
    }
}
