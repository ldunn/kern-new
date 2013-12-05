#[no_std];

#[link(name = "userland", vers="0.0")];
#[crate_type="executable"];
 #[feature(asm)] ;


#[main]
#[no_mangle]
pub extern fn main() {
        loop{};
}
