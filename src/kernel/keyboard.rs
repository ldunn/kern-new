use util;
use screen;
use idt;

static keysym: [u8, ..90] = [0, 27, '1' as u8, '2' as u8, '3' as u8, '4' as u8, '5' as u8, '6' as u8, '7' as u8, '8' as u8, /* 9 */
  '9' as u8, '0' as u8, '-' as u8, '=' as u8, '\x08' as u8, /* Backspace */
  '\t' as u8, /* Tab */
  'q' as u8, 'w' as u8, 'e' as u8, 'r' as u8, /* 19 */
  't' as u8, 'y' as u8, 'u' as u8, 'i' as u8, 'o' as u8, 'p' as u8, '[' as u8, ']' as u8, '\n' as u8, /* Enter key */
  0, /* 29 - Control */
  'a' as u8, 's' as u8, 'd' as u8, 'f' as u8, 'g' as u8, 'h' as u8, 'j' as u8, 'k' as u8, 'l' as u8, ';' as u8, /* 39 */
  '\'' as u8, '`' as u8, 0, /* Left shift */
  '\\' as u8, 'z' as u8, 'x' as u8, 'c' as u8, 'v' as u8, 'b' as u8, 'n' as u8, /* 49 */
  'm' as u8, ',' as u8, '.' as u8, '/' as u8, 0, /* Right shift */
  '*' as u8,
  0, /* Alt */
  ' ' as u8, /* Space bar */
  0, /* Caps lock */
  0, /* 59 - F1 key ... > */
  0, 0, 0, 0, 0, 0, 0, 0,
  0, /* < ... F10 */
  0, /* 69 - Num lock*/
  0, /* Scroll Lock */
  0, /* Home key */
  0, /* Up Arrow */
  0, /* Page Up */
  '-' as u8,
  0, /* Left Arrow */
  0,
  0, /* Right Arrow */
  '+' as u8,
  0, /* 79 - End key*/
  0, /* Down Arrow */
  0, /* Page Down */
  0, /* Insert Key */
  0, /* Delete Key */
  0, 0, 0,
  0, /* F11 Key */
  0, /* F12 Key */
  0, /* All other keys are undefined */];

static keysymShift: [u8, ..90] = [0, 27, '!' as u8, '@' as u8, '#' as u8, '$' as u8, '%' as u8, '^' as u8, '&' as u8, '*' as u8, /* 9 */
  '(' as u8, ')' as u8, '_' as u8, '+' as u8, '\x08' as u8, /* Backspace */
  '\t' as u8, /* Tab */
  'W' as u8, 'W' as u8, 'E' as u8, 'R' as u8, /* 19 */
  'T' as u8, 'Y' as u8, 'U' as u8, 'I' as u8, 'O' as u8, 'P' as u8, '{' as u8, '}' as u8, '\n' as u8, /* Enter key */
  0, /* 29 - Control */
  'A' as u8, 'S' as u8, 'D' as u8, 'F' as u8, 'G' as u8, 'H' as u8, 'J' as u8, 'K' as u8, 'L' as u8, ':' as u8, /* 39 */
  '"' as u8, '~' as u8, 0, /* Left shift */
  '|' as u8, 'Z' as u8, 'X' as u8, 'C' as u8, 'V' as u8, 'B' as u8, 'N' as u8, /* 49 */
  'M' as u8, '<' as u8, '>' as u8, '?' as u8, 0, /* Right shift */
  '*' as u8,
  0, /* Alt */
  ' ' as u8, /* Space bar */
  0, /* Caps lock */
  0, /* 59 - F1 key ... > */
  0, 0, 0, 0, 0, 0, 0, 0,
  0, /* < ... F10 */
  0, /* 69 - Num lock*/
  0, /* Scroll Lock */
  0, /* Home key */
  0, /* Up Arrow */
  0, /* Page Up */
  '-' as u8,
  0, /* Left Arrow */
  0,
  0, /* Right Arrow */
  '+' as u8,
  0, /* 79 - End key*/
  0, /* Down Arrow */
  0, /* Page Down */
  0, /* Insert Key */
  0, /* Delete Key */
  0, 0, 0,
  0, /* F11 Key */
  0, /* F12 Key */
  0, /* All other keys are undefined */];

static mut buff: [uint, ..64] = [0, ..64];
static mut pressed: [u8, ..90] = [0, ..90];
static mut buff_start: u8 = 0;
static mut buff_end: u8 = 0;

extern fn handler(regs: *mut idt::registers)
{
    unsafe {
        let x = util::inportb(0x60);
        
        if (x & 0x80) == 0 {
            buff[buff_end] = (x as uint) | (pressed[0x2A] as uint << 8);
            
            buff_end += 1;
            if (buff_end >= 64) {
                buff_end = 0
            }
            
            if (buff_end == buff_start) {
                if (buff_end == 0) {
                    buff_end = 63
                } else {
                    buff_end = buff_start - 1
                }
            }
            
            pressed[x] = 1
        } else {
            pressed[x & !0x80] = 0
        }
    }
    unsafe {
    let x = getChar();
    if x > 0 {
        screen::putc(x,screen::Colours{fore:7, back:0});
    }
    }
}

pub fn getChar() -> u8 {
    unsafe {
        if (buff_start == buff_end) {
            return 0
        }
        
        let code = buff[buff_start];
        
        buff_start += 1;
        if (buff_start >= 64) {
            buff_start = 0;
        }

        if (code & 0x100 > 0) {
            return keysymShift[code & 0xff]
        } else {
            return keysym[code]
        }
        0
    }
}

pub fn init()
{
    unsafe {
        idt::clear_mask(1);
        idt::register_interrupt_handler(33, handler);
    }
}

