//! writing an os by rust
#![no_std] //don't link rust standard library
#![no_main] //disable all rust-level entry points

use core::panic::PanicInfo;

static HELLO: &[u8] = b"hell on world, see you dream";

///this `fn` is the entry point, since the linker looks for a `fn` named `_start` by default
#[no_mangle] //don't mangle the name of this `fn`
pub extern "C" fn _start() -> ! {
   let vga_buf = 0xb8000 as *mut u8;
   for (i, &byte,) in HELLO.iter().enumerate() {
      unsafe {
         *vga_buf.offset(i as isize * 2,) = byte;
         *vga_buf.offset(i as isize * 2 + 1,) = 0xb;
      }
   }

   loop {}
}

///This `fn` is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo,) -> ! { loop {} }
