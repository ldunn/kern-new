pub fn range(lo: uint, hi: uint, it: |uint|) {
    let mut iter = lo;
    while iter <= hi {
        it(iter);
        iter += 1;
    }
}

pub unsafe fn strlen(s: &[u8]) -> uint {
    let mut i = 0;
    while s[i] != 0 { i += 1; };
    i - 1
}

#[no_mangle]
extern fn memset(ptr: uint, x: u8, len: uint) {
    unsafe {
        memset_u8(ptr, x, len);
    }
}

pub unsafe fn memset_u8(ptr: uint, x: u8, len: uint) {
        range(0, len, |i| {
            *((ptr + i) as *mut u8) = x;
        });
}

pub unsafe fn memset_u16(ptr: uint, x: u16, len: uint) {
    range(0, len-1, |i| {
        *((ptr + i * 2) as *mut u16) = x;
    });
}

#[no_mangle]
pub unsafe fn memcpy(to: uint, from: uint, len: uint) {
    let mut i = 0;
    while i < len {
        *((to+i) as *mut u8) = *((from + i) as *mut u8);
        i += 1;
    }
}

pub fn pow(b: uint, e: uint) -> uint {
    if e == 0 { return 1; }
    let mut x = b;
    range(1, e-1, |_| {
        x *= b;
    });
    x
}

extern {
    pub fn outportb(val: u8, port: u16);
    pub fn inportb(port: u16) -> u8;
}
