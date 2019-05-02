#![no_std]
#![feature(lang_items)]

mod base;

#[no_mangle]
pub fn main() -> ! {
    let _x = 42;

    loop {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
