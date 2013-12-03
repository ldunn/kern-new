use elf;
use screen;
use util;
use core;

pub unsafe fn load_elf(header: *elf::Elf32_Ehdr) -> uint
{
    //TODO: Actually verify that we're loading an elf file
    let pheader = (header as u32 + (*header).e_phoff) as *elf::Elf32_Phdr;
    screen::puthex((*pheader).p_vaddr as uint, screen::Colours{fore:3, back:0});
    util::memset_u8((*pheader).p_vaddr as uint, 0xcc, 1);
    util::memcpy((*pheader).p_vaddr as uint, (header as u32 + (*pheader).p_offset) as uint, (*pheader).p_filesz as uint);
    (*header).e_entry as uint
}
