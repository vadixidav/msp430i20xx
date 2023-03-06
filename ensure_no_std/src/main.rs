// ensure_no_std/src/main.rs
#![no_std]
#![no_main]
#![feature(abi_msp430_interrupt)]

mod interrupts;

use core::panic::PanicInfo;
use msp430_rt::entry;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn main() -> ! {
    /* user code */
    loop {}
}
