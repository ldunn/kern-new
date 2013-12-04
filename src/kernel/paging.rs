#[no_std]
#[no_core]

use memory;
use util;
use core::mem::size_of;
use idt;
use screen;

static mut first_page_table: *mut [uint, ..1024] = 0 as *mut [uint, ..1024];
static mut frames: *mut uint = 0 as *mut uint;
static mut num_frames: uint = 0;
static mut current_dir: *mut page_directory = 0 as *mut page_directory;

pub struct page_directory {
    tables: [uint, ..1024],
    tables_phys: [uint, ..1024],
    phys_addr: *mut u8 // the address of tables_phys
}

extern {
    fn enable_paging(directory: *mut [uint, ..1024]);
}
#[fixed_stack_segment]
pub unsafe fn init()
{
    let page_dir_addr = memory::kernel_malloc(size_of::<page_directory>(), true);
    util::memset_u8(page_dir_addr as uint, 0, size_of::<page_directory>());
    let page_dir = page_dir_addr as *mut page_directory;
    current_dir = page_dir;
    
    num_frames = 0x100000;
    let frame_addr = memory::kernel_malloc(num_frames/32, true);
    util::memset_u8(frame_addr as uint, 0, num_frames/32);
    frames = frame_addr as *mut uint;

    let mut i = 0;
    while i < 0x1000000 {
        let x = get_page(i, true, page_dir);
        *x = alloc_frame(*x, true, true);
        i += 0x1000;
    }
    let x = get_page(0xfffff000, true, page_dir);
    *x = (page_dir_addr as uint) | 0x7;
    set_frame(x as uint);
    enable_paging((&mut (*page_dir).tables_phys) as *mut [uint,..1024]);
    idt::register_interrupt_handler(14, handler);
}

pub unsafe fn set_frame(addr: uint) {
    let frame = addr/0x1000;
    let index = frame/32;
    let offset = frame % 32;
    *(((frames as uint) + index*4) as *mut uint) |= (1 << offset);
}

unsafe fn clear_frame(addr: uint) {
    let frame = addr/0x1000;
    let index = frame/32;
    let offset = frame % 32;
    *((frame as uint + index) as *mut uint) &= !(1 << offset);
}

unsafe fn first_frame() -> uint {
    let mut i = 0;
    let mut j = 0;
    while i <= num_frames/32 {
        if *((frames as uint + i*4) as *mut uint) != 0xffffffff {
            while j <= 32 {
                let test = 1 << j;
                if *((frames as uint + i*4) as *mut uint) & test == 0 {
                    return i*32+j;
                }
                j += 1;
            }
        }
        j = 0;
        i += 1;
    }
    0
}

unsafe fn alloc_frame(page: uint, is_kernel: bool, is_writeable: bool) -> uint{
    let mut new_page = page;
    if (page >> 12 != 0) { // page already has a frame
        return page
    }
    let idx = first_frame();
    set_frame(idx*0x1000);
    new_page = idx*0x1000;
    new_page |= 1; // mark as present
    if is_writeable { new_page |= 2; } // mark as writeable
    if !is_kernel { new_page |= 4; } // mark as user
    new_page
}

unsafe fn get_page(addr: uint, make: bool, dir: *mut page_directory) -> *mut uint {
    let address = addr / 0x1000; // Create an index into the page directory
    let table_index = address / 1024;
    if((*dir).tables[table_index] != 0) { // This table exists
       (((*dir).tables[table_index]) + (address%1024)*size_of::<uint>()) as *mut uint
    } else if make {
        let table_address = memory::kernel_malloc(size_of::<uint>()*1024, true);
        util::memset_u8(table_address as uint, 0, 0x1000);
        (*dir).tables[table_index] = table_address as uint;
        (*dir).tables_phys[table_index] = table_address as uint | 0x7; // present, r/w, user
        (((*dir).tables[table_index]) + (address%1024)*size_of::<uint>()) as *mut uint
    } else {
        0 as *mut uint
    }
}

extern fn handler(regs: *mut idt::registers) 
{
    unsafe {
        let mut cr2 = 0;
        asm!("mov %cr2, $0":"=r"(cr2):::);
        let x = get_page(cr2, true, current_dir);
        *x = alloc_frame(*x, false, true);
    }
}
