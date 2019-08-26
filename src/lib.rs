//! Set panicking behaviour to an infinite loop.
//!
//! Unlike other panic implementations, this one relies on intrinsics specific to MSP430, so it
//! actually builds on MSP430 targets. The implementation was originally extracted from msp430-rt
//! crate.

#![deny(missing_docs)]
#![deny(warnings)]
#![no_std]

extern crate msp430;

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // Disable interrupts to prevent further damage.
    msp430::interrupt::disable();
    loop {
        // Prevent optimizations that can remove this loop.
        msp430::asm::barrier();
    }
}
