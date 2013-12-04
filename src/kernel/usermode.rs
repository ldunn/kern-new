#[no_std]
#[no_core]

use gdt;
use util;
use memory;
use screen;
use core::mem::size_of;

#[packed]
pub struct tss_entry {
    prev_tss: uint,
    esp0: uint,
    ss0: uint,
    esp1: uint,
    ss1: uint,
    esp2: uint,
    ss2: uint,
    cr3: uint,
    eip: uint,
    eflags: uint,
    eax: uint,
    ecx: uint,
    edx: uint,
    ebx: uint,
    esp: uint,
    ebp: uint,
    esi: uint,
    edi: uint,
    es: uint,
    cs: uint,
    ss: uint,
    ds: uint,
    fs: uint,
    gs: uint,
    ldt: uint,
    trap: u16,
    iomap_base: u16
}

static mut tss: *mut tss_entry = 0 as *mut tss_entry;


pub unsafe fn tss_entry(g: *mut gdt::GDT)
{

    tss = memory::kernel_malloc(size_of::<tss_entry>(), false) as *mut tss_entry;
    let base = tss as uint;
    let limit = base + 104;

    (*g).base_low = (base as u16) & 0xFFFF;
    (*g).base_middle = (((base) >> 16) & 0xFF) as u8;
    (*g).base_high = (((base) >> 24) & 0xFF) as u8;
    (*g).limit_low = (limit as u16) & 0xFFF;
    (*g).access = 1 | gdt::GDT_CODE | 0x80;
    (*g).flags_and_limit = gdt::GDT_GRANULAR | gdt::GDT_32BIT | ((limit >> 16) as u8 & 0xF);
    util::memset_u8(tss as uint, 0, size_of::<tss_entry>());

    let mut esp = 0;
    asm!("mov %esp, $0" :"=r"(esp):::"volatile");
    (*tss).ss0 = 0x10;
    (*tss).esp0 = esp;
    (*tss).cs = 0x0b;
    (*tss).ss = 0x13;
    (*tss).ds = 0x13;
    (*tss).es = 0x13;
    (*tss).fs = 0x13;
    (*tss).gs = 0x13;
    (*tss).iomap_base = 104;
}

pub unsafe fn set_kernel_stack(stack: uint)
{
    (*tss).esp0 = stack;
}
