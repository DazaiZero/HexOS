#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(hexosv10::test_runner)]

use core::panic::PanicInfo;
use hexosv10::printlnHexOS;

#[no_mangle] 
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hexosv10::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    printlnHexOS!("Test_println output");
}