use idt;
use screen;

static mut ticks: uint = 0;

extern fn handler(_: *mut idt::registers)
{
    unsafe {
        ticks += 1;
    }
}

pub fn init()
{
    unsafe {
    idt::clear_mask(0);
    idt::register_interrupt_handler(32, handler)
    }
}

