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
use pi::atags::*;
use console::kprintln;

#[cfg(not(test))]
#[global_allocator]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

pub static FILE_SYSTEM: FileSystem = FileSystem::uninitialized();

#[no_mangle]
#[cfg(not(test))]
pub extern "C" fn kmain() {
    spin_sleep_ms(5000);

    // ALLOCATOR.initialize();

    check_atags();

    let mut gpio_05 = Gpio::new(05).into_output();
    loop {
        gpio_05.set();
        spin_sleep_ms(500);
        gpio_05.clear();
        spin_sleep_ms(500);
        // kprintln!("2");
    }
}

fn check_atags() {
    kprintln!("iterating ATAGS...");

    let mut this_atags = Atags::get();
    let mut this_atag = this_atags.current();

    loop {
        match this_atag {
            // visit pi::atags::atag::Atag::Core through pi::atags::*;
            // pi::atags::Atag::Core(core) => 
            //     kprintln!("atag core: {:#?}", core),
            Atag::Core(core) => 
                kprintln!("atag core: {:#?}", core),
            Atag::Mem(mem) =>
                kprintln!("atag mem: {:#?}", mem),
            Atag::Cmd(cmd) =>
                kprintln!("atag cmd: {:#?}", cmd),
            Atag::Unknown(id) =>
                kprintln!("atag unknown: {:#?}", id),
            Atag::None =>
                kprintln!("atag none"),
        }
        match this_atags.next() {
            Some(next_atag) => {
                this_atag = next_atag;
            },
            None => break,
        }
    }
}
