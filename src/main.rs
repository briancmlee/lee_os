#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(lee_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use lee_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    lee_os::init();

    x86_64::instructions::interrupts::int3();

    println!("Instruction after breakpoint interrupt");

    #[cfg(test)]
    test_main();

    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        x86_64::instructions::hlt();
    }
}

#[cfg(test)]

#[test_case]
fn trivial_test() {
    assert_eq!(1, 1);
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lee_os::test_panic_handler(info);
}