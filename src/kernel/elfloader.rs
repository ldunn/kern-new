use elf;
use util;

pub unsafe fn load_elf(header: *elf::Elf32_Ehdr) -> uint
{
    //TODO: Actually verify that we're loading an elf file
    let pheader = (header as u32 + (*header).e_phoff) as *elf::Elf32_Phdr;
    util::memset_u8((*pheader).p_vaddr as uint, 0xcc, 1);
    util::memcpy((*pheader).p_vaddr as uint, (header as u32 + (*pheader).p_offset) as uint, (*pheader).p_filesz as uint);
    (*header).e_entry as uint
}
