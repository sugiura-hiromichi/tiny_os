//! writing an os by rust
#![no_std] //don't link rust standard library
#![no_main] //disable all rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(tiny_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tiny_os::println;

static HELLO: &str = "hell on world, see you dream";

///this `fn` is the entry point, since the linker looks for a `fn` named `_start` by default
#[no_mangle] //don't mangle the name of this `fn`
pub extern "C" fn _start() -> ! {
   println!("{}", HELLO);

   tiny_os::init();
   fn stack_overflow() {
      //for each recursion, the return address is pushed
      stack_overflow();
   }

   stack_overflow();

   #[cfg(test)]
   test_main();

   println!("didn't crash!");
   loop {}
}

///This `fn` is called on panic
#[cfg(not(test))] //new attribute
#[panic_handler]
fn panic(info: &PanicInfo,) -> ! {
   println!("{}", info);
   loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo,) -> ! { tiny_os::test_panic_handler(info,) }

#[test_case]
fn trivial_assertion() {
   assert_eq!(1, 1);
}
