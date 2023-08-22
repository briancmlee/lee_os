#![no_std]
#![no_main]
use core::panic::PanicInfo;

use lee_os::{
    serial_println,
    exit_qemu,
    QemuExitCode
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);

    loop {}
}

// Maybe run tests in an isolated thread and exit out

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    assert_eq!(0, 1);
}