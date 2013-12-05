use screen;

#[packed]
pub struct multiboot_info {
    flags: uint,
    mem_lower: uint,
    mem_upper: uint,
    boot_device: uint,
    cmdline: uint,
    mods_count: uint,
    mods_addr: uint,
    x1: uint, // Dummy values
    x2: uint,
    x3: uint,
    x4: uint,
    mmap_length: uint,
    mmap_addr: uint,
    drives_length: uint,
    drives_addr: uint,
    config_table: uint,
    boot_loader_name: uint
}

pub struct memory_map {
    size: uint,
    base_addr_low: uint,
    base_addr_high: uint,
    length_low: uint,
    length_high: uint,
    mmap_type: uint
}

#[packed]
pub struct module {
    start: uint,
    end: uint,
    name: *uint,
    reserved: bool
}

#[no_mangle]
pub unsafe fn print(mbd: *multiboot_info, colours: screen::Colours) {
    screen::puts("* MULTIBOOT INFO *\nflags: ", colours);
    screen::puthex((*mbd).flags as uint,  colours);
    screen::putc('\n' as u8, colours);
    screen::puts("mem_lower: ", colours);
    screen::puthex((*mbd).mem_lower as uint, colours);
    screen::putc('\n' as u8, colours);
    screen::puts("mem_upper: ", colours);
    screen::puthex((*mbd).mem_upper as uint, colours);
    screen::putc('\n' as u8, colours);
    screen::puts("mmap_length: ", colours);
    screen::puthex((*mbd).mmap_length as uint, colours);
    screen::putc('\n' as u8, colours);
    screen::puts("mmap_addr: ", colours);
    screen::puthex((*mbd).mmap_addr as uint, colours);
    screen::putc('\n' as u8, colours);
    screen::puts("boot_loader_name: ", colours);
    screen::putcstr((*mbd).boot_loader_name as *u8, colours);
    screen::putc('\n' as u8, colours);
    screen::puts("mods_count: ", colours);
    screen::putdec((*mbd).mods_count as uint, colours);
    screen::putc('\n' as u8, colours);
    screen::puts("mods_addr: ", colours);
    screen::puthex((*mbd).mods_addr as uint, colours);
    screen::putc('\n' as u8, colours);
}

pub unsafe fn mmap_print(mmap: *memory_map, colours: screen::Colours) {
    screen::puthex((*mmap).base_addr_low, colours);
    screen::puts(" -> ", colours);
    screen::puthex((*mmap).base_addr_low + (*mmap).length_low, colours);
    screen::puts(" : ", colours);
    screen::putdec((*mmap).length_low/1024, colours);
    screen::puts("KB", colours);
    if (*mmap).mmap_type ==  1 {
        screen::puts(" USABLE", colours);
    } else {
        screen::puts(" UNUSABLE", colours);
    }
    screen::putc('\n' as u8, colours);
}
