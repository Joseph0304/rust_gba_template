#![no_main]
#![no_std]
#![feature(lang_items)]
#![feature(asm)]

extern crate crt0;

#[no_mangle]
pub fn main() -> ! {
    let ioram = 0x04000000 as *mut u8;
    unsafe {
        for &(offset, value) in &[
            (0, 0x03u8), // Set the 'video mode' to 3 (in which BG2 is a 16 bpp bitmap in VRAM)
            (1, 0x04u8), // Enable BG2 (BG0 = 1, BG1 = 2, BG2 = 4, ...)
        ] {
            let ptr = ioram.add(offset);
            *ptr = value;
        }
    }

    // Write pixel colours into VRAM
    let vram = 0x06000000 as *mut u16;
    unsafe {
        for &(offset, value) in &[
            (80*240 + 115, 0x001Fu16), // X = 115, Y = 80, C = 000000000011111 = R
            (80*240 + 120, 0x03E0u16), // X = 120, Y = 80, C = 000001111100000 = G
            (80*240 + 125, 0x7C00u16), // X = 125, Y = 80, C = 111110000000000 = B
        ] {
            let ptr = vram.add(offset);
            *ptr = value;
        }
    }

    loop {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
