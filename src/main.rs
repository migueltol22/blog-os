#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(_info)
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("HELLO WORLD {}", "!");

    blog_os::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();
    loop {}
}
