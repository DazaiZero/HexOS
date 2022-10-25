#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(hexosv10::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use hexosv10::printlnHexOS;

#[no_mangle] 
pub extern "C" fn _start() -> ! {
    
    printlnHexOS!("Hex OS : Custom Testing Framework ");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    printlnHexOS!("{}",_info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    hexosv10::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}


