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
pub mod screen;
mod util;
mod gdt;
pub mod idt;
mod timer;
mod keyboard;
mod multiboot;
mod paging;

#[no_mangle]
pub extern fn kmain(mbd: *multiboot::multiboot_info, magic:uint) {
    let colours = screen::Colours {fore: 7, back: 0}; // Light gray on black
    unsafe {
        memory::init();
        let mut xs = vec::Vec::with_alloc(Alloc);
        xs.push(0xdead);
        xs.push(0xdead);
        xs.push(0xdead);
        screen::cls(0);
        screen::puts("Hello!\n", colours);
        screen::puts("Multiboot magic: ", colours);
        screen::puthex(magic, colours);
        screen::puts("\n", colours);
        screen::puts("- Initializing GDT... ", colours);
        gdt::init();
        screen::puts("DONE\n", colours);
        screen::puts("- Initializing IDT... ", colours);
        idt::init();
        screen::puts("DONE\n", colours);
        screen::puts("- Initializing Paging... ", colours);
        paging::init();
        screen::puts("DONE\n", colours);
        screen::puts("- Initializing timer... ", colours);
        timer::init();
        screen::puts("DONE\n", colours);
        screen::puts("- Initializing keyboard... ", colours);
        keyboard::init();
        screen::puts("DONE\n", colours);

        multiboot::print(mbd, colours);
        let mut mmap: *multiboot::memory_map = (*mbd).mmap_addr as *multiboot::memory_map;
        while (mmap as uint) < ((*mbd).mmap_addr + (*mbd).mmap_length) {
            multiboot::mmap_print(mmap, colours);
            mmap = ((mmap as uint) + (*mmap).size + core::mem::size_of::<u32>()) as *multiboot::memory_map;
        }
    }
    loop{};
}
