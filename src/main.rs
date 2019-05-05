#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(start)]
#![feature(asm)]

mod base;

#[start]
#[no_mangle]
#[cfg(target_arch = "arm")]
pub fn _start() {
    unsafe {
        asm!(
            "b main"
        );
        asm!(
            ".fill 188, 1, 0"
        )
    }
}

#[no_mangle]
pub fn main() ->! {
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
