use memory;

use core::mem::size_of;
use util;
use usermode;

#[packed]
pub struct GDT {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    flags_and_limit: u8,
    base_high: u8
}

struct GDTPtr {
    limit: u16,
    base: uint
}

pub static GDT_PRESENT: u8 = 0x80;

pub static GDT_DPL0: u8 = 0x00; //DPL = descriptor priv level (ring level)
pub static GDT_DPL1: u8 = 0x20;
pub static GDT_DPL2: u8 = 0x40;
pub static GDT_DPL3: u8 = 0x60;

pub static GDT_CODE: u8 = 0x08;
pub static GDT_READABLE: u8 = 0x02;
pub static GDT_CONFORMING: u8 = 0x04;

pub static GDT_DATA: u8 = 0x00;
pub static GDT_WRITABLE: u8 = 0x02;
pub static GDT_GROW_DOWN: u8 = 0x04;

pub static GDT_GRANULAR: u8 = 0x80;
pub static GDT_32BIT: u8 = 0x40;
pub static GDT_16BIT: u8 = 0x00;

static mut gdt: *GDT = 0 as *GDT;
static mut next_entry: uint = 0;
static GDT_ENTRIES: uint = 256;

extern {
    pub fn set_gdt(ptr: uint, size: uint) -> ();
    fn tss_flush() -> ();
}

/* Copied with love from tsion's spideros */
unsafe fn new_entry(base: uint, limit: uint, access: u8, flags: u8)
{
    let new = &GDT {
        base_low: (base as u16) & 0xFFFF,
        base_middle: (((base) >> 16) & 0xFF) as u8,
        base_high: (((base) >> 24) & 0xFF) as u8,
        limit_low: (limit as u16) & 0xFFFF,
        flags_and_limit: flags | ((limit >> 16) as u8 & 0xF),
        access: access | 0x10
    };
    util::memcpy(gdt as uint + next_entry*size_of::<GDT>(), new as *GDT as uint, size_of::<GDT>());
    next_entry += 1;
    if next_entry > 1 {
    }
}


pub unsafe fn init() {
    gdt  = memory::kernel_malloc(size_of::<GDT>() * GDT_ENTRIES, false) as *GDT;
    new_entry(0,0,0,0);
    new_entry(0, 0xFFFFFFFF, GDT_PRESENT | GDT_DPL0 | GDT_CODE | GDT_READABLE, GDT_GRANULAR | GDT_32BIT);
    new_entry(0, 0xFFFFFFFF, GDT_PRESENT | GDT_DPL0 | GDT_DATA | GDT_WRITABLE, GDT_GRANULAR | GDT_32BIT);
    new_entry(0, 0xFFFFFFFF, GDT_PRESENT | GDT_DPL3 | GDT_CODE | GDT_READABLE, GDT_GRANULAR | GDT_32BIT);
    new_entry(0, 0xFFFFFFFF, GDT_PRESENT | GDT_DPL3 | GDT_DATA | GDT_WRITABLE, GDT_GRANULAR | GDT_32BIT);
    util::memset_u8((gdt as uint + 5*size_of::<GDT>()), 0, size_of::<GDT>());
    usermode::tss_entry((gdt as uint + 5*size_of::<GDT>()) as *mut GDT);
    set_gdt(gdt as uint, size_of::<GDT>() * GDT_ENTRIES - 1);
    tss_flush();
}

