// ensure_no_std/src/main.rs
#![no_std]
#![no_main]
#![feature(abi_msp430_interrupt)]

use msp430_rt::{entry, interrupt};

use core::panic::PanicInfo;

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

#[interrupt]
fn DefaultHandler() {}
