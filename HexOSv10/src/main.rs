#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

mod vga_buffer;


#[no_mangle] 
pub extern "C" fn _start() -> ! {
    
    printlnHexOS!("Hex OS VGA Adapter :: Println macro implemented");

    #[cfg(test)]
    test_main();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    printlnHexOS!("{}",_info);

    loop {}
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    printlnHexOS!("Running {} tests", tests.len());
    for test in tests {
        test();
    }

    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    printlnHexOS!("Trival Assertion ...");
    assert_eq!(1, 1);
    printlnHexOS!("[Ok]");
}

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