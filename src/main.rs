//! writing an os by rust

#![no_std] //don't link rust standard library
#![no_main] //disable all rust-level entry points

use core::panic::PanicInfo;

#[no_mangle] //don't mangle the name of this `fn`
pub extern "C" fn _start() -> ! {
   //this `fn` is the entry point, since the linker looks for a `fn`
   //named `_start` by default
   loop {}
}

///This `fn` is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo,) -> ! { loop {} }
