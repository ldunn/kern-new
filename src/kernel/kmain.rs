#[crate_type = "lib"];
#[link(name = "kern", vers = "0.0")];

#[no_std];

#[feature(asm)];
#[feature(macro_rules)];

use core::vec;
use core::container::Container;
use core::slice::{Slice, iter, unchecked_get};
use memory::Alloc;

#[path = "../rust-core/core/mod.rs"]
mod core;
mod memory;
mod screen;
mod util;
mod gdt;

#[no_mangle]
pub extern fn kmain() {
    let colours = screen::Colours {fore: 7, back: 0}; // Light gray on black
    unsafe { 
        memory::init();
        let mut xs = vec::Vec::with_alloc(Alloc);
        xs.push(0xdead);
        xs.push(0xdead);
        xs.push(0xdead);
        screen::cls(0);
        screen::puts("Hello!", colours);
        screen::puts("- Initializing GDT... ", colours);
        gdt::init();
        screen::puts("DONE\n", colours);
    }
    loop{};
}
