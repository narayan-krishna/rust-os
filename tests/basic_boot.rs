// this integration test is a separate executable, thus requiring 
// an entry point and a panic function

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

/// entry point for cargo test
#[no_mangle] // start function
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

// panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    rust_os::test_panic_handler(info);
}

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output")
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("this is a line!");
    }
}
