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
use pi::uart::MiniUart;
use std::fmt::Write;

#[no_mangle]
pub extern "C" fn kmain() {
    let mut gpio_05 = Gpio::new(05).into_output();
    let mut echo_miniuart = MiniUart::new();

    loop {
        let this_byte = echo_miniuart.read_byte();
        echo_miniuart.write_byte(this_byte);
        echo_miniuart.write_str("<-");
        gpio_05.set();
        spin_sleep_ms(300);
        gpio_05.clear();
    }
}
