#[no_mangle]
#[cfg(not(test))]
#[lang = "panic_fmt"]
pub extern fn panic_fmt(fmt: ::std::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
    // FIXME: Print `fmt`, `file`, and `line` to the console.
    use console::kprintln;
    kprintln!("panic at:\n
              FILE: {}\n
              LINE: {}\n
              COL: {}\n
              {}", file, line, col, fmt);
    
    // put the core in a low-power state by disabling the clocks in the core
    // while keeping the core powered up
    // and can be woken up by WFE wakeup events
    loop { unsafe { asm!("wfe") } }
}

#[cfg(not(test))] 
#[lang = "eh_personality"] 
pub extern fn eh_personality() {}
