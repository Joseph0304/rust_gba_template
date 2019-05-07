use core::panic::PanicInfo;

#[lang = "eh_personality"]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
pub fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
