#![no_std]
#![feature(lang_items)]

use core::panic::PanicInfo;

#[lang = "eh_personality"]
pub extern "C" fn rust_eh_personality() {}

#[no_mangle]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

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
