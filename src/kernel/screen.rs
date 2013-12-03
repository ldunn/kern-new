#[no_std];
#[no_core];

use util;
use core::vec;
use core::slice;
use core::option::{Some, None};
use memory::Alloc;

pub struct Cursor {
    x: uint,
    y: uint
}

pub struct Colours {
    fore: u16,
    back: u16
}

pub static mut cursor: Cursor = Cursor {x:0,y:0};

pub unsafe fn cls(background: u16) {
    util::memset_u16(0xb8000, 0x20 | (background << 12), 80*25);
    cursor = Cursor{x:0, y:0};
}

#[fixed_stack_segment]
pub unsafe fn move_cursor()
{
    let position = ((cursor.y * 80 + cursor.x)) as u16;
    util::outportb(0x0F, 0x3D4);
    util::outportb((position & 0xFF) as u8, 0x3D5);
    util::outportb(0x0E, 0x3D4);
    util::outportb(((position>>8)&0xFF) as u8, 0x3D5);
}

pub unsafe fn putc(c: u8, colours: Colours)
{
    if c == 10 { // Newline
        cursor.y += 1;
        cursor.x = 0;
        scroll(colours.back);
        move_cursor();
        return;
    }
    else if c == 8 { // Backspace
        if cursor.x > 0 {
            cursor.x -= 1;
            putc_at(' ' as u8, cursor.x, cursor.y, colours);
            move_cursor();
        }
        return;
    }
    putc_at(c, cursor.x, cursor.y, colours);
    if cursor.x <= 80 {
        cursor.x += 1;
    } else {
        cursor.x = 0;
        cursor.y += 1;
    }
    scroll(colours.back);
    move_cursor();
}


pub unsafe fn puts(s: &str, colours: Colours)
{
    let mut i = 0;
    while i < s.len() as u8 { putc(s[i], colours); i += 1; };

}

pub unsafe fn putcstr(mut s: *u8, colours: Colours) 
{
    while *s != 0 {
        putc(*s, colours);
        s = (s as uint + 1) as *u8;
    }
}

static nybble_chars: [char, ..16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];
pub unsafe fn puthex(hex: uint, colours: Colours)
{
    putc('0' as u8, colours);
    putc('x' as u8, colours);
    let tmp = 0;
    util::memcpy(&tmp as *uint as uint, &hex as *uint as uint, 4);
    let ptr = &tmp as *uint as uint as *u8;
    util::range(0,3, |i| {
        let byte = *((ptr as uint + (3-i)) as *u8);
        putc(nybble_chars[(byte >> 4) & 0xF]as u8, colours);
        putc(nybble_chars[byte & 0x0F] as u8, colours);
    });
}

pub unsafe fn putdec(dec: uint, colours: Colours)
{
    if dec == 0 {
        puts("0", colours);
        return;
    }
    let mut buf = vec::Vec::with_alloc(Alloc) ;
    let mut digit = dec % 10;
    let mut val = dec;
    while val > 0 {
        buf.push(nybble_chars[digit] as u8);
        val /= 10;
        digit = val % 10;
    }

    util::range(0, buf.len()-1, |i| {
        let x = buf.pop();
        match x {
            Some(x) => putc(x, colours),
            None => ()
        }
    });
}


pub unsafe fn scroll(background: u16) {
    if cursor.y >= 25 {
        cursor.y = 24;
        util::range(0, 24*80, |i| {
            util::memset_u16(0xb8000+i*2, *((0xb8000+(i+80)*2) as *u16), 1);
        });
        util::range(24*80+1, 25*80, |i| {
            util::memset_u16(0xb8000+i*2, 0x20 | background << 12, 1);
        });
    }
}

pub unsafe fn putc_at(c: u8, x: uint, y: uint, colours: Colours) {
    let attrib = (colours.back << 4) | (colours.fore & 0x0F) as u16;
    util::memset_u16(0xb8000 + 2 *  (y * 80 + x), (c as u16) | (attrib << 8), 1);
}

