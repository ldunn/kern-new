#[no_std];
#[no_core];

#[link(name = "userland", vers="0.0")];
#[crate_type="executable"];


pub fn main() {
    unsafe {
        loop{};
    }
}
