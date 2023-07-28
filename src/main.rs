// NEXT: CPU exceptions
// build for a bare metal target
//
// main is a separate executable, thus requiring 
// an entry point and a panic function

#![no_std] // disable std, which is os dependent
#![no_main] // disable rust level entry point
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

// don't mangle this function/give function a name that cannot be recongnized by the linker
// linker: program that generates code into an executable.
// compile for a bare metal target so linker knows not to provide C runtime
// C runtime provides programs with stack management, unwinding, heap allocation, etc

// compiler should use the C calling convention for this function
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");
    println!("Hello world {}", "2");

    #[cfg(test)]
    test_main();

    loop {}
}

/// this function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// entry point for cargo test on main
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn trivial_assertion2() {
    assert_ne!(1, 2);
}
