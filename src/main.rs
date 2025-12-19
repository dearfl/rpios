//! 2. Once finished with architectural setup, the arch code calls `kernel_init()`.

#![feature(format_args_nl)]
#![feature(trait_alias)]
#![no_main]
#![no_std]

mod bsp;
mod console;
mod cpu;
mod panic_wait;
mod print;
mod synchronization;

/// Early init code.
///
/// # Safety
///
/// - Only a single core must be active and running this function.
unsafe fn kernel_init() -> ! {
    use console::console;

    println!("[0] Hello from Rust!");
    println!("[1] Chars written: {}", console().chars_written());

    println!("[2] Stopping here.");
    cpu::wait_forever()
}
