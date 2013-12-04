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
pub mod idt;
mod timer;
    mod keyboard;
mod multiboot;
mod paging;
mod usermode;
mod elf;
mod elfloader;

extern { static stack_end: uint; fn jump_usermode(entry: uint) -> ();}

#[no_mangle]
pub extern fn kmain(mbd: *multiboot::multiboot_info, magic:uint) {
    let colours = screen::Colours {fore: 7, back: 0}; // Light gray on black
    unsafe {
        screen::cls(0);
        screen::puts("Welcome to thing!\n", colours);
        screen::puts("* Initializing GDT... ", colours);
        gdt::init();
        screen::puts("DONE\n", colours);
        screen::puts("* Initializing IDT... ", colours);
        idt::init();
        screen::puts("DONE\n", colours);
        screen::puts("* Initializing Paging... ", colours);
        paging::init();
        screen::puts("DONE\n", colours);
        screen::puts("* Multiboot magic: ", colours);
        screen::puthex(magic, colours);
        screen::puts("\n", colours);
        screen::puts("* Initializing GDT... ", colours);
        screen::puts("DONE\n", colours);
        screen::puts("* Initializing Timer... ", colours);
        timer::init();
        screen::puts("DONE\n", colours);
        screen::puts("* Initializing Keyboard... ", colours);
        keyboard::init();
        screen::puts("DONE\n", colours);

        multiboot::print(mbd, colours);
        let mut mmap: *multiboot::memory_map = (*mbd).mmap_addr as *multiboot::memory_map;
        while (mmap as uint) < ((*mbd).mmap_addr + (*mbd).mmap_length) {
            multiboot::mmap_print(mmap, colours);
            mmap = ((mmap as uint) + (*mmap).size + core::mem::size_of::<u32>()) as *multiboot::memory_map;
        }

        let module: *multiboot::module = (*mbd).mods_addr as *multiboot::module;
        let ehdr: *elf::Elf32_Ehdr = (*module).start as *elf::Elf32_Ehdr;
        let entry = elfloader::load_elf(ehdr);
        screen::puts("- Entering usermode now...\n", colours);
        jump_usermode(entry);
    }
    loop{};
}
