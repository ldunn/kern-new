#[no_std];

#[link(name = "userland", vers="0.0")];
#[crate_type="executable"];
 #[feature(asm)] ;


#[main]
#[no_mangle]
pub extern fn main() {
    unsafe {
        *(0xdeadcafe as *mut uint) = 0xdeadcafe;
        asm!("mov $0, %eax"::"r"(*(0xdeadcafe as *mut uint))::);
        loop{};
    }
}
