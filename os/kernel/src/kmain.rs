#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(exclusive_range_pattern)]
#![feature(alloc, allocator_api, global_allocator)]

#[macro_use]
#[allow(unused_imports)]
extern crate alloc;
extern crate pi;
extern crate stack_vec;
extern crate fat32;

pub mod allocator;
pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;
pub mod fs;

#[cfg(not(test))]
use allocator::Allocator;
use fs::FileSystem;
use pi::timer::spin_sleep_ms;
use pi::gpio::Gpio;
use console::kprintln;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn kmain() {
    // ALLOCATOR.initialize();
    
    let mut gpio_05 = Gpio::new(05).into_output();
    loop {
        gpio_05.set();
        spin_sleep_ms(500);

        let mut v = vec![];
        for i in 0..1000 {
            v.push(i);
            kprintln!("{:?}", v);
            kprintln!("i");
        }

        gpio_05.clear();
        spin_sleep_ms(500);
        kprintln!("2");
    }
}
