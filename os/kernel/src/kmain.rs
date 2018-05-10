#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(compiler_builtins_lib, pointer_methods)]
#![no_builtins]

extern crate pi;
extern crate stack_vec;
extern crate compiler_builtins;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

use pi::timer::spin_sleep_ms;
// use pi::timer::spin_sleep_us;
use pi::gpio::Gpio;

#[no_mangle]
pub extern "C" fn kmain() {
    let mut gpio_05 = Gpio::new(05).into_output();
    let mut gpio_06 = Gpio::new(06).into_output();
    let mut gpio_13 = Gpio::new(13).into_output();
    loop {
        gpio_05.set();
        spin_sleep_ms(300);
        // gpio_06.set();
        // spin_sleep_ms(300);
        // gpio_13.set();
        // spin_sleep_ms(300);

        gpio_05.clear();
        spin_sleep_ms(300);
        // gpio_06.clear();
        // spin_sleep_ms(300);
        // gpio_13.clear();
        // spin_sleep_ms(300);
    }
}
