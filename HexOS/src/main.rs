#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![reexport_test_harness_main = "test_main"]
#![feature(custom_test_frameworks)]
#![test_runner(HexOS::test_runner)]


use core::panic::PanicInfo;
use HexOS::println;

/// This function is called on panic.
#[cfg(not(test))] // new attribute
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
   HexOS::test_panic_handler(info)
}




#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    HexOS::init(); 

    fn stack_overflow() {
        stack_overflow(); // for each recursion, the return address is pushed
    }

    // trigger a stack overflow
    stack_overflow();

    #[cfg(test)]
    test_main();


    println!("It did not crash!");
    loop {}
}


//////////////



#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) { // new
    println!("Running {} tests", tests.len());
    for test in tests {
        test.run(); // new
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    /* print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]"); */

    //serial_print!("trivial assertion... ");
    assert_eq!(1, 1);
    //serial_println!("[ok]");
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        println!("{}...\t", core::any::type_name::<T>());
        self();
        println!("[ok]");
    }
}

//////////////
/// 

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