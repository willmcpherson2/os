#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

mod gdt;
mod interrupt;
mod serial;
mod vga;

use core::panic::PanicInfo;
use x86_64::instructions::port::Port;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serialn!("{}", info);
    exit_qemu(QemuExitCode::Failed);
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    serialn!("Hello serial world{}", "!");
    println!("Hello VGA world{}", "!");

    gdt::init();
    interrupt::init();

    #[cfg(test)]
    test_main();

    serialn!("We did not crash!");
    println!("We did not crash!");

    hlt_loop();
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
    hlt_loop();
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    serialn!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    serialn!("Tests passed");
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn test_math() {
    assert_eq!(1, 1);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
