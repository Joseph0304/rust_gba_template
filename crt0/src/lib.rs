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
        asm!( // Nintendo Logo, 156 Bytes
            "
            .word 0x51aeff24;
            .word 0x21a29a69;
            .word 0x0a82843d;
            .word 0x988b2411;
            .word 0x217f81c0;
            .word 0x19be52a3;
            .word 0x4a4a4610;
            .word 0xec3127f8;
            .word 0x33e8c758;
            .word 0x94dff485;
            .word 0xc1094bce;
            .word 0xc08a5694;
            .word 0x734d849f;
            .word 0x619acaa3;
            .word 0x27a39758;
            .word 0x61c71d23;
            .word 0x56ae0403;
            .word 0x008438bf;
            .word 0x03fe52ff;
            .word 0xf130956f;
            .word 0x85c0fb97;
            .word 0x03be63a9;
            .word 0xe2384e01;
            .word 0xff34a2f9;
            .word 0xcb900078;
            .word 0x943a1188;
            .word 0x637cc065;
            .word 0x8be425d6;
            .word 0x72ac0a38;
            .word 0x07f8d421;
            "
        );
        asm!( // Game Title, Uppercase Ascii, max 12 characters
            "
            .word 0
            .word 0
            .word 0
            "
        );
        asm!( // Game Code, Uppercase Ascii, 4 characters
            "
            .word 0
            "
        );
        asm!( // Maker code, Uppercase Ascii, 2 characters
            "
            .hword 0
            "
        );
        asm!( // Fixed value, 1 Byte
              // Must be 96h.
            "
            .byte 0x96
            "
        );
        asm!( // Main unit code, 1 Byte
            "
            .byte 0
            "
        );
        asm!( // Device type, 1 Byte
              // Normally, this entry should be zero.
            "
            .byte 0;
            "
        );
        asm!( // Reserved Area, 7 Bytes
              // Reserved, zero filled.
            "
            .byte 0;
            .byte 0;
            .byte 0;
            .byte 0;
            .byte 0;
            .byte 0;
            .byte 0;
            "
        );
        asm!( // Software version number
              //Version number of the game. Usually zero.
            "
            .byte 0
            "
        );
        asm!( // Complement check, 1 Byte
              // Header checksum, cartridge won't work if incorrect. Calculate as such:
              // chk=0:for i=0A0h to 0BCh:chk=chk-[i]:next:chk=(chk-19h) and 0FFh
            "
            .byte 0
            "
        );
        asm!( // Reserved Area, 2 Bytes
              // Reserved, zero filled.
            "
            .hword 0
            "
        );
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
