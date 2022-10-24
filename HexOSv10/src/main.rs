#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle] 
pub extern "C" fn _start() -> ! {
    
    printlnHexOS!("Hex OS VGA Adapter :: Println macro implemented");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    
    printlnHexOS!("{}",_info);

    loop {}
}

