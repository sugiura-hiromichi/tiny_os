//! writing an os by rust
#![no_std] //don't link rust standard library
#![no_main] //disable all rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(tiny_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::vec;
use alloc::vec::Vec;
use bootloader::entry_point;
use bootloader::BootInfo;
use core::panic::PanicInfo;
use tiny_os::hlt_loop;
use tiny_os::println;

static HELLO: &str = "hell on world, see you dream";

entry_point!(kernel_main);

///This `fn` is entry point of our kernel.
///By using bootloader::entry_point!, we can name arbitrarily.
///
///# Define
///
///```rust
/// entry_point!(kernel_main);
/// ```
fn kernel_main(boot_info: &'static BootInfo,) -> ! {
   println!("{}", HELLO);
   tiny_os::init();

   use tiny_os::allocator;
   use tiny_os::memory;
   use tiny_os::memory::BootInfoFrameAllocator;
   use x86_64::VirtAddr;

   let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset,);
   let mut mapper = unsafe { memory::init(phys_mem_offset,) };
   let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map,) };

   //allocate value to heap
   allocator::init_heap(&mut mapper, &mut frame_allocator,).expect("heap inititalization failed",);
   let heap_value = Box::new(44,);
   println!("heap_value at {:p}", heap_value);

   //create dynamic sized vector
   let mut vec = Vec::new();
   for i in 0..500 {
      vec.push(i,);
   }
   println!("vec at {:p}", vec.as_slice());

   //create reference counted vector -> free if reference count become 0
   let reference_counted = Rc::new(vec![1, 2, 3],);
   let cloned_reference = reference_counted.clone();
   println!("current reference count is {}", Rc::strong_count(&cloned_reference));
   core::mem::drop(reference_counted,);
   println!("reference count is {} now", Rc::strong_count(&cloned_reference));

   #[cfg(test)]
   test_main();

   println!("didn't crash!");
   hlt_loop();
}

///This `fn` is called on panic
#[cfg(not(test))] //new attribute
#[panic_handler]
fn panic(info: &PanicInfo,) -> ! {
   println!("{}", info);
   hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo,) -> ! { tiny_os::test_panic_handler(info,) }

#[test_case]
fn trivial_assertion() {
   assert_eq!(1, 1);
}
