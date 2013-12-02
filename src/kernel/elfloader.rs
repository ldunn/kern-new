use elf;
use screen;
use util;
use core;

pub unsafe fn load_elf(header: *elf::Elf32_Ehdr) -> extern "C" fn()
{
    //TODO: Actually verify that we're loading an elf file
    let pheader = (header as u32 + (*header).e_phoff) as *elf::Elf32_Phdr;
    util::range(0, (*header).e_phnum as uint - 1, |i| {
        if (*pheader).p_type == 1 {
            asm!("mov $0, %eax"::"r"((*pheader).p_vaddr as uint)::"volatile");
            util::memset_u8((*pheader).p_vaddr as uint, 0, (*pheader).p_memsz as uint);
            util::memcpy((*pheader).p_vaddr as uint, (header as u32 + (*pheader).p_offset) as uint, (*pheader).p_filesz as uint);
        }
    });
    core::mem::transmute((*header).e_entry)
}
