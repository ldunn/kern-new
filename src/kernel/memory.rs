use core::mem;
use util;
use screen;

static mut HEAP_START: uint = 0;
static BLOCK_SIZE: uint = 4096; // one byte for the free flag
static mut free_blocks: [uint, ..64] = [0xffffffff,..64]; // single-bit flags
static mut first_free: uint = 0;

pub unsafe fn find_block_run(start: uint, n: uint, current: uint) -> uint {
    if free_blocks[start / 32] & (1<<(start % 32)) != 0 {
        if n == current {
            return start - n;
        } else {
            return find_block_run(start+1, n, current + 1);
        }
    }
    return find_block_run(start+1, n, current);
}

extern {
    static end: uint; //defined by linker
}

pub unsafe fn init() {
    HEAP_START = (((0x400000 as *uint) as uint) & 0xFFFFF000) + 0x1000;
}

#[no_mangle]
#[lang="exchange_malloc"]
pub unsafe fn malloc(size: uint) -> (*mut u8, uint) {
    let block = find_block_run(first_free, size/BLOCK_SIZE + 1, 0);
    util::range(block, block + size/BLOCK_SIZE, |iter| {
        free_blocks[iter / 32] = free_blocks[iter/32] ^ (1<<(iter % 32));
    });
    return (((block*BLOCK_SIZE) + HEAP_START) as *mut u8, size);
}

pub unsafe fn malloc_phys(size: uint) -> (*mut u8, uint, *mut u8) {
    let block = find_block_run(first_free, size/BLOCK_SIZE + 1, 0);
    util::range(block, block + size/BLOCK_SIZE, |iter| {
        free_blocks[iter / 32] = free_blocks[iter/32] ^ (1<<(iter % 32));
    });
    (((block*BLOCK_SIZE) + HEAP_START) as *mut u8, size, ((block*BLOCK_SIZE) + HEAP_START) as *mut u8)
}

pub unsafe fn realloc(ptr: *mut u8, size: uint) -> (*mut u8, uint)
{
    let (new_mem,_) = malloc(size);
    util::memcpy(new_mem as uint, ptr as uint, BLOCK_SIZE); // TODO: FIX THIS;
    (new_mem, size)
}

#[lang="exchange_free"]
unsafe fn exchange_free(addr: *mut u8) {
    let addr = addr as uint;
    let block = (addr - HEAP_START) / BLOCK_SIZE;
    free_blocks[block / 32] = free_blocks[block / 32] | (1 << (block % 32));
}

pub unsafe fn free(addr: *mut u8) { exchange_free(addr); }


pub struct Alloc;
impl mem::Allocator for Alloc {
    unsafe fn alloc(&mut self, size:uint) -> (*mut u8, uint) { malloc(size) }
    unsafe fn zero_alloc(&mut self, size:uint) -> (*mut u8, uint) { malloc(size) }
    unsafe fn realloc(&mut self, ptr: *mut u8, size: uint) -> (*mut u8, uint) { realloc(ptr, size) }
    unsafe fn free(&mut self, ptr: *mut u8) { free(ptr); }
}
