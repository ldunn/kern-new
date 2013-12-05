use memory;
use util;
use screen;
use core::mem::size_of;
#[packed]
struct IDTDescr {
    offset_1: u16,
    selector: u16,
    zero: u8, // what the fuck?
    type_attr: u8,
    offset_2: u16
}

#[packed]
pub struct registers {
    ds: uint,
    edi: uint,
    esi: uint,
    ebp: uint,
    esp: uint,
    ebx: uint,
    edx: uint,
    ecx: uint,
    eax: uint,
    int_no: uint,
    err_code: uint,
    eip: uint,
    cs: uint,
    eflags: uint,
    useresp: uint,
    ss: uint
}

extern {
    pub fn set_idt(ptr: uint, size: uint) -> ();
    pub fn populate_idt_isr() -> ();
    pub fn populate_idt_irq() -> ();
    pub fn isr0() -> ();
    pub fn isr1() -> ();
    pub fn isr2() -> ();
    pub fn isr3() -> ();
    pub fn isr4() -> ();
    pub fn isr5() -> ();
    pub fn isr6() -> ();
    pub fn isr7() -> ();
    pub fn isr8() -> ();
    pub fn isr9() -> ();
    pub fn isr10() -> ();
    pub fn isr11() -> ();
    pub fn isr12() -> ();
    pub fn isr13() -> ();
    pub fn isr14() -> ();
    pub fn isr15() -> ();
    pub fn isr16() -> ();
    pub fn isr17() -> ();
    pub fn isr18() -> ();
    pub fn isr19() -> ();
    pub fn isr20() -> ();
    pub fn isr21() -> ();
    pub fn isr22() -> ();
    pub fn isr23() -> ();
    pub fn isr24() -> ();
    pub fn isr25() -> ();
    pub fn isr26() -> ();
    pub fn isr27() -> ();
    pub fn isr28() -> ();
    pub fn isr29() -> ();
    pub fn isr30() -> ();
    pub fn isr31() -> ();
    pub fn isr32() -> ();
    pub fn irq0() -> ();
    pub fn irq1() -> ();
    pub fn irq2() -> ();
    pub fn irq3() -> ();
    pub fn irq4() -> ();
    pub fn irq5() -> ();
    pub fn irq6() -> ();
    pub fn irq7() -> ();
    pub fn irq8() -> ();
    pub fn irq9() -> ();
    pub fn irq10() -> ();
    pub fn irq11() -> ();
    pub fn irq12() -> ();
    pub fn irq13() -> ();
    pub fn irq14() -> ();
    pub fn irq15() -> ();
}

static mut idt: *mut [IDTDescr,..256] = 0 as *mut [IDTDescr,..256];
static mut interrupt_handlers: *mut [extern "C" unsafe fn(regs: *mut registers), ..256] = 0 as *mut [extern "C" unsafe fn(regs: *mut registers), ..256];

pub unsafe fn register_interrupt_handler(n: u8, handler: extern "C" unsafe fn(regs: *mut registers))
{
    (*interrupt_handlers)[n] = handler;
}


pub unsafe fn set_mask(irq: uint) {
    if irq < 8 {
        // PIC1
        let val: u8 = util::inportb(0x21) | (1 << irq);
        util::outportb(val, 0x21);
    } else {
        //PIC2
        let val: u8 = util::inportb(0xA1) | (1 << irq);
        util::outportb(val, 0xA1);
    }
}

pub unsafe fn clear_mask(irq: uint) {
    if irq < 8 {
        // PIC1
        let val: u8 = util::inportb(0x21) & !(1 << irq);
        util::outportb(val, 0x21);
    } else {
        // PIC2
        let val: u8 = util::inportb(0xA1) & !(1 << irq);
        util::outportb(val, 0xA1);
    }
}

#[no_mangle]
pub extern fn irq_handler(mut regs: registers) -> uint
{
    unsafe {
    if (regs.int_no >= 40)
    {
        util::outportb(0x20, 0xA0);
    }

    util::outportb(0x20, 0x20);

    if((*interrupt_handlers)[regs.int_no] as uint != 0) {
       (*interrupt_handlers)[regs.int_no](&mut regs);
    }
    regs.eax
    }
}
#[no_mangle]
pub extern "C" fn idt_gate_from_asm(gate: uint, offset: uint)
{
    unsafe{
        if gate == 0x80 {
            set_gate(gate, offset, 0x08, 0xEE);
            return;
        }
        set_gate(gate, offset, 0x08, 0x8E);
    }
}

unsafe fn set_gate(gate: uint, offset: uint, selector: u16, type_attr: u8)
{
    (*idt)[gate] = IDTDescr {
        offset_1: (offset as u16) & 0xFFFF,
        selector: selector,
        zero: 0,
        type_attr: type_attr,
        offset_2: ((offset >> 16) as u16) & 0xFFFF
    };
}


unsafe fn remap_pic() 
{
    let mask1 = util::inportb(0x21);
    let mask2 = util::inportb(0xA1);
    util::outportb(0x11, 0x20);
    util::outportb(0x11, 0xA0);
    util::outportb(0x20, 0x21);
    util::outportb(0x28, 0xA1);
    util::outportb(0x04, 0x21);
    util::outportb(0x02, 0xA1);
    util::outportb(0x01, 0x21);
    util::outportb(0x01, 0xA1);
    util::outportb(mask1, 0x21);
    util::outportb(mask2, 0xA1);
}

#[no_mangle]
pub extern "C" fn isr_handler(mut regs: registers)
{
    unsafe {
        if ((*interrupt_handlers)[regs.int_no] as uint != 0) {
            util::outportb(0x20, 0x20);
            (*interrupt_handlers)[regs.int_no](&mut regs);
            return;
        }
        let colours = screen::Colours{fore: 15, back:4};
        screen::cls(4);
        util::range(0, 22, |_| {
            screen::putc('*' as u8, colours);
        });
        screen::putc('\n' as u8, colours);
        screen::putc('*' as u8, colours);
        screen::puts("Exception! ", colours);
        screen::puthex(regs.int_no, colours);
        screen::putc('*' as u8, colours);
        screen::putc('\n' as u8, colours);
        util::range(0, 22, |_| {
            screen::putc('*' as u8, colours);
        });
        if regs.int_no == 6 || regs.int_no == 0xD {screen::puthex(regs.eip, colours) }
        loop{};
    }
}

extern fn test()
{
    unsafe {
        let colours = screen::Colours{fore: 15, back:4};
        screen::puts("RECEIVED A SYSCALL\n", colours);
    }
}


pub unsafe fn init() {
    idt = memory::kernel_malloc(size_of::<IDTDescr>() * 256, false) as *mut [IDTDescr, ..256];
    let int_handler_addr = memory::kernel_malloc(size_of::<*extern "C" unsafe fn(regs: *mut registers)>() * 256, false);
    interrupt_handlers = int_handler_addr as *mut [extern "C" unsafe fn(regs: *mut registers),..256];
    util::memset_u8(idt as uint, 0, size_of::<IDTDescr>() * 256 -1);
    util::memset_u8(interrupt_handlers as uint, 0, size_of::<extern "C" unsafe fn(regs: *mut registers)>() * 256);
    populate_idt_isr();
    remap_pic();
    populate_idt_irq();
    set_idt(idt as uint, size_of::<IDTDescr>() * 256 -1);
}
